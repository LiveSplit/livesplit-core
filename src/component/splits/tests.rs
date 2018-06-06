use super::{Component, Settings};
use {Run, Segment, Timer};

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
