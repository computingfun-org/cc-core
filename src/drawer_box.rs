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

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, From, Into, Display, Hash)]
#[display(fmt = "{}", _0)]
pub struct Width(Inches);

impl Eq for Width {}

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
    #[strum(serialize = "20\"")]
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
