use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
};

use deku::prelude::*;
use timestamp::timestamp::TimeStamp;
use tokio::{net::UdpSocket, spawn};
use tracing::*;
use twamp_control::request_tw_session::RequestTwSession;
use twamp_test::{
    twamp_test_unauth::TwampTestPacketUnauth,
    twamp_test_unauth_reflected::TwampTestPacketUnauthReflected,
};

#[derive(Debug)]
pub struct SessionReflector {
    socket: UdpSocket,
    dest: SocketAddr,
    refwait: u64,
}

impl SessionReflector {
    /// Sets up Session-Sender from a Request-TW-Session.
    pub async fn from_request_tw_session(value: RequestTwSession) -> Self {
        let socket_addr = SocketAddrV4::new(
            if value.receiver_address == Ipv4Addr::UNSPECIFIED {
                Ipv4Addr::UNSPECIFIED
            } else {
                value.receiver_address
            },
            value.receiver_port,
        );
        let udp_socket = UdpSocket::bind(socket_addr).await.unwrap();
        let dest = SocketAddr::new(IpAddr::V4(value.sender_address), value.sender_port);
        udp_socket.connect(dest).await.unwrap();
        debug!("Binded to {}/udp, recv only from {}", socket_addr, dest);
        Self {
            socket: udp_socket,
            dest,
            refwait: value.timeout,
        }
    }

    /// Starts reflecting TWAMP-Test packets indefinitely.
    pub async fn do_reflect(self) {
        let l = self.socket.local_addr().unwrap();
        let p = self.socket.peer_addr().unwrap();
        let sock = Arc::new(self.socket);
        debug!("Listening for pkts from {} on {}", p, l);
        let mut seq: u32 = 0;
        loop {
            let sock_clone = Arc::clone(&sock);
            let mut buf = [0u8; 1472]; // 1472 for max MTU. Even though we aren't setting padding
                                       // above 27. Still setting this big for now.
            let bytes_read = sock_clone.recv(&mut buf).await.unwrap();
            let recv_timestamp = TimeStamp::default();
            debug!("bytes read: {}", bytes_read);
            let (_rest, twamp_test_unauth) = TwampTestPacketUnauth::from_bytes((&buf, 0)).unwrap();
            debug!("Twamp-Test: {:?}", twamp_test_unauth);
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
                debug!("Sent reflected pkt of bytes: {}", len);
            });
            seq += 1;
        }
    }
}
