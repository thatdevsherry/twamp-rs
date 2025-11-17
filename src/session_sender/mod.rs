use crate::timestamp::TimeStamp;
use crate::twamp_test::{TwampTestPacketUnauth, TwampTestPacketUnauthReflected};
use anyhow::Result;
use deku::prelude::*;
use std::{
    net::{SocketAddr, SocketAddrV4},
    sync::Arc,
};
use tokio::{net::UdpSocket, spawn, sync::Mutex};
use tracing::*;

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
            trace!("Twamp-Test: {:?}", twamp_test);
            let encoded = twamp_test.to_bytes().unwrap();
            let l = self.socket.local_addr().unwrap();
            let p = self.socket.peer_addr().unwrap();
            trace!("Sending pkt from {} to {}", l, p);
            let len = self.socket.send(&encoded[..]).await?;
            trace!("Twamp-Test sent of bytes: {}", len);
        }
        Ok(())
    }

    pub async fn recv(
        &self,
        number_of_packets: u32,
        reflected_pkts_shared: Arc<Mutex<Vec<(TwampTestPacketUnauthReflected, TimeStamp)>>>,
    ) {
        let sock_clone = Arc::clone(&self.socket);
        let reflect_task = spawn(async move {
            let mut count: u32 = 1;
            loop {
                let mut buf = [0u8; 1024]; // Buffer to hold incoming packets
                let bytes_read = sock_clone.recv(&mut buf).await.unwrap();
                trace!("Bytes read: {}", bytes_read);
                let (_rest, reflected_pkt) =
                    TwampTestPacketUnauthReflected::from_bytes((&buf, 0)).unwrap();
                trace!("Received reflected pkt: {:?}", reflected_pkt);
                //debug!("Adding reflector pkt to vec");
                let mut acquired_vec = reflected_pkts_shared.lock().await;
                //debug!("Added reflector pkt to vec");
                acquired_vec.push((reflected_pkt, TimeStamp::default()));
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
