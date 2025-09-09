use cosmic_text::rustybuzz::ttf_parser::{Face, GlyphId, Tag};

use crate::settings::Color;

mod colr;
mod cpal;

const COLR: Tag = Tag::from_bytes(b"COLR");
const CPAL: Tag = Tag::from_bytes(b"CPAL");

pub struct ColorTables<'f> {
    colr: &'f [u8],
    cpal: &'f [u8],
}

impl<'f> ColorTables<'f> {
    pub fn new(face: &Face<'f>) -> Option<Self> {
        let raw_face = face.raw_face();
        Some(Self {
            colr: raw_face.table(COLR)?,
            cpal: raw_face.table(CPAL)?,
        })
    }

    pub fn look_up(
        &self,
        palette: usize,
        glyph: u16,
    ) -> Option<impl Iterator<Item = (u16, Option<Color>)> + use<'_>> {
        let layers = colr::look_up(self.colr, glyph)?;
        Some(layers.iter().map(move |layer| {
            let entry_idx = layer.palette_entry_idx();
            let color = if entry_idx != 0xFFFF {
                cpal::look_up(self.cpal, palette, entry_idx)
                    .map(|c| Color::rgba8(c.red, c.green, c.blue, c.alpha))
            } else {
                None
            };
            (layer.glyph_id(), color)
        }))
    }
}

pub fn iter_colored_glyphs(
    color_tables: &Option<ColorTables>,
    palette: usize,
    glyph: GlyphId,
    mut f: impl FnMut(GlyphId, Option<Color>),
) {
    if let Some(c) = color_tables
        && let Some(iter) = c.look_up(palette, glyph.0)
    {
        for (glyph, color) in iter {
            f(GlyphId(glyph), color);
        }
    } else {
        f(glyph, None)
    }
}
