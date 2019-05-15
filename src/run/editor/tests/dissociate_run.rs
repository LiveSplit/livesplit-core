use super::super::Editor;
use crate::{Run, Segment, TimeSpan};

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
    editor.set_run_id("somerunid");

    editor
}

#[test]
fn when_changing_game_name() {
    let mut editor = base();
    editor.set_game_name("Hello");
    assert_eq!(editor.run().metadata().run_id, "");
}

#[test]
fn when_changing_category_name() {
    let mut editor = base();
    editor.set_category_name("Hello");
    assert_eq!(editor.run().metadata().run_id, "");
}

#[test]
fn not_when_changing_attempt_count() {
    let mut editor = base();
    editor.set_attempt_count(123);
    assert_ne!(editor.run().metadata().run_id, "");
}

#[test]
fn not_when_changing_offset() {
    let mut editor = base();
    editor.parse_and_set_offset("1:23").unwrap();
    assert_ne!(editor.run().metadata().run_id, "");
}

#[test]
fn not_when_changing_first_segment() {
    let mut editor = base();
    editor.select_only(0);
    editor
        .active_segment()
        .parse_and_set_split_time("1:23")
        .unwrap();
    assert_ne!(editor.run().metadata().run_id, "");
}

#[test]
fn when_changing_last_segment() {
    let mut editor = base();
    editor.select_only(1);
    editor
        .active_segment()
        .parse_and_set_split_time("1:23")
        .unwrap();
    assert_eq!(editor.run().metadata().run_id, "");
}

#[test]
fn when_inserting_last_segment() {
    let mut editor = base();
    editor.select_only(1);
    editor.insert_segment_below();
    assert_eq!(editor.run().metadata().run_id, "");
}

#[test]
fn not_when_inserting_above_last_segment() {
    let mut editor = base();
    editor.select_only(1);
    editor.insert_segment_above();
    assert_ne!(editor.run().metadata().run_id, "");
}

#[test]
fn not_when_moving_last_segment() {
    // Because it should keep the final time
    let mut editor = base();
    editor.select_only(1);
    editor.move_segments_up();
    assert_ne!(editor.run().metadata().run_id, "");
}

#[test]
fn not_when_clearing_history() {
    let mut editor = base();
    editor.clear_history();
    assert_ne!(editor.run().metadata().run_id, "");
}

#[test]
fn when_clearing_times() {
    let mut editor = base();
    editor.clear_times();
    assert_eq!(editor.run().metadata().run_id, "");
}

#[test]
fn not_when_adding_comparison() {
    let mut editor = base();
    editor.add_comparison("ok").unwrap();
    assert_ne!(editor.run().metadata().run_id, "");
}

#[test]
fn not_when_removing_first_segment() {
    let mut editor = base();
    editor.select_only(0);
    editor.remove_segments();
    assert_ne!(editor.run().metadata().run_id, "");
}

#[test]
fn when_removing_last_segment() {
    let mut editor = base();
    editor.select_only(1);
    editor.remove_segments();
    assert_eq!(editor.run().metadata().run_id, "");
}

#[test]
fn when_setting_region_name() {
    let mut editor = base();
    editor.set_region_name("ok");
    assert_eq!(editor.run().metadata().run_id, "");
}

#[test]
fn when_setting_platform_name() {
    let mut editor = base();
    editor.set_platform_name("ok");
    assert_eq!(editor.run().metadata().run_id, "");
}

#[test]
fn when_setting_emulator_usage() {
    let mut editor = base();
    editor.set_emulator_usage(true);
    assert_eq!(editor.run().metadata().run_id, "");
}

#[test]
fn when_setting_speedrun_com_variable() {
    let mut editor = base();
    editor.set_speedrun_com_variable("ok", "ok");
    assert_eq!(editor.run().metadata().run_id, "");
}

#[test]
fn when_removing_speedrun_com_variable() {
    let mut editor = base();
    editor.remove_speedrun_com_variable("ok");
    assert_eq!(editor.run().metadata().run_id, "");
}

#[test]
fn when_clearing_metadata() {
    let mut editor = base();
    editor.clear_metadata();
    assert_eq!(editor.run().metadata().run_id, "");
}

#[test]
fn not_when_generating_goal_comparison() {
    let mut editor = base();
    editor.generate_goal_comparison(TimeSpan::from_seconds(30.0));
    assert_ne!(editor.run().metadata().run_id, "");
}

#[test]
fn not_when_creating_custom_variable() {
    let mut editor = base();
    editor.set_custom_variable("Hello", "World");
    assert_ne!(editor.run().metadata().run_id, "");
}

#[test]
fn not_when_setting_custom_variable() {
    let mut editor = base();
    editor.set_custom_variable("Foo", "Bar2");
    assert_ne!(editor.run().metadata().run_id, "");
}

#[test]
fn not_when_removing_custom_variable() {
    let mut editor = base();
    editor.remove_custom_variable("Foo");
    assert_ne!(editor.run().metadata().run_id, "");
}
