#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct JobNumber(usize);

impl JobNumber {
    pub fn number(self) -> usize {
        self.0
    }

    pub fn valid(self) -> bool {
        self.0 != 0
    }
}

impl<T: Into<usize>> From<T> for JobNumber {
    fn from(job_number: T) -> Self {
        JobNumber(job_number.into())
    }
}

impl std::str::FromStr for JobNumber {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<usize>() {
            Ok(num) => Ok(JobNumber::from(num)),
            Err(err) => Err(err),
        }
    }
}

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
            job_number.into().number()
        ))
    }
}
