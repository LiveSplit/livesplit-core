use super::{Component, Settings};
use crate::{Lang, Run, Segment, Timer, settings::ImageCache};

#[test]
fn finished_runs_and_attempt_count() {
    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    let mut timer = Timer::new(run).unwrap();

    let component = Component::with_settings(Settings {
        show_finished_runs_count: true,
        show_attempt_count: true,
        ..Default::default()
    });

    let mut image_cache = ImageCache::new();

    assert_eq!(
        component
            .state(&mut image_cache, &timer, Lang::English)
            .finished_runs,
        Some(0)
    );
    assert_eq!(
        component
            .state(&mut image_cache, &timer, Lang::English)
            .attempts,
        Some(0)
    );

    timer.start().unwrap();
    assert_eq!(
        component
            .state(&mut image_cache, &timer, Lang::English)
            .finished_runs,
        Some(0)
    );
    assert_eq!(
        component
            .state(&mut image_cache, &timer, Lang::English)
            .attempts,
        Some(1)
    );

    timer.split().unwrap();
    assert_eq!(
        component
            .state(&mut image_cache, &timer, Lang::English)
            .finished_runs,
        Some(1)
    );
    assert_eq!(
        component
            .state(&mut image_cache, &timer, Lang::English)
            .attempts,
        Some(1)
    );

    timer.reset(true).unwrap();
    assert_eq!(
        component
            .state(&mut image_cache, &timer, Lang::English)
            .finished_runs,
        Some(1)
    );
    assert_eq!(
        component
            .state(&mut image_cache, &timer, Lang::English)
            .attempts,
        Some(1)
    );
}
