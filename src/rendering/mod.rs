mod font;
mod glyph_cache;
mod mesh;

use self::glyph_cache::GlyphCache;
use self::mesh::{fill_builder, stroke_builder};
use crate::{
    component::{text::Text, timer::State as TimerComponentState},
    layout::{ComponentState, LayoutState},
    settings::{Color, Gradient, ListGradient},
};
use euclid::Transform2D;
use livesplit_title_abbreviations::abbreviate;
use lyon::tessellation::{
    basic_shapes::{fill_circle, fill_polyline, stroke_polyline},
    FillOptions, FillTessellator, StrokeOptions,
};
use ordered_float::OrderedFloat;
use rusttype::Font;
use std::iter;

pub use self::mesh::{Mesh, Vertex};

pub static TEXT_FONT: &[u8] = include_bytes!("FiraSans-Regular.ttf");
pub static TIMER_FONT: &[u8] = include_bytes!("Timer.ttf");

pub type Pos = [f32; 2];
pub type Rgba = [f32; 4];
pub type Transform = Transform2D<f32>;
pub type IndexPair = [usize; 3];

const MARGIN: f32 = 0.35;
const TWO_ROW_HEIGHT: f32 = 1.75;

pub trait Backend {
    fn create_mesh(&mut self, mesh: &Mesh) -> IndexPair;
    fn render_mesh(
        &mut self,
        mesh: IndexPair,
        transform: Transform,
        colors: [Rgba; 4],
        texture: Option<IndexPair>,
    );
    fn free_mesh(&mut self, mesh: IndexPair);

    fn create_texture(&mut self, width: u32, height: u32, data: &[u8]) -> IndexPair;
    fn free_texture(&mut self, texture: IndexPair);

    fn resize(&mut self, height: f32);
}

pub struct Renderer {
    text_font: Font<'static>,
    text_glyph_cache: GlyphCache,
    timer_font: Font<'static>,
    timer_glyph_cache: GlyphCache,
    rectangle: Option<IndexPair>,
    game_icon: Option<(IndexPair, f32)>,
    split_icons: Vec<Option<(IndexPair, f32)>>,
    detailed_timer_icon: Option<(IndexPair, f32)>,
    height: Option<f32>,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
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

    pub fn render<B: Backend>(
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
            match component {
                ComponentState::BlankSpace(state) => {
                    context.render_rectangle([0.0, 0.0], [width, height], &state.background);
                }
                ComponentState::Title(component) => {
                    context.render_rectangle([0.0, 0.0], [width, height], &component.background);
                    let text_color = component.text_color.unwrap_or(state.text_color);

                    // TODO: For now let's just assume there's an icon.

                    if let Some(url) = &component.icon_change {
                        if let Some((old_texture, _)) = self.game_icon.take() {
                            context.backend.free_texture(old_texture);
                        }
                        self.game_icon = context.create_texture(url);
                    }

                    let left_bound = if let Some(icon) = self.game_icon {
                        let icon_size = 2.0 - 2.0 * MARGIN;
                        context.render_image([MARGIN, MARGIN], [icon_size, icon_size], icon);
                        2.0 * MARGIN + icon_size
                    } else {
                        MARGIN
                    };

                    let (x, align) = if component.is_centered {
                        (0.5 * width, 0.5)
                    } else {
                        (left_bound, 0.0)
                    };

                    // TODO: Positioning for a single line
                    // TODO: Abbreviations with single line are weird
                    let abbreviations = abbreviate(&component.line1);
                    let line1 = abbreviations
                        .iter()
                        .map(|line| (line.as_str(), context.measure_text(line, 0.8)))
                        .filter(|&(_, len)| len < width - MARGIN - left_bound)
                        .max_by_key(|&(_, len)| OrderedFloat(len))
                        .map(|(line, _)| line)
                        .unwrap_or(&component.line1);

                    let attempts = match (component.finished_runs, component.attempts) {
                        (Some(a), Some(b)) => format!("{}/{}", a, b),
                        (Some(a), _) | (_, Some(a)) => a.to_string(),
                        _ => String::new(),
                    };
                    let line2_end_x = context.render_numbers(
                        &attempts,
                        [width - MARGIN, 1.63],
                        0.8,
                        [text_color; 2],
                    );

                    context.render_text_align(
                        line1,
                        left_bound,
                        width + MARGIN, // TODO: Should be - MARGIN
                        [x, 0.83],
                        0.8,
                        align,
                        text_color,
                    );
                    if let Some(line2) = &component.line2 {
                        context.render_text_align(
                            line2,
                            left_bound,
                            line2_end_x - MARGIN,
                            [x, 1.63],
                            0.8,
                            align,
                            text_color,
                        );
                    }
                }
                ComponentState::Splits(component) => {
                    let split_background = match component.background {
                        ListGradient::Same(gradient) => {
                            context.render_rectangle([0.0, 0.0], [width, height], &gradient);
                            None
                        }
                        ListGradient::Alternating(even, odd) => {
                            Some((Gradient::Plain(even), Gradient::Plain(odd)))
                        }
                    };

                    let width = context.width;

                    let split_height = if component.display_two_rows {
                        TWO_ROW_HEIGHT
                    } else {
                        1.0
                    };
                    let transform = context.transform;

                    for icon_change in &component.icon_changes {
                        if icon_change.segment_index >= self.split_icons.len() {
                            self.split_icons.resize(icon_change.segment_index + 1, None);
                        }
                        let icon = &mut self.split_icons[icon_change.segment_index];
                        if let Some((old_texture, _)) = icon.take() {
                            context.backend.free_texture(old_texture);
                        }
                        *icon = context.create_texture(&icon_change.icon);
                    }

                    const COLUMN_WIDTH: f32 = 3.0;

                    if let Some(column_labels) = &component.column_labels {
                        let mut right_x = width - MARGIN;
                        for label in column_labels {
                            let left_x = right_x - COLUMN_WIDTH;
                            context.render_text_right_align(
                                label,
                                [right_x, 0.7],
                                0.8,
                                [state.text_color; 2],
                            );
                            right_x = left_x;
                        }

                        context.translate(0.0, 1.0);
                        context.render_rectangle(
                            [0.0, -0.05],
                            [width, 0.05],
                            &Gradient::Plain(state.separators_color),
                        );
                    }

                    for (i, split) in component.splits.iter().enumerate() {
                        if component.show_thin_separators && i + 1 != component.splits.len() {
                            context.render_rectangle(
                                [0.0, split_height - 0.05],
                                [width, split_height],
                                &Gradient::Plain(state.thin_separators_color),
                            );
                        }

                        if split.is_current_split {
                            context.render_rectangle(
                                [0.0, 0.0],
                                [width, split_height],
                                &component.current_split_gradient,
                            );
                        } else if let Some((even, odd)) = &split_background {
                            let color = if split.index % 2 == 0 { even } else { odd };
                            context.render_rectangle(
                                [0.0, 0.0],
                                [width, split_height - 0.05],
                                color,
                            );
                        }

                        {
                            // TODO: For now let's just assume there's an icon.
                            let icon_size = split_height - 0.2;
                            let icon_right = MARGIN + icon_size;

                            if let Some(icon) = self.split_icons.get(split.index).and_then(|&x| x) {
                                context.render_image(
                                    [MARGIN, 0.1 - 0.5 * 0.05],
                                    [icon_size, icon_size],
                                    icon,
                                );
                            }

                            let mut left_x = width - MARGIN;
                            let mut right_x = left_x;
                            for column in &split.columns {
                                if !column.value.is_empty() {
                                    left_x = context.render_numbers(
                                        &column.value,
                                        [right_x, split_height - 0.3],
                                        0.8,
                                        [column.visual_color; 2],
                                    );
                                }
                                right_x -= COLUMN_WIDTH;
                            }

                            if component.display_two_rows {
                                left_x = width;
                            }

                            context.render_text_ellipsis(
                                &split.name,
                                [icon_right + MARGIN, 0.7],
                                0.8,
                                [state.text_color; 2],
                                left_x - MARGIN,
                            );
                        }
                        context.translate(0.0, split_height);
                    }
                    if component.show_final_separator {
                        context.render_rectangle(
                            [0.0, -split_height - 0.05],
                            [width, -split_height + 0.05],
                            &Gradient::Plain(state.separators_color),
                        );
                    }
                    context.transform = transform;
                }
                ComponentState::Timer(component) => {
                    render_timer_component(&mut context, component, width, height);
                }
                ComponentState::DetailedTimer(component) => {
                    context.render_rectangle([0.0, 0.0], [width, height], &component.background);

                    let icon_size = height - 2.0 * MARGIN;

                    if let Some(url) = &component.icon_change {
                        if let Some((old_texture, _)) = self.detailed_timer_icon.take() {
                            context.backend.free_texture(old_texture);
                        }
                        self.detailed_timer_icon = context.create_texture(url);
                    }

                    let left_side = if let Some(icon) = self.detailed_timer_icon {
                        context.render_image([MARGIN, MARGIN], [icon_size, icon_size], icon);
                        2.0 * MARGIN + icon_size
                    } else {
                        MARGIN
                    };

                    let top_height = 0.55 * height;
                    let bottom_height = height - top_height;

                    let timer_end =
                        render_timer_component(&mut context, &component.timer, width, top_height);

                    if let Some(segment_name) = &component.segment_name {
                        context.render_text_ellipsis(
                            &segment_name,
                            [left_side, 0.6 * top_height],
                            0.5 * top_height,
                            [state.text_color; 2],
                            timer_end,
                        );
                    }

                    context.translate(0.0, top_height);

                    let segment_timer_end = render_timer_component(
                        &mut context,
                        &component.segment_timer,
                        width,
                        bottom_height,
                    );

                    context.translate(0.0, -top_height);

                    let mut name_end = 0.0;
                    let comparison_text_scale = 0.5 * bottom_height;
                    let comparison2_y = 0.8 * bottom_height + top_height;
                    let mut time_width = 0.0;

                    let comparison1_y = if let Some(comparison) = &component.comparison2 {
                        name_end = context
                            .render_text_ellipsis(
                                &comparison.name,
                                [left_side, comparison2_y],
                                comparison_text_scale,
                                [state.text_color; 2],
                                segment_timer_end,
                            )
                            .max(name_end);

                        time_width = context
                            .measure_numbers(&comparison.time, comparison_text_scale)
                            .max(time_width);

                        comparison2_y - comparison_text_scale
                    } else {
                        comparison2_y
                    };

                    if let Some(comparison) = &component.comparison1 {
                        name_end = context
                            .render_text_ellipsis(
                                &comparison.name,
                                [left_side, comparison1_y],
                                comparison_text_scale,
                                [state.text_color; 2],
                                segment_timer_end,
                            )
                            .max(name_end);

                        time_width = context
                            .measure_numbers(&comparison.time, comparison_text_scale)
                            .max(time_width);
                    }

                    let mut time_x = name_end + MARGIN + time_width;

                    if let Some(comparison) = &component.comparison2 {
                        context.render_numbers(
                            &comparison.time,
                            [time_x, comparison2_y],
                            comparison_text_scale,
                            [state.text_color; 2],
                        );
                    }
                    if let Some(comparison) = &component.comparison1 {
                        context.render_numbers(
                            &comparison.time,
                            [time_x, comparison1_y],
                            comparison_text_scale,
                            [state.text_color; 2],
                        );
                    }
                }
                ComponentState::CurrentComparison(component) => {
                    context.render_rectangle([0.0, 0.0], [width, height], &component.background);
                    context.render_info_text_component(
                        &component.text,
                        &component.comparison,
                        component.label_color.unwrap_or(state.text_color),
                        component.value_color.unwrap_or(state.text_color),
                        component.display_two_rows,
                    );
                }
                ComponentState::CurrentPace(component) => {
                    context.render_rectangle([0.0, 0.0], [width, height], &component.background);
                    context.render_info_time_component(
                        &component.text,
                        &component.time,
                        component.label_color.unwrap_or(state.text_color),
                        component.value_color.unwrap_or(state.text_color),
                        component.display_two_rows,
                    );
                }
                ComponentState::Delta(component) => {
                    context.render_rectangle([0.0, 0.0], [width, height], &component.background);
                    context.render_info_time_component(
                        &component.text,
                        &component.time,
                        component.label_color.unwrap_or(state.text_color),
                        component.visual_color,
                        component.display_two_rows,
                    );
                }
                // ComponentState::PbChance(component) => {
                //     context.render_rectangle([0.0, 0.0], [width, height], &component.background);
                //     context.render_info_time_component(
                //         &component.text,
                //         &component.value,
                //         component.label_color.unwrap_or(state.text_color),
                //         component.value_color.unwrap_or(state.text_color),
                //     );
                // }
                ComponentState::PossibleTimeSave(component) => {
                    context.render_rectangle([0.0, 0.0], [width, height], &component.background);
                    context.render_info_time_component(
                        &component.text,
                        &component.time,
                        component.label_color.unwrap_or(state.text_color),
                        component.value_color.unwrap_or(state.text_color),
                        component.display_two_rows,
                    );
                }
                ComponentState::PreviousSegment(component) => {
                    context.render_rectangle([0.0, 0.0], [width, height], &component.background);
                    context.render_info_time_component(
                        &component.text,
                        &component.time,
                        component.label_color.unwrap_or(state.text_color),
                        component.visual_color,
                        component.display_two_rows,
                    );
                }
                ComponentState::Separator(_) => {
                    context.render_rectangle(
                        [0.0, 0.0],
                        [width, height],
                        &Gradient::Plain(state.separators_color),
                    );
                }
                ComponentState::SumOfBest(component) => {
                    context.render_rectangle([0.0, 0.0], [width, height], &component.background);
                    context.render_info_time_component(
                        &component.text,
                        &component.time,
                        component.label_color.unwrap_or(state.text_color),
                        component.value_color.unwrap_or(state.text_color),
                        component.display_two_rows,
                    );
                }
                ComponentState::Text(component) => {
                    context.render_rectangle([0.0, 0.0], [width, height], &component.background);
                    match &component.text {
                        Text::Center(text) => context.render_text_centered(
                            text,
                            MARGIN,
                            width - MARGIN,
                            [0.5 * width, 0.7],
                            0.8,
                            component.left_center_color.unwrap_or(state.text_color),
                        ),
                        Text::Split(left, right) => context.render_info_text_component(
                            &left,
                            &right,
                            component.left_center_color.unwrap_or(state.text_color),
                            component.right_color.unwrap_or(state.text_color),
                            component.display_two_rows,
                        ),
                    }
                }
                ComponentState::TotalPlaytime(component) => {
                    context.render_rectangle([0.0, 0.0], [width, height], &component.background);
                    context.render_info_time_component(
                        &component.text,
                        &component.time,
                        component.label_color.unwrap_or(state.text_color),
                        component.value_color.unwrap_or(state.text_color),
                        component.display_two_rows,
                    );
                }
                ComponentState::Graph(component) => {
                    let (old_width, old_transform) = (context.width, context.transform);
                    context.scale(height);
                    let width = context.width;

                    const GRID_LINE_WIDTH: f32 = 0.015;
                    const LINE_WIDTH: f32 = 0.025;
                    const CIRCLE_RADIUS: f32 = 0.035;

                    context.render_rectangle(
                        [0.0, 0.0],
                        [width, component.middle],
                        &Gradient::Plain(component.top_background_color),
                    );
                    context.render_rectangle(
                        [0.0, component.middle],
                        [width, 1.0],
                        &Gradient::Plain(component.bottom_background_color),
                    );

                    for &y in &component.horizontal_grid_lines {
                        context.render_rectangle(
                            [0.0, y - GRID_LINE_WIDTH],
                            [width, y + GRID_LINE_WIDTH],
                            &Gradient::Plain(component.grid_lines_color),
                        );
                    }

                    for &x in &component.vertical_grid_lines {
                        context.render_rectangle(
                            [width * x - GRID_LINE_WIDTH, 0.0],
                            [width * x + GRID_LINE_WIDTH, 1.0],
                            &Gradient::Plain(component.grid_lines_color),
                        );
                    }

                    let mut mesh = Mesh::new();

                    let len = if component.is_live_delta_active {
                        let p1 = &component.points[component.points.len() - 2];
                        let p2 = &component.points[component.points.len() - 1];

                        fill_polyline(
                            [
                                [p1.x, component.middle],
                                [p1.x, p1.y],
                                [p2.x, p2.y],
                                [p2.x, component.middle],
                            ]
                            .iter()
                            .map(|&[x, y]| [width * x, y].into()),
                            &mut FillTessellator::new(),
                            &FillOptions::tolerance(0.005).with_normals(false),
                            &mut fill_builder(&mut mesh),
                        )
                        .unwrap();

                        let index = context.create_mesh(&mesh);
                        context.render_mesh(index, component.partial_fill_color);
                        context.free_mesh(index);

                        mesh.vertices.clear();
                        mesh.indices.clear();

                        component.points.len() - 1
                    } else {
                        component.points.len()
                    };

                    fill_polyline(
                        iter::once([0.0, component.middle].into())
                            .chain(
                                component.points[..len]
                                    .iter()
                                    .map(|p| [width * p.x, p.y].into()),
                            )
                            .chain(iter::once(
                                [width * component.points[len - 1].x, component.middle].into(),
                            )),
                        &mut FillTessellator::new(),
                        &FillOptions::tolerance(0.005).with_normals(false),
                        &mut fill_builder(&mut mesh),
                    )
                    .unwrap();

                    let index = context.create_mesh(&mesh);
                    context.render_mesh(index, component.complete_fill_color);
                    context.free_mesh(index);

                    for points in component.points.windows(2) {
                        mesh.vertices.clear();
                        mesh.indices.clear();

                        let p1 = [width * points[0].x, points[0].y].into();
                        let p2 = [width * points[1].x, points[1].y].into();

                        stroke_polyline(
                            iter::once(p1).chain(iter::once(p2)),
                            false,
                            &StrokeOptions::default().with_line_width(LINE_WIDTH),
                            &mut stroke_builder(&mut mesh),
                        );

                        let color = if points[1].is_best_segment {
                            component.best_segment_color
                        } else {
                            component.graph_lines_color
                        };

                        let index = context.create_mesh(&mesh);
                        context.render_mesh(index, color);
                        context.free_mesh(index);
                    }

                    for (i, point) in component.points.iter().enumerate().skip(1) {
                        if i != component.points.len() - 1 || !component.is_live_delta_active {
                            mesh.vertices.clear();
                            mesh.indices.clear();

                            fill_circle(
                                [width * point.x, point.y].into(),
                                CIRCLE_RADIUS,
                                &FillOptions::tolerance(0.005).with_normals(false),
                                &mut fill_builder(&mut mesh),
                            );

                            let color = if point.is_best_segment {
                                component.best_segment_color
                            } else {
                                component.graph_lines_color
                            };

                            let index = context.create_mesh(&mesh);
                            context.render_mesh(index, color);
                            context.free_mesh(index);
                        }
                    }

                    context.width = old_width;
                    context.transform = old_transform;
                }
            }
            context.translate(0.0, height);
        }
    }
}

struct RenderContext<'b, B: Backend> {
    transform: Transform,
    backend: &'b mut B,
    rectangle: &'b mut Option<IndexPair>,
    timer_font: &'b mut Font<'static>,
    timer_glyph_cache: &'b mut GlyphCache,
    text_font: &'b mut Font<'static>,
    text_glyph_cache: &'b mut GlyphCache,
    width: f32,
}

impl<'b, B: Backend> RenderContext<'b, B> {
    fn backend_render_rectangle(&mut self, [x1, y1]: Pos, [x2, y2]: Pos, colors: [Rgba; 4]) {
        let transform = self
            .transform
            .pre_translate([x1, y1].into())
            .pre_scale(x2 - x1, y2 - y1);

        let index_pair = *self.rectangle.get_or_insert_with({
            let backend = &mut self.backend;
            move || backend.create_mesh(&mesh::rectangle())
        });

        self.backend
            .render_mesh(index_pair, transform, colors, None);
    }

    fn create_mesh(&mut self, mesh: &Mesh) -> IndexPair {
        self.backend.create_mesh(mesh)
    }

    fn render_mesh(&mut self, mesh: IndexPair, color: Color) {
        self.backend
            .render_mesh(mesh, self.transform, [decode_color(&color); 4], None)
    }

    fn create_texture(&mut self, image_url: &str) -> Option<(IndexPair, f32)> {
        if image_url.starts_with("data:;base64,") {
            let url = &image_url["data:;base64,".len()..];
            let image_data = base64::decode(url).unwrap();
            let image = image::load_from_memory(&image_data).unwrap().to_rgba();
            let index = self
                .backend
                .create_texture(image.width(), image.height(), &image);
            Some((index, image.width() as f32 / image.height() as f32))
        } else {
            None
        }
    }

    fn free_mesh(&mut self, mesh: IndexPair) {
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

    fn render_image(
        &mut self,
        [mut x, mut y]: Pos,
        [mut width, mut height]: Pos,
        (texture, aspect_ratio): (IndexPair, f32),
    ) {
        let box_aspect_ratio = width / height;
        let aspect_ratio_diff = box_aspect_ratio / aspect_ratio;

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

        let index_pair = *self.rectangle.get_or_insert_with({
            let backend = &mut self.backend;
            move || backend.create_mesh(&mesh::rectangle())
        });

        self.backend
            .render_mesh(index_pair, transform, [[1.0; 4]; 4], Some(texture));
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

fn render_timer_component(
    context: &mut RenderContext<'_, impl Backend>,
    component: &TimerComponentState,
    width: f32,
    height: f32,
) -> f32 {
    context.render_rectangle([0.0, 0.0], [width, height], &component.background);
    let x = context.render_timer(
        &component.fraction,
        [width - MARGIN, 0.85 * height],
        0.8 * height,
        [component.bottom_color, component.top_color],
    );
    context.render_timer(
        &component.time,
        [x, 0.85 * height],
        1.2 * height,
        [component.bottom_color, component.top_color],
    )
}
