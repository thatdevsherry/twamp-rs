use crate::twamp_control::Accept;
use crate::twamp_control::AcceptSession;
use crate::twamp_control::ControlMessage;
use crate::twamp_control::RequestTwSession;
use crate::twamp_control::SecurityMode;
use crate::twamp_control::ServerStart;
use crate::twamp_control::StartAck;
use crate::twamp_control::StartSessions;
use crate::twamp_control::StopSessions;
use crate::twamp_control::{ServerGreeting, SetUpResponse};
use anyhow::Result;
use deku::prelude::*;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::oneshot;
use tracing::*;

/// Server is responsible for handling incoming [TWAMP-Control](crate::twamp_control) connection from a
/// [Control-Client](crate::control_client::ControlClient).
#[derive(Debug)]
pub struct Server {
    socket: TcpStream,
}

impl Server {
    pub fn new(socket: TcpStream) -> Self {
        Server { socket }
    }

    pub async fn handle_control_client(
        &mut self,
        req_tw_tx: oneshot::Sender<RequestTwSession>,
        ref_port_rx: oneshot::Receiver<u16>,
        start_ack_tx: oneshot::Sender<()>,
        stop_session_tx: oneshot::Sender<()>,
        timeout_tx: oneshot::Sender<u64>,
    ) -> Result<()> {
        self.send_server_greeting().await?;

        self.read_set_up_response().await?;
        self.send_server_start().await?;

        let request_tw_session = self.read_request_tw_session().await?;
        req_tw_tx.send(request_tw_session.clone()).unwrap();
        let final_port = ref_port_rx.await.unwrap();
        self.send_accept_session(final_port).await?;
        timeout_tx.send(request_tw_session.timeout).unwrap();

        self.read_start_sessions().await?;
        self.send_start_ack().await?;
        start_ack_tx.send(()).unwrap();

        self.read_stop_sessions().await?;
        stop_session_tx.send(()).unwrap();

        Ok(())
    }

    /// Creates a `ServerGreeting`, converts to bytes and sends it out on `TWAMP-Control`.
    pub async fn send_server_greeting(&mut self) -> Result<ServerGreeting> {
        info!("Sending ServerGreeting");
        let server_greeting = ServerGreeting::new(&[SecurityMode::Unauthenticated]);
        debug!("ServerGreeting: {:?}", server_greeting);
        let encoded = server_greeting.to_bytes().unwrap();
        self.socket.write_all(&encoded[..]).await?;
        info!("Sent ServerGreeting");
        Ok(server_greeting)
    }

    /// Reads from `TWAMP-Control` stream assuming the bytes to be received will be of a
    /// `Set-Up-Response`. Converts those bytes into a `Set-Up-Response` struct and returns it.
    pub async fn read_set_up_response(&mut self) -> Result<SetUpResponse> {
        let mut buf = [0; SetUpResponse::SERIALIZED_SIZE];
        info!("Reading Set-Up-Response");
        self.socket.read_exact(&mut buf).await?;
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
    pub async fn read_request_tw_session(&mut self) -> Result<RequestTwSession> {
        let mut buf = [0; RequestTwSession::SERIALIZED_SIZE];
        debug!("Reading Request-TW-Session");
        self.socket.read_exact(&mut buf).await?;
        let (_rest, request_tw_session) = RequestTwSession::from_bytes((&buf, 0)).unwrap();
        debug!("Request-TW-Session: {:?}", request_tw_session);
        info!("Read Request-TW-Session");
        Ok(request_tw_session)
    }

    /// Creates a `Accept-Session`, converts to bytes and sends it out on `TWAMP-Control`.
    pub async fn send_accept_session(&mut self, receiver_port: u16) -> Result<AcceptSession> {
        info!("Sending Accept-Session");
        let accept_session = AcceptSession::new(Accept::Ok, receiver_port, 0, 0);
        debug!("Accept-Session: {:?}", accept_session);
        let encoded = accept_session.to_bytes().unwrap();
        self.socket.write_all(&encoded[..]).await?;
        debug!("Sent Accept-Session");
        Ok(accept_session)
    }

    /// Reads from `TWAMP-Control` stream assuming the bytes to be received will be of a
    /// `Start-Sessions`. Converts those bytes into a `Start-Sessions` struct and returns it.
    pub async fn read_start_sessions(&mut self) -> Result<StartSessions> {
        let mut buf = [0; StartSessions::SERIALIZED_SIZE];
        debug!("Reading Start-Sessions");
        self.socket.read_exact(&mut buf).await?;
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
    pub async fn read_stop_sessions(&mut self) -> Result<StopSessions> {
        let mut buf = [0; StopSessions::SERIALIZED_SIZE];
        debug!("Reading Stop-Sessions");
        self.socket.read_exact(&mut buf).await?;
        let (_rest, stop_sessions) = StopSessions::from_bytes((&buf, 0)).unwrap();
        debug!("Stop-Sessions: {:?}", stop_sessions);
        info!("Read Stop-Sessions");
        Ok(stop_sessions)
    }
}
