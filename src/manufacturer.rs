use crate::lead_time::LeadTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Manufacturer {
    #[default]
    None,
    Central,
    Local,
    CustomOnly,
}

impl std::fmt::Display for Manufacturer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Manufacturer::None => "None",
            Manufacturer::Central => "Central (EMC/XMC)",
            Manufacturer::Local => "Local (CabCon)",
            Manufacturer::CustomOnly => "Custom Only",
        })
    }
}

impl Manufacturer {
    pub fn lead(&self) -> LeadTime {
        match self {
            Manufacturer::None => LeadTime::new(0),
            Manufacturer::Central => LeadTime::new(15),
            Manufacturer::Local => LeadTime::new(10),
            Manufacturer::CustomOnly => LeadTime::new(0),
        }
    }

    pub fn note(&self) -> &str {
        match self {
            Manufacturer::None => "",
            Manufacturer::Central => "EMC",
            Manufacturer::Local => "CabCon",
            Manufacturer::CustomOnly => "Custom Only",
        }
    }
}
