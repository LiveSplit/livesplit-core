use crate::KeyCode;
use std::{
    collections::hash_map::{Entry, HashMap},
    slice, str,
    sync::{Arc, Mutex},
};

/// The error type for this crate.
#[derive(Debug, snafu::Snafu)]
#[non_exhaustive]
pub enum Error {
    /// The hotkey was already registered.
    AlreadyRegistered,
    /// The hotkey to unregister was not registered.
    NotRegistered,
}

/// The result type for this crate.
pub type Result<T> = std::result::Result<T, Error>;

pub type EventListenerHandle = Box<dyn Fn(&str)>;

/// A hook allows you to listen to hotkeys.
pub struct Hook {
    hotkeys: Arc<Mutex<HashMap<KeyCode, Box<dyn FnMut() + Send + 'static>>>>,
    event: Option<Box<EventListenerHandle>>,
}

#[allow(improper_ctypes)]
extern "C" {
    fn HotkeyHook_new(handle: *const EventListenerHandle);
    fn HotkeyHook_drop(handle: *const EventListenerHandle);
}

impl Drop for Hook {
    fn drop(&mut self) {
        let handle = self.event.take().unwrap();
        unsafe {
            HotkeyHook_drop(&*handle);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn HotkeyHook_callback(
    ptr: *const u8,
    len: usize,
    handle: *const EventListenerHandle,
) {
    let t = str::from_utf8(slice::from_raw_parts(ptr, len)).unwrap();
    (*handle)(t);
}

impl Hook {
    /// Creates a new hook.
    pub fn new() -> Result<Self> {
        let hotkeys = Arc::new(Mutex::new(HashMap::<
            KeyCode,
            Box<dyn FnMut() + Send + 'static>,
        >::new()));

        let hotkey_map = hotkeys.clone();
        let event = Box::new(Box::new(move |code: &str| {
            if let Ok(code) = code.parse() {
                if let Some(callback) = hotkey_map.lock().unwrap().get_mut(&code) {
                    callback();
                }
            }
        }) as EventListenerHandle);

        unsafe {
            HotkeyHook_new(&*event);
        }

        Ok(Hook {
            hotkeys,
            event: Some(event),
        })
    }

    /// Registers a hotkey to listen to.
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

    /// Unregisters a previously registered hotkey.
    pub fn unregister(&self, hotkey: KeyCode) -> Result<()> {
        if self.hotkeys.lock().unwrap().remove(&hotkey).is_some() {
            Ok(())
        } else {
            Err(Error::NotRegistered)
        }
    }
}

pub(crate) fn try_resolve(_key_code: KeyCode) -> Option<String> {
    None
}
