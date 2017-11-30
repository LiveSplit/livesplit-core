use livesplit_core::{Run, Time, TimeSpan, Timer, TimerPhase, TimingMethod};
use super::{acc, acc_mut, alloc, output_str, output_time, output_time_span, own, own_drop};
use run::{NullableOwnedRun, OwnedRun};
use libc::c_char;
use shared_timer::OwnedSharedTimer;
use std::ptr;

pub type OwnedTimer = *mut Timer;
pub type NullableOwnedTimer = OwnedTimer;

#[no_mangle]
pub unsafe extern "C" fn Timer_new(run: OwnedRun) -> NullableOwnedTimer {
    Timer::new(own(run)).ok().map_or_else(ptr::null_mut, alloc)
}

#[no_mangle]
pub unsafe extern "C" fn Timer_into_shared(this: OwnedTimer) -> OwnedSharedTimer {
    alloc(own(this).into_shared())
}

#[no_mangle]
pub unsafe extern "C" fn Timer_drop(this: OwnedTimer) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn Timer_replace_run(
    this: *mut Timer,
    run: *mut Run,
    update_splits: bool,
) -> bool {
    // This working correctly relies on panic = "abort",
    // as a panic would leave the run in an uninitialized state.
    let result = acc_mut(this).replace_run(ptr::read(run), update_splits);
    let was_successful = result.is_ok();
    let result = match result {
        Ok(r) | Err(r) => r,
    };
    ptr::write(run, result);
    was_successful
}

#[no_mangle]
pub unsafe extern "C" fn Timer_set_run(this: *mut Timer, run: OwnedRun) -> NullableOwnedRun {
    acc_mut(this)
        .set_run(own(run))
        .err()
        .map_or_else(ptr::null_mut, alloc)
}

#[no_mangle]
pub unsafe extern "C" fn Timer_start(this: *mut Timer) {
    acc_mut(this).start();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_split(this: *mut Timer) {
    acc_mut(this).split();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_split_or_start(this: *mut Timer) {
    acc_mut(this).split_or_start();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_skip_split(this: *mut Timer) {
    acc_mut(this).skip_split();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_undo_split(this: *mut Timer) {
    acc_mut(this).undo_split();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_reset(this: *mut Timer, update_splits: bool) {
    acc_mut(this).reset(update_splits);
}

#[no_mangle]
pub unsafe extern "C" fn Timer_pause(this: *mut Timer) {
    acc_mut(this).pause();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_resume(this: *mut Timer) {
    acc_mut(this).resume();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_toggle_pause(this: *mut Timer) {
    acc_mut(this).toggle_pause();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_toggle_pause_or_start(this: *mut Timer) {
    acc_mut(this).toggle_pause_or_start();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_undo_all_pauses(this: *mut Timer) {
    acc_mut(this).undo_all_pauses();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_current_timing_method(this: *const Timer) -> TimingMethod {
    acc(this).current_timing_method()
}

#[no_mangle]
pub unsafe extern "C" fn Timer_set_current_timing_method(this: *mut Timer, method: TimingMethod) {
    acc_mut(this).set_current_timing_method(method);
}

#[no_mangle]
pub unsafe extern "C" fn Timer_current_comparison(this: *const Timer) -> *const c_char {
    output_str(acc(this).current_comparison())
}

#[no_mangle]
pub unsafe extern "C" fn Timer_switch_to_next_comparison(this: *mut Timer) {
    acc_mut(this).switch_to_next_comparison();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_switch_to_previous_comparison(this: *mut Timer) {
    acc_mut(this).switch_to_previous_comparison();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_is_game_time_initialized(this: *const Timer) -> bool {
    acc(this).is_game_time_initialized()
}

#[no_mangle]
pub unsafe extern "C" fn Timer_initialize_game_time(this: *mut Timer) {
    acc_mut(this).initialize_game_time();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_uninitialize_game_time(this: *mut Timer) {
    acc_mut(this).uninitialize_game_time();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_is_game_time_paused(this: *const Timer) -> bool {
    acc(this).is_game_time_paused()
}

#[no_mangle]
pub unsafe extern "C" fn Timer_pause_game_time(this: *mut Timer) {
    acc_mut(this).pause_game_time();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_resume_game_time(this: *mut Timer) {
    acc_mut(this).resume_game_time();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_set_game_time(this: *mut Timer, time: *const TimeSpan) {
    acc_mut(this).set_game_time(*acc(time));
}

#[no_mangle]
pub unsafe extern "C" fn Timer_loading_times(this: *const Timer) -> *const TimeSpan {
    output_time_span(acc(this).loading_times())
}

#[no_mangle]
pub unsafe extern "C" fn Timer_set_loading_times(this: *mut Timer, time: *const TimeSpan) {
    acc_mut(this).set_loading_times(*acc(time));
}

#[no_mangle]
pub unsafe extern "C" fn Timer_current_phase(this: *const Timer) -> TimerPhase {
    acc(this).current_phase()
}

#[no_mangle]
pub unsafe extern "C" fn Timer_get_run(this: *const Timer) -> *const Run {
    acc(this).run()
}

#[no_mangle]
pub unsafe extern "C" fn Timer_print_debug(this: *const Timer) {
    println!("{:#?}", acc(this));
}

#[no_mangle]
pub unsafe extern "C" fn Timer_current_time(this: *const Timer) -> *const Time {
    output_time(acc(this).current_time())
}
