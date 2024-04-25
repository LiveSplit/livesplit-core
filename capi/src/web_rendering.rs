//! Provides a renderer for the web that renders into a canvas. The element can
//! then be attached anywhere in the DOM with any desired positioning and size.
//!
use livesplit_core::{layout::LayoutState, rendering::web, settings::ImageCache};
use wasm_bindgen::prelude::*;
use web_sys::Element;

/// The web renderer renders into a canvas element. The element can then be
/// attached anywhere in the DOM with any desired positioning and size.
#[wasm_bindgen]
pub struct CanvasRenderer {
    inner: web::Renderer,
}

#[wasm_bindgen]
impl CanvasRenderer {
    /// Creates a new web renderer that renders into a canvas element. The
    /// element can then be attached anywhere in the DOM with any desired
    /// positioning and size. There are two CSS fonts that are used as the
    /// default fonts. They are called "timer" and "fira". Make sure they are
    /// fully loaded before creating the renderer as otherwise information about
    /// a fallback font is cached instead.
    #[allow(clippy::new_without_default)]
    pub fn new() -> CanvasRenderer {
        Self {
            inner: web::Renderer::new(),
        }
    }

    /// Returns the HTML element. This can be attached anywhere in the DOM with
    /// any desired positioning and size.
    pub fn element(&self) -> Element {
        self.inner.element().clone()
    }

    /// Renders the layout state into the canvas. The image cache is used to
    /// retrieve images that are used in the layout state.
    pub unsafe fn render(&mut self, state: usize, image_cache: usize) {
        let state = unsafe { core::mem::transmute::<usize, &LayoutState>(state) };
        let image_cache = unsafe { core::mem::transmute::<usize, &ImageCache>(image_cache) };
        self.inner.render(state, image_cache);
    }
}
