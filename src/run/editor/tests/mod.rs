use super::{Editor, RowState, SegmentGroupState, SegmentState, SelectionState, State};
use crate::{
    Lang, Run, Segment, TimeSpan,
    settings::{Image, ImageCache},
    util::tests_helper::{create_timer, run_with_splits},
};

mod comparison;
mod custom_variables;
mod dissociate_run;
mod mark_as_modified;

fn segment(state: &State, segment_index: usize) -> &SegmentState {
    state
        .rows
        .iter()
        .find_map(|row| match row {
            RowState::Segment(segment) if segment.segment_index == segment_index => Some(segment),
            _ => None,
        })
        .unwrap()
}

fn segment_group(state: &State, group_index: usize) -> &SegmentGroupState {
    state
        .rows
        .iter()
        .find_map(|row| match row {
            RowState::SegmentGroup(group) if group.group_index == group_index => Some(group),
            _ => None,
        })
        .unwrap()
}

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

    assert_eq!(segment(&state, 0).selected, SelectionState::Selected);
    assert_eq!(segment(&state, 1).selected, SelectionState::Active);
    assert_eq!(segment(&state, 2).selected, SelectionState::Selected);
    assert_eq!(segment(&state, 3).selected, SelectionState::Selected);
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

    assert_eq!(segment(&state, 0).selected, SelectionState::Selected);
    assert_eq!(segment(&state, 1).selected, SelectionState::NotSelected);
    assert_eq!(segment(&state, 2).selected, SelectionState::Selected);
    assert_eq!(segment(&state, 3).selected, SelectionState::Selected);
    assert_eq!(segment(&state, 4).selected, SelectionState::Active);
    assert_eq!(segment(&state, 5).selected, SelectionState::Selected);
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
    assert!(editor.rename_segment_group(0, Some("Renamed")));

    let mut image_cache = ImageCache::new();
    let state = editor.state(&mut image_cache, Lang::English);
    assert_eq!(state.rows.len(), 4);
    assert!(matches!(state.rows[0], RowState::SegmentGroup(_)));
    assert_eq!(segment_group(&state, 0).name, "Renamed");
    assert_eq!(
        segment_group(&state, 0).explicit_name.as_deref(),
        Some("Renamed")
    );
    assert!(segment_group(&state, 0).selected);
    assert!(segment(&state, 0).is_indented);

    assert!(editor.remove_selected_segment_groups());
    assert!(editor.close().segment_groups().groups().is_empty());
}

#[test]
fn state_presents_segment_groups_as_unified_rows() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A End", "Outro"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_only(1);
    editor.select_range(2);
    assert!(editor.create_segment_group_from_selection::<String>(None));

    let mut image_cache = ImageCache::new();
    let state = editor.state(&mut image_cache, Lang::English);

    assert!(matches!(state.rows[0], RowState::Segment(_)));
    assert!(matches!(state.rows[1], RowState::SegmentGroup(_)));
    assert!(matches!(state.rows[2], RowState::Segment(_)));
    assert!(matches!(state.rows[3], RowState::Segment(_)));
    assert!(matches!(state.rows[4], RowState::Segment(_)));

    let group = segment_group(&state, 0);
    assert_eq!(group.name, "A End");
    assert_eq!(group.explicit_name, None);
    assert!(group.selected);

    assert!(!segment(&state, 0).is_indented);
    assert!(segment(&state, 1).is_indented);
    assert!(segment(&state, 3).starts_new_section);

    // The tagged representation lets dynamically typed frontends switch on a
    // row's kind without correlating multiple arrays or inferring headers from
    // their position.
    let json = serde_json::to_value(state).unwrap();
    assert_eq!(json["rows"][1]["kind"], "SegmentGroup");
}

#[test]
fn renaming_segment_group_keeps_whole_group_selected() {
    let mut run = Run::new();
    for name in ["A", "B", "C"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_range(2);
    assert!(editor.create_segment_group_from_selection(Some("Group")));
    assert!(editor.rename_segment_group(0, Some("Renamed")));

    let mut image_cache = ImageCache::new();
    let state = editor.state(&mut image_cache, Lang::English);

    assert_eq!(segment(&state, 0).selected, SelectionState::Selected);
    assert_eq!(segment(&state, 1).selected, SelectionState::Selected);
    assert_eq!(segment(&state, 2).selected, SelectionState::Active);
}

#[test]
fn selecting_segment_group_uses_its_state_identity() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A End", "Outro"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_only(1);
    editor.select_range(2);
    assert!(editor.create_segment_group_from_selection(Some("Group")));
    editor.select_only(0);

    assert!(editor.select_segment_group(0));
    assert!(!editor.select_segment_group(1));

    let mut image_cache = ImageCache::new();
    let state = editor.state(&mut image_cache, Lang::English);
    assert_eq!(segment(&state, 0).selected, SelectionState::NotSelected);
    assert_eq!(segment(&state, 1).selected, SelectionState::Selected);
    assert_eq!(segment(&state, 2).selected, SelectionState::Active);
    assert!(segment_group(&state, 0).selected);
}

#[test]
fn segment_group_icons_can_be_explicit_or_inherited() {
    let mut run = Run::new();
    for name in ["A", "B", "C"] {
        run.push_segment(Segment::new(name));
    }
    let last_segment_icon = Image::new([1, 2, 3].as_slice().into(), Image::ICON);
    run.segment_mut(2).set_icon(last_segment_icon.clone());

    let mut editor = Editor::new(run).unwrap();
    editor.select_range(2);
    assert!(editor.create_segment_group_from_selection(Some("Group")));

    let mut image_cache = ImageCache::new();
    let state = editor.state(&mut image_cache, Lang::English);
    assert_eq!(segment_group(&state, 0).icon, *last_segment_icon.id());
    assert!(!segment_group(&state, 0).has_explicit_icon);

    let group_icon = Image::new([4, 5, 6].as_slice().into(), Image::ICON);
    assert!(editor.set_segment_group_icon(0, group_icon.clone()));

    let state = editor.state(&mut image_cache, Lang::English);
    assert_eq!(segment_group(&state, 0).icon, *group_icon.id());
    assert!(segment_group(&state, 0).has_explicit_icon);

    assert!(editor.remove_segment_group_icon(0));

    let state = editor.state(&mut image_cache, Lang::English);
    assert_eq!(segment_group(&state, 0).icon, *last_segment_icon.id());
    assert!(!segment_group(&state, 0).has_explicit_icon);
}

#[test]
fn creates_single_segment_group() {
    let mut run = Run::new();
    for name in ["A", "B", "C"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    assert!(editor.can_create_segment_group_from_selection());
    let mut image_cache = ImageCache::new();
    assert!(
        editor
            .state(&mut image_cache, Lang::English)
            .buttons
            .can_create_segment_group
    );
    assert!(editor.create_segment_group_from_selection(Some("Group")));

    let run = editor.close();
    assert_eq!(run.segment_groups().groups().len(), 1);
    assert_eq!(
        (
            run.segment_groups().groups()[0].start(),
            run.segment_groups().groups()[0].end(),
            run.segment_groups().groups()[0].name()
        ),
        (0, 1, Some("Group"))
    );
}
#[test]
fn mixed_selection_cannot_create_or_remove_segment_group() {
    let mut run = Run::new();
    for name in ["A", "B", "C", "D"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_range(2);
    assert!(editor.create_segment_group_from_selection(Some("Group")));

    editor.select_only(2);
    editor.select_range(3);
    assert!(!editor.can_create_segment_group_from_selection());
    assert!(!editor.can_remove_selected_segment_groups());
    assert!(!editor.create_segment_group_from_selection::<String>(None));
    assert!(!editor.remove_selected_segment_groups());

    editor.select_only(0);
    editor.select_range(2);
    assert!(editor.can_remove_selected_segment_groups());
    assert!(editor.remove_selected_segment_groups());
}

#[test]
fn removes_multiple_selected_segment_groups() {
    let mut run = Run::new();
    for name in ["A", "B", "C", "D", "E", "F"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_range(1);
    assert!(editor.create_segment_group_from_selection(Some("Group A")));
    editor.select_only(2);
    editor.select_range(4);
    assert!(editor.create_segment_group_from_selection(Some("Group B")));

    editor.select_only(0);
    editor.select_additionally(1);
    editor.select_additionally(2);
    editor.select_additionally(3);
    editor.select_additionally(4);
    assert!(editor.can_remove_selected_segment_groups());
    assert!(editor.remove_selected_segment_groups());
    assert!(editor.run().segment_groups().groups().is_empty());
}

#[test]
fn moving_multiple_selected_segment_groups_is_disabled() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A End", "B1", "B End", "Outro"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_only(1);
    editor.select_range(2);
    assert!(editor.create_segment_group_from_selection(Some("Group A")));
    editor.select_only(3);
    editor.select_range(4);
    assert!(editor.create_segment_group_from_selection(Some("Group B")));

    // This matches a Shift selection of the two complete groups in the Run
    // Editor. It must not take the per-segment movement path, which would
    // otherwise peel the selected boundary segments out of both groups.
    editor.select_only(1);
    editor.select_range(4);
    assert!(!editor.can_move_segments_up());
    assert!(!editor.can_move_segments_down());

    editor.move_segments_up();
    editor.move_segments_down();

    let run = editor.close();
    assert_eq!(
        run.segments().iter().map(Segment::name).collect::<Vec<_>>(),
        ["Intro", "A1", "A End", "B1", "B End", "Outro"],
    );
    assert_eq!(
        run.segment_groups()
            .groups()
            .iter()
            .map(|group| (group.start(), group.end(), group.name()))
            .collect::<Vec<_>>(),
        [(1, 3, Some("Group A")), (3, 5, Some("Group B"))],
    );
}

#[test]
fn ungrouped_selection_before_group_can_create_segment_group() {
    let mut run = Run::new();
    for name in ["A", "B", "C", "D"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_only(1);
    editor.select_range(3);
    assert!(editor.create_segment_group_from_selection(Some("Group")));

    editor.select_only(1);
    editor.move_segments_up();

    editor.select_only(0);
    editor.select_range(1);
    assert!(editor.can_create_segment_group_from_selection());
    assert!(editor.create_segment_group_from_selection(Some("New Group")));
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

#[test]
fn inserting_above_selected_group_places_segment_before_group() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A End", "Outro"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_only(1);
    editor.select_range(2);
    assert!(editor.create_segment_group_from_selection(Some("Chapter A")));
    editor.select_only(0);
    assert!(editor.select_segment_group(0));

    editor.insert_segment_above();

    assert_eq!(
        editor
            .run()
            .segments()
            .iter()
            .map(Segment::name)
            .collect::<Vec<_>>(),
        ["Intro", "", "A1", "A End", "Outro"]
    );
    let group = &editor.run().segment_groups().groups()[0];
    assert_eq!((group.start(), group.end()), (2, 4));
}

#[test]
fn inserting_below_reversed_selection_places_segment_after_selection() {
    let mut run = Run::new();
    for name in ["A", "B", "C", "D"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_only(2);
    editor.select_range(1);

    editor.insert_segment_below();

    assert_eq!(
        editor
            .run()
            .segments()
            .iter()
            .map(Segment::name)
            .collect::<Vec<_>>(),
        ["A", "B", "C", "", "D"]
    );
}

#[test]
fn moving_segments_inside_group_keeps_segment_group() {
    let mut run = Run::new();
    for name in ["A", "B", "C", "D"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_range(2);
    assert!(editor.create_segment_group_from_selection(Some("Group")));

    editor.select_only(1);
    editor.move_segments_down();

    assert_eq!(editor.run().segments()[1].name(), "C");
    assert_eq!(editor.run().segments()[2].name(), "B");
    assert_eq!(
        (
            editor.run().segment_groups().groups()[0].start(),
            editor.run().segment_groups().groups()[0].end(),
            editor.run().segment_groups().groups()[0].name()
        ),
        (0, 3, Some("Group"))
    );

    editor.move_segments_up();

    let run = editor.close();
    assert_eq!(run.segments()[1].name(), "B");
    assert_eq!(run.segments()[2].name(), "C");
    assert_eq!(
        (
            run.segment_groups().groups()[0].start(),
            run.segment_groups().groups()[0].end(),
            run.segment_groups().groups()[0].name()
        ),
        (0, 3, Some("Group"))
    );
}

#[test]
fn moving_segments_across_group_boundary_keeps_segment_group() {
    let mut run = Run::new();
    for name in ["A", "B", "C", "D", "E", "F"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_range(2);
    assert!(editor.create_segment_group_from_selection(Some("Group 1")));
    editor.select_only(3);
    editor.select_range(5);
    assert!(editor.create_segment_group_from_selection(Some("Group 2")));

    editor.select_only(2);
    editor.move_segments_down();

    assert_eq!(editor.run().segments()[0].name(), "A");
    assert_eq!(editor.run().segments()[1].name(), "B");
    assert_eq!(editor.run().segments()[2].name(), "C");
    assert_eq!(editor.run().segments()[3].name(), "D");
    assert_eq!(editor.run().segments()[4].name(), "E");
    assert_eq!(editor.run().segments()[5].name(), "F");
    assert_eq!(
        (
            editor.run().segment_groups().groups()[0].start(),
            editor.run().segment_groups().groups()[0].end(),
            editor.run().segment_groups().groups()[0].name()
        ),
        (0, 2, Some("Group 1"))
    );
    assert_eq!(
        (
            editor.run().segment_groups().groups()[1].start(),
            editor.run().segment_groups().groups()[1].end(),
            editor.run().segment_groups().groups()[1].name()
        ),
        (3, 6, Some("Group 2"))
    );

    editor.move_segments_down();

    assert_eq!(editor.run().segments()[0].name(), "A");
    assert_eq!(editor.run().segments()[1].name(), "B");
    assert_eq!(editor.run().segments()[2].name(), "C");
    assert_eq!(editor.run().segments()[3].name(), "D");
    assert_eq!(editor.run().segments()[4].name(), "E");
    assert_eq!(editor.run().segments()[5].name(), "F");
    assert_eq!(
        (
            editor.run().segment_groups().groups()[0].start(),
            editor.run().segment_groups().groups()[0].end(),
            editor.run().segment_groups().groups()[0].name()
        ),
        (0, 2, Some("Group 1"))
    );
    assert_eq!(
        (
            editor.run().segment_groups().groups()[1].start(),
            editor.run().segment_groups().groups()[1].end(),
            editor.run().segment_groups().groups()[1].name()
        ),
        (2, 6, Some("Group 2"))
    );

    editor.move_segments_down();

    assert_eq!(editor.run().segments()[0].name(), "A");
    assert_eq!(editor.run().segments()[1].name(), "B");
    assert_eq!(editor.run().segments()[2].name(), "D");
    assert_eq!(editor.run().segments()[3].name(), "C");
    assert_eq!(editor.run().segments()[4].name(), "E");
    assert_eq!(editor.run().segments()[5].name(), "F");
    assert_eq!(
        (
            editor.run().segment_groups().groups()[0].start(),
            editor.run().segment_groups().groups()[0].end(),
            editor.run().segment_groups().groups()[0].name()
        ),
        (0, 2, Some("Group 1"))
    );
    assert_eq!(
        (
            editor.run().segment_groups().groups()[1].start(),
            editor.run().segment_groups().groups()[1].end(),
            editor.run().segment_groups().groups()[1].name()
        ),
        (2, 6, Some("Group 2"))
    );
}

#[test]
fn moving_whole_segment_group_down_moves_it_past_next_group() {
    let mut run = Run::new();
    for name in ["A1", "A2", "A End", "B1", "B2", "B End", "Outro"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_range(2);
    assert!(editor.create_segment_group_from_selection(Some("Group A")));
    editor.select_only(3);
    editor.select_range(5);
    assert!(editor.create_segment_group_from_selection(Some("Group B")));

    editor.select_only(0);
    editor.select_range(2);
    assert!(editor.can_move_segments_down());

    editor.move_segments_down();

    let run = editor.close();
    assert_eq!(run.segments()[0].name(), "B1");
    assert_eq!(run.segments()[1].name(), "B2");
    assert_eq!(run.segments()[2].name(), "B End");
    assert_eq!(run.segments()[3].name(), "A1");
    assert_eq!(run.segments()[4].name(), "A2");
    assert_eq!(run.segments()[5].name(), "A End");
    assert_eq!(run.segments()[6].name(), "Outro");
    assert_eq!(
        (
            run.segment_groups().groups()[0].start(),
            run.segment_groups().groups()[0].end(),
            run.segment_groups().groups()[0].name()
        ),
        (0, 3, Some("Group B"))
    );
    assert_eq!(
        (
            run.segment_groups().groups()[1].start(),
            run.segment_groups().groups()[1].end(),
            run.segment_groups().groups()[1].name()
        ),
        (3, 6, Some("Group A"))
    );
}

#[test]
fn moving_whole_segment_group_up_moves_it_past_previous_group() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A2", "A End", "B1", "B2", "B End"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_only(1);
    editor.select_range(3);
    assert!(editor.create_segment_group_from_selection(Some("Group A")));
    editor.select_only(4);
    editor.select_range(6);
    assert!(editor.create_segment_group_from_selection(Some("Group B")));

    editor.select_only(4);
    editor.select_range(6);
    assert!(editor.can_move_segments_up());

    editor.move_segments_up();

    let run = editor.close();
    assert_eq!(run.segments()[0].name(), "Intro");
    assert_eq!(run.segments()[1].name(), "B1");
    assert_eq!(run.segments()[2].name(), "B2");
    assert_eq!(run.segments()[3].name(), "B End");
    assert_eq!(run.segments()[4].name(), "A1");
    assert_eq!(run.segments()[5].name(), "A2");
    assert_eq!(run.segments()[6].name(), "A End");
    assert_eq!(
        (
            run.segment_groups().groups()[0].start(),
            run.segment_groups().groups()[0].end(),
            run.segment_groups().groups()[0].name()
        ),
        (1, 4, Some("Group B"))
    );
    assert_eq!(
        (
            run.segment_groups().groups()[1].start(),
            run.segment_groups().groups()[1].end(),
            run.segment_groups().groups()[1].name()
        ),
        (4, 7, Some("Group A"))
    );
}
#[test]
fn moving_whole_segment_group_down_moves_it_past_ungrouped_segment() {
    let mut run = Run::new();
    for name in ["A1", "A2", "A End", "Outro"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_range(2);
    assert!(editor.create_segment_group_from_selection(Some("Group A")));
    assert!(editor.can_move_segments_down());

    editor.move_segments_down();

    let run = editor.close();
    assert_eq!(run.segments()[0].name(), "Outro");
    assert_eq!(run.segments()[1].name(), "A1");
    assert_eq!(run.segments()[2].name(), "A2");
    assert_eq!(run.segments()[3].name(), "A End");
    assert_eq!(
        (
            run.segment_groups().groups()[0].start(),
            run.segment_groups().groups()[0].end(),
            run.segment_groups().groups()[0].name()
        ),
        (1, 4, Some("Group A"))
    );
}

#[test]
fn moving_whole_segment_group_up_moves_it_past_ungrouped_segment() {
    let mut run = Run::new();
    for name in ["Intro", "A1", "A2", "A End"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_only(1);
    editor.select_range(3);
    assert!(editor.create_segment_group_from_selection(Some("Group A")));
    assert!(editor.can_move_segments_up());

    editor.move_segments_up();

    let run = editor.close();
    assert_eq!(run.segments()[0].name(), "A1");
    assert_eq!(run.segments()[1].name(), "A2");
    assert_eq!(run.segments()[2].name(), "A End");
    assert_eq!(run.segments()[3].name(), "Intro");
    assert_eq!(
        (
            run.segment_groups().groups()[0].start(),
            run.segment_groups().groups()[0].end(),
            run.segment_groups().groups()[0].name()
        ),
        (0, 3, Some("Group A"))
    );
}
#[test]
fn moving_edge_segment_out_can_leave_single_segment_group() {
    let mut run = Run::new();
    for name in ["A", "B", "C"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_range(1);
    assert!(editor.create_segment_group_from_selection(Some("Group")));
    editor.select_only(0);
    assert!(editor.can_move_segments_up());

    editor.move_segments_up();

    let run = editor.close();
    assert_eq!(run.segments()[0].name(), "A");
    assert_eq!(run.segments()[1].name(), "B");
    assert_eq!(run.segments()[2].name(), "C");
    assert_eq!(
        (
            run.segment_groups().groups()[0].start(),
            run.segment_groups().groups()[0].end(),
            run.segment_groups().groups()[0].name()
        ),
        (1, 2, Some("Group"))
    );
}

#[test]
fn moving_last_of_two_segment_group_out_does_not_backfill_group() {
    let mut run = Run::new();
    for name in ["A", "B", "C", "D"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_range(1);
    assert!(editor.create_segment_group_from_selection(Some("Group")));
    editor.select_only(1);
    assert!(editor.can_move_segments_down());

    editor.move_segments_down();

    let run = editor.close();
    assert_eq!(run.segments()[0].name(), "A");
    assert_eq!(run.segments()[1].name(), "B");
    assert_eq!(run.segments()[2].name(), "C");
    assert_eq!(run.segments()[3].name(), "D");
    assert_eq!(
        (
            run.segment_groups().groups()[0].start(),
            run.segment_groups().groups()[0].end(),
            run.segment_groups().groups()[0].name()
        ),
        (0, 1, Some("Group"))
    );
}

#[test]
fn moving_single_segment_group_moves_the_group() {
    let mut run = Run::new();
    for name in ["A", "B", "C"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    assert!(editor.create_segment_group_from_selection(Some("Group")));
    assert!(editor.can_move_segments_down());

    editor.move_segments_down();

    let run = editor.close();
    assert_eq!(run.segments()[0].name(), "B");
    assert_eq!(run.segments()[1].name(), "A");
    assert_eq!(run.segments()[2].name(), "C");
    assert_eq!(
        (
            run.segment_groups().groups()[0].start(),
            run.segment_groups().groups()[0].end(),
            run.segment_groups().groups()[0].name()
        ),
        (1, 2, Some("Group"))
    );
}
#[test]
fn moving_last_segment_down_can_move_it_out_of_final_group() {
    let mut run = Run::new();
    for name in ["A", "B", "C"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_range(2);
    assert!(editor.create_segment_group_from_selection(Some("Group")));
    editor.select_only(2);
    assert!(editor.can_move_segments_down());

    editor.move_segments_down();

    let run = editor.close();
    assert_eq!(run.segments()[0].name(), "A");
    assert_eq!(run.segments()[1].name(), "B");
    assert_eq!(run.segments()[2].name(), "C");
    assert_eq!(
        (
            run.segment_groups().groups()[0].start(),
            run.segment_groups().groups()[0].end(),
            run.segment_groups().groups()[0].name()
        ),
        (0, 2, Some("Group"))
    );
}

#[test]
fn moving_first_segment_up_can_move_it_out_of_first_group() {
    let mut run = Run::new();
    for name in ["A", "B", "C"] {
        run.push_segment(Segment::new(name));
    }

    let mut editor = Editor::new(run).unwrap();
    editor.select_range(2);
    assert!(editor.create_segment_group_from_selection(Some("Group")));
    editor.select_only(0);
    assert!(editor.can_move_segments_up());

    editor.move_segments_up();

    let run = editor.close();
    assert_eq!(run.segments()[0].name(), "A");
    assert_eq!(run.segments()[1].name(), "B");
    assert_eq!(run.segments()[2].name(), "C");
    assert_eq!(
        (
            run.segment_groups().groups()[0].start(),
            run.segment_groups().groups()[0].end(),
            run.segment_groups().groups()[0].name()
        ),
        (1, 3, Some("Group"))
    );
}
