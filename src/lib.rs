use std::time::Duration;

use anyhow::bail;
use bstr::BStr;
use clap::ValueEnum;
use log::info;
use serialport::{DataBits, Parity, SerialPort, StopBits};

pub struct Pamc112 {
    serial: Box<dyn SerialPort>,
}

impl Pamc112 {
    pub fn new(port: &str, timeout: Duration) -> anyhow::Result<Self> {
        let serial = serialport::new(port, 115200)
            .data_bits(DataBits::Eight)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .flow_control(serialport::FlowControl::None)
            .timeout(timeout)
            .open()?;
        let mut ret = Self { serial };
        ret.check_connection()?;
        Ok(ret)
    }

    pub fn check_connection(&mut self) -> anyhow::Result<()> {
        self.write(b"CON\r\n")?;
        self.read_ok(b"OK\r\n")
    }

    /// Constraints (panics otherwise)
    /// * channel < 22
    /// * 1 <= frequency <= 1500
    /// * count <= 10^4
    pub fn drive(
        &mut self,
        channel: u8,
        direction: RotationDirection,
        frequency: u16,
        count: u16,
    ) -> anyhow::Result<()> {
        assert!(channel < 22);
        assert!((1..=1500).contains(&frequency));
        assert!(count < 10000);
        let direction = match direction {
            RotationDirection::Cw => "NR",
            RotationDirection::Ccw => "RR",
        };
        let channel = (b'A' + channel) as char;
        self.write(format!("{direction}{frequency:04}{count:04}{channel}\r\n").as_bytes())?;
        self.read_ok(b"OK\r\n")
    }

    fn write(&mut self, contents: &[u8]) -> anyhow::Result<()> {
        info!("Write: {:?}", BStr::new(contents));
        Ok(self.serial.write_all(contents)?)
    }

    fn read_ok(&mut self, expect: &[u8]) -> anyhow::Result<()> {
        let mut buf = vec![0; expect.len()];
        let count = self.serial.read(&mut buf)?;
        let buf = &buf[..count];
        if buf == expect {
            Ok(())
        } else if buf == b"FIN\r" {
            self.read_ok(b"\n")
        } else {
            bail!("Unexpected response: {:?}", BStr::new(buf));
        }
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum RotationDirection {
    /// Clockwise
    Cw,
    /// Counterclockwise
    Ccw,
}
