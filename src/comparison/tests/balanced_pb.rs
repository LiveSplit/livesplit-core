use crate::comparison::balanced_pb::{BalancedPB, NAME};
use crate::tests_helper::{run_with_splits, run_with_splits_opt};
use crate::timing::formatter::{Short, TimeFormatter};
use crate::{Run, Segment, TimeSpan, Timer};

#[test]
fn test() {
    let s = TimeSpan::from_seconds;

    let mut run = Run::new();

    run.push_segment(Segment::new("First"));
    run.push_segment(Segment::new("Second"));
    run.push_segment(Segment::new("Third"));

    run.comparison_generators_mut().clear();
    run.comparison_generators_mut().push(Box::new(BalancedPB));

    let mut timer = Timer::new(run).unwrap();

    run_with_splits(&mut timer, &[1.0, 2.0, 3.0]);

    {
        let run = timer.run();
        assert_eq!(run.segment(0).comparison(NAME).game_time, Some(s(1.0)));
        assert_eq!(run.segment(1).comparison(NAME).game_time, Some(s(2.0)));
        assert_eq!(run.segment(2).comparison(NAME).game_time, Some(s(3.0)));
    }

    run_with_splits(&mut timer, &[0.5, 2.5, 3.0]);

    {
        let run = timer.run();
        assert_eq!(run.segment(0).comparison(NAME).game_time, Some(s(0.75)));
        assert_eq!(run.segment(1).comparison(NAME).game_time, Some(s(2.25)));
        assert_eq!(run.segment(2).comparison(NAME).game_time, Some(s(3.0)));
    }

    run_with_splits(&mut timer, &[0.2, 2.8, 3.0]);

    {
        let run = timer.run();
        assert_eq!(
            Short::new()
                .format(run.segment(0).comparison(NAME).game_time)
                .to_string(),
            "0.46"
        );
        assert_eq!(
            Short::new()
                .format(run.segment(1).comparison(NAME).game_time)
                .to_string(),
            "2.53"
        );
        assert_eq!(run.segment(2).comparison(NAME).game_time, Some(s(3.0)));
    }
}

#[test]
fn index_bug() {
    let mut run = Run::new();

    run.push_segment(Segment::new("First"));
    run.push_segment(Segment::new("Second"));
    run.push_segment(Segment::new("Third"));

    run.comparison_generators_mut().clear();
    run.comparison_generators_mut().push(Box::new(BalancedPB));

    let mut timer = Timer::new(run).unwrap();

    run_with_splits(&mut timer, &[1.0, 2.0]);
    run_with_splits_opt(&mut timer, &[None, None, Some(3.0)]);
}
