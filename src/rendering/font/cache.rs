use super::{Font, GlyphCache, SharedOwnership, TEXT_FONT, TIMER_FONT};
use crate::settings::{FontStretch, FontStyle, FontWeight};

pub struct CachedFont<P> {
    #[cfg(feature = "font-loading")]
    setting: Option<crate::settings::Font>,
    pub font: Font<'static>,
    pub glyph_cache: GlyphCache<P>,
}

impl<P: SharedOwnership> CachedFont<P> {
    fn new(font: Font<'static>) -> Self {
        Self {
            #[cfg(feature = "font-loading")]
            setting: None,
            font,
            glyph_cache: GlyphCache::new(),
        }
    }

    #[cfg(feature = "font-loading")]
    fn maybe_reload(
        &mut self,
        font_to_use: &Option<crate::settings::Font>,
        default_font: impl FnOnce() -> Font<'static>,
    ) {
        if &self.setting != font_to_use {
            self.font = font_to_use
                .as_ref()
                .and_then(Font::load)
                .unwrap_or_else(default_font);
            self.glyph_cache.clear();
            self.setting.clone_from(font_to_use);
        }
    }
}

pub struct FontCache<P> {
    pub timer: CachedFont<P>,
    pub times: CachedFont<P>,
    pub text: CachedFont<P>,
}

impl<P: SharedOwnership> FontCache<P> {
    pub fn new() -> Option<Self> {
        Some(Self {
            timer: CachedFont::new(Font::from_slice(
                TIMER_FONT,
                0,
                FontStyle::Normal,
                FontWeight::Bold,
                FontStretch::Normal,
            )?),
            times: CachedFont::new(Font::from_slice(
                TEXT_FONT,
                0,
                FontStyle::Normal,
                FontWeight::Bold,
                FontStretch::Normal,
            )?),
            text: CachedFont::new(Font::from_slice(
                TEXT_FONT,
                0,
                FontStyle::Normal,
                FontWeight::Normal,
                FontStretch::Normal,
            )?),
        })
    }

    #[cfg(feature = "font-loading")]
    pub fn maybe_reload(&mut self, state: &crate::layout::LayoutState) {
        self.timer.maybe_reload(&state.timer_font, || {
            Font::from_slice(
                TIMER_FONT,
                0,
                FontStyle::Normal,
                FontWeight::Bold,
                FontStretch::Normal,
            )
            .unwrap()
        });

        self.times.maybe_reload(&state.times_font, || {
            Font::from_slice(
                TEXT_FONT,
                0,
                FontStyle::Normal,
                FontWeight::Bold,
                FontStretch::Normal,
            )
            .unwrap()
        });

        self.text.maybe_reload(&state.text_font, || {
            Font::from_slice(
                TEXT_FONT,
                0,
                FontStyle::Normal,
                FontWeight::Normal,
                FontStretch::Normal,
            )
            .unwrap()
        });
    }
}
