use clap::Parser;
use tm2070::Tm2070;

#[derive(Parser)]
struct Opts {
    com_port: String,
}

fn main() -> anyhow::Result<()> {
    env_logger::builder().format_timestamp_nanos().build();
    let opts = Opts::parse();

    let mut tm2070 = Tm2070::new(&opts.com_port)?;
    let res = tm2070.single_1()?;
    println!("{res:?}");

    Ok(())
}
