use core::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{KeyCode, Modifiers};

/// A hotkey is a combination of a key code and a set of modifiers.
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Hotkey {
    /// The key code of the hotkey.
    pub key_code: KeyCode,
    /// The modifiers of the hotkey.
    pub modifiers: Modifiers,
}

impl fmt::Debug for Hotkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Hotkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.modifiers.is_empty() {
            f.write_str(self.key_code.name())
        } else {
            write!(f, "{} + {}", self.modifiers, self.key_code.name())
        }
    }
}

impl FromStr for Hotkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((modifiers, key_code)) = s.rsplit_once('+') {
            let modifiers = modifiers.trim_end().parse()?;
            let key_code = key_code.trim_start().parse()?;
            Ok(Self {
                key_code,
                modifiers,
            })
        } else {
            let key_code = s.parse()?;
            Ok(Self {
                key_code,
                modifiers: Modifiers::empty(),
            })
        }
    }
}

impl Serialize for Hotkey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.modifiers.is_empty() {
            self.key_code.serialize(serializer)
        } else {
            serializer.collect_str(self)
        }
    }
}

impl<'de> Deserialize<'de> for Hotkey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(HotkeyVisitor)
    }
}

struct HotkeyVisitor;

impl serde::de::Visitor<'_> for HotkeyVisitor {
    type Value = Hotkey;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a valid hotkey")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Hotkey::from_str(v).map_err(|()| serde::de::Error::custom("invalid hotkey"))
    }
}

impl From<KeyCode> for Hotkey {
    fn from(key_code: KeyCode) -> Self {
        Self {
            key_code,
            modifiers: Modifiers::empty(),
        }
    }
}
