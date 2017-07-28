use livesplit_core::{RunEditor, TimingMethod};
use super::{Json, alloc, own, output_vec, acc_mut, str};
use run::OwnedRun;
use libc::c_char;
use std::{slice, ptr};

pub type OwnedRunEditor = *mut RunEditor;
pub type NullableOwnedRunEditor = *mut RunEditor;

#[no_mangle]
pub unsafe extern "C" fn RunEditor_new(run: OwnedRun) -> NullableOwnedRunEditor {
    RunEditor::new(own(run))
        .ok()
        .map_or_else(ptr::null_mut, alloc)
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_close(this: OwnedRunEditor) -> OwnedRun {
    alloc(own(this).close())
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_state_as_json(this: *mut RunEditor) -> Json {
    output_vec(|o| { acc_mut(this).state().write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_select_timing_method(
    this: *mut RunEditor,
    method: TimingMethod,
) {
    acc_mut(this).select_timing_method(method);
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_unselect(this: *mut RunEditor, index: usize) {
    acc_mut(this).unselect(index);
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_select_additionally(this: *mut RunEditor, index: usize) {
    acc_mut(this).select_additionally(index);
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_select_only(this: *mut RunEditor, index: usize) {
    acc_mut(this).select_only(index);
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_set_game_name(this: *mut RunEditor, game: *const c_char) {
    acc_mut(this).set_game_name(str(game));
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_set_category_name(
    this: *mut RunEditor,
    category: *const c_char,
) {
    acc_mut(this).set_category_name(str(category));
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_parse_and_set_offset(
    this: *mut RunEditor,
    offset: *const c_char,
) -> bool {
    acc_mut(this).parse_and_set_offset(str(offset)).is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_parse_and_set_attempt_count(
    this: *mut RunEditor,
    attempts: *const c_char,
) -> bool {
    acc_mut(this)
        .parse_and_set_attempt_count(str(attempts))
        .is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_set_game_icon(
    this: *mut RunEditor,
    data: *const u8,
    length: usize,
) {
    acc_mut(this).set_game_icon(slice::from_raw_parts(data, length));
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_insert_segment_above(this: *mut RunEditor) {
    acc_mut(this).insert_segment_above();
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_insert_segment_below(this: *mut RunEditor) {
    acc_mut(this).insert_segment_below();
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_remove_segments(this: *mut RunEditor) {
    acc_mut(this).remove_segments();
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_move_segments_up(this: *mut RunEditor) {
    acc_mut(this).move_segments_up();
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_move_segments_down(this: *mut RunEditor) {
    acc_mut(this).move_segments_down();
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_selected_set_icon(
    this: *mut RunEditor,
    data: *const u8,
    length: usize,
) {
    acc_mut(this)
        .selected_segment()
        .set_icon(slice::from_raw_parts(data, length));
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_selected_set_name(this: *mut RunEditor, name: *const c_char) {
    acc_mut(this).selected_segment().set_name(str(name));
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_selected_parse_and_set_split_time(
    this: *mut RunEditor,
    time: *const c_char,
) -> bool {
    acc_mut(this)
        .selected_segment()
        .parse_and_set_split_time(str(time))
        .is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_selected_parse_and_set_segment_time(
    this: *mut RunEditor,
    time: *const c_char,
) -> bool {
    acc_mut(this)
        .selected_segment()
        .parse_and_set_segment_time(str(time))
        .is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_selected_parse_and_set_best_segment_time(
    this: *mut RunEditor,
    time: *const c_char,
) -> bool {
    acc_mut(this)
        .selected_segment()
        .parse_and_set_best_segment_time(str(time))
        .is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_selected_parse_and_set_comparison_time(
    this: *mut RunEditor,
    comparison: *const c_char,
    time: *const c_char,
) -> bool {
    acc_mut(this)
        .selected_segment()
        .parse_and_set_comparison_time(str(comparison), str(time))
        .is_ok()
}
