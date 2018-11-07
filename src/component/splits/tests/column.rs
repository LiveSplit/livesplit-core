use super::{ColumnSettings, ColumnStartWith, ColumnUpdateWith, Component, Settings, State};
use tests_helper::{run_with_splits_opt, start_run};
use {Run, Segment, TimeSpan, Timer};
use settings::SemanticColor::{
    self, AheadGainingTime as AheadGaining, BehindGainingTime as BehindGaining,
    BehindLosingTime as BehindLosing, BestSegment as Best, Default as Text,
};

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
fn column_empty_delta() {
    check_columns(
        ColumnStartWith::Empty,
        ColumnUpdateWith::Delta,
        [
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["+3.5", "", "", "", "", ""],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+3.5", "—", "", "", "", ""],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "", "", ""],
                [BehindLosing, Text, Best, Text, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "+2.5", "", ""],
                [BehindLosing, Text, Best, BehindGaining, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "+2.5", "—", ""],
                [BehindLosing, Text, Best, BehindGaining, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "+2.5", "—", "−1:00"],
                [
                    BehindLosing,
                    Text,
                    Best,
                    BehindGaining,
                    Text,
                    AheadGaining,
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
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["+3.5", "", "", "", "", ""],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+3.5", "—", "", "", "", ""],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "", "", ""],
                [BehindLosing, Text, Best, Text, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "−1.0", "", ""],
                [BehindLosing, Text, Best, AheadGaining, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "−1.0", "—", ""],
                [BehindLosing, Text, Best, AheadGaining, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "−1.0", "—", "−1:02"],
                [
                    BehindLosing,
                    Text,
                    Best,
                    AheadGaining,
                    Text,
                    AheadGaining,
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
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:10", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:10", "0:17", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:10", "0:17", "—", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:10", "0:17", "—", "0:25"],
                [Text, Text, Text, Text, Text, Text],
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
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "0:00", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:00", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:01", "0:00", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:01", "0:07", "0:00", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:01", "0:07", "—", "0:00"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:01", "0:07", "—", "0:07"],
                [Text, Text, Text, Text, Text, Text],
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
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "—", "0:15", "0:20", "1:25"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "—", "0:15", "0:20", "1:25"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:10", "0:15", "0:20", "1:25"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:10", "0:17", "0:20", "1:25"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:10", "0:17", "—", "1:25"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:10", "0:17", "—", "0:25"],
                [Text, Text, Text, Text, Text, Text],
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
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "0:00", "—", "0:10", "0:05", "1:05"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:00", "0:10", "0:05", "1:05"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:01", "0:00", "0:05", "1:05"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:01", "0:07", "0:00", "1:05"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:01", "0:07", "—", "0:00"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:01", "0:07", "—", "0:07"],
                [Text, Text, Text, Text, Text, Text],
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
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "0:15", "0:20", "1:25"],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "0:15", "0:20", "1:25"],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                // In the original LiveSplit, we showed the split time for this column type if the comparison time was missing
                // Instead, we show a dash for the third segment rather than showing the split time
                ["+3.5", "—", "—", "0:15", "0:20", "1:25"],
                [BehindLosing, Text, Best, Text, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "+2.5", "0:20", "1:25"],
                [BehindLosing, Text, Best, BehindGaining, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "+2.5", "—", "1:25"],
                [BehindLosing, Text, Best, BehindGaining, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "+2.5", "—", "−1:00"],
                [
                    BehindLosing,
                    Text,
                    Best,
                    BehindGaining,
                    Text,
                    AheadGaining,
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
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "0:10", "0:05", "1:05"],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "0:10", "0:05", "1:05"],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                // In the original LiveSplit, we showed the segment time for this column type if the comparison segment time was missing
                // Instead, we show a dash for the third segment rather than showing the segment time
                ["+3.5", "—", "—", "0:10", "0:05", "1:05"],
                [BehindLosing, Text, Best, Text, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "−1.0", "0:05", "1:05"],
                [BehindLosing, Text, Best, AheadGaining, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "−1.0", "—", "1:05"],
                [BehindLosing, Text, Best, AheadGaining, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "−1.0", "—", "−1:02"],
                [
                    BehindLosing,
                    Text,
                    Best,
                    AheadGaining,
                    Text,
                    AheadGaining,
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
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "—", "0:15", "0:20", "1:25"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "—", "0:15", "0:20", "1:25"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "—", "0:15", "0:20", "1:25"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "—", "0:15", "0:20", "1:25"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "—", "0:15", "0:20", "1:25"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "—", "0:15", "0:20", "1:25"],
                [Text, Text, Text, Text, Text, Text],
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
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "—", "0:10", "0:05", "1:05"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "—", "0:10", "0:05", "1:05"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "—", "0:10", "0:05", "1:05"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "—", "0:10", "0:05", "1:05"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "—", "0:10", "0:05", "1:05"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "—", "0:10", "0:05", "1:05"],
                [Text, Text, Text, Text, Text, Text],
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
