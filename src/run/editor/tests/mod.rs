use super::Editor;
use crate::tests_helper::{create_timer, run_with_splits};
use crate::{Run, Segment};

mod comparison;
mod custom_variables;
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
fn fix_run_upon_creation() {
    let mut timer = create_timer(&["A", "B"]);
    run_with_splits(&mut timer, &[3.0, 6.0]);
    run_with_splits(&mut timer, &[2.0, 4.0]);
    let mut run = timer.into_run(true);

    // We pop the last attempt from the history, but keep it in the segment
    // history, which makes the segment history elements unattached.
    run.attempt_history.pop().unwrap();

    // This should cause the run to be fixed.
    let run = Editor::new(run).unwrap().close();

    let segments = run.segments();

    assert_eq!(segments[0].segment_history().try_get_min_index(), Some(0));
    assert_eq!(segments[0].segment_history().try_get_max_index(), Some(1));

    assert_eq!(segments[1].segment_history().try_get_min_index(), Some(0));
    assert_eq!(segments[1].segment_history().try_get_max_index(), Some(1));
}
