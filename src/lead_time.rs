use chrono::{Datelike, Days, Local, NaiveDate, Weekday};
use derive_more::{
    Add, AddAssign, Constructor, Display, From, FromStr, Into, Not, Sub, SubAssign, Sum,
};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Constructor,
    From,
    Into,
    Clone,
    Copy,
    FromStr,
    Add,
    Sub,
    Display,
    Not,
    Sum,
    AddAssign,
    SubAssign,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub struct LeadTime(usize);

const LEAD_DAYS_PER_WEEK: usize = 5;

impl LeadTime {
    pub fn weeks(&self) -> usize {
        self.0 / LEAD_DAYS_PER_WEEK
    }
}

impl TryFrom<LeadTime> for Days {
    type Error = std::num::TryFromIntError;
    fn try_from(value: LeadTime) -> Result<Self, Self::Error> {
        Ok(Self::new(value.0.try_into()?))
    }
}

pub fn skip_weekends_forwards(date: NaiveDate) -> Option<NaiveDate> {
    let mut date = date;
    while is_weekend(date) {
        date = date.succ_opt()?;
    }
    Some(date)
}

pub fn skip_weekends_backwards(date: NaiveDate) -> Option<NaiveDate> {
    let mut date = date;
    while is_weekend(date) {
        date = date.pred_opt()?;
    }
    Some(date)
}

pub fn is_workday(date: NaiveDate) -> bool {
    !is_weekend(date)
}

pub fn is_weekend(date: NaiveDate) -> bool {
    let weekday = date.weekday();
    weekday == Weekday::Sat || weekday == Weekday::Sun
}

pub fn add_lead(start: NaiveDate, lead: LeadTime) -> Option<NaiveDate> {
    let mut date = skip_weekends_forwards(start)?;
    for _ in 0..lead.0 {
        date = date.succ_opt()?;
        date = skip_weekends_forwards(date)?;
    }
    Some(date)
}

pub fn sub_lead(start: NaiveDate, lead: LeadTime) -> Option<NaiveDate> {
    let mut date = skip_weekends_backwards(start)?;
    for _ in 0..lead.0 {
        date = date.pred_opt()?;
        date = skip_weekends_backwards(date)?;
    }
    Some(date)
}

pub fn now_plus_lead(lead: LeadTime) -> Option<NaiveDate> {
    add_lead(Local::now().date_naive(), lead)
}

pub fn now_sub_lead(lead: LeadTime) -> Option<NaiveDate> {
    sub_lead(Local::now().date_naive(), lead)
}
