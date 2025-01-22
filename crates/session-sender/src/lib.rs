use anyhow::Result;
use deku::prelude::*;
use std::{
    net::{SocketAddr, SocketAddrV4},
    sync::Arc,
};
use tokio::{net::UdpSocket, spawn};
use tracing::*;
use twamp_test::twamp_test_unauth::TwampTestPacketUnauth;

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

    pub async fn send_it(&self, number_of_packets: u32) -> Result<()> {
        info!("Sending Twamp-Test packets to {}", self.dest);
        for i in 0..number_of_packets {
            let twamp_test = TwampTestPacketUnauth::new(i, 0, true);
            debug!("Twamp-Test: {:?}", twamp_test);
            let encoded = twamp_test.to_bytes().unwrap();
            let l = self.socket.local_addr().unwrap();
            let p = self.socket.peer_addr().unwrap();
            debug!("Sending pkt from {} to {}", l, p);
            let len = self.socket.send(&encoded[..]).await?;
            info!("Twamp-Test sent of bytes: {}", len);
        }
        Ok(())
    }

    pub async fn recv(&self, number_of_packets: u32) {
        let sock_clone = Arc::clone(&self.socket);
        let reflect_task = spawn(async move {
            let mut count: u32 = 1;
            loop {
                let mut buf = [0u8; 1024]; // Buffer to hold incoming packets
                let _ = sock_clone.recv(&mut buf).await.unwrap();
                debug!("Received reflected bytes: {}", buf.len());
                if count == number_of_packets {
                    break;
                }
                count += 1;
            }
        });
        reflect_task.await.unwrap()
    }
}

#[cfg(test)]
mod tests {}
