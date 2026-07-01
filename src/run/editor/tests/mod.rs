use super::{Editor, SelectionState};
use crate::{
    Lang, Run, Segment, TimeSpan,
    settings::ImageCache,
    util::tests_helper::{create_timer, run_with_splits},
};

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
        .parse_and_set_split_time("1:00", Lang::English)
        .unwrap();

    editor.select_only(1);

    editor
        .active_segment()
        .parse_and_set_split_time("3:00", Lang::English)
        .unwrap();

    editor.insert_segment_above();

    editor
        .active_segment()
        .parse_and_set_split_time("2:30", Lang::English)
        .unwrap();

    editor
        .active_segment()
        .parse_and_set_split_time("2:00", Lang::English)
        .unwrap();

    let run = editor.close();

    assert_eq!(
        run.segment(0).personal_best_split_time().real_time,
        Some(TimeSpan::parse("1:00", Lang::English).unwrap())
    );
    assert_eq!(
        run.segment(0).best_segment_time().real_time,
        Some(TimeSpan::parse("1:00", Lang::English).unwrap())
    );
    assert_eq!(
        run.segment(1).personal_best_split_time().real_time,
        Some(TimeSpan::parse("2:00", Lang::English).unwrap())
    );
    assert_eq!(
        run.segment(1).best_segment_time().real_time,
        Some(TimeSpan::parse("1:00", Lang::English).unwrap())
    );
    assert_eq!(
        run.segment(2).personal_best_split_time().real_time,
        Some(TimeSpan::parse("3:00", Lang::English).unwrap())
    );
    assert_eq!(
        run.segment(2).best_segment_time().real_time,
        Some(TimeSpan::parse("0:30", Lang::English).unwrap())
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
#[should_panic(expected = "Index out of bounds for segment selection.")]
fn select_range_oob() {
    let mut run = Run::new();
    run.push_segment(Segment::new(""));

    let mut editor = Editor::new(run).unwrap();

    editor.select_range(1);
}

#[test]
fn select_range_keeps_existing_selection_and_updates_active_segment() {
    let mut run = Run::new();
    for name in ["A", "B", "C", "D"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_additionally(3);
    editor.select_range(1);

    let mut image_cache = ImageCache::new();
    let state = editor.state(&mut image_cache, Lang::English);

    assert_eq!(state.segments[0].selected, SelectionState::Selected);
    assert_eq!(state.segments[1].selected, SelectionState::Active);
    assert_eq!(state.segments[2].selected, SelectionState::Selected);
    assert_eq!(state.segments[3].selected, SelectionState::Selected);
}

#[test]
fn select_range_preserves_selected_segments_outside_of_the_range() {
    let mut run = Run::new();
    for name in ["A", "B", "C", "D", "E", "F"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_additionally(5);
    editor.select_additionally(2);
    editor.select_range(4);

    let mut image_cache = ImageCache::new();
    let state = editor.state(&mut image_cache, Lang::English);

    assert_eq!(state.segments[0].selected, SelectionState::Selected);
    assert_eq!(state.segments[1].selected, SelectionState::NotSelected);
    assert_eq!(state.segments[2].selected, SelectionState::Selected);
    assert_eq!(state.segments[3].selected, SelectionState::Selected);
    assert_eq!(state.segments[4].selected, SelectionState::Active);
    assert_eq!(state.segments[5].selected, SelectionState::Selected);
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

#[test]
fn creates_renames_and_removes_segment_groups() {
    let mut run = Run::new();
    for name in ["A", "B", "C"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_range(2);
    assert!(editor.create_segment_group_from_selection(Some("Group")));
    assert!(editor.rename_active_segment_group(Some("Renamed")));

    let mut image_cache = ImageCache::new();
    let state = editor.state(&mut image_cache, Lang::English);
    assert_eq!(state.segments[0].segment_group.group_index, Some(0));
    assert!(state.segments[0].segment_group.is_subsplit);
    assert!(state.segments[2].segment_group.is_major_split);
    assert_eq!(
        state.segments[2].segment_group.name.as_deref(),
        Some("Renamed")
    );

    assert!(editor.remove_active_segment_group());
    assert!(editor.close().segment_groups().groups().is_empty());
}

#[test]
fn insertion_and_deletion_repair_segment_groups() {
    let mut run = Run::new();
    for name in ["A", "B", "C"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_range(2);
    assert!(editor.create_segment_group_from_selection::<String>(None));

    editor.select_only(1);
    editor.insert_segment_below();
    assert_eq!(editor.run().segment_groups().groups()[0].end(), 4);

    editor.remove_segments();
    let run = editor.close();
    assert_eq!(
        (
            run.segment_groups().groups()[0].start(),
            run.segment_groups().groups()[0].end()
        ),
        (0, 3)
    );
}
