use super::{Component, Text, TextState};
use crate::{tests_helper::create_run, timing::formatter, Timer};

#[test]
fn resolves_variables() {
    let mut run = create_run(&["A"]);
    run.metadata_mut().custom_variable_mut("Goal Time").value = String::from("4:20:00");
    let timer = Timer::new(run).unwrap();

    let mut component = Component::new();
    component.settings_mut().text = Text::Variable(String::from("Goal Time"), true);

    let state = component.state(&timer);

    assert_eq!(
        state.text,
        TextState::Split(String::from("Goal Time"), String::from("4:20:00"))
    );

    component.settings_mut().text = Text::Variable(String::from("Goal Time"), false);

    let state = component.state(&timer);

    assert_eq!(state.text, TextState::Center(String::from("4:20:00")));
}

#[test]
fn uses_dash_for_non_existing_variables() {
    let run = create_run(&["A"]);
    let timer = Timer::new(run).unwrap();

    let mut component = Component::new();
    component.settings_mut().text = Text::Variable(String::from("Goal Time"), true);

    let state = component.state(&timer);

    assert_eq!(
        state.text,
        TextState::Split(String::from("Goal Time"), String::from(formatter::DASH))
    );
}

#[test]
fn uses_dash_for_empty_string_variable() {
    let mut run = create_run(&["A"]);
    run.metadata_mut().custom_variable_mut("Goal Time").value = String::new();
    let timer = Timer::new(run).unwrap();

    let mut component = Component::new();
    component.settings_mut().text = Text::Variable(String::from("Goal Time"), true);

    let state = component.state(&timer);

    assert_eq!(
        state.text,
        TextState::Split(String::from("Goal Time"), String::from(formatter::DASH))
    );
}

#[test]
fn uses_dash_for_whitespace_variable() {
    let mut run = create_run(&["A"]);
    run.metadata_mut().custom_variable_mut("Goal Time").value = String::from("    ");
    let timer = Timer::new(run).unwrap();

    let mut component = Component::new();
    component.settings_mut().text = Text::Variable(String::from("Goal Time"), true);

    let state = component.state(&timer);

    assert_eq!(
        state.text,
        TextState::Split(String::from("Goal Time"), String::from(formatter::DASH))
    );
}
