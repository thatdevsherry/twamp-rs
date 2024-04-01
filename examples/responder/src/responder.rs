use anyhow::Result;
use server::Server;
use session_reflector::SessionReflector;
use tokio::net::TcpStream;
use tracing::*;

#[derive(Debug)]
pub struct Responder {
    server: Server,
    session_reflector: Option<SessionReflector>,
}

impl Responder {
    pub fn new(socket: TcpStream) -> Self {
        Responder {
            server: Server::new(socket),
            session_reflector: None,
        }
    }

    pub async fn handle_controller(&mut self) -> Result<()> {
        debug!("in handle controller");
        self.server.start_twamp_control().await?;
        //self.server.send_server_greeting().await?;
        //self.server.read_set_up_response().await?;
        Ok(())
    }
}
