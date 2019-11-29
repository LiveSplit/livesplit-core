//! The rendering module provides a renderer for layout states that is
//! abstracted over different backends so that it can be used with OpenGL,
//! DirectX, Vulkan, Metal, WebGL or any other rendering framework. An optional
//! software renderer is available behind the `software-rendering` feature.
//! While it is slower than using a proper GPU backend, it might be sufficient
//! for situations where you want to create a screenshot of the layout.

// # Coordinate spaces used in this module
//
// ## Backend Coordinate Space
//
// The backend has its own coordinate space that ranges from 0 to 1 across both
// dimensions. (0, 0) is the top left corner of the rendered layout and (1, 1)
// is the bottom right corner. Since the coordinate space forms a square, the
// aspect ratio of the layout is not respected.
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
// default text size is 0.8. There is a padding of 0.35 to the left and right
// side of a component for the contents shown inside a component, such as images
// and texts. The same padding of 0.35 is also used for the minimum spacing
// between text and other content such as an icon or another text. A vertical
// padding of 10% of the height of the available space is chosen unless that is
// larger than the normal padding. If text doesn't fit, it is to be either
// abbreviated or overflown via the use of ellipsis. Numbers and times are
// supposed to be aligned to the right and should be using a monospace text
// layout. Sometimes components are rendered in two row mode. The height of
// these components is 1.8. All components also need to be able to render with
// this height in horizontal mode. Separators have a thickness of 0.1, while
// thin separators have half of this thickness.

mod component;
mod font;
mod glyph_cache;
mod icon;
mod mesh;

#[cfg(feature = "software-rendering")]
pub mod software;

use {
    self::{glyph_cache::GlyphCache, icon::Icon},
    crate::{
        layout::{ComponentState, LayoutDirection, LayoutState},
        settings::{Color, Gradient},
    },
    euclid::{Transform2D, UnknownUnit},
    rusttype::Font,
    std::iter,
};

pub use self::mesh::{Mesh, Vertex};
pub use euclid;

/// The default font to be used for general text. The font is encoded as TTF.
pub const TEXT_FONT: &[u8] = include_bytes!("fonts/FiraSans-Regular.ttf");
/// The default font to be used for timers. The font is encoded as TTF.
pub const TIMER_FONT: &[u8] = include_bytes!("fonts/Timer.ttf");

/// Describes a coordinate in 2D space.
pub type Pos = [f32; 2];
/// A color encoded as RGBA (red, green, blue, alpha) where each component is
/// stored as a value between 0 and 1.
pub type Rgba = [f32; 4];
/// A transformation matrix to apply to meshes in order to place them into the
/// scene.
pub type Transform = Transform2D<f32, UnknownUnit, UnknownUnit>;

const PADDING: f32 = 0.35;
const BOTH_PADDINGS: f32 = 2.0 * PADDING;
const BOTH_VERTICAL_PADDINGS: f32 = DEFAULT_COMPONENT_HEIGHT - DEFAULT_TEXT_SIZE;
const VERTICAL_PADDING: f32 = BOTH_VERTICAL_PADDINGS / 2.0;
const DEFAULT_COMPONENT_HEIGHT: f32 = 1.0;
const TWO_ROW_HEIGHT: f32 = 2.0 * DEFAULT_TEXT_SIZE + BOTH_VERTICAL_PADDINGS;
const DEFAULT_TEXT_SIZE: f32 = 0.8;
const DEFAULT_TEXT_ASCENT: f32 = 0.6;
const DEFAULT_TEXT_DESCENT: f32 = DEFAULT_TEXT_SIZE - DEFAULT_TEXT_ASCENT;
const TEXT_ALIGN_TOP: f32 = VERTICAL_PADDING + DEFAULT_TEXT_ASCENT;
const TEXT_ALIGN_BOTTOM: f32 = -(VERTICAL_PADDING + DEFAULT_TEXT_DESCENT);
const TEXT_ALIGN_CENTER: f32 = DEFAULT_TEXT_ASCENT - DEFAULT_TEXT_SIZE / 2.0;
const SEPARATOR_THICKNESS: f32 = 0.1;
const THIN_SEPARATOR_THICKNESS: f32 = SEPARATOR_THICKNESS / 2.0;
const PSEUDO_PIXELS: f32 = 1.0 / 24.0;
const DEFAULT_VERTICAL_WIDTH: f32 = 11.5;

fn vertical_padding(height: f32) -> f32 {
    (VERTICAL_PADDING * height).min(PADDING)
}

/// The rendering backend for the Renderer is abstracted out into the Backend
/// trait such that the rendering isn't tied to a specific rendering framework.
pub trait Backend {
    /// The type the backend uses for meshes.
    type Mesh;
    /// The type the backend uses for textures.
    type Texture;

    /// Instructs the backend to create a mesh. The mesh consists out of a
    /// vertex buffer and an index buffer that describes pairs of three indices
    /// of the vertex buffer that form a triangle each.
    fn create_mesh(&mut self, mesh: &Mesh) -> Self::Mesh;

    /// Instructs the backend to render out a mesh. The rendering uses no
    /// backface culling or depth buffering. The colors are supposed to be alpha
    /// blended and don't use sRGB. The transform represents a transformation
    /// matrix to be applied to the mesh's vertices in order to place it in the
    /// scene. The scene's coordinates are within 0..1 for both x (left..right)
    /// and y (up..down). There may be a texture that needs to be applied to the
    /// mesh based on its u and v texture coordinates. There also are four
    /// colors for are interpolated between based on the u and v texture
    /// coordinates. The colors are positioned in UV texture space in the
    /// following way:
    /// ```rust,ignore
    /// [
    ///     (0, 0), // Top Left
    ///     (1, 0), // Top Right
    ///     (1, 1), // Bottom Right
    ///     (0, 1), // Bottom Left
    /// ]
    /// ```
    fn render_mesh(
        &mut self,
        mesh: &Self::Mesh,
        transform: Transform,
        colors: [Rgba; 4],
        texture: Option<&Self::Texture>,
    );

    /// Instructs the backend to free a mesh as it is not needed anymore.
    fn free_mesh(&mut self, mesh: Self::Mesh);

    /// Instructs the backend to create a texture out of the texture data
    /// provided. The texture's resolution is provided as well. The data is an
    /// array of chunks of RGBA8 encoded pixels (red, green, blue, alpha with
    /// each channel being an u8).
    fn create_texture(&mut self, width: u32, height: u32, data: &[u8]) -> Self::Texture;

    /// Instructs the backend to free a texture as it is not needed anymore.
    fn free_texture(&mut self, texture: Self::Texture);

    /// Instructs the backend to resize the size of the render target.
    fn resize(&mut self, width: f32, height: f32);
}

enum CachedSize {
    Vertical(f32),
    Horizontal(f32),
}

/// A renderer can be used to render out layout states with the backend chosen.
pub struct Renderer<M, T> {
    text_font: Font<'static>,
    text_glyph_cache: GlyphCache<M>,
    timer_font: Font<'static>,
    timer_glyph_cache: GlyphCache<M>,
    rectangle: Option<M>,
    cached_size: Option<CachedSize>,
    icons: IconCache<T>,
}

struct IconCache<T> {
    game_icon: Option<Icon<T>>,
    split_icons: Vec<Option<Icon<T>>>,
    detailed_timer_icon: Option<Icon<T>>,
}

impl<M, T> Default for Renderer<M, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M, T> Renderer<M, T> {
    /// Creates a new renderer.
    pub fn new() -> Self {
        Self {
            timer_font: Font::from_bytes(TIMER_FONT).unwrap(),
            timer_glyph_cache: GlyphCache::new(),
            text_font: Font::from_bytes(TEXT_FONT).unwrap(),
            text_glyph_cache: GlyphCache::new(),
            rectangle: None,
            icons: IconCache {
                game_icon: None,
                split_icons: Vec::new(),
                detailed_timer_icon: None,
            },
            cached_size: None,
        }
    }

    /// Renders the layout state with the backend provided. A resolution needs
    /// to be provided as well so that the contents are rendered according to
    /// aspect ratio of the render target.
    pub fn render<B: Backend<Mesh = M, Texture = T>>(
        &mut self,
        backend: &mut B,
        resolution: (f32, f32),
        state: &LayoutState,
    ) {
        match state.direction {
            LayoutDirection::Vertical => self.render_vertical(backend, resolution, state),
            LayoutDirection::Horizontal => self.render_horizontal(backend, resolution, state),
        }
    }

    fn render_vertical<B: Backend<Mesh = M, Texture = T>>(
        &mut self,
        backend: &mut B,
        resolution: (f32, f32),
        state: &LayoutState,
    ) {
        let total_height = state.components.iter().map(component_height).sum::<f32>();

        let cached_total_size = self
            .cached_size
            .get_or_insert(CachedSize::Vertical(total_height));
        match cached_total_size {
            CachedSize::Vertical(cached_total_height) => {
                if *cached_total_height != total_height {
                    backend.resize(
                        resolution.0,
                        resolution.1 / *cached_total_height * total_height,
                    );
                    *cached_total_height = total_height;
                }
            }
            CachedSize::Horizontal(_) => {
                let to_pixels = resolution.1 / TWO_ROW_HEIGHT;
                let new_height = total_height * to_pixels;
                let new_width = DEFAULT_VERTICAL_WIDTH * to_pixels;
                backend.resize(new_width, new_height);
                *cached_total_size = CachedSize::Vertical(total_height);
            }
        }

        let aspect_ratio = resolution.0 as f32 / resolution.1 as f32;

        let mut context = RenderContext {
            backend,
            transform: Transform::identity(),
            rectangle: &mut self.rectangle,
            timer_font: &mut self.timer_font,
            timer_glyph_cache: &mut self.timer_glyph_cache,
            text_font: &mut self.text_font,
            text_glyph_cache: &mut self.text_glyph_cache,
        };

        // Initially we are in Backend Coordinate Space.
        // We can render the background here from (0, 0) to (1, 1) as we just
        // want to fill all of the background. We don't need to know anything
        // about the aspect ratio or specific sizes.
        context.render_background(&state.background);

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
            let height = component_height(component);
            let dim = [width, height];
            render_component(&mut context, &mut self.icons, component, state, dim);
            // We translate the coordinate space to the Component Coordinate
            // Space of the next component by shifting by the height of the
            // current component in the Component Coordinate Space.
            context.translate(0.0, height);
        }
    }

    fn render_horizontal<B: Backend<Mesh = M, Texture = T>>(
        &mut self,
        backend: &mut B,
        resolution: (f32, f32),
        state: &LayoutState,
    ) {
        let total_width = state.components.iter().map(component_width).sum::<f32>();

        let cached_total_size = self
            .cached_size
            .get_or_insert(CachedSize::Horizontal(total_width));
        match cached_total_size {
            CachedSize::Vertical(cached_total_height) => {
                let new_height = resolution.1 * TWO_ROW_HEIGHT / *cached_total_height;
                let new_width = total_width * new_height / TWO_ROW_HEIGHT;
                backend.resize(new_width, new_height);
                *cached_total_size = CachedSize::Horizontal(total_width);
            }
            CachedSize::Horizontal(cached_total_width) => {
                if *cached_total_width != total_width {
                    backend.resize(
                        resolution.0 / *cached_total_width * total_width,
                        resolution.1,
                    );
                    *cached_total_width = total_width;
                }
            }
        }

        let aspect_ratio = resolution.0 as f32 / resolution.1 as f32;

        let mut context = RenderContext {
            backend,
            transform: Transform::identity(),
            rectangle: &mut self.rectangle,
            timer_font: &mut self.timer_font,
            timer_glyph_cache: &mut self.timer_glyph_cache,
            text_font: &mut self.text_font,
            text_glyph_cache: &mut self.text_glyph_cache,
        };

        // Initially we are in Backend Coordinate Space.
        // We can render the background here from (0, 0) to (1, 1) as we just
        // want to fill all of the background. We don't need to know anything
        // about the aspect ratio or specific sizes.
        context.render_background(&state.background);

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
            let width = component_width(component) * width_scaling;
            let height = TWO_ROW_HEIGHT;
            let dim = [width, height];
            render_component(&mut context, &mut self.icons, component, state, dim);
            // We translate the coordinate space to the Component Coordinate
            // Space of the next component by shifting by the width of the
            // current component in the Component Coordinate Space.
            context.translate(width, 0.0);
        }
    }
}

fn render_component<B: Backend>(
    context: &mut RenderContext<'_, B>,
    icons: &mut IconCache<B::Texture>,
    component: &ComponentState,
    state: &LayoutState,
    dim: [f32; 2],
) {
    match component {
        ComponentState::BlankSpace(state) => component::blank_space::render(context, dim, state),
        ComponentState::DetailedTimer(component) => component::detailed_timer::render(
            context,
            dim,
            component,
            state,
            &mut icons.detailed_timer_icon,
        ),
        ComponentState::Graph(component) => {
            component::graph::render(context, dim, component, state)
        }
        ComponentState::KeyValue(component) => {
            component::key_value::render(context, dim, component, state)
        }
        ComponentState::Separator(component) => {
            component::separator::render(context, dim, component, state)
        }
        ComponentState::Splits(component) => {
            component::splits::render(context, dim, component, state, &mut icons.split_icons)
        }
        ComponentState::Text(component) => component::text::render(context, dim, component, state),
        ComponentState::Timer(component) => {
            component::timer::render(context, dim, component);
        }
        ComponentState::Title(component) => {
            component::title::render(context, dim, component, state, &mut icons.game_icon)
        }
    }
}

struct RenderContext<'b, B: Backend> {
    transform: Transform,
    backend: &'b mut B,
    rectangle: &'b mut Option<B::Mesh>,
    timer_font: &'b mut Font<'static>,
    timer_glyph_cache: &'b mut GlyphCache<B::Mesh>,
    text_font: &'b mut Font<'static>,
    text_glyph_cache: &'b mut GlyphCache<B::Mesh>,
}

impl<B: Backend> RenderContext<'_, B> {
    fn backend_render_rectangle(&mut self, [x1, y1]: Pos, [x2, y2]: Pos, colors: [Rgba; 4]) {
        let transform = self
            .transform
            .pre_translate([x1, y1].into())
            .pre_scale(x2 - x1, y2 - y1);

        let rectangle = self.rectangle.get_or_insert_with({
            let backend = &mut self.backend;
            move || backend.create_mesh(&mesh::rectangle())
        });

        self.backend.render_mesh(rectangle, transform, colors, None);
    }

    fn create_mesh(&mut self, mesh: &Mesh) -> B::Mesh {
        self.backend.create_mesh(mesh)
    }

    fn render_mesh(&mut self, mesh: &B::Mesh, color: Color) {
        self.backend
            .render_mesh(mesh, self.transform, [decode_color(&color); 4], None)
    }

    fn create_icon(&mut self, image_data: &[u8]) -> Option<Icon<B::Texture>> {
        if image_data.is_empty() {
            return None;
        }

        let image = image::load_from_memory(image_data).ok()?.to_rgba();
        let texture = self
            .backend
            .create_texture(image.width(), image.height(), &image);

        Some(Icon {
            texture,
            aspect_ratio: image.width() as f32 / image.height() as f32,
        })
    }

    fn free_mesh(&mut self, mesh: B::Mesh) {
        self.backend.free_mesh(mesh)
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

    fn render_icon(
        &mut self,
        [mut x, mut y]: Pos,
        [mut width, mut height]: Pos,
        icon: &Icon<B::Texture>,
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

        let rectangle = self.rectangle.get_or_insert_with({
            let backend = &mut self.backend;
            move || backend.create_mesh(&mesh::rectangle())
        });

        self.backend
            .render_mesh(rectangle, transform, [[1.0; 4]; 4], Some(&icon.texture));
    }

    fn render_background(&mut self, background: &Gradient) {
        self.render_rectangle([0.0, 0.0], [1.0, 1.0], background);
    }

    fn render_key_value_component(
        &mut self,
        key: &str,
        abbreviations: &[Box<str>],
        value: &str,
        [width, height]: [f32; 2],
        key_color: Color,
        value_color: Color,
        display_two_rows: bool,
    ) {
        let left_of_value_x = self.render_numbers(
            value,
            [width - PADDING, height + TEXT_ALIGN_BOTTOM],
            DEFAULT_TEXT_SIZE,
            [value_color; 2],
        );
        let end_x = if display_two_rows {
            width
        } else {
            left_of_value_x
        };
        let key = self.choose_abbreviation(
            iter::once(key).chain(abbreviations.iter().map(|abrv| &**abrv)),
            DEFAULT_TEXT_SIZE,
            end_x - BOTH_PADDINGS,
        );
        self.render_text_ellipsis(
            key,
            [PADDING, TEXT_ALIGN_TOP],
            DEFAULT_TEXT_SIZE,
            [key_color; 2],
            end_x - PADDING,
        );
    }

    fn render_text_ellipsis(
        &mut self,
        text: &str,
        pos: Pos,
        scale: f32,
        colors: [Color; 2],
        max_x: f32,
    ) -> f32 {
        let font = font::scaled(&self.text_font, scale);
        font::render(
            font::ellipsis(font::default_layout(font, text, pos), max_x, font),
            colors,
            &self.text_font,
            self.text_glyph_cache,
            &self.transform,
            self.backend,
        )
        .map_or(pos[0], |g| {
            g.position().x + g.unpositioned().h_metrics().advance_width
        })
    }

    fn render_text_centered(
        &mut self,
        text: &str,
        min_x: f32,
        max_x: f32,
        pos: Pos,
        scale: f32,
        color: Color,
    ) {
        let font = font::scaled(&self.text_font, scale);
        font::render(
            font::ellipsis(
                font::centered(font::default_layout(font, text.trim(), pos), min_x),
                max_x,
                font,
            ),
            [color; 2],
            &self.text_font,
            self.text_glyph_cache,
            &self.transform,
            self.backend,
        );
    }

    fn render_text_right_align(
        &mut self,
        text: &str,
        pos: Pos,
        scale: f32,
        colors: [Color; 2],
    ) -> f32 {
        let (layout, width) = font::align_right_and_measure(font::default_layout(
            font::scaled(&self.text_font, scale),
            text.trim(),
            pos,
        ));

        font::render(
            layout,
            colors,
            &self.text_font,
            self.text_glyph_cache,
            &self.transform,
            self.backend,
        );

        pos[0] - width
    }

    /// 0 = left, 0.5 = center, 1 = right
    fn render_text_align(
        &mut self,
        text: &str,
        min_x: f32,
        max_x: f32,
        pos: Pos,
        scale: f32,
        align: f32,
        color: Color,
    ) {
        let font = font::scaled(&self.text_font, scale);
        font::render(
            font::ellipsis(
                font::dynamic_align(font::default_layout(font, text.trim(), pos), align, min_x),
                max_x,
                font,
            ),
            [color; 2],
            &self.text_font,
            self.text_glyph_cache,
            &self.transform,
            self.backend,
        );
    }

    fn render_numbers(&mut self, text: &str, pos: Pos, scale: f32, colors: [Color; 2]) -> f32 {
        font::render(
            font::layout_numbers(font::scaled(&self.text_font, scale), text.trim(), pos),
            colors,
            &self.text_font,
            self.text_glyph_cache,
            &self.transform,
            self.backend,
        )
        .map_or(pos[0], |g| g.position().x)
    }

    fn render_timer(&mut self, text: &str, pos: Pos, scale: f32, colors: [Color; 2]) -> f32 {
        font::render(
            font::layout_numbers(font::scaled(&self.timer_font, scale), text.trim(), pos),
            colors,
            &self.timer_font,
            self.timer_glyph_cache,
            &self.transform,
            self.backend,
        )
        .map_or(pos[0], |g| g.position().x)
    }

    fn choose_abbreviation<'a>(
        &self,
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

    fn measure_text(&self, text: &str, scale: f32) -> f32 {
        font::measure_default_layout(font::scaled(&self.text_font, scale), text)
    }

    fn measure_numbers(&self, text: &str, scale: f32) -> f32 {
        font::layout_numbers(
            font::scaled(&self.text_font, scale),
            text.trim(),
            [0.0, 0.0],
        )
        .last()
        .map_or(0.0, |g| -g.position().x)
    }
}

fn decode_gradient(gradient: &Gradient) -> Option<[[f32; 4]; 4]> {
    Some(match gradient {
        Gradient::Transparent => return None,
        Gradient::Horizontal(left, right) => {
            let left = decode_color(left);
            let right = decode_color(right);
            [left, right, right, left]
        }
        Gradient::Vertical(top, bottom) => {
            let top = decode_color(top);
            let bottom = decode_color(bottom);
            [top, top, bottom, bottom]
        }
        Gradient::Plain(plain) => {
            let plain = decode_color(plain);
            [plain; 4]
        }
    })
}

fn decode_color(color: &Color) -> [f32; 4] {
    let (r, g, b, a) = color.rgba.into();
    [r, g, b, a]
}

fn component_width(component: &ComponentState) -> f32 {
    match component {
        ComponentState::BlankSpace(state) => state.size as f32 * PSEUDO_PIXELS,
        ComponentState::DetailedTimer(_) => 7.0,
        ComponentState::Graph(_) => 7.0,
        ComponentState::KeyValue(_) => 6.0,
        ComponentState::Separator(_) => SEPARATOR_THICKNESS,
        ComponentState::Splits(state) => {
            let column_count = 2.0; // FIXME: Not always 2.
            let split_width = 2.0 + column_count * component::splits::COLUMN_WIDTH;
            state.splits.len() as f32 * split_width
        }
        ComponentState::Text(_) => 6.0,
        ComponentState::Timer(_) => 8.25,
        ComponentState::Title(_) => 8.0,
    }
}

fn component_height(component: &ComponentState) -> f32 {
    match component {
        ComponentState::BlankSpace(state) => state.size as f32 * PSEUDO_PIXELS,
        ComponentState::DetailedTimer(_) => 2.5,
        ComponentState::Graph(state) => state.height as f32 * PSEUDO_PIXELS,
        ComponentState::KeyValue(state) => {
            if state.display_two_rows {
                TWO_ROW_HEIGHT
            } else {
                DEFAULT_COMPONENT_HEIGHT
            }
        }
        ComponentState::Separator(_) => SEPARATOR_THICKNESS,
        ComponentState::Splits(state) => {
            state.splits.len() as f32
                * if state.display_two_rows {
                    TWO_ROW_HEIGHT
                } else {
                    DEFAULT_COMPONENT_HEIGHT
                }
                + if state.column_labels.is_some() {
                    DEFAULT_COMPONENT_HEIGHT
                } else {
                    0.0
                }
        }
        ComponentState::Text(state) => {
            if state.display_two_rows {
                TWO_ROW_HEIGHT
            } else {
                DEFAULT_COMPONENT_HEIGHT
            }
        }
        ComponentState::Timer(state) => state.height as f32 * PSEUDO_PIXELS,
        ComponentState::Title(_) => TWO_ROW_HEIGHT,
    }
}
