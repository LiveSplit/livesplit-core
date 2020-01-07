use crate::{Run, Segment, TimeSpan, Timer, TimingMethod};

fn timer() -> Timer {
    use super::run;
    let mut run = run();
    run.metadata_mut()
        .custom_variable_mut("Permanent")
        .permanent();
    let timer = Timer::new(run).unwrap();
    assert!(!timer.run().has_been_modified());
    timer
}

#[test]
fn not_when_replacing_run() {
    let mut timer = timer();
    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    timer.replace_run(run, true).unwrap();
    assert!(!timer.run().has_been_modified());
}

#[test]
fn not_when_setting_run() {
    let mut timer = timer();
    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    timer.set_run(run).unwrap();
    assert!(!timer.run().has_been_modified());
}

#[test]
fn not_when_marking_as_unmodified() {
    // That would be really stupid
    let mut timer = timer();
    timer.mark_as_unmodified();
    assert!(!timer.run().has_been_modified());
}

#[test]
fn not_when_setting_current_timing_method() {
    let mut timer = timer();
    timer.set_current_timing_method(TimingMethod::RealTime);
    assert!(!timer.run().has_been_modified());
}

#[test]
fn when_starting_the_timer() {
    let mut timer = timer();
    timer.start();
    assert!(timer.run().has_been_modified());
}

#[test]
fn not_when_splitting_without_an_attempt() {
    let mut timer = timer();
    timer.split();
    assert!(!timer.run().has_been_modified());
}

#[test]
fn when_splitting_or_starting_the_timer() {
    let mut timer = timer();
    timer.split_or_start();
    assert!(timer.run().has_been_modified());
}

#[test]
fn not_when_skipping_a_split_without_an_attempt() {
    let mut timer = timer();
    timer.skip_split();
    assert!(!timer.run().has_been_modified());
}

#[test]
fn not_when_undoing_a_split_without_an_attempt() {
    let mut timer = timer();
    timer.undo_split();
    assert!(!timer.run().has_been_modified());
}

#[test]
fn when_starting_and_resetting_with_update_splits() {
    let mut timer = timer();
    timer.start();
    timer.reset(true);
    assert!(timer.run().has_been_modified());
}

#[test]
fn when_starting_and_resetting_without_update_splits() {
    let mut timer = timer();
    timer.start();
    timer.reset(false);
    assert!(timer.run().has_been_modified());
}

#[test]
fn not_when_resetting_without_an_attempt() {
    let mut timer = timer();
    timer.reset(true);
    assert!(!timer.run().has_been_modified());
}

#[test]
fn not_when_pausing_without_an_attempt() {
    let mut timer = timer();
    timer.pause();
    assert!(!timer.run().has_been_modified());
}

#[test]
fn not_when_resuming_without_an_attempt() {
    let mut timer = timer();
    timer.resume();
    assert!(!timer.run().has_been_modified());
}

#[test]
fn not_when_toggling_pause_without_an_attempt() {
    let mut timer = timer();
    timer.toggle_pause();
    assert!(!timer.run().has_been_modified());
}

#[test]
fn when_toggling_pause_or_start() {
    let mut timer = timer();
    timer.toggle_pause_or_start();
    assert!(timer.run().has_been_modified());
}

#[test]
fn not_when_undoing_all_pauses_without_an_attempt() {
    let mut timer = timer();
    timer.undo_all_pauses();
    assert!(!timer.run().has_been_modified());
}

#[test]
fn not_when_switching_comparisons() {
    let mut timer = timer();
    timer.switch_to_next_comparison();
    assert!(!timer.run().has_been_modified());
    timer.switch_to_previous_comparison();
    assert!(!timer.run().has_been_modified());
}

#[test]
fn not_when_initializing_or_deinitializing_game_time() {
    let mut timer = timer();
    timer.initialize_game_time();
    assert!(!timer.run().has_been_modified());
    timer.deinitialize_game_time();
    assert!(!timer.run().has_been_modified());
}

#[test]
fn not_when_pausing_resuming_or_setting_game_time_without_an_attempt() {
    let mut timer = timer();
    timer.pause_game_time();
    assert!(!timer.run().has_been_modified());
    timer.resume_game_time();
    assert!(!timer.run().has_been_modified());
    timer.set_game_time(TimeSpan::default());
    assert!(!timer.run().has_been_modified());
    timer.set_loading_times(TimeSpan::default());
    assert!(!timer.run().has_been_modified());
}

#[test]
fn when_setting_permanent_custom_variables() {
    let mut timer = timer();
    timer.set_custom_variable("Permanent", "okok");
    assert!(timer.run().has_been_modified());
}

#[test]
fn not_when_setting_temporary_custom_variables() {
    let mut timer = timer();
    timer.set_custom_variable("Foo", "Bar");
    assert!(!timer.run().has_been_modified());
    timer.set_custom_variable("Foo", "Bar2");
    assert!(!timer.run().has_been_modified());
}
