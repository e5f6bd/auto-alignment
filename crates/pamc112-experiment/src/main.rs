use std::{
    io::{BufWriter, Write},
    iter::repeat_with,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

use anyhow::{bail, Context};
use clap::Parser;
use fs_err::File;
use log::info;
use pamc112::{Pamc112, RotationDirection};
use radians::{Angle, Deg64, Rad64};
use tm2070::Tm2070;

#[derive(Parser)]
struct Opts {
    pamc_port: String,
    tm2070_port: String,
    channel: u8,
    direction: RotationDirection,
    step: u16,
    output_path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    env_logger::builder().format_timestamp_nanos().init();
    let opts = Opts::parse();
    if opts.channel >= 22 {
        bail!("Invalid channel number: {}", opts.channel);
    }
    if !(1..=9999).contains(&opts.step) {
        bail!("Invalid step: {}", opts.step);
    }
    let mut pamc = Pamc112::new(&opts.pamc_port, Duration::from_secs(1))?;
    let mut tm2070 = Tm2070::new(&opts.tm2070_port)?;

    let finish = Arc::new(AtomicBool::new(false));
    {
        let finish = finish.clone();
        ctrlc::set_handler(move || finish.store(true, SeqCst))?;
    }

    let initial = measure(&mut tm2070)?;
    let threshold = Deg64::new(0.5).rad();
    let condition = |angle: [Rad64; 2]| angle.into_iter().all(|x| angle_lt(x, threshold));
    if !condition(initial) {
        bail!("Already out of bounds");
    }
    let mut record = vec![initial];
    while {
        pamc.drive(opts.channel, opts.direction, 1500, opts.step)?;
        sleep(Duration::from_secs_f64(0.5));
        let res = measure(&mut tm2070)?;
        record.push(res);
        info!("{:?}", res.map(|x| x.deg()));
        condition(res) && !finish.load(SeqCst)
    } {}

    let mut file = BufWriter::new(File::create(opts.output_path)?);
    for [x, y] in record {
        writeln!(file, "{}\t{}", x.val(), y.val())?;
    }

    Ok(())
}

fn angle_lt<F, U>(x: Angle<F, U>, y: Angle<F, U>) -> bool
where
    F: radians::Float + std::cmp::PartialOrd,
    U: radians::Unit<F>,
{
    x.val() < y.val()
}

fn measure(tm2070: &mut Tm2070) -> anyhow::Result<[Rad64; 2]> {
    let handle = tm2070.continuous_1(None)?;
    let stream = repeat_with(|| {
        sleep(Duration::from_secs_f64(1. / 60.));
        handle.iter()
    });
    let count = 20;
    let mut x = 0.0;
    let mut y = 0.0;
    for data in stream.flatten().take(20) {
        let data = data?;
        x += data.x.context("ND")?.value().val();
        y += data.y.context("ND")?.value().val();
    }
    Ok([x, y].map(|x| Rad64::new(x / count as f64)))
}
