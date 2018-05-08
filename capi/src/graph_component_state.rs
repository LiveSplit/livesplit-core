//! The state object describes the information to visualize for this component.
//! All the coordinates are in the range 0..1.

use livesplit_core::component::graph::State as GraphComponentState;

/// type
pub type OwnedGraphComponentState = Box<GraphComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn GraphComponentState_drop(this: OwnedGraphComponentState) {
    drop(this);
}

/// Returns the amount of points to visualize. Connect all of them to visualize
/// the graph. If the live delta is active, the last point is to be interpreted
/// as a preview of the next split that is about to happen. Use the partial fill
/// color to visualize the region beneath that graph segment.
#[no_mangle]
pub extern "C" fn GraphComponentState_points_len(this: &GraphComponentState) -> usize {
    this.points.len()
}

/// Returns the x coordinate of the point specified. You may not provide an out
/// of bounds index.
#[no_mangle]
pub extern "C" fn GraphComponentState_point_x(this: &GraphComponentState, index: usize) -> f32 {
    this.points[index].x
}

/// Returns the y coordinate of the point specified. You may not provide an out
/// of bounds index.
#[no_mangle]
pub extern "C" fn GraphComponentState_point_y(this: &GraphComponentState, index: usize) -> f32 {
    this.points[index].y
}

/// Describes whether the segment the point specified is visualizing achieved a
/// new best segment time. Use the best segment color for it, in that case. You
/// may not provide an out of bounds index.
#[no_mangle]
pub extern "C" fn GraphComponentState_point_is_best_segment(
    this: &GraphComponentState,
    index: usize,
) -> bool {
    this.points[index].is_best_segment
}

/// Describes how many horizontal grid lines to visualize.
#[no_mangle]
pub extern "C" fn GraphComponentState_horizontal_grid_lines_len(
    this: &GraphComponentState,
) -> usize {
    this.horizontal_grid_lines.len()
}

/// Accesses the y coordinate of the horizontal grid line specified. You may not
/// provide an out of bounds index.
#[no_mangle]
pub extern "C" fn GraphComponentState_horizontal_grid_line(
    this: &GraphComponentState,
    index: usize,
) -> f32 {
    this.horizontal_grid_lines[index]
}

/// Describes how many vertical grid lines to visualize.
#[no_mangle]
pub extern "C" fn GraphComponentState_vertical_grid_lines_len(this: &GraphComponentState) -> usize {
    this.vertical_grid_lines.len()
}

/// Accesses the x coordinate of the vertical grid line specified. You may not
/// provide an out of bounds index.
#[no_mangle]
pub extern "C" fn GraphComponentState_vertical_grid_line(
    this: &GraphComponentState,
    index: usize,
) -> f32 {
    this.vertical_grid_lines[index]
}

/// The y coordinate that separates the region that shows the times that are
/// ahead of the comparison and those that are behind.
#[no_mangle]
pub extern "C" fn GraphComponentState_middle(this: &GraphComponentState) -> f32 {
    this.middle
}

/// If the live delta is active, the last point is to be interpreted as a
/// preview of the next split that is about to happen. Use the partial fill
/// color to visualize the region beneath that graph segment.
#[no_mangle]
pub extern "C" fn GraphComponentState_is_live_delta_active(this: &GraphComponentState) -> bool {
    this.is_live_delta_active
}

/// Describes whether the graph is flipped vertically. For visualizing the
/// graph, this usually doesn't need to be interpreted, as this information is
/// entirely encoded into the other variables.
#[no_mangle]
pub extern "C" fn GraphComponentState_is_flipped(this: &GraphComponentState) -> bool {
    this.is_flipped
}
