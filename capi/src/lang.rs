//! Localization bindings.

use crate::{output_str, str};
use livesplit_core::Lang;
use std::os::raw::c_char;

/// Parses a locale string into a language.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Lang_parse_locale(locale: *const c_char) -> Lang {
    // SAFETY: The caller guarantees that `locale` is valid.
    Lang::parse_locale(unsafe { str(locale) })
}

/// Parses a language name into a language.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Lang_from_name(name: *const c_char) -> Lang {
    // SAFETY: The caller guarantees that `name` is valid.
    Lang::from_name(unsafe { str(name) })
}

/// Returns the localized display name for a language.
#[unsafe(no_mangle)]
pub extern "C" fn Lang_name(lang: Lang) -> *const c_char {
    output_str(lang.name())
}
