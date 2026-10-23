//! Localized, platform-specific presentation of keyboard keys and hotkeys.
//!
//! These display wrappers do not implement serialization or parsing. Persist
//! [`Hotkey`], [`KeyCode`] and [`ModifierFlags`] instead: their stable identifiers
//! are deliberately independent of these labels. Native keyboard-layout lookup
//! is injected during construction, never performed while formatting a label.

mod key;
mod localization;
mod platform;

#[cfg(test)]
mod tests;

pub use localization::{Localize, Template, Text};

use crate::{Hook, Hotkey, KeyCode, KeyCodeClass, Modifiers as ModifierFlags};
use alloc::borrow::Cow;
use core::fmt;

/// A key's display name, with an optional already-resolved writing-system name.
/// The fallback uses the standard US layout for character keys and the supplied
/// language and native conventions for special keys. It also works without
/// native hook permissions.
#[derive(Clone)]
pub struct Key<L> {
    code: KeyCode,
    lang: L,
    writing_key: Option<Cow<'static, str>>,
}

impl<L: Localize> Key<L> {
    /// Prepare a display name without querying a native keyboard layout.
    pub const fn new(code: KeyCode, lang: L) -> Self {
        Self {
            code,
            lang,
            writing_key: None,
        }
    }

    /// Resolve a writing-system key once. Special keys, including the numeric
    /// keypad, use localized labels and do not invoke the callback. `None` or an
    /// empty observation selects the localized fallback. Supplied text is kept
    /// unchanged; the resolver owns layout-dependent case handling.
    pub fn with_resolver(
        mut self,
        resolve: impl FnOnce(KeyCode) -> Option<Cow<'static, str>>,
    ) -> Self {
        if self.code.classify() == KeyCodeClass::WritingSystem {
            self.writing_key = resolve(self.code).filter(|text| !text.is_empty());
        }

        self
    }

    /// Resolve through a native hook before formatting. The resulting label
    /// owns its observation and does not borrow the hook or register a hotkey.
    pub fn with_hook(self, hook: &Hook) -> Self {
        self.with_resolver(|code| code.try_resolve(hook).map(Cow::Owned))
    }
}

impl<L: Localize> fmt::Display for Key<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(key) = &self.writing_key {
            f.write_str(key)
        } else {
            key::format(self.code, self.lang, f)
        }
    }
}

/// A set of modifiers shown using the native platform and supplied language.
/// This is display text, not the canonical modifier string used in saved data.
#[derive(Clone, Copy)]
pub struct Modifiers<L> {
    flags: ModifierFlags,
    lang: L,
}

impl<L: Localize> Modifiers<L> {
    /// Prepare modifier labels without any native lookup or allocation.
    pub const fn new(flags: ModifierFlags, lang: L) -> Self {
        Self { flags, lang }
    }
}

impl<L: Localize> fmt::Display for Modifiers<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;

        for modifier in platform::ORDER {
            if self.flags.contains(modifier.flag()) {
                if !first {
                    f.write_str(platform::SEPARATOR)?;
                }

                f.write_str(platform::modifier_name(modifier, self.lang))?;
                first = false;
            }
        }

        Ok(())
    }
}

/// A complete binding's display text, separate from its persistent identifier.
///
/// The caller supplies translations through [`Localize`]; there is no language
/// registry in this crate. UI libraries can implement that trait for their own
/// existing language type without coupling hotkey handling to application text.
/// Display values deliberately implement neither serialization nor parsing.
#[derive(Clone)]
pub struct Binding<L> {
    key: Key<L>,
    modifiers: Modifiers<L>,
}

impl<L: Localize> Binding<L> {
    /// Prepare a binding with localized fallback key names and no native lookup.
    pub const fn new(hotkey: Hotkey, lang: L) -> Self {
        Self {
            key: Key::new(hotkey.key_code, lang),
            modifiers: Modifiers::new(hotkey.modifiers, lang),
        }
    }

    /// Resolve only the binding's writing-system key, once during construction.
    /// Formatting afterward is independent of hook lifetime and keyboard access.
    pub fn with_resolver(
        mut self,
        resolve: impl FnOnce(KeyCode) -> Option<Cow<'static, str>>,
    ) -> Self {
        self.key = self.key.with_resolver(resolve);
        self
    }

    /// Resolve the binding's writing-system key through a native hook once.
    /// Formatting afterward does not borrow or query the hook.
    pub fn with_hook(mut self, hook: &Hook) -> Self {
        self.key = self.key.with_hook(hook);
        self
    }
}

impl<L: Localize> fmt::Display for Binding<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.modifiers.flags.is_empty() {
            fmt::Display::fmt(&self.modifiers, f)?;
            f.write_str(platform::SEPARATOR)?;
        }

        fmt::Display::fmt(&self.key, f)
    }
}
