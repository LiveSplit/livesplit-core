use super::{Component, Settings};
use crate::{Run, Segment, Timer};

#[test]
fn icon_handling() {
    let mut run = Run::new();
    let mut segment = Segment::new("foo");
    segment.set_icon(&[0x00, 0x12, 0x34]);
    run.push_segment(segment);
    let mut timer = Timer::new(run).unwrap();

    let mut component = Component::with_settings(Settings {
        display_icon: true,
        ..Default::default()
    });

    let layout_settings = &Default::default();

    assert!(component
        .state(&timer, layout_settings)
        .icon_change
        .filter(|i| i.is_empty())
        .is_some());

    assert!(component
        .state(&timer, layout_settings)
        .icon_change
        .is_none());

    timer.start();

    assert!(component
        .state(&timer, layout_settings)
        .icon_change
        .filter(|s| !s.is_empty())
        .is_some());

    assert!(component
        .state(&timer, layout_settings)
        .icon_change
        .is_none());

    timer.reset(true);

    assert!(component
        .state(&timer, layout_settings)
        .icon_change
        .filter(|i| i.is_empty())
        .is_some());

    assert!(component
        .state(&timer, layout_settings)
        .icon_change
        .is_none());

    component.remount();

    assert!(component
        .state(&timer, layout_settings)
        .icon_change
        .filter(|i| i.is_empty())
        .is_some());
}
