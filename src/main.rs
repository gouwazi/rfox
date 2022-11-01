extern crate pretty_env_logger;
#[macro_use]
extern crate log;
mod config;
mod ingress;

use tokio::io;

use crate::config::RfoxConfig;
use tokio::net;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "./config.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    pretty_env_logger::init();

    let args = Args::parse();

    let rfox_config = RfoxConfig::read_from_config(&args.config).map_err(|e| {
        error!("Failed to process config {}", e);
        e
    })?;

    print!("Starting rfox with config: {:#?}", rfox_config);

    Ok(())
}
