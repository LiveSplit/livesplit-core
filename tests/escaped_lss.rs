use livesplit_core::{
    run::{parser, saver},
    Run, Segment,
};

#[test]
fn escaping_works_for_segment_name() {
    let mut run = Run::new();
    run.push_segment(Segment::new("A < B"));

    let mut buf = String::new();
    saver::livesplit::save_run(&run, &mut buf).unwrap();
    assert!(buf.contains("A &lt; B"));

    run = parser::livesplit::parse(&buf).unwrap();
    assert_eq!(run.segment(0).name(), "A < B");
}

#[test]
fn escaping_works_for_auto_splitter_settings() {
    let mut run = Run::new();
    run.auto_splitter_settings_mut()
        .push_str("<Hi>A &lt; B</Hi>");

    let mut buf = String::new();
    saver::livesplit::save_run(&run, &mut buf).unwrap();
    assert!(buf.contains("A &lt; B"));

    run = parser::livesplit::parse(&buf).unwrap();
    assert_eq!(run.auto_splitter_settings(), "<Hi>A &lt; B</Hi>");
}
