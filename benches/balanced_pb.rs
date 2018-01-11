#![feature(test)]

extern crate livesplit_core;
extern crate test;

use test::Bencher;

use livesplit_core::{Run, Segment, TimeSpan, Timer};
use livesplit_core::comparison::balanced_pb::{BalancedPB, NAME};
use livesplit_core::time::formatter::{Short, TimeFormatter};

fn run_with_splits(timer: &mut Timer, splits: &[f64]) {
    timer.start();
    timer.initialize_game_time();
    timer.pause_game_time();

    for &split in splits {
        timer.set_game_time(TimeSpan::from_seconds(split));
        timer.split();
    }

    timer.reset(true);
}

#[bench]
fn bench(b: &mut Bencher) {
    let s = TimeSpan::from_seconds;

    let mut run = Run::new();

    run.push_segment(Segment::new("First"));
    run.push_segment(Segment::new("Second"));
    run.push_segment(Segment::new("Third"));

    run.comparison_generators_mut().clear();
    run.comparison_generators_mut().push(Box::new(BalancedPB));

    let mut timer = Timer::new(run).unwrap();

    run_with_splits(&mut timer, &[1.0, 2.0, 3.0]);
    run_with_splits(&mut timer, &[0.5, 2.5, 3.0]);
    run_with_splits(&mut timer, &[0.2, 2.8, 3.0]);

    let mut run = timer.into_run(false);

    b.iter(|| run.regenerate_comparisons());
}
