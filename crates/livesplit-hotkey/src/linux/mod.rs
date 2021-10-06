use crate::KeyCode;
use mio::{unix::SourceFd, Events, Interest, Poll, Token, Waker};
use promising_future::{future_promise, Promise};
use std::{
    collections::hash_map::{Entry, HashMap},
    mem,
    os::raw::{c_int, c_uint},
    ptr,
    sync::mpsc::{channel, Sender},
    thread::{self, JoinHandle},
};
use x11_dl::xlib::{
    AnyKey, AnyModifier, Display, GrabModeAsync, KeyPress, XErrorEvent, XKeyEvent, Xlib,
};

#[derive(Debug, Copy, Clone, snafu::Snafu)]
pub enum Error {
    NoXLib,
    OpenXServerConnection,
    EPoll,
    ThreadStopped,
    AlreadyRegistered,
    NotRegistered,
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

const X_TOKEN: Token = Token(0);
const PING_TOKEN: Token = Token(1);

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

unsafe fn ungrab_all(xlib: &Xlib, display: *mut Display) {
    let screencount = (xlib.XScreenCount)(display);
    for screen in 0..screencount {
        let rootwindow = (xlib.XRootWindow)(display, screen);
        for _i in 0..rootwindow {
            // FIXME: This loop looks very stupid, but it somehow it prevents
            // button presses getting lost.
            (xlib.XUngrabKey)(display, AnyKey, AnyModifier, rootwindow);
        }
    }
}

unsafe fn grab_all(xlib: &Xlib, display: *mut Display, keylist: &[c_uint]) {
    ungrab_all(xlib, display);
    let screencount = (xlib.XScreenCount)(display);
    for screen in 0..screencount {
        let rootwindow = (xlib.XRootWindow)(display, screen);
        for &code in keylist {
            (xlib.XGrabKey)(
                display,
                code as c_int,
                AnyModifier,
                rootwindow,
                false as _,
                GrabModeAsync,
                GrabModeAsync,
            );
        }
    }
}

unsafe extern "C" fn handle_error(_: *mut Display, _: *mut XErrorEvent) -> c_int {
    0
}

fn code_for(key: KeyCode) -> Option<c_uint> {
    use self::KeyCode::*;
    Some(match key {
        Escape => 0x0009,
        Digit1 => 0x000A,
        Digit2 => 0x000B,
        Digit3 => 0x000C,
        Digit4 => 0x000D,
        Digit5 => 0x000E,
        Digit6 => 0x000F,
        Digit7 => 0x0010,
        Digit8 => 0x0011,
        Digit9 => 0x0012,
        Digit0 => 0x0013,
        Minus => 0x0014,
        Equal => 0x0015,
        Backspace => 0x0016,
        Tab => 0x0017,
        KeyQ => 0x0018,
        KeyW => 0x0019,
        KeyE => 0x001A,
        KeyR => 0x001B,
        KeyT => 0x001C,
        KeyY => 0x001D,
        KeyU => 0x001E,
        KeyI => 0x001F,
        KeyO => 0x0020,
        KeyP => 0x0021,
        BracketLeft => 0x0022,
        BracketRight => 0x0023,
        Enter => 0x0024,
        ControlLeft => 0x0025,
        KeyA => 0x0026,
        KeyS => 0x0027,
        KeyD => 0x0028,
        KeyF => 0x0029,
        KeyG => 0x002A,
        KeyH => 0x002B,
        KeyJ => 0x002C,
        KeyK => 0x002D,
        KeyL => 0x002E,
        Semicolon => 0x002F,
        Quote => 0x0030,
        Backquote => 0x0031,
        ShiftLeft => 0x0032,
        Backslash => 0x0033,
        KeyZ => 0x0034,
        KeyX => 0x0035,
        KeyC => 0x0036,
        KeyV => 0x0037,
        KeyB => 0x0038,
        KeyN => 0x0039,
        KeyM => 0x003A,
        Comma => 0x003B,
        Period => 0x003C,
        Slash => 0x003D,
        ShiftRight => 0x003E,
        NumpadMultiply => 0x003F,
        AltLeft => 0x0040,
        Space => 0x0041,
        CapsLock => 0x0042,
        F1 => 0x0043,
        F2 => 0x0044,
        F3 => 0x0045,
        F4 => 0x0046,
        F5 => 0x0047,
        F6 => 0x0048,
        F7 => 0x0049,
        F8 => 0x004A,
        F9 => 0x004B,
        F10 => 0x004C,
        NumLock => 0x004D,
        ScrollLock => 0x004E,
        Numpad7 => 0x004F,
        Numpad8 => 0x0050,
        Numpad9 => 0x0051,
        NumpadSubtract => 0x0052,
        Numpad4 => 0x0053,
        Numpad5 => 0x0054,
        Numpad6 => 0x0055,
        NumpadAdd => 0x0056,
        Numpad1 => 0x0057,
        Numpad2 => 0x0058,
        Numpad3 => 0x0059,
        Numpad0 => 0x005A,
        NumpadDecimal => 0x005B,
        Lang5 => 0x005D, // Not Firefox, Not Safari
        IntlBackslash => 0x005E,
        F11 => 0x005F,
        F12 => 0x0060,
        IntlRo => 0x0061,
        Lang3 => 0x0062, // Not Firefox, Not Safari
        Lang4 => 0x0063, // Not Firefox, Not Safari
        Convert => 0x0064,
        KanaMode => 0x0065,
        NonConvert => 0x0066,
        NumpadEnter => 0x0068,
        ControlRight => 0x0069,
        NumpadDivide => 0x006A,
        PrintScreen => 0x006B,
        AltRight => 0x006C,
        Home => 0x006E,
        ArrowUp => 0x006F,
        PageUp => 0x0070,
        ArrowLeft => 0x0071,
        ArrowRight => 0x0072,
        End => 0x0073,
        ArrowDown => 0x0074,
        PageDown => 0x0075,
        Insert => 0x0076,
        Delete => 0x0077,
        AudioVolumeMute => 0x0079,
        AudioVolumeDown => 0x007A,
        AudioVolumeUp => 0x007B,
        Power => 0x007C, // Not Firefox, Not Safari
        NumpadEqual => 0x007D,
        Pause => 0x007F,
        ShowAllWindows => 0x0080, // Chrome only
        NumpadComma => 0x0081,
        Lang1 => 0x0082,
        Lang2 => 0x0083,
        IntlYen => 0x0084,
        MetaLeft => 0x0085,
        MetaRight => 0x0086,
        ContextMenu => 0x0087,
        BrowserStop => 0x0088,
        Again => 0x0089,
        Props => 0x008A, // Not Chrome
        Undo => 0x008B,
        Select => 0x008C,
        Copy => 0x008D,
        Open => 0x008E,
        Paste => 0x008F,
        Find => 0x0090,
        Cut => 0x0091,
        Help => 0x0092,
        LaunchApp2 => 0x0094,
        Sleep => 0x0096, // Not Firefox, Not Safari
        WakeUp => 0x0097,
        LaunchApp1 => 0x0098,
        LaunchMail => 0x00A3,
        BrowserFavorites => 0x00A4,
        BrowserBack => 0x00A6,
        BrowserForward => 0x00A7,
        Eject => 0x00A9,
        MediaTrackNext => 0x00AB,
        MediaPlayPause => 0x00AC,
        MediaTrackPrevious => 0x00AD,
        MediaStop => 0x00AE,
        MediaRecord => 0x00AF, // Chrome only
        MediaRewind => 0x00B0, // Chrome only
        MediaSelect => 0x00B3,
        BrowserHome => 0x00B4,
        BrowserRefresh => 0x00B5,
        NumpadParenLeft => 0x00BB,  // Not Firefox, Not Safari
        NumpadParenRight => 0x00BC, // Not Firefox, Not Safari
        F13 => 0x00BF,
        F14 => 0x00C0,
        F15 => 0x00C1,
        F16 => 0x00C2,
        F17 => 0x00C3,
        F18 => 0x00C4,
        F19 => 0x00C5,
        F20 => 0x00C6,
        F21 => 0x00C7,
        F22 => 0x00C8,
        F23 => 0x00C9,
        F24 => 0x00CA,
        MediaPause => 0x00D1,       // Chrome only
        MediaPlay => 0x00D7,        // Chrome only
        MediaFastForward => 0x00D8, // Chrome only
        BrowserSearch => 0x00E1,
        BrightnessDown => 0x00E8,       // Chrome only
        BrightnessUp => 0x00E9,         // Chrome only
        DisplayToggleIntExt => 0x00EB,  // Chrome only
        MailSend => 0x00EF,             // Chrome only
        MailReply => 0x00F0,            // Chrome only
        MailForward => 0x00F1,          // Chrome only
        ZoomToggle => 0x017C,           // Chrome only
        LaunchControlPanel => 0x024B,   // Chrome only
        SelectTask => 0x024C,           // Chrome only
        LaunchScreenSaver => 0x024D,    // Chrome only
        LaunchAssistant => 0x024F,      // Chrome only
        KeyboardLayoutSelect => 0x0250, // Chrome only
        PrivacyScreenToggle => 0x0281,  // Chrome only
        _ => return None,
    })
}

impl Hook {
    pub fn new() -> Result<Self> {
        unsafe {
            let (sender, receiver) = channel();

            let xlib = Xlib::open().map_err(|_| Error::NoXLib)?;
            (xlib.XSetErrorHandler)(Some(handle_error));

            let display = (xlib.XOpenDisplay)(ptr::null());
            if display.is_null() {
                return Err(Error::OpenXServerConnection);
            }

            let fd = (xlib.XConnectionNumber)(display) as std::os::unix::io::RawFd;
            let mut poll = Poll::new().map_err(|_| Error::EPoll)?;

            let waker = Waker::new(poll.registry(), PING_TOKEN).map_err(|_| Error::EPoll)?;

            poll.registry()
                .register(
                    &mut SourceFd(&fd),
                    X_TOKEN,
                    Interest::READABLE | Interest::WRITABLE,
                )
                .map_err(|_| Error::EPoll)?;

            struct XData(Xlib, *mut Display);
            unsafe impl Send for XData {}
            let xdata = XData(xlib, display);

            let join_handle = thread::spawn(move || -> Result<()> {
                let XData(xlib, display) = xdata;

                let mut result = Ok(());
                let mut events = Events::with_capacity(1024);
                let mut hotkeys = HashMap::new();

                // For some reason we need to call this once for any KeyGrabs to
                // actually do anything.
                (xlib.XKeysymToKeycode)(display, 0);

                'event_loop: loop {
                    if poll.poll(&mut events, None).is_err() {
                        result = Err(Error::EPoll);
                        break 'event_loop;
                    }

                    for mio_event in &events {
                        if mio_event.token() == PING_TOKEN {
                            for message in receiver.try_iter() {
                                match message {
                                    Message::Register(key, callback, promise) => {
                                        if let Some(code) = code_for(key) {
                                            if let Entry::Vacant(vacant) = hotkeys.entry(code) {
                                                vacant.insert(callback);
                                                promise.set(Ok(()));
                                            } else {
                                                promise.set(Err(Error::AlreadyRegistered));
                                            }
                                            let keys = hotkeys.keys().copied().collect::<Vec<_>>();
                                            grab_all(&xlib, display, &keys);
                                        } else {
                                            promise.set(Ok(()));
                                        }
                                    }
                                    Message::Unregister(key, promise) => {
                                        if let Some(code) = code_for(key) {
                                            if hotkeys.remove(&code).is_some() {
                                                promise.set(Ok(()));
                                            } else {
                                                promise.set(Err(Error::NotRegistered));
                                            }
                                            let keys = hotkeys.keys().copied().collect::<Vec<_>>();
                                            grab_all(&xlib, display, &keys);
                                        } else {
                                            promise.set(Ok(()));
                                        }
                                    }
                                    Message::End => {
                                        break 'event_loop;
                                    }
                                }
                            }
                        } else if mio_event.token() == X_TOKEN {
                            while (xlib.XPending)(display) != 0 {
                                let mut event = mem::MaybeUninit::uninit();
                                (xlib.XNextEvent)(display, event.as_mut_ptr());
                                let event = event.assume_init();
                                if event.get_type() == KeyPress {
                                    let event: &XKeyEvent = event.as_ref();
                                    if let Some(callback) = hotkeys.get_mut(&event.keycode) {
                                        callback();
                                    }
                                    // FIXME: We should check else here: these amount to lost
                                    // keypresses.
                                }
                            }
                        }
                    }
                }

                ungrab_all(&xlib, display);

                (xlib.XCloseDisplay)(display);

                result
            });

            Ok(Hook {
                sender,
                waker,
                join_handle: Some(join_handle),
            })
        }
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
