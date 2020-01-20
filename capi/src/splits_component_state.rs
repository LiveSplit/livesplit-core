//! The state object that describes a single segment's information to visualize.

use super::{output_str, output_vec};
use livesplit_core::component::splits::State as SplitsComponentState;
use std::io::Write;
use std::os::raw::c_char;

/// type
pub type OwnedSplitsComponentState = Box<SplitsComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn SplitsComponentState_drop(this: OwnedSplitsComponentState) {
    drop(this);
}

/// Describes whether a more pronounced separator should be shown in front of
/// the last segment provided.
#[no_mangle]
pub extern "C" fn SplitsComponentState_final_separator_shown(this: &SplitsComponentState) -> bool {
    this.show_final_separator
}

/// Returns the amount of segments to visualize.
#[no_mangle]
pub extern "C" fn SplitsComponentState_len(this: &SplitsComponentState) -> usize {
    this.splits.len()
}

/// Returns the amount of icon changes that happened in this state object.
#[no_mangle]
pub extern "C" fn SplitsComponentState_icon_change_count(this: &SplitsComponentState) -> usize {
    this.icon_changes.len()
}

/// Accesses the index of the segment of the icon change with the specified
/// index. This is based on the index in the run, not on the index of the
/// SplitState in the State object. The corresponding index is the index field
/// of the SplitState object. You may not provide an out of bounds index.
#[no_mangle]
pub extern "C" fn SplitsComponentState_icon_change_segment_index(
    this: &SplitsComponentState,
    icon_change_index: usize,
) -> usize {
    this.icon_changes[icon_change_index].segment_index
}

/// The icon data of the segment of the icon change with the specified index.
/// The buffer may be empty. This indicates that there is no icon. You may not
/// provide an out of bounds index.
#[no_mangle]
pub extern "C" fn SplitsComponentState_icon_change_icon_ptr(
    this: &SplitsComponentState,
    icon_change_index: usize,
) -> *const u8 {
    this.icon_changes[icon_change_index].icon.as_ptr()
}

/// The length of the icon data of the segment of the icon change with the
/// specified index.
#[no_mangle]
pub extern "C" fn SplitsComponentState_icon_change_icon_len(
    this: &SplitsComponentState,
    icon_change_index: usize,
) -> usize {
    this.icon_changes[icon_change_index].icon.len()
}

/// The name of the segment with the specified index. You may not provide an out
/// of bounds index.
#[no_mangle]
pub extern "C" fn SplitsComponentState_name(
    this: &SplitsComponentState,
    index: usize,
) -> *const c_char {
    output_str(&this.splits[index].name)
}

/// The amount of columns to visualize for the segment with the specified index.
/// The columns are specified from right to left. You may not provide an out of
/// bounds index. The amount of columns to visualize may differ from segment to
/// segment.
#[no_mangle]
pub extern "C" fn SplitsComponentState_columns_len(
    this: &SplitsComponentState,
    index: usize,
) -> usize {
    this.splits[index].columns.len()
}

/// The column's value to show for the split and column with the specified
/// index. The columns are specified from right to left. You may not provide an
/// out of bounds index.
#[no_mangle]
pub extern "C" fn SplitsComponentState_column_value(
    this: &SplitsComponentState,
    index: usize,
    column_index: usize,
) -> *const c_char {
    output_str(&this.splits[index].columns[column_index].value)
}

/// The semantic coloring information the column's value carries of the segment
/// and column with the specified index. The columns are specified from right to
/// left. You may not provide an out of bounds index.
#[no_mangle]
pub extern "C" fn SplitsComponentState_column_semantic_color(
    this: &SplitsComponentState,
    index: usize,
    column_index: usize,
) -> *const c_char {
    output_vec(|f| {
        write!(
            f,
            "{:?}",
            this.splits[index].columns[column_index].semantic_color
        )
        .unwrap()
    })
}

/// Describes if the segment with the specified index is the segment the active
/// attempt is currently on.
#[no_mangle]
pub extern "C" fn SplitsComponentState_is_current_split(
    this: &SplitsComponentState,
    index: usize,
) -> bool {
    this.splits[index].is_current_split
}
