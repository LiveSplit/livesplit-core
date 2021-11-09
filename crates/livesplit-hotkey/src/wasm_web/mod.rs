use crate::KeyCode;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, Gamepad, GamepadButton, KeyboardEvent};

use std::{
    cell::Cell,
    collections::hash_map::{Entry, HashMap},
    sync::{Arc, Mutex},
};

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    AlreadyRegistered,
    NotRegistered,
    FailedToCreateHook,
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Hook {
    hotkeys: Arc<Mutex<HashMap<KeyCode, Box<dyn FnMut() + Send + 'static>>>>,
    keyboard_callback: Closure<dyn FnMut(KeyboardEvent)>,
    gamepad_callback: Closure<dyn FnMut()>,
    interval_id: Cell<Option<i32>>,
}

impl Drop for Hook {
    fn drop(&mut self) {
        if let Some(window) = window() {
            let _ = window.remove_event_listener_with_callback(
                "keydown",
                self.keyboard_callback.as_ref().unchecked_ref(),
            );
            if let Some(interval_id) = self.interval_id.get() {
                window.clear_interval_with_handle(interval_id);
            }
        }
    }
}

const TOTAL_BUTTONS: usize = 20;
static GAMEPAD_BUTTONS: [KeyCode; TOTAL_BUTTONS] = [
    KeyCode::Gamepad0,
    KeyCode::Gamepad1,
    KeyCode::Gamepad2,
    KeyCode::Gamepad3,
    KeyCode::Gamepad4,
    KeyCode::Gamepad5,
    KeyCode::Gamepad6,
    KeyCode::Gamepad7,
    KeyCode::Gamepad8,
    KeyCode::Gamepad9,
    KeyCode::Gamepad10,
    KeyCode::Gamepad11,
    KeyCode::Gamepad12,
    KeyCode::Gamepad13,
    KeyCode::Gamepad14,
    KeyCode::Gamepad15,
    KeyCode::Gamepad16,
    KeyCode::Gamepad17,
    KeyCode::Gamepad18,
    KeyCode::Gamepad19,
];

impl Hook {
    pub fn new() -> Result<Self> {
        let hotkeys = Arc::new(Mutex::new(HashMap::<
            KeyCode,
            Box<dyn FnMut() + Send + 'static>,
        >::new()));

        let window = window().ok_or(Error::FailedToCreateHook)?;

        let hotkey_map = hotkeys.clone();
        let keyboard_callback = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            if !event.repeat() {
                if let Ok(code) = event.code().parse() {
                    if let Some(callback) = hotkey_map.lock().unwrap().get_mut(&code) {
                        callback();
                    }
                }
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);

        window
            .add_event_listener_with_callback("keydown", keyboard_callback.as_ref().unchecked_ref())
            .map_err(|_| Error::FailedToCreateHook)?;

        let hotkey_map = hotkeys.clone();

        let mut states = Vec::new();
        let navigator = window.navigator();

        let gamepad_callback = Closure::wrap(Box::new(move || {
            if let Ok(gamepads) = navigator.get_gamepads() {
                let gamepads_len = gamepads.length() as usize;
                if states.len() < gamepads_len {
                    states.resize(gamepads_len, [false; TOTAL_BUTTONS]);
                }
                for (gamepad, states) in gamepads.iter().zip(&mut states) {
                    if let Ok(gamepad) = gamepad.dyn_into::<Gamepad>() {
                        for ((button, code), state) in gamepad
                            .buttons()
                            .iter()
                            .zip(GAMEPAD_BUTTONS)
                            .zip(states.iter_mut())
                        {
                            if let Ok(button) = button.dyn_into::<GamepadButton>() {
                                let pressed = button.pressed();
                                if pressed && !*state {
                                    if let Some(callback) =
                                        hotkey_map.lock().unwrap().get_mut(&code)
                                    {
                                        callback();
                                    }
                                }
                                *state = pressed;
                            }
                        }
                    }
                }
            }
        }) as Box<dyn FnMut()>);

        Ok(Hook {
            hotkeys,
            keyboard_callback,
            gamepad_callback,
            interval_id: Cell::new(None),
        })
    }

    pub fn register<F>(&self, hotkey: KeyCode, callback: F) -> Result<()>
    where
        F: FnMut() + Send + 'static,
    {
        if let Entry::Vacant(vacant) = self.hotkeys.lock().unwrap().entry(hotkey) {
            if GAMEPAD_BUTTONS.contains(&hotkey) && self.interval_id.get().is_none() {
                let interval_id = window()
                    .ok_or(Error::FailedToCreateHook)?
                    .set_interval_with_callback_and_timeout_and_arguments_0(
                        self.gamepad_callback.as_ref().unchecked_ref(),
                        1000 / 60,
                    )
                    .map_err(|_| Error::FailedToCreateHook)?;
                self.interval_id.set(Some(interval_id));
            }
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

pub(crate) fn try_resolve(_key_code: KeyCode) -> Option<String> {
    None
}
