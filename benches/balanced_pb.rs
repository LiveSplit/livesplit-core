use criterion::{criterion_group, criterion_main, Criterion};

use livesplit_core::{
    comparison::balanced_pb::BalancedPB, run::parser::livesplit, Run, Segment, TimeSpan, Timer,
};
use std::fs;

criterion_main!(benches);
criterion_group!(benches, fake_splits, actual_splits);

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

fn fake_splits(c: &mut Criterion) {
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

    c.bench_function("Balanced PB for synthetic splits", move |b| {
        b.iter(|| run.regenerate_comparisons())
    });
}

fn actual_splits(c: &mut Criterion) {
    let buf = fs::read_to_string("tests/run_files/livesplit1.6.lss").unwrap();
    let mut run = livesplit::parse(&buf).unwrap();
    run.comparison_generators_mut().clear();
    run.comparison_generators_mut().push(Box::new(BalancedPB));

    c.bench_function("Balanced PB for actual splits", move |b| {
        b.iter(|| run.regenerate_comparisons())
    });
}
