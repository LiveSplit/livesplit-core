//! The rendering module provides a [`SceneManager`] that, when provided with a
//! [`LayoutState`], places [`Entities`](Entity) into a [`Scene`] and updates it
//! accordingly as the [`LayoutState`] changes. It is up to a specific renderer
//! to then take the [`Scene`] and render out the [`Entities`](Entity). There is
//! a [`ResourceAllocator`] trait that defines the types of resources an
//! [`Entity`] consists of. A specific renderer can then provide an
//! implementation that provides the resources it can render out. Those
//! resources are images and paths, i.e. lines, quadratic and cubic bezier
//! curves. An optional software renderer is available behind the
//! `software-rendering` feature that uses tiny-skia to efficiently render the
//! paths on the CPU. It is surprisingly fast and can be considered the default
//! renderer.

// # Coordinate spaces used in this module
//
// ## Backend Coordinate Space
//
// The backend can choose its own coordinate space by passing its own width and
// height to the renderer. (0, 0) is the top left corner of the rendered layout,
// and (width, height) is the bottom right corner. In most situations width and
// height will be the actual pixel dimensions of the image that is being
// rendered to.
//
// ## Renderer Coordinate Space
//
// The renderer internally uses the so called renderer coordinate space. It has
// the dimensions [width, 1] with the width being chosen such that the renderer
// coordinate space respects the aspect ratio of the render target. This
// coordinate space is mostly an implementation detail.
//
// ## Component Coordinate Space
//
// The component coordinate space is a coordinate space local to a single
// component. This means that (0, 0) is the top left corner and (width, height)
// is the bottom right corner of the component. Width and Height are chosen
// based on various properties. In vertical mode, the height is chosen to be the
// component's actual height, while the width is dynamically adjusted based on
// the other components in the layout and the dimensions of the render target.
// In horizontal mode, the height is always the two row height, while the width
// is dynamically adjusted based the component's width preference. The width
// preference however only serves as a ratio of how much of the total width to
// distribute to the individual components. So similar to vertical mode, the
// width is fairly dynamic.
//
// ## Default Pixel Space
//
// The default pixel space describes a default scaling factor to apply to the
// component coordinate space. Both the original LiveSplit as well as
// livesplit-core internally use this coordinate space to store the component
// settings that influence dimensions of elements drawn on the component, such
// as font sizes and the dimensions of the component itself. It also serves as a
// good default size when choosing the size of a window or an image when the
// preferred size of the layout is unknown. The factor for converting component
// space coordinates to the default pixel space coordinates is 24.
//
// ### Guidelines for Spacing and Sizes in the Component Coordinate Space
//
// The default height of a component in the component coordinate space is 1.
// This is equal to the height of one split or one key value component. The
// default text size is 0.725. There is a padding of 0.35 to the left and right
// side of a component for the contents shown inside a component, such as images
// and texts. The same padding of 0.35 is also used for the minimum spacing
// between text and other content such as an icon or another text. A vertical
// padding of 10% of the height of the available space is chosen for images
// unless that is larger than the normal padding. If text doesn't fit, it is to
// be either abbreviated or overflown via the use of ellipsis. Numbers and times
// are supposed to be aligned to the right and should be using a monospace text
// layout. Sometimes components are rendered in two row mode. The height of
// these components is 1.725. All components also need to be able to render with
// this height in horizontal mode. Separators have a thickness of 0.1, while
// thin separators have half of this thickness.

mod component;
mod consts;
mod entity;
mod font;
mod icon;
mod resource;
mod scene;

#[cfg(feature = "software-rendering")]
pub mod software;

use self::{
    consts::{
        BOTH_PADDINGS, DEFAULT_TEXT_SIZE, DEFAULT_VERTICAL_WIDTH, PADDING, TEXT_ALIGN_BOTTOM,
        TEXT_ALIGN_TOP, TWO_ROW_HEIGHT,
    },
    font::FontCache,
    icon::Icon,
    resource::Handles,
};
use crate::{
    layout::{LayoutDirection, LayoutState},
    settings::{Color, Gradient},
};
use alloc::borrow::Cow;
use core::iter;
use euclid::{Transform2D, UnknownUnit};

pub use self::{
    entity::Entity,
    resource::{Handle, PathBuilder, ResourceAllocator, SharedOwnership},
    scene::{Layer, Scene},
};

pub use euclid;

/// Describes a coordinate in 2D space.
pub type Pos = [f32; 2];
/// A color encoded as RGBA (red, green, blue, alpha) where each component is
/// stored as a value between 0 and 1.
pub type Rgba = [f32; 4];
/// A transformation matrix to apply to meshes in order to place them into the
/// scene.
pub type Transform = Transform2D<f32, UnknownUnit, UnknownUnit>;

/// Specifies the colors to use when filling a path.
#[derive(Copy, Clone, PartialEq)]
pub enum FillShader {
    /// Use a single color for the whole path.
    SolidColor(Rgba),
    /// Use a vertical gradient (top, bottom) to fill the path.
    VerticalGradient(Rgba, Rgba),
    /// Use a horizontal gradient (left, right) to fill the path.
    HorizontalGradient(Rgba, Rgba),
}

enum CachedSize {
    Vertical(f32),
    Horizontal(f32),
}

/// The scene manager is the main entry point when it comes to writing a
/// renderer for livesplit-core. When provided with a [`LayoutState`], it places
/// [`Entities`](Entity) into a [`Scene`] and updates it accordingly as the
/// [`LayoutState`] changes. It is up to a specific renderer to then take the
/// [`Scene`] and render out the [`Entities`](Entity). There is a
/// [`ResourceAllocator`] trait that defines the types of resources an
/// [`Entity`] consists of. A specific renderer can then provide an
/// implementation that provides the resources it can render out. Those
/// resources are images and paths, i.e. lines, quadratic and cubic bezier
/// curves.
pub struct SceneManager<P, I> {
    scene: Scene<P, I>,
    fonts: FontCache<P>,
    icons: IconCache<I>,
    next_id: usize,
    cached_size: Option<CachedSize>,
}

struct IconCache<I> {
    game_icon: Option<Icon<I>>,
    split_icons: Vec<Option<Icon<I>>>,
    detailed_timer_icon: Option<Icon<I>>,
}

impl<P: SharedOwnership, I: SharedOwnership> SceneManager<P, I> {
    /// Creates a new scene manager.
    pub fn new(mut allocator: impl ResourceAllocator<Path = P, Image = I>) -> Self {
        let mut builder = allocator.path_builder();
        builder.move_to(0.0, 0.0);
        builder.line_to(0.0, 1.0);
        builder.line_to(1.0, 1.0);
        builder.line_to(1.0, 0.0);
        builder.close();
        let rectangle = Handle::new(0, builder.finish(&mut allocator));

        Self {
            fonts: FontCache::new().unwrap(),
            icons: IconCache {
                game_icon: None,
                split_icons: Vec::new(),
                detailed_timer_icon: None,
            },
            // We use 0 for the rectangle.
            next_id: 1,
            scene: Scene::new(rectangle),
            cached_size: None,
        }
    }

    /// Accesses the [`Scene`] in order to render the [`Entities`](Entity).
    pub fn scene(&self) -> &Scene<P, I> {
        &self.scene
    }

    /// Updates the [`Scene`] by updating the [`Entities`](Entity) according to
    /// the [`LayoutState`] provided. The [`ResourceAllocator`] is used to
    /// allocate the resources necessary that the [`Entities`](Entity) use. A
    /// resolution needs to be provided as well so that the [`Entities`](Entity)
    /// are positioned and sized correctly for a renderer to then consume. If a
    /// change in the layout size is detected, a new more suitable resolution
    /// for subsequent updates is being returned. This is however merely a hint
    /// and can be completely ignored.
    pub fn update_scene<A: ResourceAllocator<Path = P, Image = I>>(
        &mut self,
        allocator: A,
        resolution: (f32, f32),
        state: &LayoutState,
    ) -> Option<(f32, f32)> {
        #[cfg(feature = "font-loading")]
        self.fonts.maybe_reload(state);

        self.scene.clear();

        self.scene
            .set_background(decode_gradient(&state.background));

        let new_dimensions = match state.direction {
            LayoutDirection::Vertical => self.render_vertical(allocator, resolution, state),
            LayoutDirection::Horizontal => self.render_horizontal(allocator, resolution, state),
        };

        self.scene.recalculate_if_bottom_layer_changed();

        new_dimensions
    }

    fn render_vertical(
        &mut self,
        allocator: impl ResourceAllocator<Path = P, Image = I>,
        resolution: (f32, f32),
        state: &LayoutState,
    ) -> Option<(f32, f32)> {
        let total_height = component::layout_height(state);

        let cached_total_size = self
            .cached_size
            .get_or_insert(CachedSize::Vertical(total_height));
        let mut new_resolution = None;

        match cached_total_size {
            CachedSize::Vertical(cached_total_height) => {
                if cached_total_height.to_ne_bytes() != total_height.to_ne_bytes() {
                    new_resolution = Some((
                        resolution.0,
                        resolution.1 / *cached_total_height * total_height,
                    ));
                    *cached_total_height = total_height;
                }
            }
            CachedSize::Horizontal(_) => {
                let to_pixels = resolution.1 / TWO_ROW_HEIGHT;
                let new_height = total_height * to_pixels;
                let new_width = DEFAULT_VERTICAL_WIDTH * to_pixels;
                new_resolution = Some((new_width, new_height));
                *cached_total_size = CachedSize::Vertical(total_height);
            }
        }

        let aspect_ratio = resolution.0 as f32 / resolution.1 as f32;

        let mut context = RenderContext {
            handles: Handles::new(self.next_id, allocator),
            transform: Transform::scale(resolution.0 as f32, resolution.1 as f32),
            scene: &mut self.scene,
            fonts: &mut self.fonts,
        };

        // Now we transform the coordinate space to Renderer Coordinate Space by
        // non-uniformly adjusting for the aspect ratio.
        context.scale_non_uniform_x(aspect_ratio.recip());

        // We scale the coordinate space uniformly such that we have the same
        // scaling as the Component Coordinate Space. This also already is the
        // Component Coordinate Space for the component at (0, 0).
        context.scale(total_height.recip());

        // Calculate the width of the components in component space. In vertical
        // mode, all the components have the same width.
        let width = aspect_ratio * total_height;

        for component in &state.components {
            let height = component::height(component);
            let dim = [width, height];
            component::render(&mut context, &mut self.icons, component, state, dim);
            // We translate the coordinate space to the Component Coordinate
            // Space of the next component by shifting by the height of the
            // current component in the Component Coordinate Space.
            context.translate(0.0, height);
        }

        self.next_id = context.handles.into_next_id();

        new_resolution
    }

    fn render_horizontal(
        &mut self,
        allocator: impl ResourceAllocator<Path = P, Image = I>,
        resolution: (f32, f32),
        state: &LayoutState,
    ) -> Option<(f32, f32)> {
        let total_width = component::layout_width(state);

        let cached_total_size = self
            .cached_size
            .get_or_insert(CachedSize::Horizontal(total_width));
        let mut new_resolution = None;

        match cached_total_size {
            CachedSize::Vertical(cached_total_height) => {
                let new_height = resolution.1 * TWO_ROW_HEIGHT / *cached_total_height;
                let new_width = total_width * new_height / TWO_ROW_HEIGHT;
                new_resolution = Some((new_width, new_height));
                *cached_total_size = CachedSize::Horizontal(total_width);
            }
            CachedSize::Horizontal(cached_total_width) => {
                if cached_total_width.to_ne_bytes() != total_width.to_ne_bytes() {
                    new_resolution = Some((
                        resolution.0 / *cached_total_width * total_width,
                        resolution.1,
                    ));
                    *cached_total_width = total_width;
                }
            }
        }

        let aspect_ratio = resolution.0 as f32 / resolution.1 as f32;

        let mut context = RenderContext {
            handles: Handles::new(self.next_id, allocator),
            transform: Transform::scale(resolution.0 as f32, resolution.1 as f32),
            scene: &mut self.scene,
            fonts: &mut self.fonts,
        };

        // Now we transform the coordinate space to Renderer Coordinate Space by
        // non-uniformly adjusting for the aspect ratio.
        context.scale_non_uniform_x(aspect_ratio.recip());

        // We scale the coordinate space uniformly such that we have the same
        // scaling as the Component Coordinate Space. This also already is the
        // Component Coordinate Space for the component at (0, 0). Since all the
        // components use the two row height as their height, we scale by the
        // reciprocal of that.
        context.scale(TWO_ROW_HEIGHT.recip());

        // We don't take the component width we calculate. Instead we use the
        // component width as a ratio of how much of the total actual width to
        // distribute to each of the components. This factor is this adjustment.
        let width_scaling = TWO_ROW_HEIGHT * aspect_ratio / total_width;

        for component in &state.components {
            let width = component::width(component) * width_scaling;
            let height = TWO_ROW_HEIGHT;
            let dim = [width, height];
            component::render(&mut context, &mut self.icons, component, state, dim);
            // We translate the coordinate space to the Component Coordinate
            // Space of the next component by shifting by the width of the
            // current component in the Component Coordinate Space.
            context.translate(width, 0.0);
        }

        self.next_id = context.handles.into_next_id();

        new_resolution
    }
}

struct RenderContext<'b, A: ResourceAllocator> {
    transform: Transform,
    handles: Handles<A>,
    scene: &'b mut Scene<A::Path, A::Image>,
    fonts: &'b mut FontCache<A::Path>,
}

impl<A: ResourceAllocator> RenderContext<'_, A> {
    fn rectangle(&self) -> Handle<A::Path> {
        self.scene.rectangle()
    }

    fn backend_render_rectangle(&mut self, [x1, y1]: Pos, [x2, y2]: Pos, shader: FillShader) {
        let transform = self
            .transform
            .pre_translate([x1, y1].into())
            .pre_scale(x2 - x1, y2 - y1);

        let rectangle = self.rectangle();

        self.scene
            .bottom_layer_mut()
            .push(Entity::FillPath(rectangle, shader, transform));
    }

    fn backend_render_top_rectangle(&mut self, [x1, y1]: Pos, [x2, y2]: Pos, shader: FillShader) {
        let transform = self
            .transform
            .pre_translate([x1, y1].into())
            .pre_scale(x2 - x1, y2 - y1);

        let rectangle = self.rectangle();

        self.scene
            .top_layer_mut()
            .push(Entity::FillPath(rectangle, shader, transform));
    }

    fn top_layer_path(&mut self, path: Handle<A::Path>, color: Color) {
        self.scene
            .top_layer_mut()
            .push(Entity::FillPath(path, solid(&color), self.transform));
    }

    fn top_layer_stroke_path(&mut self, path: Handle<A::Path>, color: Color, stroke_width: f32) {
        self.scene.top_layer_mut().push(Entity::StrokePath(
            path,
            stroke_width,
            color.to_array(),
            self.transform,
        ));
    }

    fn create_icon(&mut self, image_data: &[u8]) -> Option<Icon<A::Image>> {
        if image_data.is_empty() {
            return None;
        }

        let image = image::load_from_memory(image_data).ok()?.to_rgba8();

        Some(Icon {
            aspect_ratio: image.width() as f32 / image.height() as f32,
            image: self
                .handles
                .create_image(image.width(), image.height(), &image),
        })
    }

    fn scale(&mut self, factor: f32) {
        self.transform = self.transform.pre_scale(factor, factor);
    }

    fn scale_non_uniform_x(&mut self, x: f32) {
        self.transform = self.transform.pre_scale(x, 1.0);
    }

    fn translate(&mut self, x: f32, y: f32) {
        self.transform = self.transform.pre_translate([x, y].into());
    }

    fn render_rectangle(&mut self, top_left: Pos, bottom_right: Pos, gradient: &Gradient) {
        if let Some(colors) = decode_gradient(gradient) {
            self.backend_render_rectangle(top_left, bottom_right, colors);
        }
    }

    fn render_top_rectangle(&mut self, top_left: Pos, bottom_right: Pos, gradient: &Gradient) {
        if let Some(colors) = decode_gradient(gradient) {
            self.backend_render_top_rectangle(top_left, bottom_right, colors);
        }
    }

    fn render_icon(
        &mut self,
        [mut x, mut y]: Pos,
        [mut width, mut height]: Pos,
        icon: &Icon<A::Image>,
    ) {
        let box_aspect_ratio = width / height;
        let aspect_ratio_diff = box_aspect_ratio / icon.aspect_ratio;

        if aspect_ratio_diff > 1.0 {
            let new_width = width / aspect_ratio_diff;
            let diff_width = width - new_width;
            x += 0.5 * diff_width;
            width = new_width;
        } else if aspect_ratio_diff < 1.0 {
            let new_height = height * aspect_ratio_diff;
            let diff_height = height - new_height;
            y += 0.5 * diff_height;
            height = new_height;
        }

        let transform = self
            .transform
            .pre_translate([x, y].into())
            .pre_scale(width, height);

        self.scene
            .bottom_layer_mut()
            .push(Entity::Image(icon.image.share(), transform));
    }

    fn render_key_value_component(
        &mut self,
        key: &str,
        abbreviations: &[Cow<'_, str>],
        value: &str,
        updates_frequently: bool,
        [width, height]: [f32; 2],
        key_color: Color,
        value_color: Color,
        display_two_rows: bool,
    ) {
        let left_of_value_x = self.render_numbers(
            value,
            Layer::from_updates_frequently(updates_frequently),
            [width - PADDING, height + TEXT_ALIGN_BOTTOM],
            DEFAULT_TEXT_SIZE,
            solid(&value_color),
        );
        let end_x = if display_two_rows {
            width
        } else {
            left_of_value_x
        };
        let key = self.choose_abbreviation(
            iter::once(key).chain(abbreviations.iter().map(|x| &**x)),
            DEFAULT_TEXT_SIZE,
            end_x - BOTH_PADDINGS,
        );
        self.render_text_ellipsis(
            key,
            [PADDING, TEXT_ALIGN_TOP],
            DEFAULT_TEXT_SIZE,
            solid(&key_color),
            end_x - PADDING,
        );
    }

    fn render_text_ellipsis(
        &mut self,
        text: &str,
        pos: Pos,
        scale: f32,
        shader: FillShader,
        max_x: f32,
    ) -> f32 {
        let mut cursor = font::Cursor::new(pos);

        let mut buffer = self.fonts.buffer.take().unwrap_or_default();
        buffer.push_str(text.trim());
        buffer.guess_segment_properties();

        let font = self.fonts.text.font.scale(scale);
        let glyphs = font.shape(buffer);

        font::render(
            glyphs.left_aligned(&mut cursor, max_x),
            shader,
            &font,
            &mut self.fonts.text.glyph_cache,
            &self.transform,
            &mut self.handles,
            self.scene.bottom_layer_mut(),
        );

        self.fonts.buffer = Some(glyphs.into_buffer());

        cursor.x
    }

    fn render_text_centered(
        &mut self,
        text: &str,
        min_x: f32,
        max_x: f32,
        pos: Pos,
        scale: f32,
        shader: FillShader,
    ) {
        let mut cursor = font::Cursor::new(pos);

        let mut buffer = self.fonts.buffer.take().unwrap_or_default();
        buffer.push_str(text.trim());
        buffer.guess_segment_properties();

        let font = self.fonts.text.font.scale(scale);
        let glyphs = font.shape(buffer);

        font::render(
            glyphs.centered(&mut cursor, min_x, max_x),
            shader,
            &font,
            &mut self.fonts.text.glyph_cache,
            &self.transform,
            &mut self.handles,
            self.scene.bottom_layer_mut(),
        );

        self.fonts.buffer = Some(glyphs.into_buffer());
    }

    fn render_text_right_align(
        &mut self,
        text: &str,
        layer: Layer,
        pos: Pos,
        scale: f32,
        shader: FillShader,
    ) -> f32 {
        let mut cursor = font::Cursor::new(pos);

        let mut buffer = self.fonts.buffer.take().unwrap_or_default();
        buffer.push_str(text.trim());
        buffer.guess_segment_properties();

        let font = self.fonts.text.font.scale(scale);
        let glyphs = font.shape(buffer);

        font::render(
            glyphs.right_aligned(&mut cursor),
            shader,
            &font,
            &mut self.fonts.text.glyph_cache,
            &self.transform,
            &mut self.handles,
            self.scene.layer_mut(layer),
        );

        self.fonts.buffer = Some(glyphs.into_buffer());

        cursor.x
    }

    fn render_text_align(
        &mut self,
        text: &str,
        min_x: f32,
        max_x: f32,
        pos: Pos,
        scale: f32,
        centered: bool,
        shader: FillShader,
    ) {
        if centered {
            self.render_text_centered(text, min_x, max_x, pos, scale, shader);
        } else {
            self.render_text_ellipsis(text, pos, scale, shader, max_x);
        }
    }

    fn render_numbers(
        &mut self,
        text: &str,
        layer: Layer,
        pos: Pos,
        scale: f32,
        shader: FillShader,
    ) -> f32 {
        let mut cursor = font::Cursor::new(pos);

        let mut buffer = self.fonts.buffer.take().unwrap_or_default();
        buffer.push_str(text.trim());
        buffer.guess_segment_properties();

        let font = self.fonts.times.font.scale(scale);
        let glyphs = font.shape_tabular_numbers(buffer);

        font::render(
            glyphs.tabular_numbers(&mut cursor),
            shader,
            &font,
            &mut self.fonts.times.glyph_cache,
            &self.transform,
            &mut self.handles,
            self.scene.layer_mut(layer),
        );

        self.fonts.buffer = Some(glyphs.into_buffer());

        cursor.x
    }

    fn render_timer(
        &mut self,
        text: &str,
        layer: Layer,
        pos: Pos,
        scale: f32,
        shader: FillShader,
    ) -> f32 {
        let mut cursor = font::Cursor::new(pos);

        let mut buffer = self.fonts.buffer.take().unwrap_or_default();
        buffer.push_str(text.trim());
        buffer.guess_segment_properties();

        let font = self.fonts.timer.font.scale(scale);
        let glyphs = font.shape_tabular_numbers(buffer);

        font::render(
            glyphs.tabular_numbers(&mut cursor),
            shader,
            &font,
            &mut self.fonts.timer.glyph_cache,
            &self.transform,
            &mut self.handles,
            self.scene.layer_mut(layer),
        );

        self.fonts.buffer = Some(glyphs.into_buffer());

        cursor.x
    }

    fn choose_abbreviation<'a>(
        &mut self,
        abbreviations: impl IntoIterator<Item = &'a str>,
        font_size: f32,
        max_width: f32,
    ) -> &'a str {
        let mut abbreviations = abbreviations.into_iter();
        let abbreviation = abbreviations.next().unwrap_or("");
        let width = self.measure_text(abbreviation, font_size);
        let (mut total_longest, mut total_longest_width) = (abbreviation, width);
        let (mut within_longest, mut within_longest_width) = if width <= max_width {
            (abbreviation, width)
        } else {
            ("", 0.0)
        };

        for abbreviation in abbreviations {
            let width = self.measure_text(abbreviation, font_size);
            if width <= max_width && width > within_longest_width {
                within_longest_width = width;
                within_longest = abbreviation;
            }
            if width > total_longest_width {
                total_longest_width = width;
                total_longest = abbreviation;
            }
        }

        if within_longest.is_empty() {
            total_longest
        } else {
            within_longest
        }
    }

    fn measure_text(&mut self, text: &str, scale: f32) -> f32 {
        let mut buffer = self.fonts.buffer.take().unwrap_or_default();
        buffer.push_str(text.trim());
        buffer.guess_segment_properties();

        let glyphs = self.fonts.text.font.scale(scale).shape(buffer);
        let width = glyphs.width();

        self.fonts.buffer = Some(glyphs.into_buffer());

        width
    }

    fn measure_numbers(&mut self, text: &str, scale: f32) -> f32 {
        let mut cursor = font::Cursor::new([0.0; 2]);

        let mut buffer = self.fonts.buffer.take().unwrap_or_default();
        buffer.push_str(text.trim());
        buffer.guess_segment_properties();

        let glyphs = self
            .fonts
            .times
            .font
            .scale(scale)
            .shape_tabular_numbers(buffer);

        // Iterate over all glyphs, to move the cursor forward.
        glyphs.tabular_numbers(&mut cursor).for_each(drop);

        // Wherever we end up is our width.
        let width = -cursor.x;

        self.fonts.buffer = Some(glyphs.into_buffer());

        width
    }
}

const fn decode_gradient(gradient: &Gradient) -> Option<FillShader> {
    Some(match gradient {
        Gradient::Transparent => return None,
        Gradient::Horizontal(left, right) => {
            FillShader::HorizontalGradient(left.to_array(), right.to_array())
        }
        Gradient::Vertical(top, bottom) => {
            FillShader::VerticalGradient(top.to_array(), bottom.to_array())
        }
        Gradient::Plain(plain) => FillShader::SolidColor(plain.to_array()),
    })
}

const fn solid(color: &Color) -> FillShader {
    FillShader::SolidColor(color.to_array())
}
