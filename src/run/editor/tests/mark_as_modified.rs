use super::super::Editor;
use crate::{Run, Segment, TimeSpan, TimingMethod};

fn base() -> Editor {
    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    run.push_segment(Segment::new(""));
    run.metadata_mut()
        .custom_variable_mut("Foo")
        .permanent()
        .set_value("Bar");

    let mut editor = Editor::new(run).unwrap();

    editor.select_only(0);
    editor
        .active_segment()
        .parse_and_set_split_time("1")
        .unwrap();
    editor.select_only(1);
    editor
        .active_segment()
        .parse_and_set_split_time("2")
        .unwrap();

    editor.add_comparison("Some Comparison").unwrap();
    editor.add_comparison("Other Comparison").unwrap();
    editor.active_segment().set_icon([2]);
    editor.set_game_icon([2]);
    editor.set_speedrun_com_variable("remove", "me");

    let mut run = editor.close();
    run.mark_as_unmodified();
    editor = Editor::new(run).unwrap();

    assert!(!editor.run().has_been_modified());

    editor
}

#[test]
fn not_when_selecting_timing_method() {
    let mut editor = base();
    editor.select_timing_method(TimingMethod::GameTime);
    assert!(!editor.run().has_been_modified());
}

#[test]
fn not_when_just_accessing_active_segment() {
    let mut editor = base();
    editor.active_segment();
    assert!(!editor.run().has_been_modified());
}

#[test]
fn when_changing_segment_icon() {
    let mut editor = base();
    editor.active_segment().set_icon([1]);
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_removing_segment_icon() {
    let mut editor = base();
    // Technically there is no icon in the base run
    editor.active_segment().remove_icon();
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_changing_segment_name() {
    let mut editor = base();
    editor.active_segment().set_name("Hello");
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_changing_split_time() {
    let mut editor = base();
    editor
        .active_segment()
        .parse_and_set_split_time("1.23")
        .unwrap();
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_changing_segment_time() {
    let mut editor = base();
    editor
        .active_segment()
        .parse_and_set_segment_time("1.23")
        .unwrap();
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_changing_best_segment_time() {
    let mut editor = base();
    editor
        .active_segment()
        .parse_and_set_best_segment_time("1.23")
        .unwrap();
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_changing_comparison_time() {
    let mut editor = base();
    editor
        .active_segment()
        .parse_and_set_comparison_time("Some Comparison", "1.23")
        .unwrap();
    assert!(editor.run().has_been_modified());
}

#[test]
fn not_when_selecting_segments() {
    let mut editor = base();
    editor.select_additionally(1);
    editor.unselect(0);
    editor.select_only(1);
    assert!(!editor.run().has_been_modified());
}

#[test]
fn when_changing_game_name() {
    let mut editor = base();
    editor.set_game_name("Hello");
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_changing_category_name() {
    let mut editor = base();
    editor.set_category_name("Hello");
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_changing_offset() {
    let mut editor = base();
    editor.parse_and_set_offset("1.23").unwrap();
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_changing_attempt_count() {
    let mut editor = base();
    editor.set_attempt_count(123);
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_changing_game_icon() {
    let mut editor = base();
    editor.set_game_icon([1]);
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_removing_game_icon() {
    let mut editor = base();
    editor.remove_game_icon();
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_setting_run_id() {
    let mut editor = base();
    editor.set_run_id("hello");
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_setting_region_name() {
    let mut editor = base();
    editor.set_region_name("hello");
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_setting_platform_name() {
    let mut editor = base();
    editor.set_platform_name("hello");
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_setting_emulator_usage() {
    let mut editor = base();
    editor.set_emulator_usage(true);
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_setting_speedrun_com_variable() {
    let mut editor = base();
    editor.set_speedrun_com_variable("some", "variable");
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_removing_speedrun_com_variable() {
    let mut editor = base();
    editor.remove_speedrun_com_variable("remove");
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_clearing_metadata() {
    let mut editor = base();
    editor.clear_metadata();
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_inserting_segment_above() {
    let mut editor = base();
    editor.insert_segment_above();
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_inserting_segment_below() {
    let mut editor = base();
    editor.insert_segment_below();
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_removing_segments() {
    let mut editor = base();
    editor.remove_segments();
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_moving_segments_up() {
    let mut editor = base();
    editor.select_only(1);
    editor.move_segments_up();
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_moving_segments_down() {
    let mut editor = base();
    editor.select_only(0);
    editor.move_segments_down();
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_adding_comparison() {
    let mut editor = base();
    editor.add_comparison("New Comparison").unwrap();
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_importing_comparison() {
    let mut editor = base();
    let mut run = Run::new();
    run.push_segment(Segment::new(""));
    editor.import_comparison(&run, "New Comparison").unwrap();
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_removing_comparison() {
    let mut editor = base();
    editor.remove_comparison("Some Comparison");
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_renaming_comparison() {
    let mut editor = base();
    editor
        .rename_comparison("Some Comparison", "New Comparison")
        .unwrap();
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_moving_comparison() {
    let mut editor = base();
    editor.move_comparison(0, 1).unwrap();
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_clearing_history() {
    let mut editor = base();
    editor.clear_history();
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_clearing_times() {
    let mut editor = base();
    editor.clear_times();
    assert!(editor.run().has_been_modified());
}

#[test]
fn not_when_cleaning_sum_of_best_without_applying_a_fix() {
    let mut editor = base();
    editor.clean_sum_of_best().next_potential_clean_up();
    assert!(!editor.run().has_been_modified());
}

#[test]
fn when_generating_goal_comparison() {
    let mut editor = base();
    editor.generate_goal_comparison(TimeSpan::from_seconds(30.0));
    assert!(editor.run().has_been_modified());
}

// FIXME: Cleaning Sum of Best

#[test]
fn when_creating_custom_variable() {
    let mut editor = base();
    editor.add_custom_variable("Hello");
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_setting_custom_variable() {
    let mut editor = base();
    editor.set_custom_variable("Foo", "Bar2");
    assert!(editor.run().has_been_modified());
}

#[test]
fn when_removing_custom_variable() {
    let mut editor = base();
    editor.remove_custom_variable("Foo");
    assert!(editor.run().has_been_modified());
}
