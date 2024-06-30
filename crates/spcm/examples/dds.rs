use anyhow::bail;
use clap::Parser;
use spcm::{Device, ExtendedFeature};

#[derive(Parser)]
struct Opts {
    address: String,
}

fn main() -> anyhow::Result<()> {
    env_logger::builder().format_timestamp_nanos().init();
    let opts = Opts::parse();

    println!("Hello!");

    let device = Device::open(&opts.address)?;

    // card type
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

    let extensions = device.extended_features()?;
    println!("Installed extended Options and Feautres: {extensions:?}");
    if !extensions.contains(ExtendedFeature::Dds) {
        bail!("The card does not support DDS");
    }

    Ok(())
}
