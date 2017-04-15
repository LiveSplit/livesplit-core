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
                    timer.write().split();
                } else if k == config.reset {
                    timer.write().reset(true);
                } else if k == config.undo {
                    timer.write().undo_split();
                } else if k == config.skip {
                    timer.write().skip_split();
                } else if k == config.pause {
                    timer.write().pause();
                } else if k == config.previous_comparison {
                    timer.write().switch_to_previous_comparison();
                } else if k == config.next_comparison {
                    timer.write().switch_to_next_comparison();
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
