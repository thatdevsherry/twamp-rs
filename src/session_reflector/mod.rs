use std::time::Duration;

use crate::timestamp::TimeStamp;
use crate::twamp_test::{TwampTestPacketUnauth, TwampTestPacketUnauthReflected};
use deku::prelude::*;
use tokio::{net::UdpSocket, time::timeout};
use tracing::*;

#[derive(Debug)]
pub struct SessionReflector {
    socket: UdpSocket,
    refwait: u16,
}

impl SessionReflector {
    /// socket should already be `connect`ed to the dest.
    pub async fn new(socket: UdpSocket, refwait: u16) -> Self {
        Self { socket, refwait }
    }

    /// Listens for TWAMP-Test packets.
    ///
    /// Once it receives one, it processes it and sends back TWAMP-Test reflected packet.
    ///
    /// It only listens for packets until REFWAIT timeout. If REFWAIT timeout is reached, it
    /// returns with an Error.
    pub async fn do_reflect(self) -> anyhow::Result<()> {
        let local_addr = self.socket.local_addr().unwrap();
        let peer_addr = self.socket.peer_addr().unwrap();
        debug!("Listening for pkts from {} on {}", peer_addr, local_addr);
        let mut seq: u32 = 0;
        loop {
            let mut buf = [0u8; 1472]; // 1472 for max MTU. Even though we aren't setting padding
            // above 27. Still setting this big for now.
            let bytes_read = timeout(
                Duration::from_secs(self.refwait.into()),
                self.socket.recv(&mut buf),
            )
            .await;
            if bytes_read.is_err() {
                return Err(anyhow!("REFWAIT expired."));
            }
            let recv_timestamp = TimeStamp::default();
            trace!("bytes read: {}", bytes_read.unwrap().unwrap());
            let (_rest, twamp_test_unauth) = TwampTestPacketUnauth::from_bytes((&buf, 0)).unwrap();
            trace!("Twamp-Test: {:?}", twamp_test_unauth);
            debug!(
                "Read Twamp-Test with seq: {}",
                twamp_test_unauth.sequence_number
            );
            let pkt_reflected =
                TwampTestPacketUnauthReflected::new(seq, twamp_test_unauth, recv_timestamp);
            let encoded = pkt_reflected.to_bytes().unwrap();
            let len = self.socket.send(&encoded[..]).await.unwrap();
            trace!("Sent reflected pkt of bytes: {}", len);
            seq += 1;
        }
    }
}
