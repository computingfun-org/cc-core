use chrono::NaiveDateTime;

use crate::JobNumber;

use self::notes::Notes;

pub mod btree_naive_date_time_serde;
pub mod lead_time;
pub mod naive_date_serde;
pub mod notes;
pub mod request;
pub mod tentative;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Schedule {
    pub job: JobNumber,
    pub notes: Notes,
    #[serde(with = "self::btree_naive_date_time_serde")]
    pub install: std::collections::BTreeSet<NaiveDateTime>,
}
