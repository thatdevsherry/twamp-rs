use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
};

use anyhow::Result;
use control_client::ControlClient;
use session_sender::SessionSender;
use tokio::{
    net::{TcpStream, UdpSocket},
    spawn,
    sync::oneshot,
    try_join,
};
use tracing::*;

#[derive(Debug, Default)]
pub struct Controller {
    control_client: ControlClient,
    session_sender: Option<Arc<SessionSender>>,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            control_client: ControlClient::default(),
            session_sender: None,
        }
    }

    /// Informs `Control-Client` to establish TCP connection with provided
    /// `server_addr` and negotiate a TWAMP session. The `Controller` does
    /// not walk `Control-Client` through the TWAMP-Control communication.
    /// That is up to `Control-Client` to handle.
    pub async fn do_twamp(
        mut self,
        responder_addr: Ipv4Addr,
        responder_port: u16,
        controller_addr: Ipv4Addr,
        mut controller_port: u16,
        responder_reflect_port: u16,
        number_of_test_packets: u32,
    ) -> Result<()> {
        let twamp_control =
            TcpStream::connect(SocketAddrV4::new(responder_addr, responder_port)).await?;
        let udp_socket =
            UdpSocket::bind(SocketAddrV4::new(controller_addr, controller_port)).await?;
        controller_port = udp_socket.local_addr().unwrap().port();

        let (start_session_tx, start_session_rx) = oneshot::channel::<()>();
        let (twamp_test_complete_tx, twamp_test_complete_rx) = oneshot::channel::<()>();
        let (reflector_port_tx, reflector_port_rx) = oneshot::channel::<u16>();
        let (timeout_tx, timeout_rx) = oneshot::channel::<u64>();
        let control_client_handle = spawn(async move {
            self.control_client
                .do_twamp_control(
                    twamp_control,
                    start_session_tx,
                    reflector_port_tx,
                    responder_reflect_port,
                    controller_port,
                    timeout_tx,
                    twamp_test_complete_rx,
                )
                .await
                .unwrap();
        });
        let session_sender_handle = spawn(async move {
            // Wait until we get the Accept-Session's port.
            let final_port = reflector_port_rx.await.unwrap();
            debug!("Received reflector port: {}", final_port);
            let _reflector_timeout = timeout_rx.await.unwrap();
            udp_socket
                .connect(SocketAddrV4::new(responder_addr, final_port))
                .await
                .unwrap();
            // Wait until start-sessions is received
            start_session_rx.await.unwrap();
            debug!("Start-Session identified. Start Session-Sender.");
            self.session_sender = Some(Arc::new(
                SessionSender::new(
                    Arc::new(udp_socket),
                    SocketAddrV4::new(responder_addr, final_port),
                )
                .await,
            ));
            let session_sender_send = Arc::clone(self.session_sender.as_ref().unwrap());
            let session_sender_recv = Arc::clone(self.session_sender.as_ref().unwrap());
            let send_task = spawn(async move {
                let _ = session_sender_send.send_it(number_of_test_packets).await;
                debug!("SEND task END now pls");
            });
            let recv_task = spawn(async move {
                let _ = session_sender_recv.recv(number_of_test_packets).await;
                debug!("RECV task END now pls");
            });
            // wait for all test pkts to be sent.
            send_task.await.unwrap();
            // stop the recv task
            recv_task.abort();
            // Inform Control-Client to send Stop-Sessions
            twamp_test_complete_tx.send(()).unwrap();
        });
        try_join!(control_client_handle, session_sender_handle).unwrap();
        Ok(())
    }
}
