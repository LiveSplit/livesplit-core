mod key_code;
pub use self::key_code::KeyCode;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, KeyboardEvent};

use std::collections::hash_map::{Entry, HashMap};
use std::sync::{Arc, Mutex};

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    AlreadyRegistered,
    NotRegistered,
    FailedToCreateHook,
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Hook {
    hotkeys: Arc<Mutex<HashMap<KeyCode, Box<dyn FnMut() + Send + 'static>>>>,
    callback: Closure<dyn FnMut(KeyboardEvent)>,
}

impl Drop for Hook {
    fn drop(&mut self) {
        if let Some(window) = window() {
            let _ = window.remove_event_listener_with_callback(
                "keypress",
                self.callback.as_ref().unchecked_ref(),
            );
        }
    }
}

impl Hook {
    pub fn new() -> Result<Self> {
        let hotkeys = Arc::new(Mutex::new(HashMap::<
            KeyCode,
            Box<dyn FnMut() + Send + 'static>,
        >::new()));

        let window = window().ok_or(Error::FailedToCreateHook)?;

        let hotkey_map = hotkeys.clone();
        let callback = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            if let Ok(code) = event.code().parse() {
                if let Some(callback) = hotkey_map.lock().unwrap().get_mut(&code) {
                    callback();
                }
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);

        window
            .add_event_listener_with_callback("keypress", callback.as_ref().unchecked_ref())
            .map_err(|_| Error::FailedToCreateHook)?;

        Ok(Hook { hotkeys, callback })
    }

    pub fn register<F>(&self, hotkey: KeyCode, callback: F) -> Result<()>
    where
        F: FnMut() + Send + 'static,
    {
        if let Entry::Vacant(vacant) = self.hotkeys.lock().unwrap().entry(hotkey) {
            vacant.insert(Box::new(callback));
            Ok(())
        } else {
            Err(Error::AlreadyRegistered)
        }
    }

    pub fn unregister(&self, hotkey: KeyCode) -> Result<()> {
        if self.hotkeys.lock().unwrap().remove(&hotkey).is_some() {
            Ok(())
        } else {
            Err(Error::NotRegistered)
        }
    }
}
