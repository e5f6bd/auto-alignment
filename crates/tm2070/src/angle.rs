use radians::{Deg64, Rad64};

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
impl Angle {
    pub fn value(&self) -> Rad64 {
        match *self {
            Angle::Min(m) => deg_from_dms(0., m, 0.).rad(),
            Angle::Deg(d) => Deg64::new(d).rad(),
            Angle::MDeg(md) => Deg64::new(md / 1000.).rad(),
            Angle::MRad(mr) => Rad64::new(mr / 1000.),
            Angle::DegMinSec(sign, d, m, s) => (deg_from_dms(d, m, s) * sign.as_f64()).rad(),
            Angle::MinSec(sign, m, s) => (deg_from_dms(0., m, s) * sign.as_f64()).rad(),
        }
    }
}

/// A helper method that should exist in `radians` crate.
fn deg_from_dms(d: f64, m: f64, s: f64) -> Deg64 {
    Deg64::new(d + m / 60. + s / 3600.)
}

#[derive(Clone, Copy, Debug)]
pub enum Sign {
    Positive,
    Negative,
}
impl Sign {
    fn as_f64(self) -> f64 {
        match self {
            Self::Positive => 1.,
            Self::Negative => -1.,
        }
    }
}
