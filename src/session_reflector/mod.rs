use std::{sync::Arc, time::Duration};

use crate::timestamp::TimeStamp;
use crate::twamp_test::{
    twamp_test_unauth::TwampTestPacketUnauth,
    twamp_test_unauth_reflected::TwampTestPacketUnauthReflected,
};
use anyhow::{Result, anyhow};
use deku::prelude::*;
use tokio::{net::UdpSocket, spawn, time::timeout};
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

    /// Starts reflecting TWAMP-Test packets indefinitely.
    pub async fn do_reflect(self) -> Result<()> {
        let l = self.socket.local_addr().unwrap();
        let p = self.socket.peer_addr().unwrap();
        let sock = Arc::new(self.socket);
        debug!("Listening for pkts from {} on {}", p, l);
        let mut seq: u32 = 0;
        loop {
            let sock_clone = Arc::clone(&sock);
            let mut buf = [0u8; 1472]; // 1472 for max MTU. Even though we aren't setting padding
            // above 27. Still setting this big for now.
            let bytes_read = timeout(
                Duration::from_secs(self.refwait.into()),
                sock_clone.recv(&mut buf),
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
            // spawn task so we still read
            spawn(async move {
                let pkt = twamp_test_unauth;
                let pkt_reflected = TwampTestPacketUnauthReflected::new(seq, pkt, recv_timestamp);
                let encoded = pkt_reflected.to_bytes().unwrap();
                let len = sock_clone.send(&encoded[..]).await.unwrap();
                trace!("Sent reflected pkt of bytes: {}", len);
            });
            seq += 1;
        }
    }
}
