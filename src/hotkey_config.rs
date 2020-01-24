#![allow(clippy::trivially_copy_pass_by_ref)]

use crate::hotkey::KeyCode;
use crate::platform::prelude::*;
use crate::settings::{Field, SettingsDescription, Value};
use serde::{Deserialize, Serialize};

/// The configuration to use for a Hotkey System. It describes with keys to use
/// as hotkeys for the different actions.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(default)]
pub struct HotkeyConfig {
    /// The key to use for splitting and starting a new attempt.
    pub split: Option<KeyCode>,
    /// The key to use for resetting the current attempt.
    pub reset: Option<KeyCode>,
    /// The key to use for undoing the last split.
    pub undo: Option<KeyCode>,
    /// The key to use for skipping the current split.
    pub skip: Option<KeyCode>,
    /// The key to use for pausing the current attempt and starting a new
    /// attempt.
    pub pause: Option<KeyCode>,
    /// The key to use for removing all the pause times from the current time.
    pub undo_all_pauses: Option<KeyCode>,
    /// The key to use for switching to the previous comparison.
    pub previous_comparison: Option<KeyCode>,
    /// The key to use for switching to the next comparison.
    pub next_comparison: Option<KeyCode>,
    /// The key to use for toggling between the `Real Time` and `Game Time`
    /// timing methods.
    pub toggle_timing_method: Option<KeyCode>,
}

#[cfg(any(windows, target_os = "linux"))]
impl Default for HotkeyConfig {
    fn default() -> Self {
        use crate::hotkey::KeyCode::*;
        Self {
            split: Some(NumPad1),
            reset: Some(NumPad3),
            undo: Some(NumPad8),
            skip: Some(NumPad2),
            pause: Some(NumPad5),
            undo_all_pauses: None,
            previous_comparison: Some(NumPad4),
            next_comparison: Some(NumPad6),
            toggle_timing_method: None,
        }
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl Default for HotkeyConfig {
    fn default() -> Self {
        use crate::hotkey::KeyCode::*;
        Self {
            split: Some(Numpad1),
            reset: Some(Numpad3),
            undo: Some(Numpad8),
            skip: Some(Numpad2),
            pause: Some(Numpad5),
            undo_all_pauses: None,
            previous_comparison: Some(Numpad4),
            next_comparison: Some(Numpad6),
            toggle_timing_method: None,
        }
    }
}

#[cfg(not(any(
    windows,
    target_os = "linux",
    all(target_arch = "wasm32", target_os = "unknown"),
)))]
impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            split: Some(KeyCode),
            reset: Some(KeyCode),
            undo: Some(KeyCode),
            skip: Some(KeyCode),
            pause: Some(KeyCode),
            undo_all_pauses: None,
            previous_comparison: Some(KeyCode),
            next_comparison: Some(KeyCode),
            toggle_timing_method: None,
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
            Field::new("Undo All Pauses".into(), self.undo_all_pauses.into()),
            Field::new(
                "Previous Comparison".into(),
                self.previous_comparison.into(),
            ),
            Field::new("Next Comparison".into(), self.next_comparison.into()),
            Field::new(
                "Toggle Timing Method".into(),
                self.toggle_timing_method.into(),
            ),
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
        let value: Option<KeyCode> = value.into();

        if value.is_some() {
            let any = [
                self.split,
                self.reset,
                self.undo,
                self.skip,
                self.pause,
                self.previous_comparison,
                self.next_comparison,
                self.undo_all_pauses,
            ]
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != index)
            .any(|(_, &v)| v == value);

            if any {
                return Err(());
            }
        }

        match index {
            0 => self.split = value,
            1 => self.reset = value,
            2 => self.undo = value,
            3 => self.skip = value,
            4 => self.pause = value,
            5 => self.undo_all_pauses = value,
            6 => self.previous_comparison = value,
            7 => self.next_comparison = value,
            8 => self.toggle_timing_method = value,
            _ => panic!("Unsupported Setting Index"),
        }

        Ok(())
    }

    /// Decodes the hotkey configuration from JSON.
    #[cfg(feature = "std")]
    pub fn from_json<R>(reader: R) -> serde_json::Result<Self>
    where
        R: std::io::Read,
    {
        serde_json::from_reader(reader)
    }

    /// Encodes the hotkey configuration as JSON.
    #[cfg(feature = "std")]
    pub fn write_json<W>(&self, writer: W) -> serde_json::Result<()>
    where
        W: std::io::Write,
    {
        serde_json::to_writer(writer, self)
    }
}
