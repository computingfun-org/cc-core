use chrono::{NaiveDate, NaiveDateTime};

pub mod btree_naive_date_time_serde;
pub mod manufacturer;
pub mod naive_date_serde;

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
    #[serde(with = "self::naive_date_serde")]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScheduledJob {
    pub job: crate::JobNumber,
    pub tags: Vec<String>,
    pub pricing: Option<usize>,
    pub spaces: Option<usize>,
    pub access: Option<String>,
    pub tear_out: Option<String>,
    pub manufacturer: Option<manufacturer::Manufacturer>,
    pub custom: Option<LeadTime>,
    pub flip_lead: Option<LeadTime>,
    pub process_lead: Option<LeadTime>,
    #[serde(with = "self::btree_naive_date_time_serde")]
    pub install: std::collections::BTreeSet<NaiveDateTime>,
    pub is_paid: bool,
    pub is_confirm: bool,
    pub is_flip: bool,
}

impl ScheduledJob {
    pub fn order(&self) -> Option<LeadTime> {
        Some(std::cmp::max(self.manufacturer?.lead(), self.custom?))
    }

    pub fn after_flip(&self) -> Option<LeadTime> {
        Some(self.order()? + self.process_lead?)
    }

    pub fn total_lead(&self) -> Option<LeadTime> {
        Some(self.after_flip()? + self.flip_lead?)
    }

    pub fn min_install(&self, start: NaiveDate) -> Option<NaiveDate> {
        add_lead(start, self.total_lead()?)
    }

    pub fn first_install(&self) -> Option<NaiveDateTime> {
        self.install.first().copied()
    }

    pub fn flip_by(&self) -> Option<NaiveDate> {
        sub_lead(self.first_install()?.date(), self.after_flip()?)
    }
}

impl ScheduledJob {
    const SEPARATOR: &str = ", ";

    pub fn notes_tags(&self) -> Option<String> {
        let mut print = String::new();
        for tag in self.tags.iter() {
            print.push_str(&format!("{tag}{}", Self::SEPARATOR));
        }
        match print.is_empty() {
            true => None,
            false => Some(print),
        }
    }

    pub fn notes_pricing(&self) -> Option<String> {
        Some(format!(
            "${}{}",
            human_format::Formatter::new()
                .with_decimals(2)
                .format(self.pricing? as f64),
            Self::SEPARATOR
        ))
    }

    pub fn notes_spaces(&self) -> Option<String> {
        Some(format!("{} space(s){}", self.spaces?, Self::SEPARATOR))
    }

    pub fn notes_access(&self) -> Option<String> {
        if let Some(access) = &self.access {
            if !access.is_empty() {
                return Some(format!("{}{}", access, Self::SEPARATOR));
            }
        }
        None
    }

    pub fn notes_tear_out(&self) -> Option<String> {
        if let Some(tear_out) = &self.tear_out {
            if !tear_out.is_empty() {
                return Some(format!("{} tear out{}", tear_out, Self::SEPARATOR));
            }
        }
        None
    }

    pub fn notes_manufacturer(&self) -> Option<String> {
        match self.manufacturer? {
            manufacturer::Manufacturer::None => Some(format!("no manufacturer{}", Self::SEPARATOR)),
            manufacturer::Manufacturer::Central => Some(format!("emc{}", Self::SEPARATOR)),
            manufacturer::Manufacturer::Local => Some(format!("cabcon{}", Self::SEPARATOR)),
            manufacturer::Manufacturer::Custom => Some(format!("custom{}", Self::SEPARATOR)),
        }
    }

    pub fn notes_custom(&self) -> Option<String> {
        if let Some(custom) = self.custom {
            return match custom.is_some() {
                true => Some(format!("{custom} custom lead{}", Self::SEPARATOR)),
                false => Some(format!("no custom{}", Self::SEPARATOR)),
            };
        }
        None
    }

    pub fn notes(&self) -> String {
        let mut notes_print = String::new();
        if let Some(note_str) = self.notes_tags() {
            notes_print.push_str(&note_str);
        }
        if let Some(note_str) = self.notes_pricing() {
            notes_print.push_str(&note_str);
        }
        if let Some(note_str) = self.notes_spaces() {
            notes_print.push_str(&note_str);
        }
        if let Some(note_str) = self.notes_access() {
            notes_print.push_str(&note_str);
        }
        if let Some(note_str) = self.notes_tear_out() {
            notes_print.push_str(&note_str);
        }
        if let Some(note_str) = self.notes_manufacturer() {
            notes_print.push_str(&note_str);
        }
        if let Some(note_str) = self.notes_custom() {
            notes_print.push_str(&note_str);
        }
        notes_print
    }
}
