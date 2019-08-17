use super::glyph_cache::GlyphCache;
use super::{decode_color, Backend, Pos, Transform};
use crate::settings::Color;
use rusttype::{point, Codepoint, Font, GlyphId, PositionedGlyph, Scale};
use smallvec::SmallVec;

#[derive(Copy, Clone)]
pub struct ScaledFont<'f, 'fd> {
    font: &'f Font<'fd>,
    scale: Scale,
}

pub fn scaled<'f, 'fd>(font: &'f Font<'fd>, scale: f32) -> ScaledFont<'f, 'fd> {
    ScaledFont {
        font,
        scale: Scale::uniform(scale),
    }
}

pub fn default_layout<'i: 'fd, 'fd>(
    font: ScaledFont<'i, 'fd>,
    text: &'i str,
    [x, y]: Pos,
) -> impl Iterator<Item = PositionedGlyph<'fd>> + Clone + 'i {
    font.font.layout(text, font.scale, point(x, y))
}

pub fn align_right_and_measure<'fd>(
    layout: impl IntoIterator<Item = PositionedGlyph<'fd>> + Clone,
) -> (impl Iterator<Item = PositionedGlyph<'fd>>, f32) {
    let width = measure(layout.clone());
    let layout = layout.into_iter().map(move |g| {
        let pos = g.position();
        g.into_unpositioned()
            .positioned(point(pos.x - width, pos.y))
    });
    (layout, width)
}

pub fn dynamic_align<'fd>(
    layout: impl IntoIterator<Item = PositionedGlyph<'fd>> + Clone,
    align: f32,
    min_x: f32,
) -> impl Iterator<Item = PositionedGlyph<'fd>> {
    let mut offset = align * measure(layout.clone());
    let mut is_first = true;
    layout.into_iter().map(move |g| {
        let pos = g.position();
        if is_first {
            if pos.x - offset < min_x {
                offset = pos.x - min_x;
            }
            is_first = false;
        }
        g.into_unpositioned()
            .positioned(point(pos.x - offset, pos.y))
    })
}

pub fn centered<'fd>(
    layout: impl IntoIterator<Item = PositionedGlyph<'fd>> + Clone,
    min_x: f32,
) -> impl Iterator<Item = PositionedGlyph<'fd>> {
    let mut offset = measure(layout.clone()) / 2.0;
    let mut is_first = true;
    layout.into_iter().map(move |g| {
        let pos = g.position();
        if is_first {
            if pos.x - offset < min_x {
                offset = pos.x - min_x;
            }
            is_first = false;
        }
        g.into_unpositioned()
            .positioned(point(pos.x - offset, pos.y))
    })
}

pub fn layout_numbers<'i: 'fd, 'fd>(
    font: ScaledFont<'i, 'fd>,
    text: &'i str,
    [mut x, y]: Pos,
) -> impl Iterator<Item = PositionedGlyph<'fd>> + Clone + 'i {
    let scale = font.scale;

    let mut digits = [GlyphId(0); 10];
    let mut digit_width = 0.0;
    for (digit, glyph) in digits.iter_mut().enumerate() {
        let the_glyph = font.font.glyph(Codepoint(digit as u32 + u32::from(b'0')));

        *glyph = the_glyph.id();

        let width = the_glyph.scaled(scale).h_metrics().advance_width;
        if width > digit_width {
            digit_width = width;
        }
    }

    let mut last = None;
    font.font.glyphs_for(text.chars().rev()).map(move |glyph| {
        let g = glyph.scaled(scale);
        let advance_width = g.h_metrics().advance_width;
        let post_advance = if digits.iter().any(|glyph| *glyph == g.id()) {
            let left_right = (digit_width - advance_width) / 2.0;
            x -= left_right + advance_width;
            left_right
        } else {
            if let Some(last) = last {
                x -= font.font.pair_kerning(scale, g.id(), last);
            }
            x -= advance_width;
            0.0
        };
        let glyph = g.positioned(point(x, y));
        last = Some(glyph.id());
        x -= post_advance;

        glyph
    })
}

pub fn ellipsis<'fd>(
    layout: impl IntoIterator<Item = PositionedGlyph<'fd>>,
    mut max_x: f32,
    font: ScaledFont<'_, 'fd>,
) -> impl Iterator<Item = PositionedGlyph<'fd>> {
    let ellipsis = font.font.glyph(Codepoint('…' as u32)).scaled(font.scale);
    let ellipsis_width = ellipsis.h_metrics().advance_width;

    let mut glyphs = layout.into_iter().collect::<SmallVec<[_; 32]>>();

    let mut positioned_ellipsis = None;
    while let Some(glyph) = glyphs.last() {
        if glyph.position().x + glyph.unpositioned().h_metrics().advance_width > max_x {
            if positioned_ellipsis.is_none() {
                max_x -= ellipsis_width;
            }
            positioned_ellipsis = Some(ellipsis.clone().positioned(glyph.position()));
            glyphs.pop();
        } else {
            break;
        }
    }
    if let Some(ellipsis) = positioned_ellipsis {
        glyphs.push(ellipsis);
    }

    glyphs.into_iter()
}

pub fn measure<'fd>(layout: impl IntoIterator<Item = PositionedGlyph<'fd>>) -> f32 {
    let mut first = None;
    layout
        .into_iter()
        .inspect(|g| {
            first.get_or_insert_with(|| g.position().x);
        })
        .last()
        .map_or(0.0, |g| {
            g.position().x + g.unpositioned().h_metrics().advance_width - first.unwrap()
        })
}

pub fn measure_default_layout(font: ScaledFont<'_, '_>, text: &str) -> f32 {
    measure(default_layout(font, text, [0.0; 2]))
}

pub fn render<'fd, B: Backend>(
    layout: impl IntoIterator<Item = PositionedGlyph<'fd>>,
    [top, bottom]: [Color; 2],
    font: &Font<'_>,
    glyph_cache: &mut GlyphCache<B::Mesh>,
    transform: &Transform,
    backend: &mut B,
) -> Option<PositionedGlyph<'fd>> {
    let top = decode_color(&top);
    let bottom = decode_color(&bottom);
    let colors = [top, top, bottom, bottom];

    let mut last_glyph = None;
    for glyph in layout {
        let glyph_mesh = glyph_cache.lookup_or_insert(font, glyph.id(), backend);
        let pos = glyph.position();
        let scale = glyph.scale();
        last_glyph = Some(glyph);

        let transform = transform
            .pre_translate([pos.x, pos.y].into())
            .pre_scale(scale.x, scale.y);

        backend.render_mesh(glyph_mesh, transform, colors, None);
    }

    last_glyph
}
