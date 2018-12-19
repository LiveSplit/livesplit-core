use criterion::{criterion_group, criterion_main, Criterion};

use livesplit_core::run::parser::livesplit;
use std::{fs, io::Cursor};

criterion_main!(benches);
criterion_group!(benches, huge_game_icon, no_icons);

fn huge_game_icon(c: &mut Criterion) {
    let buf = fs::read("tests/run_files/livesplit1.6_gametime.lss").unwrap();

    c.bench_function("Parse With Huge Game Icon", move |b| {
        b.iter(|| livesplit::parse(Cursor::new(&buf), None).unwrap())
    });
}

fn no_icons(c: &mut Criterion) {
    let buf = fs::read("tests/run_files/livesplit1.6.lss").unwrap();

    c.bench_function("Parse without Icons", move |b| {
        b.iter(|| livesplit::parse(Cursor::new(&buf), None).unwrap())
    });
}
