#![feature(test)]

extern crate livesplit_core;
extern crate test;

use test::Bencher;

use livesplit_core::run::parser::livesplit;
use std::fs::File;
use std::io::{BufReader, Cursor, Read};

#[bench]
fn huge_game_icon(b: &mut Bencher) {
    let mut reader =
        BufReader::new(File::open("tests/run_files/livesplit1.6_gametime.lss").unwrap());
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();

    b.iter(|| livesplit::parse(Cursor::new(&buf), None).unwrap());
}

#[bench]
fn no_icons(b: &mut Bencher) {
    let mut reader = BufReader::new(File::open("tests/run_files/livesplit1.6.lss").unwrap());
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();

    b.iter(|| livesplit::parse(Cursor::new(&buf), None).unwrap());
}
