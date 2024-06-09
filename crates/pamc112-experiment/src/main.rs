use std::{
    cmp::Ordering,
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
use fs_err::OpenOptions;
use log::{info, warn};
use pamc112::{
    Pamc112,
    RotationDirection::{self, *},
};
use radians::{Angle, Deg64, Rad64};
use tm2070::Tm2070;

#[derive(Parser)]
struct Opts {
    pamc_port: String,
    tm2070_port: String,
    channel: u8,
    direction: RotationDirection,
    step: u16,
    output_path: String,
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

    for i in 0..10 {
        info!("=========== STEP {i} ===========");
        make_x_zero(&mut tm2070, &mut pamc, &opts)?;

        let initial = measure(&mut tm2070)?;
        let threshold = Deg64::new(0.5).rad();
        let condition = |angle: [Rad64; 2]| angle.into_iter().all(|x| angle_lt(x, threshold));
        let mut record = vec![initial];
        while {
            pamc.drive(opts.channel, opts.direction, 1500, opts.step)?;
            sleep(Duration::from_secs_f64(0.15));
            let res = measure(&mut tm2070)?;
            record.push(res);
            condition(res) && !finish.load(SeqCst)
        } {}

        let mut file = BufWriter::new(
            OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(format!("{}{i}", opts.output_path))?,
        );
        for [x, y] in record {
            writeln!(file, "{}\t{}", x.val(), y.val())?;
        }
    }

    Ok(())
}

fn make_x_zero(tm2070: &mut Tm2070, pamc: &mut Pamc112, opts: &Opts) -> anyhow::Result<()> {
    let mut x = || anyhow::Ok(tm2070.single_1()?.x.context("ND")?.value());
    let mut drive = |step: i16| match step.cmp(&0) {
        Ordering::Less => pamc.drive(opts.channel, Ccw, 1500, step.unsigned_abs()),
        Ordering::Equal => {
            warn!("Driving 0 steps");
            Ok(())
        }
        Ordering::Greater => pamc.drive(opts.channel, Cw, 1500, step.unsigned_abs()),
    };
    let sleep = || sleep(Duration::from_secs_f64(0.15));
    while angle_lt(Rad64::new(-0.5e-3), x()?) {
        drive(-100)?;
        sleep();
    }
    while angle_lt(x()?, Rad64::new(-0.1e-3)) {
        drive(30)?;
        sleep();
    }
    while angle_lt(x()?, Rad64::new(-0.03e-3)) {
        drive(5)?;
        sleep();
    }
    while angle_lt(x()?, Rad64::new(0.)) {
        drive(1)?;
        sleep();
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
    let count = 20;
    let mut x = 0.0;
    let mut y = 0.0;
    for _ in 0..count {
        let data = tm2070.single_1()?;
        x += data.x.context("ND")?.value().val();
        y += data.y.context("ND")?.value().val();
    }
    Ok([x, y].map(|x| Rad64::new(x / count as f64)))
}
