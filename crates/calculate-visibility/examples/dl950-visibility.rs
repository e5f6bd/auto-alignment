use std::{net::IpAddr, sync::mpsc, thread::sleep, time::Duration};

use anyhow::bail;
use calculate_visibility::{calculate, Params};
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
    handle.start()?;

    let channel = ChannelNumber::new(5, 1);

    let (ctrlc_rx, ctrlc_tx) = mpsc::channel();
    ctrlc::set_handler(move || {
        if let Err(e) = ctrlc_rx.send(()) {
            eprintln!("Could not send Ctrl+C signal: {e}");
        }
    })?;
    let ctrlc = move || ctrlc_tx.try_recv().is_ok();

    let waveform = {
        println!("Sleeping for 10 seconds...");
        sleep(Duration::from_secs(10));
        handle.latch_data()?;
        println!("Latched");
        // while {
        //     if ctrlc() {
        //         bail!("Interrupped");
        //     }
        //     let count = handle.latched_acquisition_count()?;
        //     println!("{count}");
        //     count == 0
        // } {
        //     sleep(Duration::from_millis(1));
        // }
        handle.set_acquisition_index(0)?;
        println!("Acquisition index set");
        let length = handle.triggered_samples_len(channel)?;
        println!("length = {length}");
        let mut buffer = vec![0; length * 2];
        let result = handle.read_triggered_waveform(channel, &mut buffer)?;
        println!("return = {result:?}");
        if !result.completed {
            bail!("Acquisition incomplete");
        }
        buffer
            .chunks(2)
            .map(|bytes| anyhow::Ok(i16::from_le_bytes(bytes.try_into()?)))
            .collect::<Result<Vec<_>, _>>()?
    };
    calculate(Params {
        waveform: &waveform.iter().map(|&x| x as f64).collect::<Vec<_>>(),
    });

    handle.stop()?;
    Ok(())
}
