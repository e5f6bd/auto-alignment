use ordered_float::OrderedFloat;

#[derive(Debug)]
pub struct Params<'a> {
    pub waveform: &'a [f64],
}

pub fn calculate(params: Params) -> Option<f64> {
    let min = params.waveform.iter().min_by_key(|&&x| OrderedFloat(x))?;
    let max = params.waveform.iter().max_by_key(|&&x| OrderedFloat(x))?;
    let visibility = (max - min) / (max + min);
    let average = params.waveform.iter().sum::<f64>() / params.waveform.len() as f64;
    println!("{min}\t{max}\t{average:.2}\t{:.2}", visibility * 100.);
    None
}
