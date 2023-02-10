use std::fmt::Display;

use crate::{lead_time::LeadTime, manufacturer::Manufacturer};

#[derive(Debug, Clone, Default)]
pub struct Note {
    pub pricing: String,
    pub spaces: Option<usize>,
    pub access: String,
    pub tear_out: String,
    pub manufacturer: Option<Manufacturer>,
    pub custom: Option<LeadTime>,
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.pricing.is_empty() {
            f.write_fmt(format_args!("${}, ", &self.pricing))?;
        }

        match self.spaces {
            Some(spaces) => match spaces {
                0 => {}
                1 => f.write_str("1 space, ")?,
                _ => f.write_fmt(format_args!("{} spaces, ", spaces))?,
            },
            None => {}
        };

        if !self.access.is_empty() {
            f.write_fmt(format_args!("{}, ", self.access))?;
        }

        if !self.tear_out.is_empty() {
            f.write_fmt(format_args!("{} tear out, ", self.tear_out))?;
        }

        match self.manufacturer {
            Some(manufacturer) => match manufacturer {
                Manufacturer::None => f.write_str("no manufacturer parts, ")?,
                Manufacturer::Central => f.write_str("EMC, ")?,
                Manufacturer::Local => f.write_str("CabCon, ")?,
                Manufacturer::CustomOnly => f.write_str("Custom Only, ")?,
            },
            None => {}
        };

        match self.custom {
            Some(lead) => f.write_fmt(format_args!("{} custom lead, ", lead))?,
            None => {}
        };

        Ok(())
    }
}
