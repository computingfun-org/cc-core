use serde::{Deserialize, Serialize};

use crate::lead_time::LeadTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Manufacturer {
    #[default]
    None,
    Central,
    Local,
}

impl std::fmt::Display for Manufacturer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Manufacturer::None => "None",
            Manufacturer::Central => "Central (EMC/XMC)",
            Manufacturer::Local => "Local (CabCon)",
        })
    }
}

impl Manufacturer {
    pub fn lead(&self) -> LeadTime {
        match self {
            Manufacturer::None => LeadTime::new(0),
            Manufacturer::Central => LeadTime::new(15),
            Manufacturer::Local => LeadTime::new(10),
        }
    }
}
