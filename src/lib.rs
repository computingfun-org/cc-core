use derive_more::{Add, AddAssign, Display, From, FromStr, Into, Mul, MulAssign, Sum};
use std::hash::{Hash, Hasher};

pub mod drawer;
pub mod job;
pub mod kbc;
pub mod valen;

type Number = f64;

#[derive(
    Default,
    Debug,
    Copy,
    Clone,
    PartialEq,
    PartialOrd,
    From,
    Into,
    FromStr,
    Display,
    Add,
    Mul,
    Sum,
    AddAssign,
    MulAssign,
)]
#[display(fmt = "{}in", _0)]
pub struct Inches(Number);

#[derive(
    Default,
    Debug,
    Copy,
    Clone,
    PartialEq,
    PartialOrd,
    From,
    Into,
    FromStr,
    Display,
    Add,
    Mul,
    Sum,
    AddAssign,
    MulAssign,
)]
#[display(fmt = "{}mm", _0)]
pub struct Millimeters(Number);

impl Eq for Inches {}

impl Eq for Millimeters {}

const INCH_IN_MILLIMETERS: Number = 25.4;
const MILLIMETERS_IN_INCH: Number = 1.0 / INCH_IN_MILLIMETERS;

impl From<Millimeters> for Inches {
    fn from(m: Millimeters) -> Self {
        Self::from(m.0 * MILLIMETERS_IN_INCH)
    }
}

impl From<Inches> for Millimeters {
    fn from(i: Inches) -> Self {
        Self::from(i.0 * INCH_IN_MILLIMETERS)
    }
}

impl Hash for Inches {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{}", self.0).hash(state);
    }
}

impl Hash for Millimeters {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{}", self.0).hash(state);
    }
}
