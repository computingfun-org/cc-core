use crate::Inches;
use derive_more::{Display, From, Into};
use std::fmt::{Display, Formatter};

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, strum::Display, strum_macros::EnumIter, Hash,
)]
pub enum Height {
    S,
    M,
    #[default]
    L,
    XL,
}

pub fn height_iter() -> HeightIter {
    use strum::IntoEnumIterator;
    Height::iter()
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Into, Display, Hash)]
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

pub fn width_iter() -> Vec<Width> {
    vec![Width::w18(), Width::w24(), Width::w30(), Width::w36()]
}

impl Default for Width {
    fn default() -> Self {
        Self::w24()
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

pub fn depth_iter() -> DepthIter {
    use strum::IntoEnumIterator;
    Depth::iter()
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

pub fn drawer_box_iter() -> Vec<DrawerBox> {
    let mut drawer_boxes = vec![];
    for height in height_iter() {
        for width in width_iter() {
            for depth in depth_iter() {
                drawer_boxes.push(DrawerBox {
                    height,
                    width,
                    depth,
                });
            }
        }
    }
    drawer_boxes
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash, Default)]
pub struct DrawerBoxLine<T: From<DrawerBox>> {
    pub quantity: usize,
    pub drawer_box: T,
}

impl<T: From<DrawerBox> + Display> Display for DrawerBoxLine<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({})x {}", self.quantity, self.drawer_box))
    }
}

impl<T: From<DrawerBox>> From<T> for DrawerBoxLine<T> {
    fn from(drawer_box: T) -> Self {
        Self {
            quantity: 1,
            drawer_box,
        }
    }
}
