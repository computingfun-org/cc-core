use crate::drawer_box::{Depth, DrawerBox, Height, Width};
use crate::Inches;
use std::fmt::{Display, Formatter};

impl Height {
    pub fn kbc(self) -> Inches {
        match self {
            Height::S => Inches::from(3.0),
            Height::M => Inches::from(4.0),
            Height::L => Inches::from(6.0),
            Height::XL => Inches::from(10.0),
        }
    }
}

#[cfg(test)]
mod width_tests {
    use crate::drawer_box::Width;
    use crate::Inches;

    fn width_to_kbc(inches: f64) -> f64 {
        Width::from(Inches::from(inches)).kbc().into()
    }

    #[test]
    fn width() {
        assert_eq!(width_to_kbc(18.0), 17.625);
        assert_eq!(width_to_kbc(24.0), 23.625);
        assert_eq!(width_to_kbc(30.0), 29.625);
        assert_eq!(width_to_kbc(36.0), 35.625);
    }
}

impl Width {
    pub fn kbc(self) -> Inches {
        Inches::from(self) - Inches::from(0.375)
    }
}

impl Depth {
    pub fn kbc(self) -> Inches {
        match self {
            Depth::D12 => Inches::from(11.75),
            Depth::D14 => Inches::from(13.312),
            Depth::D16 => Inches::from(15.683),
            Depth::D20 => Inches::from(19.625),
        }
    }
}

pub struct KBCBox {
    pub height: Inches,
    pub width: Inches,
    pub depth: Inches,
}

impl Display for KBCBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} x {} x {}",
            self.height, self.width, self.depth
        ))
    }
}

impl From<DrawerBox> for KBCBox {
    fn from(drawer: DrawerBox) -> Self {
        KBCBox {
            height: drawer.height.kbc(),
            width: drawer.width.kbc(),
            depth: drawer.depth.kbc(),
        }
    }
}
