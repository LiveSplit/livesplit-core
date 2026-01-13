use core::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

bitflags::bitflags! {
    /// The modifier keys that are currently pressed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Modifiers: u8 {
        /// The shift key is pressed.
        const SHIFT = 1 << 0;
        /// The control key is pressed.
        const CONTROL = 1 << 1;
        /// The alt key is pressed.
        const ALT = 1 << 2;
        /// The meta key is pressed.
        const META = 1 << 3;
    }
}

impl fmt::Display for Modifiers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        if self.contains(Modifiers::CONTROL) {
            first = false;
            f.write_str("Ctrl")?;
        }
        if self.contains(Modifiers::ALT) {
            if !first {
                f.write_str(" + ")?;
            }
            first = false;
            f.write_str("Alt")?;
        }
        if self.contains(Modifiers::META) {
            if !first {
                f.write_str(" + ")?;
            }
            first = false;
            f.write_str("Meta")?;
        }
        if self.contains(Modifiers::SHIFT) {
            if !first {
                f.write_str(" + ")?;
            }
            f.write_str("Shift")?;
        }
        Ok(())
    }
}

impl FromStr for Modifiers {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut modifiers = Modifiers::empty();
        for modifier in s.split('+').map(str::trim) {
            match modifier {
                "Ctrl" => modifiers.insert(Modifiers::CONTROL),
                "Alt" => modifiers.insert(Modifiers::ALT),
                /// Option as alias for Alt used on MacOS
                "Option" => modifiers.insert(Modifiers::ALT),
                "Meta" => modifiers.insert(Modifiers::META),
                "Shift" => modifiers.insert(Modifiers::SHIFT),
                _ => return Err(()),
            }
        }
        Ok(modifiers)
    }
}

impl Serialize for Modifiers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Modifiers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ModifiersVisitor)
    }
}

struct ModifiersVisitor;

impl serde::de::Visitor<'_> for ModifiersVisitor {
    type Value = Modifiers;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("valid modifiers")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Modifiers::from_str(v).map_err(|()| serde::de::Error::custom("invalid modifiers"))
    }
}
