use criterion::{criterion_group, criterion_main, Criterion};
use livesplit_core::{run::parser::livesplit, settings::ImageCache, Layout, Run, Segment, Timer};
use std::fs;

criterion_main!(benches);
criterion_group!(
    benches,
    no_reuse_real,
    reuse_real,
    no_reuse_artificial,
    reuse_artificial
);

fn artificial() -> (Timer, Layout, ImageCache) {
    let mut run = Run::new();
    run.set_game_name("Game");
    run.set_category_name("Category");
    run.push_segment(Segment::new("Foo"));

    let mut timer = Timer::new(run).unwrap();
    timer.start().unwrap();

    (timer, Layout::default_layout(), ImageCache::new())
}

fn real() -> (Timer, Layout, ImageCache) {
    let buf = fs::read_to_string("tests/run_files/Celeste - Any% (1.2.1.5).lss").unwrap();
    let run = livesplit::parse(&buf).unwrap();

    let mut timer = Timer::new(run).unwrap();
    timer.start().unwrap();

    (timer, Layout::default_layout(), ImageCache::new())
}

fn no_reuse_real(c: &mut Criterion) {
    let (timer, mut layout, mut image_cache) = real();

    c.bench_function("No Reuse (Real)", move |b| {
        b.iter(|| layout.state(&mut image_cache, &timer.snapshot()))
    });
}

fn reuse_real(c: &mut Criterion) {
    let (timer, mut layout, mut image_cache) = real();

    let mut state = layout.state(&mut image_cache, &timer.snapshot());

    c.bench_function("Reuse (Real)", move |b| {
        b.iter(|| layout.update_state(&mut state, &mut image_cache, &timer.snapshot()))
    });
}

fn no_reuse_artificial(c: &mut Criterion) {
    let (timer, mut layout, mut image_cache) = artificial();

    c.bench_function("No Reuse (Artificial)", move |b| {
        b.iter(|| layout.state(&mut image_cache, &timer.snapshot()))
    });
}

fn reuse_artificial(c: &mut Criterion) {
    let (timer, mut layout, mut image_cache) = artificial();

    let mut state = layout.state(&mut image_cache, &timer.snapshot());

    c.bench_function("Reuse (Artificial)", move |b| {
        b.iter(|| layout.update_state(&mut state, &mut image_cache, &timer.snapshot()))
    });
}
