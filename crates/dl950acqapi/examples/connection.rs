use std::net::IpAddr;

use clap::Parser;
use dl950acqapi::{Api, WireType::Vxi11};

#[derive(Parser)]
struct Opts {
    ip_address: IpAddr,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    let api = Api::init()?;
    let handle = api.open_trigger_asnyc(Vxi11, &opts.ip_address.to_string())?;
    println!("Device connected!");
    Ok(())
}
