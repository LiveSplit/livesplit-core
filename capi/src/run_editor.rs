//! The Run Editor allows modifying Runs while ensuring that all the different
//! invariants of the Run objects are upheld no matter what kind of operations
//! are being applied to the Run. It provides the current state of the editor as
//! state objects that can be visualized by any kind of User Interface.

use super::{output_vec, str, Json};
use crate::{
    linked_layout::OwnedLinkedLayout, run::OwnedRun, slice,
    sum_of_best_cleaner::OwnedSumOfBestCleaner,
};
use livesplit_core::{
    settings::{Image, ImageCache},
    Run, RunEditor, TimingMethod,
};
use std::os::raw::c_char;

/// type
pub type OwnedRunEditor = Box<RunEditor>;
/// type
pub type NullableOwnedRunEditor = Option<OwnedRunEditor>;

/// Creates a new Run Editor that modifies the Run provided. Creation of the Run
/// Editor fails when a Run with no segments is provided. If a Run object with
/// no segments is provided, the Run Editor creation fails and <NULL> is
/// returned.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_new(run: OwnedRun) -> NullableOwnedRunEditor {
    RunEditor::new(*run).ok().map(Box::new)
}

/// Closes the Run Editor and gives back access to the modified Run object. In
/// case you want to implement a Cancel Button, just dispose the Run object you
/// get here.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_close(this: OwnedRunEditor) -> OwnedRun {
    Box::new((*this).close())
}

/// Calculates the Run Editor's state and encodes it as
/// JSON in order to visualize it.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_state_as_json(this: &RunEditor, image_cache: &mut ImageCache) -> Json {
    output_vec(|o| {
        this.state(image_cache).write_json(o).unwrap();
    })
}

/// Selects a different timing method for being modified.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_select_timing_method(this: &mut RunEditor, method: TimingMethod) {
    this.select_timing_method(method);
}

/// Unselects the segment with the given index. If it's not selected or the
/// index is out of bounds, nothing happens. The segment is not unselected,
/// when it is the only segment that is selected. If the active segment is
/// unselected, the most recently selected segment remaining becomes the
/// active segment.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_unselect(this: &mut RunEditor, index: usize) {
    this.unselect(index);
}

/// In addition to the segments that are already selected, the segment with
/// the given index is being selected. The segment chosen also becomes the
/// active segment.
///
/// This panics if the index of the segment provided is out of bounds.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_select_additionally(this: &mut RunEditor, index: usize) {
    this.select_additionally(index);
}

/// Selects the segment with the given index. All other segments are
/// unselected. The segment chosen also becomes the active segment.
///
/// This panics if the index of the segment provided is out of bounds.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_select_only(this: &mut RunEditor, index: usize) {
    this.select_only(index);
}

/// Sets the name of the game.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_set_game_name(this: &mut RunEditor, game: *const c_char) {
    // SAFETY: The caller guarantees that `game` is valid.
    this.set_game_name(unsafe { str(game) });
}

/// Sets the name of the category.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_set_category_name(
    this: &mut RunEditor,
    category: *const c_char,
) {
    // SAFETY: The caller guarantees that `category` is valid.
    this.set_category_name(unsafe { str(category) });
}

/// Parses and sets the timer offset from the string provided. The timer
/// offset specifies the time, the timer starts at when starting a new
/// attempt.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_parse_and_set_offset(
    this: &mut RunEditor,
    offset: *const c_char,
) -> bool {
    // SAFETY: The caller guarantees that `offset` is valid.
    this.parse_and_set_offset(unsafe { str(offset) }).is_ok()
}

/// Parses and sets the attempt count from the string provided. Changing
/// this has no affect on the attempt history or the segment history. This
/// number is mostly just a visual number for the runner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_parse_and_set_attempt_count(
    this: &mut RunEditor,
    attempts: *const c_char,
) -> bool {
    // SAFETY: The caller guarantees that `attempts` is valid.
    this.parse_and_set_attempt_count(unsafe { str(attempts) })
        .is_ok()
}

/// Sets the game's icon.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_set_game_icon(
    this: &mut RunEditor,
    data: *const u8,
    length: usize,
) {
    // SAFETY: The caller guarantees that `data` is valid for `length`.
    this.set_game_icon(Image::new(
        unsafe { slice(data, length).into() },
        Image::ICON,
    ));
}

/// Removes the game's icon.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_remove_game_icon(this: &mut RunEditor) {
    this.remove_game_icon();
}

/// Sets the Linked Layout of the Run. If a Layout is linked, it is supposed to
/// be loaded to visualize the Run.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_set_linked_layout(
    this: &mut RunEditor,
    linked_layout: OwnedLinkedLayout,
) {
    this.set_linked_layout(Some(*linked_layout));
}

/// Removes the Linked Layout of the Run if there is one. If a Layout is linked,
/// it is supposed to be loaded to visualize the Run.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_remove_linked_layout(this: &mut RunEditor) {
    this.set_linked_layout(None);
}

/// Sets the speedrun.com Run ID of the run. You need to ensure that the
/// record on speedrun.com matches up with the Personal Best of this run.
/// This may be empty if there's no association.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_set_run_id(this: &mut RunEditor, name: *const c_char) {
    // SAFETY: The caller guarantees that `name` is valid.
    this.set_run_id(unsafe { str(name) });
}

/// Sets the name of the region this game is from. This may be empty if it's
/// not specified.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_set_region_name(this: &mut RunEditor, name: *const c_char) {
    // SAFETY: The caller guarantees that `name` is valid.
    this.set_region_name(unsafe { str(name) });
}

/// Sets the name of the platform this game is run on. This may be empty if
/// it's not specified.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_set_platform_name(this: &mut RunEditor, name: *const c_char) {
    // SAFETY: The caller guarantees that `name` is valid.
    this.set_platform_name(unsafe { str(name) });
}

/// Specifies whether this speedrun is done on an emulator. Keep in mind
/// that <FALSE> may also mean that this information is simply not known.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_set_emulator_usage(this: &mut RunEditor, uses_emulator: bool) {
    this.set_emulator_usage(uses_emulator);
}

/// Sets the speedrun.com variable with the name specified to the value specified. A
/// variable is an arbitrary key value pair storing additional information
/// about the category. An example of this may be whether Amiibos are used
/// in this category. If the variable doesn't exist yet, it is being
/// inserted.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_set_speedrun_com_variable(
    this: &mut RunEditor,
    name: *const c_char,
    value: *const c_char,
) {
    // SAFETY: The caller guarantees that `name` and `value` are valid.
    this.set_speedrun_com_variable(unsafe { str(name) }, unsafe { str(value) });
}

/// Removes the speedrun.com variable with the name specified.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_remove_speedrun_com_variable(
    this: &mut RunEditor,
    name: *const c_char,
) {
    // SAFETY: The caller guarantees that `name` is valid.
    this.remove_speedrun_com_variable(unsafe { str(name) });
}

/// Adds a new permanent custom variable. If there's a temporary variable with
/// the same name, it gets turned into a permanent variable and its value stays.
/// If a permanent variable with the name already exists, nothing happens.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_add_custom_variable(this: &mut RunEditor, name: *const c_char) {
    // SAFETY: The caller guarantees that `name` is valid.
    this.add_custom_variable(unsafe { str(name) });
}

/// Sets the value of a custom variable with the name specified. If the custom
/// variable does not exist, or is not a permanent variable, nothing happens.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_set_custom_variable(
    this: &mut RunEditor,
    name: *const c_char,
    value: *const c_char,
) {
    // SAFETY: The caller guarantees that `name` and `value` are valid.
    this.set_custom_variable(unsafe { str(name) }, unsafe { str(value) });
}

/// Removes the custom variable with the name specified. If the custom variable
/// does not exist, or is not a permanent variable, nothing happens.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_remove_custom_variable(
    this: &mut RunEditor,
    name: *const c_char,
) {
    // SAFETY: The caller guarantees that `name` is valid.
    this.remove_custom_variable(unsafe { str(name) });
}

/// Resets all the Metadata Information.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_clear_metadata(this: &mut RunEditor) {
    this.clear_metadata();
}

/// Inserts a new empty segment above the active segment and adjusts the
/// Run's history information accordingly. The newly created segment is then
/// the only selected segment and also the active segment.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_insert_segment_above(this: &mut RunEditor) {
    this.insert_segment_above();
}

/// Inserts a new empty segment below the active segment and adjusts the
/// Run's history information accordingly. The newly created segment is then
/// the only selected segment and also the active segment.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_insert_segment_below(this: &mut RunEditor) {
    this.insert_segment_below();
}

/// Removes all the selected segments, unless all of them are selected. The
/// run's information is automatically adjusted properly. The next
/// not-to-be-removed segment after the active segment becomes the new
/// active segment. If there's none, then the next not-to-be-removed segment
/// before the active segment, becomes the new active segment.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_remove_segments(this: &mut RunEditor) {
    this.remove_segments();
}

/// Moves all the selected segments up, unless the first segment is
/// selected. The run's information is automatically adjusted properly. The
/// active segment stays the active segment.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_move_segments_up(this: &mut RunEditor) {
    this.move_segments_up();
}

/// Moves all the selected segments down, unless the last segment is
/// selected. The run's information is automatically adjusted properly. The
/// active segment stays the active segment.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_move_segments_down(this: &mut RunEditor) {
    this.move_segments_down();
}

/// Sets the icon of the active segment.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_active_set_icon(
    this: &mut RunEditor,
    data: *const u8,
    length: usize,
) {
    // SAFETY: The caller guarantees that `data` is valid for `length`.
    this.active_segment().set_icon(Image::new(
        unsafe { slice(data, length).into() },
        Image::ICON,
    ));
}

/// Removes the icon of the active segment.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_active_remove_icon(this: &mut RunEditor) {
    this.active_segment().remove_icon();
}

/// Sets the name of the active segment.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_active_set_name(this: &mut RunEditor, name: *const c_char) {
    // SAFETY: The caller guarantees that `name` is valid.
    this.active_segment().set_name(unsafe { str(name) });
}

/// Parses a split time from a string and sets it for the active segment with
/// the chosen timing method.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_active_parse_and_set_split_time(
    this: &mut RunEditor,
    time: *const c_char,
) -> bool {
    // SAFETY: The caller guarantees that `time` is valid.
    this.active_segment()
        .parse_and_set_split_time(unsafe { str(time) })
        .is_ok()
}

/// Parses a segment time from a string and sets it for the active segment with
/// the chosen timing method.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_active_parse_and_set_segment_time(
    this: &mut RunEditor,
    time: *const c_char,
) -> bool {
    // SAFETY: The caller guarantees that `time` is valid.
    this.active_segment()
        .parse_and_set_segment_time(unsafe { str(time) })
        .is_ok()
}

/// Parses a best segment time from a string and sets it for the active segment
/// with the chosen timing method.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_active_parse_and_set_best_segment_time(
    this: &mut RunEditor,
    time: *const c_char,
) -> bool {
    // SAFETY: The caller guarantees that `time` is valid.
    this.active_segment()
        .parse_and_set_best_segment_time(unsafe { str(time) })
        .is_ok()
}

/// Parses a comparison time for the provided comparison and sets it for the
/// active active segment with the chosen timing method.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_active_parse_and_set_comparison_time(
    this: &mut RunEditor,
    comparison: *const c_char,
    time: *const c_char,
) -> bool {
    // SAFETY: The caller guarantees that `comparison` and `time` are valid.
    this.active_segment()
        .parse_and_set_comparison_time(unsafe { str(comparison) }, unsafe { str(time) })
        .is_ok()
}

/// Adds a new custom comparison. It can't be added if it starts with
/// `[Race]` or it already exists.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_add_comparison(
    this: &mut RunEditor,
    comparison: *const c_char,
) -> bool {
    // SAFETY: The caller guarantees that `comparison` is valid.
    this.add_comparison(unsafe { str(comparison) }).is_ok()
}

/// Imports the Personal Best from the provided run as a comparison. The
/// comparison can't be added if its name starts with `[Race]` or it already
/// exists.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_import_comparison(
    this: &mut RunEditor,
    run: &Run,
    comparison: *const c_char,
) -> bool {
    // SAFETY: The caller guarantees that `comparison` is valid.
    this.import_comparison(run, unsafe { str(comparison) })
        .is_ok()
}

/// Imports a named comparison from the provided run as a comparison. The
/// comparison can't be added if its name starts with `[Race]` or it already
/// exists.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_import_comparison_as_comparison(
    this: &mut RunEditor,
    run: &Run,
    comparison: *const c_char,
    run_comparison: *const c_char,
) -> bool {
    // SAFETY: The caller guarantees that `comparison` is valid.
    this.import_comparison_as_comparison(run, unsafe { str(comparison) }, unsafe {
        str(run_comparison)
    })
    .is_ok()
}

/// Removes the chosen custom comparison. You can't remove a Comparison
/// Generator's Comparison or the Personal Best.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_remove_comparison(
    this: &mut RunEditor,
    comparison: *const c_char,
) {
    // SAFETY: The caller guarantees that `comparison` is valid.
    this.remove_comparison(unsafe { str(comparison) });
}

/// Renames a comparison. The comparison can't be renamed if the new name of
/// the comparison starts with `[Race]` or it already exists.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_rename_comparison(
    this: &mut RunEditor,
    old_name: *const c_char,
    new_name: *const c_char,
) -> bool {
    // SAFETY: The caller guarantees that `old_name` and `new_name` are valid.
    this.rename_comparison(unsafe { str(old_name) }, unsafe { str(new_name) })
        .is_ok()
}

/// Reorders the custom comparisons by moving the comparison with the source
/// index specified to the destination index specified. Returns <FALSE> if one
/// of the indices is invalid. The indices are based on the comparison names of
/// the Run Editor's state.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_move_comparison(
    this: &mut RunEditor,
    src_index: usize,
    dst_index: usize,
) -> bool {
    this.move_comparison(src_index, dst_index).is_ok()
}

/// Parses a goal time and generates a custom goal comparison based on the
/// parsed value. The comparison's times are automatically balanced based on the
/// runner's history such that it roughly represents what split times for the
/// goal time would roughly look like. Since it is populated by the runner's
/// history, only goal times within the sum of the best segments and the sum of
/// the worst segments are supported. Everything else is automatically capped by
/// that range. The comparison is only populated for the selected timing method.
/// The other timing method's comparison times are not modified by this, so you
/// can call this again with the other timing method to generate the comparison
/// times for both timing methods.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_parse_and_generate_goal_comparison(
    this: &mut RunEditor,
    time: *const c_char,
) -> bool {
    // SAFETY: The caller guarantees that `time` is valid.
    this.parse_and_generate_goal_comparison(unsafe { str(time) })
        .is_ok()
}

/// Copies a comparison with the given name as a new custom comparison with the
/// new name provided. It can't be added if it starts with `[Race]` or it
/// already exists. The old comparison needs to exist.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunEditor_copy_comparison(
    this: &mut RunEditor,
    old_name: *const c_char,
    new_name: *const c_char,
) -> bool {
    // SAFETY: The caller guarantees that `old_name` and `new_name` are valid.
    this.copy_comparison(unsafe { str(old_name) }, unsafe { str(new_name) })
        .is_ok()
}

/// Clears out the Attempt History and the Segment Histories of all the
/// segments.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_clear_history(this: &mut RunEditor) {
    this.clear_history();
}

/// Clears out the Attempt History, the Segment Histories, all the times,
/// sets the Attempt Count to 0 and clears the speedrun.com run id
/// association. All Custom Comparisons other than `Personal Best` are
/// deleted as well.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_clear_times(this: &mut RunEditor) {
    this.clear_times();
}

/// Creates a Sum of Best Cleaner which allows you to interactively remove
/// potential issues in the segment history that lead to an inaccurate Sum
/// of Best. If you skip a split, whenever you will do the next split, the
/// combined segment time might be faster than the sum of the individual
/// best segments. The Sum of Best Cleaner will point out all of these and
/// allows you to delete them individually if any of them seem wrong.
#[unsafe(no_mangle)]
pub extern "C" fn RunEditor_clean_sum_of_best(
    this: &'static mut RunEditor,
) -> OwnedSumOfBestCleaner {
    Box::new(this.clean_sum_of_best())
}
