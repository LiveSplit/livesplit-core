use super::{best, Prediction};
use crate::{
    comparison::best_segments,
    tests_helper::{create_timer, run_with_splits_opt, span},
    Timer, TimingMethod,
};

fn assert(timer: &Timer, got: [Option<Prediction>; 4], [a, b, c]: [(f64, usize, bool); 3]) {
    assert_eq!(
        got,
        [
            Prediction {
                time: span(0.0),
                predecessor: 0,
            }
            .into(),
            Prediction {
                time: span(a.0),
                predecessor: a.1,
            }
            .into(),
            Prediction {
                time: span(b.0),
                predecessor: b.1,
            }
            .into(),
            Prediction {
                time: span(c.0),
                predecessor: c.1,
            }
            .into(),
        ]
    );
    for (index, (segment, &(expected, _, should_be_some))) in
        timer.run().segments().iter().zip(&[a, b, c]).enumerate()
    {
        let segment_val = segment.comparison(best_segments::NAME).game_time;
        assert_eq!(
            segment_val,
            if should_be_some {
                Some(span(expected))
            } else {
                None
            },
            "Segment {} was incorrect",
            index,
        );
    }
}

#[test]
pub fn sum_of_best() {
    let mut timer = create_timer(&["A", "B", "C"]);

    run_with_splits_opt(&mut timer, &[Some(5.0), Some(20.0), Some(60.0)]);
    let mut predictions = [None; 4];
    best::calculate(
        timer.run().segments(),
        &mut predictions,
        false,
        false,
        TimingMethod::GameTime,
    );
    assert(
        &timer,
        predictions,
        [(5.0, 0, true), (20.0, 1, true), (60.0, 2, true)],
    );

    run_with_splits_opt(&mut timer, &[None, Some(10.0), None]);
    predictions = [None; 4];
    best::calculate(
        timer.run().segments(),
        &mut predictions,
        false,
        false,
        TimingMethod::GameTime,
    );
    assert(
        &timer,
        predictions,
        [(5.0, 0, false), (10.0, 0, true), (50.0, 2, true)],
    );

    run_with_splits_opt(&mut timer, &[Some(10.0), None, Some(30.0)]);
    predictions = [None; 4];
    best::calculate(
        timer.run().segments(),
        &mut predictions,
        false,
        false,
        TimingMethod::GameTime,
    );
    assert(
        &timer,
        predictions,
        [(5.0, 0, true), (10.0, 0, false), (25.0, 1, true)],
    );

    run_with_splits_opt(&mut timer, &[Some(7.0), Some(10.0), None]);
    predictions = [None; 4];
    best::calculate(
        timer.run().segments(),
        &mut predictions,
        false,
        false,
        TimingMethod::GameTime,
    );
    assert(
        &timer,
        predictions,
        [(5.0, 0, true), (8.0, 1, false), (25.0, 1, true)],
    );

    run_with_splits_opt(&mut timer, &[None, Some(15.0), Some(20.0)]);
    predictions = [None; 4];
    best::calculate(
        timer.run().segments(),
        &mut predictions,
        false,
        false,
        TimingMethod::GameTime,
    );
    assert(
        &timer,
        predictions,
        [(5.0, 0, true), (8.0, 1, true), (13.0, 2, true)],
    );
}
