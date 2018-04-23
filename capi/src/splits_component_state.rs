//! The state object that describes a single segment's information to visualize.

use super::{acc, output_str, output_vec, own_drop};
use livesplit_core::component::splits::State as SplitsComponentState;
use std::io::Write;
use std::os::raw::c_char;

/// type
pub type OwnedSplitsComponentState = *mut SplitsComponentState;

/// drop
#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_drop(this: OwnedSplitsComponentState) {
    own_drop(this);
}

/// Describes whether a more pronounced separator should be shown in front of
/// the last segment provided.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_final_separator_shown(
    this: *const SplitsComponentState,
) -> bool {
    acc(this).show_final_separator
}

/// Returns the amount of segments to visualize.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_len(this: *const SplitsComponentState) -> usize {
    acc(this).splits.len()
}

/// Returns the amount of icon changes that happened in this state object.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_icon_change_count(
    this: *const SplitsComponentState,
) -> usize {
    acc(this).icon_changes.len()
}

/// Accesses the index of the segment of the icon change with the specified
/// index. This is based on the index in the run, not on the index of the
/// SplitState in the State object. The corresponding index is the index field
/// of the SplitState object. You may not provide an out of bounds index.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_icon_change_segment_index(
    this: *const SplitsComponentState,
    icon_change_index: usize,
) -> usize {
    acc(this).icon_changes[icon_change_index].segment_index
}

/// The segment's icon encoded as a Data URL of the icon change with the
/// specified index. The String itself may be empty. This indicates that there
/// is no icon. You may not provide an out of bounds index.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_icon_change_icon(
    this: *const SplitsComponentState,
    icon_change_index: usize,
) -> *const c_char {
    output_str(&acc(this).icon_changes[icon_change_index].icon)
}

/// The name of the segment with the specified index. You may not provide an out
/// of bounds index.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_name(
    this: *const SplitsComponentState,
    index: usize,
) -> *const c_char {
    output_str(&acc(this).splits[index].name)
}

/// The delta to show for the segment with the specified index. You may not
/// provide an out of bounds index.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_delta(
    this: *const SplitsComponentState,
    index: usize,
) -> *const c_char {
    output_str(&acc(this).splits[index].delta)
}

/// The split time to show for the segment with the specified index. You may not
/// provide an out of bounds index.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_time(
    this: *const SplitsComponentState,
    index: usize,
) -> *const c_char {
    output_str(&acc(this).splits[index].time)
}

/// The semantic coloring information the delta time carries of the segment with
/// the specified index. You may not provide an out of bounds index.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_semantic_color(
    this: *const SplitsComponentState,
    index: usize,
) -> *const c_char {
    output_vec(|f| write!(f, "{:?}", acc(this).splits[index].semantic_color).unwrap())
}

/// Describes if the segment with the specified index is the segment the active
/// attempt is currently on.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_is_current_split(
    this: *const SplitsComponentState,
    index: usize,
) -> bool {
    acc(this).splits[index].is_current_split
}
