use std::{net::IpAddr, sync::mpsc, thread::sleep, time::Duration};

use calculate_visibility::{calculate, Params};
use clap::Parser;
use dl950acqapi::{Api, ChannelNumber, WireType::Vxi11};

#[derive(Parser)]
struct Opts {
    ip_address: IpAddr,
    #[clap(long)]
    repeat: bool,
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

    'outer: for i in 1.. {
        let waveform = {
            while {
                handle.latch_data()?;
                if ctrlc() {
                    break 'outer;
                }
                let count = handle.latched_acquisition_count()?;
                count < i
            } {
                sleep(Duration::from_millis(1));
            }
            handle.get_waveform(i, channel)?
        };
        calculate(Params {
            waveform: &waveform.iter().map(|&x| x as f64).collect::<Vec<_>>(),
        });
        if !opts.repeat || ctrlc() {
            break;
        }
    }

    handle.stop()?;
    Ok(())
}
