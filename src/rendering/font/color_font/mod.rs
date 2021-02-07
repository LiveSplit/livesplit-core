use ttf_parser::{Face, Tag};

use crate::settings::Color;

mod colr;
mod cpal;
mod parse_util;

pub struct ColorTables<'f> {
    colr: &'f [u8],
    cpal: &'f [u8],
}

impl<'f> ColorTables<'f> {
    pub fn new(face: &Face<'f>) -> Option<Self> {
        Some(Self {
            colr: face.table_data(Tag::from_bytes(b"COLR"))?,
            cpal: face.table_data(Tag::from_bytes(b"CPAL"))?,
        })
    }

    pub fn look_up(
        &self,
        palette: usize,
        glyph: u16,
    ) -> Option<impl Iterator<Item = (u16, Option<Color>)> + '_> {
        let layers = colr::look_up(self.colr, glyph)?;
        Some(layers.iter().map(move |layer| {
            let entry_idx = layer.palette_entry_idx();
            let color = if entry_idx != 0xFFFF {
                cpal::look_up(self.cpal, palette, entry_idx)
                    .map(|col| [col.red, col.green, col.blue, col.alpha].into())
            } else {
                None
            };
            (layer.glyph_id(), color)
        }))
    }
}

pub fn iter_colored_glyphs(
    color_tables: &Option<ColorTables<'_>>,
    palette: usize,
    glyph: u16,
    mut f: impl FnMut(u16, Option<Color>),
) {
    if let Some(iter) = color_tables
        .as_ref()
        .and_then(|c| c.look_up(palette, glyph))
    {
        for (glyph, color) in iter {
            f(glyph, color);
        }
    } else {
        f(glyph, None)
    }
}
