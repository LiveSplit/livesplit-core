use {SharedTimer, HotkeyConfig};
use hotkey::{Hook, KeyCode};
pub use hotkey::{Result, Error};

pub struct HotkeySystem {
    config: HotkeyConfig,
    hook: Hook,
    timer: SharedTimer,
}

impl HotkeySystem {
    pub fn new(timer: SharedTimer) -> Result<Self> {
        Self::with_config(timer, Default::default())
    }

    pub fn with_config(timer: SharedTimer, config: HotkeyConfig) -> Result<Self> {
        let hook = Hook::new()?;

        let inner = timer.clone();
        hook.register(config.split, move || { inner.write().split(); })?;

        let inner = timer.clone();
        hook.register(config.reset, move || { inner.write().reset(true); })?;

        let inner = timer.clone();
        hook.register(config.undo, move || { inner.write().undo_split(); })?;

        let inner = timer.clone();
        hook.register(config.skip, move || { inner.write().skip_split(); })?;

        let inner = timer.clone();
        hook.register(config.pause, move || { inner.write().pause(); })?;

        let inner = timer.clone();
        hook.register(config.previous_comparison,
                      move || { inner.write().switch_to_previous_comparison(); })?;

        let inner = timer.clone();
        hook.register(config.next_comparison,
                      move || { inner.write().switch_to_next_comparison(); })?;

        Ok(Self {
               config: config,
               hook: hook,
               timer: timer,
           })
    }

    // TODO Ignore errors in a lot of situations
    // If unregister works and register fails for example,
    // you won't be able to register again, as unregistering will fail forever.
    // Also in initial start up code ignore partially failed registers.

    pub fn set_split(&mut self, hotkey: KeyCode) -> Result<()> {
        self.hook.unregister(self.config.split)?;
        let inner = self.timer.clone();
        self.hook
            .register(hotkey, move || { inner.write().split(); })?;
        self.config.split = hotkey;
        Ok(())
    }

    pub fn set_reset(&mut self, hotkey: KeyCode) -> Result<()> {
        self.hook.unregister(self.config.reset)?;
        let inner = self.timer.clone();
        self.hook
            .register(hotkey, move || { inner.write().reset(true); })?;
        self.config.reset = hotkey;
        Ok(())
    }

    pub fn set_pause(&mut self, hotkey: KeyCode) -> Result<()> {
        self.hook.unregister(self.config.pause)?;
        let inner = self.timer.clone();
        self.hook
            .register(hotkey, move || { inner.write().pause(); })?;
        self.config.pause = hotkey;
        Ok(())
    }

    pub fn set_skip(&mut self, hotkey: KeyCode) -> Result<()> {
        self.hook.unregister(self.config.skip)?;
        let inner = self.timer.clone();
        self.hook
            .register(hotkey, move || { inner.write().skip_split(); })?;
        self.config.skip = hotkey;
        Ok(())
    }

    pub fn set_undo(&mut self, hotkey: KeyCode) -> Result<()> {
        self.hook.unregister(self.config.undo)?;
        let inner = self.timer.clone();
        self.hook
            .register(hotkey, move || { inner.write().undo_split(); })?;
        self.config.undo = hotkey;
        Ok(())
    }

    pub fn set_previous_comparison(&mut self, hotkey: KeyCode) -> Result<()> {
        self.hook.unregister(self.config.previous_comparison)?;
        let inner = self.timer.clone();
        self.hook
            .register(hotkey,
                      move || { inner.write().switch_to_previous_comparison(); })?;
        self.config.previous_comparison = hotkey;
        Ok(())
    }

    pub fn set_next_comparison(&mut self, hotkey: KeyCode) -> Result<()> {
        self.hook.unregister(self.config.next_comparison)?;
        let inner = self.timer.clone();
        self.hook
            .register(hotkey,
                      move || { inner.write().switch_to_next_comparison(); })?;
        self.config.next_comparison = hotkey;
        Ok(())
    }
}
