use chrono::{Datelike, Days, NaiveDate, Weekday};
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

#[derive(Debug, Constructor, Clone, Serialize, Deserialize)]
pub struct LeadTimeLine {
    pub flip: LeadTime,
    pub process: LeadTime,
    pub manufacturer: LeadTime,
    pub custom: LeadTime,
}

impl LeadTimeLine {
    pub fn order(&self) -> LeadTime {
        std::cmp::max(self.manufacturer, self.custom)
    }

    pub fn process_plus_order(&self) -> LeadTime {
        self.order() + self.process
    }

    pub fn total(&self) -> LeadTime {
        self.process_plus_order() + self.flip
    }

    pub fn min_install(&self, start: NaiveDate) -> Option<NaiveDate> {
        add_lead(start, self.total())
    }

    pub fn flip_by(&self, install: NaiveDate) -> Option<NaiveDate> {
        sub_lead(install, self.process_plus_order())
    }
}
