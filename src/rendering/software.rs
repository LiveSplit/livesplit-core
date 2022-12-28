//! Provides a software renderer that can be used without a GPU. The renderer is
//! surprisingly fast and can be considered the default rendering backend.

use crate::platform::prelude::*;
use alloc::rc::Rc;
use core::{mem, ops::Deref};

use super::{
    entity::Entity,
    path_based_text_engine::{Font, Label, TextEngine},
    resource::{self, ResourceAllocator},
    FillShader, FontKind, Scene, SceneManager, SharedOwnership, Transform,
};
use crate::{layout::LayoutState, settings};
#[cfg(feature = "image")]
use image::ImageBuffer;
use tiny_skia::{
    BlendMode, Color, FillRule, FilterQuality, GradientStop, LinearGradient, Paint, Path,
    PathBuilder, Pattern, Pixmap, PixmapMut, Point, Rect, Shader, SpreadMode, Stroke,
};

#[cfg(feature = "image")]
pub use image::{self, RgbaImage};

struct SkiaBuilder(PathBuilder);

type SkiaPath = Option<UnsafeRc<Path>>;
type SkiaImage = UnsafeRc<Pixmap>;
type SkiaFont = Font<SkiaPath>;
type SkiaLabel = Label<SkiaPath>;

impl resource::PathBuilder for SkiaBuilder {
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

    fn finish(self) -> Self::Path {
        self.0.finish().map(UnsafeRc::new)
    }
}

fn convert_color(&[r, g, b, a]: &[f32; 4]) -> Color {
    Color::from_rgba(r, g, b, a).unwrap()
}

fn convert_transform(transform: &Transform) -> tiny_skia::Transform {
    tiny_skia::Transform::from_row(
        transform.scale_x,
        0.0,
        0.0,
        transform.scale_y,
        transform.x,
        transform.y,
    )
}

struct SkiaAllocator {
    text_engine: TextEngine,
}

impl ResourceAllocator for SkiaAllocator {
    type PathBuilder = SkiaBuilder;
    type Path = SkiaPath;
    type Image = SkiaImage;
    type Font = SkiaFont;
    type Label = SkiaLabel;

    fn path_builder(&mut self) -> Self::PathBuilder {
        path_builder()
    }

    fn create_image(&mut self, _data: &[u8]) -> Option<(Self::Image, f32)> {
        #[cfg(feature = "image")]
        {
            let mut buf = image::load_from_memory(_data).ok()?.to_rgba8();

            // Premultiplication
            for [r, g, b, a] in bytemuck::cast_slice_mut::<u8, [u8; 4]>(&mut buf) {
                // If it's opaque we can skip the entire pixel. However this
                // hurts vectorization, so we want to avoid it if the compiler
                // can vectorize the loop. WASM, PowerPC, and MIPS are
                // unaffected at the moment.
                #[cfg(not(any(target_feature = "avx2", target_feature = "neon")))]
                if *a == 0xFF {
                    continue;
                }
                let a = *a as u16;
                *r = ((*r as u16 * a) / 255) as u8;
                *g = ((*g as u16 * a) / 255) as u8;
                *b = ((*b as u16 * a) / 255) as u8;
            }

            let (width, height) = (buf.width(), buf.height());

            let pixmap = Pixmap::from_vec(
                buf.into_raw(),
                tiny_skia_path::IntSize::from_wh(width, height)?,
            )?;

            Some((UnsafeRc::new(pixmap), width as f32 / height as f32))
        }
        #[cfg(not(feature = "image"))]
        {
            None
        }
    }

    fn create_font(&mut self, font: Option<&settings::Font>, kind: FontKind) -> Self::Font {
        self.text_engine.create_font(font, kind)
    }

    fn create_label(
        &mut self,
        text: &str,
        font: &mut Self::Font,
        max_width: Option<f32>,
    ) -> Self::Label {
        self.text_engine
            .create_label(path_builder, text, font, max_width)
    }

    fn update_label(
        &mut self,
        label: &mut Self::Label,
        text: &str,
        font: &mut Self::Font,
        max_width: Option<f32>,
    ) {
        self.text_engine
            .update_label(path_builder, label, text, font, max_width)
    }
}

fn path_builder() -> SkiaBuilder {
    SkiaBuilder(PathBuilder::new())
}

/// The software renderer allows rendering layouts entirely on the CPU. This is
/// surprisingly fast and can be considered the default renderer. There are two
/// versions of the software renderer. This version of the software renderer
/// does not own the image to render into. This allows the caller to manage
/// their own image buffer.
pub struct BorrowedRenderer {
    allocator: SkiaAllocator,
    scene_manager: SceneManager<SkiaPath, SkiaImage, SkiaFont, SkiaLabel>,
    background: Pixmap,
    min_y: f32,
    max_y: f32,
}

struct UnsafeRc<T>(Rc<T>);

impl<T: Send + Sync> Deref for UnsafeRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
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
        let mut allocator = SkiaAllocator {
            text_engine: TextEngine::new(),
        };
        let scene_manager = SceneManager::new(&mut allocator);
        Self {
            allocator,
            scene_manager,
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
                .update_scene(&mut self.allocator, (width as _, height as _), state);

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
    #[cfg(feature = "image")]
    pub fn image(&self) -> ImageBuffer<image::Rgba<u8>, &[u8]> {
        ImageBuffer::from_raw(
            self.frame_buffer.width(),
            self.frame_buffer.height(),
            self.frame_buffer.data(),
        )
        .unwrap()
    }

    /// Turns the whole renderer into the underlying image.
    #[cfg(feature = "image")]
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
    layer: &[Entity<SkiaPath, SkiaImage, SkiaLabel>],
    rectangle: &Path,
) {
    for entity in layer {
        match entity {
            Entity::FillPath(path, shader, transform) => {
                if let Some(path) = path.as_deref() {
                    let paint = convert_shader(
                        shader,
                        path,
                        |path| {
                            let bounds = path.bounds();
                            (bounds.top(), bounds.bottom())
                        },
                        |path| {
                            let bounds = path.bounds();
                            (bounds.left(), bounds.right())
                        },
                    );

                    canvas.fill_path(
                        path,
                        &paint,
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
            Entity::Label(label, shader, transform) => {
                let label = label.read().unwrap();
                let label = &*label;

                let paint = convert_shader(
                    shader,
                    label,
                    |label| {
                        let (mut top, mut bottom) = (f32::INFINITY, f32::NEG_INFINITY);
                        for glyph in label.glyphs() {
                            if let Some(path) = &glyph.path {
                                let bounds = path.bounds();
                                top = top.min(bounds.top());
                                bottom = bottom.max(bounds.bottom());
                            }
                        }
                        if bottom < top {
                            (0.0, 0.0)
                        } else {
                            (top, bottom)
                        }
                    },
                    |label| {
                        let (mut left, mut right) = (f32::INFINITY, f32::NEG_INFINITY);
                        for glyph in label.glyphs() {
                            if let Some(path) = &glyph.path {
                                let bounds = path.bounds();
                                left = left.min(bounds.left());
                                right = right.max(bounds.right());
                            }
                        }
                        if right < left {
                            (0.0, 0.0)
                        } else {
                            (left, right)
                        }
                    },
                );

                let transform = transform.pre_scale(label.scale(), label.scale());

                for glyph in label.glyphs() {
                    if let Some(path) = &glyph.path {
                        let transform = transform.pre_translate(glyph.x, glyph.y);

                        let glyph_paint;
                        let paint = if let Some(color) = &glyph.color {
                            glyph_paint = Paint {
                                shader: Shader::SolidColor(convert_color(color)),
                                ..paint
                            };
                            &glyph_paint
                        } else {
                            &paint
                        };

                        canvas.fill_path(
                            path,
                            paint,
                            FillRule::Winding,
                            convert_transform(&transform),
                            None,
                        );
                    }
                }
            }
        }
    }
}

fn convert_shader<T>(
    shader: &FillShader,
    has_bounds: &T,
    calculate_top_bottom: impl FnOnce(&T) -> (f32, f32),
    calculate_left_right: impl FnOnce(&T) -> (f32, f32),
) -> Paint<'static> {
    let shader = match shader {
        FillShader::SolidColor(col) => Shader::SolidColor(convert_color(col)),
        FillShader::VerticalGradient(top, bottom) => {
            let (bound_top, bound_bottom) = calculate_top_bottom(has_bounds);
            LinearGradient::new(
                Point::from_xy(0.0, bound_top),
                Point::from_xy(0.0, bound_bottom),
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
            let (bound_left, bound_right) = calculate_left_right(has_bounds);
            LinearGradient::new(
                Point::from_xy(bound_left, 0.0),
                Point::from_xy(bound_right, 0.0),
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

    Paint {
        shader,
        anti_alias: true,
        ..Default::default()
    }
}

fn fill_background(
    scene: &Scene<SkiaPath, SkiaImage, SkiaLabel>,
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

fn calculate_bounds(layer: &[Entity<SkiaPath, SkiaImage, SkiaLabel>]) -> (f32, f32) {
    let (mut min_y, mut max_y) = (f32::INFINITY, f32::NEG_INFINITY);
    for entity in layer.iter() {
        match entity {
            Entity::FillPath(path, _, transform) => {
                if let Some(path) = &**path {
                    let bounds = path.bounds();
                    for y in [bounds.top(), bounds.bottom()] {
                        let transformed_y = transform.transform_y(y);
                        min_y = min_y.min(transformed_y);
                        max_y = max_y.max(transformed_y);
                    }
                }
            }
            Entity::StrokePath(path, radius, _, transform) => {
                if let Some(path) = &**path {
                    let radius = transform.scale_y * radius;
                    let bounds = path.bounds();
                    for y in [bounds.top(), bounds.bottom()] {
                        let transformed_y = transform.transform_y(y);
                        min_y = min_y.min(transformed_y - radius);
                        max_y = max_y.max(transformed_y + radius);
                    }
                }
            }
            Entity::Image(_, transform) => {
                for y in [0.0, 1.0] {
                    let transformed_y = transform.transform_y(y);
                    min_y = min_y.min(transformed_y);
                    max_y = max_y.max(transformed_y);
                }
            }
            Entity::Label(label, _, transform) => {
                for glyph in label.read().unwrap().glyphs() {
                    if let Some(path) = &glyph.path {
                        let transform = transform.pre_translate(glyph.x, glyph.y);
                        let bounds = path.bounds();
                        for y in [bounds.top(), bounds.bottom()] {
                            let transformed_y = transform.transform_y(y);
                            min_y = min_y.min(transformed_y);
                            max_y = max_y.max(transformed_y);
                        }
                    }
                }
            }
        }
    }
    (min_y, max_y)
}
