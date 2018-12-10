use super::{
    ColumnSettings, ColumnStartWith, ColumnUpdateTrigger, ColumnUpdateWith, Component, Settings,
    State,
};
use crate::{Run, Segment, TimeSpan, Timer, TimingMethod};

pub mod column;

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
            update_trigger: ColumnUpdateTrigger::OnStartingSegment,
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
fn unique_split_indices() {
    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    run.push_segment(Segment::new(""));
    run.push_segment(Segment::new(""));
    let timer = Timer::new(run).unwrap();

    let mut component = Component::with_settings(Settings {
        visual_split_count: 20,
        fill_with_blank_space: true,
        ..Default::default()
    });

    let state = component.state(&timer, &Default::default());

    let mut indices = state
        .splits
        .into_iter()
        .map(|s| s.index)
        .collect::<Vec<_>>();

    indices.sort_unstable();

    assert!(indices.windows(2).all(|pair| pair[0] != pair[1]));
}
