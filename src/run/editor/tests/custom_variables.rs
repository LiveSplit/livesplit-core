use super::super::Editor;
use crate::tests_helper::create_run;

#[test]
fn cant_edit_temporary_variable() {
    let mut run = create_run(&["A"]);
    run.metadata_mut()
        .custom_variable_mut("Foo")
        .set_value("Bar");
    let mut editor = Editor::new(run).unwrap();

    editor.set_custom_variable("Foo", "No");

    assert_eq!(
        editor
            .run()
            .metadata()
            .custom_variable("Foo")
            .unwrap()
            .value,
        "Bar",
    );
}

#[test]
fn can_edit_permanent_variable() {
    let mut run = create_run(&["A"]);
    run.metadata_mut()
        .custom_variable_mut("Foo")
        .permanent()
        .set_value("Bar");
    let mut editor = Editor::new(run).unwrap();

    editor.set_custom_variable("Foo", "Ok");

    assert_eq!(
        editor
            .run()
            .metadata()
            .custom_variable("Foo")
            .unwrap()
            .value,
        "Ok",
    );
}
