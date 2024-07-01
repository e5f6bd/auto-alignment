use std::io::stdin;

use anyhow::bail;
use clap::Parser;
use spcm::{CardMode, ClockMode, DdsCommand, Device, ExtendedFeature, M2Command, TriggerMask};

#[derive(Parser)]
struct Opts {
    address: String,
}

fn main() -> anyhow::Result<()> {
    println!("{}", std::backtrace::Backtrace::capture());

    env_logger::builder().format_timestamp_nanos().init();
    let opts = Opts::parse();

    println!("Hello!");

    let mut device = Device::open(&opts.address)?;

    println!("Serial number: {:?}", device.serial_no());

    // card type
    println!(
        "Card type: {:?} ({:?})",
        device.card_type()?,
        device.card_type_str()?
    );

    let function = device.function_type()?;
    println!("Function type of the card: {function:?}");
    if !matches!(function, spcm::CardFunctionType::AnalogOutput) {
        bail!("The card does not support analog output");
    }

    let extensions = device.extended_features()?;
    println!("Installed extended Options and Feautres: {extensions:?}");
    if !extensions.contains(ExtendedFeature::DDS) {
        bail!("The card does not support DDS");
    }

    let num_modules = device.num_modules()?;
    let num_channels_per_module = device.num_channels_per_module()?;
    println!("# modules: {num_modules}, # channels per module = {num_channels_per_module}");
    if num_modules * num_channels_per_module < 4 {
        bail!("Not enough number of channels");
    }

    device.enable_channels(0b1111)?;
    device.set_card_mode(CardMode::StdDds)?;
    device.set_trigger_or_mask(TriggerMask::empty())?;

    // device.set_clock_mode(ClockMode::ExternalReferenceClock)?;
    // device.set_reference_clock_frequency(125_000_000)?;
    // println!(
    //     "Current sample rate = {} Hz",
    //     device.reference_clock_frequency()?
    // );
    // device.set_sample_rate(1_250_000_000)?;
    // println!("Current sample rate = {} Sa/s", device.sample_rate()?);
    device.set_clock_mode(ClockMode::InternalPll)?;

    device.enable_clock_out(true)?;
    println!("Clock out enabled? = {}", device.clock_out_enabled()?);
    println!("Clock out frequency = {}", device.clock_out_frequency()?);

    for i in 0..4 {
        device.set_channel_amplitude(i, 500)?;
        device.enable_channel_out(i, true)?;
    }

    device.execute_command(M2Command::CardWriteSetup)?;

    // Reset
    device.execute_dds_command(DdsCommand::Reset)?;

    // Start
    // device.execute_commands([M2Command::CardStart, M2Command::CardEnableTrigger])?;
    device.execute_command(M2Command::CardStart)?;

    // Command sequence
    for step in 0..18 {
        // Press enter to reflect changes
        println!("Press Enter to send trigger ({step}/15)");
        stdin().read_line(&mut String::new())?;
        for i in 0..4 {
            let index = if i == 0 { 0 } else { 19 + i };
            let mut core = device.dds_core_mut(index);
            core.set_amplitude(1.0)?;
            core.set_frequency(1. * 1e6)?;
            core.set_phase(if i == 0 { 20. * step as f64 } else { 0. })?;

            println!(
                "Generated signal at core {index:2}: frequency = {:10.30} Hz, phase = {} degree, and amplitude = {}", 
                core.frequency()?, core.phase()?, core.amplitude()?,
            );
        }
        // device.set_dds_trigger_source(DdsTriggerSource::None)?;
        device.execute_dds_command(DdsCommand::ExecuteAtTrigger)?;
        // Write
        device.execute_dds_command(DdsCommand::WriteToCard)?;
        // Trigger
        device.execute_command(M2Command::CardForceTrigger)?;
    }

    device.execute_command(M2Command::CardStop)?;

    Ok(())
}
