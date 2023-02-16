use std::collections::BTreeSet;

use chrono::NaiveDate;

use crate::{
    job::JobNumber,
    lead_time::{self, LeadOrDate, LeadTime},
    manufacturer::Manufacturer,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Track {
    pub done: bool,
    #[serde(with = "crate::naive_date_serde")]
    pub limit: NaiveDate,
}

impl Track {
    pub fn from_lead(lead: LeadTime, start: NaiveDate) -> Self {
        Self {
            done: false,
            limit: lead_time::add_lead(start, lead).unwrap_or_default(),
        }
    }

    pub fn from_date(date: NaiveDate) -> Self {
        date.into()
    }

    pub fn from_lead_or_date(ld: LeadOrDate, start: NaiveDate) -> Self {
        match ld {
            LeadOrDate::Lead(lead) => Self::from_lead(lead, start),
            LeadOrDate::Date(date) => Self::from_date(date),
        }
    }
}

impl From<NaiveDate> for Track {
    fn from(value: NaiveDate) -> Self {
        Self {
            done: false,
            limit: value,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Notes {
    pub tags: Vec<String>,
    pub pricing: Option<usize>,
    pub spaces: Option<usize>,
    pub access: String,
    pub tear_out: String,
    pub manufacturer: Option<Manufacturer>,
    pub custom: Option<LeadTime>,
}

impl std::fmt::Display for Notes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_tags(f)?;
        self.fmt_price(f)?;
        self.fmt_spaces(f)?;
        self.fmt_access(f)?;
        self.fmt_tear_out(f)?;
        self.fmt_manufacturer(f)?;
        self.fmt_custom(f)?;
        Ok(())
    }
}

impl Notes {
    pub fn fmt_tags(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for tag in self.tags.iter() {
            f.write_fmt(format_args!("{tag}, "))?;
        }
        Ok(())
    }

    pub fn fmt_price(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.pricing {
            Some(price) => f.write_fmt(format_args!(
                "${}, ",
                human_format::Formatter::new()
                    .with_decimals(2)
                    .format(price as f64)
            )),
            None => Ok(()),
        }
    }

    pub fn fmt_spaces(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.spaces {
            Some(spaces) => f.write_fmt(format_args!("{spaces} space(s), ")),
            None => Ok(()),
        }
    }

    pub fn fmt_access(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.access.is_empty() {
            true => Ok(()),
            false => f.write_fmt(format_args!("{}, ", self.access)),
        }
    }

    pub fn fmt_tear_out(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.tear_out.is_empty() {
            true => Ok(()),
            false => f.write_fmt(format_args!("{} tear out, ", self.tear_out)),
        }
    }

    pub fn fmt_manufacturer(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.manufacturer {
            Some(manufacturer) => f.write_str(match manufacturer {
                Manufacturer::None => "no manufacturer parts, ",
                Manufacturer::Central => "EMC, ",
                Manufacturer::Local => "CabCon, ",
            }),
            None => Ok(()),
        }
    }

    pub fn fmt_custom(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.custom {
            Some(custom) => match custom.is_some() {
                true => f.write_fmt(format_args!("{custom} custom lead, ")),
                false => f.write_str("no custom, "),
            },
            None => Ok(()),
        }
    }
}

impl Notes {
    pub fn order(&self) -> LeadTime {
        std::cmp::max(
            self.manufacturer.unwrap_or_default().lead(),
            self.custom.unwrap_or_default(),
        )
    }

    pub fn request(self, job: Option<JobNumber>) -> Request {
        Request {
            job,
            notes: self,
            flip: None,
            process: None,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Request {
    pub job: Option<JobNumber>,
    pub notes: Notes,
    pub flip: Option<LeadTime>,
    pub process: Option<LeadTime>,
}

impl Request {
    pub fn process_plus_order(&self) -> LeadTime {
        self.notes.order() + self.process.unwrap_or_default()
    }

    pub fn total_lead(&self) -> LeadTime {
        self.process_plus_order() + self.flip.unwrap_or_default()
    }

    pub fn min_install(&self, start: NaiveDate) -> Option<NaiveDate> {
        lead_time::add_lead(start, self.total_lead())
    }

    pub fn flip_by(&self, install: NaiveDate) -> Option<NaiveDate> {
        lead_time::sub_lead(install, self.process_plus_order())
    }

    pub fn tentative(
        self,
        install: BTreeSet<NaiveDate>,
        deposit_by: LeadOrDate,
        confirm_by: LeadOrDate,
        flip_by: LeadOrDate,
    ) -> Result<Tentative, TentativeError> {
        let first_install = install
            .first()
            .ok_or(TentativeError::MisingInstallDate)?
            .to_owned();
        Ok(Tentative {
            job: self.job.ok_or(TentativeError::MissingJobNumber)?,
            notes: self.notes,
            install,
            deposit: Track::from_lead_or_date(deposit_by, first_install),
            confirm: Track::from_lead_or_date(confirm_by, first_install),
            flip: Track::from_lead_or_date(flip_by, first_install),
        })
    }
}

#[derive(Debug, Copy, Clone, derive_more::Display)]
pub enum TentativeError {
    MissingJobNumber,
    MisingInstallDate,
}

impl std::error::Error for TentativeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

#[derive(Debug, Clone /*, serde::Serialize, serde::Deserialize*/)]
pub struct Tentative {
    pub job: JobNumber,
    pub notes: Notes,
    pub install: BTreeSet<NaiveDate>,
    pub deposit: Track,
    pub confirm: Track,
    pub flip: Track,
}
