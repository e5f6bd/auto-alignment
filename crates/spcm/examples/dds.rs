use std::{thread::sleep, time::Duration};

use clap::Parser;
use spcm::Device;

#[derive(Parser)]
struct Opts {
    address: String,
}

fn main() -> anyhow::Result<()> {
    env_logger::builder().format_timestamp_nanos().init();
    let opts = Opts::parse();

    println!("Hello!");

    let device = Device::open(&opts.address)?;
    let card_type = device.card_type_str()?;
    println!("{card_type:?}");
    sleep(Duration::from_secs(1));

    Ok(())
}
