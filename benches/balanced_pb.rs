#![feature(test)]

use test::Bencher;

use livesplit_core::comparison::balanced_pb::BalancedPB;
use livesplit_core::run::parser::livesplit;
use livesplit_core::{Run, Segment, TimeSpan, Timer};
use std::fs::File;
use std::io::BufReader;

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
fn fake_splits(b: &mut Bencher) {
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

#[bench]
fn actual_splits(b: &mut Bencher) {
    let reader = BufReader::new(File::open("tests/run_files/livesplit1.6.lss").unwrap());
    let mut run = livesplit::parse(reader, None).unwrap();
    run.comparison_generators_mut().clear();
    run.comparison_generators_mut().push(Box::new(BalancedPB));

    b.iter(|| run.regenerate_comparisons());
}
