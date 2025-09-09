//! Provides a renderer for the web that renders into a canvas. The element can
//! then be attached anywhere in the DOM with any desired positioning and size.

use livesplit_core::{layout::LayoutState, rendering::web, settings::ImageCache};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

/// The web renderer renders into a canvas element. The element can then be
/// attached anywhere in the DOM with any desired positioning and size.
#[wasm_bindgen]
pub struct WebRenderer {
    inner: web::Renderer,
}

#[wasm_bindgen]
impl WebRenderer {
    /// Creates a new web renderer that renders into a canvas element. The
    /// element can then be attached anywhere in the DOM with any desired
    /// positioning and size. There are two CSS fonts that are used as the
    /// default fonts. They are called "timer" and "fira". Make sure they are
    /// fully loaded before creating the renderer as otherwise information about
    /// a fallback font is cached instead.
    #[expect(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: web::Renderer::new(),
        }
    }

    /// Returns the HTML element. This can be attached anywhere in the DOM with
    /// any desired positioning and size.
    pub fn element(&self) -> HtmlElement {
        self.inner.element().clone()
    }

    /// Renders the layout state into the canvas. The image cache is used to
    /// retrieve images that are used in the layout state.
    pub unsafe fn render(
        &mut self,
        state: *const LayoutState,
        image_cache: *const ImageCache,
    ) -> Option<Box<[f32]>> {
        // SAFETY: The caller must ensure that the pointers are valid.
        unsafe { self.inner.render(&*state, &*image_cache).map(Box::from) }
    }
}
