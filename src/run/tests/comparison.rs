use crate::run::{ComparisonError, Run};

#[test]
fn adding_a_new_comparison_works() {
    let mut run = Run::new();
    let c = run.add_custom_comparison("My Comparison");
    assert_eq!(c, Ok(()));
}

#[test]
fn adding_a_duplicate_fails() {
    let mut run = Run::new();
    let c = run.add_custom_comparison("Best Segments");
    assert_eq!(c, Err(ComparisonError::DuplicateName));
}
