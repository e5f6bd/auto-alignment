use std::{net::IpAddr, thread::sleep, time::Duration};

use anyhow::bail;
use clap::Parser;
use dl950acqapi::{
    connection_mode::{TriggerAsync, TriggerMode, TriggerSync},
    Api, ChannelNumber, Handle,
    WireType::Vxi11,
};

#[derive(Parser)]
struct Opts {
    ip_address: IpAddr,
    #[arg(long)]
    sync: bool,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    let api = Api::init()?;
    let ip = opts.ip_address.to_string();
    if opts.sync {
        run(api.open_trigger_sync(Vxi11, &ip)?)?;
    } else {
        run(api.open_trigger_async(Vxi11, &ip)?)?;
    }

    println!("Done!");
    Ok(())
}

trait CustomTriggerMode: TriggerMode + Sized {
    fn resume(handle: &Handle<Self>) -> anyhow::Result<()>;
}
impl CustomTriggerMode for TriggerSync {
    fn resume(handle: &Handle<Self>) -> anyhow::Result<()> {
        println!("Resume!");
        handle.resume_trigger()?;
        Ok(())
    }
}
impl CustomTriggerMode for TriggerAsync {
    fn resume(_handle: &Handle<Self>) -> anyhow::Result<()> {
        Ok(())
    }
}

fn run<T: CustomTriggerMode>(handle: Handle<T>) -> anyhow::Result<()> {
    let channel = ChannelNumber {
        channel: 5,
        sub_channel: 1,
    };
    let mut current_count = 0;

    handle.start()?;
    for loop_index in 0..1000 {
        // dbg!();
        handle.latch_data()?;
        // dbg!();
        let next_count = handle.latched_acquisition_count()?;
        // dbg!();
        for i in current_count + 1..=next_count {
            // dbg!(i);
            handle.set_acquisition_index(i)?;
            // dbg!();
            let length = handle.triggered_samples_len(channel)?;
            // dbg!(length);
            let expected_len = length * 2;
            let mut buffer = vec![0; expected_len * 2];
            let result = handle.read_triggered_waveform(channel, &mut buffer)?;
            println!("Acquisition {i}: expected length = {expected_len}, return = {result:?}");
            println!("{:?}...", &buffer[..100.min(result.received_len)]);
            if !result.completed {
                bail!("There is more data");
            }
            T::resume(&handle)?;
        }
        current_count = next_count;
        println!(
            "Loop {loop_index}: Current acquisition count so far: {}",
            next_count
        );
        if next_count >= 5 {
            break;
        }
        sleep(Duration::from_millis(1));
    }

    // dbg!();
    handle.stop()?;
    // dbg!();

    Ok(())
}
