use super::{decode_color, glyph_cache::GlyphCache, Backend, Pos, Transform};
use crate::settings::Color;
use rustybuzz::{Face, Feature, GlyphBuffer, Tag, UnicodeBuffer};
use ttf_parser::{GlyphId, OutlineBuilder};

pub struct Font<'fd> {
    rb: Face<'fd>,
    face: ttf_parser::Face<'fd>,
    scale_factor: f32,
}

impl<'fd> Font<'fd> {
    pub fn from_slice(data: &'fd [u8], index: u32) -> Option<Self> {
        let parser = ttf_parser::Face::from_slice(data, index).ok()?;
        Some(Self {
            scale_factor: 1.0 / parser.height() as f32,
            rb: Face::from_slice(data, index)?,
            face: parser,
        })
    }

    pub fn scale(&self, scale: f32) -> ScaledFont<'_> {
        ScaledFont {
            font: self,
            scale: scale * self.scale_factor,
        }
    }

    pub fn outline_glyph(&self, glyph_id: u32, builder: &mut dyn OutlineBuilder) -> bool {
        self.face
            .outline_glyph(GlyphId(glyph_id as u16), builder)
            .is_some()
    }

    pub fn descender(&self) -> f32 {
        self.face.descender() as f32
    }

    pub fn height(&self) -> f32 {
        self.face.height() as f32
    }
}

#[derive(Copy, Clone)]
pub struct ScaledFont<'f> {
    font: &'f Font<'f>,
    scale: f32,
}

impl<'f> ScaledFont<'f> {
    fn glyph_x_advance(&self, glyph_id: u32) -> f32 {
        self.font
            .face
            .glyph_hor_advance(GlyphId(glyph_id as _))
            .unwrap_or_default() as f32
            * self.scale
    }

    pub fn shape(self, buffer: UnicodeBuffer) -> Glyphs<'f> {
        Glyphs {
            buffer: rustybuzz::shape(&self.font.rb, &[], buffer),
            font: self,
        }
    }

    pub fn shape_tabular_numbers(self, buffer: UnicodeBuffer) -> Glyphs<'f> {
        Glyphs {
            buffer: rustybuzz::shape(
                &self.font.rb,
                &[
                    // If the font has support for tabular numbers, we want to
                    // use it, so we don't have to fix up much. Though we still
                    // attempt to do so anyway, as we can neither query if tnum
                    // support is even available, nor can we really trust it all
                    // too much.
                    Feature::new(Tag::from_bytes(b"tnum"), 1, ..),
                    // FIXME: We may or may not want to disable kerning and
                    // possibly ligatures too. If the font doesn't support tnum,
                    // then kerning for e.g. `.1` may cause inconsistent
                    // positioning.
                    // Feature::new(Tag::from_bytes(b"kern"), 0, ..),
                ],
                buffer,
            ),
            font: self,
        }
    }
}

#[derive(Copy, Clone)]
pub struct PositionedGlyph {
    id: u32,
    x: f32,
    y: f32,
}

pub struct Cursor {
    pub x: f32,
    pub y: f32,
}

impl Cursor {
    pub fn new([x, y]: Pos) -> Self {
        Self { x, y }
    }
}

pub struct Glyphs<'f> {
    font: ScaledFont<'f>,
    buffer: GlyphBuffer,
}

impl<'f> Glyphs<'f> {
    pub fn left_aligned<'a>(
        &'a self,
        cursor: &'a mut Cursor,
        max_x: f32,
    ) -> impl Iterator<Item = PositionedGlyph> + 'a {
        let scale = self.font.scale;

        let mut iter = Iterator::zip(
            self.buffer.glyph_infos().iter(),
            self.buffer.glyph_positions().iter(),
        );

        let ends_at_x = cursor.x + self.width();
        let ellipsis = if ends_at_x > max_x {
            let ellipsis = self.font.font.face.glyph_index('…').unwrap_or_default();
            let ellipsis_width = self
                .font
                .font
                .face
                .glyph_hor_advance(ellipsis)
                .unwrap_or_default() as i32;

            let overshoot_width = ((ends_at_x - max_x) / self.font.scale) as i32;

            let width_to_cut_off = ellipsis_width + overshoot_width;
            let mut actually_cut_off = 0;

            iter.by_ref().rev().find(|(_, p)| {
                actually_cut_off += p.x_advance;
                actually_cut_off >= width_to_cut_off
            });

            let x = ends_at_x - actually_cut_off as f32 * self.font.scale;

            Some(PositionedGlyph {
                id: ellipsis.0 as _,
                x,
                y: cursor.y,
            })
        } else {
            None
        };

        iter.map(move |(i, p)| {
            let g = PositionedGlyph {
                id: i.codepoint,
                x: cursor.x + p.x_offset as f32 * scale,
                y: cursor.y + p.y_offset as f32 * scale,
            };
            cursor.x += p.x_advance as f32 * scale;
            cursor.y += p.y_advance as f32 * scale;
            g
        })
        .chain(ellipsis)
    }

    pub fn right_aligned<'a>(
        &'a self,
        cursor: &'a mut Cursor,
    ) -> impl Iterator<Item = PositionedGlyph> + 'a {
        let scale = self.font.scale;

        Iterator::zip(
            self.buffer.glyph_infos().iter(),
            self.buffer.glyph_positions().iter(),
        )
        .rev()
        .map(move |(i, p)| {
            cursor.x -= p.x_advance as f32 * scale;
            cursor.y -= p.y_advance as f32 * scale;
            PositionedGlyph {
                id: i.codepoint,
                x: cursor.x + p.x_offset as f32 * scale,
                y: cursor.y + p.y_offset as f32 * scale,
            }
        })
    }

    pub fn centered<'a>(
        &'a self,
        cursor: &'a mut Cursor,
        min_x: f32,
        max_x: f32,
    ) -> impl Iterator<Item = PositionedGlyph> + 'a {
        let (mut adv_x, mut adv_y) = (0, 0);

        for p in self.buffer.glyph_positions() {
            adv_x += p.x_advance;
            adv_y += p.y_advance;
        }

        cursor.x -= adv_x as f32 * self.font.scale / 2.0;
        cursor.y -= adv_y as f32 * self.font.scale / 2.0;

        if cursor.x < min_x {
            cursor.x = min_x;
        }

        self.left_aligned(cursor, max_x)
    }

    // This is a right aligned layout where all the digits have the same width.
    pub fn tabular_numbers<'a>(
        &'a self,
        cursor: &'a mut Cursor,
    ) -> impl Iterator<Item = PositionedGlyph> + 'a {
        let mut digits = [0; 10];
        let mut digit_width = 0.0;
        for (digit, glyph) in digits.iter_mut().enumerate() {
            let GlyphId(the_glyph) = self
                .font
                .font
                .face
                .glyph_index(char::from(digit as u8 + b'0'))
                .unwrap_or_default();

            *glyph = the_glyph as _;

            let width = self.font.glyph_x_advance(the_glyph as _);
            if width > digit_width {
                digit_width = width;
            }
        }

        // FIXME: There's kerning between e.g. ".1" now, which is maybe not quite
        // what we want. We may need to either stabilize `:` and `.` now or turn
        // off kerning for tabular numbers.

        let scale = self.font.scale;

        Iterator::zip(
            self.buffer.glyph_infos().iter(),
            self.buffer.glyph_positions().iter(),
        )
        .rev()
        .map(move |(i, p)| {
            let x = if digits.contains(&i.codepoint) {
                cursor.x -= digit_width;
                let wider_by = digit_width - (p.x_advance as f32 * scale);
                cursor.x + p.x_offset as f32 * scale + 0.5 * wider_by
            } else {
                cursor.x -= p.x_advance as f32 * scale;
                cursor.x + p.x_offset as f32 * scale
            };

            cursor.y -= p.y_advance as f32 * scale;
            PositionedGlyph {
                id: i.codepoint,
                x,
                y: cursor.y + p.y_offset as f32 * scale,
            }
        })
    }

    pub fn width(&self) -> f32 {
        self.buffer
            .glyph_positions()
            .iter()
            .map(|p| p.x_advance)
            .sum::<i32>() as f32
            * self.font.scale
    }

    pub fn into_buffer(self) -> UnicodeBuffer {
        self.buffer.clear()
    }
}

pub fn render<B: Backend>(
    layout: impl IntoIterator<Item = PositionedGlyph>,
    [top, bottom]: [Color; 2],
    font: &ScaledFont<'_>,
    glyph_cache: &mut GlyphCache<B::Mesh>,
    transform: &Transform,
    backend: &mut B,
) {
    let top = decode_color(&top);
    let bottom = decode_color(&bottom);
    let colors = [top, top, bottom, bottom];

    for glyph in layout {
        let glyph_mesh = glyph_cache.lookup_or_insert(font.font, glyph.id, backend);

        let transform = transform
            .pre_translate([glyph.x, glyph.y].into())
            .pre_scale(font.scale, font.scale);

        backend.render_mesh(glyph_mesh, transform, colors, None);
    }
}
