//! Compile-time keyboard display policy for native and web targets.
//!
//! Native builds compile their own naming rules; web/unknown targets keep
//! platform-neutral names. No OS probing, mutable cache or caller-supplied
//! platform is needed. Alternate policies are compiled only in unit tests.

use crate::Modifiers;

#[cfg(any(target_vendor = "apple", test))]
mod apple;
#[cfg(any(not(target_vendor = "apple"), test))]
mod conventional;

cfg_select! {
    target_vendor = "apple" => {
        pub use apple::{SEPARATOR, key_symbol, modifier_name};
    }
    _ => {
        pub use conventional::{SEPARATOR, key_symbol, modifier_name};
    }
}

#[derive(Clone, Copy)]
pub enum Modifier {
    Control,
    Alt,
    Shift,
    Meta,
}

impl Modifier {
    pub const fn flag(self) -> Modifiers {
        match self {
            Self::Control => Modifiers::CONTROL,
            Self::Alt => Modifiers::ALT,
            Self::Shift => Modifiers::SHIFT,
            Self::Meta => Modifiers::META,
        }
    }
}

// Display order is independent of the historical Control/Alt/Meta/Shift wire
// format. This follows the standard Control/Option/Shift/Command order on macOS.
// https://developer.apple.com/design/human-interface-guidelines/keyboards
pub const ORDER: [Modifier; 4] = [
    Modifier::Control,
    Modifier::Alt,
    Modifier::Shift,
    Modifier::Meta,
];

#[cfg(test)]
mod tests {
    use crate::{
        KeyCode,
        display::{
            platform::{Modifier, apple, conventional},
            tests::Labels,
        },
    };

    #[test]
    fn apple_glyphs_and_separators_follow_native_shortcut_conventions() {
        assert_eq!(apple::SEPARATOR, "");
        for (modifier, expected) in [
            (Modifier::Control, "⌃"),
            (Modifier::Alt, "⌥"),
            (Modifier::Shift, "⇧"),
            (Modifier::Meta, "⌘"),
        ] {
            assert_eq!(apple::modifier_name(modifier, Labels), expected);
        }
        for (key, expected) in [
            (KeyCode::Backspace, "⌫"),
            (KeyCode::Delete, "⌦"),
            (KeyCode::CapsLock, "⇪"),
            (KeyCode::Tab, "⇥"),
            (KeyCode::Escape, "⎋"),
            (KeyCode::Home, "↖"),
            (KeyCode::End, "↘"),
            (KeyCode::PageDown, "⇟"),
            (KeyCode::Enter, "↩"),
            (KeyCode::NumpadEnter, "⌤"),
            (KeyCode::PageUp, "⇞"),
        ] {
            assert_eq!(apple::key_symbol(key), Some(expected));
        }
        assert_eq!(apple::key_symbol(KeyCode::KeyA), None);
    }

    #[test]
    fn conventional_policy_keeps_localized_names_instead_of_apple_glyphs() {
        assert_eq!(conventional::SEPARATOR, " + ");
        assert_eq!(
            conventional::modifier_name(Modifier::Control, Labels),
            "label"
        );
        assert_eq!(conventional::key_symbol(KeyCode::Backspace), None);
    }
}
