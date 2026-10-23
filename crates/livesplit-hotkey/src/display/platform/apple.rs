//! Apple menu shortcut glyphs, independent of AppKit and native hook lifetime.
//!
//! These presentation rules never change the identifiers used to register or
//! serialize a binding. Non-Apple builds compile this module only in tests.

use crate::{
    KeyCode,
    display::{Localize, platform::Modifier},
};

// macOS modifier glyphs are adjacent, without a textual plus separator.
pub const SEPARATOR: &str = "";

pub fn modifier_name<L: Localize>(modifier: Modifier, _: L) -> &'static str {
    match modifier {
        Modifier::Control => "⌃",
        Modifier::Alt => "⌥",
        Modifier::Shift => "⇧",
        Modifier::Meta => "⌘",
    }
}

pub const fn key_symbol(code: KeyCode) -> Option<&'static str> {
    // This is an optional glyph override. The ordinary key-name table still
    // matches every KeyCode exhaustively and localizes keys without a glyph.
    Some(match code {
        KeyCode::Backspace => "⌫",
        KeyCode::Delete => "⌦",
        KeyCode::CapsLock => "⇪",
        KeyCode::Enter => "↩",
        KeyCode::NumpadEnter => "⌤",
        KeyCode::Tab => "⇥",
        KeyCode::Escape => "⎋",
        KeyCode::Home => "↖",
        KeyCode::End => "↘",
        KeyCode::PageUp => "⇞",
        KeyCode::PageDown => "⇟",
        _ => return None,
    })
}
