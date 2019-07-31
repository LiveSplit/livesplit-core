use super::for_timer;
use crate::tests_helper::{
    create_timer, make_progress_run_with_splits_opt, run_with_splits, span, start_run,
};
use crate::{Timer, TimerPhase};

fn chance(timer: &Timer) -> u32 {
    (for_timer(timer) * 100.0).round() as _
}

#[test]
fn is_100_percent_without_any_times() {
    let timer = create_timer(&["A"]);
    assert_eq!(chance(&timer), 100);
}

#[test]
fn is_100_percent_if_there_are_history_times_but_no_pb() {
    let mut timer = create_timer(&["A", "B"]);
    // Start a few runs, but don't actually complete them fully.
    run_with_splits(&mut timer, &[10.0]);
    run_with_splits(&mut timer, &[8.0]);
    run_with_splits(&mut timer, &[12.0]);
    assert_eq!(chance(&timer), 100);
}

#[test]
fn is_100_percent_if_pb_has_just_been_beaten() {
    let mut timer = create_timer(&["A"]);
    run_with_splits(&mut timer, &[10.0]);
    run_with_splits(&mut timer, &[8.0]);
    run_with_splits(&mut timer, &[12.0]);
    start_run(&mut timer);
    make_progress_run_with_splits_opt(&mut timer, &[Some(7.0)]);
    assert_eq!(timer.current_phase(), TimerPhase::Ended);
    assert_eq!(chance(&timer), 100);
}

#[test]
fn is_0_percent_if_pb_has_not_been_beaten() {
    let mut timer = create_timer(&["A"]);
    run_with_splits(&mut timer, &[10.0]);
    run_with_splits(&mut timer, &[8.0]);
    run_with_splits(&mut timer, &[12.0]);
    start_run(&mut timer);
    make_progress_run_with_splits_opt(&mut timer, &[Some(9.0)]);
    assert_eq!(timer.current_phase(), TimerPhase::Ended);
    assert_eq!(chance(&timer), 0);
}

#[test]
fn is_50_percent_after_one_run() {
    let mut timer = create_timer(&["A"]);
    run_with_splits(&mut timer, &[10.0]);
    assert_eq!(chance(&timer), 50);
}

#[test]
fn is_0_percent_if_pb_is_optimal() {
    let mut timer = create_timer(&["A"]);
    run_with_splits(&mut timer, &[10.0]);
    run_with_splits(&mut timer, &[20.0]);
    assert_eq!(chance(&timer), 0);
}

#[test]
fn is_100_percent_when_ahead_after_one_run() {
    let mut timer = create_timer(&["A", "B"]);
    run_with_splits(&mut timer, &[11.0, 20.0]);
    start_run(&mut timer);
    make_progress_run_with_splits_opt(&mut timer, &[Some(5.0)]);
    assert_eq!(chance(&timer), 100);
}

#[test]
fn is_0_percent_when_behind_after_one_run() {
    let mut timer = create_timer(&["A", "B"]);
    run_with_splits(&mut timer, &[11.0, 20.0]);
    start_run(&mut timer);
    make_progress_run_with_splits_opt(&mut timer, &[Some(15.0)]);
    assert_eq!(chance(&timer), 0);
}

#[test]
fn is_50_percent_when_tied_after_one_run() {
    let mut timer = create_timer(&["A", "B"]);
    run_with_splits(&mut timer, &[11.0, 20.0]);
    start_run(&mut timer);
    make_progress_run_with_splits_opt(&mut timer, &[Some(11.0)]);
    assert_eq!(chance(&timer), 50);
}

#[test]
fn is_below_50_percent_when_its_unlikey_to_beat_pb() {
    let mut timer = create_timer(&["A", "B"]);
    run_with_splits(&mut timer, &[20.0, 28.0]);
    run_with_splits(&mut timer, &[20.0, 27.0]);
    run_with_splits(&mut timer, &[20.0, 26.0]);
    run_with_splits(&mut timer, &[20.0, 25.0]);
    run_with_splits(&mut timer, &[20.0, 24.0]);
    run_with_splits(&mut timer, &[10.0, 15.0]);
    start_run(&mut timer);
    // Would need a final segment time of less than 4.5 seconds to beat PB, but
    // there is only a single segment time of 4 seconds in the history that is
    // less than that.
    make_progress_run_with_splits_opt(&mut timer, &[Some(10.5)]);
    // The value doesn't matter much as long as it's less than 50% and more than
    // 0%.
    let chance = chance(&timer);
    assert!(chance < 50);
    assert!(chance > 0);
}

#[test]
fn is_above_50_percent_when_its_likey_to_beat_pb() {
    let mut timer = create_timer(&["A", "B"]);
    run_with_splits(&mut timer, &[20.0, 28.0]);
    run_with_splits(&mut timer, &[20.0, 27.0]);
    run_with_splits(&mut timer, &[20.0, 26.0]);
    run_with_splits(&mut timer, &[20.0, 25.0]);
    run_with_splits(&mut timer, &[20.0, 24.0]);
    run_with_splits(&mut timer, &[10.0, 15.0]);
    start_run(&mut timer);
    // Would need a final segment time of less than 6.5 seconds to beat PB,
    // which has been achieved most of the time.
    make_progress_run_with_splits_opt(&mut timer, &[Some(8.5)]);
    // The value doesn't matter much as long as it's more than 50% and less than
    // 100%.
    let chance = chance(&timer);
    assert!(chance > 50);
    assert!(chance < 100);
}

#[test]
fn prefers_more_recent_info() {
    let mut timer = create_timer(&["A", "B"]);
    run_with_splits(&mut timer, &[20.0, 26.0]); // 2x 6 seconds
    run_with_splits(&mut timer, &[20.0, 26.0]);
    run_with_splits(&mut timer, &[10.0, 15.0]); // 1x 5 seconds (PB)
    run_with_splits(&mut timer, &[20.0, 24.0]);
    run_with_splits(&mut timer, &[20.0, 24.0]); // 2x 4 seconds
    start_run(&mut timer);
    // We started off with a final segment time of 6 seconds, got 5 seconds in
    // our PB and then have been getting 4 seconds recently. If we are tied with
    // PB, we should be seeing a chance above 50%, despite the average final
    // segment time being 5 seconds, which would tie us with PB.
    make_progress_run_with_splits_opt(&mut timer, &[Some(10.0)]);
    // The value doesn't matter much as long as it's more than 50% and less than
    // 100%.
    let chance = chance(&timer);
    assert!(chance > 50);
    assert!(chance < 100);
}

#[test]
fn is_0_percent_if_we_cant_pb_anymore() {
    let mut timer = create_timer(&["A", "B"]);
    run_with_splits(&mut timer, &[10.0, 16.0]);
    run_with_splits(&mut timer, &[8.0, 19.0]);
    run_with_splits(&mut timer, &[12.0, 20.0]);
    start_run(&mut timer);
    make_progress_run_with_splits_opt(&mut timer, &[Some(7.0)]);
    timer.set_game_time(span(21.0));
    // We don't split yet, we are simply losing so much time that we can't PB anymore.
    assert_eq!(chance(&timer), 0);
}
