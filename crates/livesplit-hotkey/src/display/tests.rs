//! Standalone display tests with injected text and layout observations.
//!
//! No native hook is created: these exercise the lower-layer formatting contract.

use crate::{
    KeyCode, Modifiers,
    display::{Binding, Key, Localize, Template, Text},
};
use alloc::{format, string::ToString};
use core::fmt;

#[derive(Clone, Copy)]
pub(crate) struct Labels;

impl Localize for Labels {
    fn text(self, text: Text) -> &'static str {
        match text {
            Text::InternationalBackslash => "international",
            _ => "label",
        }
    }

    fn format(
        self,
        template: Template,
        value: &dyn fmt::Display,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{template:?}({value})")
    }
}

#[test]
fn display_uses_an_injected_vocabulary_without_a_language_registry() {
    let binding = KeyCode::KeyA.with_modifiers(Modifiers::CONTROL | Modifiers::META);
    assert_eq!(
        Binding::new(binding, Labels).to_string(),
        if cfg!(target_vendor = "apple") {
            "⌃⌘A"
        } else if cfg!(windows) {
            "label + Win + A"
        } else {
            "label + label + A"
        }
    );
    assert_eq!(
        Key::new(KeyCode::Numpad1, Labels).to_string(),
        "NumpadKey(1)"
    );
    assert_eq!(
        Key::new(KeyCode::AltRight, Labels).to_string(),
        if cfg!(target_vendor = "apple") {
            "RightKey(⌥)"
        } else {
            "RightKey(label)"
        }
    );
    assert_eq!(binding.to_string(), "Ctrl + Meta + KeyA");
}

#[test]
fn missing_native_observation_is_distinct_from_its_text() {
    let key = Key::new(KeyCode::IntlBackslash, Labels);
    assert_eq!(
        key.clone().with_resolver(|_| None).to_string(),
        "international"
    );
    // Even text identical to a legacy fallback is a valid native observation.
    let native = key.with_resolver(|_| Some("International Backslash".into()));
    assert_eq!(format!("{native}"), "International Backslash");
}
