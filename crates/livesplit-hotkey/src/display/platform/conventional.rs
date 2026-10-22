//! Shared named-key conventions for Windows, Linux and web/unknown targets.
//!
//! The Meta label is selected here at compile time. Web builds deliberately
//! retain platform-neutral names rather than guessing the browser's host OS.

use crate::{
    KeyCode,
    display::{Localize, Text, platform::Modifier},
};

// Named shortcuts use the same separator as the existing English resolver.
pub(crate) const SEPARATOR: &str = " + ";

pub(crate) fn modifier_name<L: Localize>(modifier: Modifier, lang: L) -> &'static str {
    lang.text(match modifier {
        Modifier::Control => Text::Control,
        Modifier::Alt => Text::Alt,
        Modifier::Shift => Text::Shift,
        Modifier::Meta => {
            cfg_if::cfg_if! {
                if #[cfg(windows)] {
                    return "Win";
                } else if #[cfg(target_os = "linux")] {
                    Text::Super
                } else {
                    Text::Meta
                }
            }
        }
    })
}

pub(crate) const fn key_symbol(_: KeyCode) -> Option<&'static str> {
    None
}
