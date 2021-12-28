//! An optional path based text engine is provided that allows you to create
//! fonts and manage text labels that are based on paths. That way the
//! underlying renderer doesn't by itself need to be able to render text, as all
//! the text gets turned into paths.

use std::{fs, sync::Arc};

#[cfg(feature = "font-loading")]
use font_kit::{
    family_name::FamilyName,
    handle::Handle,
    properties::{Properties, Stretch, Style, Weight},
    source::SystemSource,
};
use hashbrown::HashMap;
use parking_lot::{const_rwlock, RwLock};
use rustybuzz::{Face, Feature, Tag, UnicodeBuffer, Variation};
use ttf_parser::{GlyphId, OutlineBuilder};

use super::{
    font::{TEXT_FONT, TIMER_FONT},
    FontKind, PathBuilder, Rgba, SharedOwnership,
};
use crate::settings::{self, FontStretch, FontStyle, FontWeight};

use self::color_font::{iter_colored_glyphs, ColorTables};

mod color_font;

/// The path based text engine allows you to create fonts and manage text labels
/// that are based on paths. That way the underlying renderer doesn't by itself
/// need to be able to render text, as all the text gets turned into paths.
pub struct TextEngine {
    #[cfg(feature = "font-loading")]
    source: SystemSource,
    buffer: Option<UnicodeBuffer>,
}

impl Default for TextEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TextEngine {
    /// Creates a new path based text engine.
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "font-loading")]
            source: SystemSource::new(),
            buffer: None,
        }
    }

    /// Creates a new font. You can call this directly from a
    /// [`ResourceAllocator`](super::ResourceAllocator).
    pub fn create_font<P>(&mut self, font: Option<&settings::Font>, kind: FontKind) -> Font<P> {
        #[cfg(feature = "font-loading")]
        if let Some(font) = font {
            if let Some(font) = Font::try_load_font(&mut self.source, font, kind) {
                return font;
            }
        }
        let (font_data, style, weight, stretch) = match kind {
            FontKind::Timer => (
                TIMER_FONT,
                FontStyle::Normal,
                FontWeight::Bold,
                FontStretch::Normal,
            ),
            FontKind::Times => (
                TEXT_FONT,
                FontStyle::Normal,
                FontWeight::Bold,
                FontStretch::Normal,
            ),
            FontKind::Text => (
                TEXT_FONT,
                FontStyle::Normal,
                FontWeight::Normal,
                FontStretch::Normal,
            ),
        };
        Font::from_slice(font_data, 0, style, weight, stretch, kind).unwrap()
    }

    /// Creates a new text label. You can call this directly from a
    /// [`ResourceAllocator`](super::ResourceAllocator).
    pub fn create_label<PB: PathBuilder>(
        &mut self,
        path_builder: impl FnMut() -> PB,
        text: &str,
        font: &mut Font<PB::Path>,
        max_width: Option<f32>,
    ) -> Label<PB::Path> {
        let mut label = Arc::new(const_rwlock(LockedLabel {
            width: 0.0,
            width_without_max_width: 0.0,
            scale: 0.0,
            glyphs: Vec::new(),
        }));

        self.update_label(path_builder, &mut label, text, font, max_width);

        label
    }

    /// Updates a text label. You can call this directly from a
    /// [`ResourceAllocator`](super::ResourceAllocator).
    pub fn update_label<PB: PathBuilder>(
        &mut self,
        mut path_builder: impl FnMut() -> PB,
        label: &mut Label<PB::Path>,
        text: &str,
        font: &mut Font<PB::Path>,
        max_width: Option<f32>,
    ) {
        let mut label = label.write();
        let label = &mut *label;

        let mut buffer = self.buffer.take().unwrap_or_else(UnicodeBuffer::new);
        buffer.push_str(text);

        let features = font
            .monotonic
            .as_ref()
            .map(|m| &m.features[..])
            .unwrap_or_default();

        let buffer = rustybuzz::shape(&font.face, features, buffer);

        let iter = Iterator::zip(buffer.glyph_infos().iter(), buffer.glyph_positions().iter());

        let (mut x, mut y) = (0.0, 0.0);

        label.glyphs.clear();

        if let Some(monotonic) = &font.monotonic {
            iter.for_each(|(info, pos)| {
                let glyph = GlyphId(info.glyph_id as _);
                let layer_glyphs = font.glyph_cache.entry(glyph).or_insert_with(|| {
                    let mut glyphs = Vec::new();
                    iter_colored_glyphs(&font.color_tables, 0, glyph, |glyph, color| {
                        let mut builder = GlyphBuilder(path_builder());
                        font.face.outline_glyph(glyph, &mut builder);
                        let path = builder.0.finish();
                        glyphs.push((color.map(|c| c.to_array()), path));
                    });
                    glyphs
                });
                let (x_advance, x_offset) =
                    if monotonic.digit_glyphs.contains(&(info.glyph_id as u16)) {
                        (
                            monotonic.digit_width,
                            0.5 * (monotonic.digit_width - pos.x_advance as f32)
                                + pos.x_offset as f32,
                        )
                    } else {
                        (pos.x_advance as f32, pos.x_offset as f32)
                    };
                let (glyph_x, glyph_y) = (x + x_offset, y + pos.y_offset as f32);
                x += x_advance;
                y += pos.y_advance as f32;
                label
                    .glyphs
                    .extend(layer_glyphs.iter().map(|(color, path)| Glyph {
                        color: *color,
                        x: glyph_x,
                        y: glyph_y,
                        path: path.share(),
                    }));
            });
        } else {
            iter.for_each(|(info, pos)| {
                let glyph = GlyphId(info.glyph_id as _);
                let layer_glyphs = font.glyph_cache.entry(glyph).or_insert_with(|| {
                    let mut glyphs = Vec::new();
                    iter_colored_glyphs(&font.color_tables, 0, glyph, |glyph, color| {
                        let mut builder = GlyphBuilder(path_builder());
                        font.face.outline_glyph(glyph, &mut builder);
                        let path = builder.0.finish();
                        glyphs.push((color.map(|c| c.to_array()), path));
                    });
                    glyphs
                });
                let (glyph_x, glyph_y) = (x + pos.x_offset as f32, y + pos.y_offset as f32);
                x += pos.x_advance as f32;
                y += pos.y_advance as f32;
                label
                    .glyphs
                    .extend(layer_glyphs.iter().map(|(color, path)| Glyph {
                        color: *color,
                        x: glyph_x,
                        y: glyph_y,
                        path: path.share(),
                    }));
            });
        };

        label.width_without_max_width = x * font.scale_factor;

        if let Some(max_width) = max_width {
            let max_width = max_width / font.scale_factor;
            if x > max_width {
                let ellipsis = font.face.glyph_index('…').unwrap_or_default();
                let ellipsis_width =
                    font.face.glyph_hor_advance(ellipsis).unwrap_or_default() as f32;

                let x_to_look_for = max_width - ellipsis_width;

                let last_index = label
                    .glyphs
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, g)| {
                        x = g.x;
                        y = g.y;
                        g.x <= x_to_look_for
                    })
                    .map(|(i, _)| i)
                    .unwrap_or_default();
                label.glyphs.drain(last_index..);
                let layer_glyphs = font.glyph_cache.entry(ellipsis).or_insert_with(|| {
                    let mut glyphs = Vec::new();
                    iter_colored_glyphs(&font.color_tables, 0, ellipsis, |glyph, color| {
                        let mut builder = GlyphBuilder(path_builder());
                        font.face.outline_glyph(glyph, &mut builder);
                        let path = builder.0.finish();
                        glyphs.push((color.map(|c| c.to_array()), path));
                    });
                    glyphs
                });
                label
                    .glyphs
                    .extend(layer_glyphs.iter().map(|(color, path)| Glyph {
                        color: *color,
                        x,
                        y,
                        path: path.share(),
                    }));
                x += ellipsis_width;
            }
        }

        self.buffer = Some(buffer.clear());

        label.width = x * font.scale_factor;
        label.scale = font.scale_factor;
    }
}

struct MonotonicInfo {
    digit_glyphs: [u16; 10],
    digit_width: f32,
    features: [Feature; 1],
}

/// The font to use in the [`ResourceAllocator`](super::ResourceAllocator).
pub struct Font<P> {
    face: Face<'static>,
    color_tables: Option<ColorTables<'static>>,
    scale_factor: f32,
    monotonic: Option<MonotonicInfo>,
    glyph_cache: HashMap<GlyphId, Vec<(Option<Rgba>, P)>>,
    #[cfg(feature = "font-loading")]
    _buf: Option<Box<[u8]>>,
}

impl<P> Font<P> {
    #[cfg(feature = "font-loading")]
    fn try_load_font(
        source: &mut SystemSource,
        font: &settings::Font,
        kind: FontKind,
    ) -> Option<Self> {
        let handle = source
            .select_best_match(
                &[FamilyName::Title(font.family.clone())],
                &Properties {
                    style: match font.style {
                        FontStyle::Normal => Style::Normal,
                        FontStyle::Italic => Style::Italic,
                    },
                    weight: Weight(font.weight.value()),
                    stretch: Stretch(font.stretch.factor()),
                },
            )
            .ok()?;

        let (buf, font_index) = match handle {
            Handle::Path { path, font_index } => (fs::read(path).ok()?, font_index),
            Handle::Memory { bytes, font_index } => (
                Arc::try_unwrap(bytes).unwrap_or_else(|bytes| (*bytes).clone()),
                font_index,
            ),
        };
        let buf = buf.into_boxed_slice();

        // Safety: We store our own buffer. If we never modify it and drop it
        // last, this is fine. It also needs to be heap allocated, so it's a
        // stable pointer. This is guaranteed by the boxed slice.
        unsafe {
            let slice: *const [u8] = &*buf;
            let mut font = Font::from_slice(
                &*slice,
                font_index,
                font.style,
                font.weight,
                font.stretch,
                kind,
            )?;
            font._buf = Some(buf);
            Some(font)
        }
    }

    fn from_slice(
        data: &'static [u8],
        index: u32,
        style: FontStyle,
        weight: FontWeight,
        stretch: FontStretch,
        kind: FontKind,
    ) -> Option<Self> {
        let mut face = Face::from_slice(data, index)?;

        let italic = style.value_for_italic();
        let weight = weight.value();
        let stretch = stretch.percentage();

        face.set_variations(&[
            Variation {
                tag: Tag::from_bytes(b"ital"),
                value: italic,
            },
            Variation {
                tag: Tag::from_bytes(b"wght"),
                value: weight,
            },
            Variation {
                tag: Tag::from_bytes(b"wdth"),
                value: stretch,
            },
        ]);

        let monotonic = kind.is_monospaced().then(|| {
            let mut digit_glyphs = [0; 10];
            let mut digit_width = 0;
            for (digit, glyph) in digit_glyphs.iter_mut().enumerate() {
                *glyph = face
                    .glyph_index(char::from(digit as u8 + b'0'))
                    .unwrap_or_default()
                    .0;

                let width = face.glyph_hor_advance(GlyphId(*glyph)).unwrap_or_default();
                if width > digit_width {
                    digit_width = width;
                }
            }
            MonotonicInfo {
                digit_glyphs,
                digit_width: digit_width as f32,
                features: [
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
            }
        });

        Some(Self {
            scale_factor: 1.0 / face.height() as f32,
            color_tables: ColorTables::new(&face),
            face,
            #[cfg(feature = "font-loading")]
            _buf: None,
            monotonic,
            glyph_cache: HashMap::new(),
        })
    }
}

struct GlyphBuilder<PB>(PB);

impl<PB: PathBuilder> OutlineBuilder for GlyphBuilder<PB> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.0.move_to(x, -y);
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.0.line_to(x, -y);
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.0.quad_to(x1, -y1, x, -y);
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.0.curve_to(x1, -y1, x2, -y2, x, -y);
    }
    fn close(&mut self) {
        self.0.close();
    }
}

/// The label to use in the [`ResourceAllocator`](super::ResourceAllocator).
pub type Label<P> = Arc<RwLock<LockedLabel<P>>>;

/// You need to lock the [`Label`] to use it. This is the locked type that
/// provides all the methods you need to use the label.
pub struct LockedLabel<P> {
    width: f32,
    width_without_max_width: f32,
    scale: f32,
    glyphs: Vec<Glyph<P>>,
}

impl<P> LockedLabel<P> {
    /// Apply this scale to the transform of the [`Entity`](super::Entity) with
    /// [`Transform::pre_scale`](super::Transform::pre_scale).
    pub const fn scale(&self) -> f32 {
        self.scale
    }

    /// The glyphs to render.
    pub fn glyphs(&self) -> &[Glyph<P>] {
        &self.glyphs
    }
}

impl<P> super::Label for Label<P> {
    fn width(&self, scale: f32) -> f32 {
        self.read().width * scale
    }

    fn width_without_max_width(&self, scale: f32) -> f32 {
        self.read().width_without_max_width * scale
    }
}

/// A glyph to render.
pub struct Glyph<P> {
    /// A glyph may provide its own color that overrides the fill shader of the
    /// [`Entity`](super::Entity).
    pub color: Option<Rgba>,
    /// The x-coordinate of the glyph.
    pub x: f32,
    /// The y-coordinate of the glyph.
    pub y: f32,
    /// The path to render.
    pub path: P,
}
