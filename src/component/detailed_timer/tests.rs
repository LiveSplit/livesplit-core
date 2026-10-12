use super::{Component, IconDisplayMode, Settings, State};
use crate::{
    GeneralLayoutSettings, Lang, Run, Segment, TimeSpan, Timer, TimingMethod,
    comparison::{best_segments, none, personal_best},
    settings::{Image, ImageCache, SemanticColor, Value},
    timing::formatter::Accuracy,
    util::tests_helper::{create_run, start_run},
};

#[track_caller]
fn state(component: &Component, timer: &Timer) -> State {
    component.state(
        &mut ImageCache::new(),
        &timer.snapshot(),
        &GeneralLayoutSettings::default(),
        Lang::English,
    )
}

#[test]
fn constructors_produce_equivalent_settings_and_states() {
    let new = Component::new();
    let default = Component::default();
    let expected_settings = serde_json::to_value(Settings::default()).unwrap();
    assert_eq!(
        serde_json::to_value(new.settings()).unwrap(),
        expected_settings
    );
    assert_eq!(
        serde_json::to_value(default.settings()).unwrap(),
        expected_settings
    );

    let mut timer = Timer::new(create_run(&["First", "Second"])).unwrap();
    for phase in 0..=5 {
        match phase {
            1 => start_run(&mut timer),
            2 => {
                timer.set_game_time(TimeSpan::from_seconds(10.0)).unwrap();
                timer.split().unwrap();
            }
            3 => {
                timer.pause().unwrap();
            }
            4 => {
                timer.resume().unwrap();
                timer.set_game_time(TimeSpan::from_seconds(35.25)).unwrap();
                timer.split().unwrap();
            }
            5 => {
                timer.reset(false).unwrap();
            }
            _ => {}
        }
        let snapshot = timer.snapshot();
        let mut images = ImageCache::new();
        let layout = GeneralLayoutSettings::default();
        let expected = new.state(&mut images, &snapshot, &layout, Lang::English);
        let actual = default.state(&mut images, &snapshot, &layout, Lang::English);
        // Compare the full public state, including hints, colors, and update
        // frequency, with both constructors observing the same timer snapshot.
        assert_eq!(
            serde_json::to_value(actual).unwrap(),
            serde_json::to_value(expected).unwrap(),
            "constructors differ at phase {phase}"
        );
    }
}

#[test]
fn constructors_use_segment_timing_and_the_default_segment_color() {
    let mut run = create_run(&["First", "Second"]);
    for (segment, seconds) in run.segments_mut().iter_mut().zip([10.0, 100.0]) {
        segment.personal_best_split_time_mut().game_time = Some(TimeSpan::from_seconds(seconds));
    }
    let mut timer = Timer::new(run).unwrap();
    start_run(&mut timer);
    timer.set_game_time(TimeSpan::from_seconds(10.0)).unwrap();
    timer.split().unwrap();
    timer.set_game_time(TimeSpan::from_seconds(35.25)).unwrap();
    let expected_colors = super::timer::top_and_bottom_color(super::SEGMENT_TIMER_DEFAULT_COLOR);

    for component in [Component::new(), Component::default()] {
        assert!(!component.settings().timer.is_segment_timer);
        assert!(component.settings().segment_timer.is_segment_timer);
        assert_eq!(component.settings().timer.height, 40);
        assert_eq!(component.settings().segment_timer.height, 25);
        let state = state(&component, &timer);
        assert_eq!(state.timer.height, 40);
        assert_eq!(state.segment_timer.height, 25);
        assert_eq!(state.timer.time, "35");
        assert_eq!(state.segment_timer.time, "25");
        assert_eq!(state.segment_timer.fraction, ".25");
        assert_eq!(state.segment_timer.semantic_color, SemanticColor::Default);
        assert_eq!(
            (
                state.segment_timer.top_color,
                state.segment_timer.bottom_color
            ),
            expected_colors
        );
        assert_ne!(
            (state.timer.top_color, state.timer.bottom_color),
            expected_colors,
            "the segment color is independent of the attempt's comparison color"
        );
    }
}

#[test]
fn constructors_apply_segment_height_edits_from_the_correct_default() {
    let timer = Timer::new(create_run(&["First"])).unwrap();
    for mut component in [Component::new(), Component::default()] {
        let before = state(&component, &timer);
        assert_eq!(component.settings().segment_timer.height, 25);
        assert_eq!(before.segment_timer.height, 25);

        // Exercise the editor's single-setting update rather than replacing
        // all settings, which would conceal incorrectly initialized timers.
        component.set_value(6, Value::UInt(26));
        let after = state(&component, &timer);
        assert_eq!(component.settings().segment_timer.height, 26);
        assert_eq!(after.segment_timer.height, before.segment_timer.height + 1);
        assert_eq!(after.segment_timer.height, 26);
        assert_eq!(after.timer.height, 40);
    }
}

const TARGET: &str = "Target";

#[track_caller]
fn timer_with_target(times: &[Option<f64>], method: TimingMethod) -> Timer {
    let mut run = Run::new();
    run.add_custom_comparison(TARGET).unwrap();
    for &time in times {
        let mut segment = Segment::new("Segment");
        segment.comparison_mut(TARGET)[method] = time.map(TimeSpan::from_seconds);
        run.push_segment(segment);
    }
    let mut timer = Timer::new(run).unwrap();
    timer.set_current_comparison(TARGET).unwrap();
    timer.set_current_timing_method(method);
    timer
}

#[test]
fn comparison_hints_reserve_their_own_longest_segment_instead_of_the_total() {
    let mut run = create_run(&["Short", "Long", "Short Again"]);
    run.add_custom_comparison(TARGET).unwrap();
    for (index, (pb, target, best)) in [
        (1000.0, 50.0, 50.0),
        (2000.0, 3200.0, 60.0),
        (3000.0, 3250.0, 80.0),
    ]
    .into_iter()
    .enumerate()
    {
        let segment = run.segment_mut(index);
        segment.personal_best_split_time_mut().game_time = Some(TimeSpan::from_seconds(pb));
        segment.comparison_mut(TARGET).game_time = Some(TimeSpan::from_seconds(target));
        segment.best_segment_time_mut().game_time = Some(TimeSpan::from_seconds(best));
    }
    let mut timer = Timer::new(run).unwrap();
    timer.set_current_timing_method(TimingMethod::GameTime);
    let pb = state(&Component::new(), &timer);
    assert_eq!(
        pb.comparison_size_hints[0].as_ref().unwrap().time,
        "88:88.88"
    );
    assert_eq!(
        pb.comparison_size_hints[1].as_ref().unwrap().time,
        "8:88.88"
    );
    timer.set_current_comparison(TARGET).unwrap();
    let target = state(&Component::new(), &timer);
    assert_eq!(
        target.comparison_size_hints[0].as_ref().unwrap().time,
        "88:88.88"
    );
    assert_eq!(
        target.comparison_size_hints[1].as_ref().unwrap().time,
        "8:88.88"
    );
}

#[test]
fn comparison_hints_use_the_exact_maximum_and_normalize_digits() {
    let component = Component::with_settings(Settings {
        hide_second_comparison: true,
        ..Default::default()
    });
    for (seconds, expected) in [
        (5.0, "8.88"),
        (9.99, "8.88"),
        (10.0, "88.88"),
        (49.0, "88.88"),
        (50.0, "88.88"),
        (59.99, "88.88"),
        (60.0, "8:88.88"),
        (500.0, "8:88.88"),
        (599.99, "8:88.88"),
        (600.0, "88:88.88"),
        (3000.0, "88:88.88"),
        (3599.99, "88:88.88"),
        (3600.0, "8:88:88.88"),
        (30000.0, "8:88:88.88"),
        (35999.99, "8:88:88.88"),
        (36000.0, "88:88:88.88"),
    ] {
        let timer = timer_with_target(&[Some(seconds)], TimingMethod::GameTime);
        assert_eq!(
            state(&component, &timer).comparison_size_hints[0]
                .as_ref()
                .unwrap()
                .time,
            expected
        );
    }
}

#[test]
fn comparison_hints_only_count_segment_times_that_can_be_displayed() {
    let component = Component::new();
    for (times, expected) in [
        ([Some(5.0), None, Some(5000.0), Some(5005.0)], "8.88"),
        ([None, Some(50.0), None, Some(5000.0)], "—"),
        ([None; 4], "—"),
        ([Some(0.0); 4], "8.88"),
    ] {
        let timer = timer_with_target(&times, TimingMethod::GameTime);
        assert_eq!(
            state(&component, &timer).comparison_size_hints[0]
                .as_ref()
                .unwrap()
                .time,
            expected
        );
    }
}

#[test]
fn comparison_hints_follow_the_comparison_timing_method_override() {
    let timer = timer_with_target(&[Some(50.0)], TimingMethod::RealTime);
    let mut run = timer.into_run(false);
    run.segment_mut(0).comparison_mut(TARGET).game_time = Some(TimeSpan::from_seconds(3200.0));
    let mut timer = Timer::new(run).unwrap();
    timer.set_current_comparison(TARGET).unwrap();
    timer.set_current_timing_method(TimingMethod::GameTime);
    for (method, expected) in [
        (TimingMethod::RealTime, "88.88"),
        (TimingMethod::GameTime, "88:88.88"),
    ] {
        let mut settings = Settings::default();
        settings.timer.timing_method = Some(method);
        let component = Component::with_settings(settings);
        assert_eq!(
            state(&component, &timer).comparison_size_hints[0]
                .as_ref()
                .unwrap()
                .time,
            expected
        );
    }
}

#[test]
fn comparison_hints_follow_accuracy_and_decimal_separator() {
    let timer = timer_with_target(&[Some(50.0)], TimingMethod::GameTime);
    for (accuracy, fraction) in [
        (Accuracy::Seconds, ""),
        (Accuracy::Tenths, ".8"),
        (Accuracy::Hundredths, ".88"),
        (Accuracy::Milliseconds, ".888"),
    ] {
        let component = Component::with_settings(Settings {
            comparison_times_accuracy: accuracy,
            ..Default::default()
        });
        let expected = format!("88{fraction}");
        assert_eq!(
            state(&component, &timer).comparison_size_hints[0]
                .as_ref()
                .unwrap()
                .time,
            expected
        );
        #[cfg(feature = "localization")]
        {
            let localized = component.state(
                &mut ImageCache::new(),
                &timer.snapshot(),
                &GeneralLayoutSettings::default(),
                Lang::German,
            );
            assert_eq!(
                localized.comparison_size_hints[0].as_ref().unwrap().time,
                expected.replace('.', ",")
            );
        }
    }
}

#[test]
fn comparison_reservations_use_the_same_selection_before_and_during_an_attempt() {
    for (first, second, hide_second, names) in [
        (None, None, false, &["PB"][..]),
        (None, Some(best_segments::NAME), false, &["PB", "Best"]),
        (None, Some(best_segments::NAME), true, &["PB"]),
        (
            Some(personal_best::NAME),
            Some(personal_best::NAME),
            false,
            &["PB"],
        ),
        (Some("Missing"), Some(best_segments::NAME), false, &["Best"]),
        (
            Some(none::NAME),
            Some(best_segments::NAME),
            false,
            &["Best"],
        ),
        (None, Some(none::NAME), false, &["PB"]),
        (Some("Missing"), Some("Missing"), false, &["PB"]),
        (Some(none::NAME), Some(best_segments::NAME), true, &["PB"]),
    ] {
        let mut timer = Timer::new(create_run(&["Start"])).unwrap();
        let component = Component::with_settings(Settings {
            comparison1: first.map(Into::into),
            comparison2: second.map(Into::into),
            hide_second_comparison: hide_second,
            ..Default::default()
        });
        let before = state(&component, &timer);
        assert!(before.comparison1.is_none() && before.comparison2.is_none());
        assert!(
            before
                .comparison_size_hints
                .iter()
                .flatten()
                .map(|h| h.name.as_str())
                .eq(names.iter().copied())
        );
        start_run(&mut timer);
        let after = state(&component, &timer);
        assert!(
            [after.comparison1, after.comparison2]
                .iter()
                .flatten()
                .map(|c| c.name.as_str())
                .eq(names.iter().copied())
        );
        assert!(
            after
                .comparison_size_hints
                .iter()
                .flatten()
                .map(|h| (&h.name, &h.time))
                .eq(before
                    .comparison_size_hints
                    .iter()
                    .flatten()
                    .map(|h| (&h.name, &h.time)))
        );
    }
}

#[test]
fn comparison_reservations_update_reused_states_when_configuration_changes() {
    let mut run = timer_with_target(&[Some(3200.0)], TimingMethod::GameTime).into_run(false);
    run.segment_mut(0).personal_best_split_time_mut().game_time = Some(TimeSpan::from_seconds(5.0));
    run.segment_mut(0).best_segment_time_mut().game_time = Some(TimeSpan::from_seconds(2.5));
    let mut timer = Timer::new(run).unwrap();
    timer.set_current_comparison(TARGET).unwrap();
    timer.set_current_timing_method(TimingMethod::GameTime);
    let mut component = Component::new();
    let mut state = State::default();
    let mut images = ImageCache::new();
    let layout = GeneralLayoutSettings::default();

    for (first, second, hide_second, expected) in [
        (
            None,
            Some(best_segments::NAME),
            false,
            [Some((TARGET, "88:88.88")), Some(("Best", "8.88"))],
        ),
        (None, None, false, [Some((TARGET, "88:88.88")), None]),
        (
            Some(personal_best::NAME),
            Some(TARGET),
            false,
            [Some(("PB", "8.88")), Some((TARGET, "88:88.88"))],
        ),
        (
            Some(best_segments::NAME),
            Some(TARGET),
            true,
            [Some(("Best", "8.88")), None],
        ),
        (
            Some(personal_best::NAME),
            Some(best_segments::NAME),
            false,
            [Some(("PB", "8.88")), Some(("Best", "8.88"))],
        ),
    ] {
        component.set_settings(Settings {
            comparison1: first.map(Into::into),
            comparison2: second.map(Into::into),
            hide_second_comparison: hide_second,
            ..Default::default()
        });
        component.update_state(
            &mut state,
            &mut images,
            &timer.snapshot(),
            &layout,
            Lang::English,
        );
        // Reusing a state must replace old names and formats and remove the
        // second reservation when a comparison is disabled or deduplicated.
        assert_eq!(
            state
                .comparison_size_hints
                .each_ref()
                .map(|hint| hint.as_ref().map(|h| (h.name.as_str(), h.time.as_str()))),
            expected,
        );
        assert!(state.comparison1.is_none() && state.comparison2.is_none());
    }
}

#[test]
fn sizes_segment_timer_for_the_longest_segment() {
    let mut run = Run::new();
    for seconds in [100.0, 3000.0, 5900.0] {
        let mut segment = Segment::new("Segment");
        segment.personal_best_split_time_mut().real_time = Some(TimeSpan::from_seconds(seconds));
        run.push_segment(segment);
    }
    let timer = Timer::new(run).unwrap();
    let state = Component::new().state(
        &mut ImageCache::new(),
        &timer.snapshot(),
        &GeneralLayoutSettings::default(),
        Lang::English,
    );
    assert_eq!(state.timer.time_size_hint, "8:88:88");
    assert_eq!(state.segment_timer.time_size_hint, "88:88");
}

#[track_caller]
fn prepare() -> (Timer, Component, GeneralLayoutSettings, ImageCache) {
    let mut run = Run::new();
    let mut segment = Segment::new("foo");
    segment.set_icon(Image::new([0x00, 0x12, 0x34].into(), Image::ICON));
    run.push_segment(segment);
    let timer = Timer::new(run).unwrap();

    let component = Component::with_settings(Settings {
        display_icon: IconDisplayMode::BothRows,
        show_segment_name: true,
        ..Default::default()
    });

    (
        timer,
        component,
        GeneralLayoutSettings::default(),
        ImageCache::new(),
    )
}

#[test]
fn doesnt_show_segment_name_outside_attempt() {
    let (timer, component, layout_settings, mut image_cache) = prepare();

    assert_eq!(
        component
            .state(
                &mut image_cache,
                &timer.snapshot(),
                &layout_settings,
                Lang::English
            )
            .segment_name,
        None
    );
}

#[test]
fn shows_segment_name_during_attempt() {
    let (mut timer, component, layout_settings, mut image_cache) = prepare();

    timer.start().unwrap();

    assert_eq!(
        component
            .state(
                &mut image_cache,
                &timer.snapshot(),
                &layout_settings,
                Lang::English
            )
            .segment_name
            .unwrap(),
        "foo",
    );
}

#[test]
fn shows_segment_name_at_the_end_of_an_attempt() {
    let (mut timer, component, layout_settings, mut image_cache) = prepare();

    timer.start().unwrap();
    timer.split().unwrap();

    assert_eq!(
        component
            .state(
                &mut image_cache,
                &timer.snapshot(),
                &layout_settings,
                Lang::English
            )
            .segment_name
            .unwrap(),
        "foo",
    );
}

#[test]
fn stops_showing_segment_name_when_resetting() {
    let (mut timer, component, layout_settings, mut image_cache) = prepare();

    timer.start().unwrap();
    timer.split().unwrap();
    timer.reset(true).unwrap();

    assert_eq!(
        component
            .state(
                &mut image_cache,
                &timer.snapshot(),
                &layout_settings,
                Lang::English
            )
            .segment_name,
        None
    );
}

#[test]
fn doesnt_show_icon_outside_attempt() {
    let (timer, component, layout_settings, mut image_cache) = prepare();

    assert!(
        component
            .state(
                &mut image_cache,
                &timer.snapshot(),
                &layout_settings,
                Lang::English
            )
            .icon
            .is_empty()
    );
}

#[test]
fn shows_icon_during_attempt() {
    let (mut timer, component, layout_settings, mut image_cache) = prepare();

    component.state(
        &mut image_cache,
        &timer.snapshot(),
        &layout_settings,
        Lang::English,
    );

    timer.start().unwrap();

    assert!(
        !component
            .state(
                &mut image_cache,
                &timer.snapshot(),
                &layout_settings,
                Lang::English
            )
            .icon
            .is_empty()
    );
}

#[test]
fn still_shows_icon_of_last_segment_at_the_end_of_an_attempt() {
    let (mut timer, component, layout_settings, mut image_cache) = prepare();

    component.state(
        &mut image_cache,
        &timer.snapshot(),
        &layout_settings,
        Lang::English,
    );

    timer.start().unwrap();

    component.state(
        &mut image_cache,
        &timer.snapshot(),
        &layout_settings,
        Lang::English,
    );

    timer.split().unwrap();

    assert!(
        !component
            .state(
                &mut image_cache,
                &timer.snapshot(),
                &layout_settings,
                Lang::English
            )
            .icon
            .is_empty()
    );
}

#[test]
fn stops_showing_icon_when_resetting() {
    let (mut timer, component, layout_settings, mut image_cache) = prepare();

    component.state(
        &mut image_cache,
        &timer.snapshot(),
        &layout_settings,
        Lang::English,
    );

    timer.start().unwrap();

    component.state(
        &mut image_cache,
        &timer.snapshot(),
        &layout_settings,
        Lang::English,
    );

    timer.split().unwrap();

    component.state(
        &mut image_cache,
        &timer.snapshot(),
        &layout_settings,
        Lang::English,
    );

    timer.reset(true).unwrap();

    assert!(
        component
            .state(
                &mut image_cache,
                &timer.snapshot(),
                &layout_settings,
                Lang::English
            )
            .icon
            .is_empty()
    );
}

#[test]
fn reserves_all_names_and_the_shortest_configured_timer_format() {
    use crate::timing::formatter::DigitsFormat;
    let mut run = Run::new();
    for name in ["Start", "Ancient Underground Laboratory Escape", ""] {
        run.push_segment(Segment::new(name));
    }
    let mut timer = Timer::new(run).unwrap();
    timer.set_current_timing_method(crate::TimingMethod::GameTime);
    timer.start().unwrap();
    timer.initialize_game_time().unwrap();
    timer.pause_game_time().unwrap();
    timer.set_game_time(TimeSpan::zero()).unwrap();
    let mut settings = Settings {
        show_segment_name: true,
        ..Default::default()
    };
    for format in [
        DigitsFormat::SingleDigitSeconds,
        DigitsFormat::DoubleDigitSeconds,
        DigitsFormat::SingleDigitMinutes,
        DigitsFormat::DoubleDigitMinutes,
        DigitsFormat::SingleDigitHours,
        DigitsFormat::DoubleDigitHours,
    ] {
        settings.timer.digits_format = format;
        let component = Component::with_settings(settings.clone());
        let state = component.state(
            &mut ImageCache::new(),
            &timer.snapshot(),
            &GeneralLayoutSettings::default(),
            Lang::English,
        );
        // Compare with the actual zero-time formatter rather than duplicating
        // the renderer's format table in this test.
        let shortest = state.timer.time.replace('0', "8");
        assert_eq!(state.timer_minimum_size_hint, shortest);
        assert!(
            state
                .segment_name_size_hints
                .iter()
                .map(|hint| hint.name.as_str())
                .eq(["Start", "Ancient Underground Laboratory Escape", "",])
        );
    }
    settings.show_segment_name = false;
    let state = Component::with_settings(settings).state(
        &mut ImageCache::new(),
        &timer.snapshot(),
        &GeneralLayoutSettings::default(),
        Lang::English,
    );
    assert!(state.segment_name_size_hints.is_empty());
}

#[test]
fn name_reservations_include_future_icon_presence_and_update_without_stale_flags() {
    let mut run = create_run(&["Start", "Underground Laboratory Escape", "Finish"]);
    let icon = Image::new([1].into(), Image::ICON);
    run.segment_mut(0).set_icon(icon.clone());
    run.segment_mut(2).set_icon(icon.clone());
    let mut timer = Timer::new(run).unwrap();
    let component = Component::with_settings(Settings {
        show_segment_name: true,
        display_icon: IconDisplayMode::FirstRow,
        ..Default::default()
    });
    let mut images = ImageCache::new();
    let mut state = component.state(
        &mut images,
        &timer.snapshot(),
        &GeneralLayoutSettings::default(),
        Lang::English,
    );
    assert!(
        state
            .segment_name_size_hints
            .iter()
            .map(|hint| (hint.name.as_str(), hint.has_icon))
            .eq([
                ("Start", true),
                ("Underground Laboratory Escape", false),
                ("Finish", true)
            ])
    );
    // Changing a future segment must invalidate the name reservation even
    // though its image and name are not the ones currently being displayed.
    let mut before = std::collections::hash_map::DefaultHasher::new();
    state.content_fingerprint(&mut before);
    let mut run = timer.run().clone();
    run.segment_mut(1).set_icon(icon);
    run.segment_mut(2).set_icon(Image::EMPTY.clone());
    timer = Timer::new(run).unwrap();
    component.update_state(
        &mut state,
        &mut images,
        &timer.snapshot(),
        &GeneralLayoutSettings::default(),
        Lang::English,
    );
    assert!(
        state
            .segment_name_size_hints
            .iter()
            .map(|hint| hint.has_icon)
            .eq([true, true, false])
    );
    let mut after = std::collections::hash_map::DefaultHasher::new();
    state.content_fingerprint(&mut after);
    use core::hash::Hasher;
    assert_ne!(before.finish(), after.finish());
    let serialized = serde_json::to_value(&state).unwrap();
    assert_eq!(
        serialized["segment_name_size_hints"][2],
        serde_json::json!({"name":"Finish","has_icon":false})
    );
}

#[test]
fn exposes_icon_display_mode_in_state() {
    let (mut timer, mut component, layout_settings, mut image_cache) = prepare();
    timer.start().unwrap();

    // Setting indices are part of the generic component settings API. Keep
    // this assertion on the generic path so layout editors are covered in
    // addition to callers that modify the strongly typed settings directly.
    component.set_value(20, Value::IconDisplayMode(IconDisplayMode::FirstRow));

    let state = component.state(
        &mut image_cache,
        &timer.snapshot(),
        &layout_settings,
        Lang::English,
    );
    let description = component.settings_description(Lang::English);

    assert_eq!(state.display_icon, IconDisplayMode::FirstRow);
    assert!(matches!(
        description.fields[20].value,
        Value::IconDisplayMode(IconDisplayMode::FirstRow)
    ));
}

#[test]
fn deserializes_released_display_icon_setting() {
    let hidden: Settings = serde_json::from_str(r#"{"display_icon":false}"#).unwrap();
    let visible: Settings = serde_json::from_str(r#"{"display_icon":true}"#).unwrap();

    assert_eq!(hidden.display_icon, IconDisplayMode::Hidden);
    assert_eq!(visible.display_icon, IconDisplayMode::BothRows);
}

#[test]
fn serializes_and_deserializes_display_icon_mode() {
    let settings = Settings {
        display_icon: IconDisplayMode::FirstRow,
        ..Default::default()
    };

    let json = serde_json::to_string(&settings).unwrap();
    assert!(json.contains(r#""display_icon":"FirstRow""#));

    let deserialized: Settings = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.display_icon, IconDisplayMode::FirstRow);
}
