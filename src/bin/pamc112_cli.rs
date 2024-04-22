use std::time::Duration;

use clap::{Parser, Subcommand};
use mirrormount_joystick::{Pamc112, RotationDirection};

#[derive(Parser)]
struct Opts {
    port: String,
    #[clap(long, default_value = "1.0")]
    timeout_secs: f64,
    #[command(subcommand)]
    sub: Sub,
}

#[derive(Subcommand)]
enum Sub {
    Check,
    Drive {
        channel: u8,
        direction: RotationDirection,
        frequency: u16,
        count: u16,
    },
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let mut controller = Pamc112::new(&opts.port, Duration::from_secs_f64(opts.timeout_secs))?;
    if let Sub::Drive {
        channel,
        direction,
        frequency,
        count,
    } = opts.sub
    {
        controller.drive(channel, direction, frequency, count)?;
    }
    Ok(())
}
