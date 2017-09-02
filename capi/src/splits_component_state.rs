use livesplit_core::component::splits::State as SplitsComponentState;
use super::{acc, output_str, output_str_with, own_drop};
use libc::c_char;
use std::fmt::Write;

pub type OwnedSplitsComponentState = *mut SplitsComponentState;

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_drop(this: OwnedSplitsComponentState) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_final_separator_shown(
    this: *const SplitsComponentState,
) -> bool {
    acc(this).show_final_separator
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_len(this: *const SplitsComponentState) -> usize {
    acc(this).splits.len()
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_icon_change_count(
    this: *const SplitsComponentState,
) -> usize {
    acc(this).icon_changes.len()
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_icon_change_segment_index(
    this: *const SplitsComponentState,
    index: usize,
) -> usize {
    acc(this).icon_changes[index].segment_index
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_icon_change_icon(
    this: *const SplitsComponentState,
    index: usize,
) -> *const c_char {
    output_str(&acc(this).icon_changes[index].icon)
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_name(
    this: *const SplitsComponentState,
    index: usize,
) -> *const c_char {
    output_str(&acc(this).splits[index].name)
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_delta(
    this: *const SplitsComponentState,
    index: usize,
) -> *const c_char {
    output_str(&acc(this).splits[index].delta)
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_time(
    this: *const SplitsComponentState,
    index: usize,
) -> *const c_char {
    output_str(&acc(this).splits[index].time)
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_semantic_color(
    this: *const SplitsComponentState,
    index: usize,
) -> *const c_char {
    output_str_with(|f| {
        write!(f, "{:?}", acc(this).splits[index].semantic_color).unwrap()
    })
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_is_current_split(
    this: *const SplitsComponentState,
    index: usize,
) -> bool {
    acc(this).splits[index].is_current_split
}
