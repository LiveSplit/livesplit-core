use super::{Component, Settings};
use crate::{Run, Segment, Timer};

#[test]
fn finished_runs_and_attempt_count() {
    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    let mut timer = Timer::new(run).unwrap();

    let mut component = Component::with_settings(Settings {
        show_finished_runs_count: true,
        show_attempt_count: true,
        ..Default::default()
    });

    assert_eq!(component.state(&timer).finished_runs, Some(0));
    assert_eq!(component.state(&timer).attempts, Some(0));

    timer.start();
    assert_eq!(component.state(&timer).finished_runs, Some(0));
    assert_eq!(component.state(&timer).attempts, Some(1));

    timer.split();
    assert_eq!(component.state(&timer).finished_runs, Some(1));
    assert_eq!(component.state(&timer).attempts, Some(1));

    timer.reset(true);
    assert_eq!(component.state(&timer).finished_runs, Some(1));
    assert_eq!(component.state(&timer).attempts, Some(1));
}
