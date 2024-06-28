use std::{borrow::Cow, path::PathBuf, sync::mpsc, thread::sleep, time::Duration};

use chrono::{DateTime, Local};
use clap::Parser;
use fs_err::File;
use log::error;
use serde::{Deserialize, Serialize};
use tm2070::Tm2070;

#[derive(Parser)]
struct Opts {
    save_dir: PathBuf,
    com_ports: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Record<'a> {
    time: DateTime<Local>,
    com_port: Cow<'a, str>,
    x: Option<f64>,
    y: Option<f64>,
}

fn main() -> anyhow::Result<()> {
    env_logger::builder().format_timestamp_nanos().init();
    let opts = Opts::parse();

    fs_err::create_dir_all(&opts.save_dir)?;

    let (rx, tx) = mpsc::channel();
    ctrlc::set_handler(move || {
        if let Err(e) = rx.send(()) {
            error!("Failed to send Ctrl-C signal: {e:#}");
        }
    })?;

    let mut tm2070s = opts
        .com_ports
        .iter()
        .map(|port| anyhow::Ok((port, Tm2070::new(port)?)))
        .collect::<Result<Vec<_>, _>>()?;
    let mut handles = tm2070s
        .iter_mut()
        .map(|(port, tm2070)| anyhow::Ok((*port, tm2070.continuous_1(None)?)))
        .collect::<Result<Vec<_>, _>>()?;
    let mut records = vec![];
    while tx.try_recv().is_err() {
        for (port, handle) in &mut handles {
            for data in handle.iter() {
                let data = match data {
                    Ok(data) => data,
                    Err(e) => {
                        error!("Error: {e:#}");
                        continue;
                    }
                };
                let record = Record {
                    time: Local::now(),
                    com_port: Cow::Borrowed(*port),
                    x: data.x.map(|x| x.value().val()),
                    y: data.y.map(|y| y.value().val()),
                };
                records.push(record);
            }
        }
        sleep(Duration::from_millis(10));
    }

    serde_json::to_writer(
        File::create(opts.save_dir.join(format!(
            "{}.json",
            Local::now().format("%Y-%m-%d_%H-%M-%S")
        )))?,
        &records,
    )?;

    Ok(())
}
