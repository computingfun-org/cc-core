use chrono::NaiveDate;

use crate::JobNumber;

use super::{
    lead_time::{self, LeadTime},
    notes::Notes,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Request {
    pub job: JobNumber,
    pub notes: Notes,
    pub flip: Option<LeadTime>,
    pub process: Option<LeadTime>,
}

impl Request {
    pub fn after_flip(&self) -> Option<LeadTime> {
        Some(self.notes.order()? + self.process?)
    }

    pub fn total_lead(&self) -> Option<LeadTime> {
        Some(self.after_flip()? + self.flip?)
    }

    pub fn min_install(&self, start: NaiveDate) -> Option<NaiveDate> {
        lead_time::add_lead(start, self.total_lead()?)
    }

    pub fn flip_by(&self, install: NaiveDate) -> Option<NaiveDate> {
        lead_time::sub_lead(install, self.after_flip()?)
    }
}
