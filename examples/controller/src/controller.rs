use std::net::Ipv4Addr;

use anyhow::Result;
use control_client::ControlClient;
use session_sender::SessionSender;

pub struct Controller {
    control_client: ControlClient,
    session_sender: Option<SessionSender>,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            control_client: ControlClient::new(),
            session_sender: None,
        }
    }

    pub async fn connect(mut self, server_ip: Ipv4Addr) -> Result<()> {
        self.control_client.connect(server_ip).await?;
        self.control_client.read_server_greeting().await?;
        //self.control_client.read_mode().await?;
        self.control_client.send_set_up_response().await?;
        self.control_client.read_server_start().await?;
        Ok(())
    }
}
