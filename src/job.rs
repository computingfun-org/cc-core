use std::num::NonZeroU128;

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
pub struct JobNumber(NonZeroU128);

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

impl<T: Into<JobNumber>> From<T> for JobURL {
    fn from(job_number: T) -> Self {
        JobURL(format!(
            "https://dashboard.calclosets.com/?j={}",
            job_number.into()
        ))
    }
}
