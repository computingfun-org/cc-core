use chrono::{NaiveDate, NaiveDateTime};

use crate::JobNumber;

use super::{
    lead_time::{Backwards, LeadTime, Stepper},
    notes::Notes,
};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Tentative {
    pub job: JobNumber,
    pub notes: Notes,
    #[serde(with = "super::btree_naive_date_time_serde")]
    pub install: std::collections::BTreeSet<NaiveDateTime>,
    pub payment: Tracker,
    pub confimation: Tracker,
    pub flip: Tracker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Tracker {
    #[serde(with = "super::naive_date_serde")]
    CompleteBy(NaiveDate),
    #[serde(with = "super::naive_date_serde")]
    Complete(NaiveDate),
}

pub enum TrackerState {
    Watch,
    Warn,
    Late,
    Done,
}

impl Tracker {
    fn date_state(date: NaiveDate, now: NaiveDate, warn_at: LeadTime) -> TrackerState {
        if date >= now {
            return TrackerState::Late;
        }

        if date >= Backwards::time(now, warn_at) {
            return TrackerState::Warn;
        }

        TrackerState::Watch
    }

    pub fn state(&self, now: NaiveDate, warn_at: LeadTime) -> TrackerState {
        match self {
            Tracker::CompleteBy(date) => Self::date_state(*date, now, warn_at),
            Tracker::Complete(_) => TrackerState::Done,
        }
    }
}
