use std::{
    collections::hash_map::HashMap, os::unix::prelude::AsRawFd, ptr, sync::mpsc::channel, thread,
};

use evdev::{Device, EventType, InputEventKind, Key};
use mio::{unix::SourceFd, Events, Interest, Poll, Token, Waker};
use x11_dl::xlib::{Xlib, _XDisplay};

use super::{x11_impl, Message};
use crate::{Error, Hook, KeyCode, Modifiers, Result};

// Low numbered tokens are allocated to devices.
const PING_TOKEN: Token = Token(usize::MAX);

pub const fn code_for(key: KeyCode) -> Option<Key> {
    // This mapping is based on all the different browsers. They however all use
    // the X11 scan codes. Fortunately those have a trivial 1:1 mapping to evdev
    // scan codes.
    // https://github.com/freedesktop/xorg-xf86-input-evdev/blob/71036116be11b8c9d39ce153738875c44183cc60/src/evdev.c#L280
    // You simply need to subtract 8 from the X11 scan code to get to the evdev
    // scan code. So we take the mapping from the browsers, subtract 8 from each
    // value and then use the named constant for that value.
    // The USB HID to scan code translation in Linux is this table:
    // https://github.com/torvalds/linux/blob/fe91c4725aeed35023ba4f7a1e1adfebb6878c23/drivers/hid/hid-input.c#L27-L44
    use self::KeyCode::*;
    Some(match key {
        Escape => Key::KEY_ESC,
        Digit1 => Key::KEY_1,
        Digit2 => Key::KEY_2,
        Digit3 => Key::KEY_3,
        Digit4 => Key::KEY_4,
        Digit5 => Key::KEY_5,
        Digit6 => Key::KEY_6,
        Digit7 => Key::KEY_7,
        Digit8 => Key::KEY_8,
        Digit9 => Key::KEY_9,
        Digit0 => Key::KEY_0,
        Minus => Key::KEY_MINUS,
        Equal => Key::KEY_EQUAL,
        Backspace => Key::KEY_BACKSPACE,
        Tab => Key::KEY_TAB,
        KeyQ => Key::KEY_Q,
        KeyW => Key::KEY_W,
        KeyE => Key::KEY_E,
        KeyR => Key::KEY_R,
        KeyT => Key::KEY_T,
        KeyY => Key::KEY_Y,
        KeyU => Key::KEY_U,
        KeyI => Key::KEY_I,
        KeyO => Key::KEY_O,
        KeyP => Key::KEY_P,
        BracketLeft => Key::KEY_LEFTBRACE,
        BracketRight => Key::KEY_RIGHTBRACE,
        Enter => Key::KEY_ENTER,
        ControlLeft => Key::KEY_LEFTCTRL,
        KeyA => Key::KEY_A,
        KeyS => Key::KEY_S,
        KeyD => Key::KEY_D,
        KeyF => Key::KEY_F,
        KeyG => Key::KEY_G,
        KeyH => Key::KEY_H,
        KeyJ => Key::KEY_J,
        KeyK => Key::KEY_K,
        KeyL => Key::KEY_L,
        Semicolon => Key::KEY_SEMICOLON,
        Quote => Key::KEY_APOSTROPHE,
        Backquote => Key::KEY_GRAVE,
        ShiftLeft => Key::KEY_LEFTSHIFT,
        Backslash => Key::KEY_BACKSLASH,
        KeyZ => Key::KEY_Z,
        KeyX => Key::KEY_X,
        KeyC => Key::KEY_C,
        KeyV => Key::KEY_V,
        KeyB => Key::KEY_B,
        KeyN => Key::KEY_N,
        KeyM => Key::KEY_M,
        Comma => Key::KEY_COMMA,
        Period => Key::KEY_DOT,
        Slash => Key::KEY_SLASH,
        ShiftRight => Key::KEY_RIGHTSHIFT,
        NumpadMultiply => Key::KEY_KPASTERISK,
        AltLeft => Key::KEY_LEFTALT,
        Space => Key::KEY_SPACE,
        CapsLock => Key::KEY_CAPSLOCK,
        F1 => Key::KEY_F1,
        F2 => Key::KEY_F2,
        F3 => Key::KEY_F3,
        F4 => Key::KEY_F4,
        F5 => Key::KEY_F5,
        F6 => Key::KEY_F6,
        F7 => Key::KEY_F7,
        F8 => Key::KEY_F8,
        F9 => Key::KEY_F9,
        F10 => Key::KEY_F10,
        NumLock => Key::KEY_NUMLOCK,
        ScrollLock => Key::KEY_SCROLLLOCK,
        Numpad7 => Key::KEY_KP7,
        Numpad8 => Key::KEY_KP8,
        Numpad9 => Key::KEY_KP9,
        NumpadSubtract => Key::KEY_KPMINUS,
        Numpad4 => Key::KEY_KP4,
        Numpad5 => Key::KEY_KP5,
        Numpad6 => Key::KEY_KP6,
        NumpadAdd => Key::KEY_KPPLUS,
        Numpad1 => Key::KEY_KP1,
        Numpad2 => Key::KEY_KP2,
        Numpad3 => Key::KEY_KP3,
        Numpad0 => Key::KEY_KP0,
        NumpadDecimal => Key::KEY_KPDOT,
        Lang5 => Key::KEY_ZENKAKUHANKAKU, // Not Firefox, Not Safari
        IntlBackslash => Key::KEY_102ND,
        F11 => Key::KEY_F11,
        F12 => Key::KEY_F12,
        IntlRo => Key::KEY_RO,
        Lang3 => Key::KEY_KATAKANA, // Not Firefox, Not Safari
        Lang4 => Key::KEY_HIRAGANA, // Not Firefox, Not Safari
        Convert => Key::KEY_HENKAN,
        KanaMode => Key::KEY_KATAKANAHIRAGANA,
        NonConvert => Key::KEY_MUHENKAN,
        NumpadEnter => Key::KEY_KPENTER,
        ControlRight => Key::KEY_RIGHTCTRL,
        NumpadDivide => Key::KEY_KPSLASH,
        PrintScreen => Key::KEY_SYSRQ,
        AltRight => Key::KEY_RIGHTALT,
        Home => Key::KEY_HOME,
        ArrowUp => Key::KEY_UP,
        PageUp => Key::KEY_PAGEUP,
        ArrowLeft => Key::KEY_LEFT,
        ArrowRight => Key::KEY_RIGHT,
        End => Key::KEY_END,
        ArrowDown => Key::KEY_DOWN,
        PageDown => Key::KEY_PAGEDOWN,
        Insert => Key::KEY_INSERT,
        Delete => Key::KEY_DELETE,
        AudioVolumeMute => Key::KEY_MUTE,
        AudioVolumeDown => Key::KEY_VOLUMEDOWN,
        AudioVolumeUp => Key::KEY_VOLUMEUP,
        Power => Key::KEY_POWER, // Not Firefox, Not Safari
        NumpadEqual => Key::KEY_KPEQUAL,
        Pause => Key::KEY_PAUSE,
        ShowAllWindows => Key::KEY_SCALE, // Chrome only
        NumpadComma => Key::KEY_KPCOMMA,
        Lang1 => Key::KEY_HANGEUL,
        Lang2 => Key::KEY_HANJA,
        IntlYen => Key::KEY_YEN,
        MetaLeft => Key::KEY_LEFTMETA,
        MetaRight => Key::KEY_RIGHTMETA,
        ContextMenu => Key::KEY_COMPOSE,
        BrowserStop => Key::KEY_STOP,
        Again => Key::KEY_AGAIN,
        Props => Key::KEY_PROPS, // Not Chrome
        Undo => Key::KEY_UNDO,
        Select => Key::KEY_FRONT,
        Copy => Key::KEY_COPY,
        Open => Key::KEY_OPEN,
        Paste => Key::KEY_PASTE,
        Find => Key::KEY_FIND,
        Cut => Key::KEY_CUT,
        Help => Key::KEY_HELP,
        LaunchApp2 => Key::KEY_CALC,
        Sleep => Key::KEY_SLEEP, // Not Firefox, Not Safari
        WakeUp => Key::KEY_WAKEUP,
        LaunchApp1 => Key::KEY_FILE,
        LaunchMail => Key::KEY_MAIL,
        BrowserFavorites => Key::KEY_BOOKMARKS,
        BrowserBack => Key::KEY_BACK,
        BrowserForward => Key::KEY_FORWARD,
        Eject => Key::KEY_EJECTCD,
        MediaTrackNext => Key::KEY_NEXTSONG,
        MediaPlayPause => Key::KEY_PLAYPAUSE,
        MediaTrackPrevious => Key::KEY_PREVIOUSSONG,
        MediaStop => Key::KEY_STOPCD,
        MediaRecord => Key::KEY_RECORD, // Chrome only
        MediaRewind => Key::KEY_REWIND, // Chrome only
        MediaSelect => Key::KEY_CONFIG,
        BrowserHome => Key::KEY_HOMEPAGE,
        BrowserRefresh => Key::KEY_REFRESH,
        NumpadParenLeft => Key::KEY_KPLEFTPAREN, // Not Firefox, Not Safari
        NumpadParenRight => Key::KEY_KPRIGHTPAREN, // Not Firefox, Not Safari
        F13 => Key::KEY_F13,
        F14 => Key::KEY_F14,
        F15 => Key::KEY_F15,
        F16 => Key::KEY_F16,
        F17 => Key::KEY_F17,
        F18 => Key::KEY_F18,
        F19 => Key::KEY_F19,
        F20 => Key::KEY_F20,
        F21 => Key::KEY_F21,
        F22 => Key::KEY_F22,
        F23 => Key::KEY_F23,
        F24 => Key::KEY_F24,
        MediaPause => Key::KEY_PAUSECD,           // Chrome only
        MediaPlay => Key::KEY_PLAY,               // Chrome only
        MediaFastForward => Key::KEY_FASTFORWARD, // Chrome only
        BrowserSearch => Key::KEY_SEARCH,
        BrightnessDown => Key::KEY_BRIGHTNESSDOWN, // Chrome only
        BrightnessUp => Key::KEY_BRIGHTNESSUP,     // Chrome only
        DisplayToggleIntExt => Key::KEY_SWITCHVIDEOMODE, // Chrome only
        MailSend => Key::KEY_SEND,                 // Chrome only
        MailReply => Key::KEY_REPLY,               // Chrome only
        MailForward => Key::KEY_FORWARDMAIL,       // Chrome only
        MicrophoneMuteToggle => Key::KEY_MICMUTE,  // Chrome only
        ZoomToggle => Key::KEY_ZOOM,               // Chrome only
        LaunchControlPanel => Key::KEY_CONTROLPANEL, // Chrome only
        SelectTask => Key::KEY_APPSELECT,          // Chrome only
        LaunchScreenSaver => Key::KEY_SCREENSAVER, // Chrome only
        LaunchAssistant => Key::KEY_ASSISTANT,     // Chrome only
        KeyboardLayoutSelect => Key::KEY_KBD_LAYOUT_NEXT, // Chrome only
        PrivacyScreenToggle => Key::KEY_PRIVACY_SCREEN_TOGGLE, // Chrome only

        // In addition evdev supports gamepads. So we base this off the
        // "Standard Gamepad" defined here:
        // https://w3c.github.io/gamepad/#dfn-standard-gamepad
        // And here the buttons this maps to:
        // https://www.kernel.org/doc/html/v4.12/input/gamepad.html#geometry
        // Though the naming isn't fully the same, so we somewhat based it off
        // gilrs:
        // https://gitlab.com/gilrs-project/gilrs/-/blob/60883ea0f1b95b66e4ae1e00e5b7366cc605068e/gilrs-core/src/platform/wasm/gamepad.rs#L349-367
        Gamepad0 => Key::BTN_SOUTH,
        Gamepad1 => Key::BTN_EAST,
        Gamepad2 => Key::BTN_WEST,
        Gamepad3 => Key::BTN_NORTH,
        Gamepad4 => Key::BTN_TL,
        Gamepad5 => Key::BTN_TR,
        Gamepad6 => Key::BTN_TL2,
        Gamepad7 => Key::BTN_TR2,
        Gamepad8 => Key::BTN_SELECT,
        Gamepad9 => Key::BTN_START,
        Gamepad10 => Key::BTN_THUMBL,
        Gamepad11 => Key::BTN_THUMBR,
        Gamepad12 => Key::BTN_DPAD_UP,
        Gamepad13 => Key::BTN_DPAD_DOWN,
        Gamepad14 => Key::BTN_DPAD_LEFT,
        Gamepad15 => Key::BTN_DPAD_RIGHT,
        Gamepad16 => Key::BTN_MODE,
        _ => return None,
    })
}

pub fn new() -> Result<Hook> {
    let (sender, receiver) = channel();
    let mut poll = Poll::new().map_err(|_| Error::EPoll)?;
    let waker = Waker::new(poll.registry(), PING_TOKEN).map_err(|_| Error::EPoll)?;

    let mut devices: Vec<Device> = evdev::enumerate()
        .map(|(_, d)| d)
        .filter(|d| d.supported_events().contains(EventType::KEY))
        .collect();

    for (i, fd) in devices.iter().enumerate() {
        poll.registry()
            .register(&mut SourceFd(&fd.as_raw_fd()), Token(i), Interest::READABLE)
            .map_err(|_| Error::EPoll)?;
    }

    let join_handle = thread::spawn(move || -> Result<()> {
        let mut result = Ok(());
        let mut events = Events::with_capacity(1024);
        let mut hotkeys: HashMap<(Key, Modifiers), Box<dyn FnMut() + Send>> = HashMap::new();
        let mut modifiers = Modifiers::empty();

        let (mut xlib, mut display) = (None, None);

        'event_loop: loop {
            if poll.poll(&mut events, None).is_err() {
                result = Err(Error::EPoll);
                break 'event_loop;
            }

            for mio_event in &events {
                if mio_event.token().0 < devices.len() {
                    let idx = mio_event.token().0;
                    for ev in devices[idx].fetch_events().map_err(|_| Error::EvDev)? {
                        if let InputEventKind::Key(k) = ev.kind() {
                            const RELEASED: i32 = 0;
                            const PRESSED: i32 = 1;
                            match ev.value() {
                                PRESSED => {
                                    if let Some(callback) = hotkeys.get_mut(&(k, modifiers)) {
                                        callback();
                                    }
                                    match k {
                                        Key::KEY_LEFTALT | Key::KEY_RIGHTALT => {
                                            modifiers.insert(Modifiers::ALT);
                                        }
                                        Key::KEY_LEFTCTRL | Key::KEY_RIGHTCTRL => {
                                            modifiers.insert(Modifiers::CONTROL);
                                        }
                                        Key::KEY_LEFTMETA | Key::KEY_RIGHTMETA => {
                                            modifiers.insert(Modifiers::META);
                                        }
                                        Key::KEY_LEFTSHIFT | Key::KEY_RIGHTSHIFT => {
                                            modifiers.insert(Modifiers::SHIFT);
                                        }
                                        _ => {}
                                    }
                                }
                                RELEASED => match k {
                                    Key::KEY_LEFTALT | Key::KEY_RIGHTALT => {
                                        modifiers.remove(Modifiers::ALT);
                                    }
                                    Key::KEY_LEFTCTRL | Key::KEY_RIGHTCTRL => {
                                        modifiers.remove(Modifiers::CONTROL);
                                    }
                                    Key::KEY_LEFTMETA | Key::KEY_RIGHTMETA => {
                                        modifiers.remove(Modifiers::META);
                                    }
                                    Key::KEY_LEFTSHIFT | Key::KEY_RIGHTSHIFT => {
                                        modifiers.remove(Modifiers::SHIFT);
                                    }
                                    _ => {}
                                },
                                _ => {} // Ignore repeating
                            }
                        }
                    }
                } else if mio_event.token() == PING_TOKEN {
                    for message in receiver.try_iter() {
                        match message {
                            Message::Register(key, callback, promise) => {
                                promise.set(
                                    if code_for(key.key_code)
                                        .and_then(|k| hotkeys.insert((k, key.modifiers), callback))
                                        .is_some()
                                    {
                                        Err(Error::AlreadyRegistered)
                                    } else {
                                        Ok(())
                                    },
                                );
                            }
                            Message::Unregister(key, promise) => promise.set(
                                code_for(key.key_code)
                                    .and_then(|k| hotkeys.remove(&(k, key.modifiers)).map(drop))
                                    .ok_or(Error::NotRegistered),
                            ),
                            Message::Resolve(key_code, promise) => {
                                promise.set(resolve(&mut xlib, &mut display, key_code))
                            }
                            Message::End => {
                                break 'event_loop;
                            }
                        }
                    }
                }
            }
        }

        if let (Some(xlib), Some(display)) = (xlib, display) {
            unsafe { (xlib.XCloseDisplay)(display) };
        }

        result
    });

    Ok(Hook {
        sender,
        waker,
        join_handle: Some(join_handle),
    })
}

fn resolve(
    xlib: &mut Option<Xlib>,
    display: &mut Option<*mut _XDisplay>,
    key_code: KeyCode,
) -> Option<char> {
    if xlib.is_none() {
        *xlib = Xlib::open().ok();
    }
    let xlib = xlib.as_ref()?;
    if display.is_none() {
        unsafe {
            let result = (xlib.XOpenDisplay)(ptr::null());
            if !result.is_null() {
                *display = Some(result);
            }
        }
    }
    let xdisplay = (*display)?;
    x11_impl::resolve(xlib, xdisplay, key_code)
}
