//! Display-policy, native-resolution and persistence compatibility regressions.
//!
//! Native layout observations are injected, so every platform/language is tested
//! without registering hooks, opening windows or requiring input permissions.

mod identifiers;

use crate::{
    Lang,
    hotkey::{
        Hotkey, KeyCode, Modifiers as ModifierFlags,
        display::{Binding, Key, Modifiers},
    },
    localization::tests::identifiers::KEY_IDENTIFIERS,
};
use alloc::{format, string::ToString};
use core::{
    cell::Cell,
    fmt::{self, Write},
};

// These are the historical wire-format strings, independently of the new
// formatter and in the original modifier-bit order (Shift, Control, Alt, Meta).
const CANONICAL_MODIFIERS: [&str; 16] = [
    "",
    "Shift",
    "Ctrl",
    "Ctrl + Shift",
    "Alt",
    "Alt + Shift",
    "Ctrl + Alt",
    "Ctrl + Alt + Shift",
    "Meta",
    "Meta + Shift",
    "Ctrl + Meta",
    "Ctrl + Meta + Shift",
    "Alt + Meta",
    "Alt + Meta + Shift",
    "Ctrl + Alt + Meta",
    "Ctrl + Alt + Meta + Shift",
];

const LANGUAGES: &[Lang] = &[
    Lang::English,
    #[cfg(feature = "localization")]
    Lang::German,
    #[cfg(feature = "localization")]
    Lang::Dutch,
    #[cfg(feature = "localization")]
    Lang::French,
    #[cfg(feature = "localization")]
    Lang::Italian,
    #[cfg(feature = "localization")]
    Lang::Spanish,
    #[cfg(feature = "localization")]
    Lang::Portuguese,
    #[cfg(feature = "localization")]
    Lang::BrazilianPortuguese,
    #[cfg(feature = "localization")]
    Lang::Polish,
    #[cfg(feature = "localization")]
    Lang::Russian,
    #[cfg(feature = "localization")]
    Lang::ChineseSimplified,
    #[cfg(feature = "localization")]
    Lang::ChineseTraditional,
    #[cfg(feature = "localization")]
    Lang::Japanese,
    #[cfg(feature = "localization")]
    Lang::Korean,
];

#[test]
fn every_key_and_modifier_combination_keeps_the_exact_persistent_identifier() {
    for &(code, identifier) in KEY_IDENTIFIERS {
        assert_eq!(code.name(), identifier);
        assert_eq!(
            serde_json::to_string(&code).unwrap(),
            format!("\"{identifier}\"")
        );

        for (bits, prefix) in CANONICAL_MODIFIERS.into_iter().enumerate() {
            let modifiers = ModifierFlags::from_bits(bits as u8).unwrap();
            let binding = code.with_modifiers(modifiers);
            let expected = if prefix.is_empty() {
                identifier.to_string()
            } else {
                format!("{prefix} + {identifier}")
            };
            assert_eq!(modifiers.to_string(), prefix);
            assert_eq!(
                serde_json::to_string(&modifiers).unwrap(),
                format!("\"{prefix}\"")
            );
            assert_eq!(binding.to_string(), expected);
            let serialized = serde_json::to_string(&binding).unwrap();
            assert_eq!(serialized, format!("\"{expected}\""));
            assert_eq!(
                serde_json::from_str::<Hotkey>(&serialized).unwrap(),
                binding
            );
            assert_eq!(expected.parse::<Hotkey>().unwrap(), binding);
        }
    }
}

#[test]
fn every_language_resolves_every_key_without_a_hook() {
    for &lang in LANGUAGES {
        for &(code, _) in KEY_IDENTIFIERS {
            let binding = code.with_modifiers(ModifierFlags::all());
            let before = serde_json::to_string(&binding).unwrap();
            assert!(!Key::new(code, lang).to_string().is_empty());
            assert!(!Binding::new(binding, lang).to_string().is_empty());
            assert_eq!(serde_json::to_string(&binding).unwrap(), before);
        }
    }
}

#[test]
fn display_uses_native_target_conventions() {
    let binding = KeyCode::KeyA.with_modifiers(ModifierFlags::all());
    let expected = if cfg!(target_vendor = "apple") {
        "⌃⌥⇧⌘A"
    } else if cfg!(windows) {
        "Ctrl + Alt + Shift + Win + A"
    } else if cfg!(target_os = "linux") {
        "Ctrl + Alt + Shift + Super + A"
    } else {
        "Ctrl + Alt + Shift + Meta + A"
    };
    assert_eq!(Binding::new(binding, Lang::English).to_string(), expected);
    assert_eq!(
        Key::new(KeyCode::Backspace, Lang::English).to_string(),
        if cfg!(target_vendor = "apple") {
            "⌫"
        } else {
            "Backspace"
        }
    );
    assert_eq!(
        Key::new(KeyCode::NumpadEnter, Lang::English).to_string(),
        if cfg!(target_vendor = "apple") {
            "⌤"
        } else {
            "Numpad Enter"
        }
    );
}

#[cfg(feature = "localization")]
#[test]
fn keypad_and_named_keys_use_the_selected_language() {
    for (lang, expected) in [
        (Lang::English, "Numpad 1"),
        (Lang::German, "Num 1"),
        (Lang::Dutch, "Num 1"),
        (Lang::French, "Pavé num. 1"),
        (Lang::Italian, "Tast. num. 1"),
        (Lang::Spanish, "Teclado num. 1"),
        (Lang::Portuguese, "Teclado num. 1"),
        (Lang::BrazilianPortuguese, "Teclado num. 1"),
        (Lang::Polish, "Num 1"),
        (Lang::Russian, "Цифр. 1"),
        (Lang::Japanese, "テンキー 1"),
        (Lang::Korean, "숫자 키패드 1"),
        (Lang::ChineseSimplified, "小键盘 1"),
        (Lang::ChineseTraditional, "數字鍵盤 1"),
    ] {
        assert_eq!(Key::new(KeyCode::Numpad1, lang).to_string(), expected);
    }

    assert_eq!(
        Modifiers::new(ModifierFlags::CONTROL, Lang::German).to_string(),
        if cfg!(target_vendor = "apple") {
            "⌃"
        } else {
            "Strg"
        }
    );
    assert_eq!(
        Key::new(KeyCode::ControlLeft, Lang::German).to_string(),
        if cfg!(target_vendor = "apple") {
            "⌃ links"
        } else {
            "Strg links"
        }
    );
    assert_eq!(
        Key::new(KeyCode::PageDown, Lang::French).to_string(),
        if cfg!(target_vendor = "apple") {
            "⇟"
        } else {
            "Page suivante"
        }
    );
    assert_eq!(
        Key::new(KeyCode::AudioVolumeMute, Lang::Korean).to_string(),
        "음소거"
    );
    assert_eq!(
        Key::new(KeyCode::Gamepad19, Lang::Japanese).to_string(),
        "ゲームパッド 19"
    );
    assert_eq!(
        Key::new(KeyCode::NumpadClearEntry, Lang::German).to_string(),
        "Num Eingabe löschen"
    );
}

#[test]
fn layout_lookup_is_eager_once_and_used_only_for_writing_system_keys() {
    let calls = Cell::new(0);
    let binding = Binding::new(
        KeyCode::KeyA.with_modifiers(ModifierFlags::META),
        Lang::English,
    )
    .with_resolver(|code| {
        assert_eq!(code, KeyCode::KeyA);
        calls.set(calls.get() + 1);
        Some("Й".into())
    });
    assert_eq!(calls.get(), 1);
    let expected = if cfg!(target_vendor = "apple") {
        "⌘Й"
    } else if cfg!(windows) {
        "Win + Й"
    } else if cfg!(target_os = "linux") {
        "Super + Й"
    } else {
        "Meta + Й"
    };
    assert_eq!(binding.to_string(), expected);
    assert_eq!(binding.to_string(), expected);
    assert_eq!(calls.get(), 1);

    for code in [
        KeyCode::Numpad1,
        KeyCode::MetaLeft,
        KeyCode::AudioVolumeMute,
        KeyCode::F1,
    ] {
        let label = Key::new(code, Lang::English)
            .with_resolver(|_| panic!("special keys must not query the native layout"));
        assert!(!label.to_string().is_empty());
    }
    assert_eq!(
        Key::new(KeyCode::KeyA, Lang::English)
            .with_resolver(|_| Some("".into()))
            .to_string(),
        "A"
    );
    assert_eq!(
        Key::new(KeyCode::KeyA, Lang::English)
            .with_resolver(|_| Some("ß".into()))
            .to_string(),
        "ß"
    );
}

#[cfg(feature = "localization")]
#[test]
fn unresolved_international_key_fallback_is_localized_without_touching_layout_text() {
    let key = KeyCode::IntlBackslash;
    assert_eq!(
        Key::new(key, Lang::German)
            .with_resolver(|_| None)
            .to_string(),
        "Internationale Backslash-Taste"
    );
    assert_eq!(
        Key::new(key, Lang::German)
            .with_resolver(|_| Some("<".into()))
            .to_string(),
        "<"
    );
}

#[test]
fn formatting_propagates_writer_errors_and_unmodified_labels_have_no_prefix() {
    struct FailingWriter;
    impl Write for FailingWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let binding = Binding::new(KeyCode::Numpad1.into(), Lang::English);
    assert_eq!(binding.to_string(), "Numpad 1");
    assert_eq!(
        Modifiers::new(ModifierFlags::empty(), Lang::English).to_string(),
        ""
    );
    assert!(write!(&mut FailingWriter, "{binding}").is_err());
}

#[cfg(feature = "std")]
#[test]
fn hotkey_configuration_keeps_its_existing_json_schema() {
    let config = crate::HotkeyConfig::default();
    assert_eq!(
        serde_json::to_string(&config).unwrap(),
        r#"{"split":"Numpad1","reset":"Numpad3","undo":"Numpad8","skip":"Numpad2","pause":"Numpad5","undo_all_pauses":null,"previous_comparison":"Numpad4","next_comparison":"Numpad6","toggle_timing_method":null}"#
    );
}
