use {Timer, TimeSpan, TimerPhase, state_helper};
use serde_json::{to_writer, Result};
use std::io::Write;

const GRAPH_EDGE_VALUE: f32 = 200.0;
const GRAPH_EDGE_MIN: f32 = 5.0;

const WIDTH: f32 = 180.0;
const HEIGHT: f32 = 120.0;

#[derive(Default)]
pub struct Component {
    settings: Settings,
}

pub struct Settings {
    pub live_graph: bool,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub points: Vec<(f32, f32)>,
    pub horizontal_grid_lines: Vec<f32>,
    pub vertical_grid_lines: Vec<f32>,
    pub middle: f32,
    pub is_live_delta_active: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { live_graph: true }
    }
}

impl State {
    pub fn write_json<W>(&self, mut writer: W) -> Result<()>
        where W: Write
    {
        to_writer(&mut writer, self)
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

    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    pub fn state(&self, timer: &Timer) -> State {
        let mut draw_info = DrawInfo {
            deltas: Vec::with_capacity(timer.run().len() + 1),
            ..Default::default()
        };

        let comparison = timer.current_comparison();
        self.calculate_final_split(timer, &mut draw_info);
        self.calculate_deltas(timer, comparison, &mut draw_info);
        self.check_live_segment_delta(timer, comparison, &mut draw_info);

        let mut state = self.calculate_points(timer, &draw_info);

        self.make_uniform(&mut state);

        state
    }

    fn make_uniform(&self, state: &mut State) {
        for grid_line in &mut state.horizontal_grid_lines {
            *grid_line /= HEIGHT;
        }

        for grid_line in &mut state.vertical_grid_lines {
            *grid_line /= WIDTH;
        }

        state.middle /= HEIGHT;

        for &mut (ref mut x, ref mut y) in &mut state.points {
            *x /= WIDTH;
            *y /= HEIGHT;
        }
    }

    fn calculate_points(&self, timer: &Timer, draw_info: &DrawInfo) -> State {
        let total_delta = draw_info.min_delta - draw_info.max_delta;

        let (graph_edge, graph_height, middle) =
            self.calculate_middle_and_graph_edge(total_delta, draw_info);

        let (grid_value_x, grid_value_y) =
            self.calculate_grid_lines(timer, total_delta, graph_edge, graph_height, draw_info);

        let (horizontal_grid_lines, vertical_grid_lines) =
            self.make_grid_lines_list(graph_height, middle, grid_value_x, grid_value_y);

        let points = self.make_points_list(draw_info, timer, total_delta, graph_edge, graph_height);

        State {
            points,
            horizontal_grid_lines,
            vertical_grid_lines,
            middle,
            is_live_delta_active: draw_info.is_live_delta_active,
        }
    }

    fn make_points_list(&self,
                        draw_info: &DrawInfo,
                        timer: &Timer,
                        total_delta: TimeSpan,
                        graph_edge: f32,
                        graph_height: f32)
                        -> Vec<(f32, f32)> {
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

            points_list.push((width_one, height_one));

            for (y, &delta) in draw_info.deltas.iter().enumerate() {
                if let Some(delta) = delta {
                    self.calculate_right_side_coordinates(draw_info,
                                                          timer,
                                                          total_delta,
                                                          graph_edge,
                                                          graph_height,
                                                          delta,
                                                          &mut height_two,
                                                          &mut width_two,
                                                          y);

                    points_list.push((width_two, height_two));

                    self.calculate_left_side_coordinates(draw_info,
                                                         timer,
                                                         total_delta,
                                                         graph_edge,
                                                         graph_height,
                                                         delta,
                                                         &mut height_one,
                                                         &mut width_one,
                                                         y);
                }
            }
        }
        points_list
    }

    fn calculate_left_side_coordinates(&self,
                                       draw_info: &DrawInfo,
                                       timer: &Timer,
                                       total_delta: TimeSpan,
                                       graph_edge: f32,
                                       graph_height: f32,
                                       delta: TimeSpan,
                                       height_one: &mut f32,
                                       width_one: &mut f32,
                                       y: usize) {
        if total_delta != TimeSpan::zero() {
            *height_one = (delta.total_milliseconds() as f32 -
                           draw_info.max_delta.total_milliseconds() as f32) /
                          total_delta.total_milliseconds() as f32 *
                          (graph_height - graph_edge) * 2.0 + graph_edge;
        } else {
            *height_one = graph_height;
        }

        if y + 1 != draw_info.deltas.len() {
            if let Some(split_time) = timer.run().segment(y).split_time()
                   [timer.current_timing_method()] {
                *width_one = (split_time.total_milliseconds() as f32 /
                              draw_info.final_split.total_milliseconds() as f32) *
                             WIDTH;
            }
        }
    }

    fn calculate_right_side_coordinates(&self,
                                        draw_info: &DrawInfo,
                                        timer: &Timer,
                                        total_delta: TimeSpan,
                                        graph_edge: f32,
                                        graph_height: f32,
                                        delta: TimeSpan,
                                        height_two: &mut f32,
                                        width_two: &mut f32,
                                        y: usize) {
        if y + 1 == draw_info.deltas.len() && draw_info.is_live_delta_active {
            *width_two = WIDTH;
        } else if let Some(split_time) = timer.run().segment(y).split_time()
                      [timer.current_timing_method()] {
            *width_two = (split_time.total_milliseconds() as f32 /
                          draw_info.final_split.total_milliseconds() as f32) *
                         WIDTH;
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

    fn make_grid_lines_list(&self,
                            graph_height: f32,
                            middle: f32,
                            grid_value_x: f32,
                            grid_value_y: f32)
                            -> (Vec<f32>, Vec<f32>) {
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

    fn calculate_grid_lines(&self,
                            timer: &Timer,
                            total_delta: TimeSpan,
                            graph_edge: f32,
                            graph_height: f32,
                            draw_info: &DrawInfo)
                            -> (f32, f32) {
        let (mut grid_value_x, mut grid_value_y);

        let current_phase = timer.current_phase();
        if current_phase != TimerPhase::NotRunning && draw_info.final_split > TimeSpan::zero() {
            grid_value_x = 1000.0;
            while draw_info.final_split.total_milliseconds() as f32 / grid_value_x > WIDTH / 20.0 {
                grid_value_x *= 6.0;
            }
            grid_value_x = (grid_value_x / draw_info.final_split.total_milliseconds() as f32) *
                           WIDTH;
        } else {
            grid_value_x = -1.0;
        }
        if current_phase != TimerPhase::NotRunning && total_delta < TimeSpan::zero() {
            grid_value_y = 1000.0;
            while (-total_delta.total_milliseconds() as f32) / grid_value_y >
                  (graph_height - graph_edge) * 2.0 / 20.0 {
                grid_value_y *= 6.0;
            }
            grid_value_y = (grid_value_y / (-total_delta.total_milliseconds() as f32)) *
                           (graph_height - graph_edge) * 2.0;
        } else {
            grid_value_y = -1.0;
        }

        (grid_value_x, grid_value_y)
    }

    fn calculate_middle_and_graph_edge(&self,
                                       total_delta: TimeSpan,
                                       draw_info: &DrawInfo)
                                       -> (f32, f32, f32) {
        let mut graph_edge = 0.0;
        let graph_height = HEIGHT / 2.0; // TODO Make const
        let mut middle = graph_height;
        if total_delta != TimeSpan::zero() {
            graph_edge = GRAPH_EDGE_VALUE /
                         (-total_delta.total_milliseconds() as f32 + 2.0 * GRAPH_EDGE_VALUE) *
                         (graph_height * 2.0 - GRAPH_EDGE_MIN * 2.0);
            graph_edge += GRAPH_EDGE_MIN;
            middle = (-(draw_info.max_delta.total_milliseconds() as f32 /
                        total_delta.total_milliseconds() as f32)) *
                     (graph_height - graph_edge) * 2.0 + graph_edge;
        }
        (graph_edge, graph_height, middle)
    }

    fn calculate_final_split(&self, timer: &Timer, draw_info: &mut DrawInfo) {
        if self.settings.live_graph {
            if timer.current_phase() != TimerPhase::NotRunning {
                let current_time = timer.current_time();
                let timing_method = timer.current_timing_method();
                draw_info.final_split = current_time[timing_method]
                    .or_else(|| current_time.real_time)
                    .unwrap_or_else(TimeSpan::zero);
            }
        } else {
            let timing_method = timer.current_timing_method();
            for segment in timer.run().segments() {
                if let Some(time) = segment.split_time()[timing_method] {
                    draw_info.final_split = time;
                    return;
                }
            }
        }
    }

    fn calculate_deltas(&self, timer: &Timer, comparison: &str, draw_info: &mut DrawInfo) {
        let timing_method = timer.current_timing_method();
        for segment in timer.run().segments() {
            let time = TimeSpan::option_op(segment.split_time()[timing_method],
                                           segment.comparison(comparison)[timing_method],
                                           |a, b| a - b);
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
                    state_helper::check_live_delta(timer, true, comparison, timing_method);
                // TODO Try if let instead of checking current phase up there, so we can skip this unwrap
                let current_split = timer.current_split().unwrap().comparison(comparison)
                    [timing_method];
                let current_time = timer.current_time()[timing_method];
                if let (Some(current_time), Some(current_split), None) =
                    (current_time, current_split, best_segment) {
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
