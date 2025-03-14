//! The software renderer allows rendering layouts entirely on the CPU. This is
//! surprisingly fast and can be considered the default renderer.

use livesplit_core::{layout::LayoutState, settings::ImageCache};

#[cfg(feature = "software-rendering")]
use livesplit_core::rendering::software::BorrowedRenderer as SoftwareRenderer;

use crate::slice_mut;

#[cfg(not(feature = "software-rendering"))]
/// dummy
pub struct SoftwareRenderer;
#[cfg(not(feature = "software-rendering"))]
impl SoftwareRenderer {
    fn new() -> Self {
        panic!("The software renderer is not compiled in.")
    }

    #[allow(warnings)]
    fn render(
        &mut self,
        _: &LayoutState,
        _: &ImageCache,
        _: &mut [u8],
        _: [u32; 2],
        _: u32,
        _: bool,
    ) {
    }
}

/// type
pub type OwnedSoftwareRenderer = Box<SoftwareRenderer>;

/// Creates a new software renderer.
#[unsafe(no_mangle)]
pub extern "C" fn SoftwareRenderer_new() -> OwnedSoftwareRenderer {
    Box::new(SoftwareRenderer::new())
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn SoftwareRenderer_drop(this: OwnedSoftwareRenderer) {
    drop(this);
}

/// Renders the layout state provided into the image buffer provided. The image
/// has to be an array of RGBA8 encoded pixels (red, green, blue, alpha with
/// each channel being an u8). Some frameworks may over allocate an image's
/// dimensions. So an image with dimensions 100x50 may be over allocated as
/// 128x64. In that case you provide the real dimensions of 100x50 as the width
/// and height, but a stride of 128 pixels as that correlates with the real
/// width of the underlying buffer. By default the renderer will try not to
/// redraw parts of the image that haven't changed. You can force a redraw in
/// case the image provided or its contents have changed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoftwareRenderer_render(
    this: &mut SoftwareRenderer,
    layout_state: &LayoutState,
    image_cache: &ImageCache,
    data: *mut u8,
    width: u32,
    height: u32,
    stride: u32,
    force_redraw: bool,
) {
    this.render(
        layout_state,
        image_cache,
        // SAFETY: The caller guarantees that `data` is valid for `stride *
        // height * 4` bytes.
        unsafe { slice_mut(data, stride as usize * height as usize * 4) },
        [width, height],
        stride,
        force_redraw,
    );
}
