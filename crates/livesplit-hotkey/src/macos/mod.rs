mod cf;
mod cg;
mod key_code;

use self::{
    cf::{
        kCFAllocatorDefault, kCFRunLoopDefaultMode, CFMachPortCreateRunLoopSource, CFRelease,
        CFRunLoopAddSource, CFRunLoopContainsSource, CFRunLoopGetCurrent, CFRunLoopRemoveSource,
        CFRunLoopRun,
    },
    cg::{
        CGEventTapCreate, EventMask, EventRef, EventTapLocation, EventTapOptions,
        EventTapPlacement, EventTapProxy, EventType,
    },
};
use cg::EventField;
use parking_lot::Mutex;
use std::{
    collections::{hash_map::Entry, HashMap},
    ffi::c_void,
    sync::{mpsc::channel, Arc},
    thread,
};

pub use self::key_code::KeyCode;

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    AlreadyRegistered,
    NotRegistered,
    CouldntCreateEventTap,
    CouldntCreateRunLoopSource,
    CouldntGetCurrentRunLoop,
    ThreadStoppedUnexpectedly,
}

pub type Result<T> = std::result::Result<T, Error>;

struct Owned<T>(*mut T);

impl<T> Drop for Owned<T> {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.0.cast());
        }
    }
}

#[derive(Copy, Clone)]
struct RunLoop(cf::RunLoopRef);

unsafe impl Send for RunLoop {}

type RegisteredKeys = Mutex<HashMap<KeyCode, Box<dyn FnMut() + Send + 'static>>>;

pub struct Hook {
    event_loop: RunLoop,
    hotkeys: Arc<RegisteredKeys>,
}

impl Drop for Hook {
    fn drop(&mut self) {
        unsafe {
            let mode = cf::CFRunLoopCopyCurrentMode(self.event_loop.0);
            if !mode.is_null() {
                cf::CFRelease(mode.cast());
                cf::CFRunLoopStop(self.event_loop.0);
            }
        }
    }
}

impl Hook {
    pub fn new() -> Result<Self> {
        let hotkeys = Arc::new(Mutex::new(HashMap::new()));
        let thread_hotkeys = hotkeys.clone();

        let (sender, receiver) = channel();

        // The code here is mostly based on:
        // https://github.com/kwhat/libuiohook/blob/f4bb19be8aee7d7ee5ead89b5a89dbf440e2a71a/src/darwin/input_hook.c#L1086

        thread::spawn(move || unsafe {
            let hotkeys_ptr: *const Mutex<_> = &*thread_hotkeys;

            let port = CGEventTapCreate(
                EventTapLocation::Session,
                EventTapPlacement::HeadInsertEventTap,
                EventTapOptions::DefaultTap,
                EventMask::KEY_DOWN,
                Some(callback),
                hotkeys_ptr as *mut c_void,
            );
            if port.is_null() {
                let _ = sender.send(Err(Error::CouldntCreateEventTap));
                return;
            }
            let port = Owned(port);

            let source = CFMachPortCreateRunLoopSource(kCFAllocatorDefault, port.0, 0);
            if source.is_null() {
                let _ = sender.send(Err(Error::CouldntCreateRunLoopSource));
                return;
            }
            let source = Owned(source);

            let event_loop = CFRunLoopGetCurrent();
            if event_loop.is_null() {
                let _ = sender.send(Err(Error::CouldntGetCurrentRunLoop));
                return;
            }

            CFRunLoopAddSource(event_loop, source.0, kCFRunLoopDefaultMode);

            if { sender }.send(Ok(RunLoop(event_loop))).is_ok() {
                CFRunLoopRun();
            }

            if CFRunLoopContainsSource(event_loop, source.0, kCFRunLoopDefaultMode) {
                CFRunLoopRemoveSource(event_loop, source.0, kCFRunLoopDefaultMode);
            }
        });

        let event_loop = receiver
            .recv()
            .map_err(|_| Error::ThreadStoppedUnexpectedly)??;

        Ok(Hook {
            event_loop,
            hotkeys,
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

unsafe extern "C" fn callback(
    _: EventTapProxy,
    ty: EventType,
    event: EventRef,
    user_info: *mut c_void,
) -> EventRef {
    if matches!(ty, EventType::KeyDown) {
        let key_code = cg::CGEventGetIntegerValueField(event, EventField::KeyboardEventKeycode);
        let key_code = match key_code {
            0x00 => KeyCode::A,
            0x01 => KeyCode::S,
            0x02 => KeyCode::D,
            0x03 => KeyCode::F,
            0x04 => KeyCode::H,
            0x05 => KeyCode::G,
            0x06 => KeyCode::Z,
            0x07 => KeyCode::X,
            0x08 => KeyCode::C,
            0x09 => KeyCode::V,
            0x0A => KeyCode::IsoSection,
            0x0B => KeyCode::B,
            0x0C => KeyCode::Q,
            0x0D => KeyCode::W,
            0x0E => KeyCode::E,
            0x0F => KeyCode::R,
            0x10 => KeyCode::Y,
            0x11 => KeyCode::T,
            0x12 => KeyCode::Digit1,
            0x13 => KeyCode::Digit2,
            0x14 => KeyCode::Digit3,
            0x15 => KeyCode::Digit4,
            0x16 => KeyCode::Digit6,
            0x17 => KeyCode::Digit5,
            0x18 => KeyCode::Equal,
            0x19 => KeyCode::Digit9,
            0x1A => KeyCode::Digit7,
            0x1B => KeyCode::Minus,
            0x1C => KeyCode::Digit8,
            0x1D => KeyCode::Digit0,
            0x1E => KeyCode::RightBracket,
            0x1F => KeyCode::O,
            0x20 => KeyCode::U,
            0x21 => KeyCode::LeftBracket,
            0x22 => KeyCode::I,
            0x23 => KeyCode::P,
            0x24 => KeyCode::Return,
            0x25 => KeyCode::L,
            0x26 => KeyCode::J,
            0x27 => KeyCode::Quote,
            0x28 => KeyCode::K,
            0x29 => KeyCode::Semicolon,
            0x2A => KeyCode::Backslash,
            0x2B => KeyCode::Comma,
            0x2C => KeyCode::Slash,
            0x2D => KeyCode::N,
            0x2E => KeyCode::M,
            0x2F => KeyCode::Period,
            0x30 => KeyCode::Tab,
            0x31 => KeyCode::Space,
            0x32 => KeyCode::Grave,
            0x33 => KeyCode::Delete,
            0x35 => KeyCode::Escape,
            0x36 => KeyCode::RightCommand,
            0x37 => KeyCode::Command,
            0x38 => KeyCode::Shift,
            0x39 => KeyCode::CapsLock,
            0x3A => KeyCode::Option,
            0x3B => KeyCode::Control,
            0x3C => KeyCode::RightShift,
            0x3D => KeyCode::RightOption,
            0x3E => KeyCode::RightControl,
            0x3F => KeyCode::Function,
            0x40 => KeyCode::F17,
            0x41 => KeyCode::NumpadDecimal,
            0x43 => KeyCode::NumpadMultiply,
            0x45 => KeyCode::NumpadPlus,
            0x47 => KeyCode::NumpadClear,
            0x48 => KeyCode::VolumeUp,
            0x49 => KeyCode::VolumeDown,
            0x4A => KeyCode::Mute,
            0x4B => KeyCode::NumpadDivide,
            0x4C => KeyCode::NumpadEnter,
            0x4E => KeyCode::NumpadMinus,
            0x4F => KeyCode::F18,
            0x50 => KeyCode::F19,
            0x51 => KeyCode::NumpadEquals,
            0x52 => KeyCode::Numpad0,
            0x53 => KeyCode::Numpad1,
            0x54 => KeyCode::Numpad2,
            0x55 => KeyCode::Numpad3,
            0x56 => KeyCode::Numpad4,
            0x57 => KeyCode::Numpad5,
            0x58 => KeyCode::Numpad6,
            0x59 => KeyCode::Numpad7,
            0x5A => KeyCode::F20,
            0x5B => KeyCode::Numpad8,
            0x5C => KeyCode::Numpad9,
            0x5D => KeyCode::JisYen,
            0x5E => KeyCode::JisUnderscore,
            0x5F => KeyCode::JisKeypadComma,
            0x60 => KeyCode::F5,
            0x61 => KeyCode::F6,
            0x62 => KeyCode::F7,
            0x63 => KeyCode::F3,
            0x64 => KeyCode::F8,
            0x65 => KeyCode::F9,
            0x66 => KeyCode::JisEisu,
            0x67 => KeyCode::F11,
            0x68 => KeyCode::JisKana,
            0x69 => KeyCode::F13,
            0x6A => KeyCode::F16,
            0x6B => KeyCode::F14,
            0x6D => KeyCode::F10,
            0x6F => KeyCode::F12,
            0x71 => KeyCode::F15,
            0x72 => KeyCode::Help,
            0x73 => KeyCode::Home,
            0x74 => KeyCode::PageUp,
            0x75 => KeyCode::ForwardDelete,
            0x76 => KeyCode::F4,
            0x77 => KeyCode::End,
            0x78 => KeyCode::F2,
            0x79 => KeyCode::PageDown,
            0x7A => KeyCode::F1,
            0x7B => KeyCode::LeftArrow,
            0x7C => KeyCode::RightArrow,
            0x7D => KeyCode::DownArrow,
            0x7E => KeyCode::UpArrow,
            _ => return event,
        };

        let hotkeys = user_info as *const RegisteredKeys;
        let hotkeys = &*hotkeys;
        if let Some(callback) = hotkeys.lock().get_mut(&key_code) {
            callback();
        }
    }
    event
}
