use std::marker::PhantomData;

use crate::{
    rendering::resource::{Handle, Handles, PathBuilder, ResourceAllocator, SharedOwnership},
    settings::Color,
};

use super::{color_font::iter_colored_glyphs, Font};
use hashbrown::HashMap;
use ttf_parser::OutlineBuilder;

struct GlyphBuilder<B, PB>(PB, PhantomData<fn(&mut B)>);

impl<B, PB: PathBuilder<B>> OutlineBuilder for GlyphBuilder<B, PB> {
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

pub struct GlyphCache<P> {
    glyphs: HashMap<u32, Vec<(Option<Color>, Handle<P>)>>,
}

impl<P> Default for GlyphCache<P> {
    fn default() -> Self {
        Self {
            glyphs: Default::default(),
        }
    }
}

impl<P: SharedOwnership> GlyphCache<P> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "font-loading")]
    pub fn clear(&mut self) {
        self.glyphs.clear();
    }

    pub fn lookup_or_insert(
        &mut self,
        font: &Font<'_>,
        glyph: u32,
        handles: &mut Handles<impl ResourceAllocator<Path = P>>,
    ) -> &[(Option<Color>, Handle<P>)] {
        self.glyphs.entry(glyph).or_insert_with(|| {
            let mut glyphs = Vec::new();
            iter_colored_glyphs(&font.color_tables, 0, glyph as _, |glyph, color| {
                let mut builder = GlyphBuilder(handles.path_builder(), PhantomData);
                font.outline_glyph(glyph, &mut builder);
                let path = builder.0.finish(handles);
                glyphs.push((color, path));
            });
            glyphs
        })
    }
}
