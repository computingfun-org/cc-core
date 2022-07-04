use crate::drawer::{Depth, DrawerBox, Height, Width};
use crate::Millimeters;
use derive_more::Display;

impl Depth {
    pub fn valen_box(self) -> Millimeters {
        match self {
            Depth::D12 => Millimeters::from(-1.0),
            Depth::D14 => Millimeters::from(327.0),
            Depth::D16 => Millimeters::from(-1.0),
            Depth::D20 => Millimeters::from(487.0),
        }
    }

    pub fn valen_bottom(self) -> Millimeters {
        match self {
            Depth::D12 => Millimeters::from(-1.0),
            Depth::D14 => Millimeters::from(318.0),
            Depth::D16 => Millimeters::from(-1.0),
            Depth::D20 => Millimeters::from(478.0),
        }
    }
}

impl Width {
    pub fn valen_box(self) -> Millimeters {
        match self {
            Width::W18 => Millimeters::from(441.0),
            Width::W24 => Millimeters::from(594.0),
            Width::W30 => Millimeters::from(746.0),
            Width::W36 => Millimeters::from(898.0),
            Width::Custom(_) => Millimeters::from(-1.0),
        }
    }

    pub fn valen_bottom(self) -> Millimeters {
        match self {
            Width::W18 => Millimeters::from(427.0),
            Width::W24 => Millimeters::from(580.0),
            Width::W30 => Millimeters::from(732.0),
            Width::W36 => Millimeters::from(884.0),
            Width::Custom(_) => Millimeters::from(-1.0),
        }
    }
}

impl Height {
    pub fn valen_box(self) -> Millimeters {
        match self {
            Height::S | Height::MS => Millimeters::from(70.0),
            Height::M => Millimeters::from(115.0),
            Height::L => Millimeters::from(170.0),
            Height::XL | Height::FILE => Millimeters::from(253.0),
        }
    }
}

impl DrawerBox {
    pub fn valen(self) -> ValenBox {
        ValenBox::from(self)
    }
}

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
    pub width: Millimeters,
    pub height: Millimeters,
    pub depth: Millimeters,
    pub bottom_depth: Millimeters,
    pub bottom_width: Millimeters,
}

impl From<DrawerBox> for ValenBox {
    fn from(drawer: DrawerBox) -> Self {
        Self {
            width: drawer.width.valen_box(),
            height: drawer.height.valen_box(),
            depth: drawer.depth.valen_box(),
            bottom_depth: drawer.depth.valen_bottom(),
            bottom_width: drawer.width.valen_bottom(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{} x {}", width, height)]
pub struct ValenFront {
    pub width: Millimeters,
    pub height: Millimeters,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{} x {}", width, height)]
pub struct ValenBack {
    pub width: Millimeters,
    pub height: Millimeters,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{} x {}", depth, height)]
pub struct ValenSideLogo {
    pub depth: Millimeters,
    pub height: Millimeters,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{} x {}", depth, height)]
pub struct ValenSidePlane {
    pub depth: Millimeters,
    pub height: Millimeters,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{} x {}", depth, width)]
pub struct ValenBottom {
    pub depth: Millimeters,
    pub width: Millimeters,
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

impl ValenBox {
    pub fn front(&self) -> ValenFront {
        ValenFront::from(*self)
    }

    pub fn back(&self) -> ValenBack {
        ValenBack::from(*self)
    }

    pub fn side_logo(&self) -> ValenSideLogo {
        ValenSideLogo::from(*self)
    }

    pub fn side_plane(&self) -> ValenSidePlane {
        ValenSidePlane::from(*self)
    }

    pub fn bottom(&self) -> ValenBottom {
        ValenBottom::from(*self)
    }
}
