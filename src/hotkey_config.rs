use hotkey::KeyCode;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct HotkeyConfig {
    pub split: KeyCode,
    pub reset: KeyCode,
    pub undo: KeyCode,
    pub skip: KeyCode,
    pub pause: KeyCode,
    pub previous_comparison: KeyCode,
    pub next_comparison: KeyCode,
}

#[cfg(windows)]
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

#[cfg(not(any(windows)))]
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
