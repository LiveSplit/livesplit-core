use super::{Component, Settings};
use crate::{
    GeneralLayoutSettings, Lang, Run, Segment, Timer,
    settings::{Image, ImageCache},
};

fn prepare() -> (Timer, Component, GeneralLayoutSettings, ImageCache) {
    let mut run = Run::new();
    let mut segment = Segment::new("foo");
    segment.set_icon(Image::new([0x00, 0x12, 0x34].into(), Image::ICON));
    run.push_segment(segment);
    let timer = Timer::new(run).unwrap();

    let component = Component::with_settings(Settings {
        display_icon: true,
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
