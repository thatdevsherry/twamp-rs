pub mod controller;

use std::net::Ipv4Addr;
use std::process;

use anyhow::Result;
use clap::Parser;

use controller::Controller;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    server: Ipv4Addr,
}

fn try_main() -> Result<()> {
    let args = Args::parse();
    let controller = Controller::new();
    println!("Controller initialized");

    // init connection with server 862/tcp
    controller.connect(args.server);
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {:#?}", e);
        process::exit(1)
    }
}
