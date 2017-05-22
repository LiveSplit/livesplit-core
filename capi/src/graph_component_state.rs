use livesplit_core::component::graph::State as GraphComponentState;
use super::{own_drop, acc};

pub type OwnedGraphComponentState = *mut GraphComponentState;

#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_drop(this: OwnedGraphComponentState) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_points_len(this: *const GraphComponentState) -> usize {
    acc(this).points.len()
}

#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_point_x(this: *const GraphComponentState,
                                                     index: usize)
                                                     -> f32 {
    acc(this).points[index].0
}

#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_point_y(this: *const GraphComponentState,
                                                     index: usize)
                                                     -> f32 {
    acc(this).points[index].1
}

#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_horizontal_grid_lines_len(this: *const GraphComponentState) -> usize{
    acc(this).horizontal_grid_lines.len()
}

#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_horizontal_grid_line(this: *const GraphComponentState, index: usize) -> f32{
    acc(this).horizontal_grid_lines[index]
}

#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_vertical_grid_lines_len(this: *const GraphComponentState) -> usize{
    acc(this).vertical_grid_lines.len()
}

#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_vertical_grid_line(this: *const GraphComponentState,
                                                                index: usize)
                                                                -> f32 {
    acc(this).vertical_grid_lines[index]
}

#[no_mangle]
pub unsafe extern "C" fn GraphComponentState_middle(this: *const GraphComponentState) -> f32 {
    acc(this).middle
}
