use chrono::NaiveDate;

use super::lead_time::{self, LeadOrDate, LeadTime};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tracker {
    pub complete: bool,
    #[serde(with = "super::naive_date_serde")]
    pub complete_by: NaiveDate,
}

impl Tracker {
    pub fn from_lead(lead: LeadTime, start: NaiveDate) -> Option<Self> {
        Some(Self {
            complete: false,
            complete_by: lead_time::add_lead(start, lead)?,
        })
    }

    pub fn from_date(date: NaiveDate) -> Self {
        Self {
            complete: false,
            complete_by: date,
        }
    }

    pub fn from_lead_or_date(ld: LeadOrDate, start: NaiveDate) -> Option<Self> {
        match ld {
            LeadOrDate::Lead(lead) => Some(Self::from_lead(lead, start)?),
            LeadOrDate::Date(date) => Some(Self::from_date(date)),
        }
    }
}
