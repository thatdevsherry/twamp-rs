use anyhow::Result;
use bincode::Options;
use std::mem::size_of;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::*;
use twamp_control::constants::TWAMP_CONTROL_WELL_KNOWN_PORT;
use twamp_control::security_mode::Mode;
use twamp_control::server_greeting::ServerGreeting;
use twamp_control::server_start::ServerStart;
use twamp_control::set_up_response::SetUpResponse;

pub struct ControlClient {
    stream: Option<TcpStream>,
    server_greeting: Option<ServerGreeting>,
    server_start: Option<ServerStart>,
}

impl ControlClient {
    /// Construct an empty `ControlClient` with no context.
    /// TODO: should probably init this by storing `server_addr` rather
    /// than with no context.
    pub fn new() -> Self {
        ControlClient {
            stream: None,
            server_greeting: None,
            server_start: None,
        }
    }

    /// The `ControlClient` is responsible for communicating with `Server` and
    /// should not be handled by the `Controller`. `ControlClient`'s logic for
    /// negotiating a session should be encapsulated within itself, and the
    /// `Controller` should only have access to `ControlClient`'s API for
    /// initiating TWAMP session negotiation using `TWAMP-Control`.
    pub async fn connect(&mut self, server_addr: Ipv4Addr) -> Result<()> {
        let socket_addr = SocketAddrV4::new(server_addr, TWAMP_CONTROL_WELL_KNOWN_PORT);
        let stream = TcpStream::connect(socket_addr).await?;
        self.stream = Some(stream);
        self.server_greeting = Some(self.read_server_greeting().await?);
        //self.control_client.read_mode().await?;
        self.send_set_up_response().await?;
        self.server_start = Some(self.read_server_start().await?);
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
                    .with_fixint_encoding()
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
        debug!("mode: {}", mode);
        if mode == 0u32 {
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
        self.stream.as_mut().unwrap().write(&encoded[..]).await?;
        debug!("Set-Up-Response sent");
        Ok(())
    }
}
