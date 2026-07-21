use crate::{
    Timer, TimingMethod,
    analysis::{
        comparison_segment_time_for_range, live_segment_delta_for_range,
        live_segment_time_for_range, previous_segment_delta_for_range,
        previous_segment_time_for_range,
    },
    comparison::{best_segments, personal_best},
    util::tests_helper::{create_run, create_timer, run_with_splits_opt, span, start_run},
};

#[test]
fn segment_ranges_use_the_split_before_the_range() {
    let mut run = create_run(&["A", "B", "C", "D"]);
    for (index, (split_time, comparison_time, best_segment)) in [
        (12.0, 10.0, 10.0),
        (30.0, 25.0, 13.0),
        (48.0, 40.0, 14.0),
        (80.0, 70.0, 20.0),
    ]
    .into_iter()
    .enumerate()
    {
        let segment = run.segment_mut(index);
        segment.split_time_mut()[TimingMethod::GameTime] = Some(span(split_time));
        segment.personal_best_split_time_mut()[TimingMethod::GameTime] =
            Some(span(comparison_time));
        segment.comparison_mut(personal_best::NAME)[TimingMethod::GameTime] =
            Some(span(comparison_time));
        segment.best_segment_time_mut()[TimingMethod::GameTime] = Some(span(best_segment));
    }

    let mut timer = Timer::new(run).unwrap();

    assert_eq!(
        previous_segment_time_for_range(&timer, 1, 2, TimingMethod::GameTime),
        Some(span(36.0)),
    );
    assert_eq!(
        previous_segment_delta_for_range(&timer, 1, 2, personal_best::NAME, TimingMethod::GameTime,),
        Some(span(6.0)),
    );
    assert_eq!(
        comparison_segment_time_for_range(
            timer.run(),
            1,
            2,
            personal_best::NAME,
            TimingMethod::GameTime,
        ),
        Some(span(30.0)),
    );
    assert_eq!(
        comparison_segment_time_for_range(
            timer.run(),
            1,
            2,
            best_segments::NAME,
            TimingMethod::GameTime,
        ),
        Some(span(27.0)),
    );

    start_run(&mut timer);
    timer.set_game_time(span(12.0)).unwrap();
    timer.split().unwrap();
    timer.set_game_time(span(30.0)).unwrap();
    timer.split().unwrap();
    timer.set_game_time(span(50.0)).unwrap();

    let snapshot = timer.snapshot();
    assert_eq!(
        live_segment_time_for_range(&snapshot, 1, TimingMethod::GameTime),
        Some(span(38.0)),
    );
    assert_eq!(
        live_segment_delta_for_range(&snapshot, 1, 2, personal_best::NAME, TimingMethod::GameTime,),
        Some(span(8.0)),
    );
}

#[test]
fn best_segments_range_uses_the_fastest_path() {
    let mut timer = create_timer(&["A", "B", "C"]);

    run_with_splits_opt(&mut timer, &[Some(10.0), Some(30.0), Some(50.0)]);
    run_with_splits_opt(&mut timer, &[Some(10.0), None, Some(25.0)]);

    assert_eq!(
        comparison_segment_time_for_range(
            timer.run(),
            1,
            2,
            best_segments::NAME,
            TimingMethod::GameTime,
        ),
        Some(span(15.0)),
    );
}
