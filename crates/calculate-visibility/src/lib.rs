use std::mem::size_of;

use itertools::Itertools;
use ordered_float::OrderedFloat;
use polyfit_rs::polyfit_rs::polyfit;

#[derive(Debug)]
pub struct Params<'a> {
    pub waveform: &'a [f64],
}

pub fn calculate(params: Params) -> Option<f64> {
    let &min = params.waveform.iter().min_by_key(|&&x| OrderedFloat(x))?;
    let &max = params.waveform.iter().max_by_key(|&&x| OrderedFloat(x))?;
    let f = |x: f64| min + (max - min) * x;
    let (lb, ub) = (f(0.1), f(0.9));
    let bound = |x: f64| (x < lb, ub < x);
    let visibility_minmax = visibility(min, max);

    let [mut min, mut max] = [None; 2];
    let mut extremes = vec![];

    for segment in params.waveform.chunk_by(|&x, &y| bound(x) == bound(y)) {
        let start =
            (segment.as_ptr() as usize - params.waveform.as_ptr() as usize) / size_of::<f64>();
        let end = start + segment.len();
        if start == 0 || end == params.waveform.len() {
            continue;
        }

        let (is_min, is_max) = bound(segment[0]);
        if !(is_min || is_max) || segment.len() < 20 {
            continue;
        }
        let x = (0..segment.len()).map(|x| x as f64).collect_vec();
        let Ok(res) = polyfit(&x, segment, 2) else {
            println!("Skipped!");
            continue;
        };
        // y = c x^2 + b x + a
        // y' = 0 @ (2cx + b = 0) <=> x = -b / 2c
        let [a, b, c]: [f64; 3] = res.try_into().unwrap();
        let f = |x: f64| a + b * x + c * x * x;
        let extreme = f(-b / 2. / c);
        if is_min {
            min = Some(extreme);
        }
        if is_max {
            max = Some(extreme);
        }
        extremes.push(extreme);
    }

    let average = params.waveform.iter().sum::<f64>() / params.waveform.len() as f64;
    print!("{average:6.1}    ");
    if let (Some(min), Some(max)) = (min, max) {
        let visibility_fit = visibility(min, max);
        print!(
            "{:3.2}  {:3.2}",
            visibility_fit * 100.,
            visibility_minmax * 100.
        );
        for extreme in extremes {
            print!("  {:6.1}", extreme);
        }
    }
    println!();
    Some(visibility_minmax)
}

fn visibility(min: f64, max: f64) -> f64 {
    (max - min) / (max + min)
}
