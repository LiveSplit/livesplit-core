use super::{Component, Settings, State};
use crate::{
    GeneralLayoutSettings, Lang, TimeSpan, Timer, TimingMethod,
    comparison::{average_segments, median_segments, personal_best},
    timing::formatter::{Accuracy, DigitsFormat},
    util::tests_helper::{create_run, start_run},
};

const TARGET: &str = "Target";
const COMPARISONS: [&str; 4] = [
    personal_best::NAME,
    TARGET,
    median_segments::NAME,
    average_segments::NAME,
];

fn timer_with_comparisons(times: [Option<f64>; 4], method: TimingMethod) -> Timer {
    let mut run = create_run(&["A", "B"]);
    // Explicit comparison values let each source be tested independently.
    run.comparison_generators_mut().clear();
    for comparison in &COMPARISONS[1..] {
        run.add_custom_comparison(*comparison).unwrap();
    }
    for (comparison, time) in COMPARISONS.into_iter().zip(times) {
        run.segment_mut(1).comparison_mut(comparison)[method] = time.map(TimeSpan::from_seconds);
    }
    let mut timer = Timer::new(run).unwrap();
    timer.set_current_comparison(TARGET).unwrap();
    timer.set_current_timing_method(method);
    timer
}

fn state(component: &Component, timer: &Timer) -> State {
    component.state(
        &timer.snapshot(),
        &GeneralLayoutSettings::default(),
        Lang::English,
    )
}

#[test]
fn reserves_largest_final_comparison_with_twenty_percent_buffer() {
    for largest in 0..4 {
        let mut times = [Some(30.0); 4];
        times[largest] = Some(3200.0);
        let timer = timer_with_comparisons(times, TimingMethod::RealTime);
        let state = state(&Component::new(), &timer);
        assert_eq!(state.time, "0");
        assert_eq!(state.time_size_hint, "8:88:88");
        assert_eq!(state.fraction_size_hint, ".88");
    }
}

#[test]
fn reserves_one_hour_without_final_times() {
    let mut run = create_run(&["A", "B"]);
    run.segment_mut(0).personal_best_split_time_mut().real_time =
        Some(TimeSpan::from_seconds(30.0));
    let timer = Timer::new(run).unwrap();
    assert_eq!(state(&Component::new(), &timer).time_size_hint, "8:88:88");
}

#[test]
fn uses_available_comparison_without_personal_best() {
    let timer = timer_with_comparisons([None, Some(50.0), None, None], TimingMethod::RealTime);
    assert_eq!(state(&Component::new(), &timer).time_size_hint, "8:88");
}

#[test]
fn respects_timing_method_override_and_real_time_fallback() {
    let mut timer =
        timer_with_comparisons([Some(3200.0), None, None, None], TimingMethod::RealTime);
    let component = Component::with_settings(Settings {
        timing_method: Some(TimingMethod::GameTime),
        ..Default::default()
    });
    assert_eq!(state(&component, &timer).time_size_hint, "8:88:88");

    let mut run = timer.into_run(false);
    run.segment_mut(1).personal_best_split_time_mut().game_time =
        Some(TimeSpan::from_seconds(120.0));
    timer = Timer::new(run).unwrap();
    assert_eq!(state(&component, &timer).time_size_hint, "8:88");
    assert_eq!(state(&Component::new(), &timer).time_size_hint, "8:88:88");
}

#[test]
fn size_hint_stays_stable_until_the_timer_needs_more_digits() {
    let mut timer =
        timer_with_comparisons([Some(3200.0), None, None, None], TimingMethod::GameTime);
    let component = Component::new();
    start_run(&mut timer);
    let mut state = state(&component, &timer);
    for (seconds, text, hint) in [
        (1.0, "1", "8:88:88"),
        (10.0, "10", "8:88:88"),
        (60.0, "1:00", "8:88:88"),
        (3600.0, "1:00:00", "8:88:88"),
        (35999.0, "9:59:59", "8:88:88"),
        (36000.0, "10:00:00", "88:88:88"),
        (36001.0, "10:00:01", "88:88:88"),
        (360000.0, "100:00:00", "888:88:88"),
    ] {
        timer
            .set_game_time(TimeSpan::from_seconds(seconds))
            .unwrap();
        component.update_state(
            &mut state,
            &timer.snapshot(),
            &GeneralLayoutSettings::default(),
            Lang::English,
        );
        assert_eq!(state.time, text);
        assert_eq!(state.time_size_hint, hint);
    }
}

#[test]
fn size_hint_covers_negative_times_and_the_missing_time_glyph() {
    let mut timer = timer_with_comparisons([Some(5.0), None, None, None], TimingMethod::GameTime);
    let component = Component::new();
    start_run(&mut timer);
    for (seconds, hint) in [(-0.1, "−8"), (-60.0, "−8:88"), (-36000.0, "−88:88:88")] {
        timer
            .set_game_time(TimeSpan::from_seconds(seconds))
            .unwrap();
        assert_eq!(state(&component, &timer).time_size_hint, hint);
    }
    let segment_timer = Component::with_settings(Settings {
        is_segment_timer: true,
        ..Default::default()
    });
    timer.skip_split().unwrap();
    let state = state(&segment_timer, &timer);
    assert_eq!(state.time, "—");
    assert!(state.time_size_hint.starts_with('—'));
}

#[test]
fn hints_ignore_digit_values_when_the_format_stays_the_same() {
    let component = Component::new();
    let a = timer_with_comparisons([Some(101.01), None, None, None], TimingMethod::RealTime);
    let b = timer_with_comparisons([Some(201.23), None, None, None], TimingMethod::RealTime);
    let a = state(&component, &a);
    let b = state(&component, &b);
    assert_eq!(a.time_size_hint, "8:88");
    assert_eq!(a.time_size_hint, b.time_size_hint);
    assert_eq!(a.fraction_size_hint, ".88");
    assert_eq!(a.fraction_size_hint, b.fraction_size_hint);
}

#[test]
fn hints_follow_digits_accuracy_and_decimal_separator() {
    let timer = timer_with_comparisons([Some(5.0), None, None, None], TimingMethod::RealTime);
    let mut component = Component::with_settings(Settings {
        digits_format: DigitsFormat::DoubleDigitHours,
        accuracy: Accuracy::Milliseconds,
        ..Default::default()
    });
    let localized_state = component.state(
        &timer.snapshot(),
        &GeneralLayoutSettings::default(),
        Lang::English,
    );
    assert_eq!(localized_state.time_size_hint, "88:88:88");
    assert_eq!(localized_state.fraction_size_hint, ".888");
    #[cfg(feature = "localization")]
    {
        let localized_state = component.state(
            &timer.snapshot(),
            &GeneralLayoutSettings::default(),
            Lang::German,
        );
        assert_eq!(localized_state.fraction_size_hint, ",888");
    }
    component.settings_mut().accuracy = Accuracy::Seconds;
    assert!(state(&component, &timer).fraction_size_hint.is_empty());
}

#[test]
fn segment_timer_reserves_ten_minutes_without_segment_times() {
    let timer = timer_with_comparisons([None; 4], TimingMethod::GameTime);
    let component = Component::with_settings(Settings {
        is_segment_timer: true,
        ..Default::default()
    });
    assert_eq!(state(&component, &timer).time_size_hint, "88:88");
}

#[test]
fn segment_timer_uses_short_known_times_despite_missing_times() {
    let component = Component::with_settings(Settings {
        is_segment_timer: true,
        ..Default::default()
    });
    for comparison in COMPARISONS {
        let timer = timer_with_comparisons([None; 4], TimingMethod::GameTime);
        let mut run = timer.into_run(false);
        run.segment_mut(0).comparison_mut(comparison).game_time = Some(TimeSpan::from_seconds(5.0));
        let mut timer = Timer::new(run).unwrap();
        timer.set_current_comparison(TARGET).unwrap();
        timer.set_current_timing_method(TimingMethod::GameTime);
        assert_eq!(state(&component, &timer).time_size_hint, "8");
    }
}

#[test]
fn segment_timer_reserves_longest_segment_instead_of_total_duration() {
    let timer = timer_with_comparisons(
        [Some(100.0), Some(5000.0), None, None],
        TimingMethod::GameTime,
    );
    let mut run = timer.into_run(false);
    run.segment_mut(0).personal_best_split_time_mut().game_time =
        Some(TimeSpan::from_seconds(80.0));
    run.segment_mut(0).comparison_mut(TARGET).game_time = Some(TimeSpan::from_seconds(2500.0));
    let mut timer = Timer::new(run).unwrap();
    timer.set_current_comparison(TARGET).unwrap();
    let component = Component::with_settings(Settings {
        is_segment_timer: true,
        timing_method: Some(TimingMethod::GameTime),
        ..Default::default()
    });
    assert_eq!(state(&component, &timer).time_size_hint, "88:88");
}

#[test]
fn segment_hint_is_stable_across_segments_and_timer_phases() {
    let mut run = create_run(&["Short", "Long", "Short Again"]);
    for (index, seconds) in [1.0, 51.0, 52.0].into_iter().enumerate() {
        run.segment_mut(index)
            .personal_best_split_time_mut()
            .game_time = Some(TimeSpan::from_seconds(seconds));
    }
    let mut timer = Timer::new(run).unwrap();
    timer.set_current_timing_method(TimingMethod::GameTime);
    let component = Component::with_settings(Settings {
        is_segment_timer: true,
        ..Default::default()
    });
    // The long middle segment needs a minute-sized hint after the buffer,
    // even while the timer is showing either of the one-second segments.
    assert_eq!(state(&component, &timer).time_size_hint, "8:88");
    start_run(&mut timer);
    for seconds in [1.0, 51.0, 52.0] {
        assert_eq!(state(&component, &timer).time_size_hint, "8:88");
        timer
            .set_game_time(TimeSpan::from_seconds(seconds))
            .unwrap();
        timer.split().unwrap();
    }
    assert_eq!(state(&component, &timer).time_size_hint, "8:88");
    timer.reset(false).unwrap();
    assert_eq!(state(&component, &timer).time_size_hint, "8:88");
}
