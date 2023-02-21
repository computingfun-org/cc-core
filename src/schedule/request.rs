use chrono::NaiveDate;

use crate::JobNumber;

use super::{
    lead_time::{Backwards, Forwards, LeadTime, Stepper},
    notes::Notes,
};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Request {
    pub job: JobNumber,
    pub notes: Notes,
    pub flip_lead: LeadTime,
    pub process_lead: LeadTime,
}

impl Request {
    pub fn lead_time(&self) -> LeadTime {
        self.notes.lead_time() + self.flip_lead + self.process_lead
    }

    pub fn min_install(&self, start: NaiveDate) -> NaiveDate {
        Forwards::time(start, self.lead_time())
    }

    pub fn flip_by(&self, install: NaiveDate) -> NaiveDate {
        Backwards::time(install, self.notes.lead_time() + self.process_lead)
    }
}
