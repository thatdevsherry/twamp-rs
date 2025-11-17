use crate::timestamp::TimeStamp;
use crate::twamp_test::{TwampTestPacketUnauth, TwampTestPacketUnauthReflected};
use deku::prelude::*;
use std::{
    io,
    net::{SocketAddr, SocketAddrV4},
    sync::Arc,
};
use thiserror::Error;
use tokio::{net::UdpSocket, sync::Mutex};
use tracing::*;

/// Errors that can be raised by [SessionSender].
#[derive(Error, Debug)]
pub enum SessionSenderError {
    /// Raised when "connect" call to a Session-Reflector fails.
    ///
    /// - First field is the socket address of the Session-Reflector it was trying to connect to.
    #[error("failed to connect to Session-Reflector: {0}")]
    SessionReflectorConnectError(SocketAddrV4, #[source] io::Error),

    /// Indicates read error on TWAMP-Test **after** it was initiated.
    ///
    /// - First field is the number of packets that were reflected back to Session-Sender before
    /// this error.
    #[error("failed to read UDP datagram from socket after {0} packets were reflected")]
    SessionReflectorReadError(u32, #[source] io::Error),

    #[error("failed to send UDP datagram to Session-Reflector after {0} packets were sent")]
    SessionReflectorWriteError(u32, #[source] io::Error),

    /// Indicates Session-Sender not being bound to a socket.
    ///
    /// This usually indicates an OS level error, since UdpSocket is made from
    /// "bind", and this error indicates something unbinded the UdpSocket after it's creation.
    #[error("Session-Sender was not not bound to a socket")]
    NotBound(#[source] io::Error),

    /// Indicates Session-Sender was not "connected" to Session-Reflector's UdpSocket.
    ///
    /// TODO: This can also indicate an OS level issue, since "connection" is established in
    /// [SessionSender::new].
    #[error("Session-Reflector was not connected to socket")]
    SessionReflectorNotConnected(#[source] io::Error),

    /// Indicates underlying (de)serialization failure.
    #[error("failed to convert between rust and wire format")]
    WireConversionError(#[source] deku::error::DekuError),
}

#[derive(Debug)]
pub struct SessionSender {
    pub socket: Arc<UdpSocket>,
    pub dest: SocketAddr,
}

impl SessionSender {
    pub async fn new(socket: Arc<UdpSocket>, dest: SocketAddrV4) -> Self {
        Self {
            socket,
            dest: SocketAddr::V4(dest),
        }
    }

    pub async fn send_it(&self, number_of_packets: u32) -> Result<(), SessionSenderError> {
        info!("Sending Twamp-Test packets to {}", self.dest);
        for i in 0..number_of_packets {
            let twamp_test = TwampTestPacketUnauth::new(i, 0, true);
            trace!("Twamp-Test: {:?}", twamp_test);
            let encoded = twamp_test
                .to_bytes()
                .map_err(|err| SessionSenderError::WireConversionError(err))?;
            let local_addr = self
                .socket
                .local_addr()
                .map_err(|err| SessionSenderError::NotBound(err))?;
            let peer_addr = self
                .socket
                .peer_addr()
                .map_err(|err| SessionSenderError::SessionReflectorNotConnected(err))?;
            trace!("Sending pkt from {} to {}", local_addr, peer_addr);
            let len = self
                .socket
                .send(&encoded[..])
                .await
                .map_err(|err| SessionSenderError::SessionReflectorWriteError(i, err))?;
            trace!("Twamp-Test sent of bytes: {}", len);
        }
        Ok(())
    }

    pub async fn recv(
        &self,
        number_of_packets: u32,
        reflected_pkts_shared: Arc<Mutex<Vec<(TwampTestPacketUnauthReflected, TimeStamp)>>>,
    ) -> Result<(), SessionSenderError> {
        let mut count: u32 = 0;
        loop {
            let mut buf = [0u8; 1024]; // Buffer to hold incoming packets
            let bytes_read = self
                .socket
                .recv(&mut buf)
                .await
                .map_err(|err| SessionSenderError::SessionReflectorReadError(count, err))?;
            trace!("Bytes read: {}", bytes_read);
            let (_rest, reflected_pkt) = TwampTestPacketUnauthReflected::from_bytes((&buf, 0))
                .map_err(|err| SessionSenderError::WireConversionError(err))?;
            trace!("Received reflected pkt: {:?}", reflected_pkt);
            let mut acquired_vec = reflected_pkts_shared.lock().await;
            acquired_vec.push((reflected_pkt, TimeStamp::default()));
            count += 1;
            if count == number_of_packets {
                break;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
