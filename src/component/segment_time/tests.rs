use super::Component;
use crate::{
    tests_helper::{self, make_progress_run_with_splits_opt, run_with_splits, start_run},
    Timer,
};

fn create_timer() -> Timer {
    let mut timer = tests_helper::create_timer(&["A", "B"]);
    run_with_splits(&mut timer, &[12.3, 45.6]);
    timer
}

#[test]
fn is_empty_when_no_attempt_is_started() {
    let component = Component::new();
    let timer = create_timer();
    let state = component.state(&timer);
    assert_eq!(&*state.value, "—");
}

#[test]
fn is_not_empty_when_attempt_is_started() {
    let component = Component::new();
    let mut timer = create_timer();
    start_run(&mut timer);
    make_progress_run_with_splits_opt(&mut timer, &[Some(467.23)]);
    let state = component.state(&timer);
    assert_eq!(&*state.value, "33.30");
}

#[test]
fn is_not_empty_when_attempt_is_paused() {
    let component = Component::new();
    let mut timer = create_timer();
    start_run(&mut timer);
    make_progress_run_with_splits_opt(&mut timer, &[Some(467.23)]);
    timer.pause();
    let state = component.state(&timer);
    assert_eq!(&*state.value, "33.30");
}

#[test]
fn is_empty_when_attempt_is_finished() {
    let component = Component::new();
    let mut timer = create_timer();
    start_run(&mut timer);
    make_progress_run_with_splits_opt(&mut timer, &[Some(467.23), Some(742.65)]);
    let state = component.state(&timer);
    assert_eq!(&*state.value, "—");
}
