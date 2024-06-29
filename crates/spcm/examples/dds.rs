use std::{thread::sleep, time::Duration};

use anyhow::bail;
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
    println!(
        "Card Type: {:?} ({:?})",
        device.card_type()?,
        device.card_type_str()?
    );
    let function = device.function_type()?;
    println!("Card Function Type: {function:?}");
    if !matches!(function, spcm::CardFunctionType::AnalogOutput) {
        bail!("The card does not support analog output");
    }
    sleep(Duration::from_secs(1));

    Ok(())
}
