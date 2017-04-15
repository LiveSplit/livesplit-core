use {SharedTimer, HotkeyConfig, SharedHotkeyConfig};
use hotkey::{register_hook, Hook, KeyEvent};
use parking_lot::{RwLockReadGuard, RwLockWriteGuard};

pub struct HotkeySystem {
    config: SharedHotkeyConfig,
    _hook: Hook,
}

impl HotkeySystem {
    pub fn new(timer: SharedTimer) -> Result<Self, ()> {
        Self::with_config(timer, Default::default())
    }

    pub fn with_config(timer: SharedTimer, config: HotkeyConfig) -> Result<Self, ()> {
        let config = config.into_shared();

        Ok(Self {
            config: config.clone(),
            _hook: register_hook(move |k| if let KeyEvent::KeyDown(k) = k {
                let config = config.read();
                if k == config.split {
                    timer.split();
                } else if k == config.reset {
                    timer.reset(true);
                } else if k == config.skip {
                    timer.skip_split();
                } else if k == config.undo {
                    timer.undo();
                } else if k == config.redo {
                    timer.redo();
                } else if k == config.pause {
                    timer.pause();
                } else if k == config.previous_comparison {
                    timer.switch_to_previous_comparison();
                } else if k == config.next_comparison {
                    timer.switch_to_next_comparison();
                }
            })?,
        })
    }

    pub fn read_config(&self) -> RwLockReadGuard<HotkeyConfig> {
        self.config.read()
    }

    pub fn write_config(&self) -> RwLockWriteGuard<HotkeyConfig> {
        self.config.write()
    }
}
