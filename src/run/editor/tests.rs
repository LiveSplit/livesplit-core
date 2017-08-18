use {Run, Segment};
use super::Editor;

#[test]
fn new_best_segment() {
    let mut run = Run::new();
    run.segments.push(Segment::new(""));
    run.segments.push(Segment::new(""));

    let mut editor = Editor::new(run).unwrap();

    editor
        .selected_segment()
        .parse_and_set_split_time("1:00")
        .unwrap();

    editor.select_only(1);

    editor
        .selected_segment()
        .parse_and_set_split_time("3:00")
        .unwrap();

    editor.insert_segment_above();

    editor
        .selected_segment()
        .parse_and_set_split_time("2:30")
        .unwrap();

    editor
        .selected_segment()
        .parse_and_set_split_time("2:00")
        .unwrap();

    let run = editor.close();

    assert_eq!(
        run.segments[0].personal_best_split_time().real_time,
        Some("1:00".parse().unwrap())
    );
    assert_eq!(
        run.segments[0].best_segment_time.real_time,
        Some("1:00".parse().unwrap())
    );
    assert_eq!(
        run.segments[1].personal_best_split_time().real_time,
        Some("2:00".parse().unwrap())
    );
    assert_eq!(
        run.segments[1].best_segment_time.real_time,
        Some("1:00".parse().unwrap())
    );
    assert_eq!(
        run.segments[2].personal_best_split_time().real_time,
        Some("3:00".parse().unwrap())
    );
    assert_eq!(
        run.segments[2].best_segment_time.real_time,
        Some("0:30".parse().unwrap())
    );
}
