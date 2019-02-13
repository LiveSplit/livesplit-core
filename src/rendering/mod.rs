//! The rendering module provides a renderer for layout states that is
//! abstracted over different backends so that it can be used with OpenGL,
//! DirectX, Vulkan, Metal, WebGL or any other rendering framework. An optional
//! software renderer is available behind the `software-rendering` feature.
//! While it is slower than using a proper GPU backend, it might be sufficient
//! for situations where you want to create a screenshot of the layout.

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
        layout::{ComponentState, LayoutState},
        settings::{Color, Gradient},
    },
    euclid::Transform2D,
    rusttype::Font,
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
pub type Transform = Transform2D<f32>;

const MARGIN: f32 = 0.35;
const TWO_ROW_HEIGHT: f32 = 1.75;

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
    fn resize(&mut self, height: f32);
}

/// A renderer can be used to render out layout states with the backend chosen.
pub struct Renderer<M, T> {
    text_font: Font<'static>,
    text_glyph_cache: GlyphCache<M>,
    timer_font: Font<'static>,
    timer_glyph_cache: GlyphCache<M>,
    rectangle: Option<M>,
    game_icon: Option<Icon<T>>,
    split_icons: Vec<Option<Icon<T>>>,
    detailed_timer_icon: Option<Icon<T>>,
    height: Option<f32>,
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
            game_icon: None,
            split_icons: Vec::new(),
            detailed_timer_icon: None,
            height: None,
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
        let total_height = state.components.iter().map(component_height).sum::<f32>();
        {
            let cached_total_height = self.height.get_or_insert(total_height);

            if *cached_total_height != total_height {
                backend.resize(resolution.1 / *cached_total_height * total_height);
                *cached_total_height = total_height;
            }
        }
        let width = resolution.0 as f32 / resolution.1 as f32;

        let mut context = RenderContext {
            backend,
            transform: Transform::identity(),
            rectangle: &mut self.rectangle,
            timer_font: &mut self.timer_font,
            timer_glyph_cache: &mut self.timer_glyph_cache,
            text_font: &mut self.text_font,
            text_glyph_cache: &mut self.text_glyph_cache,
            width,
        };

        context.render_background(&state.background);

        context.scale_non_uniform_x(width.recip());
        context.scale(total_height.recip());

        for component in &state.components {
            let height = component_height(component);
            let width = context.width;
            let dim = [width, height];
            match component {
                ComponentState::BlankSpace(state) => {
                    component::blank_space::render(&mut context, dim, state)
                }
                ComponentState::Title(component) => component::title::render(
                    &mut context,
                    dim,
                    component,
                    state,
                    &mut self.game_icon,
                ),
                ComponentState::Splits(component) => component::splits::render(
                    &mut context,
                    dim,
                    component,
                    state,
                    &mut self.split_icons,
                ),
                ComponentState::Timer(component) => {
                    component::timer::render(&mut context, dim, component);
                }
                ComponentState::DetailedTimer(component) => component::detailed_timer::render(
                    &mut context,
                    dim,
                    component,
                    state,
                    &mut self.detailed_timer_icon,
                ),
                ComponentState::CurrentComparison(component) => {
                    component::current_comparison::render(&mut context, dim, component, state)
                }
                ComponentState::CurrentPace(component) => {
                    component::current_pace::render(&mut context, dim, component, state)
                }
                ComponentState::Delta(component) => {
                    component::delta::render(&mut context, dim, component, state)
                }
                ComponentState::PossibleTimeSave(component) => {
                    component::possible_time_save::render(&mut context, dim, component, state)
                }
                ComponentState::PreviousSegment(component) => {
                    component::previous_segment::render(&mut context, dim, component, state)
                }
                ComponentState::Separator(component) => {
                    component::separator::render(&mut context, dim, component, state)
                }
                ComponentState::SumOfBest(component) => {
                    component::sum_of_best::render(&mut context, dim, component, state)
                }
                ComponentState::Text(component) => {
                    component::text::render(&mut context, dim, component, state)
                }
                ComponentState::TotalPlaytime(component) => {
                    component::total_playtime::render(&mut context, dim, component, state)
                }
                ComponentState::Graph(component) => {
                    component::graph::render(&mut context, dim, component, state)
                }
            }
            context.translate(0.0, height);
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
    width: f32,
}

impl<'b, B: Backend> RenderContext<'b, B> {
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

    fn create_icon(&mut self, image_url: &str) -> Option<Icon<B::Texture>> {
        if !image_url.starts_with("data:;base64,") {
            return None;
        }

        let url = &image_url["data:;base64,".len()..];
        let image_data = base64::decode(url).ok()?;
        let image = image::load_from_memory(&image_data).ok()?.to_rgba();
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
        self.width /= factor;
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

    fn render_info_time_component(
        &mut self,
        text: &str,
        value: &str,
        text_color: Color,
        value_color: Color,
        display_two_rows: bool,
    ) {
        let width = self.width;
        let height = if display_two_rows {
            TWO_ROW_HEIGHT
        } else {
            1.0
        };
        let end_x = if display_two_rows {
            width
        } else {
            self.render_numbers(value, [width - MARGIN, height - 0.3], 0.8, [value_color; 2])
        };
        self.render_text_ellipsis(text, [MARGIN, 0.7], 0.8, [text_color; 2], end_x - MARGIN);
    }

    fn render_info_text_component(
        &mut self,
        text: &str,
        value: &str,
        text_color: Color,
        value_color: Color,
        display_two_rows: bool,
    ) {
        let width = self.width;
        let height = if display_two_rows {
            TWO_ROW_HEIGHT
        } else {
            1.0
        };
        let end_x = if display_two_rows {
            width
        } else {
            self.render_text_right_align(
                value,
                [width - MARGIN, height - 0.3],
                0.8,
                [value_color; 2],
            )
        };
        self.render_text_ellipsis(text, [MARGIN, 0.7], 0.8, [text_color; 2], end_x - MARGIN);
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

fn component_height(component: &ComponentState) -> f32 {
    const PSEUDO_PIXELS: f32 = 1.0 / 24.0;

    match component {
        ComponentState::BlankSpace(state) => state.height as f32 * PSEUDO_PIXELS,
        ComponentState::CurrentComparison(state) => {
            if state.display_two_rows {
                TWO_ROW_HEIGHT
            } else {
                1.0
            }
        }
        ComponentState::CurrentPace(state) => {
            if state.display_two_rows {
                TWO_ROW_HEIGHT
            } else {
                1.0
            }
        }
        ComponentState::DetailedTimer(_) => 2.5,
        ComponentState::Delta(state) => {
            if state.display_two_rows {
                TWO_ROW_HEIGHT
            } else {
                1.0
            }
        }
        ComponentState::Graph(state) => state.height as f32 * PSEUDO_PIXELS,
        ComponentState::Separator(_) => 0.1,
        ComponentState::PossibleTimeSave(state) => {
            if state.display_two_rows {
                TWO_ROW_HEIGHT
            } else {
                1.0
            }
        }
        ComponentState::PreviousSegment(state) => {
            if state.display_two_rows {
                TWO_ROW_HEIGHT
            } else {
                1.0
            }
        }
        ComponentState::Splits(state) => {
            state.splits.len() as f32
                * if state.display_two_rows {
                    TWO_ROW_HEIGHT
                } else {
                    1.0
                }
                + if state.column_labels.is_some() {
                    1.0
                } else {
                    0.0
                }
        }
        ComponentState::SumOfBest(state) => {
            if state.display_two_rows {
                TWO_ROW_HEIGHT
            } else {
                1.0
            }
        }
        ComponentState::Text(state) => {
            if state.display_two_rows {
                TWO_ROW_HEIGHT
            } else {
                1.0
            }
        }
        ComponentState::Timer(state) => state.height as f32 * PSEUDO_PIXELS,
        ComponentState::Title(_) => 2.0,
        ComponentState::TotalPlaytime(state) => {
            if state.display_two_rows {
                TWO_ROW_HEIGHT
            } else {
                1.0
            }
        }
    }
}
