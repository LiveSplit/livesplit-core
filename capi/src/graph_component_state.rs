//! The state object describes the information to visualize for this component.
//! All the coordinates are in the range 0..1.

use super::{acc, own_drop};
use livesplit_core::component::graph::State as GraphComponentState;

/// type
pub type OwnedGraphComponentState = *mut GraphComponentState;

/// drop
#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_drop(this: OwnedGraphComponentState) {
    own_drop(this);
}

/// Returns the amount of points to visualize. Connect all of them to visualize
/// the graph. If the live delta is active, the last point is to be interpreted
/// as a preview of the next split that is about to happen. Use the partial fill
/// color to visualize the region beneath that graph segment.
#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_points_len(this: *const GraphComponentState) -> usize {
    acc(this).points.len()
}

/// Returns the x coordinate of the point specified. You may not provide an out
/// of bounds index.
#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_point_x(
    this: *const GraphComponentState,
    index: usize,
) -> f32 {
    acc(this).points[index].x
}

/// Returns the y coordinate of the point specified. You may not provide an out
/// of bounds index.
#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_point_y(
    this: *const GraphComponentState,
    index: usize,
) -> f32 {
    acc(this).points[index].y
}

/// Describes whether the segment the point specified is visualizing achieved a
/// new best segment time. Use the best segment color for it, in that case. You
/// may not provide an out of bounds index.
#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_point_is_best_segment(
    this: *const GraphComponentState,
    index: usize,
) -> bool {
    acc(this).points[index].is_best_segment
}

/// Describes how many horizontal grid lines to visualize.
#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_horizontal_grid_lines_len(
    this: *const GraphComponentState,
) -> usize {
    acc(this).horizontal_grid_lines.len()
}

/// Accesses the y coordinate of the horizontal grid line specified. You may not
/// provide an out of bounds index.
#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_horizontal_grid_line(
    this: *const GraphComponentState,
    index: usize,
) -> f32 {
    acc(this).horizontal_grid_lines[index]
}

/// Describes how many vertical grid lines to visualize.
#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_vertical_grid_lines_len(
    this: *const GraphComponentState,
) -> usize {
    acc(this).vertical_grid_lines.len()
}

/// Accesses the x coordinate of the vertical grid line specified. You may not
/// provide an out of bounds index.
#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_vertical_grid_line(
    this: *const GraphComponentState,
    index: usize,
) -> f32 {
    acc(this).vertical_grid_lines[index]
}

/// The y coordinate that separates the region that shows the times that are
/// ahead of the comparison and those that are behind.
#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_middle(this: *const GraphComponentState) -> f32 {
    acc(this).middle
}

/// If the live delta is active, the last point is to be interpreted as a
/// preview of the next split that is about to happen. Use the partial fill
/// color to visualize the region beneath that graph segment.
#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_is_live_delta_active(
    this: *const GraphComponentState,
) -> bool {
    acc(this).is_live_delta_active
}

/// Describes whether the graph is flipped vertically. For visualizing the
/// graph, this usually doesn't need to be interpreted, as this information is
/// entirely encoded into the other variables.
#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_is_flipped(this: *const GraphComponentState) -> bool {
    acc(this).is_flipped
}
