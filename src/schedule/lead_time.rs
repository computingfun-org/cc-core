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
pub struct LeadTime(isize);

const LEAD_DAYS_PER_WEEK: isize = 5;

impl LeadTime {
    pub fn weeks(&self) -> isize {
        self.0 / LEAD_DAYS_PER_WEEK
    }

    pub fn is_none(&self) -> bool {
        self.0 == 0
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn abs(&self) -> Self {
        Self(self.0.abs())
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
    let weekday = chrono::Datelike::weekday(&date);
    weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun
}

pub fn add_lead(start: NaiveDate, lead: LeadTime) -> Option<NaiveDate> {
    if lead.is_none() {
        return Some(start);
    }

    if lead.0 < 0 {
        return sub_lead(start, lead.abs());
    }

    let mut date = skip_weekends_forwards(start)?;
    for _ in 0..lead.0 {
        date = date.succ_opt()?;
        date = skip_weekends_forwards(date)?;
    }
    Some(date)
}

pub fn sub_lead(start: NaiveDate, lead: LeadTime) -> Option<NaiveDate> {
    if lead.is_none() {
        return Some(start);
    }

    if lead.0 < 0 {
        return add_lead(start, lead.abs());
    }

    let mut date = skip_weekends_backwards(start)?;
    for _ in 0..lead.0 {
        date = date.pred_opt()?;
        date = skip_weekends_backwards(date)?;
    }
    Some(date)
}

#[derive(
    Debug,
    derive_more::From,
    Clone,
    Copy,
    derive_more::Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum LeadOrDate {
    Lead(LeadTime),
    #[serde(with = "super::naive_date_serde")]
    Date(NaiveDate),
}

impl LeadOrDate {
    pub fn lead_or<F>(self, f: F) -> LeadTime
    where
        F: FnOnce(NaiveDate) -> LeadTime,
    {
        match self {
            LeadOrDate::Lead(lead) => lead,
            LeadOrDate::Date(date) => f(date),
        }
    }

    pub fn date_or<F>(self, f: F) -> NaiveDate
    where
        F: FnOnce(LeadTime) -> NaiveDate,
    {
        match self {
            LeadOrDate::Lead(lead) => f(lead),
            LeadOrDate::Date(date) => date,
        }
    }
}
