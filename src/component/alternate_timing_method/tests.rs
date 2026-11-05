use super::Component;
use crate::{
    TimeSpan, TimingMethod,
    localization::Lang,
    timing::formatter::DASH,
    util::tests_helper::{create_timer, start_run},
};

#[test]
fn shows_the_other_timing_method() {
    let mut timer = create_timer(&["A"]);
    let component = Component::new();

    timer.set_current_timing_method(TimingMethod::RealTime);
    assert_eq!(&*component.state(&timer, Lang::English).key, "Game Time");

    timer.set_current_timing_method(TimingMethod::GameTime);
    assert_eq!(&*component.state(&timer, Lang::English).key, "Real Time");
}

#[test]
fn shows_game_time_when_comparing_against_real_time() {
    let mut timer = create_timer(&["A"]);
    let component = Component::new();

    start_run(&mut timer);
    timer.set_game_time(TimeSpan::from_seconds(83.456)).unwrap();
    timer.set_current_timing_method(TimingMethod::RealTime);

    let state = component.state(&timer, Lang::English);
    assert_eq!(&*state.key, "Game Time");
    assert_eq!(&*state.value, "1:23.45");
}

#[test]
fn shows_dash_without_game_time() {
    let mut timer = create_timer(&["A"]);
    let component = Component::new();

    timer.set_current_timing_method(TimingMethod::RealTime);
    timer.start().unwrap();

    let state = component.state(&timer, Lang::English);
    assert_eq!(&*state.key, "Game Time");
    assert_eq!(&*state.value, DASH);
    assert!(!state.updates_frequently);
}
