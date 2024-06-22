use alloc::borrow::Cow;

use crate::{
    event,
    hotkey::{ConsumePreference, Hook, Hotkey, KeyCode},
    HotkeyConfig,
};

pub use crate::hotkey::Result;

// This enum might be better situated in hotkey_config, but the last method should stay in this file
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Action {
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

impl Action {
    fn set_hotkey(self, config: &mut HotkeyConfig, hotkey: Option<Hotkey>) {
        match self {
            Action::Split => config.split = hotkey,
            Action::Reset => config.reset = hotkey,
            Action::Undo => config.undo = hotkey,
            Action::Skip => config.skip = hotkey,
            Action::Pause => config.pause = hotkey,
            Action::UndoAllPauses => config.undo_all_pauses = hotkey,
            Action::PreviousComparison => config.previous_comparison = hotkey,
            Action::NextComparison => config.next_comparison = hotkey,
            Action::ToggleTimingMethod => config.toggle_timing_method = hotkey,
        }
    }

    const fn get_hotkey(self, config: &HotkeyConfig) -> Option<Hotkey> {
        match self {
            Action::Split => config.split,
            Action::Reset => config.reset,
            Action::Undo => config.undo,
            Action::Skip => config.skip,
            Action::Pause => config.pause,
            Action::UndoAllPauses => config.undo_all_pauses,
            Action::PreviousComparison => config.previous_comparison,
            Action::NextComparison => config.next_comparison,
            Action::ToggleTimingMethod => config.toggle_timing_method,
        }
    }

    fn callback<S: event::CommandSink + Send + 'static>(
        self,
        command_sink: S,
    ) -> Box<dyn FnMut() + Send + 'static> {
        match self {
            Action::Split => Box::new(move || {
                drop(command_sink.split_or_start());
            }),
            Action::Reset => Box::new(move || {
                drop(command_sink.reset(None));
            }),
            Action::Undo => Box::new(move || {
                drop(command_sink.undo_split());
            }),
            Action::Skip => Box::new(move || {
                drop(command_sink.skip_split());
            }),
            Action::Pause => Box::new(move || {
                drop(command_sink.toggle_pause_or_start());
            }),
            Action::UndoAllPauses => Box::new(move || {
                drop(command_sink.undo_all_pauses());
            }),
            Action::PreviousComparison => Box::new(move || {
                drop(command_sink.switch_to_previous_comparison());
            }),
            Action::NextComparison => Box::new(move || {
                drop(command_sink.switch_to_next_comparison());
            }),
            Action::ToggleTimingMethod => Box::new(move || {
                drop(command_sink.toggle_timing_method());
            }),
        }
    }
}

/// With a `HotkeySystem` the runner can use hotkeys on their keyboard to control
/// the Timer. The hotkeys are global, so the application doesn't need to be in
/// focus. The behavior of the hotkeys depends on the platform and is stubbed
/// out on platforms that don't support hotkeys. You can turn off a `HotkeySystem`
/// temporarily. By default the `HotkeySystem` is activated.
pub struct HotkeySystem<S> {
    config: HotkeyConfig,
    hook: Hook,
    command_sink: S,
    is_active: bool,
}

impl<S: event::CommandSink + Clone + Send + 'static> HotkeySystem<S> {
    /// Creates a new Hotkey System for a Timer with the default hotkeys.
    pub fn new(command_sink: S) -> Result<Self> {
        Self::with_config(command_sink, Default::default())
    }

    /// Creates a new Hotkey System for a Timer with a custom configuration for
    /// the hotkeys.
    pub fn with_config(command_sink: S, config: HotkeyConfig) -> Result<Self> {
        let mut hotkey_system = Self {
            config,
            hook: Hook::with_consume_preference(ConsumePreference::PreferNoConsume)?,
            command_sink,
            is_active: false,
        };
        hotkey_system.activate()?;
        Ok(hotkey_system)
    }

    // This method should never be public, because it might mess up the internal
    // state and we might leak a registered hotkey
    fn register_inner(&mut self, action: Action) -> Result<()> {
        let inner = self.command_sink.clone();
        if let Some(hotkey) = action.get_hotkey(&self.config) {
            self.hook.register(hotkey, action.callback(inner))?;
        }
        Ok(())
    }

    fn register(&mut self, action: Action, hotkey: Option<Hotkey>) -> Result<()> {
        action.set_hotkey(&mut self.config, hotkey);
        self.register_inner(action)
    }

    // This method should never be public, because it might mess up the internal
    // state and we might leak a registered hotkey
    fn unregister_inner(&mut self, action: Action) -> Result<()> {
        if let Some(hotkey) = action.get_hotkey(&self.config) {
            self.hook.unregister(hotkey)?;
        }
        Ok(())
    }

    fn unregister(&mut self, action: Action) -> Result<()> {
        self.unregister_inner(action)?;
        action.set_hotkey(&mut self.config, None);
        Ok(())
    }

    fn set_hotkey(&mut self, action: Action, hotkey: Option<Hotkey>) -> Result<()> {
        // FIXME: We do not check whether the hotkey is already in use
        if action.get_hotkey(&self.config) == hotkey {
            return Ok(());
        }
        if self.is_active {
            self.unregister(action)?;
            self.register(action, hotkey)?;
        } else {
            action.set_hotkey(&mut self.config, hotkey);
        }
        Ok(())
    }

    /// Sets the key to use for splitting and starting a new attempt.
    pub fn set_split(&mut self, hotkey: Option<Hotkey>) -> Result<()> {
        self.set_hotkey(Action::Split, hotkey)
    }

    /// Sets the key to use for resetting the current attempt.
    pub fn set_reset(&mut self, hotkey: Option<Hotkey>) -> Result<()> {
        self.set_hotkey(Action::Reset, hotkey)
    }

    /// Sets the key to use for pausing the current attempt and starting a new
    /// attempt.
    pub fn set_pause(&mut self, hotkey: Option<Hotkey>) -> Result<()> {
        self.set_hotkey(Action::Pause, hotkey)
    }

    /// Sets the key to use for skipping the current split.
    pub fn set_skip(&mut self, hotkey: Option<Hotkey>) -> Result<()> {
        self.set_hotkey(Action::Skip, hotkey)
    }

    /// Sets the key to use for undoing the last split.
    pub fn set_undo(&mut self, hotkey: Option<Hotkey>) -> Result<()> {
        self.set_hotkey(Action::Undo, hotkey)
    }

    /// Sets the key to use for switching to the previous comparison.
    pub fn set_previous_comparison(&mut self, hotkey: Option<Hotkey>) -> Result<()> {
        self.set_hotkey(Action::PreviousComparison, hotkey)
    }

    /// Sets the key to use for switching to the next comparison.
    pub fn set_next_comparison(&mut self, hotkey: Option<Hotkey>) -> Result<()> {
        self.set_hotkey(Action::NextComparison, hotkey)
    }

    /// Sets the key to use for removing all the pause times from the current
    /// time.
    pub fn set_undo_all_pauses(&mut self, hotkey: Option<Hotkey>) -> Result<()> {
        self.set_hotkey(Action::UndoAllPauses, hotkey)
    }

    /// Sets the key to use for toggling between the `Real Time` and `Game Time`
    /// timing methods.
    pub fn set_toggle_timing_method(&mut self, hotkey: Option<Hotkey>) -> Result<()> {
        self.set_hotkey(Action::ToggleTimingMethod, hotkey)
    }

    /// Deactivates the Hotkey System. No hotkeys will go through until it gets
    /// activated again. If it's already deactivated, nothing happens.
    pub fn deactivate(&mut self) -> Result<()> {
        if self.is_active {
            self.unregister_inner(Action::Split)?;
            self.unregister_inner(Action::Reset)?;
            self.unregister_inner(Action::Undo)?;
            self.unregister_inner(Action::Skip)?;
            self.unregister_inner(Action::Pause)?;
            self.unregister_inner(Action::UndoAllPauses)?;
            self.unregister_inner(Action::PreviousComparison)?;
            self.unregister_inner(Action::NextComparison)?;
            self.unregister_inner(Action::ToggleTimingMethod)?;
        }
        self.is_active = false;
        Ok(())
    }

    /// Activates a previously deactivated Hotkey System. If it's already
    /// active, nothing happens.
    pub fn activate(&mut self) -> Result<()> {
        if !self.is_active {
            self.register_inner(Action::Split)?;
            self.register_inner(Action::Reset)?;
            self.register_inner(Action::Undo)?;
            self.register_inner(Action::Skip)?;
            self.register_inner(Action::Pause)?;
            self.register_inner(Action::UndoAllPauses)?;
            self.register_inner(Action::PreviousComparison)?;
            self.register_inner(Action::NextComparison)?;
            self.register_inner(Action::ToggleTimingMethod)?;
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

    /// Resolves the key according to the current keyboard layout.
    pub fn resolve(&self, key_code: KeyCode) -> Cow<'static, str> {
        key_code.resolve(&self.hook)
    }
}
