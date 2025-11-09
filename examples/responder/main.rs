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
use twamp_rs::twamp_control::constants::TWAMP_CONTROL_WELL_KNOWN_PORT;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1")]
    addr: Ipv4Addr,

    #[arg(short, long, default_value_t = TWAMP_CONTROL_WELL_KNOWN_PORT)]
    port: u16,

    #[arg(short, long, default_value = "900")]
    refwait: u16,
}

async fn handle_client(socket: TcpStream, refwait: u16) {
    let responder = Responder::new(socket);
    debug!("Responder created: {:?}", responder);
    responder.handle_controller(refwait).await.unwrap();
}

async fn try_main() -> Result<()> {
    let args = Args::parse();
    let socket_addr = SocketAddrV4::new(args.addr, args.port);
    debug!("Attempting to bind to: {}/tcp", socket_addr);

    let listener = TcpListener::bind(socket_addr).await?;
    debug!("Successfully binded to: {}/tcp", listener.local_addr()?);

    info!("Listening TWAMP-Control on: {}/tcp", listener.local_addr()?);
    loop {
        let (socket, client_addr) = listener.accept().await?;
        info!("Received connection from {}/tcp", client_addr);
        task::spawn(async move {
            handle_client(socket, args.refwait).await;
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
