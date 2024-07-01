use clap::Parser;
use spcm::{CardMode, Device, M2Command};

#[derive(Parser)]
struct Opts {
    address: String,
    frequency: u32,
    phase: u16,
}

fn main() -> anyhow::Result<()> {
    env_logger::builder().format_timestamp_nanos().init();
    let opts = Opts::parse();

    let mut device = Device::open(&opts.address)?;
    device.enable_channels(0b1111)?;
    device.set_card_mode(CardMode::StdDds)?;
    device.execute_command(M2Command::CardWriteSetup)?;
    let mut core = device.dds_core_mut(0);
    core.set_frequency_exact(opts.frequency)?;
    println!("{}", core.frequency_exact()?);
    core.set_phase_exact(opts.phase)?;
    println!("{}", core.phase_exact()?);

    Ok(())
}
