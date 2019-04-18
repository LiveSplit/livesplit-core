use super::{
    ColumnSettings, ColumnStartWith, ColumnUpdateTrigger, ColumnUpdateWith, Component, Settings,
    State,
};
use crate::settings::SemanticColor::{
    self, AheadGainingTime as AheadGaining, BehindLosingTime as BehindLosing, BestSegment as Best,
    Default as Text,
};
use crate::tests_helper::{make_progress_run_with_splits_opt, run_with_splits_opt, start_run};
use crate::{Run, Segment, TimeSpan, Timer};

type Values = &'static [([&'static str; 6], [SemanticColor; 6])];

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

fn check_column_state(state: &State, state_index: usize, expected_values: Values) {
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
        &[
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
                // The fourth segment is a best segment because the current
                // run's combined segment time from the first to the fourth
                // segment is 9 seconds while the best segments comparison has a
                // combined segment time of 10 seconds. The third segment is
                // empty in the best segments comparison because it is not a
                // part of the shortest path in the Sum of Best calculation.
                ["+3.5", "—", "—", "+2.5", "", ""],
                [BehindLosing, Text, Best, Best, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "+2.5", "—", ""],
                [BehindLosing, Text, Best, Best, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "+2.5", "—", "−1:00"],
                [BehindLosing, Text, Best, Best, Text, AheadGaining],
            ),
        ],
    );
}

#[test]
fn column_empty_segment_delta() {
    check_columns(
        ColumnStartWith::Empty,
        ColumnUpdateWith::SegmentDelta,
        &[
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
                // The fourth segment is a best segment because the current
                // run's combined segment time from the first to the fourth
                // segment is 9 seconds while the best segments comparison has a
                // combined segment time of 10 seconds. The third segment is
                // empty in the best segments comparison because it is not a
                // part of the shortest path in the Sum of Best calculation.
                ["+3.5", "—", "—", "−1.0", "", ""],
                [BehindLosing, Text, Best, Best, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "−1.0", "—", ""],
                [BehindLosing, Text, Best, Best, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "−1.0", "—", "−1:02"],
                [BehindLosing, Text, Best, Best, Text, AheadGaining],
            ),
        ],
    )
}

#[test]
fn column_empty_split_time() {
    check_columns(
        ColumnStartWith::Empty,
        ColumnUpdateWith::SplitTime,
        &[
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
        &[
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
                ["0:08", "—", "0:01", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:01", "0:07", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:01", "0:07", "—", ""],
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
        &[
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
        &[
            (
                ["0:05", "—", "—", "0:10", "0:05", "1:05"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "—", "0:10", "0:05", "1:05"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "—", "0:10", "0:05", "1:05"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:01", "0:10", "0:05", "1:05"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:01", "0:07", "0:05", "1:05"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:08", "—", "0:01", "0:07", "—", "1:05"],
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
        &[
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
                // In the original LiveSplit, we showed the split time for this
                // column type if the comparison time was missing. Instead, we
                // show a dash for the third segment rather than showing the
                // split time.
                ["+3.5", "—", "—", "0:15", "0:20", "1:25"],
                [BehindLosing, Text, Best, Text, Text, Text],
            ),
            (
                // The fourth segment is a best segment because the current
                // run's combined segment time from the first to the fourth
                // segment is 9 seconds while the best segments comparison has a
                // combined segment time of 10 seconds. The third segment is
                // empty in the best segments comparison because it is not a
                // part of the shortest path in the Sum of Best calculation.
                ["+3.5", "—", "—", "+2.5", "0:20", "1:25"],
                [BehindLosing, Text, Best, Best, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "+2.5", "—", "1:25"],
                [BehindLosing, Text, Best, Best, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "+2.5", "—", "−1:00"],
                [BehindLosing, Text, Best, Best, Text, AheadGaining],
            ),
        ],
    )
}

#[test]
fn column_comparison_segment_time_segment_delta() {
    check_columns(
        ColumnStartWith::ComparisonSegmentTime,
        ColumnUpdateWith::SegmentDelta,
        &[
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
                // In the original LiveSplit, we showed the segment time for
                // this column type if the comparison segment time was missing.
                // Instead, we show a dash for the third segment rather than
                // showing the segment time.
                ["+3.5", "—", "—", "0:10", "0:05", "1:05"],
                [BehindLosing, Text, Best, Text, Text, Text],
            ),
            (
                // The fourth segment is a best segment because the current
                // run's combined segment time from the first to the fourth
                // segment is 9 seconds while the best segments comparison has a
                // combined segment time of 10 seconds. The third segment is
                // empty in the best segments comparison because it is not a
                // part of the shortest path in the Sum of Best calculation.
                ["+3.5", "—", "—", "−1.0", "0:05", "1:05"],
                [BehindLosing, Text, Best, Best, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "−1.0", "—", "1:05"],
                [BehindLosing, Text, Best, Best, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "−1.0", "—", "−1:02"],
                [BehindLosing, Text, Best, Best, Text, AheadGaining],
            ),
        ],
    )
}

#[test]
fn column_comparison_time_dont_update() {
    check_columns(
        ColumnStartWith::ComparisonTime,
        ColumnUpdateWith::DontUpdate,
        &[(
            ["0:05", "—", "—", "0:15", "0:20", "1:25"],
            [Text, Text, Text, Text, Text, Text],
        ); 7],
    )
}

#[test]
fn column_possible_time_save_dont_update() {
    check_columns(
        ColumnStartWith::PossibleTimeSave,
        ColumnUpdateWith::DontUpdate,
        &[(
            ["0.00", "—", "—", "0.00", "4.00", "1:04.00"],
            [Text, Text, Text, Text, Text, Text],
        ); 7],
    )
}

#[test]
fn column_possible_time_save_segment_delta() {
    check_columns(
        ColumnStartWith::PossibleTimeSave,
        ColumnUpdateWith::SegmentDelta,
        &[
            (
                ["0.00", "—", "—", "0.00", "4.00", "1:04.00"],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "0.00", "4.00", "1:04.00"],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "0.00", "4.00", "1:04.00"],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "0.00", "4.00", "1:04.00"],
                [BehindLosing, Text, Best, Text, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "−1.0", "4.00", "1:04.00"],
                [BehindLosing, Text, Best, Best, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "−1.0", "—", "1:04.00"],
                [BehindLosing, Text, Best, Best, Text, Text],
            ),
            (
                ["+3.5", "—", "—", "−1.0", "—", "−1:02"],
                [BehindLosing, Text, Best, Best, Text, AheadGaining],
            ),
        ],
    )
}

#[test]
fn column_comparison_segment_time_dont_update() {
    check_columns(
        ColumnStartWith::ComparisonSegmentTime,
        ColumnUpdateWith::DontUpdate,
        &[(
            ["0:05", "—", "—", "0:10", "0:05", "1:05"],
            [Text, Text, Text, Text, Text, Text],
        ); 7],
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

    timer.set_game_time(TimeSpan::from_seconds(8.5));
    timer.split();

    let state = component.state(&timer, &layout_settings);
    check_column_state(&state, 1, &expected_values);

    timer.skip_split();

    let state = component.state(&timer, &layout_settings);
    check_column_state(&state, 2, &expected_values);

    timer.set_game_time(TimeSpan::from_seconds(10.0));
    timer.split();

    let state = component.state(&timer, &layout_settings);
    check_column_state(&state, 3, &expected_values);

    timer.set_game_time(TimeSpan::from_seconds(17.5));
    timer.split();

    let state = component.state(&timer, &layout_settings);
    check_column_state(&state, 4, &expected_values);

    timer.skip_split();

    let state = component.state(&timer, &layout_settings);
    check_column_state(&state, 5, &expected_values);

    timer.set_game_time(TimeSpan::from_seconds(25.0));
    timer.split();

    let state = component.state(&timer, &layout_settings);
    check_column_state(&state, 6, &expected_values);
}

#[test]
fn column_delta_update_on_ending_segment() {
    check_columns_update_trigger(
        ColumnUpdateWith::Delta,
        ColumnUpdateTrigger::OnEndingSegment,
        &[
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "", "", "", ""],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "", "", "", ""],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "+3.0", "—", "", ""],
                [BehindLosing, Text, BehindLosing, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "+3.0", "—", "", ""],
                [BehindLosing, Text, BehindLosing, Text, Text, Text],
            ),
        ],
    )
}

#[test]
fn column_segment_delta_update_on_ending_segment() {
    check_columns_update_trigger(
        ColumnUpdateWith::SegmentDelta,
        ColumnUpdateTrigger::OnEndingSegment,
        &[
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "", "", "", ""],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "", "", "", ""],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "+2.5", "—", "", ""],
                [BehindLosing, Text, BehindLosing, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "+2.5", "—", "", ""],
                [BehindLosing, Text, BehindLosing, Text, Text, Text],
            ),
        ],
    )
}

#[test]
fn column_split_time_update_on_ending_segment() {
    check_columns_update_trigger(
        ColumnUpdateWith::SplitTime,
        ColumnUpdateTrigger::OnEndingSegment,
        &[
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "0:18", "—", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "0:18", "—", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
        ],
    )
}

#[test]
fn column_segment_time_update_on_ending_segment() {
    check_columns_update_trigger(
        ColumnUpdateWith::SegmentTime,
        ColumnUpdateTrigger::OnEndingSegment,
        &[
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "0:12", "—", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "0:12", "—", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
        ],
    )
}

#[test]
fn column_delta_update_contextual() {
    check_columns_update_trigger(
        ColumnUpdateWith::Delta,
        ColumnUpdateTrigger::Contextual,
        &[
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["−1.5", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "", "", "", ""],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "−2.0", "", "", ""],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "+3.0", "—", "", ""],
                [BehindLosing, Text, BehindLosing, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "+3.0", "—", "+1.0", ""],
                [BehindLosing, Text, BehindLosing, Text, Text, Text],
            ),
        ],
    )
}

#[test]
fn column_segment_delta_update_contextual() {
    check_columns_update_trigger(
        ColumnUpdateWith::SegmentDelta,
        ColumnUpdateTrigger::Contextual,
        &[
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["−1.5", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "", "", "", ""],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "−2.5", "", "", ""],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "+2.5", "—", "", ""],
                [BehindLosing, Text, BehindLosing, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "+2.5", "—", "", ""],
                [BehindLosing, Text, BehindLosing, Text, Text, Text],
            ),
        ],
    )
}

#[test]
fn column_split_time_update_contextual() {
    check_columns_update_trigger(
        ColumnUpdateWith::SplitTime,
        ColumnUpdateTrigger::Contextual,
        &[
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:03", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "0:13", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "0:18", "—", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "0:18", "—", "0:31", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
        ],
    )
}

#[test]
fn column_segment_time_update_contextual() {
    check_columns_update_trigger(
        ColumnUpdateWith::SegmentTime,
        ColumnUpdateTrigger::Contextual,
        &[
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:03", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "0:07", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "0:12", "—", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "0:12", "—", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
        ],
    )
}

#[test]
fn column_delta_update_on_starting_segment() {
    check_columns_update_trigger(
        ColumnUpdateWith::Delta,
        ColumnUpdateTrigger::OnStartingSegment,
        &[
            (
                ["−5.0", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["−3.0", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["−1.5", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "−4.0", "", "", ""],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "−2.0", "", "", ""],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "+3.0", "—", "−1.0", ""],
                [BehindLosing, Text, BehindLosing, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "+3.0", "—", "+1.0", ""],
                [BehindLosing, Text, BehindLosing, Text, Text, Text],
            ),
        ],
    )
}

#[test]
fn column_segment_delta_update_on_starting_segment() {
    check_columns_update_trigger(
        ColumnUpdateWith::SegmentDelta,
        ColumnUpdateTrigger::OnStartingSegment,
        &[
            (
                ["−5.0", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["−3.0", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["−1.5", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "−4.5", "", "", ""],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "−2.5", "", "", ""],
                [BehindLosing, Text, Text, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "+2.5", "—", "−4.0", ""],
                [BehindLosing, Text, BehindLosing, Text, Text, Text],
            ),
            (
                ["+0.5", "—", "+2.5", "—", "−2.0", ""],
                [BehindLosing, Text, BehindLosing, Text, Text, Text],
            ),
        ],
    )
}

#[test]
fn column_split_time_update_on_starting_segment() {
    check_columns_update_trigger(
        ColumnUpdateWith::SplitTime,
        ColumnUpdateTrigger::OnStartingSegment,
        &[
            (
                ["0:00", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:02", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:03", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "0:11", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "0:13", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "0:18", "—", "0:29", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "0:18", "—", "0:31", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
        ],
    )
}

#[test]
fn column_segment_time_update_on_starting_segment() {
    check_columns_update_trigger(
        ColumnUpdateWith::SegmentTime,
        ColumnUpdateTrigger::OnStartingSegment,
        &[
            (
                ["0:00", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:02", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:03", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "", "", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "0:05", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "0:07", "", "", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "0:12", "—", "0:11", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
            (
                ["0:05", "—", "0:12", "—", "0:13", ""],
                [Text, Text, Text, Text, Text, Text],
            ),
        ],
    )
}

fn check_columns_update_trigger(
    update_with: ColumnUpdateWith,
    update_trigger: ColumnUpdateTrigger,
    expected_values: Values,
) {
    let mut timer = timer();

    // Set best segment times
    run_with_splits_opt(
        &mut timer,
        &[
            Some(3.0),
            Some(6.0),
            Some(10.0),
            None,
            Some(35.0),
            Some(40.0),
        ],
    );

    // Set personal best times
    run_with_splits_opt(
        &mut timer,
        &[
            Some(5.0),
            Some(10.0),
            Some(15.0),
            None,
            Some(30.0),
            Some(35.0),
        ],
    );

    start_run(&mut timer);

    let layout_settings = Default::default();
    let mut component = Component::with_settings(Settings {
        columns: vec![ColumnSettings {
            start_with: ColumnStartWith::Empty,
            update_with,
            update_trigger,
            ..Default::default()
        }],
        fill_with_blank_space: false,
        ..Default::default()
    });

    let state = component.state(&timer, &layout_settings);
    // Timer at 0, contextual shouldn't show live times yet
    check_column_state(&state, 0, &expected_values);

    timer.set_game_time(TimeSpan::from_seconds(2.0));

    let state = component.state(&timer, &layout_settings);
    // Shorter than the best segment, contextual shouldn't show live times yet
    check_column_state(&state, 1, &expected_values);

    timer.set_game_time(TimeSpan::from_seconds(3.5));

    let state = component.state(&timer, &layout_settings);
    // Longer than the best segment, contextual should show live times
    check_column_state(&state, 2, &expected_values);

    timer.set_game_time(TimeSpan::from_seconds(5.5));

    let state = component.state(&timer, &layout_settings);
    // Behind the personal best, contextual should show live times
    check_column_state(&state, 3, &expected_values);

    timer.split();
    timer.skip_split();
    timer.set_game_time(TimeSpan::from_seconds(11.0));

    let state = component.state(&timer, &layout_settings);
    // Shorter than the combined best segment, contextual shouldn't show live
    // times yet
    check_column_state(&state, 4, &expected_values);

    timer.set_game_time(TimeSpan::from_seconds(13.0));

    let state = component.state(&timer, &layout_settings);
    // Longer than the combined best segment, contextual should show live times
    check_column_state(&state, 5, &expected_values);

    timer.set_game_time(TimeSpan::from_seconds(18.0));
    timer.split();
    timer.skip_split();
    timer.set_game_time(TimeSpan::from_seconds(29.0));

    let state = component.state(&timer, &layout_settings);
    // Not behind the personal best, contextual should not show live times yet
    check_column_state(&state, 6, &expected_values);

    timer.set_game_time(TimeSpan::from_seconds(31.0));

    let state = component.state(&timer, &layout_settings);
    // Behind the personal best, contextual should show live times only if a
    // split time or delta column
    check_column_state(&state, 7, &expected_values);
}

#[test]
fn column_delta_best_segment_colors() {
    let mut timer = timer();

    // Set best segment times, but no PB time
    run_with_splits_opt(
        &mut timer,
        &[Some(5.0), Some(8.0), Some(12.0), None, Some(20.0)],
    );

    start_run(&mut timer);

    let layout_settings = Default::default();
    let mut component = Component::with_settings(Settings {
        columns: vec![ColumnSettings {
            start_with: ColumnStartWith::Empty,
            update_with: ColumnUpdateWith::Delta,
            ..Default::default()
        }],
        fill_with_blank_space: false,
        ..Default::default()
    });

    let state = component.state(&timer, &layout_settings);
    check_column_color(&state, 0, Text);

    timer.set_game_time(TimeSpan::from_seconds(5.1));
    timer.split();

    let state = component.state(&timer, &layout_settings);
    // 5.1 is longer than the best segment of 5.0, so this isn't a best segment
    check_column_color(&state, 0, Text);

    timer.undo_split();
    timer.set_game_time(TimeSpan::from_seconds(4.9));
    timer.split();

    let state = component.state(&timer, &layout_settings);
    // 4.9 is shorter than the best segment of 5.0, so this is a best segment
    check_column_color(&state, 0, Best);

    timer.skip_split();

    let state = component.state(&timer, &layout_settings);
    // After skipping a split, the first best segment should stay
    check_column_color(&state, 0, Best);
    // The skipped split is not a best segment
    check_column_color(&state, 1, Text);

    timer.set_game_time(TimeSpan::from_seconds(12.0));
    timer.split();

    let state = component.state(&timer, &layout_settings);
    // The first best segment should stay
    check_column_color(&state, 0, Best);
    // The second split is still skipped, so still not a best segment
    check_column_color(&state, 1, Text);
    // The combined segment of 7.1 is longer than the combined best segments of
    // 7.0, so this is not a best segment
    check_column_color(&state, 2, Text);

    timer.undo_split();
    timer.set_game_time(TimeSpan::from_seconds(11.8));
    timer.split();

    let state = component.state(&timer, &layout_settings);
    // The first best segment should stay
    check_column_color(&state, 0, Best);
    // The second split is still skipped, so still not a best segment
    check_column_color(&state, 1, Text);
    // The combined segment of 6.9 is shorter than the combined best segments of
    // 7.0, so this is a best segment
    check_column_color(&state, 2, Best);

    timer.set_game_time(TimeSpan::from_seconds(21.0));
    timer.split();

    let state = component.state(&timer, &layout_settings);
    // The best segment is empty, so the segment of 9.2 is a best segment
    check_column_color(&state, 3, Best);

    timer.set_game_time(TimeSpan::from_seconds(28.9));
    timer.split();

    let state = component.state(&timer, &layout_settings);
    // The segment of 7.9 is shorter than the best segment of 8.0, so this is a best segment
    check_column_color(&state, 4, Best);

    timer.undo_split();
    timer.set_game_time(TimeSpan::from_seconds(29.1));
    timer.split();

    let state = component.state(&timer, &layout_settings);
    // The segment of 8.1 is longer than the best segment of 8.0, so this is not a best segment
    check_column_color(&state, 4, Text);
}

#[test]
fn delta_or_split_time() {
    // Tests whether the `Delta or Split Time` Column Type behaves the same way
    // in livesplit-core.

    let mut timer = timer();

    let layout_settings = Default::default();
    let mut component = Component::with_settings(Settings {
        columns: vec![ColumnSettings {
            start_with: ColumnStartWith::ComparisonTime,
            update_with: ColumnUpdateWith::DeltaWithFallback,
            ..Default::default()
        }],
        fill_with_blank_space: false,
        ..Default::default()
    });

    // We prepare splits with every split having a time and the second split
    // a low best segment.
    start_run(&mut timer);
    make_progress_run_with_splits_opt(
        &mut timer,
        &[
            Some(5.0),
            Some(6.0),
            Some(15.0),
            Some(20.0),
            Some(25.0),
            Some(30.0),
        ],
    );
    check_column_state(
        &component.state(&timer, &layout_settings),
        0,
        &[(
            ["0:05", "0:06", "0:15", "0:20", "0:25", "0:30"],
            [Best, Best, Best, Best, Best, Best],
        )],
    );
    timer.reset(true);

    // We do another run, but this time with the second and second to last split
    // being skipped.
    start_run(&mut timer);
    make_progress_run_with_splits_opt(
        &mut timer,
        &[Some(4.0), None, Some(8.0), Some(12.0), None, Some(20.0)],
    );
    check_column_state(
        &component.state(&timer, &layout_settings),
        0,
        &[(
            ["−1.0", "—", "−7.0", "−8.0", "—", "−10.0"],
            [Best, Text, Best, Best, Text, Best],
        )],
    );
    timer.reset(true);

    // In this third run, we should have split times instead of deltas for the
    // two skipped splits we had before. The way we set them up, the second to
    // last one is a best segment and the second split isn't.
    start_run(&mut timer);
    make_progress_run_with_splits_opt(
        &mut timer,
        &[
            Some(2.0),
            Some(4.0),
            Some(6.0),
            Some(8.0),
            Some(12.0),
            Some(14.0),
        ],
    );
    check_column_state(
        &component.state(&timer, &layout_settings),
        0,
        &[(
            ["−2.0", "0:04", "−2.0", "−4.0", "0:12", "−6.0"],
            [Best, Text, Best, Best, Best, Best],
        )],
    );
    timer.reset(true);
}

fn check_column_color(state: &State, split_index: usize, expected_color: SemanticColor) {
    assert_eq!(
        state.splits[split_index].columns[0].semantic_color,
        expected_color
    );
}
