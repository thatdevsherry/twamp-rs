use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Result;
use server::Server;
use session_reflector::SessionReflector;
use tokio::spawn;
use tokio::{net::TcpStream, sync::oneshot};
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
        let (stop_session_cmd_tx, stop_session_cmd_rx) = oneshot::channel::<()>();
        debug!("in handle controller");
        let control_client_result = self.server.handle_control_client().await;
        match control_client_result {
            Ok(()) => (),
            Err(_) => ()
        }
        Ok(())
    }
}
