use anyhow::Result;
use deku::prelude::*;
use std::mem::size_of;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::*;
use twamp_control::accept_session::AcceptSession;
use twamp_control::constants::TWAMP_CONTROL_WELL_KNOWN_PORT;
use twamp_control::request_tw_session::RequestTwSession;
use twamp_control::security_mode::Mode;
use twamp_control::server_greeting::ServerGreeting;
use twamp_control::server_start::ServerStart;
use twamp_control::set_up_response::SetUpResponse;

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

    /// [Server Greeting](ServerGreeting) received from Server.
    pub server_greeting: Option<ServerGreeting>,

    /// [Server-Start](ServerStart) received from Server.
    pub server_start: Option<ServerStart>,
}

impl ControlClient {
    /// Initiates TCP connection and starts the [TWAMP-Control](twamp_control) protocol with
    /// Server, handling communication until the test ends or connection is killed/stopped.
    pub async fn connect(&mut self, server_addr: Ipv4Addr) -> Result<()> {
        let socket_addr = SocketAddrV4::new(server_addr, TWAMP_CONTROL_WELL_KNOWN_PORT);
        let stream = TcpStream::connect(socket_addr).await?;
        self.stream = Some(stream);
        self.server_greeting = Some(self.read_server_greeting().await?);
        //self.control_client.read_mode().await?;
        self.send_set_up_response().await?;
        self.server_start = Some(self.read_server_start().await?);
        self.send_request_tw_session().await?;
        self.read_accept_session().await?;
        Ok(())
    }

    pub async fn read_accept_session(&mut self) -> Result<AcceptSession> {
        let mut buf = [0; size_of::<AcceptSession>()];
        info!("Reading Accept-Session");
        self.stream.as_mut().unwrap().read(&mut buf).await?;
        let (_rest, accept_session) = AcceptSession::from_bytes((&buf, 0)).unwrap();
        debug!("Accept-Session: {:?}", accept_session);
        info!("Read Accept-Session");

        Ok(accept_session)
    }

    /// Reads from TWAMP-Control stream assuming the bytes to be received
    /// will be of a `ServerGreeting`. Converts those bytes into a `ServerGreeting`
    /// struct and returns it.
    pub async fn read_server_greeting(&mut self) -> Result<ServerGreeting> {
        let mut buf = [0; size_of::<ServerGreeting>()];
        info!("Reading ServerGreeting");
        self.stream.as_mut().unwrap().read(&mut buf).await?;
        let (_rest, server_greeting) = ServerGreeting::from_bytes((&buf, 0)).unwrap();
        debug!("Server greeting: {:?}", server_greeting);
        info!("Read ServerGreeting");
        Ok(server_greeting)
    }

    /// Reads from `TWAMP-Control` stream assuming the bytes to be received
    /// will be of a `ServerStart`. Converts those bytes into a `ServerStart`
    /// struct and returns it.
    pub async fn read_server_start(&mut self) -> Result<ServerStart> {
        let mut buf = [0; size_of::<ServerStart>()];
        info!("Reading Server-Start");
        self.stream.as_mut().unwrap().read(&mut buf).await?;
        let (_rest, server_start) = ServerStart::from_bytes((&buf, 0)).unwrap();
        debug!("Server-Start: {:?}", server_start);
        info!("Read Server-Start");
        Ok(server_start)
    }

    /// Creates a `SetUpResponse`, converts to bytes and sends it out on
    /// `TWAMP-Control`.
    pub async fn send_set_up_response(&mut self) -> Result<()> {
        info!("Preparing Set-Up-Response");
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

    /// Creates a `Request-Tw-Session`, converts to bytes and sends it out on
    /// `TWAMP-Control`.
    pub async fn send_request_tw_session(&mut self) -> Result<()> {
        info!("Preparing Request-TW-Session");
        let request_tw_session = RequestTwSession::from(self.stream.as_ref().unwrap());
        debug!("request-tw-session: {:?}", request_tw_session);
        let encoded = request_tw_session.to_bytes().unwrap();
        self.stream
            .as_mut()
            .unwrap()
            .write_all(&encoded[..])
            .await?;
        info!("Request-TW-Session sent");
        Ok(())
    }
}

impl Default for ControlClient {
    /// Construct an empty `ControlClient` with no context.
    fn default() -> Self {
        ControlClient {
            stream: None,
            server_greeting: None,
            server_start: None,
        }
    }
}
