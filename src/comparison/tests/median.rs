use crate::comparison::median_segments::{MedianSegments, NAME};
use crate::tests_helper::run_with_splits;
use crate::{Run, Segment, TimeSpan, Timer};

#[test]
fn test() {
    let s = TimeSpan::from_seconds;

    let mut run = Run::new();

    run.push_segment(Segment::new("First"));

    run.comparison_generators_mut().clear();
    run.comparison_generators_mut()
        .push(Box::new(MedianSegments));

    let mut timer = Timer::new(run).unwrap();

    {
        let run = timer.run();
        assert_eq!(run.segment(0).comparison(NAME).game_time, None);
    }

    run_with_splits(&mut timer, &[0.0]);

    {
        let run = timer.run();
        assert_eq!(run.segment(0).comparison(NAME).game_time, Some(s(0.0)));
    }

    run_with_splits(&mut timer, &[1.0]);

    {
        let run = timer.run();
        assert_eq!(run.segment(0).comparison(NAME).game_time, Some(s(1.0)));
    }

    run_with_splits(&mut timer, &[0.6]);

    {
        let run = timer.run();
        assert!(run.segment(0).comparison(NAME).game_time < Some(s(0.61)));
        assert!(run.segment(0).comparison(NAME).game_time > Some(s(0.59)));
    }

    run_with_splits(&mut timer, &[0.3]);

    {
        let run = timer.run();
        assert!(run.segment(0).comparison(NAME).game_time < Some(s(0.31)));
        assert!(run.segment(0).comparison(NAME).game_time > Some(s(0.29)));
    }

    run_with_splits(&mut timer, &[1.0]);

    {
        let run = timer.run();
        assert!(run.segment(0).comparison(NAME).game_time < Some(s(0.61)));
        assert!(run.segment(0).comparison(NAME).game_time > Some(s(0.59)));
    }
}
