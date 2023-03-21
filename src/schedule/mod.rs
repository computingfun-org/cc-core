use std::num::NonZeroUsize;

use chrono::{NaiveDate, NaiveDateTime};
use non_empty_string::NonEmptyString;

use crate::{JobNumber, JobURL};

use self::lead_time::{LeadTime, Stepper};

pub mod btree_naive_date_time_serde;
pub mod lead_time;
pub mod naive_date_serde;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Schedule {
    pub job_number: JobNumber,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<NonEmptyString>,
    #[serde(default, skip_serializing_if = "serde_skip_pricing")]
    pub pricing: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spaces: Option<NonZeroUsize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access: Option<NonEmptyString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tear_out: Option<NonEmptyString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manufacturer_name: Option<NonEmptyString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flip_lead: Option<LeadTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_lead: Option<LeadTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manufacturer_lead: Option<LeadTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_lead: Option<LeadTime>,

    #[serde(
        with = "self::btree_naive_date_time_serde",
        default,
        skip_serializing_if = "std::collections::BTreeSet::is_empty"
    )]
    pub install: std::collections::BTreeSet<NaiveDateTime>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub deposit: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub confimation: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub flip: bool,
}

fn serde_skip_pricing(value: &Option<f64>) -> bool {
    match value {
        Some(inner) => !inner.is_normal(),
        None => true,
    }
}

impl Schedule {
    pub fn new(job_number: JobNumber) -> Self {
        Self {
            job_number,
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
        let min = lead_time::Forwards::time(now, self.lead());
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
        self.job_number.url()
    }
}

impl Schedule {
    const SEP: &str = ", ";

    pub fn to_string_tags(&self) -> Option<NonEmptyString> {
        NonEmptyString::new(
            self.tags
                .iter()
                .map(|tag| tag.as_str())
                .collect::<Vec<&str>>()
                .join(Self::SEP),
        )
        .ok()
    }

    pub fn to_string_pricing(&self) -> Option<NonEmptyString> {
        let pricing = self.pricing?;
        let text = std::panic::catch_unwind(|| {
            human_format::Formatter::new()
                .with_decimals(1)
                .format(pricing)
        })
        .ok()?;
        NonEmptyString::new(format!("${text}")).ok()
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

    pub fn notes(&self) -> Option<NonEmptyString> {
        NonEmptyString::new(
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
            .flatten()
            .map(|non_empty_string| non_empty_string.into_inner())
            .collect::<Vec<String>>()
            .join(Self::SEP),
        )
        .ok()
    }
}

impl std::fmt::Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            self.notes()
                .as_ref()
                .map(|notes| notes.as_str())
                .unwrap_or_default(),
        )
    }
}
