use derive_more::{Display, From, FromStr, Into};

#[derive(
    Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, From, Into, FromStr, Display,
)]
#[display(fmt = "Job # {}", _0)]
pub struct JobNumber(usize);

#[derive(Clone, PartialEq, Eq)]
pub struct JobURL(String);

impl JobURL {
    pub fn url(&self) -> &str {
        self.0.as_str()
    }

    pub fn open(&self) -> Option<std::io::Error> {
        open::that(&self.url()).err()
    }
}

impl<T: Into<JobNumber>> From<T> for JobURL {
    fn from(job_number: T) -> Self {
        JobURL(format!(
            "https://dashboard.calclosets.com/?j={}",
            usize::from(job_number.into())
        ))
    }
}
