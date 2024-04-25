use std::{
    borrow::Cow,
    collections::VecDeque,
    sync::mpsc,
    thread::{sleep, spawn},
    time::Duration,
};

use anyhow::bail;
use bstr::BStr;
use clap::ValueEnum;
use itertools::Itertools;
use log::{error, info};
use serialport::{DataBits, Parity, SerialPort, StopBits};

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

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum RotationDirection {
    /// Clockwise
    Cw,
    /// Counterclockwise
    Ccw,
}

struct SerialWrapper {
    read_tx: mpsc::Receiver<Vec<u8>>,
    write_rx: mpsc::Sender<Cow<'static, [u8]>>,
}

impl SerialWrapper {
    fn new(mut serial: Box<dyn SerialPort>) -> Self {
        let (read_rx, read_tx) = mpsc::channel();
        let (write_rx, write_tx) = mpsc::channel::<Cow<'static, [u8]>>();
        spawn(move || {
            let res = (|| -> anyhow::Result<()> {
                let mut read_deque = VecDeque::new();
                loop {
                    // Write
                    for message in write_tx.try_iter() {
                        serial.write_all(&message)?;
                    }

                    // Read
                    let expected_read = serial.bytes_to_read()? as usize;
                    if expected_read > 0 {
                        let mut buf = vec![0; expected_read];
                        let actual_read = serial.read(&mut buf)?;
                        if actual_read < expected_read {
                            bail!("Expected {expected_read} bytes, found {actual_read} bytes");
                        }
                        read_deque.extend(buf);
                        while let Some(i) = (read_deque.iter().tuple_windows().enumerate())
                            .find_map(|(i, (&x, &y))| (&[x, y] == b"\r\n").then_some(i))
                        {
                            let line = Vec::from_iter(read_deque.drain(..i + 2).take(i));
                            read_rx.send(line)?;
                        }
                    }
                    // 1 / (115200 Hz) = 8 microseconds, so wait 20 microseconds
                    sleep(Duration::from_micros(20));
                }
            })();
            if let Err(e) = res {
                error!("{:#}", e);
            }
        });
        Self { read_tx, write_rx }
    }
}
