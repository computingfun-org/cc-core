extern crate core;

pub mod drawers;
pub mod schedule;

type Number = f64;

#[derive(
    Default,
    Debug,
    Copy,
    Clone,
    PartialEq,
    PartialOrd,
    derive_more::From,
    derive_more::Into,
    derive_more::FromStr,
    derive_more::Display,
    derive_more::Add,
    derive_more::Sub,
    derive_more::Mul,
    derive_more::Sum,
    derive_more::Div,
    derive_more::AddAssign,
    derive_more::SubAssign,
    derive_more::DivAssign,
    derive_more::MulAssign,
    serde::Serialize,
    serde::Deserialize,
)]
#[display(fmt = "{}\"", _0)]
#[repr(transparent)]
pub struct Inches(Number);

#[derive(
    Default,
    Debug,
    Copy,
    Clone,
    PartialEq,
    PartialOrd,
    derive_more::From,
    derive_more::Into,
    derive_more::FromStr,
    derive_more::Display,
    derive_more::Add,
    derive_more::Sub,
    derive_more::Mul,
    derive_more::Sum,
    derive_more::AddAssign,
    derive_more::SubAssign,
    derive_more::MulAssign,
    derive_more::Div,
    derive_more::DivAssign,
    serde::Serialize,
    serde::Deserialize,
)]
#[display(fmt = "{}mm", _0)]
#[repr(transparent)]
pub struct Millimeters(Number);

impl Eq for Inches {}

impl Eq for Millimeters {}

const MILLIMETERS_IN_INCH: Number = 25.4;

impl From<Millimeters> for Inches {
    fn from(m: Millimeters) -> Self {
        Self::from(m.0 / MILLIMETERS_IN_INCH)
    }
}

#[cfg(test)]
mod millimeters_to_inches_test {
    use crate::{Millimeters, Number};

    #[test]
    fn main() {
        test(25.4, 1.0);
        test(254.0, 10.0);
        test(457.2, 18.0);
        test(0.0, 0.0);
        test(-127.0, -5.0);
        test(-406.4, -16.0);
    }

    fn test(mm: Number, inc: Number) {
        let m = Millimeters::from(mm);
        let i = m.into_inches();
        let result = i.into_number();
        assert_eq!(result, inc, "{}mm -> {}\" | result:{}\"", mm, inc, result);
    }
}

impl From<Inches> for Millimeters {
    fn from(i: Inches) -> Self {
        Self::from(i.0 * MILLIMETERS_IN_INCH)
    }
}

#[cfg(test)]
mod inches_to_millimeters_test {
    use crate::{Inches, Number};

    #[test]
    fn main() {
        test(1.0, 25.4);
        test(10.0, 254.0);
        test(18.0, 457.2);
        test(0.0, 0.0);
        test(-5.0, -127.0);
        test(-16.0, -406.4);
    }

    fn test(inc: Number, mm: Number) {
        let i = Inches::from(inc);
        let m = i.into_millimeters();
        let result = m.into_number();
        assert_eq!(result, mm, "{}\" -> {}mm | result:{}mm", inc, mm, result);
    }
}

impl Inches {
    pub fn into_number(self) -> Number {
        self.into()
    }

    pub fn into_millimeters(self) -> Millimeters {
        self.into()
    }
}

impl Millimeters {
    pub fn into_number(self) -> Number {
        self.into()
    }

    pub fn into_inches(self) -> Inches {
        self.into()
    }
}

impl std::hash::Hash for Inches {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{}", self.0).hash(state);
    }
}

impl std::hash::Hash for Millimeters {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{}", self.0).hash(state);
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::From,
    derive_more::Into,
    derive_more::FromStr,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
)]
#[display(fmt = "{}", _0)]
#[repr(transparent)]
pub struct JobNumber(std::num::NonZeroUsize);

impl JobNumber {
    pub fn url(&self) -> JobURL {
        JobURL(format!("https://dashboard.calclosets.com/?j={}", self.0))
    }
}

#[derive(Clone, PartialEq, Eq, derive_more::Display, serde::Serialize, serde::Deserialize)]
#[display(fmt = "{}", _0)]
#[repr(transparent)]
pub struct JobURL(String);

impl JobURL {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[cfg(feature = "open")]
    pub fn open(&self) -> Option<std::io::Error> {
        open::that(&self.as_str()).err()
    }
}
