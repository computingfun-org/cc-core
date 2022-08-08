use crate::Inches;
use derive_more::{Display, Into};
use std::fmt::{Display, Formatter};

pub const STANDARD_HEIGHTS: [Height; 4] = [Height::S, Height::M, Height::L, Height::XL];
pub const STANDARD_WIDTHS: [Width; 4] = [
    Width::standard_18(),
    Width::standard_24(),
    Width::standard_30(),
    Width::standard_36(),
];
pub const STANDARD_DEPTHS: [Depth; 2] = [Depth::D14, Depth::D20];

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum::Display,
    strum_macros::EnumIter,
    Hash,
)]
pub enum Height {
    S,
    M,
    #[default]
    L,
    XL,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Into, Display, Hash)]
#[display(fmt = "{}", _0)]
pub struct Width(Inches);

impl Eq for Width {}

impl Width {
    pub const fn standard_18() -> Self {
        Width(Inches(18.0))
    }

    pub const fn standard_24() -> Self {
        Width(Inches(24.0))
    }

    pub const fn standard_30() -> Self {
        Width(Inches(30.0))
    }

    pub const fn standard_36() -> Self {
        Width(Inches(36.0))
    }

    pub fn custom(width: Inches) -> Self {
        Width(width)
    }
}

impl Default for Width {
    fn default() -> Self {
        Self::standard_24()
    }
}

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum::Display,
    strum_macros::EnumIter,
    Hash,
)]
pub enum Depth {
    #[strum(serialize = "12\"")]
    D12,
    #[default]
    #[strum(serialize = "14\"")]
    D14,
    #[strum(serialize = "16\"")]
    D16,
    //#[strum(serialize = "18\"")]
    //D18,
    #[strum(serialize = "20\"")]
    D20,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash, Default)]
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
