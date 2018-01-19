use {Run, Segment, Time, TimeSpan};
use super::Editor;

#[test]
fn new_best_segment() {
    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    run.push_segment(Segment::new(""));

    let mut editor = Editor::new(run).unwrap();

    editor
        .active_segment()
        .parse_and_set_split_time("1:00")
        .unwrap();

    editor.select_only(1);

    editor
        .active_segment()
        .parse_and_set_split_time("3:00")
        .unwrap();

    editor.insert_segment_above();

    editor
        .active_segment()
        .parse_and_set_split_time("2:30")
        .unwrap();

    editor
        .active_segment()
        .parse_and_set_split_time("2:00")
        .unwrap();

    let run = editor.close();

    assert_eq!(
        run.segment(0).personal_best_split_time().real_time,
        Some("1:00".parse().unwrap())
    );
    assert_eq!(
        run.segment(0).best_segment_time().real_time,
        Some("1:00".parse().unwrap())
    );
    assert_eq!(
        run.segment(1).personal_best_split_time().real_time,
        Some("2:00".parse().unwrap())
    );
    assert_eq!(
        run.segment(1).best_segment_time().real_time,
        Some("1:00".parse().unwrap())
    );
    assert_eq!(
        run.segment(2).personal_best_split_time().real_time,
        Some("3:00".parse().unwrap())
    );
    assert_eq!(
        run.segment(2).best_segment_time().real_time,
        Some("0:30".parse().unwrap())
    );
}

#[test]
#[should_panic(expected = "Index out of bounds for segment selection.")]
fn select_only_oob() {
    let mut run = Run::new();
    run.push_segment(Segment::new(""));

    let mut editor = Editor::new(run).unwrap();

    editor.select_only(1);
}

#[test]
#[should_panic(expected = "Index out of bounds for segment selection.")]
fn select_additionally_oob() {
    let mut run = Run::new();
    run.push_segment(Segment::new(""));

    let mut editor = Editor::new(run).unwrap();

    editor.select_additionally(1);
}

#[test]
fn clear_times_clears_run_id() {
    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    run.metadata_mut().set_run_id("Bla");

    let mut editor = Editor::new(run).unwrap();

    assert_eq!(editor.run().metadata().run_id(), "Bla");
    editor.clear_times();
    assert_eq!(editor.run().metadata().run_id(), "");
}

#[test]
fn setting_game_name_clears_run_id() {
    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    run.metadata_mut().set_run_id("Bla");

    let mut editor = Editor::new(run).unwrap();

    assert_eq!(editor.run().metadata().run_id(), "Bla");
    editor.set_game_name("Hi");
    assert_eq!(editor.run().metadata().run_id(), "");
}

#[test]
fn setting_category_name_clears_run_id() {
    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    run.metadata_mut().set_run_id("Bla");

    let mut editor = Editor::new(run).unwrap();

    assert_eq!(editor.run().metadata().run_id(), "Bla");
    editor.set_category_name("Hi");
    assert_eq!(editor.run().metadata().run_id(), "");
}

#[test]
fn modifying_last_segments_split_time_clears_run_id() {
    let mut run = Run::new();
    let mut segment = Segment::new("");
    let original_time = Some(TimeSpan::from_seconds(1.0));
    segment.set_personal_best_split_time(Time::new().with_real_time(original_time));
    run.push_segment(segment);
    run.metadata_mut().set_run_id("Bla");

    let mut editor = Editor::new(run).unwrap();

    assert_eq!(editor.run().metadata().run_id(), "Bla");
    editor.active_segment().set_split_time(original_time);
    assert_eq!(editor.run().metadata().run_id(), "Bla");
    editor
        .active_segment()
        .set_split_time(Some(TimeSpan::from_seconds(2.0)));
    assert_eq!(editor.run().metadata().run_id(), "");
}
