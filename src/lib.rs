pub mod job_notes;
mod native_date_serde;
pub mod scheduling;
mod test;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DashJob {
    pub id: DashID,
    #[serde(flatten)]
    pub notes: job_notes::JobNotes,
}

impl DashJob {
    pub fn open(&self) -> Result<(), std::io::Error> {
        self.id.open()
    }

    pub fn url(&self) -> Box<str> {
        self.id.url()
    }
}

impl std::fmt::Display for DashJob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {}", self.id, self.notes))
    }
}

impl From<DashID> for DashJob {
    fn from(id: DashID) -> Self {
        Self {
            id,
            notes: job_notes::JobNotes::new(),
        }
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
#[repr(transparent)]
pub struct DashID(std::num::NonZeroUsize);

impl DashID {
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
