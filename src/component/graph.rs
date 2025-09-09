//! Provides the Graph Component and relevant types for using it. The Graph
//! Component visualizes how far the current attempt has been ahead or behind
//! the chosen comparison throughout the whole attempt. Every point of the graph
//! represents a split. Its x-coordinate is proportional to the split time and
//! its y-coordinate is proportional to the split delta. The entire diagram is
//! referred to as the chart and it contains the graph. The x-axis is the
//! horizontal line that separates positive deltas from negative ones.

// The words "padding" and "content" are from the CSS box model. "Padding" is an
// area at the top/bottom that stays empty so that the graph doesn't touch the
// edge. "Content" is the rest (the area inside).

use crate::{
    GeneralLayoutSettings, TimeSpan, Timer, TimerPhase, analysis, comparison,
    platform::prelude::*,
    settings::{Color, Field, SettingsDescription, Value},
    timing::Snapshot,
};
use alloc::borrow::Cow;
use serde_derive::{Deserialize, Serialize};

const WIDTH: f32 = 1.0;
const HEIGHT: f32 = 1.0;
const DEFAULT_X_AXIS: f32 = HEIGHT / 2.0;

/// The Graph Component visualizes how far the current attempt has been ahead or
/// behind the chosen comparison throughout the whole attempt. All the
/// individual deltas are shown as points in a graph.
#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
}

/// The Settings for this component.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// The comparison chosen. Uses the Timer's current comparison if set to
    /// `None`.
    pub comparison_override: Option<String>,
    /// Specifies if the best segments should be colored with the layout's best
    /// segment color.
    pub show_best_segments: bool,
    /// Specifies if the graph should automatically adjust to all changes. If
    /// this is deactivated, changes to the graph only happen whenever the
    /// current segment changes.
    pub live_graph: bool,
    /// Flips the chart. If set to `false`, split times which are ahead of the
    /// comparison are displayed below the x-axis and times which are behind are
    /// above it. Enabling this settings flips it.
    pub flip_graph: bool,
    /// The background color for the chart region containing the times that are
    /// behind the comparison.
    pub behind_background_color: Color,
    /// The background color for the chart region containing the times that are
    /// ahead of the comparison.
    pub ahead_background_color: Color,
    /// The color of the chart's grid lines.
    pub grid_lines_color: Color,
    /// The color of the lines connecting the graph's points.
    pub graph_lines_color: Color,
    /// The color of the region enclosed by the x-axis and the graph. The
    /// partial fill color is only used for live changes. More specifically,
    /// this color is used in the interval from the last split time to the
    /// current time.
    pub partial_fill_color: Color,
    /// The color of the region enclosed by the x-axis and the graph, excluding
    /// the graph segment with live changes.
    pub complete_fill_color: Color,
    /// The height of the chart.
    pub height: u32,
}

/// The state object describes the information to visualize for this component.
/// All coordinates are in the range `0..1`.
#[derive(Default, Serialize, Deserialize)]
pub struct State {
    /// All of the graph's points. Connect them to visualize the graph.
    /// If the live delta is active, the last point is to be interpreted as a
    /// preview of the next split. Use the partial fill color to visualize the
    /// region beneath that graph segment.
    pub points: Vec<Point>,
    /// The y-coordinates of all the horizontal grid lines.
    pub horizontal_grid_lines: Vec<f32>,
    /// The x-coordinates of all the vertical grid lines.
    pub vertical_grid_lines: Vec<f32>,
    /// The y-coordinate of the x-axis.
    pub middle: f32,
    /// If the live delta is active, the last point is to be interpreted as a
    /// preview of the next split. Use the partial fill color to visualize the
    /// region beneath that graph segment.
    pub is_live_delta_active: bool,
    /// Describes whether the chart is flipped vertically. For visualization,
    /// this can usually be ignored, as it is already regarded in the
    /// other variables.
    pub is_flipped: bool,
    /// The background color of the region of the chart that is above the
    /// x-axis.
    pub top_background_color: Color,
    /// The background color of the region of the chart that is below the
    /// x-axis.
    pub bottom_background_color: Color,
    /// The color of the chart's grid lines.
    pub grid_lines_color: Color,
    /// The color of the lines connecting the graph's points.
    pub graph_lines_color: Color,
    /// The color of the region enclosed by the x-axis and the graph. The
    /// partial fill color is only used for live changes. More specifically,
    /// this color is used in the interval from the last split time to the
    /// current time.
    pub partial_fill_color: Color,
    /// The color of the region enclosed by the x-axis and the graph, excluding
    /// the graph segment with live changes.
    pub complete_fill_color: Color,
    /// The color of the lines of graph segments that achieved a new best
    /// segment time.
    pub best_segment_color: Color,
    /// The height of the chart.
    pub height: u32,
    /// This value indicates whether the graph is currently frequently being
    /// updated. This can be used for rendering optimizations.
    pub updates_frequently: bool,
}

/// Describes a point on the graph to visualize.
#[derive(Serialize, Deserialize)]
pub struct Point {
    /// The x-coordinate of the point.
    pub x: f32,
    /// The y-coordinate of the point.
    // N.B. this is initially set to an intermediate value which needs to be
    // transformed before being sent to the renderer, see transform_y_coordinates.
    pub y: f32,
    /// Describes whether the segment this point is visualizing achieved a new
    /// best segment time. Use the best segment color for it, in that case.
    pub is_best_segment: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            comparison_override: None,
            show_best_segments: false,
            live_graph: true,
            flip_graph: false,
            behind_background_color: Color::rgba(115.0 / 255.0, 40.0 / 255.0, 40.0 / 255.0, 1.0),
            ahead_background_color: Color::rgba(40.0 / 255.0, 115.0 / 255.0, 52.0 / 255.0, 1.0),
            grid_lines_color: Color::rgba(0.0, 0.0, 0.0, 0.15),
            graph_lines_color: Color::rgba(1.0, 1.0, 1.0, 1.0),
            partial_fill_color: Color::rgba(1.0, 1.0, 1.0, 0.25),
            complete_fill_color: Color::rgba(1.0, 1.0, 1.0, 0.4),
            height: 80,
        }
    }
}

#[cfg(feature = "std")]
impl State {
    /// Encodes the state object's information as JSON.
    pub fn write_json<W>(&self, writer: W) -> serde_json::Result<()>
    where
        W: std::io::Write,
    {
        serde_json::to_writer(writer, self)
    }
}

/// Private struct to reduce the number of function arguments.
#[derive(Default)]
struct DrawInfo {
    points: Vec<Point>,
    /// The lowest delta value in seconds.
    min_delta: f32,
    /// The highest delta value in seconds.
    max_delta: f32,
    scale_factor_x: Option<f32>,
    scale_factor_y: Option<f32>,
    padding_y: f32,
    split_index: usize,
    flip_graph: bool,
    is_live_delta_active: bool,
}

#[derive(Default)]
struct GridLines {
    /// The offset of the first grid line followed by the grid line distance.
    horizontal: Option<[f32; 2]>,
    vertical: Option<f32>,
}

impl Component {
    /// Creates a new Graph Component.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new Graph Component with the given settings.
    pub const fn with_settings(settings: Settings) -> Self {
        Self { settings }
    }

    /// Accesses the settings of the component.
    pub const fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Grants mutable access to the settings of the component.
    pub const fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    /// Accesses the name of the component.
    pub fn name(&self) -> Cow<'static, str> {
        self.text(
            self.settings
                .comparison_override
                .as_ref()
                .map(String::as_ref),
        )
    }

    fn text(&self, comparison: Option<&str>) -> Cow<'static, str> {
        if let Some(comparison) = comparison {
            format!("Graph ({})", comparison::shorten(comparison)).into()
        } else {
            "Graph".into()
        }
    }

    /// Updates the component's state based on the timer and layout settings
    /// provided.
    pub fn update_state(
        &self,
        state: &mut State,
        timer: &Snapshot,
        layout_settings: &GeneralLayoutSettings,
    ) {
        let mut draw_info = DrawInfo {
            flip_graph: self.settings.flip_graph,
            ..DrawInfo::default()
        };

        let x_axis = self
            .calculate_graph(timer, &mut draw_info)
            .unwrap_or(DEFAULT_X_AXIS);

        if draw_info.points.is_empty() {
            draw_info.points.push(Point {
                x: 0.0,
                y: DEFAULT_X_AXIS,
                is_best_segment: false,
            });
        }

        let grid_lines = calculate_grid_lines(&draw_info, x_axis);
        update_grid_line_vecs(state, grid_lines);
        self.copy_settings_to_state(state);
        state.best_segment_color = layout_settings.best_segment_color;
        state.middle = x_axis;
        state.is_live_delta_active = draw_info.is_live_delta_active;
        state.points = draw_info.points;
        state.updates_frequently = timer
            .current_phase()
            .updates_frequently(timer.current_timing_method());
    }

    /// Calculates the component's state based on the timer and layout settings
    /// provided.
    pub fn state(&self, timer: &Snapshot, layout_settings: &GeneralLayoutSettings) -> State {
        let mut state = State::default();
        self.update_state(&mut state, timer, layout_settings);
        state
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                "Comparison".into(),
                "The comparison to use for the graph. If not specified, the current comparison is used.".into(),
                self.settings.comparison_override.clone().into(),
            ),
            Field::new(
                "Height".into(),
                "The height of the chart.".into(),
                u64::from(self.settings.height).into(),
            ),
            Field::new(
                "Show Best Segments".into(),
                "Specifies whether to color the best segments with the layout's best segment color.".into(),
                self.settings.show_best_segments.into(),
            ),
            Field::new(
                "Live Graph".into(),
                "Specifies whether the graph should automatically refresh all the time. If this is deactivated, changes to the graph only happen whenever the current segment changes.".into(),
                self.settings.live_graph.into(),
            ),
            Field::new(
                "Flip Graph".into(),
                "Specifies whether the chart should be flipped vertically. If not enabled, split times which are ahead of the comparison are displayed below the x-axis and times which are behind are above it. Enabling this settings flips it.".into(),
                self.settings.flip_graph.into(),
            ),
            Field::new(
                "Behind Background Color".into(),
                "The background color for the chart region containing the times that are behind the comparison.".into(),
                self.settings.behind_background_color.into(),
            ),
            Field::new(
                "Ahead Background Color".into(),
                "The background color for the chart region containing the times that are ahead of the comparison.".into(),
                self.settings.ahead_background_color.into(),
            ),
            Field::new(
                "Grid Lines Color".into(),
                "The color of the chart's grid lines.".into(),
                self.settings.grid_lines_color.into(),
            ),
            Field::new(
                "Graph Lines Color".into(),
                "The color of the lines connecting the graph's points.".into(),
                self.settings.graph_lines_color.into(),
            ),
            Field::new(
                "Partial Fill Color".into(),
                "The color of the region enclosed by the x-axis and the graph. The partial fill color is only used for live changes. More specifically, this color is used in the interval from the last split time to the current time.".into(),
                self.settings.partial_fill_color.into(),
            ),
            Field::new(
                "Complete Fill Color".into(),
                "The color of the region enclosed by the x-axis and the graph, excluding the graph segment with live changes.".into(),
                self.settings.complete_fill_color.into(),
            ),
        ])
    }

    /// Sets a setting's value by its index to the given value.
    ///
    /// # Panics
    ///
    /// This panics if the type of the value to be set is not compatible with
    /// the type of the setting's value. A panic can also occur if the index of
    /// the setting provided is out of bounds.
    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.comparison_override = value.into(),
            1 => self.settings.height = value.into_uint().unwrap() as _,
            2 => self.settings.show_best_segments = value.into(),
            3 => self.settings.live_graph = value.into(),
            4 => self.settings.flip_graph = value.into(),
            5 => self.settings.behind_background_color = value.into(),
            6 => self.settings.ahead_background_color = value.into(),
            7 => self.settings.grid_lines_color = value.into(),
            8 => self.settings.graph_lines_color = value.into(),
            9 => self.settings.partial_fill_color = value.into(),
            10 => self.settings.complete_fill_color = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }

    fn calculate_graph(&self, timer: &Snapshot, draw_info: &mut DrawInfo) -> Option<f32> {
        let settings = &self.settings;
        draw_info.split_index = timer.current_split_index()?;
        let comparison = comparison::resolve(&self.settings.comparison_override, timer);
        let comparison = comparison::or_current(comparison, timer);

        calculate_horizontal_scaling(timer, draw_info, settings.live_graph);
        draw_info.scale_factor_x?;

        draw_info.points = Vec::with_capacity(draw_info.split_index + 1);
        draw_info.points.push(Point {
            x: 0.0,
            y: 0.0, // Not the final value of y, this will end up on the x-axis.
            is_best_segment: false,
        });

        calculate_split_points(timer, draw_info, comparison, settings.show_best_segments);
        if settings.live_graph {
            calculate_live_delta_point(timer, draw_info, comparison);
        }

        calculate_vertical_scaling(draw_info);
        let x_axis = calculate_x_axis(draw_info);

        transform_y_coordinates(draw_info);

        Some(x_axis)
    }

    const fn copy_settings_to_state(&self, state: &mut State) {
        let settings = &self.settings;
        (state.top_background_color, state.bottom_background_color) = if settings.flip_graph {
            (
                settings.ahead_background_color,
                settings.behind_background_color,
            )
        } else {
            (
                settings.behind_background_color,
                settings.ahead_background_color,
            )
        };

        state.is_flipped = settings.flip_graph;
        state.grid_lines_color = settings.grid_lines_color;
        state.graph_lines_color = settings.graph_lines_color;
        state.partial_fill_color = settings.partial_fill_color;
        state.complete_fill_color = settings.complete_fill_color;
        state.height = settings.height;
    }
}

fn calculate_horizontal_scaling(timer: &Snapshot, draw_info: &mut DrawInfo, live_graph: bool) {
    let timing_method = timer.current_timing_method();

    // final_split is the split time of a theoretical point on the right edge of
    // the chart.
    let mut final_split = 0.0;
    if live_graph {
        let current_time = timer.current_time();
        final_split = current_time[timing_method]
            .or(current_time.real_time)
            .unwrap_or_else(TimeSpan::zero)
            .total_seconds() as f32;
    } else {
        // Find the last segment with a split time.
        for segment in timer.run().segments()[..draw_info.split_index].iter().rev() {
            if let Some(time) = segment.split_time()[timing_method] {
                final_split = time.total_seconds() as f32;
                break;
            }
        }
    }

    if final_split > 0.0 {
        draw_info.scale_factor_x = Some(WIDTH / final_split);
    }

    // Else scaling doesn't matter and scale_factor_x stays None.
}

/// Calculates the points' x-coordinates and their deltas, which determine their
/// y-coordinates. The deltas are stored as the points' y-coordinates and will
/// have to be corrected before rendering.
fn calculate_split_points(
    timer: &Timer,
    draw_info: &mut DrawInfo,
    comparison: &str,
    show_best_segments: bool,
) {
    let timing_method = timer.current_timing_method();

    for (i, segment) in timer.run().segments()[..draw_info.split_index]
        .iter()
        .enumerate()
    {
        catch! {
            let split_time = segment.split_time()[timing_method]?;
            let comparison_time = segment.comparison(comparison)[timing_method]?;
            let delta = (split_time - comparison_time).total_seconds() as f32;

            if delta > draw_info.max_delta {
                draw_info.max_delta = delta;
            } else if delta < draw_info.min_delta {
                draw_info.min_delta = delta;
            }

            let x = split_time.total_seconds() as f32 * draw_info.scale_factor_x.unwrap_or(0.0);

            let is_best_segment =
                show_best_segments && analysis::check_best_segment(timer, i, timing_method);

            draw_info.points.push(Point {
                x,
                y: delta, // Not the final value of y.
                is_best_segment,
            });
        };
    }
}

fn calculate_live_delta_point(timer: &Snapshot, draw_info: &mut DrawInfo, comparison: &str) {
    if timer.current_phase() == TimerPhase::Ended {
        return;
    }

    let timing_method = timer.current_timing_method();
    let mut live_delta = analysis::check_live_delta(timer, true, comparison, timing_method);
    let current_time = timer.current_time()[timing_method];
    let current_split_comparison = timer
        .run()
        .segment(draw_info.split_index)
        .comparison(comparison)[timing_method];

    if let Some(current_time) = current_time
        && let Some(current_split_comparison) = current_split_comparison
        && let None = live_delta
    {
        // Live delta should be shown despite what analysis::check_live_delta says.
        let delta = current_time - current_split_comparison;
        if delta.total_seconds() as f32 > draw_info.min_delta {
            live_delta = Some(delta);
        }
    }

    if let Some(live_delta) = live_delta {
        let delta = live_delta.total_seconds() as f32;
        if delta > draw_info.max_delta {
            draw_info.max_delta = delta;
        } else if delta < draw_info.min_delta {
            draw_info.min_delta = delta;
        }

        draw_info.points.push(Point {
            x: WIDTH,
            y: delta, // Not the final value of y.
            is_best_segment: false,
        });
        draw_info.is_live_delta_active = true;
    }
}

/// Calculates the size of the chart's padding and its vertical scale factor.
/// The padding is an area at the top/bottom that stays empty so that the graph
/// doesn't touch the edge of the chart. This value depends on
/// `min_`/`max_delta` because otherwise the scale factor would be huge for
/// small graphs (graphs with only small deltas).
fn calculate_vertical_scaling(draw_info: &mut DrawInfo) {
    const MIN_PADDING: f32 = HEIGHT / 24.0;
    const MAX_CONTENT_HEIGHT: f32 = HEIGHT - MIN_PADDING * 2.0;
    // The bigger this value, the longer it will take for padding_y to get close
    // to MIN_PADDING.
    const SMOOTHNESS: f32 = 0.2;

    let total_delta = draw_info.max_delta - draw_info.min_delta;
    if total_delta > 0.0 {
        // A hyperbola works well, this looks something like f(x) = 1/(x + 2)
        draw_info.padding_y =
            MAX_CONTENT_HEIGHT * SMOOTHNESS / (total_delta + SMOOTHNESS * 2.0) + MIN_PADDING;

        let content_height = HEIGHT - draw_info.padding_y * 2.0;
        draw_info.scale_factor_y = Some(content_height / total_delta);
    }

    // Else padding_y stays 0 and scale_factor_y stays None, because vertical
    // scaling doesn't matter if all the points are at y=0.
}

fn calculate_x_axis(draw_info: &DrawInfo) -> f32 {
    if let Some(scale_factor_y) = draw_info.scale_factor_y {
        let x_axis = draw_info.max_delta * scale_factor_y + draw_info.padding_y;
        if draw_info.flip_graph {
            HEIGHT - x_axis
        } else {
            x_axis
        }
    } else {
        DEFAULT_X_AXIS
    }
}

fn calculate_grid_lines(draw_info: &DrawInfo, x_axis: f32) -> GridLines {
    // Initially, the grid lines are all one second apart. Once a certain amount
    // of lines is on screen, that number of seconds increases.

    // When to reduce the amount of grid lines.
    const REDUCE_LINES_THRESHOLD_HORIZONTAL: f32 = HEIGHT / 6.0;
    const REDUCE_LINES_THRESHOLD_VERTICAL: f32 = HEIGHT / 9.0;
    // How much bigger the distance between the lines should get.
    const LINE_DISTANCE_FACTOR: f32 = 6.0;

    let mut ret = GridLines::default();
    if let Some(scale_factor_y) = draw_info.scale_factor_y {
        let mut distance = scale_factor_y;
        while distance < REDUCE_LINES_THRESHOLD_HORIZONTAL {
            distance *= LINE_DISTANCE_FACTOR;
        }

        // The x-axis should always be on a grid line.
        let offset = x_axis % distance;

        ret.horizontal = Some([offset, distance]);
    } else {
        // Show just one grid line, the x-axis.
        ret.horizontal = Some([DEFAULT_X_AXIS, f32::INFINITY]);
    }

    if let Some(scale_factor_x) = draw_info.scale_factor_x {
        let mut distance = scale_factor_x;
        while distance < REDUCE_LINES_THRESHOLD_VERTICAL {
            distance *= LINE_DISTANCE_FACTOR;
        }

        ret.vertical = Some(distance);
    }

    ret
}

/// Copies the information from `grid_lines` into `Vec`s.
fn update_grid_line_vecs(state: &mut State, grid_lines: GridLines) {
    state.horizontal_grid_lines.clear();
    if let Some([offset, distance]) = grid_lines.horizontal {
        let mut y = offset;
        while y < HEIGHT {
            state.horizontal_grid_lines.push(y);
            y += distance;
        }
    }

    state.vertical_grid_lines.clear();
    if let Some(distance) = grid_lines.vertical {
        let mut x = distance;
        while x < WIDTH {
            state.vertical_grid_lines.push(x);
            x += distance;
        }
    }
}

/// Before calling this function, the deltas are stored as the points'.
/// y-coordinates. This will calculate the actual y-coordinates and replace the
/// deltas. The reason why this can't be done in the first loop is that
/// `min_`/`max_delta` is not known yet at that point in time.
fn transform_y_coordinates(draw_info: &mut DrawInfo) {
    if let Some(scale_factor_y) = draw_info.scale_factor_y {
        for point in &mut draw_info.points {
            let delta = point.y;
            point.y = (draw_info.max_delta - delta) * scale_factor_y + draw_info.padding_y;
            if draw_info.flip_graph {
                point.y = HEIGHT - point.y;
            }
        }
    } else {
        for point in &mut draw_info.points {
            point.y = DEFAULT_X_AXIS;
        }
    }
}
