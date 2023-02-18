use serde::{Deserialize, Serialize};

use super::LeadTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Manufacturer {
    #[default]
    None,
    Central,
    Local,
    Custom,
}

impl std::fmt::Display for Manufacturer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Manufacturer::None => "None",
            Manufacturer::Central => "Central (EMC/XMC)",
            Manufacturer::Local => "Local (CabCon)",
            Manufacturer::Custom => "Custom",
        })
    }
}

impl Manufacturer {
    pub fn lead(&self) -> LeadTime {
        match self {
            Manufacturer::None => LeadTime::new(0),
            Manufacturer::Central => LeadTime::new(15),
            Manufacturer::Local => LeadTime::new(10),
            Manufacturer::Custom => LeadTime::new(0),
        }
    }
}
