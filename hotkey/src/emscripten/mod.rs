mod key_code;
pub use self::key_code::KeyCode;

use parking_lot::Mutex;
use std::collections::hash_map::{Entry, HashMap};
use std::sync::Arc;
use stdweb::web::event::{IKeyboardEvent, KeypressEvent};
use stdweb::web::{window, EventListenerHandle, IEventTarget};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        AlreadyRegistered {}
        NotRegistered {}
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Hook {
    hotkeys: Arc<Mutex<HashMap<KeyCode, Box<dyn FnMut() + Send + 'static>>>>,
    event: Option<EventListenerHandle>,
}

impl Drop for Hook {
    fn drop(&mut self) {
        self.event.take().unwrap().remove();
    }
}

impl Hook {
    pub fn new() -> Result<Self> {
        stdweb::initialize();

        js! {
            var keyProt = KeyboardEvent && KeyboardEvent.prototype || Event.prototype;

            if (Object.getOwnPropertyDescriptor(keyProt, "code") !== undefined) {
                return;
            }

            Object.defineProperties(keyProt, {
                "code": {
                    get: function() {
                        switch (this.keyCode) {
                            case 8: return "Backspace";
                            case 9: return "Tab";
                            case 12: return "NumpadEqual";
                            case 13: return "Enter";
                            case 16: return "ShiftLeft";
                            case 17: return "ControlLeft";
                            case 18: return "AltLeft";
                            case 19: return "Pause";
                            case 20: return "CapsLock";
                            case 27: return "Escape";
                            case 32: return "Space";
                            case 33: return "PageUp";
                            case 34: return "PageDown";
                            case 35: return "End";
                            case 36: return "Home";
                            case 37: return "ArrowLeft";
                            case 38: return "ArrowUp";
                            case 39: return "ArrowRight";
                            case 40: return "ArrowDown";
                            case 44: return "PrintScreen";
                            case 45: return "Insert";
                            case 46: return "Delete";
                            case 47: return "Help";
                            // FeelsBadMan, those are supposed to be digits
                            case 48: return "Numpad0";
                            case 49: return "Numpad1";
                            case 50: return "Numpad2";
                            case 51: return "Numpad3";
                            case 52: return "Numpad4";
                            case 53: return "Numpad5";
                            case 54: return "Numpad6";
                            case 55: return "Numpad7";
                            case 56: return "Numpad8";
                            case 57: return "Numpad9";
                            case 65: return "KeyA";
                            case 66: return "KeyB";
                            case 67: return "KeyC";
                            case 68: return "KeyD";
                            case 69: return "KeyE";
                            case 70: return "KeyF";
                            case 71: return "KeyG";
                            case 72: return "KeyH";
                            case 73: return "KeyI";
                            case 74: return "KeyJ";
                            case 75: return "KeyK";
                            case 76: return "KeyL";
                            case 77: return "KeyM";
                            case 78: return "KeyN";
                            case 79: return "KeyO";
                            case 80: return "KeyP";
                            case 81: return "KeyQ";
                            case 82: return "KeyR";
                            case 83: return "KeyS";
                            case 84: return "KeyT";
                            case 85: return "KeyU";
                            case 86: return "KeyV";
                            case 87: return "KeyW";
                            case 88: return "KeyX";
                            case 89: return "KeyY";
                            case 90: return "KeyZ";
                            case 91: return "MetaLeft";
                            case 92: return "MetaRight";
                            case 93: return "ContextMenu";
                            case 96: return "Numpad0";
                            case 97: return "Numpad1";
                            case 98: return "Numpad2";
                            case 99: return "Numpad3";
                            case 100: return "Numpad4";
                            case 101: return "Numpad5";
                            case 102: return "Numpad6";
                            case 103: return "Numpad7";
                            case 104: return "Numpad8";
                            case 105: return "Numpad9";
                            case 106: return "NumpadMultiply";
                            case 107: return "NumpadAdd";
                            case 109: return "NumpadSubtract";
                            case 110: return "NumpadDecimal";
                            case 111: return "NumpadDivide";
                            case 112: return "F1";
                            case 113: return "F2";
                            case 114: return "F3";
                            case 115: return "F4";
                            case 116: return "F5";
                            case 117: return "F6";
                            case 118: return "F7";
                            case 119: return "F8";
                            case 120: return "F9";
                            case 121: return "F10";
                            case 122: return "F11";
                            case 123: return "F12";
                            case 124: return "F13";
                            case 125: return "F14";
                            case 126: return "F15";
                            case 127: return "F16";
                            case 128: return "F17";
                            case 129: return "F18";
                            case 130: return "F19";
                            case 131: return "F20";
                            case 132: return "F21";
                            case 133: return "F22";
                            case 134: return "F23";
                            case 135: return "F24";
                            case 144: return "NumLock";
                            case 145: return "ScrollLock";
                            case 186: return "Semicolon";
                            case 187: return "Equal";
                            case 188: return "Comma";
                            case 189: return "Minus";
                            case 190: return "Period";
                            case 191: return "Slash";
                            case 192: return "Backquote";
                            case 193: return "IntlRo";
                            case 194: return "NumpadComma";
                            case 219: return "BracketLeft";
                            case 220: return "Backslash";
                            case 221: return "BracketRight";
                            case 222: return "Quote";
                            case 226: return "IntlBackslash";
                            case 255: return "IntlYen";
                            default: return "Unidentified";
                        }
                    }
                }
            });
        }

        let hotkeys = Arc::new(Mutex::new(HashMap::<
            KeyCode,
            Box<dyn FnMut() + Send + 'static>,
        >::new()));

        let hotkey_map = hotkeys.clone();

        let event = window().add_event_listener(move |event: KeypressEvent| {
            let code = event.code();
            if let Ok(code) = code.parse() {
                if let Some(callback) = hotkey_map.lock().get_mut(&code) {
                    callback();
                }
            }
        });

        Ok(Hook {
            hotkeys,
            event: Some(event),
        })
    }

    pub fn register<F>(&self, hotkey: KeyCode, callback: F) -> Result<()>
    where
        F: FnMut() + Send + 'static,
    {
        if let Entry::Vacant(vacant) = self.hotkeys.lock().entry(hotkey) {
            vacant.insert(Box::new(callback));
            Ok(())
        } else {
            Err(Error::AlreadyRegistered)
        }
    }

    pub fn unregister(&self, hotkey: KeyCode) -> Result<()> {
        if self.hotkeys.lock().remove(&hotkey).is_some() {
            Ok(())
        } else {
            Err(Error::NotRegistered)
        }
    }
}
