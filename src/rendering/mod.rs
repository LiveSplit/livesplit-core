//! The rendering module provides a [`SceneManager`], that when provided with a
//! [`LayoutState`], places [`Entities`](Entity) into a [`Scene`] and updates it
//! accordingly as the [`LayoutState`] changes. It is up to a specific renderer
//! to then take the [`Scene`] and render out the [`Entities`](Entity). There is
//! a [`ResourceAllocator`] trait that defines the types of resources an
//! [`Entity`] consists of. A specific renderer can then provide an
//! implementation that provides the resources it can render out. Those
//! resources are images, paths, i.e. lines, quadratic and cubic bezier curves,
//! fonts and labels. An optional software renderer is available behind the
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

#[cfg(feature = "path-based-text-engine")]
pub mod path_based_text_engine;

#[cfg(feature = "software-rendering")]
pub mod software;

use self::{
    consts::{
        DEFAULT_TEXT_SIZE, DEFAULT_VERTICAL_WIDTH, PADDING, TEXT_ALIGN_BOTTOM, TEXT_ALIGN_TOP,
        TWO_ROW_HEIGHT,
    },
    font::{AbbreviatedLabel, CachedLabel, FontCache},
    icon::Icon,
    resource::Handles,
};
use crate::{
    layout::{LayoutDirection, LayoutState},
    platform::prelude::*,
    settings::{Color, Gradient},
};
use alloc::borrow::Cow;
use bytemuck::{Pod, Zeroable};
use core::iter;

pub use self::{
    entity::Entity,
    font::{TEXT_FONT, TIMER_FONT},
    resource::{
        FontKind, Handle, Label, LabelHandle, PathBuilder, ResourceAllocator, SharedOwnership,
    },
    scene::{Layer, Scene},
};

/// Describes a coordinate in 2D space.
pub type Pos = [f32; 2];
/// A color encoded as RGBA (red, green, blue, alpha) where each component is
/// stored as a value between 0 and 1.
pub type Rgba = [f32; 4];

/// A transformation to apply to the entities in order to place them into the
/// scene.
#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Transform {
    /// Scale the x coordinate by this value.
    pub scale_x: f32,
    /// Scale the y coordinate by this value.
    pub scale_y: f32,
    /// Add this value to the x coordinate after scaling it.
    pub x: f32,
    /// Add this value to the y coordinate after scaling it.
    pub y: f32,
}

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
/// resources are images, paths, i.e. lines, quadratic and cubic bezier
/// curves, fonts and labels.
pub struct SceneManager<P, I, F, L> {
    scene: Scene<P, I, L>,
    components: Vec<component::Cache<I, L>>,
    next_id: usize,
    cached_size: Option<CachedSize>,
    fonts: FontCache<F>,
}

impl<P: SharedOwnership, I: SharedOwnership, F, L: SharedOwnership> SceneManager<P, I, F, L> {
    /// Creates a new scene manager.
    pub fn new(
        mut allocator: impl ResourceAllocator<Path = P, Image = I, Font = F, Label = L>,
    ) -> Self {
        let mut builder = allocator.path_builder();
        builder.move_to(0.0, 0.0);
        builder.line_to(0.0, 1.0);
        builder.line_to(1.0, 1.0);
        builder.line_to(1.0, 0.0);
        builder.close();
        let rectangle = Handle::new(0, builder.finish());

        let mut handles = Handles::new(1, allocator);
        let fonts = FontCache::new(&mut handles);

        Self {
            components: Vec::new(),
            next_id: handles.into_next_id(),
            scene: Scene::new(rectangle),
            cached_size: None,
            fonts,
        }
    }

    /// Accesses the [`Scene`] in order to render the [`Entities`](Entity).
    pub const fn scene(&self) -> &Scene<P, I, L> {
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
    pub fn update_scene<A: ResourceAllocator<Path = P, Image = I, Font = F, Label = L>>(
        &mut self,
        allocator: A,
        resolution: (f32, f32),
        state: &LayoutState,
    ) -> Option<(f32, f32)> {
        self.scene.clear();

        self.scene
            .set_background(decode_gradient(&state.background));

        // Ensure we have exactly as many cached components as the layout state.
        if let Some(new_components) = state.components.get(self.components.len()..) {
            self.components
                .extend(new_components.iter().map(component::Cache::new));
        } else {
            self.components.truncate(state.components.len());
        }

        let new_dimensions = match state.direction {
            LayoutDirection::Vertical => self.render_vertical(allocator, resolution, state),
            LayoutDirection::Horizontal => self.render_horizontal(allocator, resolution, state),
        };

        self.scene.recalculate_if_bottom_layer_changed();

        new_dimensions
    }

    fn render_vertical(
        &mut self,
        allocator: impl ResourceAllocator<Path = P, Image = I, Font = F, Label = L>,
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
                if cached_total_height.to_bits() != total_height.to_bits() {
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

        let aspect_ratio = resolution.0 / resolution.1;

        let mut context = RenderContext {
            handles: Handles::new(self.next_id, allocator),
            transform: Transform::scale(resolution.0, resolution.1),
            scene: &mut self.scene,
            fonts: &mut self.fonts,
        };

        context.fonts.maybe_reload(&mut context.handles, state);

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

        for (component, cache) in state.components.iter().zip(&mut self.components) {
            let height = component::height(component);
            let dim = [width, height];
            component::render(cache, &mut context, component, state, dim);
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
        allocator: impl ResourceAllocator<Path = P, Image = I, Font = F, Label = L>,
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
                if cached_total_width.to_bits() != total_width.to_bits() {
                    new_resolution = Some((
                        resolution.0 / *cached_total_width * total_width,
                        resolution.1,
                    ));
                    *cached_total_width = total_width;
                }
            }
        }

        let aspect_ratio = resolution.0 / resolution.1;

        let mut context = RenderContext {
            handles: Handles::new(self.next_id, allocator),
            transform: Transform::scale(resolution.0, resolution.1),
            scene: &mut self.scene,
            fonts: &mut self.fonts,
        };

        context.fonts.maybe_reload(&mut context.handles, state);

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

        for (component, cache) in state.components.iter().zip(&mut self.components) {
            let width = component::width(component) * width_scaling;
            let height = TWO_ROW_HEIGHT;
            let dim = [width, height];
            component::render(cache, &mut context, component, state, dim);
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
    scene: &'b mut Scene<A::Path, A::Image, A::Label>,
    fonts: &'b mut FontCache<A::Font>,
}

impl<A: ResourceAllocator> RenderContext<'_, A> {
    fn rectangle(&self) -> Handle<A::Path> {
        self.scene.rectangle()
    }

    fn render_background(&mut self, [w, h]: Pos, gradient: &Gradient) {
        if let Some(shader) = decode_gradient(gradient) {
            let rectangle = self.rectangle();
            self.scene.bottom_layer_mut().push(Entity::FillPath(
                rectangle,
                shader,
                self.transform.pre_scale(w, h),
            ));
        }
    }

    fn backend_render_rectangle(&mut self, [x1, y1]: Pos, [x2, y2]: Pos, shader: FillShader) {
        let transform = self
            .transform
            .pre_translate(x1, y1)
            .pre_scale(x2 - x1, y2 - y1);

        let rectangle = self.rectangle();

        self.scene
            .bottom_layer_mut()
            .push(Entity::FillPath(rectangle, shader, transform));
    }

    fn backend_render_top_rectangle(&mut self, [x1, y1]: Pos, [x2, y2]: Pos, shader: FillShader) {
        let transform = self
            .transform
            .pre_translate(x1, y1)
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
        let (image, aspect_ratio) = self.handles.create_image(image_data)?;
        Some(Icon {
            aspect_ratio,
            image,
        })
    }

    fn scale(&mut self, factor: f32) {
        self.transform = self.transform.pre_scale(factor, factor);
    }

    fn scale_non_uniform_x(&mut self, x: f32) {
        self.transform = self.transform.pre_scale(x, 1.0);
    }

    fn translate(&mut self, x: f32, y: f32) {
        self.transform = self.transform.pre_translate(x, y);
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

        let transform = self.transform.pre_translate(x, y).pre_scale(width, height);

        self.scene
            .bottom_layer_mut()
            .push(Entity::Image(icon.image.share(), transform));
    }

    fn render_key_value_component(
        &mut self,
        key: &str,
        abbreviations: &[Cow<'_, str>],
        key_label: &mut AbbreviatedLabel<A::Label>,
        value: &str,
        value_label: &mut CachedLabel<A::Label>,
        updates_frequently: bool,
        [width, height]: [f32; 2],
        key_color: Color,
        value_color: Color,
        display_two_rows: bool,
    ) {
        let left_of_value_x = self.render_numbers(
            value,
            value_label,
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

        self.render_abbreviated_text_ellipsis(
            iter::once(key).chain(abbreviations.iter().map(|x| &**x)),
            key_label,
            [PADDING, TEXT_ALIGN_TOP],
            DEFAULT_TEXT_SIZE,
            solid(&key_color),
            end_x - PADDING,
        );
    }

    fn render_abbreviated_text_ellipsis<'a>(
        &mut self,
        abbreviations: impl IntoIterator<Item = &'a str> + Clone,
        label: &mut AbbreviatedLabel<A::Label>,
        pos @ [x, _]: Pos,
        scale: f32,
        shader: FillShader,
        max_x: f32,
    ) -> f32 {
        let label = label.update(
            abbreviations,
            &mut self.handles,
            &mut self.fonts.text.font,
            (max_x - x) / scale,
        );

        self.scene.bottom_layer_mut().push(Entity::Label(
            label.share(),
            shader,
            font::left_aligned(&self.transform, pos, scale),
        ));

        x + label.width(scale)
    }

    fn render_text_ellipsis(
        &mut self,
        text: &str,
        label: &mut CachedLabel<A::Label>,
        pos @ [x, _]: Pos,
        scale: f32,
        shader: FillShader,
        max_x: f32,
    ) -> f32 {
        let label = label.update(
            text,
            &mut self.handles,
            &mut self.fonts.text.font,
            Some((max_x - x) / scale),
        );

        self.scene.bottom_layer_mut().push(Entity::Label(
            label.share(),
            shader,
            font::left_aligned(&self.transform, pos, scale),
        ));

        x + label.width(scale)
    }

    fn render_text_centered(
        &mut self,
        text: &str,
        label: &mut CachedLabel<A::Label>,
        min_x: f32,
        max_x: f32,
        pos: Pos,
        scale: f32,
        shader: FillShader,
    ) {
        let label = label.update(
            text,
            &mut self.handles,
            &mut self.fonts.text.font,
            Some((max_x - min_x) / scale),
        );

        self.scene.bottom_layer_mut().push(Entity::Label(
            label.share(),
            shader,
            font::centered(
                &self.transform,
                pos,
                scale,
                label.width(scale),
                min_x,
                max_x,
            ),
        ));
    }

    fn render_abbreviated_text_centered<'a>(
        &mut self,
        abbreviations: impl IntoIterator<Item = &'a str> + Clone,
        label: &mut AbbreviatedLabel<A::Label>,
        min_x: f32,
        max_x: f32,
        pos: Pos,
        scale: f32,
        shader: FillShader,
    ) {
        let label = label.update(
            abbreviations,
            &mut self.handles,
            &mut self.fonts.text.font,
            (max_x - min_x) / scale,
        );

        self.scene.bottom_layer_mut().push(Entity::Label(
            label.share(),
            shader,
            font::centered(
                &self.transform,
                pos,
                scale,
                label.width(scale),
                min_x,
                max_x,
            ),
        ));
    }

    fn render_text_right_align(
        &mut self,
        text: &str,
        label: &mut CachedLabel<A::Label>,
        layer: Layer,
        pos @ [x, _]: Pos,
        scale: f32,
        shader: FillShader,
    ) -> f32 {
        let label = label.update(text, &mut self.handles, &mut self.fonts.text.font, None);
        let width = label.width(scale);

        self.scene.layer_mut(layer).push(Entity::Label(
            label.share(),
            shader,
            font::right_aligned(&self.transform, pos, scale, width),
        ));

        x - width
    }

    fn render_abbreviated_text_align<'a>(
        &mut self,
        abbreviations: impl IntoIterator<Item = &'a str> + Clone,
        label: &mut AbbreviatedLabel<A::Label>,
        min_x: f32,
        max_x: f32,
        pos: Pos,
        scale: f32,
        centered: bool,
        shader: FillShader,
    ) {
        if centered {
            self.render_abbreviated_text_centered(
                abbreviations,
                label,
                min_x,
                max_x,
                pos,
                scale,
                shader,
            );
        } else {
            self.render_abbreviated_text_ellipsis(abbreviations, label, pos, scale, shader, max_x);
        }
    }

    fn render_numbers(
        &mut self,
        text: &str,
        label: &mut CachedLabel<A::Label>,
        layer: Layer,
        pos @ [x, _]: Pos,
        scale: f32,
        shader: FillShader,
    ) -> f32 {
        let label = label.update(text, &mut self.handles, &mut self.fonts.times.font, None);
        let width = label.width(scale);

        self.scene.layer_mut(layer).push(Entity::Label(
            label.share(),
            shader,
            font::right_aligned(&self.transform, pos, scale, width),
        ));

        x - width
    }

    fn render_timer(
        &mut self,
        text: &str,
        label: &mut CachedLabel<A::Label>,
        layer: Layer,
        pos @ [x, _]: Pos,
        scale: f32,
        shader: FillShader,
    ) -> f32 {
        let label = label.update(text, &mut self.handles, &mut self.fonts.timer.font, None);
        let width = label.width(scale);

        self.scene.layer_mut(layer).push(Entity::Label(
            label.share(),
            shader,
            font::right_aligned(&self.transform, pos, scale, width),
        ));

        x - width
    }

    fn measure_numbers(
        &mut self,
        text: &str,
        label: &mut CachedLabel<A::Label>,
        scale: f32,
    ) -> f32 {
        let label = label.update(text, &mut self.handles, &mut self.fonts.times.font, None);
        label.width(scale)
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

impl Transform {
    const fn scale(scale_x: f32, scale_y: f32) -> Transform {
        Self {
            x: 0.0,
            y: 0.0,
            scale_x,
            scale_y,
        }
    }

    /// Returns a new transform that first scales the coordinates.
    pub fn pre_scale(&self, scale_x: f32, scale_y: f32) -> Transform {
        Self {
            scale_x: self.scale_x * scale_x,
            scale_y: self.scale_y * scale_y,
            x: self.x,
            y: self.y,
        }
    }

    /// Returns a new transform that first translates the coordinates.
    pub fn pre_translate(&self, x: f32, y: f32) -> Transform {
        Self {
            scale_x: self.scale_x,
            scale_y: self.scale_y,
            x: self.x + self.scale_x * x,
            y: self.y + self.scale_y * y,
        }
    }

    #[cfg(feature = "software-rendering")]
    fn transform_y(&self, y: f32) -> f32 {
        self.y + self.scale_y * y
    }
}
