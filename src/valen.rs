use crate::drawer::{Depth, DrawerBox, Height, Width};
use crate::Millimeters;
use derive_more::{Display, From, Into};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
#[display(
    fmt = "{} x {} x {} x {} x {}",
    width,
    height,
    depth,
    bottom_depth,
    bottom_width
)]
pub struct ValenBox {
    pub width: BoxWidth,
    pub height: BoxHeight,
    pub depth: BoxDepth,
    pub bottom_depth: BottomDepth,
    pub bottom_width: BottomWidth,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{} x {}", width, height)]
pub struct ValenFront {
    pub width: BoxWidth,
    pub height: BoxHeight,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{} x {}", width, height)]
pub struct ValenBack {
    pub width: BoxWidth,
    pub height: BoxHeight,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{} x {}", depth, height)]
pub struct ValenSideLogo {
    pub depth: BoxDepth,
    pub height: BoxHeight,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{} x {}", depth, height)]
pub struct ValenSidePlane {
    pub depth: BoxDepth,
    pub height: BoxHeight,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{} x {}", depth, width)]
pub struct ValenBottom {
    pub depth: BottomDepth,
    pub width: BottomWidth,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, From, Into, Display)]
pub struct BoxDepth(Millimeters);

impl From<Depth> for BoxDepth {
    fn from(d: Depth) -> Self {
        match d {
            Depth::D12 => Self(Millimeters::from(-1.0)),
            Depth::D14 => Self(Millimeters::from(327.0)),
            Depth::D16 => Self(Millimeters::from(-1.0)),
            Depth::D20 => Self(Millimeters::from(487.0)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, From, Into, Display)]
pub struct BoxWidth(Millimeters);

impl From<Width> for BoxWidth {
    fn from(w: Width) -> Self {
        match w {
            Width::W18 => Self(Millimeters::from(441.0)),
            Width::W24 => Self(Millimeters::from(594.0)),
            Width::W30 => Self(Millimeters::from(746.0)),
            Width::W36 => Self(Millimeters::from(898.0)),
            Width::Custom(_) => Self(Millimeters::from(-1.0)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, From, Into, Display)]
pub struct BoxHeight(Millimeters);

impl From<Height> for BoxHeight {
    fn from(h: Height) -> Self {
        match h {
            Height::S | Height::MS => Self(Millimeters::from(70.0)),
            Height::M => Self(Millimeters::from(115.0)),
            Height::L => Self(Millimeters::from(170.0)),
            Height::XL | Height::FILE => Self(Millimeters::from(253.0)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, From, Into, Display)]
pub struct BottomDepth(Millimeters);

impl From<Depth> for BottomDepth {
    fn from(d: Depth) -> Self {
        match d {
            Depth::D12 => Self(Millimeters::from(-1.0)),
            Depth::D14 => Self(Millimeters::from(318.0)),
            Depth::D16 => Self(Millimeters::from(-1.0)),
            Depth::D20 => Self(Millimeters::from(478.0)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, From, Into, Display)]
pub struct BottomWidth(Millimeters);

impl From<Width> for BottomWidth {
    fn from(w: Width) -> Self {
        match w {
            Width::W18 => Self(Millimeters::from(427.0)),
            Width::W24 => Self(Millimeters::from(580.0)),
            Width::W30 => Self(Millimeters::from(732.0)),
            Width::W36 => Self(Millimeters::from(884.0)),
            Width::Custom(_) => Self(Millimeters::from(-1.0)),
        }
    }
}

impl From<DrawerBox> for ValenBox {
    fn from(drawer: DrawerBox) -> Self {
        Self {
            width: BoxWidth::from(drawer.width),
            height: BoxHeight::from(drawer.height),
            depth: BoxDepth::from(drawer.depth),
            bottom_depth: BottomDepth::from(drawer.depth),
            bottom_width: BottomWidth::from(drawer.width),
        }
    }
}

impl From<ValenBox> for ValenFront {
    fn from(vbox: ValenBox) -> Self {
        Self {
            width: vbox.width,
            height: vbox.height,
        }
    }
}

impl From<ValenBox> for ValenBack {
    fn from(vbox: ValenBox) -> Self {
        Self {
            width: vbox.width,
            height: vbox.height,
        }
    }
}

impl From<ValenBox> for ValenSideLogo {
    fn from(vbox: ValenBox) -> Self {
        Self {
            depth: vbox.depth,
            height: vbox.height,
        }
    }
}

impl From<ValenBox> for ValenSidePlane {
    fn from(vbox: ValenBox) -> Self {
        Self {
            depth: vbox.depth,
            height: vbox.height,
        }
    }
}

impl From<ValenBox> for ValenBottom {
    fn from(vbox: ValenBox) -> Self {
        Self {
            depth: vbox.bottom_depth,
            width: vbox.bottom_width,
        }
    }
}
