use super::{Component, Settings};
use crate::{GeneralLayoutSettings, Run, Segment, Timer};

fn prepare() -> (Timer, Component, GeneralLayoutSettings) {
    let mut run = Run::new();
    let mut segment = Segment::new("foo");
    segment.set_icon([0x00, 0x12, 0x34]);
    run.push_segment(segment);
    let timer = Timer::new(run).unwrap();

    let component = Component::with_settings(Settings {
        display_icon: true,
        show_segment_name: true,
        ..Default::default()
    });

    (timer, component, GeneralLayoutSettings::default())
}

#[test]
fn doesnt_show_segment_name_outside_attempt() {
    let (timer, mut component, layout_settings) = prepare();

    assert_eq!(
        component
            .state(&timer.snapshot(), &layout_settings)
            .segment_name,
        None
    );
}

#[test]
fn shows_segment_name_during_attempt() {
    let (mut timer, mut component, layout_settings) = prepare();

    timer.start();

    assert_eq!(
        component
            .state(&timer.snapshot(), &layout_settings)
            .segment_name
            .unwrap(),
        "foo",
    );
}

#[test]
fn shows_segment_name_at_the_end_of_an_attempt() {
    let (mut timer, mut component, layout_settings) = prepare();

    timer.start();
    timer.split();

    assert_eq!(
        component
            .state(&timer.snapshot(), &layout_settings)
            .segment_name
            .unwrap(),
        "foo",
    );
}

#[test]
fn stops_showing_segment_name_when_resetting() {
    let (mut timer, mut component, layout_settings) = prepare();

    timer.start();
    timer.split();
    timer.reset(true);

    assert_eq!(
        component
            .state(&timer.snapshot(), &layout_settings)
            .segment_name,
        None
    );
}

#[test]
fn doesnt_show_icon_outside_attempt() {
    let (timer, mut component, layout_settings) = prepare();

    assert!(component
        .state(&timer.snapshot(), &layout_settings)
        .icon_change
        .filter(|i| i.is_empty())
        .is_some());

    assert!(component
        .state(&timer.snapshot(), &layout_settings)
        .icon_change
        .is_none());
}

#[test]
fn shows_icon_during_attempt() {
    let (mut timer, mut component, layout_settings) = prepare();

    component.state(&timer.snapshot(), &layout_settings);

    timer.start();

    assert!(component
        .state(&timer.snapshot(), &layout_settings)
        .icon_change
        .filter(|s| !s.is_empty())
        .is_some());

    assert!(component
        .state(&timer.snapshot(), &layout_settings)
        .icon_change
        .is_none());
}

#[test]
fn still_shows_icon_of_last_segment_at_the_end_of_an_attempt() {
    let (mut timer, mut component, layout_settings) = prepare();

    component.state(&timer.snapshot(), &layout_settings);

    timer.start();

    component.state(&timer.snapshot(), &layout_settings);

    timer.split();

    assert!(component
        .state(&timer.snapshot(), &layout_settings)
        .icon_change
        .is_none());
}

#[test]
fn stops_showing_icon_when_resetting() {
    let (mut timer, mut component, layout_settings) = prepare();

    component.state(&timer.snapshot(), &layout_settings);

    timer.start();

    component.state(&timer.snapshot(), &layout_settings);

    timer.split();

    component.state(&timer.snapshot(), &layout_settings);

    timer.reset(true);

    assert!(component
        .state(&timer.snapshot(), &layout_settings)
        .icon_change
        .filter(|i| i.is_empty())
        .is_some());

    assert!(component
        .state(&timer.snapshot(), &layout_settings)
        .icon_change
        .is_none());
}

#[test]
fn shows_icon_again_when_remounting() {
    let (mut timer, mut component, layout_settings) = prepare();

    component.state(&timer.snapshot(), &layout_settings);

    timer.start();

    component.state(&timer.snapshot(), &layout_settings);

    component.remount();

    assert!(component
        .state(&timer.snapshot(), &layout_settings)
        .icon_change
        .filter(|s| !s.is_empty())
        .is_some());

    component.remount();

    assert!(component
        .state(&timer.snapshot(), &layout_settings)
        .icon_change
        .filter(|s| !s.is_empty())
        .is_some());
}
