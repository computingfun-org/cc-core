use std::num::NonZeroUsize;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Note {
    pub tags: Vec<Box<str>>,
    pub price: usize,
    pub spaces: usize,
    pub access: Access,
    pub tear_out: TearOut,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Access {
    House(Floors),
    TownHouse(Floors),
    Condo(String),
    Other(String, Floors),
}

impl Default for Access {
    fn default() -> Self {
        Self::House(Floors::default())
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::From,
    derive_more::Into,
    derive_more::FromStr,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(transparent)]
pub struct Floors(NonZeroUsize);

impl Default for Floors {
    fn default() -> Self {
        Self(NonZeroUsize::MIN)
    }
}

impl std::fmt::Display for Floors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            NonZeroUsize::MIN => f.write_str("1st floor"),
            _ => f.write_fmt(format_args!("{} floors", self.0)),
        }
    }
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub enum TearOut {
    #[default]
    None,
    LightWire,
    Wire,
    HeavyWire,
    VentilatedWood,
    CCSystem,
    Custom(String),
}

impl std::fmt::Display for TearOut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TearOut::None => f.write_str("no tear out"),
            TearOut::LightWire => f.write_str("light wire tear out"),
            TearOut::Wire => f.write_str("wire tear out"),
            TearOut::HeavyWire => f.write_str("heavy wire tear out"),
            TearOut::VentilatedWood => f.write_str("ventilated wood tear out"),
            TearOut::CCSystem => f.write_str("CC tear out"),
            TearOut::Custom(tear_out) => f.write_str(&tear_out),
        }
    }
}
