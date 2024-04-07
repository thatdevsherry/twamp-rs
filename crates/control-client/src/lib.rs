use anyhow::Result;
use bincode::Options;
use std::mem::size_of;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::*;
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
        //self.send_request_tw_session().await?;
        Ok(())
    }

    /// Reads from TWAMP-Control stream assuming the bytes to be received
    /// will be of a `ServerGreeting`. Converts those bytes into a `ServerGreeting`
    /// struct and returns it.
    pub async fn read_server_greeting(&mut self) -> Result<ServerGreeting> {
        let mut buf = [0; size_of::<ServerGreeting>()];
        let server_greeting: ServerGreeting;
        loop {
            debug!("Reading ServerGreeting");
            let bytes_read = self.stream.as_mut().unwrap().read(&mut buf).await?;
            debug!("bytes_read: {}", bytes_read);
            if bytes_read == size_of::<ServerGreeting>() {
                server_greeting = bincode::DefaultOptions::new()
                    // deal with endianness when reading/writing to network
                    .with_fixint_encoding()
                    .with_big_endian()
                    .deserialize(&buf[..])?;
                break;
            }
        }
        debug!("Server greeting: {:?}", server_greeting);

        Ok(server_greeting)
    }

    /// Reads from `TWAMP-Control` stream assuming the bytes to be received
    /// will be of a `ServerStart`. Converts those bytes into a `ServerStart`
    /// struct and returns it.
    pub async fn read_server_start(&mut self) -> Result<ServerStart> {
        let mut buf = [0; size_of::<ServerStart>()];
        let server_start: ServerStart;
        loop {
            debug!("Reading Server-Start");
            let bytes_read = self.stream.as_mut().unwrap().read(&mut buf).await?;
            debug!("bytes_read: {}", bytes_read);
            if bytes_read == size_of::<ServerStart>() {
                debug!("{:?}", &buf[..]);
                server_start = bincode::DefaultOptions::new()
                    // deal with endianness when reading/writing to network
                    //.with_fixint_encoding()
                    .with_big_endian()
                    .deserialize(&buf[..])?;
                break;
            }
        }
        debug!("Server-Start: {:?}", server_start);
        Ok(server_start)
    }

    pub async fn read_mode(&mut self) -> Result<()> {
        let mode = self.server_greeting.as_ref().unwrap().mode;
        debug!("mode: {:?}", mode);
        if mode == Mode::Abort {
            // TODO: exit here
        }
        Ok(())
    }

    /// Creates a `SetUpResponse`, converts to bytes and sends it out on
    /// `TWAMP-Control`.
    pub async fn send_set_up_response(&mut self) -> Result<()> {
        debug!("Preparing Set-Up-Response");
        let set_up_response = SetUpResponse::new(Mode::UnAuthenticated);
        let encoded = bincode::DefaultOptions::new()
            // TODO: might wanna check simple_endian to encode endianness
            // to the data type.
            .with_big_endian()
            .with_fixint_encoding()
            .serialize(&set_up_response)?;
        debug!("set-up-response: {:?}", &encoded[..]);
        self.stream
            .as_mut()
            .unwrap()
            .write_all(&encoded[..])
            .await?;
        debug!("Set-Up-Response sent");
        Ok(())
    }

    /// Creates a `Request-Tw-Session`, converts to bytes and sends it out on
    /// `TWAMP-Control`.
    pub async fn send_request_tw_session(&mut self) -> Result<()> {
        debug!("Preparing Request-TW-Session");
        let request_tw_session = RequestTwSession::from(self.stream.as_ref().unwrap());
        let encoded = bincode::DefaultOptions::new()
            // TODO: might wanna check simple_endian to encode endianness
            // to the data type.
            .with_big_endian()
            .with_fixint_encoding()
            .serialize(&request_tw_session)?;
        debug!("request-tw-session: {:?}", &encoded[..]);
        self.stream
            .as_mut()
            .unwrap()
            .write_all(&encoded[..])
            .await?;
        debug!("Request-TW-Session sent");
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
