use std::net::Ipv4Addr;

use anyhow::Result;
use control_client::ControlClient;
use session_sender::SessionSender;

pub struct Controller {
    control_client: ControlClient,
    session_sender: SessionSender,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            control_client: ControlClient::new(),
            session_sender: SessionSender::new(),
        }
    }

    pub async fn connect(self, server_ip: Ipv4Addr) -> Result<()> {
        let _ = self.control_client.connect(server_ip).await;
        Ok(())
    }
}
