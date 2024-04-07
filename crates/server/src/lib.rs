use std::mem::size_of;

use anyhow::Result;
use bincode::Options;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::*;
use twamp_control::constants::Messages;
use twamp_control::request_tw_session::RequestTwSession;
use twamp_control::server_start::{Accept, ServerStart};
use twamp_control::{server_greeting::ServerGreeting, set_up_response::SetUpResponse};

/// Server is responsible for handling incoming [TWAMP-Control](twamp_control) connection from a
/// Control-Client.
#[derive(Debug)]
pub struct Server {
    socket: TcpStream,
    server_greeting: Option<ServerGreeting>,
    set_up_response: Option<SetUpResponse>,
    server_start: Option<ServerStart>,
    request_tw_session: Option<RequestTwSession>,
}

impl Server {
    fn up_next(&self) -> Messages {
        if self.set_up_response.is_none() {
            Messages::SetUpResponse
        } else if self.request_tw_session.is_none() {
            Messages::RequestTwSession
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
            request_tw_session: None,
        }
    }

    pub async fn handle_control_client(&mut self) -> Result<()> {
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
                    self.set_up_response = Some(self.read_set_up_response(&buf).await?);
                    self.server_start = Some(self.send_server_start().await?);
                }
                Messages::RequestTwSession => {
                    self.request_tw_session = Some(self.read_request_tw_session(&buf).await?);
                }
            }
        }

        Ok(())
    }

    pub async fn send_server_start(&mut self) -> Result<ServerStart> {
        let server_start = ServerStart::new(Accept::Ok);
        let encoded = bincode::DefaultOptions::new()
            // TODO: might wanna check simple_endian to encode endianness
            // to the data type.
            .with_big_endian()
            .with_fixint_encoding()
            .serialize(&server_start)?;
        self.socket.write_all(&encoded[..]).await?;
        debug!("Server start sent");
        Ok(server_start)
    }

    pub async fn send_server_greeting(&mut self) -> Result<ServerGreeting> {
        let server_greeting = ServerGreeting::new();
        let encoded = bincode::DefaultOptions::new()
            // TODO: might wanna check simple_endian to encode endianness
            // to the data type.
            .with_big_endian()
            .with_fixint_encoding()
            .serialize(&server_greeting)?;
        self.socket.write_all(&encoded[..]).await?;
        debug!("Server greeting sent");
        Ok(server_greeting)
    }

    pub async fn read_set_up_response(&mut self, buf: &[u8]) -> Result<SetUpResponse> {
        let size = size_of::<SetUpResponse>();
        debug!("reading setup response");
        let set_up_response: SetUpResponse = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .with_big_endian()
            .deserialize(&buf[..size])?;
        debug!("received Set-Up-Response");
        Ok(set_up_response)
    }

    pub async fn read_request_tw_session(&mut self, buf: &[u8]) -> Result<RequestTwSession> {
        let size = size_of::<RequestTwSession>();
        debug!("reading Request-TW-Session");
        let request_tw_session: RequestTwSession = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .with_big_endian()
            .deserialize(&buf[..size])?;
        debug!("received Request-TW-Session: {:?}", request_tw_session);
        Ok(request_tw_session)
    }
}
