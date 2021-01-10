//! Provides a software renderer that can be used without a GPU. The renderer is
//! surprisingly fast and can be considered the default rendering backend.

use super::{Backend, FillShader, Renderer, Rgba, Transform};
use crate::layout::LayoutState;
use image::ImageBuffer;
use tiny_skia::{
    Canvas, Color, FillRule, FilterQuality, GradientStop, LinearGradient, Paint, Path, PathBuilder,
    Pattern, Pixmap, PixmapMut, Point, Shader, SpreadMode, Stroke,
};

pub use image::{self, RgbaImage};

struct SkiaBuilder(PathBuilder);

impl super::PathBuilder<SoftwareBackend<'_>> for SkiaBuilder {
    type Path = Option<Path>;

    fn move_to(&mut self, x: f32, y: f32) {
        self.0.move_to(x, y)
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.0.line_to(x, y)
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.0.quad_to(x1, y1, x, y)
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.0.cubic_to(x1, y1, x2, y2, x, y)
    }

    fn close(&mut self) {
        self.0.close()
    }

    fn finish(self, _: &mut SoftwareBackend<'_>) -> Self::Path {
        self.0.finish()
    }
}

fn convert_color([r, g, b, a]: [f32; 4]) -> Color {
    Color::from_rgba(r, g, b, a).unwrap()
}

fn convert_transform(transform: Transform) -> tiny_skia::Transform {
    let [sx, ky, kx, sy, tx, ty] = transform.to_array();
    tiny_skia::Transform::from_row(sx, ky, kx, sy, tx, ty).unwrap()
}

struct SoftwareBackend<'a> {
    canvas: Canvas<'a>,
}

impl Backend for SoftwareBackend<'_> {
    type PathBuilder = SkiaBuilder;
    type Path = Option<Path>;
    type Image = Option<Pixmap>;

    fn path_builder(&mut self) -> Self::PathBuilder {
        SkiaBuilder(PathBuilder::new())
    }

    fn render_fill_path(&mut self, path: &Self::Path, shader: FillShader, transform: Transform) {
        if let Some(path) = path {
            self.canvas.set_transform(convert_transform(transform));

            let shader = match shader {
                FillShader::SolidColor(col) => Shader::SolidColor(convert_color(col)),
                FillShader::VerticalGradient(top, bottom) => {
                    let bounds = path.bounds();
                    LinearGradient::new(
                        Point::from_xy(0.0, bounds.top()),
                        Point::from_xy(0.0, bounds.bottom()),
                        vec![
                            GradientStop::new(0.0, convert_color(top)),
                            GradientStop::new(1.0, convert_color(bottom)),
                        ],
                        SpreadMode::Pad,
                        tiny_skia::Transform::identity(),
                    )
                    .unwrap()
                }
                FillShader::HorizontalGradient(left, right) => {
                    let bounds = path.bounds();
                    LinearGradient::new(
                        Point::from_xy(bounds.left(), 0.0),
                        Point::from_xy(bounds.right(), 0.0),
                        vec![
                            GradientStop::new(0.0, convert_color(left)),
                            GradientStop::new(1.0, convert_color(right)),
                        ],
                        SpreadMode::Pad,
                        tiny_skia::Transform::identity(),
                    )
                    .unwrap()
                }
            };

            self.canvas.fill_path(
                path,
                &Paint {
                    shader,
                    anti_alias: true,
                    ..Default::default()
                },
                FillRule::Winding,
            );
        }
    }

    fn render_stroke_path(
        &mut self,
        path: &Self::Path,
        stroke_width: f32,
        color: Rgba,
        transform: Transform,
    ) {
        if let Some(path) = path {
            self.canvas.set_transform(convert_transform(transform));

            self.canvas.stroke_path(
                path,
                &Paint {
                    shader: Shader::SolidColor(convert_color(color)),
                    anti_alias: true,
                    ..Default::default()
                },
                &Stroke {
                    width: stroke_width,
                    ..Default::default()
                },
            );
        }
    }

    fn render_image(&mut self, image: &Self::Image, rectangle: &Self::Path, transform: Transform) {
        if let (Some(path), Some(image)) = (rectangle, image) {
            self.canvas.set_transform(convert_transform(transform));

            self.canvas.fill_path(
                path,
                &Paint {
                    shader: Pattern::new(
                        image.as_ref(),
                        SpreadMode::Pad,
                        FilterQuality::Bilinear,
                        1.0,
                        tiny_skia::Transform::from_scale(
                            1.0 / image.width() as f32,
                            1.0 / image.height() as f32,
                        )
                        .unwrap(),
                    ),
                    anti_alias: true,
                    ..Default::default()
                },
                FillRule::Winding,
            );
        }
    }

    fn free_path(&mut self, _: Self::Path) {}

    fn create_image(&mut self, width: u32, height: u32, data: &[u8]) -> Self::Image {
        let mut image = Pixmap::new(width, height)?;
        for (d, &[r, g, b, a]) in image
            .pixels_mut()
            .iter_mut()
            .zip(bytemuck::cast_slice::<_, [u8; 4]>(data))
        {
            *d = tiny_skia::ColorU8::from_rgba(r, g, b, a).premultiply();
        }
        Some(image)
    }

    fn free_image(&mut self, _: Self::Image) {}
}

/// The software renderer allows rendering layouts entirely on the CPU. This is
/// surprisingly fast and can be considered the default renderer. There are two
/// versions of the software renderer. This version of the software renderer
/// does not own the image to render into. This allows the caller to manage
/// their own image buffer.
pub struct BorrowedSoftwareRenderer {
    renderer: Renderer<Option<Path>, Option<Pixmap>>,
}

impl Default for BorrowedSoftwareRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl BorrowedSoftwareRenderer {
    /// Creates a new software renderer.
    pub fn new() -> Self {
        Self {
            renderer: Renderer::new(),
        }
    }

    /// Renders the layout state provided into the image buffer provided. The
    /// image has to be an array of RGBA8 encoded pixels (red, green, blue,
    /// alpha with each channel being an u8). Some frameworks may over allocate
    /// an image's dimensions. So an image with dimensions 100x50 may be over
    /// allocated as 128x64. In that case you provide the real dimensions of
    /// [100, 50] as the width and height, but a stride of 128 pixels as that
    /// correlates with the real width of the underlying buffer. It may detect
    /// that the layout got resized. In that case it returns the new ideal size.
    /// This is entirely a hint and can be ignored entirely. The image is always
    /// rendered with the resolution provided.
    pub fn render(
        &mut self,
        state: &LayoutState,
        image: &mut [u8],
        [width, height]: [u32; 2],
        stride: u32,
    ) -> Option<(f32, f32)> {
        let mut pixmap = PixmapMut::from_bytes(image, stride, height).unwrap();

        // FIXME: .fill() once it's stable.
        for b in pixmap.data_mut() {
            *b = 0;
        }

        let mut backend = SoftwareBackend {
            canvas: Canvas::from(pixmap),
        };

        self.renderer
            .render(&mut backend, (width as _, height as _), &state)
    }
}

/// The software renderer allows rendering layouts entirely on the CPU. This is
/// surprisingly fast and can be considered the default renderer. There are two
/// versions of the software renderer. This version of the software renderer
/// owns the image it renders into.
pub struct SoftwareRenderer {
    renderer: Renderer<Option<Path>, Option<Pixmap>>,
    pixmap: Pixmap,
}

impl Default for SoftwareRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl SoftwareRenderer {
    /// Creates a new software renderer.
    pub fn new() -> Self {
        Self {
            renderer: Renderer::new(),
            pixmap: Pixmap::new(1, 1).unwrap(),
        }
    }

    /// Renders the layout state provided with the chosen resolution. It may
    /// detect that the layout got resized. In that case it returns the new
    /// ideal size. This is entirely a hint and can be ignored entirely. The
    /// image is always rendered with the resolution provided.
    pub fn render(&mut self, state: &LayoutState, [width, height]: [u32; 2]) -> Option<(f32, f32)> {
        if width != self.pixmap.width() || height != self.pixmap.height() {
            self.pixmap = Pixmap::new(width, height).unwrap();
        } else {
            // FIXME: .fill() once it's stable.
            for b in self.pixmap.data_mut() {
                *b = 0;
            }
        }

        let mut backend = SoftwareBackend {
            canvas: Canvas::from(self.pixmap.as_mut()),
        };

        self.renderer
            .render(&mut backend, (width as _, height as _), &state)
    }

    /// Accesses the image as a byte slice of RGBA8 encoded pixels (red, green,
    /// blue, alpha with each channel being an u8).
    pub fn image_data(&self) -> &[u8] {
        self.pixmap.data()
    }

    /// Turns the whole renderer into the underlying image buffer of RGBA8
    /// encoded pixels (red, green, blue, alpha with each channel being an u8).
    pub fn into_image_data(self) -> Vec<u8> {
        self.pixmap.take()
    }

    /// Accesses the image.
    pub fn image(&self) -> ImageBuffer<image::Rgba<u8>, &[u8]> {
        ImageBuffer::from_raw(
            self.pixmap.width(),
            self.pixmap.height(),
            self.pixmap.data(),
        )
        .unwrap()
    }

    /// Turns the whole renderer into the underlying image.
    pub fn into_image(self) -> RgbaImage {
        RgbaImage::from_raw(
            self.pixmap.width(),
            self.pixmap.height(),
            self.pixmap.take(),
        )
        .unwrap()
    }
}
