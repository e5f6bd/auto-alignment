#[derive(Clone, Copy, Debug)]
pub enum Unit {
    Min,
    Deg,
    MDeg,
    MRad,
    DegMinSec,
    MinSec,
}

#[derive(Clone, Copy, Debug)]
pub enum Angle {
    Min(f64),
    Deg(f64),
    MDeg(f64),
    MRad(f64),
    DegMinSec(Sign, f64, f64, f64),
    MinSec(Sign, f64, f64),
}

#[derive(Clone, Copy, Debug)]
pub enum Sign {
    Positive,
    Negative,
}
