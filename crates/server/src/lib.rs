use anyhow::Result;
use deku::prelude::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::*;
use twamp_control::accept_session::AcceptSession;
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
                    self.send_accept_session().await?;
                }
            }
        }

        Ok(())
    }

    pub async fn send_accept_session(&mut self) -> Result<AcceptSession> {
        info!("Sending Accept-Session");
        let accept_session = AcceptSession::new(Accept::Ok, 0);
        debug!("Accept-Session: {:?}", accept_session);
        let encoded = accept_session.to_bytes().unwrap();
        self.socket.write_all(&encoded[..]).await?;
        debug!("Sent Accept-Session");
        Ok(accept_session)
    }

    pub async fn send_server_start(&mut self) -> Result<ServerStart> {
        info!("Sending Server-Start");
        let server_start = ServerStart::default();
        debug!("Server-Start: {:?}", server_start);
        let encoded = server_start.to_bytes().unwrap();
        self.socket.write_all(&encoded[..]).await?;
        info!("Sent Server-Start");
        Ok(server_start)
    }

    pub async fn send_server_greeting(&mut self) -> Result<ServerGreeting> {
        info!("Sending ServerGreeting");
        let server_greeting = ServerGreeting::default();
        debug!("ServerGreeting: {:?}", server_greeting);
        let encoded = server_greeting.to_bytes().unwrap();
        self.socket.write_all(&encoded[..]).await?;
        info!("Sent ServerGreeting");
        Ok(server_greeting)
    }

    pub async fn read_set_up_response(&mut self, buf: &[u8]) -> Result<SetUpResponse> {
        info!("Reading Set-Up-Response");
        let (_rest, set_up_response) = SetUpResponse::from_bytes((&buf, 0)).unwrap();
        debug!("Set-Up-Response: {:?}", set_up_response);
        info!("Read Set-Up-Response");
        Ok(set_up_response)
    }

    pub async fn read_request_tw_session(&mut self, buf: &[u8]) -> Result<RequestTwSession> {
        debug!("Reading Request-TW-Session");
        let (_rest, request_tw_session) = RequestTwSession::from_bytes((&buf, 0)).unwrap();
        debug!("Request-TW-Session: {:?}", request_tw_session);
        info!("Read Request-TW-Session");
        Ok(request_tw_session)
    }
}
