use std::mem::size_of;

use anyhow::Result;
use bincode::Options;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::*;
use twamp_control::constants::Messages;
use twamp_control::server_start::ServerStart;
use twamp_control::{
    security_mode::Mode, server_greeting::ServerGreeting, set_up_response::SetUpResponse,
};

#[derive(Debug)]
pub struct Server {
    socket: TcpStream,
    server_greeting: Option<ServerGreeting>,
    set_up_response: Option<SetUpResponse>,
    server_start: Option<ServerStart>,
}

impl Server {
    fn up_next(&self) -> Messages {
        if self.set_up_response.is_none() {
            Messages::SetUpResponse
        } else {
            panic!("dunno what to expect");
        }
    }
    pub fn new(socket: TcpStream) -> Self {
        Server {
            socket,
            server_greeting: None,
            set_up_response: None,
            server_start: None,
        }
    }

    pub async fn start_twamp_control(&mut self) -> Result<()> {
        self.server_greeting = Some(self.send_server_greeting().await?);

        // Testing out what is a good way to write code for
        // reading/writing. Using a loop thingy here and testing
        // definitely-not-dry approach on client side.
        loop {
            let mut buf = [0u8; 512];
            let bytes_read = self.socket.read(&mut buf).await?;
            debug!("bytes read: {}", bytes_read);

            if bytes_read == 0 {
                debug!("Control-Client closed connection");
                break;
            }

            match self.up_next() {
                Messages::SetUpResponse => {
                    self.read_set_up_response(&buf).await?;
                    self.send_server_start().await?;
                }
            }
        }
        Ok(())
    }

    pub async fn send_server_start(&mut self) -> Result<()> {
        let server_start = ServerStart::new();
        let encoded = bincode::DefaultOptions::new()
            // TODO: might wanna check simple_endian to encode endianness
            // to the data type.
            .with_big_endian()
            .with_fixint_encoding()
            .serialize(&server_start)?;
        self.socket.write(&encoded[..]).await?;
        debug!("Server start sent");
        Ok(())
    }

    pub async fn send_server_greeting(&mut self) -> Result<ServerGreeting> {
        let server_greeting = ServerGreeting::new(Mode::UnAuthenticated);
        let encoded = bincode::DefaultOptions::new()
            // TODO: might wanna check simple_endian to encode endianness
            // to the data type.
            .with_big_endian()
            .with_fixint_encoding()
            .serialize(&server_greeting)?;
        self.socket.write(&encoded[..]).await?;
        debug!("Server greeting sent");
        Ok(server_greeting)
    }

    pub async fn read_set_up_response(&mut self, buf: &[u8]) -> Result<()> {
        let size = size_of::<SetUpResponse>();
        debug!("reading setup response");
        self.set_up_response = Some(
            bincode::DefaultOptions::new()
                .with_fixint_encoding()
                .with_big_endian()
                .deserialize(&buf[..size])?,
        );
        debug!("received Set-Up-Response");
        Ok(())
    }
}
