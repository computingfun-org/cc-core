use std::{num::NonZeroUsize, panic};

use chrono::{NaiveDate, NaiveDateTime};
use non_empty_string::NonEmptyString;

use crate::{JobNumber, JobURL};

use self::lead_time::{LeadTime, Stepper};

pub mod btree_naive_date_time_serde;
pub mod lead_time;
pub mod naive_date_serde;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Schedule {
    pub job: JobNumber,

    #[serde(default)]
    pub tags: Vec<NonEmptyString>,
    pub pricing: Option<NonZeroUsize>,
    pub spaces: Option<NonZeroUsize>,
    pub access: Option<NonEmptyString>,
    pub tear_out: Option<NonEmptyString>,
    pub manufacturer_name: Option<NonEmptyString>,

    pub flip_lead: Option<LeadTime>,
    pub process_lead: Option<LeadTime>,
    pub manufacturer_lead: Option<LeadTime>,
    pub custom_lead: Option<LeadTime>,

    #[serde(default)]
    #[serde(with = "self::btree_naive_date_time_serde")]
    pub install: std::collections::BTreeSet<NaiveDateTime>,

    #[serde(default)]
    pub deposit: bool,
    #[serde(default)]
    pub confimation: bool,
    #[serde(default)]
    pub flip: bool,
}

impl Schedule {
    pub fn new(job: JobNumber) -> Self {
        Self {
            job,
            tags: Default::default(),
            pricing: Default::default(),
            spaces: Default::default(),
            access: Default::default(),
            tear_out: Default::default(),
            manufacturer_name: Default::default(),
            flip_lead: Default::default(),
            process_lead: Default::default(),
            manufacturer_lead: Default::default(),
            custom_lead: Default::default(),
            install: Default::default(),
            deposit: Default::default(),
            confimation: Default::default(),
            flip: Default::default(),
        }
    }

    pub fn order_lead(&self) -> LeadTime {
        std::cmp::max(
            self.manufacturer_lead.unwrap_or_default(),
            self.custom_lead.unwrap_or_default(),
        )
    }

    pub fn proccess_order_lead(&self) -> LeadTime {
        self.process_lead.unwrap_or_default() + self.order_lead()
    }

    pub fn lead(&self) -> LeadTime {
        self.flip_lead.unwrap_or_default() + self.proccess_order_lead()
    }

    pub fn min_install(&self, now: NaiveDate) -> (NaiveDate, bool) {
        let min = lead_time::Backwards::time(now, self.lead());
        let check = match self.install.first() {
            Some(first) => first.date() >= min,
            None => false,
        };
        (min, check)
    }

    pub fn flip_by(&self) -> Option<NaiveDate> {
        Some(lead_time::Backwards::time(
            self.install.first()?.date(),
            self.proccess_order_lead(),
        ))
    }

    pub fn watch(&self, now: NaiveDate, ahead: LeadTime) -> bool {
        if !self.deposit || !self.confimation {
            return true;
        }

        if !self.flip {
            let flip_by = match self.flip_by() {
                Some(flip_by) => flip_by,
                None => return true,
            };
            let watch_at = lead_time::Backwards::time(flip_by, ahead);
            if watch_at >= now {
                return true;
            }
        }

        false
    }

    pub fn dash_link(&self) -> JobURL {
        self.job.url()
    }
}

impl Schedule {
    pub fn to_string_tags(&self) -> Option<NonEmptyString> {
        let mut print = String::new();
        for tag in self.tags.iter() {
            print.push_str(&format!("{tag} "));
        }
        NonEmptyString::new(print).ok()
    }

    pub fn to_string_pricing(&self) -> Option<NonEmptyString> {
        NonEmptyString::new(format!(
            "${}",
            panic::catch_unwind(move || human_format::Formatter::new()
                .with_decimals(1)
                .format(self.pricing.unwrap().get() as f64))
            .ok()?
        ))
        .ok()
    }

    pub fn to_string_spaces(&self) -> Option<NonEmptyString> {
        NonEmptyString::new(format!("{} space(s)", self.spaces?.get())).ok()
    }

    pub fn to_string_access(&self) -> Option<NonEmptyString> {
        NonEmptyString::new(format!("{} access", self.access.clone()?)).ok()
    }

    pub fn to_string_tear_out(&self) -> Option<NonEmptyString> {
        NonEmptyString::new(format!("{} tear out", self.tear_out.clone()?)).ok()
    }

    pub fn to_string_custom(&self) -> Option<NonEmptyString> {
        match self.custom_lead?.strict() {
            Some(custom) => NonEmptyString::new(format!("{custom} custom lead")).ok(),
            None => NonEmptyString::new(format!("no custom")).ok(),
        }
    }
}

impl std::fmt::Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            vec![
                self.to_string_tags(),
                self.to_string_pricing(),
                self.to_string_spaces(),
                self.to_string_access(),
                self.to_string_tear_out(),
                self.manufacturer_name.clone(),
                self.to_string_custom(),
            ]
            .into_iter()
            .map(|tag| tag.map_or(String::new(), |t| t.to_string()))
            .collect::<Vec<String>>()
            .join(", ")
            .as_str(),
        )
    }
}
