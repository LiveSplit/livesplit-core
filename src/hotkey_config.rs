use hotkey::KeyCode;
use serde_json::{self, from_reader, to_writer};
use settings::{Field, SettingsDescription, Value};
use std::io::{Read, Write};

/// The configuration to use for a Hotkey System. It describes with keys to use
/// as hotkeys for the different actions.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(default)]
pub struct HotkeyConfig {
    /// The key to use for splitting and starting a new attempt.
    pub split: KeyCode,
    /// The key to use for resetting the current attempt.
    pub reset: KeyCode,
    /// The key to use for undoing the last split.
    pub undo: KeyCode,
    /// The key to use for skipping the current split.
    pub skip: KeyCode,
    /// The key to use for pausing the current attempt and starting a new
    /// attempt.
    pub pause: KeyCode,
    /// The key to use for switching to the previous comparison.
    pub previous_comparison: KeyCode,
    /// The key to use for switching to the next comparison.
    pub next_comparison: KeyCode,
}

#[cfg(any(windows, target_os = "linux"))]
impl Default for HotkeyConfig {
    fn default() -> Self {
        use hotkey::KeyCode::*;
        Self {
            split: NumPad1,
            reset: NumPad3,
            undo: NumPad8,
            skip: NumPad2,
            pause: NumPad5,
            previous_comparison: NumPad4,
            next_comparison: NumPad6,
        }
    }
}

#[cfg(any(
    target_os = "emscripten",
    all(target_arch = "wasm32", target_os = "unknown")
))]
impl Default for HotkeyConfig {
    fn default() -> Self {
        use hotkey::KeyCode::*;
        Self {
            split: Numpad1,
            reset: Numpad3,
            undo: Numpad8,
            skip: Numpad2,
            pause: Numpad5,
            previous_comparison: Numpad4,
            next_comparison: Numpad6,
        }
    }
}

#[cfg(not(any(
    windows,
    target_os = "linux",
    target_os = "emscripten",
    all(target_arch = "wasm32", target_os = "unknown")
)))]
impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            split: KeyCode,
            reset: KeyCode,
            undo: KeyCode,
            skip: KeyCode,
            pause: KeyCode,
            previous_comparison: KeyCode,
            next_comparison: KeyCode,
        }
    }
}

impl HotkeyConfig {
    /// Accesses a generic description of the settings available for the hotkey
    /// configuration and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new("Start / Split".into(), self.split.into()),
            Field::new("Reset".into(), self.reset.into()),
            Field::new("Undo Split".into(), self.undo.into()),
            Field::new("Skip Split".into(), self.skip.into()),
            Field::new("Pause".into(), self.pause.into()),
            Field::new(
                "Previous Comparison".into(),
                self.previous_comparison.into(),
            ),
            Field::new("Next Comparison".into(), self.next_comparison.into()),
        ])
    }

    /// Sets a setting's value by its index to the given value.
    ///
    /// # Errors
    ///
    /// An error is returned if a hotkey is already in use by a different
    /// action.
    ///
    /// # Panics
    ///
    /// This panics if the type of the value to be set is not compatible with
    /// the type of the setting's value. A panic can also occur if the index of
    /// the setting provided is out of bounds.
    pub fn set_value(&mut self, index: usize, value: Value) -> Result<(), ()> {
        let value: KeyCode = value.into();

        let any = [
            self.split,
            self.reset,
            self.undo,
            self.skip,
            self.pause,
            self.previous_comparison,
            self.next_comparison,
        ]
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != index)
            .any(|(_, &v)| v == value);

        if any {
            return Err(());
        }

        match index {
            0 => self.split = value,
            1 => self.reset = value,
            2 => self.undo = value,
            3 => self.skip = value,
            4 => self.pause = value,
            5 => self.previous_comparison = value,
            6 => self.next_comparison = value,
            _ => panic!("Unsupported Setting Index"),
        }

        Ok(())
    }

    /// Decodes the hotkey configuration from JSON.
    pub fn from_json<R>(reader: R) -> serde_json::Result<Self>
    where
        R: Read,
    {
        from_reader(reader)
    }

    /// Encodes the hotkey configuration as JSON.
    pub fn write_json<W>(&self, writer: W) -> serde_json::Result<()>
    where
        W: Write,
    {
        to_writer(writer, self)
    }
}
