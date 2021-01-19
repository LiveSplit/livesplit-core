use crate::run::Run;

#[test]
fn no_parentheses() {
    let mut run = Run::new();
    run.set_category_name("100% Speedrun");

    let name = run.extended_category_name(false, false, false).to_string();
    assert_eq!(name, "100% Speedrun");
}

#[test]
fn ends_with_parentheses() {
    let mut run = Run::new();
    run.set_category_name("Any% (No Tuner)");

    let name = run.extended_category_name(false, false, false).to_string();
    assert_eq!(name, "Any% (No Tuner)");
}

#[test]
fn has_parentheses() {
    let mut run = Run::new();
    run.set_category_name("Any% (Tuner) Speedrun");

    let name = run.extended_category_name(false, false, false).to_string();
    assert_eq!(name, "Any% (Tuner) Speedrun");
}

#[test]
fn no_parentheses_with_additional_info() {
    let mut run = Run::new();
    run.set_category_name("100% Speedrun");
    let metadata = run.metadata_mut();
    metadata.set_region_name("REGION");

    let name = run.extended_category_name(true, false, false).to_string();
    assert_eq!(name, "100% Speedrun (REGION)");
}

#[test]
fn has_parentheses_with_additional_info() {
    let mut run = Run::new();
    run.set_category_name("Any% (Tuner) Speedrun");
    let metadata = run.metadata_mut();
    metadata.set_region_name("REGION");

    let name = run.extended_category_name(true, false, false).to_string();
    assert_eq!(name, "Any% (Tuner, REGION) Speedrun");
}
