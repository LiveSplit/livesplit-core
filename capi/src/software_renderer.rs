//! The software renderer allows rendering layouts entirely on the CPU. This is
//! surprisingly fast and can be considered the default renderer.

use livesplit_core::layout::LayoutState;

#[cfg(feature = "software-rendering")]
use livesplit_core::rendering::software::BorrowedSoftwareRenderer as SoftwareRenderer;

#[cfg(not(feature = "software-rendering"))]
pub struct SoftwareRenderer;
#[cfg(not(feature = "software-rendering"))]
impl SoftwareRenderer {
    fn new() -> Self {
        panic!("The software renderer is not compiled in.")
    }

    fn render(&mut self, _: &LayoutState, _: &mut [u8], _: [u32; 2], _: u32) {}
}

/// type
pub type OwnedSoftwareRenderer = Box<SoftwareRenderer>;

/// Creates a new software renderer.
#[no_mangle]
pub extern "C" fn SoftwareRenderer_new() -> OwnedSoftwareRenderer {
    Box::new(SoftwareRenderer::new())
}

/// drop
#[no_mangle]
pub extern "C" fn SoftwareRenderer_drop(this: OwnedSoftwareRenderer) {
    drop(this);
}

/// Renders the layout state provided into the image buffer provided. The
/// image has to be an array of RGBA8 encoded pixels (red, green, blue,
/// alpha with each channel being an u8). Some frameworks may over allocate
/// an image's dimensions. So an image with dimensions 100x50 may be over
/// allocated as 128x64. In that case you provide the real dimensions of
/// 100 and 50 as the width and height, but a stride of 128 pixels as that
/// correlates with the real width of the underlying buffer.
#[no_mangle]
pub unsafe extern "C" fn SoftwareRenderer_render(
    this: &mut SoftwareRenderer,
    layout_state: &LayoutState,
    data: *mut u8,
    width: u32,
    height: u32,
    stride: u32,
) {
    this.render(
        layout_state,
        std::slice::from_raw_parts_mut(data, stride as usize * height as usize * 4),
        [width, height],
        stride,
    );
}
