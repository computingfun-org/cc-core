pub mod job_notes;
mod native_date_serde;
pub mod paperwork_follow_up;
mod test;

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
#[repr(transparent)]
pub struct DashJob(std::num::NonZeroUsize);

impl DashJob {
    pub fn new(number: usize) -> Option<Self> {
        std::num::NonZeroUsize::new(number).map(Self::from)
    }

    pub fn number(&self) -> usize {
        self.0.get()
    }

    pub fn url(&self) -> Box<str> {
        self.url_string().into_boxed_str()
    }

    pub fn url_string(&self) -> String {
        format!("https://dashboard.calclosets.com/?j={}", self.number())
    }

    pub fn open(&self) -> Result<(), std::io::Error> {
        webbrowser::open(&self.url())
    }
}
