use super::{ColumnSettings, ColumnStartWith, ColumnUpdateWith, Component, Settings, State};
use settings::SemanticColor;
use tests_helper::{run_with_splits_opt, start_run};
use {Run, Segment, TimeSpan, Timer, TimingMethod};

const NONE: SemanticColor = SemanticColor::Default;
const BEST: SemanticColor = SemanticColor::BestSegment;
const BEHIND_LOSING: SemanticColor = SemanticColor::BehindLosingTime;
const BEHIND_GAINING: SemanticColor = SemanticColor::BehindGainingTime;
const AHEAD_GAINING: SemanticColor = SemanticColor::AheadGainingTime;

type Values = [([&'static str; 6], [SemanticColor; 6]); 7];

fn run() -> Run {
    let mut run = Run::new();

    run.push_segment(Segment::new("A"));
    run.push_segment(Segment::new("B"));
    run.push_segment(Segment::new("C"));
    run.push_segment(Segment::new("D"));
    run.push_segment(Segment::new("E"));
    run.push_segment(Segment::new("F"));

    run
}

fn timer() -> Timer {
    Timer::new(run()).unwrap()
}

fn check_column_state(state: &State, state_index: usize, expected_values: &Values) {
    let actual_values = state
        .splits
        .iter()
        .map(|split| split.columns[0].value.as_str())
        .collect::<Vec<_>>();
    let actual_colors = state
        .splits
        .iter()
        .map(|split| split.columns[0].semantic_color)
        .collect::<Vec<_>>();
    let actual_state = (actual_values, actual_colors);
    let (expected_values, expected_colors) = &expected_values[state_index];
    let expected_state = (expected_values.to_vec(), expected_colors.to_vec());
    assert_eq!(actual_state, expected_state, "State index: {}", state_index);
}

#[test]
fn zero_visual_split_count_always_shows_all_splits() {
    let mut run = Run::new();
    for _ in 0..32 {
        run.push_segment(Segment::new(""));
    }
    let timer = Timer::new(run).unwrap();
    let layout_settings = Default::default();
    let mut component = Component::with_settings(Settings {
        visual_split_count: 0,
        ..Default::default()
    });

    let mut state = component.state(&timer, &layout_settings);
    assert_eq!(state.splits.len(), 32);

    component.scroll_down();
    state = component.state(&timer, &layout_settings);
    assert_eq!(state.splits.len(), 32);

    component.scroll_down();
    state = component.state(&timer, &layout_settings);
    assert_eq!(state.splits.len(), 32);

    component.scroll_up();
    state = component.state(&timer, &layout_settings);
    assert_eq!(state.splits.len(), 32);
}

#[test]
fn negative_segment_times() {
    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    let mut timer = Timer::new(run).unwrap();
    let layout_settings = Default::default();
    let mut component = Component::with_settings(Settings {
        columns: vec![ColumnSettings {
            start_with: ColumnStartWith::Empty,
            update_with: ColumnUpdateWith::SegmentTime,
            ..Default::default()
        }],
        ..Default::default()
    });

    timer.start();

    // Emulate a negative offset through game time.
    timer.set_current_timing_method(TimingMethod::GameTime);
    timer.initialize_game_time();
    timer.pause_game_time();
    timer.set_game_time(TimeSpan::from_seconds(-1.0));

    let state = component.state(&timer, &layout_settings);
    assert_eq!(state.splits[0].columns[0].value, "−0:01");
}

#[test]
fn column_empty_delta() {
    check_columns(
        ColumnStartWith::Empty,
        ColumnUpdateWith::Delta,
        [
            (
                ["", "", "", "", "", ""],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["+3.5", "", "", "", "", ""],
                [BEHIND_LOSING, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["+3.5", "—", "", "", "", ""],
                [BEHIND_LOSING, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["+3.5", "—", "—", "", "", ""],
                [BEHIND_LOSING, NONE, BEST, NONE, NONE, NONE],
            ),
            (
                ["+3.5", "—", "—", "+2.5", "", ""],
                [BEHIND_LOSING, NONE, BEST, BEHIND_GAINING, NONE, NONE],
            ),
            (
                ["+3.5", "—", "—", "+2.5", "—", ""],
                [BEHIND_LOSING, NONE, BEST, BEHIND_GAINING, NONE, NONE],
            ),
            (
                ["+3.5", "—", "—", "+2.5", "—", "−1:00"],
                [
                    BEHIND_LOSING,
                    NONE,
                    BEST,
                    BEHIND_GAINING,
                    NONE,
                    AHEAD_GAINING,
                ],
            ),
        ],
    );
}

#[test]
fn column_empty_segment_delta() {
    check_columns(
        ColumnStartWith::Empty,
        ColumnUpdateWith::SegmentDelta,
        [
            (
                ["", "", "", "", "", ""],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["+3.5", "", "", "", "", ""],
                [BEHIND_LOSING, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["+3.5", "—", "", "", "", ""],
                [BEHIND_LOSING, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["+3.5", "—", "—", "", "", ""],
                [BEHIND_LOSING, NONE, BEST, NONE, NONE, NONE],
            ),
            (
                ["+3.5", "—", "—", "−1.0", "", ""],
                [BEHIND_LOSING, NONE, BEST, AHEAD_GAINING, NONE, NONE],
            ),
            (
                ["+3.5", "—", "—", "−1.0", "—", ""],
                [BEHIND_LOSING, NONE, BEST, AHEAD_GAINING, NONE, NONE],
            ),
            (
                ["+3.5", "—", "—", "−1.0", "—", "−1:02"],
                [
                    BEHIND_LOSING,
                    NONE,
                    BEST,
                    AHEAD_GAINING,
                    NONE,
                    AHEAD_GAINING,
                ],
            ),
        ],
    )
}

#[test]
fn column_empty_split_time() {
    check_columns(
        ColumnStartWith::Empty,
        ColumnUpdateWith::SplitTime,
        [
            (
                ["", "", "", "", "", ""],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "", "", "", "", ""],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "", "", "", ""],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "0:10", "", "", ""],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "0:10", "0:17", "", ""],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "0:10", "0:17", "—", ""],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "0:10", "0:17", "—", "0:25"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
        ],
    )
}

#[test]
fn column_empty_segment_time() {
    check_columns(
        ColumnStartWith::Empty,
        ColumnUpdateWith::SegmentTime,
        [
            (
                ["0:00", "", "", "", "", ""],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "0:00", "", "", "", ""],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "0:00", "", "", ""],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "0:01", "0:00", "", ""],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "0:01", "0:07", "0:00", ""],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "0:01", "0:07", "—", "0:00"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "0:01", "0:07", "—", "0:07"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
        ],
    )
}

#[test]
fn column_comparison_time_split_time() {
    check_columns(
        ColumnStartWith::ComparisonTime,
        ColumnUpdateWith::SplitTime,
        [
            (
                ["0:05", "—", "—", "0:15", "0:20", "1:25"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "—", "0:15", "0:20", "1:25"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "—", "0:15", "0:20", "1:25"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "0:10", "0:15", "0:20", "1:25"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "0:10", "0:17", "0:20", "1:25"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "0:10", "0:17", "—", "1:25"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "0:10", "0:17", "—", "0:25"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
        ],
    )
}

#[test]
fn column_comparison_segment_time_segment_time() {
    check_columns(
        ColumnStartWith::ComparisonSegmentTime,
        ColumnUpdateWith::SegmentTime,
        [
            (
                ["0:00", "—", "—", "0:10", "0:05", "1:05"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "0:00", "—", "0:10", "0:05", "1:05"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "0:00", "0:10", "0:05", "1:05"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "0:01", "0:00", "0:05", "1:05"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "0:01", "0:07", "0:00", "1:05"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "0:01", "0:07", "—", "0:00"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:08", "—", "0:01", "0:07", "—", "0:07"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
        ],
    )
}

#[test]
fn column_comparison_time_delta() {
    check_columns(
        ColumnStartWith::ComparisonTime,
        ColumnUpdateWith::Delta,
        [
            (
                ["0:05", "—", "—", "0:15", "0:20", "1:25"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["+3.5", "—", "—", "0:15", "0:20", "1:25"],
                [BEHIND_LOSING, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["+3.5", "—", "—", "0:15", "0:20", "1:25"],
                [BEHIND_LOSING, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                // In the original LiveSplit, we showed the split time for this column type if the comparison time was missing
                // Instead, we show a dash for the third segment rather than showing the split time
                ["+3.5", "—", "—", "0:15", "0:20", "1:25"],
                [BEHIND_LOSING, NONE, BEST, NONE, NONE, NONE],
            ),
            (
                ["+3.5", "—", "—", "+2.5", "0:20", "1:25"],
                [BEHIND_LOSING, NONE, BEST, BEHIND_GAINING, NONE, NONE],
            ),
            (
                ["+3.5", "—", "—", "+2.5", "—", "1:25"],
                [BEHIND_LOSING, NONE, BEST, BEHIND_GAINING, NONE, NONE],
            ),
            (
                ["+3.5", "—", "—", "+2.5", "—", "−1:00"],
                [
                    BEHIND_LOSING,
                    NONE,
                    BEST,
                    BEHIND_GAINING,
                    NONE,
                    AHEAD_GAINING,
                ],
            ),
        ],
    )
}

#[test]
fn column_comparison_segment_time_segment_delta() {
    check_columns(
        ColumnStartWith::ComparisonSegmentTime,
        ColumnUpdateWith::SegmentDelta,
        [
            (
                ["0:05", "—", "—", "0:10", "0:05", "1:05"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["+3.5", "—", "—", "0:10", "0:05", "1:05"],
                [BEHIND_LOSING, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["+3.5", "—", "—", "0:10", "0:05", "1:05"],
                [BEHIND_LOSING, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                // In the original LiveSplit, we showed the segment time for this column type if the comparison segment time was missing
                // Instead, we show a dash for the third segment rather than showing the segment time
                ["+3.5", "—", "—", "0:10", "0:05", "1:05"],
                [BEHIND_LOSING, NONE, BEST, NONE, NONE, NONE],
            ),
            (
                ["+3.5", "—", "—", "−1.0", "0:05", "1:05"],
                [BEHIND_LOSING, NONE, BEST, AHEAD_GAINING, NONE, NONE],
            ),
            (
                ["+3.5", "—", "—", "−1.0", "—", "1:05"],
                [BEHIND_LOSING, NONE, BEST, AHEAD_GAINING, NONE, NONE],
            ),
            (
                ["+3.5", "—", "—", "−1.0", "—", "−1:02"],
                [
                    BEHIND_LOSING,
                    NONE,
                    BEST,
                    AHEAD_GAINING,
                    NONE,
                    AHEAD_GAINING,
                ],
            ),
        ],
    )
}

#[test]
fn column_comparison_time_dont_update() {
    check_columns(
        ColumnStartWith::ComparisonTime,
        ColumnUpdateWith::DontUpdate,
        [
            (
                ["0:05", "—", "—", "0:15", "0:20", "1:25"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:05", "—", "—", "0:15", "0:20", "1:25"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:05", "—", "—", "0:15", "0:20", "1:25"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:05", "—", "—", "0:15", "0:20", "1:25"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:05", "—", "—", "0:15", "0:20", "1:25"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:05", "—", "—", "0:15", "0:20", "1:25"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:05", "—", "—", "0:15", "0:20", "1:25"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
        ],
    )
}

#[test]
fn column_comparison_segment_time_dont_update() {
    check_columns(
        ColumnStartWith::ComparisonSegmentTime,
        ColumnUpdateWith::DontUpdate,
        [
            (
                ["0:05", "—", "—", "0:10", "0:05", "1:05"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:05", "—", "—", "0:10", "0:05", "1:05"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:05", "—", "—", "0:10", "0:05", "1:05"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:05", "—", "—", "0:10", "0:05", "1:05"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:05", "—", "—", "0:10", "0:05", "1:05"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:05", "—", "—", "0:10", "0:05", "1:05"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
            (
                ["0:05", "—", "—", "0:10", "0:05", "1:05"],
                [NONE, NONE, NONE, NONE, NONE, NONE],
            ),
        ],
    )
}

fn check_columns(
    start_with: ColumnStartWith,
    update_with: ColumnUpdateWith,
    expected_values: Values,
) {
    let mut timer = timer();

    // Set initial best segment times
    run_with_splits_opt(
        &mut timer,
        &[
            Some(6.0),
            None,
            Some(100.0),
            Some(101.0),
            Some(102.0),
            Some(103.0),
        ],
    );

    // Set personal best times
    run_with_splits_opt(
        &mut timer,
        &[Some(5.0), None, None, Some(15.0), Some(20.0), Some(85.0)],
    );

    start_run(&mut timer);

    let layout_settings = Default::default();
    let mut component = Component::with_settings(Settings {
        columns: vec![ColumnSettings {
            start_with,
            update_with,
            ..Default::default()
        }],
        fill_with_blank_space: false,
        ..Default::default()
    });

    let state = component.state(&timer, &layout_settings);
    check_column_state(&state, 0, &expected_values);

    let first = TimeSpan::from_seconds(8.5);
    timer.set_game_time(first);
    timer.split();

    let state = component.state(&timer, &layout_settings);
    check_column_state(&state, 1, &expected_values);

    timer.skip_split();

    let state = component.state(&timer, &layout_settings);
    check_column_state(&state, 2, &expected_values);

    let third = TimeSpan::from_seconds(10.0);
    timer.set_game_time(third);
    timer.split();

    let state = component.state(&timer, &layout_settings);
    check_column_state(&state, 3, &expected_values);

    let fourth = TimeSpan::from_seconds(17.5);
    timer.set_game_time(fourth);
    timer.split();

    let state = component.state(&timer, &layout_settings);
    check_column_state(&state, 4, &expected_values);

    timer.skip_split();

    let state = component.state(&timer, &layout_settings);
    check_column_state(&state, 5, &expected_values);

    let sixth = TimeSpan::from_seconds(25.0);
    timer.set_game_time(sixth);
    timer.split();

    let state = component.state(&timer, &layout_settings);
    check_column_state(&state, 6, &expected_values);
}
