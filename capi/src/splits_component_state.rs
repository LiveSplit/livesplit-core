//! The state object that describes a single segment's information to visualize.

use super::{output_str, output_vec};
use livesplit_core::component::splits::State as SplitsComponentState;
use std::{io::Write, os::raw::c_char};

/// type
pub type OwnedSplitsComponentState = Box<SplitsComponentState>;

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn SplitsComponentState_drop(this: OwnedSplitsComponentState) {
    drop(this);
}

/// Describes whether a more pronounced separator should be shown in front of
/// the last segment provided.
#[unsafe(no_mangle)]
pub extern "C" fn SplitsComponentState_final_separator_shown(this: &SplitsComponentState) -> bool {
    this.show_final_separator
}

/// Returns the amount of segments to visualize.
#[unsafe(no_mangle)]
pub extern "C" fn SplitsComponentState_len(this: &SplitsComponentState) -> usize {
    this.splits.len()
}

/// The icon of the segment. The associated image can be looked up in the image
/// cache. The image may be the empty image. This indicates that there is no
/// icon.
#[unsafe(no_mangle)]
pub extern "C" fn SplitsComponentState_icon(
    this: &SplitsComponentState,
    index: usize,
) -> *const c_char {
    output_str(this.splits[index].icon.format_str(&mut [0; 64]))
}

/// The name of the segment with the specified index. You may not provide an out
/// of bounds index.
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub extern "C" fn SplitsComponentState_columns_len(
    this: &SplitsComponentState,
    index: usize,
) -> usize {
    this.splits[index].columns.len()
}

/// The column's value to show for the split and column with the specified
/// index. The columns are specified from right to left. You may not provide an
/// out of bounds index.
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub extern "C" fn SplitsComponentState_is_current_split(
    this: &SplitsComponentState,
    index: usize,
) -> bool {
    this.splits[index].is_current_split
}

/// Describes if the columns have labels that are meant to be shown. If this is
/// `false`, no labels are supposed to be visualized.
#[unsafe(no_mangle)]
pub extern "C" fn SplitsComponentState_has_column_labels(this: &SplitsComponentState) -> bool {
    this.column_labels.is_some()
}

/// Returns the label of the column specified. The list is specified from right
/// to left. You may not provide an out of bounds index.
#[unsafe(no_mangle)]
pub extern "C" fn SplitsComponentState_column_label(
    this: &SplitsComponentState,
    index: usize,
) -> *const c_char {
    output_str(if let Some(labels) = &this.column_labels {
        &labels[index]
    } else {
        ""
    })
}
