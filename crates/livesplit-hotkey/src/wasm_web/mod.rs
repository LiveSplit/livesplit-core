use crate::{Hotkey, KeyCode, Modifiers};
use js_sys::{Function, Promise, Reflect};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, Event, Gamepad, GamepadButton, KeyboardEvent};

use std::{
    cell::{Cell, RefCell},
    collections::hash_map::{Entry, HashMap},
    rc::Rc,
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
    /// Failed creating the hook.
    FailedToCreateHook,
}

/// The result type for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// A hook allows you to listen to hotkeys.
pub struct Hook {
    hotkeys: Arc<Mutex<HashMap<Hotkey, Box<dyn FnMut() + Send + 'static>>>>,
    keyboard_callback: Closure<dyn FnMut(Event)>,
    gamepad_callback: Closure<dyn FnMut()>,
    interval_id: Cell<Option<i32>>,
    keyboard_layout_resolver: Rc<RefCell<Option<(JsValue, Function)>>>,
    _keyboard_layout_closure: Option<Closure<dyn FnMut(JsValue)>>,
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
    /// Creates a new hook.
    pub fn new() -> Result<Self> {
        let hotkeys = Arc::new(Mutex::new(HashMap::<
            Hotkey,
            Box<dyn FnMut() + Send + 'static>,
        >::new()));

        let window = window().ok_or(Error::FailedToCreateHook)?;

        let hotkey_map = hotkeys.clone();
        let keyboard_callback = Closure::wrap(Box::new(move |event: Event| {
            // Despite all sorts of documentation claiming that `keydown` events
            // pass you a `KeyboardEvent`, this is not actually always the case
            // in browsers. At least in Chrome selecting an element of an
            // `input` sends a `keydown` event that is not a `KeyboardEvent`.
            if let Ok(event) = event.dyn_into::<KeyboardEvent>() {
                if !event.repeat() {
                    if let Ok(code) = event.code().parse::<KeyCode>() {
                        let mut modifiers = Modifiers::empty();
                        if event.shift_key()
                            && !matches!(code, KeyCode::ShiftLeft | KeyCode::ShiftRight)
                        {
                            modifiers.insert(Modifiers::SHIFT);
                        }
                        if event.ctrl_key()
                            && !matches!(code, KeyCode::ControlLeft | KeyCode::ControlRight)
                        {
                            modifiers.insert(Modifiers::CONTROL);
                        }
                        if event.alt_key() && !matches!(code, KeyCode::AltLeft | KeyCode::AltRight)
                        {
                            modifiers.insert(Modifiers::ALT);
                        }
                        if event.meta_key()
                            && !matches!(code, KeyCode::MetaLeft | KeyCode::MetaRight)
                        {
                            modifiers.insert(Modifiers::META);
                        }

                        if let Some(callback) = hotkey_map
                            .lock()
                            .unwrap()
                            .get_mut(&code.with_modifiers(modifiers))
                        {
                            callback();
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(Event)>);

        window
            .add_event_listener_with_callback("keydown", keyboard_callback.as_ref().unchecked_ref())
            .map_err(|_| Error::FailedToCreateHook)?;

        let hotkey_map = hotkeys.clone();

        let mut states = Vec::new();
        let navigator = window.navigator();

        let keyboard_layout_resolver = Rc::new(RefCell::new(None));
        let _keyboard_layout_closure = (|| {
            let keyboard = Reflect::get(navigator.as_ref(), &JsValue::from_str("keyboard")).ok()?;
            if keyboard.is_undefined() {
                return None;
            }

            let get_layout_map =
                Reflect::get(&keyboard, &JsValue::from_str("getLayoutMap")).ok()?;

            let layout_map_promise = get_layout_map
                .dyn_ref::<Function>()?
                .call0(&keyboard)
                .ok()?;

            let keyboard_layout_resolver = keyboard_layout_resolver.clone();

            let closure = Closure::wrap(Box::new(move |layout_map| {
                if let Ok(get_fn) = Reflect::get(&layout_map, &JsValue::from_str("get")) {
                    if let Ok(get_fn) = get_fn.dyn_into::<Function>() {
                        *keyboard_layout_resolver.borrow_mut() = Some((layout_map, get_fn));
                    }
                }
            }) as Box<dyn FnMut(JsValue)>);

            let _ = layout_map_promise.dyn_ref::<Promise>()?.then(&closure);

            Some(closure)
        })();

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
                                        hotkey_map.lock().unwrap().get_mut(&code.into())
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
            keyboard_layout_resolver,
            _keyboard_layout_closure,
        })
    }

    /// Registers a hotkey to listen to.
    pub fn register<F>(&self, hotkey: Hotkey, callback: F) -> Result<()>
    where
        F: FnMut() + Send + 'static,
    {
        if let Entry::Vacant(vacant) = self.hotkeys.lock().unwrap().entry(hotkey) {
            if GAMEPAD_BUTTONS.contains(&hotkey.key_code) && self.interval_id.get().is_none() {
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

    /// Unregisters a previously registered hotkey.
    pub fn unregister(&self, hotkey: Hotkey) -> Result<()> {
        if self.hotkeys.lock().unwrap().remove(&hotkey).is_some() {
            Ok(())
        } else {
            Err(Error::NotRegistered)
        }
    }

    pub(crate) fn try_resolve(&self, key_code: KeyCode) -> Option<String> {
        let keyboard_layout_resolver = self.keyboard_layout_resolver.borrow();
        let (layout, resolve_fn) = keyboard_layout_resolver.as_ref()?;

        resolve_fn
            .call1(layout, &JsValue::from_str(key_code.name()))
            .ok()?
            .as_string()
    }
}
