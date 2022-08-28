use crate::drawer_box::{Depth, DrawerBox, Height, Width};
use crate::Millimeters;

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

#[cfg(test)]
mod width_tests {
    use crate::drawer_box::Width;
    use crate::Inches;

    fn width_from_inches(inches: f64) -> Width {
        Width::custom(Inches::from(inches))
    }

    fn box_width_from_inches(inches: f64) -> f64 {
        f64::from(width_from_inches(inches).valen_box())
    }

    fn bottom_width_from_inches(inches: f64) -> f64 {
        f64::from(width_from_inches(inches).valen_bottom())
    }

    #[test]
    fn width_box() {
        assert_eq!(box_width_from_inches(18.0), 441.0);
        assert_eq!(box_width_from_inches(24.0), 594.0);
        assert_eq!(box_width_from_inches(30.0), 746.0);
        assert_eq!(box_width_from_inches(36.0), 898.0);
    }

    #[test]
    fn width_bottom() {
        assert_eq!(bottom_width_from_inches(18.0), 427.0);
        assert_eq!(bottom_width_from_inches(24.0), 580.0);
        assert_eq!(bottom_width_from_inches(30.0), 732.0);
        assert_eq!(bottom_width_from_inches(36.0), 884.0);
    }
}

impl Width {
    pub fn valen_box(self) -> Millimeters {
        Millimeters::from((self.millimeters().into_number() - 16.0).round())
    }

    pub fn valen_bottom(self) -> Millimeters {
        Millimeters::from((self.millimeters().into_number() - 30.0).round())
    }
}

impl Height {
    pub fn valen_box(self) -> Millimeters {
        match self {
            Height::S => Millimeters::from(70.0),
            Height::M => Millimeters::from(115.0),
            Height::L => Millimeters::from(170.0),
            Height::XL => Millimeters::from(253.0),
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Hash,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
)]
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

impl From<&DrawerBox> for ValenBox {
    fn from(drawer: &DrawerBox) -> Self {
        Self {
            width: drawer.width.valen_box(),
            height: drawer.height.valen_box(),
            depth: drawer.depth.valen_box(),
            bottom_depth: drawer.depth.valen_bottom(),
            bottom_width: drawer.width.valen_bottom(),
        }
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Hash,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
)]
#[display(fmt = "{} x {}", width, height)]
pub struct ValenFront {
    pub width: Millimeters,
    pub height: Millimeters,
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Hash,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
)]
#[display(fmt = "{} x {}", width, height)]
pub struct ValenBack {
    pub width: Millimeters,
    pub height: Millimeters,
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Hash,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
)]
#[display(fmt = "{} x {}", depth, height)]
pub struct ValenSideLogo {
    pub depth: Millimeters,
    pub height: Millimeters,
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Hash,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
)]
#[display(fmt = "{} x {}", depth, height)]
pub struct ValenSidePlane {
    pub depth: Millimeters,
    pub height: Millimeters,
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Hash,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
)]
#[display(fmt = "{} x {}", depth, width)]
pub struct ValenBottom {
    pub depth: Millimeters,
    pub width: Millimeters,
}

impl ValenBox {
    pub fn front(&self) -> ValenFront {
        ValenFront {
            width: self.width,
            height: self.height,
        }
    }

    pub fn back(&self) -> ValenBack {
        ValenBack {
            width: self.width,
            height: self.height,
        }
    }

    pub fn side_logo(&self) -> ValenSideLogo {
        ValenSideLogo {
            depth: self.depth,
            height: self.height,
        }
    }

    pub fn side_plane(&self) -> ValenSidePlane {
        ValenSidePlane {
            depth: self.depth,
            height: self.height,
        }
    }

    pub fn bottom(&self) -> ValenBottom {
        ValenBottom {
            depth: self.bottom_depth,
            width: self.bottom_width,
        }
    }
}
