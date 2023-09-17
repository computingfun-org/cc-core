extern crate core;

pub mod drawers;
pub mod schedule;

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

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display)]
#[display(fmt = "{}", _0)]
#[repr(transparent)]
pub struct JobURL(std::boxed::Box<str>);

impl From<JobNumber> for JobURL {
    fn from(value: JobNumber) -> Self {
        JobURL(format!("https://dashboard.calclosets.com/?j={}", value.0).into())
    }
}

impl JobURL {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
