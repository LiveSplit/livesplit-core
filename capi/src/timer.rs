//! A Timer provides all the capabilities necessary for doing speedrun attempts.

use super::{output_str, output_time, output_time_span, output_vec};
use crate::run::{NullableOwnedRun, OwnedRun};
use crate::shared_timer::OwnedSharedTimer;
use livesplit_core::run::saver;
use livesplit_core::{Run, Time, TimeSpan, Timer, TimerPhase, TimingMethod};
use std::os::raw::c_char;
use std::ptr;

/// type
pub type OwnedTimer = Box<Timer>;
/// type
pub type NullableOwnedTimer = Option<OwnedTimer>;

/// Creates a new Timer based on a Run object storing all the information
/// about the splits. The Run object needs to have at least one segment, so
/// that the Timer can store the final time. If a Run object with no
/// segments is provided, the Timer creation fails and <NULL> is returned.
#[no_mangle]
pub extern "C" fn Timer_new(run: OwnedRun) -> NullableOwnedTimer {
    Timer::new(*run).ok().map(Box::new)
}

/// Consumes the Timer and creates a Shared Timer that can be shared across
/// multiple threads with multiple owners.
#[no_mangle]
pub extern "C" fn Timer_into_shared(this: OwnedTimer) -> OwnedSharedTimer {
    Box::new((*this).into_shared())
}

/// Takes out the Run from the Timer and resets the current attempt if there
/// is one in progress. If the splits are to be updated, all the information
/// of the current attempt is stored in the Run's history. Otherwise the
/// current attempt's information is discarded.
#[no_mangle]
pub extern "C" fn Timer_into_run(this: OwnedTimer, update_splits: bool) -> OwnedRun {
    Box::new((*this).into_run(update_splits))
}

/// drop
#[no_mangle]
pub extern "C" fn Timer_drop(this: OwnedTimer) {
    drop(this);
}

/// Replaces the Run object used by the Timer with the Run object provided. If
/// the Run provided contains no segments, it can't be used for timing and is
/// not being modified. Otherwise the Run that was in use by the Timer gets
/// stored in the Run object provided. Before the Run is returned, the current
/// attempt is reset and the splits are being updated depending on the
/// `update_splits` parameter. The return value indicates whether the Run got
/// replaced successfully.
#[no_mangle]
pub unsafe extern "C" fn Timer_replace_run(
    this: &mut Timer,
    run: &mut Run,
    update_splits: bool,
) -> bool {
    // This working correctly relies on panic = "abort",
    // as a panic would leave the run in an uninitialized state.
    let result = this.replace_run(ptr::read(run), update_splits);
    let was_successful = result.is_ok();
    let result = match result {
        Ok(r) | Err(r) => r,
    };
    ptr::write(run, result);
    was_successful
}

/// Sets the Run object used by the Timer with the Run object provided. If the
/// Run provided contains no segments, it can't be used for timing and gets
/// returned again. If the Run object can be set, the original Run object in use
/// by the Timer is disposed by this method and <NULL> is returned.
#[no_mangle]
pub extern "C" fn Timer_set_run(this: &mut Timer, run: OwnedRun) -> NullableOwnedRun {
    this.set_run(*run).err().map(Box::new)
}

/// Starts the Timer if there is no attempt in progress. If that's not the
/// case, nothing happens.
#[no_mangle]
pub extern "C" fn Timer_start(this: &mut Timer) {
    this.start();
}

/// If an attempt is in progress, stores the current time as the time of the
/// current split. The attempt ends if the last split time is stored.
#[no_mangle]
pub extern "C" fn Timer_split(this: &mut Timer) {
    this.split();
}

/// Starts a new attempt or stores the current time as the time of the
/// current split. The attempt ends if the last split time is stored.
#[no_mangle]
pub extern "C" fn Timer_split_or_start(this: &mut Timer) {
    this.split_or_start();
}

/// Skips the current split if an attempt is in progress and the
/// current split is not the last split.
#[no_mangle]
pub extern "C" fn Timer_skip_split(this: &mut Timer) {
    this.skip_split();
}

/// Removes the split time from the last split if an attempt is in progress
/// and there is a previous split. The Timer Phase also switches to
/// `Running` if it previously was `Ended`.
#[no_mangle]
pub extern "C" fn Timer_undo_split(this: &mut Timer) {
    this.undo_split();
}

/// Resets the current attempt if there is one in progress. If the splits
/// are to be updated, all the information of the current attempt is stored
/// in the Run's history. Otherwise the current attempt's information is
/// discarded.
#[no_mangle]
pub extern "C" fn Timer_reset(this: &mut Timer, update_splits: bool) {
    this.reset(update_splits);
}

/// Resets the current attempt if there is one in progress. The splits are
/// updated such that the current attempt's split times are being stored as
/// the new Personal Best.
#[no_mangle]
pub extern "C" fn Timer_reset_and_set_attempt_as_pb(this: &mut Timer) {
    this.reset_and_set_attempt_as_pb();
}

/// Pauses an active attempt that is not paused.
#[no_mangle]
pub extern "C" fn Timer_pause(this: &mut Timer) {
    this.pause();
}

/// Resumes an attempt that is paused.
#[no_mangle]
pub extern "C" fn Timer_resume(this: &mut Timer) {
    this.resume();
}

/// Toggles an active attempt between `Paused` and `Running`.
#[no_mangle]
pub extern "C" fn Timer_toggle_pause(this: &mut Timer) {
    this.toggle_pause();
}

/// Toggles an active attempt between `Paused` and `Running` or starts an
/// attempt if there's none in progress.
#[no_mangle]
pub extern "C" fn Timer_toggle_pause_or_start(this: &mut Timer) {
    this.toggle_pause_or_start();
}

/// Removes all the pause times from the current time. If the current
/// attempt is paused, it also resumes that attempt. Additionally, if the
/// attempt is finished, the final split time is adjusted to not include the
/// pause times as well.
///
/// # Warning
///
/// This behavior is not entirely optimal, as generally only the final split
/// time is modified, while all other split times are left unmodified, which
/// may not be what actually happened during the run.
#[no_mangle]
pub extern "C" fn Timer_undo_all_pauses(this: &mut Timer) {
    this.undo_all_pauses();
}

/// Returns the currently selected Timing Method.
#[no_mangle]
pub extern "C" fn Timer_current_timing_method(this: &Timer) -> TimingMethod {
    this.current_timing_method()
}

/// Sets the current Timing Method to the Timing Method provided.
#[no_mangle]
pub extern "C" fn Timer_set_current_timing_method(this: &mut Timer, method: TimingMethod) {
    this.set_current_timing_method(method);
}

/// Returns the current comparison that is being compared against. This may
/// be a custom comparison or one of the Comparison Generators.
#[no_mangle]
pub extern "C" fn Timer_current_comparison(this: &Timer) -> *const c_char {
    output_str(this.current_comparison())
}

/// Switches the current comparison to the next comparison in the list.
#[no_mangle]
pub extern "C" fn Timer_switch_to_next_comparison(this: &mut Timer) {
    this.switch_to_next_comparison();
}

/// Switches the current comparison to the previous comparison in the list.
#[no_mangle]
pub extern "C" fn Timer_switch_to_previous_comparison(this: &mut Timer) {
    this.switch_to_previous_comparison();
}

/// Returns whether Game Time is currently initialized. Game Time
/// automatically gets uninitialized for each new attempt.
#[no_mangle]
pub extern "C" fn Timer_is_game_time_initialized(this: &Timer) -> bool {
    this.is_game_time_initialized()
}

/// Initializes Game Time for the current attempt. Game Time automatically
/// gets uninitialized for each new attempt.
#[no_mangle]
pub extern "C" fn Timer_initialize_game_time(this: &mut Timer) {
    this.initialize_game_time();
}

/// Deinitializes Game Time for the current attempt.
#[no_mangle]
pub extern "C" fn Timer_deinitialize_game_time(this: &mut Timer) {
    this.deinitialize_game_time();
}

/// Returns whether the Game Timer is currently paused. If the Game Timer is
/// not paused, it automatically increments similar to Real Time.
#[no_mangle]
pub extern "C" fn Timer_is_game_time_paused(this: &Timer) -> bool {
    this.is_game_time_paused()
}

/// Pauses the Game Timer such that it doesn't automatically increment
/// similar to Real Time.
#[no_mangle]
pub extern "C" fn Timer_pause_game_time(this: &mut Timer) {
    this.pause_game_time();
}

/// Resumes the Game Timer such that it automatically increments similar to
/// Real Time, starting from the Game Time it was paused at.
#[no_mangle]
pub extern "C" fn Timer_resume_game_time(this: &mut Timer) {
    this.resume_game_time();
}

/// Sets the Game Time to the time specified. This also works if the Game
/// Time is paused, which can be used as a way of updating the Game Timer
/// periodically without it automatically moving forward. This ensures that
/// the Game Timer never shows any time that is not coming from the game.
#[no_mangle]
pub extern "C" fn Timer_set_game_time(this: &mut Timer, time: &TimeSpan) {
    this.set_game_time(*time);
}

/// Accesses the loading times. Loading times are defined as Game Time - Real Time.
#[no_mangle]
pub extern "C" fn Timer_loading_times(this: &Timer) -> *const TimeSpan {
    output_time_span(this.loading_times())
}

/// Instead of setting the Game Time directly, this method can be used to
/// just specify the amount of time the game has been loading. The Game Time
/// is then automatically determined by Real Time - Loading Times.
#[no_mangle]
pub extern "C" fn Timer_set_loading_times(this: &mut Timer, time: &TimeSpan) {
    this.set_loading_times(*time);
}

/// Returns the current Timer Phase.
#[no_mangle]
pub extern "C" fn Timer_current_phase(this: &Timer) -> TimerPhase {
    this.current_phase()
}

/// Accesses the Run in use by the Timer.
#[no_mangle]
pub extern "C" fn Timer_get_run(this: &Timer) -> &Run {
    this.run()
}

/// Saves the Run in use by the Timer as a LiveSplit splits file (*.lss).
#[no_mangle]
pub extern "C" fn Timer_save_as_lss(this: &Timer) -> *const c_char {
    output_vec(|o| {
        saver::livesplit::save_timer(this, o).unwrap();
    })
}

/// Marks the Run as unmodified, so that it is known that all the changes
/// have been saved.
#[no_mangle]
pub extern "C" fn Timer_mark_as_unmodified(this: &mut Timer) {
    this.mark_as_unmodified();
}

/// Prints out debug information representing the whole state of the Timer. This
/// is being written to stdout.
#[no_mangle]
pub extern "C" fn Timer_print_debug(this: &Timer) {
    println!("{:#?}", this);
}

/// Returns the current time of the Timer. The Game Time is <NULL> if the Game
/// Time has not been initialized.
#[no_mangle]
pub extern "C" fn Timer_current_time(this: &Timer) -> *const Time {
    output_time(this.current_time())
}
