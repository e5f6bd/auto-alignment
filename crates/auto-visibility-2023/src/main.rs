use std::{
    net::IpAddr,
    path::PathBuf,
    sync::mpsc,
    thread::sleep,
    time::{Duration, Instant},
};

use anyhow::{bail, Context};
use calculate_visibility::Params;
use clap::Parser;
use dl950acqapi::{connection_mode::TriggerAsync, ChannelNumber, Handle, WireType::Vxi11};
use log::{error, info};
use pamc112::{Pamc112, RotationDirection::*};
use serde::Deserialize;

#[derive(Parser)]
struct Opts {
    config_path: PathBuf,
}

#[derive(Deserialize)]
struct Config {
    pamc_port: String,
    dl950_address: IpAddr,
    channel: u8,
    sub_channel: u8,

    #[allow(unused)]
    input1: f64,
    #[allow(unused)]
    input2: f64,
    #[allow(unused)]
    base_line: f64,
    // Ratio of speed for Cw / speed for Ccw
    pamc_coef: [f64; 4],
    // Base step of gradient (pulse count; default = 10)
    pamc_step: f64,

    pamc_wait: f64,
}

fn main() -> anyhow::Result<()> {
    env_logger::builder().format_timestamp_nanos().init();

    let opts = Opts::parse();
    let config: Config = toml::from_str(&fs_err::read_to_string(opts.config_path)?)?;
    let channel = ChannelNumber {
        channel: config.channel,
        sub_channel: config.sub_channel,
    };

    let mut pamc = Pamc112::new(&config.pamc_port, Duration::from_secs(1))?;
    let api = dl950acqapi::Api::init()?;
    let handle = api.open_trigger_async(Vxi11, &config.dl950_address.to_string())?;
    handle.start()?;
    handle.latch_data()?;

    let (ctrlc_rx, ctrlc_tx) = mpsc::channel();
    ctrlc::set_handler(move || {
        if let Err(e) = ctrlc_rx.send(()) {
            error!("Could not send Ctrl+C signal: {e}");
        }
    })?;
    let ctrlc = move || {
        let ret = ctrlc_tx.try_recv().is_ok();
        if ret {
            info!("Ctrl-C detected, aborting.");
        }
        ret
    };

    let _ser_s = ();
    let _ser1 = ();

    let _com1 = "COM4"; // for coupling reflection light into photodiode
    let _com2 = "COM5"; // for half beam splitter
    let _com3 = "COM10"; // for improving visibility
    let _com_s = "COM3"; // for optical shutter
    let _com_w = "COM9"; // for motorized filter wheel
    let _bit_rate = 115200;
    let device_name = "Dev1/ai0"; // for digitizer
    let (min_set, max_set, m_time, m_timev) = (-3, 3, 0.3, 1.0); // measurement range and time of digitizer
    let _measured_parameter = (device_name, min_set, max_set, m_time);
    let _measured_parameterv = (device_name, min_set, max_set, m_timev);

    // improving visibility
    let start = Instant::now();
    // sleep(Duration::from_secs(2));
    // shutter(ser_s, 0, "close"); // signal light is port 0
    // shutter(ser_s, 1, "close"); // LO light is port 1
    // shutter(ser_s, 2, "close");
    // shutter(ser_s, 3, "close");
    // // Not needed for now
    // // wheel(ser_w, 6); // Attention: must weaken signal light
    // let base_line = ave(measured_parameter);
    // sleep(Duration::from_secs(2));
    // shutter(ser_s, 2, "open");
    // shutter(ser_s, 0, "open");
    // sleep(Duration::from_secs(1));
    // let ave1 = ave(measured_parameter);
    // sleep(Duration::from_secs_f64(m_time));
    // let input1 = ave1 - base_line;
    // shutter(ser_s, 1, "open");
    // shutter(ser_s, 0, "close");
    // sleep(Duration::from_secs(1));
    // let ave2 = ave(measured_parameter);
    // let input2 = ave2 - base_line;
    // shutter(ser_s, 0, "open");
    // sleep(Duration::from_secs(1));

    // COM_s = "COM3" // for optical shutter
    // COM = "COM10" // for improving visibility
    let _bit_rate = 115200;
    let device_name = "Dev1/ai0";
    let (min_set, max_set, m_time) = (-3, 3, 0.5);
    let _measured_parameter = (device_name, min_set, max_set, m_time);
    let e = 1e-05;
    // let move_p = 10; //pulses

    // let constant_a = 2.28;
    // let constant_b = 2.85;
    // let constant_c = 1.33;
    // let constant_d = 2.8;
    // let constant = [constant_a, constant_b, constant_c, constant_d];

    let mut a = vec![];
    let mut b = vec![];
    let mut c = vec![];
    let mut d = vec![];
    let mut z = vec![];
    let mut t = vec![];

    // sleep(Duration::from_secs(2));
    // shutter(ser_s, 3, "close");
    // shutter(ser_s, 0, "close"); // signal light is port 0
    // shutter(ser_s, 1, "close"); // LO light is port 1
    // let base_line = ave(measured_parameter);
    // sleep(Duration::from_secs_f64(m_time));
    // shutter(ser_s, 0, "open");
    // sleep(Duration::from_secs(1));
    // let ave1 = ave(measured_parameter);
    // sleep(Duration::from_secs_f64(m_time));
    // let _input_1 = ave1 - base_line;
    // shutter(ser_s, 1, "open");
    // shutter(ser_s, 0, "close");
    // sleep(Duration::from_secs(1));
    // let ave2 = ave(measured_parameter);
    // let _input_2 = ave2 - base_line;
    // shutter(ser_s, 0, "open");
    // sleep(Duration::from_secs(1));

    let mut vis = vis_func(&config, &ctrlc, &handle, channel)?;
    if vis < e {
        bail!("Please improve this visibility");
    }
    // let date = Local::now().format("%H%M%S").to_string();
    // let dotmap = measurement(measured_parameter);
    // TODO Save
    // let df = DataFrame::new(dotmap);
    // let path = Path::new(&format!("f{}initial_wave.csv", date));
    // let mut file = File::create(&path).unwrap();
    // df.write_csv(&mut file).unwrap();

    let mut rotation = [0.0; 4];
    let end = Instant::now();
    let time = end.duration_since(start).as_secs_f64();

    a.push(rotation[0]);
    b.push(rotation[1]);
    c.push(rotation[2]);
    d.push(rotation[3]);
    z.push(vec![vis]);
    t.push(time);
    let mut i = 1;
    let mut flag = false;

    while !flag && !ctrlc() {
        info!("{} times", i);
        let mut grad = gradient(&config, &ctrlc, &mut pamc, &handle, channel)?;
        let (da, db, dc, dd) = (grad[0], grad[1], grad[2], grad[3]);
        let absgrad = [da.abs(), db.abs(), dc.abs(), dd.abs()];
        let index = (0..4).fold(0, |i, j| if absgrad[i] > absgrad[j] { i } else { j });

        if da.abs() < e && db.abs() < e && dc.abs() < e && dd.abs() < e {
            info!("Program finished");
            break;
        }

        let mut rate = [1.0; 4];
        for k in 1..4 {
            rate[k + index - 4] = (grad[index + k - 4] / grad[index]).abs();
        }

        let mut r = 0.0;
        let mut dirr = 1.0;
        let mut r_vec = vec![r];
        let mut result = vec![];
        let step_size = (16 - 2 * (i - 1)) as f64;
        let mut step_size1 = step_size;

        result.push(vis);
        if vis > 99.0 {
            break;
        } else {
            info!("Next Optimization!!");
        }

        while !ctrlc() {
            let mut dire = [Cw; 4];
            let mut movement = [step_size1; 4];
            for j in 0..4 {
                if grad[j] > 0.0 {
                    dire[j] = Cw;
                    movement[j] = step_size1 * config.pamc_coef[j];
                } else {
                    dire[j] = Ccw;
                }
                movement[j] *= rate[j];
                pamc.drive(j as u8, dire[j], 1500, movement[j] as u16)?;
                sleep(Duration::from_secs_f64(config.pamc_wait));
                let direction_coef = if let Cw = dire[j] { 1. } else { -1. };
                rotation[j] += direction_coef * step_size1 * rate[j];
            }

            r += dirr * step_size1;
            a.push(rotation[0]);
            b.push(rotation[1]);
            c.push(rotation[2]);
            d.push(rotation[3]);
            r_vec.push(r);
            sleep(Duration::from_millis(200));
            let temp = vis_func(&config, &ctrlc, &handle, channel)?;
            result.push(temp);
            info!("Visibility: {}", temp);
            let end = Instant::now();
            let time = end.duration_since(start).as_secs_f64();
            t.push(time);

            if r_vec.len() >= 10 || (vis - temp).abs() < 0.1 {
                break;
            }

            if vis <= temp {
                vis = temp;
            } else {
                dirr = -dirr;
                step_size1 *= 0.9;
                if step_size1 < 1.0 {
                    break;
                }
                vis = temp;
                grad = [-grad[0], -grad[1], -grad[2], -grad[3]];
            }

            if vis > 99.0 {
                flag = true;
                info!("Visibility is over 99%, program finish");
                break;
            }
        }

        // TODO save
        // let n = (0..r_vec.len()).map(|x| x as f64).collect::<Vec<_>>();
        // let data = vec![&n, &r_vec, &result].concat();
        // let raw_data2 = DataFrame::new(data);
        // let path = Path::new(&format!("f{}step{}.csv", t, i));
        // let mut file = File::create(&path).unwrap();
        // raw_data2.write_csv(&mut file).unwrap();
        result.pop();
        z.push(result);

        if step_size < 2.0 {
            break;
        }
        i += 1;
    }

    let z: Vec<_> = z.into_iter().flatten().collect();
    // let n = (0..z.len()).collect::<Vec<_>>();
    info!("Final Visibility: {}", z.last().unwrap());
    // TODO save
    // let result = vec![&n, &t, &a, &b, &c, &d, &z].concat();
    // let raw_data3 = DataFrame::new(result);
    // let path = Path::new(&format!("f{}Time{}.csv", t, time.round()));
    // let mut file = File::create(&path).unwrap();
    // raw_data3.write_csv(&mut file).unwrap();
    info!("{}", time);

    Ok(())
}

// #[allow(unused)]
// fn shutter(_: (), channel: u8, command: &'static str) {}

// #[allow(unused)]
// fn ave((device_name, min_set, max_set, m_time): (&'static str, i32, i32, f64)) -> f64 {
//     // not-todo-anymore Get average of all datapoints
//     0.0
// }

fn vis_func(
    _config: &Config,
    ctrlc: impl Fn() -> bool,
    handle: &Handle<TriggerAsync>,
    channel: ChannelNumber,
) -> anyhow::Result<f64> {
    handle.latch_data()?;
    let count_current = handle.latched_acquisition_count()?;
    while {
        handle.latch_data()?;
        if ctrlc() {
            bail!("Ctrl-C");
        }
        let count = handle.latched_acquisition_count()?;
        count == count_current
    } {
        sleep(Duration::from_millis(1));
    }
    let waveform = handle.get_waveform(count_current + 1, channel)?;
    let visibility = calculate_visibility::calculate(Params {
        waveform: &waveform.iter().map(|&x| x as f64).collect::<Vec<_>>(),
    })
    .context("Visibility could not be found")?;
    Ok(visibility)
}

// #[allow(unused)]
// fn measurement(parameter: (&str, i32, i32, f64)) -> Vec<(f64, f64)> {
//     // TODO get from oscilloscope
//     vec![]
// }

#[allow(clippy::too_many_arguments)]
fn gradient(
    config: &Config,
    ctrlc: impl Fn() -> bool + Clone,
    pamc: &mut Pamc112,
    handle: &Handle<TriggerAsync>,
    channel: ChannelNumber,
) -> anyhow::Result<[f64; 4]> {
    let e = 75.0;
    let mut big = [0.0; 4];
    let mut small = [0.0; 4];

    let move_p = config.pamc_step;

    let o = vis_func(config, ctrlc.clone(), handle, channel)?;
    info!("Now visibility is {o:.4}");
    for i in 0..4 {
        pamc.drive(i as u8, Cw, 1500, (move_p * config.pamc_coef[i]) as u16)?; // clockwise
        sleep(Duration::from_secs_f64(config.pamc_wait));
        big[i] = vis_func(config, ctrlc.clone(), handle, channel)?;

        pamc.drive(i as u8, Ccw, 1500, move_p as u16)?;
        sleep(Duration::from_secs_f64(config.pamc_wait));
        let o_temp = vis_func(config, ctrlc.clone(), handle, channel)?;

        if (o - o_temp).abs() > e {
            bail!("Error: I cannot come back to the original point. ({o:.4}, {o_temp:.4})",);
        }

        pamc.drive(i as u8, Ccw, 1500, move_p as u16)?; // Anticlockwise
        sleep(Duration::from_secs_f64(config.pamc_wait));
        small[i] = vis_func(config, ctrlc.clone(), handle, channel)?;

        pamc.drive(i as u8, Cw, 1500, (move_p * config.pamc_coef[i]) as u16)?;
        sleep(Duration::from_secs_f64(config.pamc_wait));
        let o_temp = vis_func(config, ctrlc.clone(), handle, channel)?;

        if (o - o_temp).abs() > e {
            bail!("Error: I cannot come back to the original point. ({o:.4}, {o_temp:.4})",);
        }
    }

    let gradient = big
        .iter()
        .zip(&small)
        .map(|(b, s)| (b - s) / move_p)
        .collect::<Vec<f64>>();
    info!("∇Vis: {:?}", gradient);
    Ok(gradient.try_into().unwrap())
}

// struct DataFrame {
//     data: Vec<Vec<f64>>,
//     columns: Vec<String>,
// }
//
// impl DataFrame {
//     fn new(data: Vec<f64>) -> DataFrame {
//         let columns: Vec<_> = vec!["N", "R", "Visibility"]
//             .iter()
//             .map(|s| s.to_string())
//             .collect();
//         let rows = data.len() / columns.len();
//         let mut data_matrix = vec![vec![0.0; columns.len()]; rows];
//         for i in 0..rows {
//             for j in 0..columns.len() {
//                 data_matrix[i][j] = data[i * columns.len() + j];
//             }
//         }
//         DataFrame {
//             data: data_matrix,
//             columns,
//         }
//     }
//
//     fn write_csv(&self, file: &mut File) -> std::io::Result<()> {
//         writeln!(file, "{}", self.columns.join(","))?;
//         for row in &self.data {
//             let row_str: Vec<String> = row.iter().map(|v| v.to_string()).collect();
//             writeln!(file, "{}", row_str.join(","))?;
//         }
//         Ok(())
//     }
// }

// struct Shutters {
//     serial: SerialWrapper,
// }
//
// impl Shutters {
//     fn new(port: &str, timeout: Duration) -> anyhow::Result<Self> {
//         let serial = serialport::new(port, 115200)
//             .data_bits(DataBits::Eight)
//             .parity(Parity::None)
//             .stop_bits(StopBits::One)
//             .flow_control(serialport::FlowControl::None)
//             .timeout(timeout)
//             .open()?;
//         let serial = SerialWrapper::new(serial);
//         Ok(Self { serial })
//     }
//
//     fn open(&self, port: usize) -> anyhow::Result<()> {
//         self.drive(port, OpenClose::Open)
//     }
//
//     fn close(&self, port: usize) -> anyhow::Result<()> {
//         self.drive(port, OpenClose::Close)
//     }
//
//     fn drive(&self, port: usize, open_close: OpenClose) -> anyhow::Result<()> {
//         assert!(port < 3);
//         self.serial.write_rx.send(
//             match open_close {
//                 OpenClose::Open => &b"open"[..],
//                 OpenClose::Close => &b"close"[..],
//             }
//             .into(),
//         )?;
//         Ok(())
//     }
// }
//
// enum OpenClose {
//     Open,
//     Close,
// }
