use anyhow::Result;
use std::net::Ipv4Addr;
use tokio::net::TcpStream;

pub struct ControlClient;

impl ControlClient {
    pub fn new() -> Self {
        ControlClient
    }

    pub async fn connect(self, server_addr: Ipv4Addr) -> Result<()> {
        let socket_addr = format!("{}:862", server_addr);
        println!("Connecting to {socket_addr}");
        //let _stream = TcpStream::connect(socket_addr).await;
        Ok(())
    }
}
