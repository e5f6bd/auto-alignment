use anyhow::bail;
use clap::Parser;
use pamc112::RotationDirection::{self, *};
use serde::Deserialize;
use std::{
    path::PathBuf,
    thread::sleep,
    time::{Duration, Instant},
};

#[derive(Parser)]
struct Opts {
    config_path: PathBuf,
}

#[derive(Deserialize)]
struct Config {}

fn main() -> anyhow::Result<()> {
    let ser_s = ();
    let ser1 = ();

    let _com1 = "COM4"; // for coupling reflection light into photodiode
    let _com2 = "COM5"; // for half beam splitter
    let _com3 = "COM10"; // for improving visibility
    let _com_s = "COM3"; // for optical shutter
    let _com_w = "COM9"; // for motorized filter wheel
    let _bit_rate = 115200;
    let device_name = "Dev1/ai0"; // for digitizer
    let (min_set, max_set, m_time, m_timev) = (-3, 3, 0.3, 1.0); // measurement range and time of digitizer
    let measured_parameter = (device_name, min_set, max_set, m_time);
    let _measured_parameterv = (device_name, min_set, max_set, m_timev);

    // improving visibility
    let start = Instant::now();
    sleep(Duration::from_secs(2));
    shutter(ser_s, 0, "close"); // signal light is port 0
    shutter(ser_s, 1, "close"); // LO light is port 1
    shutter(ser_s, 2, "close");
    shutter(ser_s, 3, "close");
    // Not needed for now
    // wheel(ser_w, 6); // Attention: must weaken signal light
    let base_line = ave(measured_parameter);
    sleep(Duration::from_secs(2));
    shutter(ser_s, 2, "open");
    shutter(ser_s, 0, "open");
    sleep(Duration::from_secs(1));
    let ave1 = ave(measured_parameter);
    sleep(Duration::from_secs_f64(m_time));
    let input1 = ave1 - base_line;
    shutter(ser_s, 1, "open");
    shutter(ser_s, 0, "close");
    sleep(Duration::from_secs(1));
    let ave2 = ave(measured_parameter);
    let input2 = ave2 - base_line;
    shutter(ser_s, 0, "open");
    sleep(Duration::from_secs(1));

    // COM_s = "COM3" // for optical shutter
    // COM = "COM10" // for improving visibility
    let _bit_rate = 115200;
    let device_name = "Dev1/ai0";
    let (min_set, max_set, m_time) = (-3, 3, 0.5);
    let measured_parameter = (device_name, min_set, max_set, m_time);
    let e = 1e-05;
    let move_p = 10; //pulses
    let constant_a = 2.28;
    let constant_b = 2.85;
    let constant_c = 1.33;
    let constant_d = 2.8;
    let constant = [constant_a, constant_b, constant_c, constant_d];

    let mut a = vec![];
    let mut b = vec![];
    let mut c = vec![];
    let mut d = vec![];
    let mut z = vec![];
    let mut t = vec![];
    sleep(Duration::from_secs(2));
    shutter(ser_s, 3, "close");
    shutter(ser_s, 0, "close"); // signal light is port 0
    shutter(ser_s, 1, "close"); // LO light is port 1
    let base_line = ave(measured_parameter);
    sleep(Duration::from_secs_f64(m_time));
    shutter(ser_s, 0, "open");
    sleep(Duration::from_secs(1));
    let ave1 = ave(measured_parameter);
    sleep(Duration::from_secs_f64(m_time));
    let _input_1 = ave1 - base_line;
    shutter(ser_s, 1, "open");
    shutter(ser_s, 0, "close");
    sleep(Duration::from_secs(1));
    let ave2 = ave(measured_parameter);
    let _input_2 = ave2 - base_line;
    shutter(ser_s, 0, "open");
    sleep(Duration::from_secs(1));

    let mut vis = vis_func(input1, input2, base_line, measured_parameter);
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

    while !flag {
        println!("{} times", i);
        let mut grad = gradient(
            input1,
            input2,
            base_line,
            move_p,
            &constant,
            ser1,
            measured_parameter,
        )?;
        let (da, db, dc, dd) = (grad[0], grad[1], grad[2], grad[3]);
        let absgrad = [da.abs(), db.abs(), dc.abs(), dd.abs()];
        let index = (0..4).fold(0, |i, j| if absgrad[i] > absgrad[j] { i } else { j });

        if da.abs() < e && db.abs() < e && dc.abs() < e && dd.abs() < e {
            println!("Program finished");
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
            println!("Next Optimization!!");
        }

        loop {
            let mut dire = [Cw; 4];
            let mut movement = [step_size1; 4];
            for j in 0..4 {
                if grad[j] > 0.0 {
                    dire[j] = Cw;
                    movement[j] = step_size1 * constant[j];
                } else {
                    dire[j] = Ccw;
                }
                movement[j] *= rate[j];
                move_servo(ser1, dire[j], movement[j], j as u8);
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
            let temp = vis_func(input1, input2, base_line, measured_parameter);
            result.push(temp);
            println!("Visibility: {}", temp);
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
                println!("Visibility is over 99%, program finish");
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
    println!("Final Visibility: {}", z.last().unwrap());
    // TODO save
    // let result = vec![&n, &t, &a, &b, &c, &d, &z].concat();
    // let raw_data3 = DataFrame::new(result);
    // let path = Path::new(&format!("f{}Time{}.csv", t, time.round()));
    // let mut file = File::create(&path).unwrap();
    // raw_data3.write_csv(&mut file).unwrap();
    println!("{}", time);

    Ok(())
}

#[allow(unused)]
fn shutter(_: (), channel: u8, command: &'static str) {}

#[allow(unused)]
fn ave((device_name, min_set, max_set, m_time): (&'static str, i32, i32, f64)) -> f64 {
    // TODO Get average of all datapoints
    0.0
}

#[allow(unused)]
fn vis_func(input1: f64, input2: f64, base_line: f64, parameter: (&str, i32, i32, f64)) -> f64 {
    // TODO measure visibility
    0.0
}

#[allow(unused)]
fn measurement(parameter: (&str, i32, i32, f64)) -> Vec<(f64, f64)> {
    // TODO get from oscilloscope
    vec![]
}

#[allow(unused)]
fn gradient(
    input1: f64,
    input2: f64,
    base_line: f64,
    move_p: i32,
    constant: &[f64; 4],
    ser: (),
    parameter: (&str, i32, i32, f64),
) -> anyhow::Result<[f64; 4]> {
    let e = 75.0;
    let mut big = [0.0; 4];
    let mut small = [0.0; 4];

    let move_p = move_p as f64;

    let o = vis_func(input1, input2, base_line, parameter);
    println!("Now visibility is {o:.4}");
    for i in 0..4 {
        move_servo(ser, Cw, move_p * constant[i], i as u8); // clockwise
        big[i] = vis_func(input1, input2, base_line, parameter);
        move_servo(ser, Ccw, move_p, i as u8);
        let o_temp = vis_func(input1, input2, base_line, parameter);
        if (o - o_temp).abs() > e {
            bail!("Error: I cannot come back to the original point. ({o:.4}, {o_temp:.4})",);
        }
        move_servo(ser, Ccw, move_p, i as u8); // Anticlockwise
        small[i] = vis_func(input1, input2, base_line, parameter);
        move_servo(ser, Cw, move_p * constant[i], i as u8);
        let o_temp = vis_func(input1, input2, base_line, parameter);
        if (o - o_temp).abs() > e {
            bail!("Error: I cannot come back to the original point. ({o:.4}, {o_temp:.4})",);
        }
    }

    let gradient = big
        .iter()
        .zip(&small)
        .map(|(b, s)| (b - s) / move_p)
        .collect::<Vec<f64>>();
    println!("∇Vis: {:?}", gradient);
    Ok(gradient.try_into().unwrap())
}

#[allow(unused)]
fn move_servo(ser: (), direction: RotationDirection, movement: f64, index: u8) {}

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
