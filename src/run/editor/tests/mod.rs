use super::{Editor, SelectionState};
use crate::{
    Lang, Run, Segment, TimeSpan,
    settings::{Image, ImageCache},
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
fn renaming_segment_group_keeps_whole_group_selected() {
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

    assert_eq!(state.segments[0].selected, SelectionState::Selected);
    assert_eq!(state.segments[1].selected, SelectionState::Selected);
    assert_eq!(state.segments[2].selected, SelectionState::Active);
}

#[test]
fn segment_group_icons_can_be_explicit_or_inherited() {
    let mut run = Run::new();
    for name in ["A", "B", "C"] {
        run.push_segment(Segment::new(name));
    }
    let major_icon = Image::new([1, 2, 3].as_slice().into(), Image::ICON);
    run.segment_mut(2).set_icon(major_icon.clone());

    let mut editor = Editor::new(run).unwrap();
    editor.select_range(2);
    assert!(editor.create_segment_group_from_selection(Some("Group")));

    let mut image_cache = ImageCache::new();
    let state = editor.state(&mut image_cache, Lang::English);
    assert_eq!(state.segments[0].segment_group.icon, *major_icon.id());
    assert!(!state.segments[0].segment_group.has_explicit_icon);

    let group_icon = Image::new([4, 5, 6].as_slice().into(), Image::ICON);
    assert!(editor.set_active_segment_group_icon(group_icon.clone()));

    let state = editor.state(&mut image_cache, Lang::English);
    assert_eq!(state.segments[0].segment_group.icon, *group_icon.id());
    assert!(state.segments[0].segment_group.has_explicit_icon);

    assert!(editor.remove_active_segment_group_icon());

    let state = editor.state(&mut image_cache, Lang::English);
    assert_eq!(state.segments[0].segment_group.icon, *major_icon.id());
    assert!(!state.segments[0].segment_group.has_explicit_icon);
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
    assert!(!editor.can_remove_active_segment_group());
    assert!(!editor.create_segment_group_from_selection::<String>(None));
    assert!(!editor.remove_active_segment_group());

    editor.select_only(0);
    editor.select_range(2);
    assert!(editor.can_remove_active_segment_group());
    assert!(editor.remove_active_segment_group());
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
