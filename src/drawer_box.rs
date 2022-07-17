use crate::Inches;
use derive_more::{Display, From, Into};
use std::fmt::{Display, Formatter};

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, strum::Display, strum_macros::EnumIter, Hash,
)]
pub enum Height {
    S,
    M,
    L,
    XL,
}

impl Default for Height {
    fn default() -> Self {
        Self::L
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, From, Into, Display, Hash)]
#[display(fmt = "{}", _0)]
pub struct Width(Inches);

impl Eq for Width {}

impl Width {
    pub fn w18() -> Self {
        Width(Inches(18.0))
    }

    pub fn w24() -> Self {
        Width(Inches(24.0))
    }

    pub fn w30() -> Self {
        Width(Inches(30.0))
    }

    pub fn w36() -> Self {
        Width(Inches(36.0))
    }

    pub fn custom(width: Inches) -> Self {
        Width(width)
    }
}

impl Default for Width {
    fn default() -> Self {
        Self::w24()
    }
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, strum::Display, strum_macros::EnumIter, Hash,
)]
pub enum Depth {
    #[strum(serialize = "12\"")]
    D12,
    #[strum(serialize = "14\"")]
    D14,
    #[strum(serialize = "16\"")]
    D16,
    //#[strum(serialize = "18\"")]
    //D18,
    #[strum(serialize = "20\"")]
    D20,
}

impl Default for Depth {
    fn default() -> Self {
        Self::D14
    }
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash, Default)]
pub struct DrawerBoxLine {
    pub quantity: isize,
    pub drawer_box: DrawerBox,
}

impl Display for DrawerBoxLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({})x {}", self.quantity, self.drawer_box))
    }
}

impl DrawerBox {
    pub fn line(self, qty: isize) -> DrawerBoxLine {
        DrawerBoxLine {
            quantity: qty,
            drawer_box: self,
        }
    }
}

impl From<DrawerBox> for DrawerBoxLine {
    fn from(drawer_box: DrawerBox) -> Self {
        drawer_box.line(1)
    }
}
