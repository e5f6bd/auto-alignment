use std::{borrow::Cow, time::Duration};

use anyhow::bail;
use bstr::BStr;
#[cfg(feature = "clap")]
use clap::ValueEnum;
use log::info;
use serialport::{DataBits, Parity, StopBits};

use serial_wrapper::SerialWrapper;

pub struct Pamc112 {
    serial_wrapper: SerialWrapper,
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
        let serial_wrapper = SerialWrapper::new(serial);
        let mut ret = Self { serial_wrapper };
        ret.check_connection()?;
        Ok(ret)
    }

    pub fn check_connection(&mut self) -> anyhow::Result<()> {
        self.write(&b"CON\r\n"[..])?;
        self.read_wait(b"OK")
    }

    /// Constraints (panics otherwise)
    /// * channel < 22
    /// * 1 <= frequency <= 1500
    /// * 1 <= count <= 10^4
    pub fn drive(
        &mut self,
        channel: u8,
        direction: RotationDirection,
        frequency: u16,
        count: u16,
    ) -> anyhow::Result<()> {
        assert!(channel < 22);
        assert!((1..=1500).contains(&frequency));
        assert!(count > 0, "Setting count to 0 causes an indefinite drive!");
        assert!(count < 10000);
        let direction = match direction {
            RotationDirection::Cw => "NR",
            RotationDirection::Ccw => "RR",
        };
        let channel = (b'A' + channel) as char;
        self.write(
            format!("{direction}{frequency:04}{count:04}{channel}\r\n")
                .as_bytes()
                .to_owned(),
        )?;
        self.read_wait(b"OK")?;
        self.read_wait(b"FIN")
    }

    fn write(&mut self, contents: impl Into<Cow<'static, [u8]>>) -> anyhow::Result<()> {
        let contents = contents.into();
        info!("Write: {:?}", BStr::new(&contents));
        Ok(self.serial_wrapper.write_rx.send(contents)?)
    }

    fn read_wait(&mut self, expect: &[u8]) -> anyhow::Result<()> {
        // Blocking read
        let read = self.serial_wrapper.read_tx.recv()?;
        info!("Read: {:?}", BStr::new(&read));
        if read == expect {
            Ok(())
        } else {
            bail!(
                "Expected {:?}, found {:?}",
                BStr::new(&expect),
                BStr::new(&read)
            );
        }
        // let mut buf = vec![0; expect.len()];
        // let count = self.serial.read(&mut buf)?;
        // let buf = &buf[..count];
        // if buf == expect {
        //     Ok(())
        // } else if buf == b"FIN\r" {
        //     self.read_ok(b"\n")
        // } else {
        //     bail!("Unexpected response: {:?}", BStr::new(buf));
        // }
    }
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum RotationDirection {
    /// Clockwise
    Cw,
    /// Counterclockwise
    Ccw,
}
