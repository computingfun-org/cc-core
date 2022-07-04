use crate::drawer::{Depth, DrawerBox, Height, Width};
use crate::Inches;
use std::fmt::{Display, Formatter};

impl Height {
    pub fn kbc(self) -> Inches {
        match self {
            Height::S | Height::MS => Inches::from(3.0),
            Height::M => Inches::from(4.0),
            Height::L => Inches::from(6.0),
            Height::XL | Height::FILE => Inches::from(10.0),
        }
    }
}

impl Width {
    pub fn kbc(self) -> Inches {
        self.inches() + Inches::from(-0.375)
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
    height: Inches,
    width: Inches,
    depth: Inches,
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

impl DrawerBox {
    pub fn kbc(self) -> KBCBox {
        KBCBox::from(self)
    }
}
