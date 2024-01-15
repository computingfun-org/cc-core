use chrono::NaiveDate;

use crate::{native_date_serde, DashJob};

impl DashJob {
    pub fn request_schedule(self, schedule_due: NaiveDate) -> PendingSchedule {
        PendingSchedule {
            job: self,
            due: schedule_due,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PendingSchedule {
    #[serde(flatten)]
    pub job: DashJob,
    #[serde(
        serialize_with = "native_date_serde::serialize",
        deserialize_with = "native_date_serde::deserialize"
    )]
    pub due: NaiveDate,
}

impl PendingSchedule {
    pub fn schedule(self, install: NaiveDate, paperwork_due: NaiveDate) -> PendingPaperwork {
        PendingPaperwork {
            job: self.job,
            install,
            due: paperwork_due,
        }
    }

    pub fn remove_request(self) -> DashJob {
        self.job
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PendingPaperwork {
    #[serde(flatten)]
    pub job: DashJob,
    #[serde(
        serialize_with = "native_date_serde::serialize",
        deserialize_with = "native_date_serde::deserialize"
    )]
    pub install: NaiveDate,
    #[serde(
        serialize_with = "native_date_serde::serialize",
        deserialize_with = "native_date_serde::deserialize"
    )]
    pub due: NaiveDate,
}

impl PendingPaperwork {
    pub fn turn_in(self, turned_in: NaiveDate, folder_due: NaiveDate) -> PendingFolder {
        PendingFolder {
            job: self.job,
            install: self.install,
            paperwork: turned_in,
            due: folder_due,
        }
    }

    pub fn remove_install(self) -> DashJob {
        self.job
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PendingFolder {
    #[serde(flatten)]
    pub job: DashJob,
    #[serde(
        serialize_with = "native_date_serde::serialize",
        deserialize_with = "native_date_serde::deserialize"
    )]
    pub install: NaiveDate,
    #[serde(
        serialize_with = "native_date_serde::serialize",
        deserialize_with = "native_date_serde::deserialize"
    )]
    pub paperwork: NaiveDate,
    #[serde(
        serialize_with = "native_date_serde::serialize",
        deserialize_with = "native_date_serde::deserialize"
    )]
    pub due: NaiveDate,
}

impl PendingFolder {
    pub fn make(self, made: NaiveDate, order_due: NaiveDate) -> PendingOrder {
        PendingOrder {
            job: self.job,
            install: self.install,
            paperwork: self.paperwork,
            folder: made,
            due: order_due,
        }
    }

    pub fn remove_paperwork(self, paperwork_due: NaiveDate) -> PendingPaperwork {
        PendingPaperwork {
            job: self.job,
            install: self.install,
            due: paperwork_due,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PendingOrder {
    #[serde(flatten)]
    pub job: DashJob,
    #[serde(
        serialize_with = "native_date_serde::serialize",
        deserialize_with = "native_date_serde::deserialize"
    )]
    pub install: NaiveDate,
    #[serde(
        serialize_with = "native_date_serde::serialize",
        deserialize_with = "native_date_serde::deserialize"
    )]
    pub paperwork: NaiveDate,
    #[serde(
        serialize_with = "native_date_serde::serialize",
        deserialize_with = "native_date_serde::deserialize"
    )]
    pub folder: NaiveDate,
    #[serde(
        serialize_with = "native_date_serde::serialize",
        deserialize_with = "native_date_serde::deserialize"
    )]
    pub due: NaiveDate,
}

impl PendingOrder {
    pub fn order(self, placed: NaiveDate) -> Ready {
        Ready {
            job: self.job,
            install: self.install,
            paperwork: self.paperwork,
            folder: self.folder,
            order: placed,
        }
    }

    pub fn remove_folder(self, folder_due: NaiveDate) -> PendingFolder {
        PendingFolder {
            job: self.job,
            install: self.install,
            paperwork: self.paperwork,
            due: folder_due,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ready {
    #[serde(flatten)]
    pub job: DashJob,
    #[serde(
        serialize_with = "native_date_serde::serialize",
        deserialize_with = "native_date_serde::deserialize"
    )]
    pub install: NaiveDate,
    #[serde(
        serialize_with = "native_date_serde::serialize",
        deserialize_with = "native_date_serde::deserialize"
    )]
    pub paperwork: NaiveDate,
    #[serde(
        serialize_with = "native_date_serde::serialize",
        deserialize_with = "native_date_serde::deserialize"
    )]
    pub folder: NaiveDate,
    #[serde(
        serialize_with = "native_date_serde::serialize",
        deserialize_with = "native_date_serde::deserialize"
    )]
    pub order: NaiveDate,
}
