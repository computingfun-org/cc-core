use chrono::NaiveDate;
use non_empty_string::NonEmptyString;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Notes {
    pub tags: Vec<NonEmptyString>,
    pub price: String,
    pub spaces: usize,
    pub access: String,
    pub tear_out: String,
}

#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct State {
    pub deposit: bool,
    pub confirmation: bool,
    pub marked_ready: bool,
}

impl State {
    pub fn is_done(&self) -> bool {
        self.deposit && self.confirmation && self.marked_ready
    }
}

#[derive(
    Debug,
    derive_more::Constructor,
    derive_more::From,
    derive_more::Into,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
#[repr(transparent)]
pub struct LeadTime(chrono::Days);

#[derive(Debug, Clone)]
pub struct LeadItem {
    pub name: NonEmptyString,
    pub lead: LeadTime,
}

#[derive(Debug, Clone)]
pub struct TimeLine {
    pub install: NaiveDate,
    pub production: LeadTime,
    pub pre_production: LeadTime,
    pub paperwork: LeadTime,
}
