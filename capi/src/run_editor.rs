use livesplit_core::{Run, RunEditor, TimingMethod};
use super::{acc, acc_mut, alloc, output_vec, own, str, Json};
use run::OwnedRun;
use sum_of_best_cleaner::OwnedSumOfBestCleaner;
use libc::c_char;
use std::{ptr, slice};

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
    output_vec(|o| { acc_mut(&this).state().write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_select_timing_method(
    this: *mut RunEditor,
    method: TimingMethod,
) {
    acc_mut(&this).select_timing_method(method);
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_unselect(this: *mut RunEditor, index: usize) {
    acc_mut(&this).unselect(index);
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_select_additionally(this: *mut RunEditor, index: usize) {
    acc_mut(&this).select_additionally(index);
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_select_only(this: *mut RunEditor, index: usize) {
    acc_mut(&this).select_only(index);
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_set_game_name(this: *mut RunEditor, game: *const c_char) {
    acc_mut(&this).set_game_name(str(&game));
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_set_category_name(
    this: *mut RunEditor,
    category: *const c_char,
) {
    acc_mut(&this).set_category_name(str(&category));
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_parse_and_set_offset(
    this: *mut RunEditor,
    offset: *const c_char,
) -> bool {
    acc_mut(&this).parse_and_set_offset(str(&offset)).is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_parse_and_set_attempt_count(
    this: *mut RunEditor,
    attempts: *const c_char,
) -> bool {
    acc_mut(&this)
        .parse_and_set_attempt_count(str(&attempts))
        .is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_set_game_icon(
    this: *mut RunEditor,
    data: *const u8,
    length: usize,
) {
    acc_mut(&this).set_game_icon(slice::from_raw_parts(data, length));
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_remove_game_icon(this: *mut RunEditor) {
    acc_mut(&this).remove_game_icon();
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_insert_segment_above(this: *mut RunEditor) {
    acc_mut(&this).insert_segment_above();
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_insert_segment_below(this: *mut RunEditor) {
    acc_mut(&this).insert_segment_below();
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_remove_segments(this: *mut RunEditor) {
    acc_mut(&this).remove_segments();
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_move_segments_up(this: *mut RunEditor) {
    acc_mut(&this).move_segments_up();
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_move_segments_down(this: *mut RunEditor) {
    acc_mut(&this).move_segments_down();
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_selected_set_icon(
    this: *mut RunEditor,
    data: *const u8,
    length: usize,
) {
    acc_mut(&this)
        .selected_segment()
        .set_icon(slice::from_raw_parts(data, length));
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_selected_remove_icon(this: *mut RunEditor) {
    acc_mut(&this).selected_segment().remove_icon();
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_selected_set_name(this: *mut RunEditor, name: *const c_char) {
    acc_mut(&this).selected_segment().set_name(str(&name));
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_selected_parse_and_set_split_time(
    this: *mut RunEditor,
    time: *const c_char,
) -> bool {
    acc_mut(&this)
        .selected_segment()
        .parse_and_set_split_time(str(&time))
        .is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_selected_parse_and_set_segment_time(
    this: *mut RunEditor,
    time: *const c_char,
) -> bool {
    acc_mut(&this)
        .selected_segment()
        .parse_and_set_segment_time(str(&time))
        .is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_selected_parse_and_set_best_segment_time(
    this: *mut RunEditor,
    time: *const c_char,
) -> bool {
    acc_mut(&this)
        .selected_segment()
        .parse_and_set_best_segment_time(str(&time))
        .is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_selected_parse_and_set_comparison_time(
    this: *mut RunEditor,
    comparison: *const c_char,
    time: *const c_char,
) -> bool {
    acc_mut(&this)
        .selected_segment()
        .parse_and_set_comparison_time(str(&comparison), str(&time))
        .is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_add_comparison(
    this: *mut RunEditor,
    comparison: *const c_char,
) -> bool {
    acc_mut(&this).add_comparison(str(&comparison)).is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_import_comparison(
    this: *mut RunEditor,
    run: *const Run,
    comparison: *const c_char,
) -> bool {
    acc_mut(&this)
        .import_comparison(acc(&run), str(&comparison))
        .is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_remove_comparison(
    this: *mut RunEditor,
    comparison: *const c_char,
) {
    acc_mut(&this).remove_comparison(str(&comparison));
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_rename_comparison(
    this: *mut RunEditor,
    old_name: *const c_char,
    new_name: *const c_char,
) -> bool {
    acc_mut(&this)
        .rename_comparison(str(&old_name), str(&new_name))
        .is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_clear_history(this: *mut RunEditor) {
    acc_mut(&this).clear_history();
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_clear_times(this: *mut RunEditor) {
    acc_mut(&this).clear_times();
}

/// # Safety
/// `this` must outlive `OwnedSumOfBestCleaner`
#[no_mangle]
pub unsafe extern "C" fn RunEditor_clean_sum_of_best<'a>(
    this: *mut RunEditor,
) -> OwnedSumOfBestCleaner<'a> {
    alloc((&mut *this).clean_sum_of_best())
}
