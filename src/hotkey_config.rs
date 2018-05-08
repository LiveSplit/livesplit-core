use hotkey::KeyCode;

/// The configuration to use for a Hotkey System. It describes with keys to use
/// as hotkeys for the different actions.
#[derive(Debug, Eq, PartialEq, Hash)]
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

#[cfg(any(target_os = "emscripten", all(target_arch = "wasm32", target_os = "unknown")))]
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

#[cfg(
    not(
        any(
            windows,
            target_os = "linux",
            target_os = "emscripten",
            all(target_arch = "wasm32", target_os = "unknown")
        )
    )
)]
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
