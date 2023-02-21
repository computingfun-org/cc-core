use non_empty_string::NonEmptyString;

use super::lead_time::LeadTime;

#[derive(
    Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, derive_more::Display,
)]
#[display(fmt = "{} {} {}", name, note, lead)]
pub struct Manufacturer {
    pub name: NonEmptyString,
    pub note: NonEmptyString,
    pub lead: LeadTime,
}

impl Manufacturer {
    pub fn central() -> Self {
        Self {
            name: NonEmptyString::new(format!("Central (EMC/XMC)")).unwrap(),
            note: NonEmptyString::new(format!("EMC")).unwrap(),
            lead: LeadTime::new(15),
        }
    }

    pub fn local() -> Self {
        Self {
            name: NonEmptyString::new(format!("Local (CabCon)")).unwrap(),
            note: NonEmptyString::new(format!("CabCon")).unwrap(),
            lead: LeadTime::new(10),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Notes {
    pub tags: Vec<NonEmptyString>,
    pub pricing: usize,
    pub spaces: usize,
    pub access: NonEmptyString,
    pub tear_out: NonEmptyString,
    pub manufacturer: Manufacturer,
    pub custom_lead: LeadTime,
}

impl Notes {
    pub fn lead_time(&self) -> LeadTime {
        std::cmp::max(self.manufacturer.lead, self.custom_lead)
    }
}

impl Notes {
    pub fn to_string_tags(&self) -> String {
        let mut print = String::new();
        for tag in self.tags.iter() {
            print.push_str(&format!("{tag} "));
        }
        print
    }

    pub fn to_string_pricing(&self) -> String {
        format!(
            "${}",
            human_format::Formatter::new()
                .with_decimals(2)
                .format(self.pricing as f64)
        )
    }

    pub fn to_string_spaces(&self) -> String {
        format!("{} space(s)", self.spaces)
    }

    pub fn to_string_access(&self) -> String {
        format!("{} access", self.access)
    }

    pub fn to_string_tear_out(&self) -> String {
        format!("{} tear out", self.tear_out)
    }

    pub fn to_string_custom(&self) -> String {
        match self.custom_lead.strict() {
            Some(custom) => format!("{custom} custom lead"),
            None => format!("no custom"),
        }
    }
}

impl std::fmt::Display for Notes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{}, {}, {}, {}, {}, {}",
            self.to_string_tags(),
            self.to_string_pricing(),
            self.to_string_spaces(),
            self.to_string_access(),
            self.to_string_tear_out(),
            self.manufacturer.note.as_str(),
            self.to_string_custom(),
        ))
    }
}
