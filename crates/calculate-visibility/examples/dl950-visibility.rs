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
        while {
            handle.latch_data()?;
            if ctrlc() {
                bail!("Interrupped");
            }
            let count = handle.latched_acquisition_count()?;
            count == 0
        } {
            sleep(Duration::from_millis(1));
        }
        handle.set_acquisition_index(1)?;
        let length = handle.triggered_samples_len(channel)?;
        let mut buffer = vec![0; length * 2 + 1]; // The last byte is reversed for b'\n'.
                                                  // Without this byte, the library mistakenly
                                                  // thinks that writing is not complete
        let result = handle.read_triggered_waveform(channel, &mut buffer)?;
        if !result.completed {
            bail!("Acquisition incomplete");
        }
        buffer[..length * 2]
            .chunks(2)
            // Guaranteed to be even-lengthed
            .map(|bytes| i16::from_le_bytes(bytes.try_into().unwrap()))
            .collect::<Vec<_>>()
    };
    calculate(Params {
        waveform: &waveform.iter().map(|&x| x as f64).collect::<Vec<_>>(),
    });

    handle.stop()?;
    Ok(())
}
