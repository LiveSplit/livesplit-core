use crate::{
    hotkey::{Hook, KeyCode},
    HotkeyConfig, SharedTimer,
};

pub use crate::hotkey::{Error, Result};

// This enum might be better situated in hotkey_config, but the last method should stay in this file
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Hotkey {
    Split,
    /// The key to use for resetting the current attempt.
    Reset,
    /// The key to use for undoing the last split.
    Undo,
    /// The key to use for skipping the current split.
    Skip,
    /// The key to use for pausing the current attempt and starting a new
    /// attempt.
    Pause,
    /// The key to use for removing all the pause times from the current time.
    UndoAllPauses,
    /// The key to use for switching to the previous comparison.
    PreviousComparison,
    /// The key to use for switching to the next comparison.
    NextComparison,
    /// The key to use for toggling between the `Real Time` and `Game Time`
    /// timing methods.
    ToggleTimingMethod,
}

impl Hotkey {
    fn set_keycode(self, config: &mut HotkeyConfig, keycode: Option<KeyCode>) {
        match self {
            Hotkey::Split => config.split = keycode,
            Hotkey::Reset => config.reset = keycode,
            Hotkey::Undo => config.undo = keycode,
            Hotkey::Skip => config.skip = keycode,
            Hotkey::Pause => config.pause = keycode,
            Hotkey::UndoAllPauses => config.undo_all_pauses = keycode,
            Hotkey::PreviousComparison => config.previous_comparison = keycode,
            Hotkey::NextComparison => config.next_comparison = keycode,
            Hotkey::ToggleTimingMethod => config.toggle_timing_method = keycode,
        }
    }

    const fn get_keycode(self, config: &HotkeyConfig) -> Option<KeyCode> {
        match self {
            Hotkey::Split => config.split,
            Hotkey::Reset => config.reset,
            Hotkey::Undo => config.undo,
            Hotkey::Skip => config.skip,
            Hotkey::Pause => config.pause,
            Hotkey::UndoAllPauses => config.undo_all_pauses,
            Hotkey::PreviousComparison => config.previous_comparison,
            Hotkey::NextComparison => config.next_comparison,
            Hotkey::ToggleTimingMethod => config.toggle_timing_method,
        }
    }

    fn callback(self, timer: SharedTimer) -> Box<dyn FnMut() + Send + 'static> {
        match self {
            Hotkey::Split => Box::new(move || timer.write().unwrap().split_or_start()),
            Hotkey::Reset => Box::new(move || timer.write().unwrap().reset(true)),
            Hotkey::Undo => Box::new(move || timer.write().unwrap().undo_split()),
            Hotkey::Skip => Box::new(move || timer.write().unwrap().skip_split()),
            Hotkey::Pause => Box::new(move || timer.write().unwrap().toggle_pause_or_start()),
            Hotkey::UndoAllPauses => Box::new(move || timer.write().unwrap().undo_all_pauses()),
            Hotkey::PreviousComparison => {
                Box::new(move || timer.write().unwrap().switch_to_previous_comparison())
            }
            Hotkey::NextComparison => {
                Box::new(move || timer.write().unwrap().switch_to_next_comparison())
            }
            Hotkey::ToggleTimingMethod => {
                Box::new(move || timer.write().unwrap().toggle_timing_method())
            }
        }
    }
}

/// With a Hotkey System the runner can use hotkeys on their keyboard to control
/// the Timer. The hotkeys are global, so the application doesn't need to be in
/// focus. The behavior of the hotkeys depends on the platform and is stubbed
/// out on platforms that don't support hotkeys. You can turn off a Hotkey
/// System temporarily. By default the Hotkey System is activated.
pub struct HotkeySystem {
    config: HotkeyConfig,
    hook: Hook,
    timer: SharedTimer,
    is_active: bool,
}

impl HotkeySystem {
    /// Creates a new Hotkey System for a Timer with the default hotkeys.
    pub fn new(timer: SharedTimer) -> Result<Self> {
        Self::with_config(timer, Default::default())
    }

    /// Creates a new Hotkey System for a Timer with a custom configuration for
    /// the hotkeys.
    pub fn with_config(timer: SharedTimer, config: HotkeyConfig) -> Result<Self> {
        let mut hotkey_system = Self {
            config,
            hook: Hook::new()?,
            timer,
            is_active: false,
        };
        hotkey_system.activate()?;
        Ok(hotkey_system)
    }

    // This method should never be public, because it might mess up the internal
    // state and we might leak a registered hotkey
    fn register_inner(&mut self, hotkey: Hotkey) -> Result<()> {
        let inner = self.timer.clone();
        if let Some(keycode) = hotkey.get_keycode(&self.config) {
            self.hook.register(keycode, hotkey.callback(inner))?;
        }
        Ok(())
    }

    fn register(&mut self, hotkey: Hotkey, keycode: Option<KeyCode>) -> Result<()> {
        hotkey.set_keycode(&mut self.config, keycode);
        self.register_inner(hotkey)
    }

    // This method should never be public, because it might mess up the internal
    // state and we might leak a registered hotkey
    fn unregister_inner(&mut self, hotkey: Hotkey) -> Result<()> {
        if let Some(keycode) = hotkey.get_keycode(&self.config) {
            self.hook.unregister(keycode)?;
        }
        Ok(())
    }

    fn unregister(&mut self, hotkey: Hotkey) -> Result<()> {
        self.unregister_inner(hotkey)?;
        hotkey.set_keycode(&mut self.config, None);
        Ok(())
    }

    fn set_hotkey(&mut self, hotkey: Hotkey, keycode: Option<KeyCode>) -> Result<()> {
        // FIXME: We do not check whether the keycode is already in use
        if hotkey.get_keycode(&self.config) == keycode {
            return Ok(());
        }
        if self.is_active {
            self.unregister(hotkey)?;
            self.register(hotkey, keycode)?;
        } else {
            hotkey.set_keycode(&mut self.config, keycode);
        }
        Ok(())
    }

    /// Sets the key to use for splitting and starting a new attempt.
    pub fn set_split(&mut self, hotkey: Option<KeyCode>) -> Result<()> {
        self.set_hotkey(Hotkey::Split, hotkey)
    }

    /// Sets the key to use for resetting the current attempt.
    pub fn set_reset(&mut self, hotkey: Option<KeyCode>) -> Result<()> {
        self.set_hotkey(Hotkey::Reset, hotkey)
    }

    /// Sets the key to use for pausing the current attempt and starting a new
    /// attempt.
    pub fn set_pause(&mut self, hotkey: Option<KeyCode>) -> Result<()> {
        self.set_hotkey(Hotkey::Pause, hotkey)
    }

    /// Sets the key to use for skipping the current split.
    pub fn set_skip(&mut self, hotkey: Option<KeyCode>) -> Result<()> {
        self.set_hotkey(Hotkey::Skip, hotkey)
    }

    /// Sets the key to use for undoing the last split.
    pub fn set_undo(&mut self, hotkey: Option<KeyCode>) -> Result<()> {
        self.set_hotkey(Hotkey::Undo, hotkey)
    }

    /// Sets the key to use for switching to the previous comparison.
    pub fn set_previous_comparison(&mut self, hotkey: Option<KeyCode>) -> Result<()> {
        self.set_hotkey(Hotkey::PreviousComparison, hotkey)
    }

    /// Sets the key to use for switching to the next comparison.
    pub fn set_next_comparison(&mut self, hotkey: Option<KeyCode>) -> Result<()> {
        self.set_hotkey(Hotkey::NextComparison, hotkey)
    }

    /// Sets the key to use for removing all the pause times from the current
    /// time.
    pub fn set_undo_all_pauses(&mut self, hotkey: Option<KeyCode>) -> Result<()> {
        self.set_hotkey(Hotkey::UndoAllPauses, hotkey)
    }

    /// Sets the key to use for toggling between the `Real Time` and `Game Time`
    /// timing methods.
    pub fn set_toggle_timing_method(&mut self, hotkey: Option<KeyCode>) -> Result<()> {
        self.set_hotkey(Hotkey::ToggleTimingMethod, hotkey)
    }

    /// Deactivates the Hotkey System. No hotkeys will go through until it gets
    /// activated again. If it's already deactivated, nothing happens.
    pub fn deactivate(&mut self) -> Result<()> {
        if self.is_active {
            self.unregister_inner(Hotkey::Split)?;
            self.unregister_inner(Hotkey::Reset)?;
            self.unregister_inner(Hotkey::Undo)?;
            self.unregister_inner(Hotkey::Skip)?;
            self.unregister_inner(Hotkey::Pause)?;
            self.unregister_inner(Hotkey::UndoAllPauses)?;
            self.unregister_inner(Hotkey::PreviousComparison)?;
            self.unregister_inner(Hotkey::NextComparison)?;
            self.unregister_inner(Hotkey::ToggleTimingMethod)?;
        }
        self.is_active = false;
        Ok(())
    }

    /// Activates a previously deactivated Hotkey System. If it's already
    /// active, nothing happens.
    pub fn activate(&mut self) -> Result<()> {
        if !self.is_active {
            self.register_inner(Hotkey::Split)?;
            self.register_inner(Hotkey::Reset)?;
            self.register_inner(Hotkey::Undo)?;
            self.register_inner(Hotkey::Skip)?;
            self.register_inner(Hotkey::Pause)?;
            self.register_inner(Hotkey::UndoAllPauses)?;
            self.register_inner(Hotkey::PreviousComparison)?;
            self.register_inner(Hotkey::NextComparison)?;
            self.register_inner(Hotkey::ToggleTimingMethod)?;
        }
        self.is_active = true;
        Ok(())
    }

    /// Returns true if the Hotkey System is active, false otherwise.
    pub const fn is_active(&self) -> bool {
        self.is_active
    }

    /// Returns the hotkey configuration currently in use by the Hotkey System.
    pub const fn config(&self) -> HotkeyConfig {
        self.config
    }

    /// Applies a new hotkey configuration to the Hotkey System. Each hotkey is
    /// changed to the one specified in the configuration. This operation may
    /// fail if you provide a hotkey configuration where a hotkey is used for
    /// multiple operations.
    pub fn set_config(&mut self, config: HotkeyConfig) -> Result<()> {
        self.set_split(config.split)?;
        self.set_reset(config.reset)?;
        self.set_undo(config.undo)?;
        self.set_skip(config.skip)?;
        self.set_pause(config.pause)?;
        self.set_previous_comparison(config.previous_comparison)?;
        self.set_next_comparison(config.next_comparison)?;
        self.set_undo_all_pauses(config.undo_all_pauses)?;
        self.set_toggle_timing_method(config.toggle_timing_method)?;

        Ok(())
    }
}
