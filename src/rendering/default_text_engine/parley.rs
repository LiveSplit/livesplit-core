use alloc::borrow::Cow;

use hashbrown::HashMap;
use parley::{
    FontContext, FontFamily, FontFamilyName, FontFeatures, FontStyle, FontWeight, FontWidth,
    Layout, LayoutContext, PositionedLayoutItem, StyleProperty,
    fontique::{Blob, Collection, CollectionOptions, SourceCache},
};
use skrifa::{
    FontRef, GlyphId, MetadataProvider,
    color::{Brush, ColorGlyphFormat, ColorPainter, CompositeMode, Transform},
    instance::{LocationRef, NormalizedCoord, Size},
    outline::{DrawSettings, OutlinePen},
};

use crate::{
    platform::{Arc, RwLock, prelude::*},
    settings::{self, FontStretch, FontStyle as SettingsFontStyle},
};

use super::super::{FontKind, PathBuilder, Rgba, SharedOwnership, TEXT_FONT, TIMER_FONT};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct GlyphKey {
    blob_id: u64,
    font_index: u32,
    glyph_id: u32,
    coords_hash: u64,
}

struct CachedGlyph<P> {
    scale: f32,
    paths: Vec<(Option<Rgba>, P)>,
}

/// The text engine allows you to create fonts and manage text labels. That way
/// the underlying renderer doesn't by itself need to be able to render text.
pub struct TextEngine<P> {
    font_context: FontContext,
    layout_context: LayoutContext<()>,
    glyph_cache: HashMap<GlyphKey, CachedGlyph<P>>,
}

impl<P: SharedOwnership> Default for TextEngine<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: SharedOwnership> TextEngine<P> {
    /// Creates a new path based text engine.
    pub fn new() -> Self {
        let mut collection = Collection::new(CollectionOptions {
            shared: false,
            system_fonts: cfg!(feature = "font-loading"),
        });
        collection.register_fonts(Blob::new(Arc::new(TIMER_FONT)), None);
        collection.register_fonts(Blob::new(Arc::new(TEXT_FONT)), None);

        Self {
            font_context: FontContext {
                collection,
                source_cache: SourceCache::default(),
            },
            layout_context: LayoutContext::new(),
            glyph_cache: HashMap::new(),
        }
    }

    /// Creates a new font. You can call this directly from a
    /// [`ResourceAllocator`](super::super::ResourceAllocator).
    pub fn create_font(&mut self, font: Option<&settings::Font>, kind: FontKind) -> Font {
        let fallback_family = match kind {
            FontKind::Timer => "LiveSplit Timer",
            _ => "Fira Sans",
        };

        let (family, width, style, weight) = if let Some(font) = font {
            (
                Some(font.family.clone()),
                to_font_width(font.stretch),
                to_font_style(font.style),
                FontWeight::new(font.weight.to_f32()),
            )
        } else {
            (
                None,
                FontWidth::NORMAL,
                FontStyle::Normal,
                if kind == FontKind::Text {
                    FontWeight::NORMAL
                } else {
                    FontWeight::BOLD
                },
            )
        };

        let mut font = Font {
            family,
            fallback_family,
            width,
            style,
            weight,
            monospaced: kind.is_monospaced(),
            monotonic: None,
            ellipsis_width: 0.0,
        };

        if font.monospaced {
            let mut digit_glyphs = [None; 10];
            let mut digit_width: f32 = 0.0;
            for (digit, slot) in digit_glyphs.iter_mut().enumerate() {
                let text = char::from(b'0' + digit as u8);
                let mut encoded = [0; 4];
                let layout = self.layout(text.encode_utf8(&mut encoded), &font);
                if let Some((key, width)) = first_glyph(&layout) {
                    *slot = Some(key);
                    digit_width = digit_width.max(width);
                }
            }
            font.monotonic = Some(MonotonicInfo {
                digit_glyphs,
                digit_width,
            });
        }

        font.ellipsis_width = self.layout("…", &font).full_width();
        font
    }

    fn layout(&mut self, text: &str, font: &Font) -> Layout<()> {
        let mut builder =
            self.layout_context
                .ranged_builder(&mut self.font_context, text, 1.0, false);
        let fallback = FontFamilyName::Named(Cow::Borrowed(font.fallback_family));
        let requested;
        let families = if let Some(family) = font.family.as_deref() {
            requested = [FontFamilyName::Named(Cow::Borrowed(family)), fallback];
            &requested[..]
        } else {
            requested = [fallback.clone(), fallback];
            &requested[..1]
        };
        builder.push_default(FontFamily::from(families));
        builder.push_default(StyleProperty::FontSize(1.0));
        builder.push_default(StyleProperty::FontWidth(font.width));
        builder.push_default(StyleProperty::FontStyle(font.style));
        builder.push_default(StyleProperty::FontWeight(font.weight));
        if font.monospaced {
            builder.push_default(FontFeatures::from(
                "\"kern\" off, \"liga\" off, \"clig\" off, \"tnum\" on",
            ));
        }
        let mut layout = builder.build(text);
        layout.break_all_lines(None);
        layout
    }

    /// Creates a new text label. You can call this directly from a
    /// [`ResourceAllocator`](super::super::ResourceAllocator).
    pub fn create_label<PB: PathBuilder<Path = P>>(
        &mut self,
        path_builder: impl FnMut() -> PB,
        text: &str,
        font: &Font,
        max_width: Option<f32>,
    ) -> Label<P> {
        let label = Arc::new(RwLock::new(LockedLabel {
            width: 0.0,
            width_without_max_width: 0.0,
            glyphs: Vec::new(),
        }));
        self.update_label(path_builder, &label, text, font, max_width);
        label
    }

    /// Updates a text label. You can call this directly from a
    /// [`ResourceAllocator`](super::super::ResourceAllocator).
    pub fn update_label<PB: PathBuilder<Path = P>>(
        &mut self,
        mut path_builder: impl FnMut() -> PB,
        label: &Label<P>,
        text: &str,
        font: &Font,
        max_width: Option<f32>,
    ) {
        let layout = self.layout(text, font);
        let width_without_max_width = layout.full_width();
        let mut glyphs = Vec::new();
        self.append_layout(&mut path_builder, &mut glyphs, &layout, font, 0.0);

        let width = if let Some(max_width) = max_width
            && width_without_max_width > max_width
        {
            let target = max_width - font.ellipsis_width;
            let mut ellipsis_x = 0.0;
            glyphs.retain(|glyph| {
                if glyph.x <= target {
                    ellipsis_x = glyph.x;
                    true
                } else {
                    false
                }
            });
            let ellipsis = self.layout("…", font);
            self.append_layout(&mut path_builder, &mut glyphs, &ellipsis, font, ellipsis_x);
            ellipsis_x + font.ellipsis_width
        } else {
            width_without_max_width
        };

        let mut label = label.write().unwrap();
        label.width = width;
        label.width_without_max_width = width_without_max_width;
        label.glyphs = glyphs;
    }

    fn append_layout<PB: PathBuilder<Path = P>>(
        &mut self,
        path_builder: &mut impl FnMut() -> PB,
        glyphs: &mut Vec<Glyph<P>>,
        layout: &Layout<()>,
        font: &Font,
        x_offset: f32,
    ) {
        for line in layout.lines() {
            let mut monotonic_offset = 0.0;
            for item in line.items() {
                let PositionedLayoutItem::GlyphRun(run) = item else {
                    continue;
                };
                let font_data = run.run().font();
                let normalized_coords = run.run().normalized_coords();
                let coords_hash = hash_coords(normalized_coords);
                for glyph in run.positioned_glyphs() {
                    let key = GlyphKey {
                        blob_id: font_data.data.id(),
                        font_index: font_data.index,
                        glyph_id: glyph.id,
                        coords_hash,
                    };
                    let mut centering_offset = 0.0;
                    if let Some(monotonic) = &font.monotonic
                        && monotonic.digit_glyphs.contains(&Some(key))
                    {
                        centering_offset = 0.5 * (monotonic.digit_width - glyph.advance);
                    }
                    let cached = cache_glyph(
                        &mut self.glyph_cache,
                        font_data,
                        normalized_coords,
                        key,
                        path_builder,
                    );
                    glyphs.extend(cached.paths.iter().map(|(color, path)| Glyph {
                        color: *color,
                        x: x_offset + glyph.x + monotonic_offset + centering_offset,
                        y: glyph.y - run.baseline(),
                        path: path.share(),
                        scale: cached.scale,
                    }));
                    if let Some(monotonic) = &font.monotonic
                        && monotonic.digit_glyphs.contains(&Some(key))
                    {
                        monotonic_offset += monotonic.digit_width - glyph.advance;
                    }
                }
            }
        }
    }
}

fn first_glyph(layout: &Layout<()>) -> Option<(GlyphKey, f32)> {
    for line in layout.lines() {
        for item in line.items() {
            if let PositionedLayoutItem::GlyphRun(run) = item
                && let Some(glyph) = run.glyphs().next()
            {
                let font = run.run().font();
                let normalized_coords = run.run().normalized_coords();
                return Some((
                    GlyphKey {
                        blob_id: font.data.id(),
                        font_index: font.index,
                        glyph_id: glyph.id,
                        coords_hash: hash_coords(normalized_coords),
                    },
                    glyph.advance,
                ));
            }
        }
    }
    None
}

fn hash_coords(coords: &[i16]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for coord in coords {
        for byte in coord.to_ne_bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    hash
}

fn cache_glyph<'a, P, PB: PathBuilder<Path = P>>(
    cache: &'a mut HashMap<GlyphKey, CachedGlyph<P>>,
    font_data: &parley::FontData,
    normalized_coords: &[i16],
    key: GlyphKey,
    path_builder: &mut impl FnMut() -> PB,
) -> &'a CachedGlyph<P> {
    cache.entry(key).or_insert_with(|| {
        let font = FontRef::from_index(font_data.data.as_ref(), font_data.index).unwrap();
        let normalized_coords: Vec<_> = normalized_coords
            .iter()
            .copied()
            .map(NormalizedCoord::from_bits)
            .collect();
        let location = LocationRef::new(&normalized_coords);
        let glyph_id = GlyphId::new(key.glyph_id);
        let mut paths = Vec::new();

        if let Some(color_glyph) = font
            .color_glyphs()
            .get_with_format(glyph_id, ColorGlyphFormat::ColrV0)
        {
            let palettes = font.color_palettes();
            let palette = palettes.get(0);
            let mut painter = ColrPainter {
                font: &font,
                location,
                palette,
                path_builder,
                paths: &mut paths,
                marker: core::marker::PhantomData,
            };
            let _ = color_glyph.paint(location, &mut painter);
        } else {
            push_glyph_path(&font, location, glyph_id, None, path_builder, &mut paths);
        }

        CachedGlyph {
            scale: f32::recip(
                font.metrics(Size::unscaled(), LocationRef::default())
                    .units_per_em as f32,
            ),
            paths,
        }
    })
}

fn push_glyph_path<P, PB: PathBuilder<Path = P>>(
    font: &FontRef<'_>,
    location: LocationRef<'_>,
    glyph_id: GlyphId,
    color: Option<Rgba>,
    path_builder: &mut impl FnMut() -> PB,
    paths: &mut Vec<(Option<Rgba>, P)>,
) {
    let mut builder = GlyphBuilder(path_builder());
    if let Some(glyph) = font.outline_glyphs().get(glyph_id) {
        let settings = DrawSettings::unhinted(Size::unscaled(), location);
        let _ = glyph.draw(settings, &mut builder);
    }
    paths.push((color, builder.0.finish()));
}

struct ColrPainter<'a, 'f, P, PB, F> {
    font: &'f FontRef<'f>,
    location: LocationRef<'a>,
    palette: Option<skrifa::color::ColorPalette<'f>>,
    path_builder: &'a mut F,
    paths: &'a mut Vec<(Option<Rgba>, P)>,
    marker: core::marker::PhantomData<fn() -> PB>,
}

impl<P, PB, F> ColorPainter for ColrPainter<'_, '_, P, PB, F>
where
    PB: PathBuilder<Path = P>,
    F: FnMut() -> PB,
{
    fn push_transform(&mut self, _: Transform) {}

    fn pop_transform(&mut self) {}

    fn push_clip_glyph(&mut self, _: GlyphId) {}

    fn push_clip_box(&mut self, _: skrifa::raw::types::BoundingBox<f32>) {}

    fn pop_clip(&mut self) {}

    fn fill(&mut self, _: Brush<'_>) {}

    fn fill_glyph(&mut self, glyph_id: GlyphId, _: Option<Transform>, brush: Brush<'_>) {
        let Brush::Solid {
            palette_index,
            alpha,
        } = brush
        else {
            return;
        };
        let color = if palette_index == 0xFFFF {
            None
        } else {
            self.palette
                .as_ref()
                .and_then(|palette| palette.colors().get(usize::from(palette_index)))
                .map(|color| {
                    [
                        f32::from(color.red) / 255.0,
                        f32::from(color.green) / 255.0,
                        f32::from(color.blue) / 255.0,
                        f32::from(color.alpha) / 255.0 * alpha,
                    ]
                })
        };
        push_glyph_path(
            self.font,
            self.location,
            glyph_id,
            color,
            self.path_builder,
            self.paths,
        );
    }

    fn push_layer(&mut self, _: CompositeMode) {}
}

const fn to_font_width(stretch: FontStretch) -> FontWidth {
    FontWidth::from_percentage(stretch.percentage())
}

const fn to_font_style(style: SettingsFontStyle) -> FontStyle {
    match style {
        SettingsFontStyle::Normal => FontStyle::Normal,
        SettingsFontStyle::Italic => FontStyle::Italic,
        SettingsFontStyle::Oblique => FontStyle::Oblique(None),
    }
}

struct MonotonicInfo {
    digit_glyphs: [Option<GlyphKey>; 10],
    digit_width: f32,
}

/// The font to use in the [`ResourceAllocator`](super::super::ResourceAllocator).
pub struct Font {
    family: Option<String>,
    fallback_family: &'static str,
    width: FontWidth,
    style: FontStyle,
    weight: FontWeight,
    monospaced: bool,
    monotonic: Option<MonotonicInfo>,
    ellipsis_width: f32,
}

struct GlyphBuilder<PB>(PB);

impl<PB: PathBuilder> OutlinePen for GlyphBuilder<PB> {
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

/// The label to use in the [`ResourceAllocator`](super::super::ResourceAllocator).
pub type Label<P> = Arc<RwLock<LockedLabel<P>>>;

/// You need to lock the [`Label`] to use it. This is the locked type that
/// provides all the methods you need to use the label.
pub struct LockedLabel<P> {
    width: f32,
    width_without_max_width: f32,
    glyphs: Vec<Glyph<P>>,
}

impl<P> LockedLabel<P> {
    /// The glyphs to render.
    pub fn glyphs(&self) -> &[Glyph<P>] {
        &self.glyphs
    }
}

impl<P> super::super::Label for Label<P> {
    fn width(&self, scale: f32) -> f32 {
        self.read().unwrap().width * scale
    }

    fn width_without_max_width(&self, scale: f32) -> f32 {
        self.read().unwrap().width_without_max_width * scale
    }
}

/// A glyph to render.
pub struct Glyph<P> {
    /// A glyph may provide its own color that overrides the fill shader of the
    /// [`Entity`](super::super::Entity).
    pub color: Option<Rgba>,
    /// The x-coordinate of the glyph.
    pub x: f32,
    /// The y-coordinate of the glyph.
    pub y: f32,
    /// The path to render.
    pub path: P,
    /// The scale of the glyph.
    pub scale: f32,
}
