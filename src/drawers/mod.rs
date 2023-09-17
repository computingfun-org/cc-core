pub mod valen_box;

use measurements::length::Length;

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
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Height {
    S,
    M,
    #[default]
    L,
    XL,
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    PartialOrd,
    derive_more::From,
    derive_more::Into,
    derive_more::Display,
)]
#[display(fmt = "{}", _0)]
pub struct Width(Length);

impl Eq for Width {}

impl Width {
    pub fn standard_18() -> Self {
        Width(Length::from_inches(18.0))
    }

    pub fn standard_24() -> Self {
        Width(Length::from_inches(24.0))
    }

    pub fn standard_30() -> Self {
        Width(Length::from_inches(30.0))
    }

    pub fn standard_36() -> Self {
        Width(Length::from_inches(30.0))
    }

    pub fn custom(width: f64) -> Self {
        Width(Length::from_inches(width))
    }

    pub fn custom_mm(width: f64) -> Self {
        Width(Length::from_millimeters(width))
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
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Depth {
    #[strum(serialize = "12\"")]
    D12,
    #[default]
    #[strum(serialize = "14\"")]
    D14,
    #[strum(serialize = "16\"")]
    D16,
    // 18"D drawers are no longer available
    //#[strum(serialize = "18\"")]
    //D18,
    #[strum(serialize = "20\"")]
    D20,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct DrawerBox {
    pub height: Height,
    pub width: Width,
    pub depth: Depth,
}

impl std::fmt::Display for DrawerBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {} x {}",
            self.height, self.width, self.depth
        ))
    }
}
