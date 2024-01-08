use crate::native_date_serde;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PaperworkFollowUp {
    pub confirm: FollowUp,
    pub deposit: FollowUp,
    pub marked: FollowUp,
    pub lead: Option<usize>,
    pub notes: String,
    //pub install: Option<chrono::NaiveDate>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum FollowUp {
    #[default]
    Missing,
    #[serde(
        serialize_with = "native_date_serde::serialize",
        deserialize_with = "native_date_serde::deserialize"
    )]
    Date(chrono::NaiveDate),
    Done,
}

impl PaperworkFollowUp {
    pub fn new() -> Self {
        Self::default()
    }

    //pub fn follow_up(&self) -> FollowUp {}
}
