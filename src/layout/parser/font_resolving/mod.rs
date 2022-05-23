use crate::platform::prelude::*;

mod gdi;
mod name;

pub struct FontInfo {
    pub family: String,
    pub italic: bool,
    pub weight: i32,
}

impl FontInfo {
    pub fn from_gdi(name: &str, bold: bool, italic: bool) -> Option<Self> {
        let mut font = gdi::Font::new(name, bold, italic)?;
        let mut dc = gdi::DeviceContext::new()?;
        dc.select_font(&mut font)?;
        let metrics = dc.get_font_metrics()?;
        let name_table = dc.get_font_table(*b"name")?;
        let family = name::look_up_family_name(&name_table)?;

        Some(Self {
            family,
            italic: metrics.tmItalic != 0,
            weight: metrics.tmWeight,
        })
    }
}
