//! Provides a software renderer that can be used without a GPU. The renderer is
//! surprisingly fast and can be considered the default rendering backend.

use std::{mem, ops::Deref, rc::Rc};

use super::{
    entity::Entity,
    resource::{self, ResourceAllocator},
    FillShader, Scene, SceneManager, SharedOwnership, Transform,
};
use crate::layout::LayoutState;
use image::ImageBuffer;
use tiny_skia::{
    BlendMode, Color, FillRule, FilterQuality, GradientStop, LinearGradient, Paint, Path,
    PathBuilder, Pattern, Pixmap, PixmapMut, Point, Rect, Shader, SpreadMode, Stroke,
};

pub use image::{self, RgbaImage};

struct SkiaBuilder(PathBuilder);

type SkiaPath = Option<UnsafeRc<Path>>;
type SkiaImage = Option<UnsafeRc<Pixmap>>;

impl resource::PathBuilder<SkiaAllocator> for SkiaBuilder {
    type Path = SkiaPath;

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

    fn finish(self, _: &mut SkiaAllocator) -> Self::Path {
        self.0.finish().map(UnsafeRc::new)
    }
}

fn convert_color(&[r, g, b, a]: &[f32; 4]) -> Color {
    Color::from_rgba(r, g, b, a).unwrap()
}

fn convert_transform(transform: &Transform) -> tiny_skia::Transform {
    let [sx, ky, kx, sy, tx, ty] = transform.to_array();
    tiny_skia::Transform::from_row(sx, ky, kx, sy, tx, ty)
}

struct SkiaAllocator;

impl ResourceAllocator for SkiaAllocator {
    type PathBuilder = SkiaBuilder;
    type Path = SkiaPath;
    type Image = SkiaImage;

    fn path_builder(&mut self) -> Self::PathBuilder {
        SkiaBuilder(PathBuilder::new())
    }

    fn create_image(&mut self, width: u32, height: u32, data: &[u8]) -> Self::Image {
        let mut image = Pixmap::new(width, height)?;
        for (d, &[r, g, b, a]) in image
            .pixels_mut()
            .iter_mut()
            .zip(bytemuck::cast_slice::<_, [u8; 4]>(data))
        {
            *d = tiny_skia::ColorU8::from_rgba(r, g, b, a).premultiply();
        }
        Some(UnsafeRc::new(image))
    }
}

/// The software renderer allows rendering layouts entirely on the CPU. This is
/// surprisingly fast and can be considered the default renderer. There are two
/// versions of the software renderer. This version of the software renderer
/// does not own the image to render into. This allows the caller to manage
/// their own image buffer.
pub struct BorrowedRenderer {
    scene_manager: SceneManager<SkiaPath, SkiaImage>,
    background: Pixmap,
    min_y: f32,
    max_y: f32,
}

struct UnsafeRc<T>(Rc<T>);

impl<T: Send + Sync> Deref for UnsafeRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<T> UnsafeRc<T> {
    fn new(val: T) -> Self {
        Self(Rc::new(val))
    }
}

impl<T: Send + Sync> SharedOwnership for UnsafeRc<T> {
    fn share(&self) -> Self {
        Self(self.0.share())
    }
}

// TODO: Make sure this is actually true:

// Safety: This is safe because the BorrowedSoftwareRenderer and the
// SceneManager never share any of their resources with anyone. For the
// BorrowedSoftwareRenderer this is trivially true as it doesn't share any its
// fields with anyone, you provide the image to render into yourself. For the
// SceneManager it's harder to prove. However as long as the trait bounds for
// the ResourceAllocator's Image and Path types do not require Sync or Send,
// then the SceneManager simply can't share any of the allocated resources
// across any threads at all.
unsafe impl<T: Send + Sync> Send for UnsafeRc<T> {}

// Safety: The BorrowedSoftwareRenderer only has a render method which requires
// exclusive access. The SceneManager could still mess it up. But as long as the
// ResourceAllocator's Image and Path types do not require Sync or Send, it
// can't make use of the Sync bound in any dangerous way anyway.
unsafe impl<T: Send + Sync> Sync for UnsafeRc<T> {}

impl Default for BorrowedRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl BorrowedRenderer {
    /// Creates a new software renderer.
    pub fn new() -> Self {
        Self {
            scene_manager: SceneManager::new(SkiaAllocator),
            background: Pixmap::new(1, 1).unwrap(),
            min_y: f32::INFINITY,
            max_y: f32::NEG_INFINITY,
        }
    }

    /// Renders the layout state provided into the image buffer provided. The
    /// image has to be an array of `RGBA8` encoded pixels (red, green, blue,
    /// alpha with each channel being an u8). Some frameworks may over allocate
    /// an image's dimensions. So an image with dimensions `100x50` may be over
    /// allocated as `128x64`. In that case you provide the real dimensions of
    /// `100x50` as the width and height, but a stride of `128` pixels as that
    /// correlates with the real width of the underlying buffer. It may detect
    /// that the layout got resized. In that case it returns the new ideal size.
    /// This is just a hint and can be ignored entirely. The image is always
    /// rendered with the resolution provided. By default the renderer will try
    /// not to redraw parts of the image that haven't changed. You can force a
    /// redraw in case the image provided or its contents have changed.
    pub fn render(
        &mut self,
        state: &LayoutState,
        image: &mut [u8],
        [width, height]: [u32; 2],
        stride: u32,
        force_redraw: bool,
    ) -> Option<(f32, f32)> {
        let mut frame_buffer = PixmapMut::from_bytes(image, stride, height).unwrap();

        if stride != self.background.width() || height != self.background.height() {
            self.background = Pixmap::new(stride, height).unwrap();
        }

        let new_resolution =
            self.scene_manager
                .update_scene(SkiaAllocator, (width as _, height as _), &state);

        let scene = self.scene_manager.scene();
        let rectangle = scene.rectangle();
        let rectangle = rectangle.as_deref().unwrap();

        let bottom_layer_changed = scene.bottom_layer_changed();

        let mut background = self.background.as_mut();

        if bottom_layer_changed {
            fill_background(scene, &mut background, width, height);
            render_layer(&mut background, scene.bottom_layer(), rectangle);
        }

        let top_layer = scene.top_layer();

        let (min_y, max_y) = calculate_bounds(top_layer);
        let min_y = mem::replace(&mut self.min_y, min_y).min(min_y);
        let max_y = mem::replace(&mut self.max_y, max_y).max(max_y);

        if force_redraw || bottom_layer_changed {
            frame_buffer
                .data_mut()
                .copy_from_slice(background.data_mut());
        } else if min_y <= max_y {
            let stride = 4 * stride as usize;
            let min_y = stride * (min_y - 1.0) as usize;
            let max_y = stride * ((max_y + 2.0) as usize).min(height as usize);

            frame_buffer.data_mut()[min_y..max_y]
                .copy_from_slice(&background.data_mut()[min_y..max_y]);
        }

        render_layer(&mut frame_buffer, top_layer, rectangle);

        new_resolution
    }
}

/// The software renderer allows rendering layouts entirely on the CPU. This is
/// surprisingly fast and can be considered the default renderer. There are two
/// versions of the software renderer. This version of the software renderer
/// owns the image it renders into.
pub struct Renderer {
    renderer: BorrowedRenderer,
    frame_buffer: Pixmap,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
    /// Creates a new software renderer.
    pub fn new() -> Self {
        Self {
            renderer: BorrowedRenderer::new(),
            frame_buffer: Pixmap::new(1, 1).unwrap(),
        }
    }

    /// Renders the layout state provided with the chosen resolution. It may
    /// detect that the layout got resized. In that case it returns the new
    /// ideal size. This is just a hint and can be ignored entirely. The image
    /// is always rendered with the resolution provided.
    pub fn render(&mut self, state: &LayoutState, [width, height]: [u32; 2]) -> Option<(f32, f32)> {
        if width != self.frame_buffer.width() || height != self.frame_buffer.height() {
            self.frame_buffer = Pixmap::new(width, height).unwrap();
        }

        self.renderer.render(
            state,
            self.frame_buffer.data_mut(),
            [width, height],
            width,
            false,
        )
    }

    /// Accesses the image as a byte slice of RGBA8 encoded pixels (red, green,
    /// blue, alpha with each channel being an u8).
    pub fn image_data(&self) -> &[u8] {
        self.frame_buffer.data()
    }

    /// Turns the whole renderer into the underlying image buffer of RGBA8
    /// encoded pixels (red, green, blue, alpha with each channel being an u8).
    pub fn into_image_data(self) -> Vec<u8> {
        self.frame_buffer.take()
    }

    /// Accesses the image.
    pub fn image(&self) -> ImageBuffer<image::Rgba<u8>, &[u8]> {
        ImageBuffer::from_raw(
            self.frame_buffer.width(),
            self.frame_buffer.height(),
            self.frame_buffer.data(),
        )
        .unwrap()
    }

    /// Turns the whole renderer into the underlying image.
    pub fn into_image(self) -> RgbaImage {
        RgbaImage::from_raw(
            self.frame_buffer.width(),
            self.frame_buffer.height(),
            self.frame_buffer.take(),
        )
        .unwrap()
    }
}

fn render_layer(
    canvas: &mut PixmapMut<'_>,
    layer: &[Entity<SkiaPath, SkiaImage>],
    rectangle: &Path,
) {
    for entity in layer {
        match entity {
            Entity::FillPath(path, shader, transform) => {
                if let Some(path) = path.as_deref() {
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

                    canvas.fill_path(
                        path,
                        &Paint {
                            shader,
                            anti_alias: true,
                            ..Default::default()
                        },
                        FillRule::Winding,
                        convert_transform(transform),
                        None,
                    );
                }
            }
            Entity::StrokePath(path, stroke_width, color, transform) => {
                if let Some(path) = path.as_deref() {
                    canvas.stroke_path(
                        path,
                        &Paint {
                            shader: Shader::SolidColor(convert_color(color)),
                            anti_alias: true,
                            ..Default::default()
                        },
                        &Stroke {
                            width: *stroke_width,
                            ..Default::default()
                        },
                        convert_transform(transform),
                        None,
                    );
                }
            }
            Entity::Image(image, transform) => {
                if let Some(image) = image.as_deref() {
                    canvas.fill_path(
                        rectangle,
                        &Paint {
                            shader: Pattern::new(
                                image.as_ref(),
                                SpreadMode::Pad,
                                FilterQuality::Bilinear,
                                1.0,
                                tiny_skia::Transform::from_scale(
                                    1.0 / image.width() as f32,
                                    1.0 / image.height() as f32,
                                ),
                            ),
                            anti_alias: true,
                            ..Default::default()
                        },
                        FillRule::Winding,
                        convert_transform(transform),
                        None,
                    );
                }
            }
        }
    }
}

fn fill_background(
    scene: &Scene<SkiaPath, SkiaImage>,
    background: &mut PixmapMut<'_>,
    width: u32,
    height: u32,
) {
    match scene.background() {
        Some(shader) => match shader {
            FillShader::SolidColor(color) => {
                background
                    .pixels_mut()
                    .fill(convert_color(color).premultiply().to_color_u8());
            }
            FillShader::VerticalGradient(top, bottom) => {
                background.fill_rect(
                    Rect::from_xywh(0.0, 0.0, width as _, height as _).unwrap(),
                    &Paint {
                        shader: LinearGradient::new(
                            Point::from_xy(0.0, 0.0),
                            Point::from_xy(0.0, height as _),
                            vec![
                                GradientStop::new(0.0, convert_color(top)),
                                GradientStop::new(1.0, convert_color(bottom)),
                            ],
                            SpreadMode::Pad,
                            tiny_skia::Transform::identity(),
                        )
                        .unwrap(),
                        blend_mode: BlendMode::Source,
                        ..Default::default()
                    },
                    tiny_skia::Transform::identity(),
                    None,
                );
            }
            FillShader::HorizontalGradient(left, right) => {
                background.fill_rect(
                    Rect::from_xywh(0.0, 0.0, width as _, height as _).unwrap(),
                    &Paint {
                        shader: LinearGradient::new(
                            Point::from_xy(0.0, 0.0),
                            Point::from_xy(width as _, 0.0),
                            vec![
                                GradientStop::new(0.0, convert_color(left)),
                                GradientStop::new(1.0, convert_color(right)),
                            ],
                            SpreadMode::Pad,
                            tiny_skia::Transform::identity(),
                        )
                        .unwrap(),
                        blend_mode: BlendMode::Source,
                        ..Default::default()
                    },
                    tiny_skia::Transform::identity(),
                    None,
                );
            }
        },
        None => background.data_mut().fill(0),
    }
}

fn calculate_bounds(layer: &[Entity<SkiaPath, SkiaImage>]) -> (f32, f32) {
    let (mut min_y, mut max_y) = (f32::INFINITY, f32::NEG_INFINITY);
    for entity in layer.iter() {
        match entity {
            Entity::FillPath(path, _, transform) | Entity::StrokePath(path, _, _, transform) => {
                if let Some(path) = &**path {
                    let [_, ky, _, sy, _, ty] = transform.to_array();
                    let bounds = path.bounds();
                    let (l, r, t, b) =
                        (bounds.left(), bounds.right(), bounds.top(), bounds.bottom());
                    for &(x, y) in &[(l, t), (r, t), (l, b), (r, b)] {
                        let transformed_y = ky * x + sy * y + ty;
                        min_y = min_y.min(transformed_y);
                        max_y = max_y.max(transformed_y);
                    }
                }
            }
            Entity::Image(image, transform) => {
                if image.is_some() {
                    let [_, ky, _, sy, _, ty] = transform.to_array();
                    let (l, r, t, b) = (0.0, 1.0, 0.0, 1.0);
                    for &(x, y) in &[(l, t), (r, t), (l, b), (r, b)] {
                        let transformed_y = ky * x + sy * y + ty;
                        min_y = min_y.min(transformed_y);
                        max_y = max_y.max(transformed_y);
                    }
                }
            }
        }
    }
    (min_y, max_y)
}
