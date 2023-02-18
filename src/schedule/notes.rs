use super::{lead_time::LeadTime, manufacturer::Manufacturer};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Notes {
    pub tags: Vec<String>,
    pub pricing: Option<usize>,
    pub spaces: Option<usize>,
    pub access: Option<String>,
    pub tear_out: Option<String>,
    pub manufacturer: Option<Manufacturer>,
    pub custom: Option<LeadTime>,
}

impl Notes {
    pub fn order(&self) -> Option<LeadTime> {
        Some(std::cmp::max(self.manufacturer?.lead(), self.custom?))
    }
}

impl std::fmt::Display for Notes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_tags(f)?;
        self.fmt_pricing(f)?;
        self.fmt_spaces(f)?;
        self.fmt_access(f)?;
        self.fmt_tear_out(f)?;
        self.fmt_manufacturer(f)?;
        self.fmt_custom(f)?;
        Ok(())
    }
}

impl Notes {
    const SEPARATOR: &str = ", ";

    pub fn fmt_tags(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for tag in self.tags.iter() {
            f.write_fmt(format_args!("{tag}{}", Self::SEPARATOR))?;
        }
        Ok(())
    }

    pub fn fmt_pricing(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(pricing) = self.pricing {
            let pretty = human_format::Formatter::new()
                .with_decimals(2)
                .format(pricing as f64);
            f.write_fmt(format_args!("${pretty}{}", Self::SEPARATOR))?;
        }
        Ok(())
    }

    pub fn fmt_spaces(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(spaces) = self.spaces {
            f.write_fmt(format_args!("{spaces} space(s){}", Self::SEPARATOR))?;
        }
        Ok(())
    }

    pub fn fmt_access(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(access) = &self.access {
            if !access.is_empty() {
                f.write_fmt(format_args!("{access}{}", Self::SEPARATOR))?;
            }
        }
        Ok(())
    }

    pub fn fmt_tear_out(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(tear_out) = &self.tear_out {
            if !tear_out.is_empty() {
                f.write_fmt(format_args!("{tear_out} tear out{}", Self::SEPARATOR))?;
            }
        }
        Ok(())
    }

    pub fn fmt_manufacturer(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(manufacturer) = self.manufacturer {
            match manufacturer {
                Manufacturer::None => {
                    f.write_fmt(format_args!("no manufacturer{}", Notes::SEPARATOR))?
                }
                Manufacturer::Central => f.write_fmt(format_args!("emc{}", Notes::SEPARATOR))?,
                Manufacturer::Local => f.write_fmt(format_args!("cabcon{}", Notes::SEPARATOR))?,
                Manufacturer::Custom => f.write_fmt(format_args!("custom{}", Notes::SEPARATOR))?,
            };
        }
        Ok(())
    }

    pub fn fmt_custom(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(custom) = self.custom {
            match custom.is_some() {
                true => f.write_fmt(format_args!("{custom} custom lead{}", Self::SEPARATOR))?,
                false => f.write_fmt(format_args!("no custom{}", Self::SEPARATOR))?,
            };
        }
        Ok(())
    }
}
