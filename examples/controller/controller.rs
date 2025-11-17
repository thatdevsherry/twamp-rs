use core::f64;
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
    time::Duration,
};

use anyhow::Result;
use tokio::{
    net::{TcpStream, UdpSocket},
    select, spawn,
    sync::{Mutex, oneshot},
    time::sleep,
    try_join,
};
use tracing::*;
use twamp_rs::control_client::ControlClient;
use twamp_rs::session_sender::SessionSender;
use twamp_rs::timestamp::TimeStamp;
use twamp_rs::twamp_test::TwampTestPacketUnauthReflected;

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
        reflector_timeout: u64,
        stop_session_sleep: u64,
    ) -> Result<()> {
        let twamp_control =
            TcpStream::connect(SocketAddrV4::new(responder_addr, responder_port)).await?;
        let udp_socket =
            UdpSocket::bind(SocketAddrV4::new(controller_addr, controller_port)).await?;
        controller_port = udp_socket.local_addr().unwrap().port();

        let (start_session_tx, start_session_rx) = oneshot::channel::<()>();
        let (twamp_test_complete_tx, twamp_test_complete_rx) = oneshot::channel::<()>();
        let (reflector_port_tx, reflector_port_rx) = oneshot::channel::<u16>();
        let control_client_handle = spawn(async move {
            self.control_client
                .do_twamp_control(
                    twamp_control,
                    start_session_tx,
                    reflector_port_tx,
                    responder_reflect_port,
                    controller_port,
                    reflector_timeout,
                    twamp_test_complete_rx,
                )
                .await
                .unwrap();
        });
        let reflected_pkts_vec: Arc<Mutex<Vec<(TwampTestPacketUnauthReflected, TimeStamp)>>> =
            Arc::new(Mutex::new(Vec::new()));
        let reflected_pkts_vec_cloned = Arc::clone(&reflected_pkts_vec);
        let session_sender_handle = spawn(async move {
            // Wait until we get the Accept-Session's port.
            let final_port = reflector_port_rx.await.unwrap();
            debug!("Received reflector port: {}", final_port);
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
                info!("Sent all test packets");
            });
            let recv_task = spawn(async move {
                let _ = session_sender_recv
                    .recv(number_of_test_packets, reflected_pkts_vec_cloned)
                    .await;
                info!("Got back all test packets");
            });
            // wait for all test pkts to be sent.
            send_task.await.unwrap();

            select! {
                // If stop-session-sleep duration finishes before all pkts are received, drop
                // recv task and conclude.
                _ = sleep(Duration::from_secs(stop_session_sleep)) => (),
                // Ignore stop-session-sleep duration if session-sender got all test pkts before
                // duration.
                _ = recv_task => ()
            }
            // Inform Control-Client to send Stop-Sessions
            twamp_test_complete_tx.send(()).unwrap();
        });
        try_join!(control_client_handle, session_sender_handle).unwrap();
        debug!("Control-Client & Session-Sender tasks completed.");
        let acquired_vec = reflected_pkts_vec.lock().await;
        debug!("Reflected pkts len: {}", acquired_vec.len());
        get_metrics(&acquired_vec, number_of_test_packets as f64);
        Ok(())
    }
}

fn get_metrics(pkts: &Vec<(TwampTestPacketUnauthReflected, TimeStamp)>, total_sent: f64) {
    info!("Producing metrics");
    let received = pkts.len() as f64;
    let total_packets_lost = total_sent - received;
    let total_packets_sent = total_sent;
    let packet_loss = (total_packets_lost / total_packets_sent) * 100.0;
    info!("Packet loss: {}%", packet_loss.trunc());

    // RTT
    let mut rtt_pkts: Vec<f64> = vec![];
    let mut sender_to_reflector: Vec<f64> = vec![];
    let mut reflector_to_sender: Vec<f64> = vec![];
    for pkt in pkts {
        let t1: f64 = pkt.0.sender_timestamp.into();
        let t2: f64 = pkt.0.receive_timestamp.into();
        let t3: f64 = pkt.0.timestamp.into();
        let t4: f64 = pkt.1.into();

        let rtt = (t4 - t1) - (t3 - t2);
        let one_way_delay_sent = t2 - t1;
        let one_way_delay_recv = t4 - t3;
        rtt_pkts.push(rtt);
        sender_to_reflector.push(one_way_delay_sent);
        reflector_to_sender.push(one_way_delay_recv);
    }
    let rtt_avg = rtt_pkts.iter().sum::<f64>() / received;
    let sender_to_reflector_avg = sender_to_reflector.iter().sum::<f64>() / received;
    let reflector_to_sender_avg = reflector_to_sender.iter().sum::<f64>() / received;
    let rtt_min = rtt_pkts.iter().copied().fold(f64::INFINITY, f64::min);
    let rtt_max = rtt_pkts.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    info!("RTT (MIN): {:.2}ms", (rtt_min * 1e3));
    info!("RTT (MAX): {:.2}ms", (rtt_max * 1e3));
    info!("RTT (AVG): {:.2}ms", (rtt_avg * 1e3));
    info!(
        "OWD (Sender -> Reflector) (AVG): {:.2}ms",
        (sender_to_reflector_avg * 1e3)
    );
    info!(
        "OWD (Reflector -> Sender) (AVG): {:.2}ms",
        (reflector_to_sender_avg * 1e3)
    );

    let mut jitter = 0.0;
    for i in 1..rtt_pkts.len() {
        let rtt_diff = (rtt_pkts[i] - rtt_pkts[i - 1]).abs();
        jitter = jitter + (rtt_diff - jitter) / 16.0;
    }

    info!("Jitter: {:.2}ms", jitter * 1e3)
}
