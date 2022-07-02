use crate::drawer::{Depth, DrawerBox, Height, Width};
use crate::Inches;

pub fn height(h: Height) -> Inches {
    match h {
        Height::S | Height::MS => Inches::from(3.0),
        Height::M => Inches::from(4.0),
        Height::L => Inches::from(6.0),
        Height::XL | Height::FILE => Inches::from(10.0),
    }
}

pub fn width(w: Width) -> Inches {
    w.inches() + Inches::from(-0.375)
}

pub fn depth(d: Depth) -> Inches {
    match d {
        Depth::D12 => Inches::from(11.75),
        Depth::D14 => Inches::from(13.312),
        Depth::D16 => Inches::from(15.683),
        Depth::D20 => Inches::from(19.625),
    }
}

pub fn drawer_box(drawer: DrawerBox) -> (Inches, Inches, Inches) {
    (
        height(drawer.height),
        width(drawer.width),
        depth(drawer.depth),
    )
}

pub fn drawer_box_string(drawer: DrawerBox) -> String {
    let (h, w, d) = drawer_box(drawer);
    format!("{} x {} x {}", h, w, d)
}
