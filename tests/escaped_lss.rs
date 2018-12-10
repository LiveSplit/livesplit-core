use livesplit_core::run::{parser, saver};
use livesplit_core::{Run, Segment};
use memmem::{Searcher, TwoWaySearcher};

#[test]
fn escaping_works_for_segment_name() {
    let mut run = Run::new();
    run.push_segment(Segment::new("A < B"));

    let mut buf = Vec::new();
    saver::livesplit::save_run(&run, &mut buf).unwrap();
    assert!(TwoWaySearcher::new(b"A &lt; B").search_in(&buf).is_some());

    run = parser::livesplit::parse(buf.as_slice(), None).unwrap();
    assert_eq!(run.segment(0).name(), "A < B");
}

#[test]
fn escaping_works_for_auto_splitter_settings() {
    let mut run = Run::new();
    run.auto_splitter_settings_mut()
        .extend(b"<Hi>A &lt; B</Hi>");

    let mut buf = Vec::new();
    saver::livesplit::save_run(&run, &mut buf).unwrap();
    assert!(TwoWaySearcher::new(b"A &lt; B").search_in(&buf).is_some());

    run = parser::livesplit::parse(buf.as_slice(), None).unwrap();
    assert_eq!(run.auto_splitter_settings(), b"<Hi>A &lt; B</Hi>");
}
