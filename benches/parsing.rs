use criterion::{criterion_group, criterion_main, Criterion};

use livesplit_core::run::parser::livesplit;
use std::fs;

criterion_main!(benches);
criterion_group!(benches, huge_game_icon, lots_of_icons, no_icons);

fn huge_game_icon(c: &mut Criterion) {
    let buf = fs::read_to_string("tests/run_files/livesplit1.6_gametime.lss").unwrap();

    c.bench_function("Parse With Huge Game Icon", move |b| {
        b.iter(|| livesplit::parse(&buf).unwrap())
    });
}

fn lots_of_icons(c: &mut Criterion) {
    let buf = fs::read_to_string("tests/run_files/Celeste - Any% (1.2.1.5).lss").unwrap();

    c.bench_function("Parse with lots of Icons", move |b| {
        b.iter(|| livesplit::parse(&buf).unwrap())
    });
}

fn no_icons(c: &mut Criterion) {
    let buf = fs::read_to_string("tests/run_files/livesplit1.6.lss").unwrap();

    c.bench_function("Parse without Icons", move |b| {
        b.iter(|| livesplit::parse(&buf).unwrap())
    });
}
