use std::{net::IpAddr, thread::sleep, time::Duration};

use clap::Parser;
use dl950acqapi::{Api, ChannelNumber, WireType::Vxi11};

#[derive(Parser)]
struct Opts {
    ip_address: IpAddr,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    let api = Api::init()?;
    let handle = api.open_trigger_async(Vxi11, &opts.ip_address.to_string())?;

    let channel = ChannelNumber {
        channel: 5,
        sub_channel: 1,
    };
    let mut current_count = 0;

    handle.start()?;
    for loop_index in 0..100 {
        handle.latch_data()?;
        let next_count = handle.latched_acquisition_count()?;
        for i in current_count + 1..=next_count {
            handle.set_acquisition_index(i)?;
            let length = handle.triggered_samples_len(channel)?;
            loop {
                let expected_len = length * 2;
                let mut buffer = vec![0; expected_len * 2];
                let result = handle.read_triggered_waveform(channel, &mut buffer)?;
                println!("Acquisition {i}: expected length = {expected_len}, return = {result:?}");
                println!("{:?}...", &buffer[..100.min(result.received_len)]);
                if result.completed {
                    break;
                }
            }
        }
        current_count = next_count;
        println!(
            "Loop {loop_index}: Current acquisition count so far: {}",
            next_count
        );
        if next_count >= 1 {
            break;
        }
        sleep(Duration::from_millis(1));
    }
    handle.stop()?;

    println!("Done!");
    Ok(())
}
