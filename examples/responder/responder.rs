use std::{net::SocketAddrV4, time::Duration};

use anyhow::Result;
use tokio::{
    net::{TcpStream, UdpSocket},
    select, spawn,
    sync::oneshot,
    time::sleep,
    try_join,
};
use tracing::*;
use twamp_rs::server::Server;
use twamp_rs::session_reflector::SessionReflector;
use twamp_rs::twamp_control::request_tw_session::RequestTwSession;

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

    pub async fn handle_controller(mut self, refwait: u16) -> Result<()> {
        debug!("in handle controller");
        // the port that was requested by Control-Client.
        let (req_tw_tx, req_tw_rx) = oneshot::channel::<RequestTwSession>();
        let (ref_port_tx, ref_port_rx) = oneshot::channel::<u16>();
        let (start_ack_tx, start_ack_rx) = oneshot::channel::<()>();
        let (stop_sessions_tx, stop_sessions_rx) = oneshot::channel::<()>();
        let (timeout_tx, timeout_rx) = oneshot::channel::<u64>();
        let server_handle = spawn(async move {
            self.server
                .handle_control_client(
                    req_tw_tx,
                    ref_port_rx,
                    start_ack_tx,
                    stop_sessions_tx,
                    timeout_tx,
                )
                .await
                .unwrap();
        });
        let session_reflector_handle = spawn(async move {
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
                debug!(
                    "Requested port not available, suggesting new port: {}/udp",
                    reflector_addr_new
                );
                udp_socket_result = UdpSocket::bind(reflector_addr_new).await;
            }
            let udp_socket = udp_socket_result.unwrap();
            udp_socket.connect(session_sender_addr).await.unwrap();
            debug!("hmm: {:?}", udp_socket.peer_addr());
            let local_addr_port = udp_socket.local_addr().unwrap().port();
            ref_port_tx.send(local_addr_port).unwrap();

            // Wait for signal to start reflecting.
            start_ack_rx.await.unwrap();

            let session_reflector = SessionReflector::new(udp_socket, refwait).await;
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
        });
        try_join!(server_handle, session_reflector_handle).unwrap();
        debug!("Server & Refector tasks ended successfully.");
        Ok(())
    }
}
