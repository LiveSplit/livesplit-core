use super::Editor;
use crate::{Run, Segment};

mod dissociate_run;
mod mark_as_modified;

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
fn move_comparison() {
    let mut run = Run::new();
    let segment = Segment::new("");
    run.push_segment(segment);
    run.add_custom_comparison("A").unwrap();
    run.add_custom_comparison("B").unwrap();
    run.add_custom_comparison("C").unwrap();
    run.add_custom_comparison("D").unwrap();
    let mut editor = Editor::new(run).unwrap();

    editor.move_comparison(3, 1).unwrap();
    assert_eq!(
        &editor.run().custom_comparisons()[1..],
        ["A", "D", "B", "C"]
    );
    editor.move_comparison(0, 2).unwrap();
    assert_eq!(
        &editor.run().custom_comparisons()[1..],
        ["D", "B", "A", "C"]
    );
    editor.move_comparison(1, 3).unwrap();
    assert_eq!(
        &editor.run().custom_comparisons()[1..],
        ["D", "A", "C", "B"]
    );
    editor.move_comparison(3, 0).unwrap();
    assert_eq!(
        &editor.run().custom_comparisons()[1..],
        ["B", "D", "A", "C"]
    );
    editor.move_comparison(0, 3).unwrap();
    assert_eq!(
        &editor.run().custom_comparisons()[1..],
        ["D", "A", "C", "B"]
    );
}
