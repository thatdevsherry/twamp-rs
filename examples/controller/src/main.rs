pub mod controller;

use std::net::{Ipv4Addr, SocketAddrV4};
use std::process;

use anyhow::Result;
use clap::Parser;
use tracing::*;

use controller::Controller;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    server: Ipv4Addr,
}

async fn try_main() -> Result<()> {
    let args = Args::parse();
    let controller = Controller::new();
    info!("Controller initialized");

    controller.connect(args.server).await?;
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
