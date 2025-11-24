use std::{net::SocketAddrV4, time::Duration};
use thiserror::Error;
use tokio::{
    net::{TcpStream, UdpSocket},
    select, spawn,
    sync::oneshot,
    time::sleep,
    try_join,
};
use tracing::*;
use twamp_rs::twamp_control::RequestTwSession;
use twamp_rs::{server::Server, session_reflector::SessionReflectorError};
use twamp_rs::{server::ServerError, session_reflector::SessionReflector};

#[derive(Error, Debug)]
pub enum ResponderError {
    #[error("Server encountered an error.")]
    ServerError(#[from] ServerError),

    #[error("Session-Reflector encountered an error.")]
    SessionReflectorError(#[from] SessionReflectorError),
}

#[derive(Debug)]
pub struct Responder {
    server: Server,
}

impl Responder {
    pub fn new(socket: TcpStream) -> Self {
        Responder {
            server: Server::new(socket),
        }
    }

    pub async fn handle_controller(mut self, refwait: u16) -> Result<(), ResponderError> {
        debug!("in handle controller");
        // the port that was requested by Control-Client.
        let (req_tw_tx, req_tw_rx) = oneshot::channel::<RequestTwSession>();
        let (ref_port_tx, ref_port_rx) = oneshot::channel::<u16>();
        let (start_ack_tx, start_ack_rx) = oneshot::channel::<()>();
        let (stop_sessions_tx, stop_sessions_rx) = oneshot::channel::<()>();
        let (timeout_tx, timeout_rx) = oneshot::channel::<u64>();
        let task_server = spawn(async move {
            self.server
                .handle_control_client(
                    req_tw_tx,
                    ref_port_rx,
                    start_ack_tx,
                    stop_sessions_tx,
                    timeout_tx,
                )
                .await
        });
        let task_session_reflector = spawn(async move {
            let req_tw_session = req_tw_rx.await.unwrap();
            let session_sender_addr =
                SocketAddrV4::new(req_tw_session.sender_address, req_tw_session.sender_port);
            debug!(
                "Binding to: {}:{}/udp",
                req_tw_session.receiver_address, req_tw_session.receiver_port
            );
            let mut udp_socket_result = UdpSocket::bind(SocketAddrV4::new(
                req_tw_session.receiver_address,
                req_tw_session.receiver_port,
            ))
            .await;
            if udp_socket_result.is_err() {
                let reflector_addr_new = SocketAddrV4::new(req_tw_session.receiver_address, 0);
                udp_socket_result = UdpSocket::bind(reflector_addr_new).await;
                debug!(
                    "Requested port {} not available, suggesting new available port: {}/udp",
                    req_tw_session.receiver_port,
                    udp_socket_result
                        .as_ref()
                        .unwrap()
                        .local_addr()
                        .unwrap()
                        .port()
                );
            }
            let udp_socket = udp_socket_result.unwrap();
            let local_addr_port = udp_socket.local_addr().unwrap().port();
            ref_port_tx.send(local_addr_port).unwrap();

            // Wait for signal to start reflecting.
            start_ack_rx.await.unwrap();

            let session_reflector =
                SessionReflector::new(udp_socket, session_sender_addr, refwait).await?;
            let (reflect_abort_tx, reflect_abort_rx) = oneshot::channel::<()>();
            let reflect_task = spawn(async move {
                let reflect_result = session_reflector.do_reflect();
                select! {
                    _ = reflect_result => {
                        debug!("REFWAIT expired.");
                    }
                    _ = reflect_abort_rx => {
                        debug!("Abort message received. Shutting down reflector.")
                    }
                }
            });

            select! {
                _ = reflect_task => {
                    debug!("Reflect task ended. Meaning REFWAIT expired.");
                }
                _ = stop_sessions_rx => {
                    debug!("Stop-Sessions received. Run until now+timeout");
                    let timeout = timeout_rx.await.unwrap();
                    debug!("Timeout: {}", timeout);
                    sleep(Duration::from_secs(timeout)).await;
                    reflect_abort_tx.send(()).unwrap();
                }
            }
            Ok::<_, SessionReflectorError>(())
        });
        let (server_response, session_reflector_response) =
            try_join!(task_server, task_session_reflector)
                .expect("server and session-reflector tasks should be joined");
        if let Err(e) = server_response {
            error!("{}", e);
            return Err(ResponderError::ServerError(e));
        }
        if let Err(e) = session_reflector_response {
            error!("{}", e);
            return Err(ResponderError::SessionReflectorError(e));
        }
        debug!("Server & Refector tasks ended successfully.");
        Ok(())
    }
}
