use std::io;
use std::net::SocketAddrV4;
use std::time::Duration;

use crate::timestamp::TimeStamp;
use crate::twamp_test::{TwampTestPacketUnauth, TwampTestPacketUnauthReflected};
use deku::prelude::*;
use thiserror::Error;
use tokio::{net::UdpSocket, time::timeout};
use tracing::*;

#[derive(Error, Debug)]
pub enum SessionReflectorError {
    /// Raised when "connect" call to a Session-Sender fails.
    ///
    /// - First field is the socket address of the Session-Sender it was trying to connect to.
    #[error("failed to connect to Session-Sender: {0}")]
    SessionSenderConnectError(SocketAddrV4, #[source] io::Error),

    /// Indicates read error on TWAMP-Test **after** it was initiated.
    ///
    /// - First field is the number of packets that were processed by Session-Reflector before
    /// this error.
    #[error("failed to read UDP datagram from socket after {0} packets were processed")]
    SessionSenderReadError(u32, #[source] io::Error),

    #[error("failed to send UDP datagram to Session-Sender after {0} packets were processed")]
    SessionSenderWriteError(u32, #[source] io::Error),

    /// Indicates REFWAIT timeout was reached when Session-Reflector was waiting for a TWAMP-Test
    /// packet.
    ///
    /// - First field is the REFWAIT timeout in seconds
    /// - Second is number of TWAMP-Test packets that Session-Reflector had processed before
    /// REFWAIT timeout was reached
    #[error("REFWAIT timeout of {0} seconds reached; pkts processed: {1}")]
    RefwaitTimeout(u16, u32),

    /// Indicates Session-Reflector not being bound to a socket.
    ///
    /// This usually indicates an OS level error, since UdpSocket is made from
    /// "bind", and this error indicates something unbinded the UdpSocket after it's creation.
    #[error("Session-Reflector was not not bound to a socket")]
    NotBound(#[source] io::Error),

    /// Indicates Session-Sender was not "connected" to Session-Reflector's UdpSocket.
    ///
    /// This can also indicate an OS level issue, since "connection" is established in
    /// [SessionReflector::new].
    #[error("Session-Sender was not connected to socket")]
    SessionSenderNotConnected(#[source] io::Error),

    /// Indicates underlying (de)serialization failure.
    #[error("failed to convert between rust and wire format")]
    WireConversionError(#[source] deku::error::DekuError),
}

/// Part of the TWAMP-Test protocol.
///
/// It is responsible for listening for TWAMP-Test packets from
/// [Session-Sender](crate::session_sender::SessionSender) and reflecting
/// back "Reflected" [TWAMP-Test packets](crate::twamp_test::TwampTestPacketUnauthReflected).
///
/// It performs minimal computation on the received packets, usually copying over
/// sequence number and a few fields to the "Reflected" TWAMP-Test packets.
///
/// See [Section 4.2](https://datatracker.ietf.org/doc/html/rfc5357#section-4.2)
/// of RFC for more details.
#[derive(Debug)]
pub struct SessionReflector {
    socket: UdpSocket,
    refwait: u16,
}

impl SessionReflector {
    /// Creates a new Session-Reflector and connects socket to the Session-Sender.
    pub async fn new(
        socket: UdpSocket,
        session_sender_addr: SocketAddrV4,
        refwait: u16,
    ) -> Result<Self, SessionReflectorError> {
        socket.connect(session_sender_addr).await.map_err(|err| {
            SessionReflectorError::SessionSenderConnectError(session_sender_addr, err)
        })?;
        Ok(Self { socket, refwait })
    }

    /// Listens for [TWAMP-Test](crate::twamp_test::TwampTestPacketUnauth) packets.
    ///
    /// Once it receives one, it processes it and sends back
    /// [TWAMP-Test reflected packet](crate::twamp_test::TwampTestPacketUnauthReflected).
    ///
    /// It only listens for packets until REFWAIT timeout. If REFWAIT timeout is reached, it
    /// returns with an Error.
    pub async fn do_reflect(self) -> Result<(), SessionReflectorError> {
        let local_addr = self
            .socket
            .local_addr()
            .map_err(|err| SessionReflectorError::NotBound(err))?;
        let peer_addr = self
            .socket
            .peer_addr()
            .map_err(|err| SessionReflectorError::SessionSenderNotConnected(err))?;
        debug!("Listening for pkts from {} on {}", peer_addr, local_addr);
        let mut pkts_processed_number: u32 = 0;
        loop {
            let mut buf = [0u8; 1472]; // 1472 for max MTU. Even though we aren't setting padding
            // above 27. Still setting this big for now.
            let bytes_read = timeout(
                Duration::from_secs(self.refwait.into()),
                self.socket.recv(&mut buf),
            )
            .await
            .map_err(|_| {
                SessionReflectorError::RefwaitTimeout(self.refwait, pkts_processed_number)
            })?
            .map_err(|err| {
                SessionReflectorError::SessionSenderReadError(pkts_processed_number, err)
            })?;
            let recv_timestamp = TimeStamp::default();
            trace!("bytes read: {}", bytes_read);
            let (_rest, twamp_test_unauth) = TwampTestPacketUnauth::from_bytes((&buf, 0))
                .map_err(|err| SessionReflectorError::WireConversionError(err))?;
            trace!("Twamp-Test: {:?}", twamp_test_unauth);
            debug!(
                "Read Twamp-Test with seq: {}",
                twamp_test_unauth.sequence_number
            );
            let pkt_reflected = TwampTestPacketUnauthReflected::new(
                pkts_processed_number,
                twamp_test_unauth,
                recv_timestamp,
            );
            let encoded = pkt_reflected
                .to_bytes()
                .map_err(|err| SessionReflectorError::WireConversionError(err))?;
            let len = self.socket.send(&encoded[..]).await.map_err(|err| {
                SessionReflectorError::SessionSenderWriteError(pkts_processed_number, err)
            })?;
            trace!("Sent reflected pkt of bytes: {}", len);
            pkts_processed_number += 1;
        }
    }
}
