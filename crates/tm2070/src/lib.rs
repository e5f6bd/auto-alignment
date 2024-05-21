pub mod angle;

use std::{borrow::Cow, time::Duration};

use angle::Angle;
use anyhow::{bail, Context};
use bstr::{BStr, ByteSlice};
use log::{error, info, warn};
use radians::Deg64;
use serial_wrapper::SerialWrapper;
use serialport::{DataBits, Parity, StopBits};

pub struct Tm2070 {
    serial_wrapper: SerialWrapper,
}

impl Tm2070 {
    pub fn new(port: &str) -> anyhow::Result<Self> {
        let serial = serialport::new(port, 38400)
            .data_bits(DataBits::Eight)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .flow_control(serialport::FlowControl::None)
            .timeout(Duration::from_secs(1))
            .open()?;
        let serial_wrapper = SerialWrapper::new(serial);
        let ret = Self { serial_wrapper };
        Ok(ret)
    }

    fn writeln(&mut self, contents: impl Into<Cow<'static, [u8]>>) -> anyhow::Result<()> {
        let contents = contents.into();
        info!("Write: {:?}", BStr::new(&contents));
        self.serial_wrapper.write_rx.send(contents)?;
        self.serial_wrapper.write_rx.send(Cow::Borrowed(b"\r\n"))?;
        Ok(())
    }

    fn read(&mut self) -> anyhow::Result<Vec<u8>> {
        // Blocking read
        let read = self.serial_wrapper.read_tx.recv()?;
        info!("Read: {:?}", BStr::new(&read));
        if read == b"ERR\r\n" {
            bail!("Error response received");
        }
        Ok(read)
    }
}

#[derive(Debug)]
pub struct SamplingData1 {
    pub unit: angle::Unit,
    pub x: Option<Angle>,
    pub y: Option<Angle>,
    pub norm: Option<Angle>,
    pub direction: TiltDirection,
    pub judge: Judge,
}
fn parse_sampling_data_1(line: &[u8]) -> anyhow::Result<SamplingData1> {
    let response = BStr::new(line);
    let mut words = response.split(|&c| c == b',').map(|s| s.trim());
    (|| {
        if words.next() != Some(b"1") {
            bail!("Invalid entry found");
        }
        let unit = parse_unit(words.next().context("Unit not found")?)?;
        let x = parse_angle(unit, &mut words)?;
        let y = parse_angle(unit, &mut words)?;
        let norm = parse_angle(unit, &mut words)?;
        let direction = TiltDirection(Deg64::new(
            words
                .next()
                .context("Direction not found")?
                .to_str()?
                .parse::<f64>()?,
        ));
        let judge = parse_judge(words.next().context("Judge not found")?);
        if words.next().is_some() {
            bail!("Excessive entry found");
        }
        Ok(SamplingData1 {
            unit,
            x,
            y,
            norm,
            direction,
            judge,
        })
    })()
    .with_context(|| format!("Failed to parse as SamplingData1: {response:?}"))
}

impl Tm2070 {
    pub fn single_1(&mut self) -> anyhow::Result<SamplingData1> {
        self.writeln(b"G")?;
        parse_sampling_data_1(&self.read()?)
    }

    /// Panics if interval == 0.
    pub fn continuous_1(
        &mut self,
        interval: impl Into<Option<usize>>,
    ) -> anyhow::Result<Continuous1Handle> {
        let interval = interval.into();
        assert!(interval.map_or(true, |i| i > 0));
        match interval {
            None => self.writeln(b"L")?,
            Some(interval) => self.writeln(format!("L,{interval}").as_bytes().to_owned())?,
        }
        Ok(Continuous1Handle(self))
    }
}

pub struct Continuous1Handle<'a>(&'a mut Tm2070);
impl Continuous1Handle<'_> {
    pub fn iter(&self) -> impl Iterator<Item = anyhow::Result<SamplingData1>> + '_ {
        self.0.serial_wrapper.read_tx.try_iter().map(|line| {
            info!("Read: {:?}", BStr::new(&line));
            if line == b"ERR\r\n" {
                bail!("Error response received");
            }
            parse_sampling_data_1(&line)
        })
    }

    pub fn close(mut self) -> anyhow::Result<()> {
        self.close_impl()
    }

    fn close_impl(&mut self) -> anyhow::Result<()> {
        self.0.writeln(b"S")
    }
}
impl Drop for Continuous1Handle<'_> {
    fn drop(&mut self) {
        if let Err(e) = self.close_impl() {
            error!("Failed to close continuous fetch: {e:#}")
        }
    }
}

fn parse_unit(unit: &[u8]) -> anyhow::Result<angle::Unit> {
    use angle::Unit::*;
    Ok(match unit {
        b"min" => Min,
        b"deg" => Deg,
        b"mdeg" => MDeg,
        b"mrad" => MRad,
        b"deg-min-sec" => DegMinSec,
        b"min-sec" => MinSec,
        _ => bail!("Invalid unit: {:?}", BStr::new(unit)),
    })
}

fn parse_angle<'w, I>(unit: angle::Unit, mut words: I) -> anyhow::Result<Option<Angle>>
where
    I: Iterator<Item = &'w [u8]>,
{
    let words = &mut words;
    let parse = |words: &mut I| {
        anyhow::Ok(match words.next().context("No more word")? {
            b"-" => None,
            s => Some(s.to_str()?.parse::<f64>()?),
        })
    };
    use angle::Unit::*;
    Ok(match unit {
        Min => parse(words)?.map(Angle::Min),
        Deg => parse(words)?.map(Angle::Deg),
        MDeg => parse(words)?.map(Angle::MDeg),
        MRad => parse(words)?.map(Angle::MRad),
        DegMinSec => {
            let sign = parse_sign(words.next().context("No more word")?)?;
            let d = parse(words).context("Failed to parse degrees")?;
            let m = parse(words).context("Failed to parse minutes")?;
            let s = parse(words).context("Failed to parse seconds")?;
            match (sign, d, m, s) {
                (Some(sign), Some(d), Some(m), Some(s)) => Some(Angle::DegMinSec(sign, d, m, s)),
                (None, None, None, None) => None,
                _ => bail!("Inconsistent use of hyphen"),
            }
        }
        MinSec => {
            let sign = parse_sign(words.next().context("No more word")?)?;
            let m = parse(words).context("Failed to parse minutes")?;
            let s = parse(words).context("Failed to parse seconds")?;
            match (sign, m, s) {
                (Some(sign), Some(m), Some(s)) => Some(Angle::MinSec(sign, m, s)),
                (None, None, None) => None,
                _ => bail!("Inconsistent use of hyphen"),
            }
        }
    })
}

fn parse_sign(word: &[u8]) -> anyhow::Result<Option<angle::Sign>> {
    Ok(match word {
        b"P" => Some(angle::Sign::Positive),
        b"M" => Some(angle::Sign::Negative),
        b"-" => None,
        _ => bail!("Unexpected sign: {:?}", BStr::new(word)),
    })
}

#[derive(Clone, Copy, Debug)]
pub struct TiltDirection(pub Deg64);

#[derive(Clone, Debug)]
pub enum Judge {
    Ok,
    Ng,
    Nd,
    Er,
    Unknown(Vec<u8>),
}
fn parse_judge(word: &[u8]) -> Judge {
    use Judge::*;
    match word {
        b"OK" => Ok,
        b"NG" => Ng,
        b"ND" => Nd,
        b"ER" => Er,
        _ => {
            warn!("Unknown judge: {:?}", BStr::new(word));
            Unknown(word.to_owned())
        }
    }
}
