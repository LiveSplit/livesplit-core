use crate::hotkey::{Hook, KeyCode};
use crate::{HotkeyConfig, SharedTimer};
use alloc::sync::Arc;
use core::sync::atomic::{AtomicBool, Ordering};

pub use crate::hotkey::{Error, Result};

/// With a Hotkey System the runner can use hotkeys on their keyboard to control
/// the Timer. The hotkeys are global, so the application doesn't need to be in
/// focus. The behavior of the hotkeys depends on the platform and is stubbed
/// out on platforms that don't support hotkeys. You can turn off a Hotkey
/// System temporarily. By default the Hotkey System is activated.
pub struct HotkeySystem {
    config: HotkeyConfig,
    hook: Hook,
    timer: SharedTimer,
    is_active: Arc<AtomicBool>,
}

impl HotkeySystem {
    /// Creates a new Hotkey System for a Timer with the default hotkeys.
    pub fn new(timer: SharedTimer) -> Result<Self> {
        Self::with_config(timer, Default::default())
    }

    /// Creates a new Hotkey System for a Timer with a custom configuration for
    /// the hotkeys.
    pub fn with_config(timer: SharedTimer, config: HotkeyConfig) -> Result<Self> {
        let hook = Hook::new()?;

        let is_active = Arc::new(AtomicBool::new(true));

        if let Some(split) = config.split {
            let inner = timer.clone();
            let active = is_active.clone();
            hook.register(split, move || {
                if active.load(Ordering::Acquire) {
                    inner.write().split_or_start();
                }
            })?;
        }

        if let Some(reset) = config.reset {
            let inner = timer.clone();
            let active = is_active.clone();
            hook.register(reset, move || {
                if active.load(Ordering::Acquire) {
                    inner.write().reset(true);
                }
            })?;
        }

        if let Some(undo) = config.undo {
            let inner = timer.clone();
            let active = is_active.clone();
            hook.register(undo, move || {
                if active.load(Ordering::Acquire) {
                    inner.write().undo_split();
                }
            })?;
        }

        if let Some(skip) = config.skip {
            let inner = timer.clone();
            let active = is_active.clone();
            hook.register(skip, move || {
                if active.load(Ordering::Acquire) {
                    inner.write().skip_split();
                }
            })?;
        }

        if let Some(pause) = config.pause {
            let inner = timer.clone();
            let active = is_active.clone();
            hook.register(pause, move || {
                if active.load(Ordering::Acquire) {
                    inner.write().toggle_pause_or_start();
                }
            })?;
        }

        if let Some(previous_comparison) = config.previous_comparison {
            let inner = timer.clone();
            let active = is_active.clone();
            hook.register(previous_comparison, move || {
                if active.load(Ordering::Acquire) {
                    inner.write().switch_to_previous_comparison();
                }
            })?;
        }

        if let Some(next_comparison) = config.next_comparison {
            let inner = timer.clone();
            let active = is_active.clone();
            hook.register(next_comparison, move || {
                if active.load(Ordering::Acquire) {
                    inner.write().switch_to_next_comparison();
                }
            })?;
        }

        if let Some(undo_all_pauses) = config.undo_all_pauses {
            let inner = timer.clone();
            let active = is_active.clone();
            hook.register(undo_all_pauses, move || {
                if active.load(Ordering::Acquire) {
                    inner.write().undo_all_pauses();
                }
            })?;
        }

        if let Some(toggle_timing_method) = config.toggle_timing_method {
            let inner = timer.clone();
            let active = is_active.clone();
            hook.register(toggle_timing_method, move || {
                if active.load(Ordering::Acquire) {
                    inner.write().toggle_timing_method();
                }
            })?;
        }

        Ok(Self {
            config,
            hook,
            timer,
            is_active,
        })
    }

    /// Sets the key to use for splitting and starting a new attempt.
    pub fn set_split(&mut self, hotkey: Option<KeyCode>) -> Result<()> {
        if self.config.split == hotkey {
            return Ok(());
        }
        let inner = self.timer.clone();
        let active = self.is_active.clone();
        if let Some(hotkey) = hotkey {
            self.hook.register(hotkey, move || {
                if active.load(Ordering::Acquire) {
                    inner.write().split_or_start();
                }
            })?;
        }
        if let Some(split) = self.config.split {
            self.hook.unregister(split)?;
        }
        self.config.split = hotkey;
        Ok(())
    }

    /// Sets the key to use for resetting the current attempt.
    pub fn set_reset(&mut self, hotkey: Option<KeyCode>) -> Result<()> {
        if self.config.reset == hotkey {
            return Ok(());
        }
        let inner = self.timer.clone();
        let active = self.is_active.clone();
        if let Some(hotkey) = hotkey {
            self.hook.register(hotkey, move || {
                if active.load(Ordering::Acquire) {
                    inner.write().reset(true);
                }
            })?;
        }
        if let Some(reset) = self.config.reset {
            self.hook.unregister(reset)?;
        }
        self.config.reset = hotkey;
        Ok(())
    }

    /// Sets the key to use for pausing the current attempt and starting a new
    /// attempt.
    pub fn set_pause(&mut self, hotkey: Option<KeyCode>) -> Result<()> {
        if self.config.pause == hotkey {
            return Ok(());
        }
        let inner = self.timer.clone();
        let active = self.is_active.clone();
        if let Some(hotkey) = hotkey {
            self.hook.register(hotkey, move || {
                if active.load(Ordering::Acquire) {
                    inner.write().toggle_pause_or_start();
                }
            })?;
        }
        if let Some(pause) = self.config.pause {
            self.hook.unregister(pause)?;
        }
        self.config.pause = hotkey;
        Ok(())
    }

    /// Sets the key to use for skipping the current split.
    pub fn set_skip(&mut self, hotkey: Option<KeyCode>) -> Result<()> {
        if self.config.skip == hotkey {
            return Ok(());
        }
        let inner = self.timer.clone();
        let active = self.is_active.clone();
        if let Some(hotkey) = hotkey {
            self.hook.register(hotkey, move || {
                if active.load(Ordering::Acquire) {
                    inner.write().skip_split();
                }
            })?;
        }
        if let Some(skip) = self.config.skip {
            self.hook.unregister(skip)?;
        }
        self.config.skip = hotkey;
        Ok(())
    }

    /// Sets the key to use for undoing the last split.
    pub fn set_undo(&mut self, hotkey: Option<KeyCode>) -> Result<()> {
        if self.config.undo == hotkey {
            return Ok(());
        }
        let inner = self.timer.clone();
        let active = self.is_active.clone();
        if let Some(hotkey) = hotkey {
            self.hook.register(hotkey, move || {
                if active.load(Ordering::Acquire) {
                    inner.write().undo_split();
                }
            })?;
        }
        if let Some(undo) = self.config.undo {
            self.hook.unregister(undo)?;
        }
        self.config.undo = hotkey;
        Ok(())
    }

    /// Sets the key to use for switching to the previous comparison.
    pub fn set_previous_comparison(&mut self, hotkey: Option<KeyCode>) -> Result<()> {
        if self.config.previous_comparison == hotkey {
            return Ok(());
        }
        let inner = self.timer.clone();
        let active = self.is_active.clone();
        if let Some(hotkey) = hotkey {
            self.hook.register(hotkey, move || {
                if active.load(Ordering::Acquire) {
                    inner.write().switch_to_previous_comparison();
                }
            })?;
        }
        if let Some(previous_comparison) = self.config.previous_comparison {
            self.hook.unregister(previous_comparison)?;
        }
        self.config.previous_comparison = hotkey;
        Ok(())
    }

    /// Sets the key to use for switching to the next comparison.
    pub fn set_next_comparison(&mut self, hotkey: Option<KeyCode>) -> Result<()> {
        if self.config.next_comparison == hotkey {
            return Ok(());
        }
        let inner = self.timer.clone();
        let active = self.is_active.clone();
        if let Some(hotkey) = hotkey {
            self.hook.register(hotkey, move || {
                if active.load(Ordering::Acquire) {
                    inner.write().switch_to_next_comparison();
                }
            })?;
        }
        if let Some(next_comparison) = self.config.next_comparison {
            self.hook.unregister(next_comparison)?;
        }
        self.config.next_comparison = hotkey;
        Ok(())
    }

    /// Sets the key to use for removing all the pause times from the current
    /// time.
    pub fn set_undo_all_pauses(&mut self, hotkey: Option<KeyCode>) -> Result<()> {
        if self.config.undo_all_pauses == hotkey {
            return Ok(());
        }
        let inner = self.timer.clone();
        let active = self.is_active.clone();
        if let Some(hotkey) = hotkey {
            self.hook.register(hotkey, move || {
                if active.load(Ordering::Acquire) {
                    inner.write().undo_all_pauses();
                }
            })?;
        }
        if let Some(undo_all_pauses) = self.config.undo_all_pauses {
            self.hook.unregister(undo_all_pauses)?;
        }
        self.config.undo_all_pauses = hotkey;
        Ok(())
    }

    /// Sets the key to use for toggling between the `Real Time` and `Game Time`
    /// timing methods.
    pub fn set_toggle_timing_method(&mut self, hotkey: Option<KeyCode>) -> Result<()> {
        if self.config.toggle_timing_method == hotkey {
            return Ok(());
        }
        let inner = self.timer.clone();
        let active = self.is_active.clone();
        if let Some(hotkey) = hotkey {
            self.hook.register(hotkey, move || {
                if active.load(Ordering::Acquire) {
                    inner.write().toggle_timing_method();
                }
            })?;
        }
        if let Some(toggle_timing_method) = self.config.toggle_timing_method {
            self.hook.unregister(toggle_timing_method)?;
        }
        self.config.toggle_timing_method = hotkey;
        Ok(())
    }

    /// Deactivates the Hotkey System. No hotkeys will go through until it gets
    /// activated again. If it's already deactivated, nothing happens.
    pub fn deactivate(&self) {
        self.is_active.store(false, Ordering::Release);
    }

    /// Activates a previously deactivated Hotkey System. If it's already
    /// active, nothing happens.
    pub fn activate(&self) {
        self.is_active.store(true, Ordering::Release);
    }

    /// Returns the hotkey configuration currently in use by the Hotkey System.
    pub fn config(&self) -> HotkeyConfig {
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
