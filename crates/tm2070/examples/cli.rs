use std::{sync::mpsc, thread::sleep, time::Duration};

use clap::Parser;
use log::error;
use tm2070::Tm2070;

#[derive(Parser)]
struct Opts {
    com_port: String,

    #[clap(long)]
    continuous: bool,
}

fn main() -> anyhow::Result<()> {
    env_logger::builder().format_timestamp_nanos().init();
    let opts = Opts::parse();

    let (rx, tx) = mpsc::channel();
    ctrlc::set_handler(move || {
        if let Err(e) = rx.send(()) {
            error!("Failed to send Ctrl-C signal: {e:#}");
        }
    })?;

    let mut tm2070 = Tm2070::new(&opts.com_port)?;
    if opts.continuous {
        let handle = tm2070.continuous_1(None)?;
        while tx.try_recv().is_err() {
            for data in handle.iter() {
                println!("{:?}", data?);
            }
            sleep(Duration::from_millis(10));
        }
    } else {
        let res = tm2070.single_1()?;
        println!("{res:?}");
    }

    Ok(())
}
