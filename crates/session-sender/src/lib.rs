use anyhow::Result;
use deku::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio::net::UdpSocket;
use tracing::*;
use twamp_control::request_tw_session::RequestTwSession;
use twamp_test::twamp_test_unauth::TwampTestPacketUnauth;

#[derive(Debug)]
pub struct SessionSender {
    pub socket: UdpSocket,
    pub dest: SocketAddr,
}

impl SessionSender {
    /// Sets up Session-Sender from a Request-TW-Session.
    pub async fn from_request_tw_session(value: RequestTwSession) -> Self {
        let socket_addr = SocketAddrV4::new(
            if value.sender_address == Ipv4Addr::UNSPECIFIED {
                Ipv4Addr::UNSPECIFIED
            } else {
                value.sender_address
            },
            value.sender_port,
        );
        let udp_socket = UdpSocket::bind(socket_addr).await.unwrap();
        let dest = SocketAddr::new(IpAddr::V4(value.receiver_address), value.receiver_port);
        udp_socket.connect(dest).await.unwrap();
        debug!("Binded to {}/udp", socket_addr);
        Self {
            socket: udp_socket,
            dest: SocketAddr::new(IpAddr::V4(value.receiver_address), value.receiver_port),
        }
    }

    pub async fn send_it(&self, number_of_packets: u32) -> Result<()> {
        info!("Sending Twamp-Test packets to {}", self.dest);
        for i in 0..number_of_packets {
            let twamp_test = TwampTestPacketUnauth::new(i, 0, true);
            debug!("Twamp-Test: {:?}", twamp_test);
            let encoded = twamp_test.to_bytes().unwrap();
            self.socket.send(&encoded[..]).await?;
            info!("Twamp-Test sent");
        }
        Ok(())
    }

    pub async fn recv(&self) {}
}

#[cfg(test)]
mod tests {}
