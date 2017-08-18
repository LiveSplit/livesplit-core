use {analysis, comparison, GeneralLayoutSettings, TimeSpan, Timer, TimerPhase};
use serde_json::{to_writer, Result};
use std::io::Write;
use std::borrow::Cow;
use settings::{Color, Field, SettingsDescription, Value};

const GRAPH_EDGE_VALUE: f32 = 200.0;
const GRAPH_EDGE_MIN: f32 = 5.0;

const WIDTH: f32 = 180.0;
const HEIGHT: f32 = 120.0;

#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub comparison_override: Option<String>,
    pub show_best_segments: bool,
    pub live_graph: bool,
    pub flip_graph: bool,
    pub behind_background_color: Color,
    pub ahead_background_color: Color,
    pub grid_lines_color: Color,
    pub graph_lines_color: Color,
    pub partial_fill_color: Color,
    pub complete_fill_color: Color,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub points: Vec<Point>,
    pub horizontal_grid_lines: Vec<f32>,
    pub vertical_grid_lines: Vec<f32>,
    pub middle: f32,
    pub is_live_delta_active: bool,
    pub is_flipped: bool,
    pub top_background_color: Color,
    pub bottom_background_color: Color,
    pub grid_lines_color: Color,
    pub graph_lines_color: Color,
    pub partial_fill_color: Color,
    pub complete_fill_color: Color,
    pub best_segment_color: Color,
}

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub is_best_segment: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            comparison_override: None,
            show_best_segments: false,
            live_graph: true,
            flip_graph: false,
            behind_background_color: (115.0 / 255.0, 40.0 / 255.0, 40.0 / 255.0, 1.0).into(),
            ahead_background_color: (40.0 / 255.0, 115.0 / 255.0, 52.0 / 255.0, 1.0).into(),
            grid_lines_color: (0.0, 0.0, 0.0, 0.15).into(),
            graph_lines_color: (1.0, 1.0, 1.0, 1.0).into(),
            partial_fill_color: (1.0, 1.0, 1.0, 0.25).into(),
            complete_fill_color: (1.0, 1.0, 1.0, 0.4).into(),
        }
    }
}

impl State {
    pub fn write_json<W>(&self, writer: W) -> Result<()>
    where
        W: Write,
    {
        to_writer(writer, self)
    }
}

#[derive(Default)]
struct DrawInfo {
    final_split: TimeSpan,
    deltas: Vec<Option<TimeSpan>>,
    max_delta: TimeSpan,
    min_delta: TimeSpan,
    is_live_delta_active: bool,
}

impl Component {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_settings(settings: Settings) -> Self {
        Self {
            settings,
            ..Default::default()
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    pub fn name(&self) -> Cow<str> {
        self.text(
            self.settings
                .comparison_override
                .as_ref()
                .map(String::as_ref),
        )
    }

    fn text(&self, comparison: Option<&str>) -> Cow<str> {
        if let Some(comparison) = comparison {
            format!("Graph ({})", comparison::shorten(comparison)).into()
        } else {
            "Graph".into()
        }
    }

    pub fn state(&self, timer: &Timer, layout_settings: &GeneralLayoutSettings) -> State {
        let comparison = comparison::resolve(&self.settings.comparison_override, timer);
        let comparison = comparison::or_current(comparison, timer);

        let mut draw_info = DrawInfo {
            deltas: Vec::with_capacity(timer.run().len() + 1),
            ..Default::default()
        };

        self.calculate_final_split(timer, &mut draw_info);
        self.calculate_deltas(timer, comparison, &mut draw_info);
        self.check_live_segment_delta(timer, comparison, &mut draw_info);

        let mut state = self.calculate_points(timer, &draw_info, layout_settings);

        self.make_uniform(&mut state);
        self.flip(&mut state);

        state
    }

    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                "Comparison".into(),
                self.settings.comparison_override.clone().into(),
            ),
            Field::new(
                "Show Best Segments".into(),
                self.settings.show_best_segments.into(),
            ),
            Field::new("Live Graph".into(), self.settings.live_graph.into()),
            Field::new("Flip Graph".into(), self.settings.flip_graph.into()),
            Field::new(
                "Behind Background Color".into(),
                self.settings.behind_background_color.into(),
            ),
            Field::new(
                "Ahead Background Color".into(),
                self.settings.ahead_background_color.into(),
            ),
            Field::new(
                "Grid Lines Color".into(),
                self.settings.grid_lines_color.into(),
            ),
            Field::new(
                "Graph Lines Color".into(),
                self.settings.graph_lines_color.into(),
            ),
            Field::new(
                "Partial Fill Color".into(),
                self.settings.partial_fill_color.into(),
            ),
            Field::new(
                "Complete Fill Color".into(),
                self.settings.complete_fill_color.into(),
            ),
        ])
    }

    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.comparison_override = value.into(),
            1 => self.settings.show_best_segments = value.into(),
            2 => self.settings.live_graph = value.into(),
            3 => self.settings.flip_graph = value.into(),
            4 => self.settings.behind_background_color = value.into(),
            5 => self.settings.ahead_background_color = value.into(),
            6 => self.settings.grid_lines_color = value.into(),
            7 => self.settings.graph_lines_color = value.into(),
            8 => self.settings.partial_fill_color = value.into(),
            9 => self.settings.complete_fill_color = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }

    fn make_uniform(&self, state: &mut State) {
        for grid_line in &mut state.horizontal_grid_lines {
            *grid_line /= HEIGHT;
        }

        for grid_line in &mut state.vertical_grid_lines {
            *grid_line /= WIDTH;
        }

        state.middle /= HEIGHT;

        for point in &mut state.points {
            point.x /= WIDTH;
            point.y /= HEIGHT;
        }
    }

    fn flip(&self, state: &mut State) {
        if self.settings.flip_graph {
            for y in &mut state.horizontal_grid_lines {
                *y = 1.0 - *y;
            }

            for point in &mut state.points {
                point.y = 1.0 - point.y;
            }

            state.middle = 1.0 - state.middle;
        }
    }

    fn calculate_points(
        &self,
        timer: &Timer,
        draw_info: &DrawInfo,
        layout_settings: &GeneralLayoutSettings,
    ) -> State {
        let total_delta = draw_info.min_delta - draw_info.max_delta;

        let (graph_edge, graph_height, middle) =
            self.calculate_middle_and_graph_edge(total_delta, draw_info);

        let (grid_value_x, grid_value_y) =
            self.calculate_grid_lines(timer, total_delta, graph_edge, graph_height, draw_info);

        let (horizontal_grid_lines, vertical_grid_lines) =
            self.make_grid_lines_list(graph_height, middle, grid_value_x, grid_value_y);

        let points = self.make_points_list(draw_info, timer, total_delta, graph_edge, graph_height);

        let (top_background_color, bottom_background_color) = if self.settings.flip_graph {
            (
                self.settings.ahead_background_color,
                self.settings.behind_background_color,
            )
        } else {
            (
                self.settings.behind_background_color,
                self.settings.ahead_background_color,
            )
        };

        State {
            points,
            horizontal_grid_lines,
            vertical_grid_lines,
            middle,
            is_live_delta_active: draw_info.is_live_delta_active,
            is_flipped: self.settings.flip_graph,
            top_background_color,
            bottom_background_color,
            grid_lines_color: self.settings.grid_lines_color,
            graph_lines_color: self.settings.graph_lines_color,
            partial_fill_color: self.settings.partial_fill_color,
            complete_fill_color: self.settings.complete_fill_color,
            best_segment_color: layout_settings.best_segment_color,
        }
    }

    fn make_points_list(
        &self,
        draw_info: &DrawInfo,
        timer: &Timer,
        total_delta: TimeSpan,
        graph_edge: f32,
        graph_height: f32,
    ) -> Vec<Point> {
        let mut points_list = Vec::new();
        if !draw_info.deltas.is_empty() {
            let mut height_one = if total_delta != TimeSpan::zero() {
                (-draw_info.max_delta.total_milliseconds() / total_delta.total_milliseconds()) as
                    f32 *
                    (graph_height - graph_edge) * 2.0 + graph_edge
            } else {
                graph_height
            };
            let (mut width_one, mut width_two, mut height_two) = (0.0, 0.0, 0.0);

            points_list.push(Point {
                x: width_one,
                y: height_one,
                is_best_segment: false,
            });

            for (y, &delta) in draw_info.deltas.iter().enumerate() {
                if let Some(delta) = delta {
                    self.calculate_right_side_coordinates(
                        draw_info,
                        timer,
                        total_delta,
                        graph_edge,
                        graph_height,
                        delta,
                        &mut height_two,
                        &mut width_two,
                        y,
                    );

                    let is_best_segment = self.check_best_segment(timer, y);

                    points_list.push(Point {
                        x: width_two,
                        y: height_two,
                        is_best_segment,
                    });

                    self.calculate_left_side_coordinates(
                        draw_info,
                        timer,
                        total_delta,
                        graph_edge,
                        graph_height,
                        delta,
                        &mut height_one,
                        &mut width_one,
                        y,
                    );
                }
            }
        }
        points_list
    }

    fn check_best_segment(&self, timer: &Timer, split_number: usize) -> bool {
        self.settings.show_best_segments && split_number < timer.run().len() &&
            analysis::check_best_segment(timer, split_number, timer.current_timing_method())
    }

    fn calculate_left_side_coordinates(
        &self,
        draw_info: &DrawInfo,
        timer: &Timer,
        total_delta: TimeSpan,
        graph_edge: f32,
        graph_height: f32,
        delta: TimeSpan,
        height_one: &mut f32,
        width_one: &mut f32,
        y: usize,
    ) {
        if total_delta != TimeSpan::zero() {
            *height_one = (delta.total_milliseconds() as f32 -
                draw_info.max_delta.total_milliseconds() as f32) /
                total_delta.total_milliseconds() as f32 *
                (graph_height - graph_edge) * 2.0 + graph_edge;
        } else {
            *height_one = graph_height;
        }

        if y + 1 != draw_info.deltas.len() {
            if let Some(split_time) =
                timer.run().segments[y].split_time[timer.current_timing_method()]
            {
                *width_one = (split_time.total_milliseconds() as f32 /
                    draw_info.final_split.total_milliseconds() as f32) *
                    WIDTH;
            }
        }
    }

    fn calculate_right_side_coordinates(
        &self,
        draw_info: &DrawInfo,
        timer: &Timer,
        total_delta: TimeSpan,
        graph_edge: f32,
        graph_height: f32,
        delta: TimeSpan,
        height_two: &mut f32,
        width_two: &mut f32,
        y: usize,
    ) {
        if y + 1 == draw_info.deltas.len() && draw_info.is_live_delta_active {
            *width_two = WIDTH;
        } else if let Some(split_time) =
            timer.run().segments[y].split_time[timer.current_timing_method()]
        {
            *width_two = (split_time.total_milliseconds() as f32 /
                draw_info.final_split.total_milliseconds() as f32) * WIDTH;
        }

        if total_delta != TimeSpan::zero() {
            *height_two = (delta.total_milliseconds() as f32 -
                draw_info.max_delta.total_milliseconds() as f32) /
                total_delta.total_milliseconds() as f32 *
                (graph_height - graph_edge) * 2.0 + graph_edge;
        } else {
            *height_two = graph_height;
        }
    }

    fn make_grid_lines_list(
        &self,
        graph_height: f32,
        middle: f32,
        grid_value_x: f32,
        grid_value_y: f32,
    ) -> (Vec<f32>, Vec<f32>) {
        let (mut horizontal_grid_lines, mut vertical_grid_lines) = (Vec::new(), Vec::new());

        if grid_value_x > 0.0 {
            let mut x = grid_value_x;
            while x < WIDTH {
                vertical_grid_lines.push(x);
                x += grid_value_x;
            }
        }

        let mut y = middle - 1.0;
        while y > 0.0 {
            horizontal_grid_lines.push(y);
            if grid_value_y < 0.0 {
                break;
            }
            y -= grid_value_y;
        }

        let mut y = middle;
        while y < 2.0 * graph_height {
            horizontal_grid_lines.push(y);
            if grid_value_y < 0.0 {
                break;
            }
            y += grid_value_y;
        }

        (horizontal_grid_lines, vertical_grid_lines)
    }

    fn calculate_grid_lines(
        &self,
        timer: &Timer,
        total_delta: TimeSpan,
        graph_edge: f32,
        graph_height: f32,
        draw_info: &DrawInfo,
    ) -> (f32, f32) {
        let (mut grid_value_x, mut grid_value_y);

        let current_phase = timer.current_phase();
        if current_phase != TimerPhase::NotRunning && draw_info.final_split > TimeSpan::zero() {
            grid_value_x = 1000.0;
            while draw_info.final_split.total_milliseconds() as f32 / grid_value_x > WIDTH / 20.0 {
                grid_value_x *= 6.0;
            }
            grid_value_x =
                (grid_value_x / draw_info.final_split.total_milliseconds() as f32) * WIDTH;
        } else {
            grid_value_x = -1.0;
        }
        if current_phase != TimerPhase::NotRunning && total_delta < TimeSpan::zero() {
            grid_value_y = 1000.0;
            while (-total_delta.total_milliseconds() as f32) / grid_value_y >
                (graph_height - graph_edge) * 2.0 / 20.0
            {
                grid_value_y *= 6.0;
            }
            grid_value_y = (grid_value_y / (-total_delta.total_milliseconds() as f32)) *
                (graph_height - graph_edge) * 2.0;
        } else {
            grid_value_y = -1.0;
        }

        (grid_value_x, grid_value_y)
    }

    fn calculate_middle_and_graph_edge(
        &self,
        total_delta: TimeSpan,
        draw_info: &DrawInfo,
    ) -> (f32, f32, f32) {
        let mut graph_edge = 0.0;
        let graph_height = HEIGHT / 2.0; // TODO Make const
        let middle = if total_delta != TimeSpan::zero() {
            graph_edge = GRAPH_EDGE_VALUE /
                (-total_delta.total_milliseconds() as f32 + 2.0 * GRAPH_EDGE_VALUE) *
                (graph_height * 2.0 - GRAPH_EDGE_MIN * 2.0);
            graph_edge += GRAPH_EDGE_MIN;
            (-(draw_info.max_delta.total_milliseconds() as f32 /
                total_delta.total_milliseconds() as f32)) *
                (graph_height - graph_edge) * 2.0 + graph_edge
        } else {
            graph_height
        };
        (graph_edge, graph_height, middle)
    }

    fn calculate_final_split(&self, timer: &Timer, draw_info: &mut DrawInfo) {
        if timer.current_phase() != TimerPhase::NotRunning {
            if self.settings.live_graph {
                let current_time = timer.current_time();
                let timing_method = timer.current_timing_method();
                draw_info.final_split = current_time[timing_method]
                    .or_else(|| current_time.real_time)
                    .unwrap_or_else(TimeSpan::zero);
            } else {
                let timing_method = timer.current_timing_method();
                for segment in timer.run().segments[..timer.current_split_index() as usize]
                    .iter()
                    .rev()
                {
                    if let Some(time) = segment.split_time[timing_method] {
                        draw_info.final_split = time;
                        return;
                    }
                }
            }
        }
    }

    fn calculate_deltas(&self, timer: &Timer, comparison: &str, draw_info: &mut DrawInfo) {
        let timing_method = timer.current_timing_method();
        for segment in &timer.run().segments {
            let time = TimeSpan::option_sub(
                segment.split_time[timing_method],
                segment.comparison(comparison)[timing_method],
            );
            if let Some(time) = time {
                if time > draw_info.max_delta {
                    draw_info.max_delta = time;
                } else if time < draw_info.min_delta {
                    draw_info.min_delta = time;
                }
            }
            draw_info.deltas.push(time);
        }
    }

    fn check_live_segment_delta(&self, timer: &Timer, comparison: &str, draw_info: &mut DrawInfo) {
        if self.settings.live_graph {
            let current_phase = timer.current_phase();
            if current_phase == TimerPhase::Running || current_phase == TimerPhase::Paused {
                let timing_method = timer.current_timing_method();
                let mut best_segment =
                    analysis::check_live_delta(timer, true, comparison, timing_method);
                // TODO Try if let instead of checking current phase up there, so we can skip this unwrap
                let current_split =
                    timer.current_split().unwrap().comparison(comparison)[timing_method];
                let current_time = timer.current_time()[timing_method];
                if let (Some(current_time), Some(current_split), None) =
                    (current_time, current_split, best_segment)
                {
                    let diff = current_time - current_split;
                    if diff > draw_info.min_delta {
                        best_segment = Some(diff);
                    }
                }
                if let Some(best_segment) = best_segment {
                    if best_segment > draw_info.max_delta {
                        draw_info.max_delta = best_segment;
                    } else if best_segment < draw_info.min_delta {
                        draw_info.min_delta = best_segment;
                    }
                    draw_info.deltas.push(Some(best_segment));
                    draw_info.is_live_delta_active = true;
                }
            }
        }
    }
}
