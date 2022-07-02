use crate::Inches;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Eq, strum::Display, strum_macros::EnumIter, Hash)]
pub enum Height {
    S,
    MS,
    M,
    L,
    XL,
    FILE,
}

#[derive(Debug, Copy, Clone, PartialEq, strum::Display, strum_macros::EnumIter, Hash)]
pub enum Width {
    #[strum(serialize = "18")]
    W18,
    #[strum(serialize = "24")]
    W24,
    #[strum(serialize = "30")]
    W30,
    #[strum(serialize = "36")]
    W36,
    Custom(Inches),
}

impl Width {
    pub fn inches(self) -> Inches {
        match self {
            Width::W18 => Inches::from(18.0),
            Width::W24 => Inches::from(24.0),
            Width::W30 => Inches::from(30.0),
            Width::W36 => Inches::from(36.0),
            Width::Custom(c) => c,
        }
    }
}

impl Eq for Width {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, strum::Display, strum_macros::EnumIter, Hash)]
pub enum Depth {
    #[strum(serialize = "12")]
    D12,
    #[strum(serialize = "14")]
    D14,
    #[strum(serialize = "16")]
    D16,
    #[strum(serialize = "20")]
    D20,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct DrawerBox {
    pub height: Height,
    pub width: Width,
    pub depth: Depth,
}

impl Display for DrawerBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {} x {}",
            self.height, self.width, self.depth
        ))
    }
}
