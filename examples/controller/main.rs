pub mod controller;

use std::net::Ipv4Addr;
use std::process;

use anyhow::Result;
use clap::Parser;
use tracing::*;

use controller::Controller;
use twamp_rs::twamp_control::constants::TWAMP_CONTROL_WELL_KNOWN_PORT;
use twamp_rs::twamp_test::constants::TWAMP_TEST_WELL_KNOWN_PORT;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, help = "IP address of Responder.")]
    responder_addr: Ipv4Addr,

    #[arg(
        long,
        default_value_t = TWAMP_CONTROL_WELL_KNOWN_PORT,
        help = "Port on which Responder is listening for TWAMP-Control.")]
    responder_port: u16,

    #[arg(long, help = "IP address of Controller.", default_value = "0.0.0.0")]
    controller_addr: Ipv4Addr,

    #[arg(
        long,
        default_value = "0",
        help = "Port for Session-Sender to bind to. Delegates to OS if not provided."
    )]
    controller_test_port: u16,

    #[arg(
        long,
        default_value_t = TWAMP_TEST_WELL_KNOWN_PORT,
        help = "Port that Session-Reflector should listen on."
    )]
    responder_reflect_port: u16,

    #[arg(
        long,
        default_value = "10",
        help = "Number of TWAMP-Test packets to reflect."
    )]
    number_of_test_packets: u32,

    #[arg(
        long,
        default_value = "900",
        help = "Timeout (seconds) used in Request-TW-Session."
    )]
    timeout: u64,

    #[arg(
        long,
        default_value = "5",
        help = "Duration (seconds) to wait before sending Stop-Sessions after test pkts are sent"
    )]
    stop_session_sleep: u64,
}

async fn try_main() -> Result<()> {
    let args = Args::parse();
    let controller = Controller::new();
    info!("Controller initialized");

    controller
        .do_twamp(
            args.responder_addr,
            args.responder_port,
            args.controller_addr,
            args.controller_test_port,
            args.responder_reflect_port,
            args.number_of_test_packets,
            args.timeout,
            args.stop_session_sleep,
        )
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    if let Err(e) = try_main().await {
        error!("Error: {:#?}", e);
        process::exit(1)
    }
}
