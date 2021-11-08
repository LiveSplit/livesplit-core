use crate::KeyCode;
use evdev::{self, Device, EventType, InputEventKind, Key};
use mio::{unix::SourceFd, Events, Interest, Poll, Token, Waker};
use promising_future::{future_promise, Promise};
use std::{
    collections::hash_map::HashMap,
    convert::{TryFrom, TryInto},
    os::unix::prelude::{AsRawFd, RawFd},
    sync::mpsc::{channel, Sender},
    thread::{self, JoinHandle},
};

#[derive(Debug, Copy, Clone, snafu::Snafu)]
pub enum Error {
    EPoll,
    ThreadStopped,
    AlreadyRegistered,
    NotRegistered,
    UnknownKey,
    EvDev,
}

pub type Result<T> = std::result::Result<T, Error>;

enum Message {
    Register(
        KeyCode,
        Box<dyn FnMut() + Send + 'static>,
        Promise<Result<()>>,
    ),
    Unregister(KeyCode, Promise<Result<()>>),
    End,
}

// Low numbered tokens are allocated to devices
const PING_TOKEN: Token = Token(256);

pub struct Hook {
    sender: Sender<Message>,
    waker: Waker,
    join_handle: Option<JoinHandle<Result<()>>>,
}

impl Drop for Hook {
    fn drop(&mut self) {
        self.sender.send(Message::End).ok();
        self.waker.wake().ok();
        if let Some(handle) = self.join_handle.take() {
            handle.join().ok();
        }
    }
}

impl Hook {
    pub fn new() -> Result<Self> {
        let (sender, receiver) = channel();
        let mut poll = Poll::new().map_err(|_| Error::EPoll)?;
        let waker = Waker::new(poll.registry(), PING_TOKEN).map_err(|_| Error::EPoll)?;

        let mut devices: Vec<Device> = evdev::enumerate()
            .filter(|d| d.supported_events().contains(EventType::KEY))
            .collect();
        let fds: Vec<RawFd> = devices.iter().map(|d| d.as_raw_fd()).collect();

        for (i, fd) in fds.iter().enumerate() {
            poll.registry()
                .register(&mut SourceFd(fd), Token(i), Interest::READABLE)
                .map_err(|_| Error::EPoll)?;
        }

        let join_handle = thread::spawn(move || -> Result<()> {
            let mut result = Ok(());
            let mut events = Events::with_capacity(1024);
            let mut hotkeys: HashMap<Key, Box<dyn FnMut() + Send>> = HashMap::new();

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
                                // println!("{:?} - {}", k, ev.value());
                                if ev.value() != 0 {
                                    if let Some(callback) = hotkeys.get_mut(&k) {
                                        callback();
                                    }
                                }
                            }
                        }
                    } else if mio_event.token() == PING_TOKEN {
                        for message in receiver.try_iter() {
                            match message {
                                Message::Register(key, callback, promise) => {
                                    promise.set(key.try_into().and_then(|k| {
                                        if hotkeys.insert(k, callback).is_some() {
                                            Err(Error::AlreadyRegistered)
                                        } else {
                                            Ok(())
                                        }
                                    }))
                                }
                                Message::Unregister(key, promise) => promise.set(
                                    key.try_into()
                                        .and_then(|k| {
                                            hotkeys.remove(&k).ok_or(Error::NotRegistered)
                                        })
                                        .and(Ok(())),
                                ),
                                Message::End => {
                                    break 'event_loop;
                                }
                            }
                        }
                    }
                }
            }
            result
        });

        Ok(Hook {
            sender,
            waker,
            join_handle: Some(join_handle),
        })
    }

    pub fn register<F>(&self, hotkey: KeyCode, callback: F) -> Result<()>
    where
        F: FnMut() + Send + 'static,
    {
        let (future, promise) = future_promise();

        self.sender
            .send(Message::Register(hotkey, Box::new(callback), promise))
            .map_err(|_| Error::ThreadStopped)?;

        self.waker.wake().map_err(|_| Error::ThreadStopped)?;

        future.value().ok_or(Error::ThreadStopped)?
    }

    pub fn unregister(&self, hotkey: KeyCode) -> Result<()> {
        let (future, promise) = future_promise();

        self.sender
            .send(Message::Unregister(hotkey, promise))
            .map_err(|_| Error::ThreadStopped)?;

        self.waker.wake().map_err(|_| Error::ThreadStopped)?;

        future.value().ok_or(Error::ThreadStopped)?
    }
}

pub(crate) fn try_resolve(_key_code: KeyCode) -> Option<String> {
    None
}

impl TryFrom<KeyCode> for Key {
    type Error = Error;
    fn try_from(k: KeyCode) -> Result<Self> {
        use self::KeyCode::*;
        Ok(match k {
            Again => Key::KEY_AGAIN,
            AltLeft => Key::KEY_LEFTALT,
            AltRight => Key::KEY_RIGHTALT,
            ArrowDown => Key::KEY_DOWN,
            ArrowLeft => Key::KEY_LEFT,
            ArrowRight => Key::KEY_RIGHT,
            ArrowUp => Key::KEY_UP,
            AudioVolumeDown => Key::KEY_VOLUMEUP,
            AudioVolumeMute => Key::KEY_MUTE,
            AudioVolumeUp => Key::KEY_VOLUMEDOWN,
            Backquote => Key::KEY_GRAVE,
            Backslash => Key::KEY_BACKSLASH,
            Backspace => Key::KEY_BACKSPACE,
            BracketLeft => Key::KEY_LEFTBRACE,
            BracketRight => Key::KEY_RIGHTBRACE,
            BrightnessDown => Key::KEY_BRIGHTNESSDOWN,
            BrightnessUp => Key::KEY_BRIGHTNESSUP,
            BrowserBack => Key::KEY_BACK,
            BrowserFavorites => Key::KEY_FAVORITES,
            BrowserForward => Key::KEY_FORWARD,
            BrowserHome => Key::KEY_HOMEPAGE,
            BrowserRefresh => Key::KEY_REFRESH,
            BrowserSearch => Key::KEY_SEARCH,
            BrowserStop => Key::KEY_STOP,
            CapsLock => Key::KEY_CAPSLOCK,
            Comma => Key::KEY_COMMA,
            ContextMenu => Key::KEY_CONTEXT_MENU,
            ControlLeft => Key::KEY_LEFTCTRL,
            ControlRight => Key::KEY_RIGHTCTRL,
            Convert => Key::KEY_KATAKANA,
            Copy => Key::KEY_COPY,
            Cut => Key::KEY_CUT,
            Delete => Key::KEY_DELETE,
            Digit0 => Key::KEY_0,
            Digit1 => Key::KEY_1,
            Digit2 => Key::KEY_2,
            Digit3 => Key::KEY_3,
            Digit4 => Key::KEY_4,
            Digit5 => Key::KEY_5,
            Digit6 => Key::KEY_6,
            Digit7 => Key::KEY_7,
            Digit8 => Key::KEY_8,
            Digit9 => Key::KEY_9,
            DisplayToggleIntExt => Key::KEY_DISPLAYTOGGLE,
            Eject => Key::KEY_EJECTCD,
            End => Key::KEY_END,
            Enter => Key::KEY_ENTER,
            Equal => Key::KEY_EQUAL,
            Escape => Key::KEY_ESC,
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
            F11 => Key::KEY_F11,
            F12 => Key::KEY_F12,
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
            Find => Key::KEY_FIND,
            Fn => Key::KEY_FN,
            Gamepad0 => Key::BTN_SOUTH,
            Gamepad1 => Key::BTN_EAST,
            Gamepad2 => Key::BTN_C,
            Gamepad3 => Key::BTN_NORTH,
            Gamepad4 => Key::BTN_WEST,
            Gamepad5 => Key::BTN_Z,
            Gamepad6 => Key::BTN_TL,
            Gamepad7 => Key::BTN_TR,
            Gamepad8 => Key::BTN_TL2,
            Gamepad9 => Key::BTN_TR2,
            Gamepad10 => Key::BTN_SELECT,
            Gamepad11 => Key::BTN_START,
            Gamepad12 => Key::BTN_MODE,
            Gamepad13 => Key::BTN_THUMBL,
            Help => Key::KEY_HELP,
            Home => Key::KEY_HOME,
            Insert => Key::KEY_INSERT,
            IntlYen => Key::KEY_YEN,
            KanaMode => Key::KEY_KATAKANAHIRAGANA,
            KeyA => Key::KEY_A,
            KeyB => Key::KEY_B,
            KeyC => Key::KEY_C,
            KeyD => Key::KEY_D,
            KeyE => Key::KEY_E,
            KeyF => Key::KEY_F,
            KeyG => Key::KEY_G,
            KeyH => Key::KEY_H,
            KeyI => Key::KEY_I,
            KeyJ => Key::KEY_J,
            KeyK => Key::KEY_K,
            KeyL => Key::KEY_L,
            KeyM => Key::KEY_M,
            KeyN => Key::KEY_N,
            KeyO => Key::KEY_O,
            KeyP => Key::KEY_P,
            KeyQ => Key::KEY_Q,
            KeyR => Key::KEY_R,
            KeyS => Key::KEY_S,
            KeyT => Key::KEY_T,
            KeyU => Key::KEY_U,
            KeyV => Key::KEY_V,
            KeyW => Key::KEY_W,
            KeyX => Key::KEY_X,
            KeyY => Key::KEY_Y,
            KeyZ => Key::KEY_Z,
            KeyboardLayoutSelect => Key::KEY_KBD_LAYOUT_NEXT,
            Lang1 => Key::KEY_LANGUAGE,
            LaunchAssistant => Key::KEY_ASSISTANT,
            LaunchControlPanel => Key::KEY_CONTROLPANEL,
            LaunchMail => Key::KEY_MAIL,
            LaunchScreenSaver => Key::KEY_SCREENSAVER,
            MailForward => Key::KEY_FORWARDMAIL,
            MailReply => Key::KEY_REPLY,
            MailSend => Key::KEY_SEND,
            MediaFastForward => Key::KEY_FASTFORWARD,
            MediaPause => Key::KEY_PAUSE,
            MediaPlay => Key::KEY_PLAY,
            MediaPlayPause => Key::KEY_PLAYPAUSE,
            MediaRecord => Key::KEY_RECORD,
            MediaRewind => Key::KEY_REWIND,
            MediaSelect => Key::KEY_SELECT,
            MediaStop => Key::KEY_STOPCD,
            MediaTrackNext => Key::KEY_NEXTSONG,
            MediaTrackPrevious => Key::KEY_PREVIOUSSONG,
            MetaLeft => Key::KEY_LEFTMETA,
            MetaRight => Key::KEY_RIGHTMETA,
            Minus => Key::KEY_MINUS,
            NumLock => Key::KEY_NUMLOCK,
            Numpad0 => Key::KEY_NUMERIC_0,
            Numpad1 => Key::KEY_NUMERIC_1,
            Numpad2 => Key::KEY_NUMERIC_2,
            Numpad3 => Key::KEY_NUMERIC_3,
            Numpad4 => Key::KEY_NUMERIC_4,
            Numpad5 => Key::KEY_NUMERIC_5,
            Numpad6 => Key::KEY_NUMERIC_6,
            Numpad7 => Key::KEY_NUMERIC_7,
            Numpad8 => Key::KEY_NUMERIC_8,
            Numpad9 => Key::KEY_NUMERIC_9,
            NumpadAdd => Key::KEY_KPPLUS,
            NumpadComma => Key::KEY_KPCOMMA,
            NumpadDecimal => Key::KEY_KPDOT,
            NumpadDivide => Key::KEY_KPSLASH,
            NumpadEnter => Key::KEY_KPENTER,
            NumpadEqual => Key::KEY_KPEQUAL,
            NumpadHash => Key::KEY_NUMERIC_POUND,
            NumpadParenLeft => Key::KEY_KPLEFTPAREN,
            NumpadParenRight => Key::KEY_KPRIGHTPAREN,
            NumpadStar => Key::KEY_NUMERIC_STAR,
            NumpadSubtract => Key::KEY_KPMINUS,
            Open => Key::KEY_OPEN,
            PageDown => Key::KEY_PAGEDOWN,
            PageUp => Key::KEY_PAGEUP,
            Paste => Key::KEY_PASTE,
            Pause => Key::KEY_PAUSE,
            Period => Key::KEY_DOT,
            Power => Key::KEY_POWER,
            PrintScreen => Key::KEY_PRINT,
            PrivacyScreenToggle => Key::KEY_PRIVACY_SCREEN_TOGGLE,
            Props => Key::KEY_PROPS,
            Quote => Key::KEY_APOSTROPHE,
            ScrollLock => Key::KEY_SCROLLLOCK,
            Select => Key::KEY_SELECT,
            Semicolon => Key::KEY_SEMICOLON,
            ShiftLeft => Key::KEY_LEFTSHIFT,
            ShiftRight => Key::KEY_RIGHTSHIFT,
            ShowAllWindows => Key::KEY_CYCLEWINDOWS,
            Slash => Key::KEY_SLASH,
            Sleep => Key::KEY_SLEEP,
            Space => Key::KEY_SPACE,
            Tab => Key::KEY_TAB,
            Undo => Key::KEY_UNDO,
            WakeUp => Key::KEY_WAKEUP,
            ZoomToggle => Key::KEY_ZOOM,
            // TODO: test on a gamepad with 20 buttons to see what the higher indices are
            Gamepad14 | Gamepad15 | Gamepad16 | Gamepad17 | Gamepad18 | Gamepad19 => {
                return Err(Error::UnknownKey)
            }
            // TODO: find a keyboard with these keys and see which they correspond to
            SelectTask | IntlBackslash | IntlRo | Lang2 | Lang3 | Lang4 | Lang5
            | NonConvert | FnLock | LaunchApp1 | LaunchApp2 | NumpadBackspace
            | NumpadClear | NumpadClearEntry | NumpadMemoryAdd | NumpadMemoryClear
            | NumpadMemoryRecall | NumpadMemoryStore | NumpadMemorySubtract
            | NumpadMultiply => return Err(Error::UnknownKey),
        })
    }
}
