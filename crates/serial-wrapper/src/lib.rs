use std::{
    borrow::Cow,
    collections::VecDeque,
    sync::mpsc,
    thread::{sleep, spawn},
    time::Duration,
};

use anyhow::bail;
use itertools::Itertools;
use log::error;
use serialport::SerialPort;

pub struct SerialWrapper {
    pub read_tx: mpsc::Receiver<Vec<u8>>,
    pub write_rx: mpsc::Sender<Cow<'static, [u8]>>,
}

impl SerialWrapper {
    pub fn new(mut serial: Box<dyn SerialPort>) -> Self {
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
