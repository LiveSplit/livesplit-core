use {Timer, HotkeyConfig};
use hotkey::{register_hook, Hook, KeyEvent};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct HotkeyTimer {
    timer: Arc<RwLock<Timer>>,
    config: Arc<RwLock<HotkeyConfig>>,
    _hook: Hook,
}

impl HotkeyTimer {
    pub fn new(timer: Timer) -> Result<Self, ()> {
        Self::with_config(timer, Default::default())
    }

    pub fn with_config(timer: Timer, config: HotkeyConfig) -> Result<Self, ()> {
        let timer = Arc::new(RwLock::new(timer));
        let config = Arc::new(RwLock::new(config));

        Ok(Self {
            timer: timer.clone(),
            config: config.clone(),
            _hook: register_hook(move |k| if let KeyEvent::KeyDown(k) = k {
                let config = config.read().unwrap();
                if k == config.split {
                    timer.write().unwrap().split();
                } else if k == config.reset {
                    timer.write().unwrap().reset(true);
                } else if k == config.undo {
                    timer.write().unwrap().undo_split();
                } else if k == config.skip {
                    timer.write().unwrap().skip_split();
                } else if k == config.pause {
                    timer.write().unwrap().pause();
                } else if k == config.previous_comparison {
                    timer.write().unwrap().switch_to_previous_comparison();
                } else if k == config.next_comparison {
                    timer.write().unwrap().switch_to_next_comparison();
                }
            })?,
        })
    }

    pub fn read(&self) -> RwLockReadGuard<Timer> {
        self.timer.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<Timer> {
        self.timer.write().unwrap()
    }

    pub fn read_config(&self) -> RwLockReadGuard<HotkeyConfig> {
        self.config.read().unwrap()
    }

    pub fn write_config(&self) -> RwLockWriteGuard<HotkeyConfig> {
        self.config.write().unwrap()
    }
}
