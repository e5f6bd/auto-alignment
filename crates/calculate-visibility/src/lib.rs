#[derive(Debug)]
pub struct Params<'a> {
    pub waveform: &'a [f64],
}

pub fn calculate(params: Params) -> Option<f64> {
    println!("{params:?}");
    None
}
