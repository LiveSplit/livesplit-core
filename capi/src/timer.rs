use livesplit_core::{Timer, TimeSpan, TimingMethod, TimerPhase, Run};
use super::{alloc, own, acc_mut, drop, acc, output_str, output_time_span};
use run::OwnedRun;
use libc::c_char;

pub type OwnedTimer = *mut Timer;

#[no_mangle]
pub unsafe extern "C" fn Timer_new(run: OwnedRun) -> OwnedTimer {
    alloc(Timer::new(own(run)))
}

#[no_mangle]
pub unsafe extern "C" fn Timer_drop(this: OwnedTimer) {
    drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn Timer_split(this: *mut Timer) {
    acc_mut(this).split();
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
pub unsafe extern "C" fn Timer_unpause_game_time(this: *mut Timer) {
    acc_mut(this).unpause_game_time();
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
pub unsafe extern "C" fn Timer_clone_run(this: *const Timer) -> OwnedRun {
    alloc(acc(this).run().clone())
}

#[no_mangle]
pub unsafe extern "C" fn Timer_print_debug(this: *const Timer) {
    println!("{:#?}", acc(this));
}
