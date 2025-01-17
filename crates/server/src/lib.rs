use anyhow::Result;
use deku::prelude::*;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::*;
use twamp_control::accept::Accept;
use twamp_control::accept_session::AcceptSession;
use twamp_control::constants::Messages;
use twamp_control::request_tw_session::RequestTwSession;
use twamp_control::security_mode::Mode;
use twamp_control::server_start::ServerStart;
use twamp_control::start_ack::StartAck;
use twamp_control::start_sessions::StartSessions;
use twamp_control::stop_sessions::StopSessions;
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
    accept_session: Option<AcceptSession>,
    start_sessions: Option<StartSessions>,
    start_ack: Option<StartAck>,
    stop_sessions: Option<StopSessions>,
}

impl Server {
    fn up_next(&self) -> Messages {
        if self.set_up_response.is_none() {
            Messages::SetUpResponse
        } else if self.request_tw_session.is_none() {
            Messages::RequestTwSession
        } else if self.start_sessions.is_none() {
            Messages::StartSessions
        } else if self.start_ack.is_some() {
            Messages::StopSessions
        } else {
            panic!("Next message to expect should be defined");
        }
    }

    pub fn new(socket: TcpStream) -> Self {
        Server {
            socket,
            server_greeting: None,
            set_up_response: None,
            server_start: None,
            request_tw_session: None,
            accept_session: None,
            start_sessions: None,
            start_ack: None,
            stop_sessions: None,
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
                    self.accept_session = Some(self.send_accept_session().await?);
                }
                Messages::StartSessions => {
                    self.start_sessions = Some(self.read_start_sessions(&buf).await?);
                    self.start_ack = Some(self.send_start_ack().await?);
                }
                Messages::StopSessions => {
                    debug!("TODO: Impelement TWAMP-Test.");
                    self.stop_sessions = Some(self.read_stop_sessions(&buf).await?);
                }
            }
        }

        Ok(())
    }

    /// Creates a `ServerGreeting`, converts to bytes and sends it out on `TWAMP-Control`.
    pub async fn send_server_greeting(&mut self) -> Result<ServerGreeting> {
        info!("Sending ServerGreeting");
        let server_greeting = ServerGreeting::new(&[Mode::Unauthenticated]);
        debug!("ServerGreeting: {:?}", server_greeting);
        let encoded = server_greeting.to_bytes().unwrap();
        self.socket.write_all(&encoded[..]).await?;
        info!("Sent ServerGreeting");
        Ok(server_greeting)
    }

    /// Reads from `TWAMP-Control` stream assuming the bytes to be received will be of a
    /// `Set-Up-Response`. Converts those bytes into a `Set-Up-Response` struct and returns it.
    pub async fn read_set_up_response(&mut self, buf: &[u8]) -> Result<SetUpResponse> {
        info!("Reading Set-Up-Response");
        let (_rest, set_up_response) = SetUpResponse::from_bytes((&buf, 0)).unwrap();
        debug!("Set-Up-Response: {:?}", set_up_response);
        info!("Read Set-Up-Response");
        Ok(set_up_response)
    }

    /// Creates a `Server-Start`, converts to bytes and sends it out on `TWAMP-Control`.
    pub async fn send_server_start(&mut self) -> Result<ServerStart> {
        info!("Sending Server-Start");
        let server_start = ServerStart::new(Accept::Ok, Duration::new(123456, 789));
        debug!("Server-Start: {:?}", server_start);
        let encoded = server_start.to_bytes().unwrap();
        self.socket.write_all(&encoded[..]).await?;
        info!("Sent Server-Start");
        Ok(server_start)
    }

    /// Reads from `TWAMP-Control` stream assuming the bytes to be received will be of a
    /// `Request-TW-Session`. Converts those bytes into a `Request-TW-Session` struct and returns it.
    pub async fn read_request_tw_session(&mut self, buf: &[u8]) -> Result<RequestTwSession> {
        debug!("Reading Request-TW-Session");
        let (_rest, request_tw_session) = RequestTwSession::from_bytes((&buf, 0)).unwrap();
        debug!("Request-TW-Session: {:?}", request_tw_session);
        info!("Read Request-TW-Session");
        Ok(request_tw_session)
    }

    /// Creates a `Accept-Session`, converts to bytes and sends it out on `TWAMP-Control`.
    pub async fn send_accept_session(&mut self) -> Result<AcceptSession> {
        info!("Sending Accept-Session");
        let accept_session = AcceptSession::new(Accept::Ok, 0, 0, 0);
        debug!("Accept-Session: {:?}", accept_session);
        let encoded = accept_session.to_bytes().unwrap();
        self.socket.write_all(&encoded[..]).await?;
        debug!("Sent Accept-Session");
        Ok(accept_session)
    }

    /// Reads from `TWAMP-Control` stream assuming the bytes to be received will be of a
    /// `Start-Sessions`. Converts those bytes into a `Start-Sessions` struct and returns it.
    pub async fn read_start_sessions(&mut self, buf: &[u8]) -> Result<StartSessions> {
        debug!("Reading Start-Sessions");
        let (_rest, start_sessions) = StartSessions::from_bytes((&buf, 0)).unwrap();
        debug!("Start-Sessions: {:?}", start_sessions);
        info!("Read Start-Sessions");
        Ok(start_sessions)
    }

    /// Creates a `Start-Ack`, converts to bytes and sends it out on `TWAMP-Control`.
    pub async fn send_start_ack(&mut self) -> Result<StartAck> {
        info!("Sending Start-Ack");
        let start_ack = StartAck::new(Accept::Ok);
        debug!("Start-Ack: {:?}", start_ack);
        let encoded = start_ack.to_bytes().unwrap();
        self.socket.write_all(&encoded[..]).await?;
        info!("Sent Start-Ack");
        Ok(start_ack)
    }

    /// Reads from `TWAMP-Control` stream assuming the bytes to be received will be of a
    /// `Stop-Sessions`. Converts those bytes into a `Stop-Sessions` struct and returns it.
    pub async fn read_stop_sessions(&mut self, buf: &[u8]) -> Result<StopSessions> {
        debug!("Reading Stop-Sessions");
        let (_rest, stop_sessions) = StopSessions::from_bytes((&buf, 0)).unwrap();
        debug!("Stop-Sessions: {:?}", stop_sessions);
        info!("Read Stop-Sessions");
        Ok(stop_sessions)
    }
}
