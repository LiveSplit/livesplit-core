use crate::comparison::average_segments::{AverageSegments, NAME};
use crate::tests_helper::run_with_splits;
use crate::{Run, Segment, TimeSpan, Timer};

#[test]
fn test() {
    let s = TimeSpan::from_seconds;

    let mut run = Run::new();

    run.push_segment(Segment::new("First"));

    run.comparison_generators_mut().clear();
    run.comparison_generators_mut()
        .push(Box::new(AverageSegments));

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

    let last_average = {
        let run = timer.run();
        let average = run.segment(0).comparison(NAME).game_time;
        assert!(average < Some(s(1.0)));
        assert!(average >= Some(s(0.5)));
        average
    };

    run_with_splits(&mut timer, &[1.0]);

    {
        let run = timer.run();
        let current_average = run.segment(0).comparison(NAME).game_time;
        assert!(current_average < Some(s(1.0)));
        assert!(current_average > last_average);
    }
}
