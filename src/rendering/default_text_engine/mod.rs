//! An optional text engine is provided that allows you to create fonts and
//! manage text labels. That way the underlying renderer doesn't by itself need
//! to be able to render text, as all the text gets turned into paths.

use core::str;

use crate::{
    platform::{Arc, RwLock, prelude::*},
    settings::{FontStretch, FontStyle},
};

use cosmic_text::{
    Attrs, AttrsList, Family, FeatureTag, FontFeatures, FontSystem, ShapeLine, Shaping, Stretch,
    Style, Weight,
    fontdb::{Database, ID, Query, Source},
    rustybuzz::ttf_parser::{GlyphId, OutlineBuilder},
};
use hashbrown::HashMap;

use crate::settings;

use self::color_font::ColorTables;

use super::{FontKind, PathBuilder, Rgba, SharedOwnership, TEXT_FONT, TIMER_FONT};

mod color_font;

struct CachedGlyph<P> {
    scale: f32,
    paths: Vec<(Option<Rgba>, P)>,
}

/// The text engine allows you to create fonts and manage text labels. That way
/// the underlying renderer doesn't by itself need to be able to render text.
pub struct TextEngine<P> {
    font_system: FontSystem,
    glyph_cache: HashMap<(ID, u16), CachedGlyph<P>>,
}

impl<P: SharedOwnership> Default for TextEngine<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: SharedOwnership> TextEngine<P> {
    /// Creates a new path based text engine.
    pub fn new() -> Self {
        let mut db = Database::new();

        #[cfg(feature = "font-loading")]
        db.load_system_fonts();

        db.load_font_source(Source::Binary(Arc::<&[u8]>::from(TIMER_FONT)));
        db.load_font_source(Source::Binary(Arc::<&[u8]>::from(TEXT_FONT)));

        Self {
            // FIXME: Whenever we introduce localization, we need to make sure
            // to use the correct locale here.
            font_system: FontSystem::new_with_locale_and_db(String::from("en-US"), db),
            glyph_cache: HashMap::new(),
        }
    }

    /// Creates a new font. You can call this directly from a
    /// [`ResourceAllocator`](super::ResourceAllocator).
    pub fn create_font(&mut self, font: Option<&settings::Font>, kind: FontKind) -> Font {
        let fallback_family = &match kind {
            FontKind::Timer => "LiveSplit Timer",
            _ => "Fira Sans",
        };

        let (mut stretch, mut style, mut weight, mut family, single, multiple, families);

        if let Some(font) = font {
            stretch = match font.stretch {
                FontStretch::UltraCondensed => Stretch::UltraCondensed,
                FontStretch::ExtraCondensed => Stretch::ExtraCondensed,
                FontStretch::Condensed => Stretch::Condensed,
                FontStretch::SemiCondensed => Stretch::SemiCondensed,
                FontStretch::Normal => Stretch::Normal,
                FontStretch::SemiExpanded => Stretch::SemiExpanded,
                FontStretch::Expanded => Stretch::Expanded,
                FontStretch::ExtraExpanded => Stretch::ExtraExpanded,
                FontStretch::UltraExpanded => Stretch::UltraExpanded,
            };
            style = match font.style {
                FontStyle::Normal => Style::Normal,
                FontStyle::Italic => Style::Italic,
                FontStyle::Oblique => Style::Oblique,
            };
            weight = Weight(font.weight.to_u16());
            family = font.family.as_str();
            multiple = [Family::Name(&font.family), Family::Name(fallback_family)];
            families = &multiple[..];
        } else {
            stretch = Stretch::Normal;
            style = Style::Normal;
            weight = match kind {
                FontKind::Text => Weight::NORMAL,
                _ => Weight::BOLD,
            };
            family = fallback_family;
            single = [Family::Name(fallback_family)];
            families = &single[..];
        }

        // FIXME: We mostly do a manual query right now because cosmic-text
        // defaults to the emoji font if a single property doesn't match
        // exactly. So we do a more relaxed query by ourselves and then reassign
        // the properties to the exact properties that we find in the database.
        // https://github.com/pop-os/cosmic-text/issues/58
        if let Some(found_id) = self.font_system.db().query(&Query {
            families,
            weight,
            stretch,
            style,
        }) && let Some(info) = self.font_system.db().face(found_id)
        {
            stretch = info.stretch;
            style = info.style;
            weight = info.weight;
            if let [(info_family, _), ..] = &*info.families {
                family = info_family;
            }
        }

        let mut attrs = Attrs::new()
            .family(Family::Name(family))
            .stretch(stretch)
            .style(style)
            .weight(weight);

        if kind.is_monospaced() {
            let mut features = FontFeatures::new();
            features.disable(FeatureTag::KERNING);
            features.disable(FeatureTag::STANDARD_LIGATURES);
            features.disable(FeatureTag::CONTEXTUAL_LIGATURES);
            features.enable(FeatureTag::new(b"tnum"));
            attrs = attrs.font_features(features);
        }

        let attrs_list = AttrsList::new(&attrs);

        let monotonic = kind.is_monospaced().then(|| {
            let mut digit_glyphs = [(ID::dummy(), 0); 10];
            let mut digit_width = 0.0;
            for (digit, glyph) in digit_glyphs.iter_mut().enumerate() {
                // SAFETY: We iterate through the 10 ASCII digits. They are
                // all valid UTF-8.
                if let Some((font_id, glyph_id, width)) = unsafe {
                    self.glyph_width(str::from_utf8_unchecked(&[digit as u8 + b'0']), &attrs_list)
                } {
                    *glyph = (font_id, glyph_id);
                    if width > digit_width {
                        digit_width = width;
                    }
                }
            }
            MonotonicInfo {
                digit_glyphs,
                digit_width,
            }
        });

        let (ellipsis_font_id, ellipsis_glyph_id, ellipsis_width) = self
            .glyph_width("…", &attrs_list)
            .unwrap_or_else(|| (ID::dummy(), 0, 0.0));

        Font {
            attrs_list,
            monotonic,
            ellipsis_font_id,
            ellipsis_glyph_id,
            ellipsis_width,
        }
    }

    fn glyph_width(&mut self, glyph_text: &str, attrs_list: &AttrsList) -> Option<(ID, u16, f32)> {
        let shape_line = ShapeLine::new(
            &mut self.font_system,
            glyph_text,
            attrs_list,
            Shaping::Advanced,
            4,
        );

        if let [span] = &*shape_line.spans
            && let [word] = &*span.words
            && let [glyph] = &*word.glyphs
        {
            Some((glyph.font_id, glyph.glyph_id, glyph.x_advance))
        } else {
            None
        }
    }

    /// Creates a new text label. You can call this directly from a
    /// [`ResourceAllocator`](super::ResourceAllocator).
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
    /// [`ResourceAllocator`](super::ResourceAllocator).
    pub fn update_label<PB: PathBuilder<Path = P>>(
        &mut self,
        mut path_builder: impl FnMut() -> PB,
        label: &Label<P>,
        text: &str,
        font: &Font,
        max_width: Option<f32>,
    ) {
        let mut label = label.write().unwrap();

        label.glyphs.clear();

        // FIXME: Look into shape plans in 0.11
        let shape_line = ShapeLine::new(
            &mut self.font_system,
            text,
            &font.attrs_list,
            Shaping::Advanced,
            4,
        );
        let [mut x, mut y] = [0.0; 2];

        if let Some(monotonic) = &font.monotonic {
            for span in &shape_line.spans {
                for word in &span.words {
                    if !word.blank {
                        let mut glyphs = word.glyphs.iter();
                        while let Some(glyph) = if shape_line.rtl {
                            glyphs.next_back()
                        } else {
                            glyphs.next()
                        } {
                            let cached_glyph = cache_glyph(
                                &mut self.glyph_cache,
                                &mut self.font_system,
                                glyph.font_id,
                                glyph.glyph_id,
                                &mut path_builder,
                            );

                            let (x_advance, x_offset) = if monotonic
                                .digit_glyphs
                                .contains(&(glyph.font_id, glyph.glyph_id))
                            {
                                (
                                    monotonic.digit_width,
                                    0.5 * (monotonic.digit_width - glyph.x_advance)
                                        + glyph.x_offset,
                                )
                            } else {
                                (glyph.x_advance, glyph.x_offset)
                            };

                            label
                                .glyphs
                                .extend(cached_glyph.paths.iter().map(|(color, path)| Glyph {
                                    color: *color,
                                    x: x + x_offset,
                                    y: y - glyph.y_offset,
                                    path: path.share(),
                                    scale: cached_glyph.scale,
                                }));

                            x += x_advance;
                            y -= glyph.y_advance;
                        }
                    } else {
                        x += word.width(1.0);
                    }
                }
            }
        } else {
            for span in &shape_line.spans {
                for word in &span.words {
                    if !word.blank {
                        let [mut glyph_x, mut glyph_y] = [x, y];
                        let mut glyphs = word.glyphs.iter();
                        while let Some(glyph) = if shape_line.rtl {
                            glyphs.next_back()
                        } else {
                            glyphs.next()
                        } {
                            let cached_glyph = cache_glyph(
                                &mut self.glyph_cache,
                                &mut self.font_system,
                                glyph.font_id,
                                glyph.glyph_id,
                                &mut path_builder,
                            );

                            label
                                .glyphs
                                .extend(cached_glyph.paths.iter().map(|(color, path)| Glyph {
                                    color: *color,
                                    x: glyph_x + glyph.x_offset,
                                    y: glyph_y - glyph.y_offset,
                                    path: path.share(),
                                    scale: cached_glyph.scale,
                                }));

                            glyph_x += glyph.x_advance;
                            glyph_y -= glyph.y_advance;
                        }
                    }
                    x += word.width(1.0);
                }
            }
        }

        label.width_without_max_width = x;

        if let Some(max_width) = max_width
            && x > max_width
        {
            let x_to_look_for = max_width - font.ellipsis_width;

            let last_index = label
                .glyphs
                .iter()
                .enumerate()
                .rfind(|(_, g)| {
                    x = g.x;
                    y = g.y;
                    g.x <= x_to_look_for
                })
                .map(|(i, _)| i)
                .unwrap_or_default();
            label.glyphs.drain(last_index..);

            // FIXME: Test RTL text.

            let cached_glyph = cache_glyph(
                &mut self.glyph_cache,
                &mut self.font_system,
                font.ellipsis_font_id,
                font.ellipsis_glyph_id,
                &mut path_builder,
            );

            label
                .glyphs
                .extend(cached_glyph.paths.iter().map(|(color, path)| Glyph {
                    color: *color,
                    x,
                    y,
                    path: path.share(),
                    scale: cached_glyph.scale,
                }));
            x += font.ellipsis_width;
        }

        label.width = x;
    }
}

fn cache_glyph<'gc, P, PB: PathBuilder<Path = P>>(
    glyph_cache: &'gc mut HashMap<(ID, u16), CachedGlyph<P>>,
    font_system: &mut FontSystem,
    font_id: ID,
    glyph_id: u16,
    path_builder: &mut impl FnMut() -> PB,
) -> &'gc mut CachedGlyph<P> {
    glyph_cache.entry((font_id, glyph_id)).or_insert_with(|| {
        let font = font_system.get_font(font_id).unwrap();
        let font = font.rustybuzz();
        let mut paths = Vec::new();
        let color_tables = ColorTables::new(font);
        let glyph = GlyphId(glyph_id);
        color_font::iter_colored_glyphs(&color_tables, 0, glyph, |glyph, color| {
            let mut builder = GlyphBuilder(path_builder());
            font.outline_glyph(glyph, &mut builder);
            let path = builder.0.finish();
            paths.push((color.map(|c| c.to_array()), path));
        });
        let scale = f32::recip(font.units_per_em() as _);
        CachedGlyph { scale, paths }
    })
}

struct MonotonicInfo {
    digit_glyphs: [(ID, u16); 10],
    digit_width: f32,
}

/// The font to use in the [`ResourceAllocator`](super::ResourceAllocator).
pub struct Font {
    attrs_list: AttrsList,
    monotonic: Option<MonotonicInfo>,
    ellipsis_font_id: ID,
    ellipsis_glyph_id: u16,
    ellipsis_width: f32,
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
    glyphs: Vec<Glyph<P>>,
}

impl<P> LockedLabel<P> {
    /// The glyphs to render.
    pub fn glyphs(&self) -> &[Glyph<P>] {
        &self.glyphs
    }
}

impl<P> super::Label for Label<P> {
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
    /// [`Entity`](super::Entity).
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
