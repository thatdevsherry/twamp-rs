use anyhow::{anyhow, Result};
use deku::prelude::*;
use std::mem::size_of;
use std::net::IpAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::oneshot;
use tracing::*;
use twamp_control::accept::Accept;
use twamp_control::accept_session::AcceptSession;
use twamp_control::request_tw_session::RequestTwSession;
use twamp_control::security_mode::Mode;
use twamp_control::server_greeting::ServerGreeting;
use twamp_control::server_start::ServerStart;
use twamp_control::set_up_response::SetUpResponse;
use twamp_control::start_ack::StartAck;
use twamp_control::start_sessions::StartSessions;
use twamp_control::stop_sessions::StopSessions;

/// Control-Client is responsible for initiating and handling TWAMP-Control with a Server.
///
/// Responsibilites of Control-Client on TWAMP-Control are:
/// -   [Read Server Greeting](Self::read_server_greeting)
/// -   [Send Set-Up-Response](Self::send_set_up_response)
/// -   [Read Server-Start](Self::read_server_start)
/// -   [Send Request-TW-Session](Self::send_request_tw_session)
#[derive(Debug)]
pub struct ControlClient {
    /// TCP stream on which TWAMP-Control is being used.
    pub stream: Option<TcpStream>,
}

impl ControlClient {
    pub fn new() -> Self {
        Self { stream: None }
    }
    /// Initiates TCP connection and starts the [TWAMP-Control](twamp_control) protocol with
    /// Server, handling communication until the test ends or connection is killed/stopped.
    pub async fn do_twamp_control(
        &mut self,
        twamp_control: TcpStream,
        start_session_tx: oneshot::Sender<()>,
        reflector_port_tx: oneshot::Sender<u16>,
        responder_reflect_port: u16,
        controller_port: u16,
        reflector_timeout: u64,
        twamp_test_complete_rx: oneshot::Receiver<()>,
    ) -> Result<()> {
        self.stream = Some(twamp_control);
        self.read_server_greeting().await?;
        self.send_set_up_response().await?;
        self.read_server_start().await?;
        self.send_request_tw_session(responder_reflect_port, controller_port, reflector_timeout)
            .await?;
        let accept_session = self.read_accept_session().await?;
        if accept_session.accept != Accept::Ok {
            return Err(anyhow!("Did not receive Ok in Accept-Session"));
        };

        debug!("Responder provided port: {}", accept_session.port);
        reflector_port_tx.send(accept_session.port).unwrap();
        self.send_start_sessions().await?;
        let start_ack = self.read_start_ack().await?;
        if start_ack.accept != Accept::Ok {
            return Err(anyhow!("Start-Ack should be zero"));
        }
        start_session_tx.send(()).unwrap();
        // testing
        debug!(
            "Waiting for Session-Sender to complete, Control-Client will then send Stop-Sessions."
        );
        let _ = twamp_test_complete_rx.await;
        debug!("Received confirmation that TWAMP-Test is complete. Sending Stop-Sessions");
        self.send_stop_sessions().await?;
        Ok(())
    }

    /// Reads from TWAMP-Control stream assuming the bytes to be received will be of a
    /// `ServerGreeting`. Converts those bytes into a `ServerGreeting` struct and returns it.
    pub async fn read_server_greeting(&mut self) -> Result<ServerGreeting> {
        let mut buf = [0; size_of::<ServerGreeting>()];
        info!("Reading ServerGreeting");
        self.stream.as_mut().unwrap().read_exact(&mut buf).await?;
        let (_rest, server_greeting) = ServerGreeting::from_bytes((&buf, 0)).unwrap();
        debug!("Server greeting: {:?}", server_greeting);
        info!("Done reading ServerGreeting");
        Ok(server_greeting)
    }

    /// Creates a `SetUpResponse`, converts to bytes and sends it out on `TWAMP-Control`.
    pub async fn send_set_up_response(&mut self) -> Result<()> {
        info!("Preparing to send Set-Up-Response");
        let set_up_response = SetUpResponse::new(Mode::Unauthenticated);
        debug!("Set-Up-Response: {:?}", set_up_response);
        let encoded = set_up_response.unwrap().to_bytes().unwrap();
        self.stream
            .as_mut()
            .unwrap()
            .write_all(&encoded[..])
            .await?;
        info!("Set-Up-Response sent");
        Ok(())
    }

    /// Reads from `TWAMP-Control` stream assuming the bytes to be received will be of a
    /// `ServerStart`. Converts those bytes into a `ServerStart` struct and returns it.
    pub async fn read_server_start(&mut self) -> Result<ServerStart> {
        let mut buf = [0; size_of::<ServerStart>()];
        info!("Reading Server-Start");
        self.stream.as_mut().unwrap().read_exact(&mut buf).await?;
        let (_rest, server_start) = ServerStart::from_bytes((&buf, 0)).unwrap();
        debug!("Server-Start: {:?}", server_start);
        info!("Done reading Server-Start");
        Ok(server_start)
    }

    /// Creates a `Request-Tw-Session`, converts to bytes and sends it out on `TWAMP-Control`.
    pub async fn send_request_tw_session(
        &mut self,
        session_reflector_port: u16,
        controller_port: u16,
        timeout: u64,
    ) -> Result<RequestTwSession> {
        info!("Preparing to send Request-TW-Session");
        let stream = self.stream.as_ref().unwrap();
        let sender_address = match stream.local_addr().unwrap().ip() {
            IpAddr::V4(ip) => ip,
            IpAddr::V6(ip) => panic!("da hail did v6 come from: {ip}"),
        };
        let receiver_address = match stream.peer_addr().unwrap().ip() {
            IpAddr::V4(ip) => ip,
            IpAddr::V6(ip) => panic!("da hail did v6 come from: {ip}"),
        };
        debug!(
            "Request-TW-Session reflector port: {}",
            session_reflector_port
        );
        let request_tw_session = RequestTwSession::new(
            sender_address,
            controller_port,
            receiver_address,
            session_reflector_port,
            None,
            timeout,
        );
        debug!("request-tw-session: {:?}", request_tw_session);
        let encoded = request_tw_session.to_bytes().unwrap();
        self.stream
            .as_mut()
            .unwrap()
            .write_all(&encoded[..])
            .await?;
        info!("Request-TW-Session sent");
        Ok(request_tw_session)
    }

    /// Reads from `TWAMP-Control` stream assuming the bytes to be received will be of a
    /// `AcceptSession`. Converts those bytes into a `AcceptSession` struct and returns it.
    pub async fn read_accept_session(&mut self) -> Result<AcceptSession> {
        let mut buf = [0; size_of::<AcceptSession>()];
        info!("Reading Accept-Session");
        self.stream.as_mut().unwrap().read_exact(&mut buf).await?;
        let (_rest, accept_session) = AcceptSession::from_bytes((&buf, 0)).unwrap();
        debug!("Accept-Session: {:?}", accept_session);
        info!("Read Accept-Session");

        Ok(accept_session)
    }

    /// Creates a `Start-Sessions`, converts to bytes and sends it out on `TWAMP-Control`.
    pub async fn send_start_sessions(&mut self) -> Result<()> {
        info!("Preparing to send Start-Sessions");
        let start_sessions = StartSessions::new();
        debug!("Start-Sessions: {:?}", start_sessions);
        let encoded = start_sessions.to_bytes().unwrap();
        self.stream
            .as_mut()
            .unwrap()
            .write_all(&encoded[..])
            .await?;
        info!("Start-Sessions sent");
        Ok(())
    }

    /// Reads from `TWAMP-Control` stream assuming the bytes to be received will be of a
    /// `Start-Ack`. Converts those bytes into a `Start-Ack` struct and returns it.
    pub async fn read_start_ack(&mut self) -> Result<StartAck> {
        let mut buf = [0; size_of::<StartAck>()];
        info!("Reading Start-Ack");
        self.stream.as_mut().unwrap().read_exact(&mut buf).await?;
        let (_rest, start_ack) = StartAck::from_bytes((&buf, 0)).unwrap();
        debug!("Start-Ack: {:?}", start_ack);
        info!("Done reading Start-Ack");
        Ok(start_ack)
    }

    /// Creates a `Stop-Sessions`, converts to bytes and sends it out on `TWAMP-Control`.
    pub async fn send_stop_sessions(&mut self) -> Result<()> {
        info!("Preparing to send Stop-Sessions");
        let stop_sessions = StopSessions::new(Accept::Ok);
        debug!("Stop-Sessions: {:?}", stop_sessions);
        let encoded = stop_sessions.to_bytes().unwrap();
        self.stream
            .as_mut()
            .unwrap()
            .write_all(&encoded[..])
            .await?;
        info!("Stop-Sessions sent");
        Ok(())
    }
}

impl Default for ControlClient {
    /// Construct an empty `ControlClient` with no context.
    fn default() -> Self {
        ControlClient { stream: None }
    }
}
