use std::collections::BTreeSet;

use chrono::NaiveDateTime;

use crate::JobNumber;

use super::{notes::Notes, tracker::Tracker};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tentative {
    pub job: JobNumber,
    pub notes: Notes,
    #[serde(with = "super::btree_naive_date_time_serde")]
    pub install: BTreeSet<NaiveDateTime>,
    pub deposit: Tracker,
    pub confirm: Tracker,
    pub flip: Tracker,
}

impl Tentative {}
