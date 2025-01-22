pub mod responder;

use anyhow::Result;
use clap::Parser;
use responder::Responder;
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    process,
};
use tokio::net::{TcpListener, TcpStream};
use tokio::task;
use tracing::*;
use twamp_control::constants::TWAMP_CONTROL_WELL_KNOWN_PORT;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1")]
    addr: Ipv4Addr,

    #[arg(short, long, default_value_t = TWAMP_CONTROL_WELL_KNOWN_PORT)]
    port: u16,
}

async fn handle_client(socket: TcpStream) {
    let mut responder = Responder::new(socket);
    debug!("Responder created: {:?}", responder);
    let _ = responder.handle_controller().await;
}

async fn try_main() -> Result<()> {
    let args = Args::parse();
    let socket_addr = SocketAddrV4::new(args.addr, args.port);
    // listen for clients, then open up tokio task for each one.
    debug!("Attempting to bind to: {}", socket_addr);

    let listener = TcpListener::bind(socket_addr).await?;
    debug!("Successfully binded to: {}", listener.local_addr()?);

    info!("Listening TWAMP-Control on: {}/tcp", listener.local_addr()?);
    loop {
        let (socket, client_addr) = listener.accept().await?;
        info!("Received connection from {}", client_addr);
        task::spawn(async move {
            handle_client(socket).await;
        });
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    if let Err(e) = try_main().await {
        error!("Error: {:#?}", e);
        process::exit(1)
    }
}
