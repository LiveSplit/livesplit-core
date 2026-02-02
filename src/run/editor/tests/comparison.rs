use crate::{
    Run, Segment,
    comparison::{best_segments, personal_best},
    run::{AddComparisonError, CopyComparisonError, Editor, RenameError},
};

#[test]
fn adding_a_new_comparison_works() {
    let mut run = Run::new();
    run.push_segment(Segment::new("s"));
    let mut editor = Editor::new(run).unwrap();
    let c = editor.add_comparison("My Comparison");
    assert_eq!(c, Ok(()));
}

#[test]
fn adding_a_duplicate_fails() {
    let mut run = Run::new();
    run.push_segment(Segment::new("s"));
    let mut editor = Editor::new(run).unwrap();
    let c = editor.add_comparison(best_segments::NAME);
    assert_eq!(c, Err(AddComparisonError::DuplicateName));
}

#[test]
fn copying_a_comparison_works() {
    let mut run = Run::new();
    run.push_segment(Segment::new("s"));
    let mut editor = Editor::new(run).unwrap();
    let c = editor.copy_comparison(personal_best::NAME, "My Comparison");
    assert_eq!(c, Ok(()));
}

#[test]
fn copying_a_duplicate_fails() {
    let mut run = Run::new();
    run.push_segment(Segment::new("s"));
    let mut editor = Editor::new(run).unwrap();
    let c = editor.copy_comparison(personal_best::NAME, best_segments::NAME);
    assert_eq!(
        c,
        Err(CopyComparisonError::AddComparison {
            source: AddComparisonError::DuplicateName,
        })
    );
}

#[test]
fn copying_to_a_race_name_fails() {
    let mut run = Run::new();
    run.push_segment(Segment::new("s"));
    let mut editor = Editor::new(run).unwrap();
    let c = editor.copy_comparison(personal_best::NAME, "[Race]Custom");
    assert_eq!(
        c,
        Err(CopyComparisonError::AddComparison {
            source: AddComparisonError::NameStartsWithRace,
        })
    );
}

#[test]
fn copying_an_inexistent_comparison_fails() {
    let mut run = Run::new();
    run.push_segment(Segment::new("s"));
    let mut editor = Editor::new(run).unwrap();
    let c = editor.copy_comparison("My Comparison", "My Comparison 2");
    assert_eq!(c, Err(CopyComparisonError::NoSuchComparison));
}

#[test]
fn renaming_works() {
    let mut run = Run::new();
    run.push_segment(Segment::new("s"));
    run.add_custom_comparison("Custom").ok();
    let mut editor = Editor::new(run).unwrap();
    let c = editor.rename_comparison("Custom", "My Comparison");
    assert_eq!(c, Ok(()));
}

#[test]
fn renaming_a_missing_comparison_fails() {
    let mut run = Run::new();
    run.push_segment(Segment::new("s"));
    run.add_custom_comparison("Custom").ok();
    let mut editor = Editor::new(run).unwrap();
    let c = editor.rename_comparison("Bad", "My Comparison");
    assert_eq!(c, Err(RenameError::OldNameNotFound));
}

#[test]
fn renaming_to_a_race_name_fails() {
    let mut run = Run::new();
    run.push_segment(Segment::new("s"));
    run.add_custom_comparison("Custom").ok();
    let mut editor = Editor::new(run).unwrap();
    let c = editor.rename_comparison("Custom", "[Race]Custom");
    assert_eq!(
        c,
        Err(RenameError::InvalidName {
            source: AddComparisonError::NameStartsWithRace
        })
    );
}

#[test]
fn renaming_to_an_existing_name_fails() {
    let mut run = Run::new();
    run.push_segment(Segment::new("s"));
    run.add_custom_comparison("Custom").ok();
    run.add_custom_comparison("Custom2").ok();
    let mut editor = Editor::new(run).unwrap();
    let c = editor.rename_comparison("Custom2", "Custom");
    assert_eq!(
        c,
        Err(RenameError::InvalidName {
            source: AddComparisonError::DuplicateName
        })
    );
}

#[test]
fn reordering_works() {
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
