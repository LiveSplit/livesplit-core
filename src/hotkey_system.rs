use {HotkeyConfig, SharedTimer};
use hotkey::{Hook, KeyCode};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub use hotkey::{Error, Result};

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

        let inner = timer.clone();
        let active = is_active.clone();
        hook.register(config.split, move || {
            if active.load(Ordering::Acquire) {
                inner.write().split_or_start();
            }
        })?;

        let inner = timer.clone();
        let active = is_active.clone();
        hook.register(config.reset, move || {
            if active.load(Ordering::Acquire) {
                inner.write().reset(true);
            }
        })?;

        let inner = timer.clone();
        let active = is_active.clone();
        hook.register(config.undo, move || {
            if active.load(Ordering::Acquire) {
                inner.write().undo_split();
            }
        })?;

        let inner = timer.clone();
        let active = is_active.clone();
        hook.register(config.skip, move || {
            if active.load(Ordering::Acquire) {
                inner.write().skip_split();
            }
        })?;

        let inner = timer.clone();
        let active = is_active.clone();
        hook.register(config.pause, move || {
            if active.load(Ordering::Acquire) {
                inner.write().toggle_pause_or_start();
            }
        })?;

        let inner = timer.clone();
        let active = is_active.clone();
        hook.register(config.previous_comparison, move || {
            if active.load(Ordering::Acquire) {
                inner.write().switch_to_previous_comparison();
            }
        })?;

        let inner = timer.clone();
        let active = is_active.clone();
        hook.register(config.next_comparison, move || {
            if active.load(Ordering::Acquire) {
                inner.write().switch_to_next_comparison();
            }
        })?;

        Ok(Self {
            config,
            hook,
            timer,
            is_active,
        })
    }

    // TODO Ignore errors in a lot of situations
    //
    // If unregister works and register fails for example, you won't be able to
    // register again, as unregistering will fail forever. Also in initial start
    // up code ignore partially failed registers.

    /// Sets the key to use for splitting and starting a new attempt.
    pub fn set_split(&mut self, hotkey: KeyCode) -> Result<()> {
        self.hook.unregister(self.config.split)?;
        let inner = self.timer.clone();
        let active = self.is_active.clone();
        self.hook.register(hotkey, move || {
            if active.load(Ordering::Acquire) {
                inner.write().split_or_start();
            }
        })?;
        self.config.split = hotkey;
        Ok(())
    }

    /// Sets the key to use for resetting the current attempt.
    pub fn set_reset(&mut self, hotkey: KeyCode) -> Result<()> {
        self.hook.unregister(self.config.reset)?;
        let inner = self.timer.clone();
        let active = self.is_active.clone();
        self.hook.register(hotkey, move || {
            if active.load(Ordering::Acquire) {
                inner.write().reset(true);
            }
        })?;
        self.config.reset = hotkey;
        Ok(())
    }

    /// Sets the key to use for pausing the current attempt and starting a new
    /// attempt.
    pub fn set_pause(&mut self, hotkey: KeyCode) -> Result<()> {
        self.hook.unregister(self.config.pause)?;
        let inner = self.timer.clone();
        let active = self.is_active.clone();
        self.hook.register(hotkey, move || {
            if active.load(Ordering::Acquire) {
                inner.write().toggle_pause_or_start();
            }
        })?;
        self.config.pause = hotkey;
        Ok(())
    }

    /// Sets the key to use for skipping the current split.
    pub fn set_skip(&mut self, hotkey: KeyCode) -> Result<()> {
        self.hook.unregister(self.config.skip)?;
        let inner = self.timer.clone();
        let active = self.is_active.clone();
        self.hook.register(hotkey, move || {
            if active.load(Ordering::Acquire) {
                inner.write().skip_split();
            }
        })?;
        self.config.skip = hotkey;
        Ok(())
    }

    /// Sets the key to use for undoing the last split.
    pub fn set_undo(&mut self, hotkey: KeyCode) -> Result<()> {
        self.hook.unregister(self.config.undo)?;
        let inner = self.timer.clone();
        let active = self.is_active.clone();
        self.hook.register(hotkey, move || {
            if active.load(Ordering::Acquire) {
                inner.write().undo_split();
            }
        })?;
        self.config.undo = hotkey;
        Ok(())
    }

    /// Sets the key to use for switching to the previous comparison.
    pub fn set_previous_comparison(&mut self, hotkey: KeyCode) -> Result<()> {
        self.hook.unregister(self.config.previous_comparison)?;
        let inner = self.timer.clone();
        let active = self.is_active.clone();
        self.hook.register(hotkey, move || {
            if active.load(Ordering::Acquire) {
                inner.write().switch_to_previous_comparison();
            }
        })?;
        self.config.previous_comparison = hotkey;
        Ok(())
    }

    /// Sets the key to use for switching to the next comparison.
    pub fn set_next_comparison(&mut self, hotkey: KeyCode) -> Result<()> {
        self.hook.unregister(self.config.next_comparison)?;
        let inner = self.timer.clone();
        let active = self.is_active.clone();
        self.hook.register(hotkey, move || {
            if active.load(Ordering::Acquire) {
                inner.write().switch_to_next_comparison();
            }
        })?;
        self.config.next_comparison = hotkey;
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
}
