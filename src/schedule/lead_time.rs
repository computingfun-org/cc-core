use std::num::NonZeroUsize;

use chrono::NaiveDate;

#[derive(
    Debug,
    Default,
    derive_more::Constructor,
    derive_more::From,
    derive_more::Into,
    Clone,
    Copy,
    derive_more::FromStr,
    derive_more::Add,
    derive_more::Sub,
    derive_more::Display,
    derive_more::Not,
    derive_more::Sum,
    derive_more::AddAssign,
    derive_more::SubAssign,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(transparent)]
pub struct LeadTime(usize);

impl LeadTime {
    pub fn lax(self) -> usize {
        self.into()
    }

    pub fn strict(self) -> Option<NonZeroUsize> {
        self.into()
    }
}

impl From<NonZeroUsize> for LeadTime {
    fn from(value: NonZeroUsize) -> Self {
        Self(value.get())
    }
}

impl From<Option<NonZeroUsize>> for LeadTime {
    fn from(value: Option<NonZeroUsize>) -> Self {
        Self(value.map_or(0, |v| v.get()))
    }
}

impl From<LeadTime> for Option<NonZeroUsize> {
    fn from(value: LeadTime) -> Self {
        NonZeroUsize::new(value.into())
    }
}

pub trait Stepper {
    fn step(date: NaiveDate) -> NaiveDate;

    fn time(date: NaiveDate, time: LeadTime) -> NaiveDate {
        let mut date = date;
        for _ in 0..time.into() {
            date = Self::step(date);
        }
        date
    }
}

fn is_weekend(date: &NaiveDate) -> bool {
    use chrono::Datelike;
    let weekday = date.weekday();
    weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun
}

pub struct Forwards;
impl Stepper for Forwards {
    fn step(date: NaiveDate) -> NaiveDate {
        match date.succ_opt() {
            Some(date) => match is_weekend(&date) {
                true => Self::step(date),
                false => date,
            },
            None => NaiveDate::MAX,
        }
    }
}

pub struct Backwards;
impl Stepper for Backwards {
    fn step(date: NaiveDate) -> NaiveDate {
        match date.pred_opt() {
            Some(date) => match is_weekend(&date) {
                true => Self::step(date),
                false => date,
            },
            None => NaiveDate::MIN,
        }
    }
}
