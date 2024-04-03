use std::net::Ipv4Addr;

use anyhow::Result;
use control_client::ControlClient;
use session_sender::SessionSender;

#[derive(Debug, Default)]
pub struct Controller {
    control_client: ControlClient,
    #[allow(dead_code)]
    session_sender: Option<SessionSender>,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            control_client: ControlClient::new(),
            session_sender: None,
        }
    }

    /// Informs `Control-Client` to establish TCP connection with provided
    /// `server_addr` and negotiate a TWAMP session. The `Controller` does
    /// not walk `Control-Client` through the TWAMP-Control communication.
    /// That is up to `Control-Client` to handle.
    pub async fn connect(&mut self, server_addr: Ipv4Addr) -> Result<()> {
        self.control_client.connect(server_addr).await?;
        Ok(())
    }
}
