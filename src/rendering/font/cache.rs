use super::{
    super::{FontKind, Handle},
    ResourceAllocator,
};
use crate::{layout::LayoutState, settings::Font};

pub struct CachedFont<F> {
    setting: Option<Font>,
    pub font: Handle<F>,
}

impl<F> CachedFont<F> {
    const fn new(font: Handle<F>) -> Self {
        Self {
            setting: None,
            font,
        }
    }

    fn maybe_reload(
        &mut self,
        allocator: &mut impl ResourceAllocator<Font = Handle<F>>,
        font_to_use: &Option<Font>,
        font_kind: FontKind,
    ) {
        if &self.setting != font_to_use {
            self.font = allocator.create_font(font_to_use.as_ref(), font_kind);
            self.setting.clone_from(font_to_use);
        }
    }
}

pub struct FontCache<F> {
    pub timer: CachedFont<F>,
    pub times: CachedFont<F>,
    pub text: CachedFont<F>,
}

impl<F> FontCache<F> {
    pub fn new(allocator: &mut impl ResourceAllocator<Font = Handle<F>>) -> Self {
        Self {
            timer: CachedFont::new(allocator.create_font(None, FontKind::Timer)),
            times: CachedFont::new(allocator.create_font(None, FontKind::Times)),
            text: CachedFont::new(allocator.create_font(None, FontKind::Text)),
        }
    }

    pub fn maybe_reload(
        &mut self,
        allocator: &mut impl ResourceAllocator<Font = Handle<F>>,
        state: &LayoutState,
    ) {
        self.timer
            .maybe_reload(allocator, &state.timer_font, FontKind::Timer);
        self.times
            .maybe_reload(allocator, &state.times_font, FontKind::Times);
        self.text
            .maybe_reload(allocator, &state.text_font, FontKind::Text);
    }
}
