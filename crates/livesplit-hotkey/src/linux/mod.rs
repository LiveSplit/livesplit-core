mod key_code;
pub use self::key_code::KeyCode;

use mio::unix::EventedFd;
use mio::{Events, Poll, PollOpt, Ready, Registration, SetReadiness, Token};
use promising_future::{future_promise, Promise};
use std::collections::hash_map::{Entry, HashMap};
use std::os::raw::{c_int, c_uint, c_ulong};
use std::sync::mpsc::{channel, Sender};
use std::thread::{self, JoinHandle};
use std::{mem, ptr};
use x11_dl::xlib::{
    Display, GrabModeAsync, KeyPress, KeyPressMask, Mod2Mask, XErrorEvent, XKeyEvent, Xlib,
};

#[derive(Debug, snafu::Snafu)]
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

pub struct Hook {
    sender: Sender<Message>,
    ping: SetReadiness,
    _registration: Registration,
    join_handle: Option<JoinHandle<Result<()>>>,
}

impl Drop for Hook {
    fn drop(&mut self) {
        self.sender.send(Message::End).ok();
        self.ping.set_readiness(Ready::readable()).ok();
        if let Some(handle) = self.join_handle.take() {
            handle.join().ok();
        }
    }
}

unsafe fn unregister(xlib: &Xlib, display: *mut Display, window: c_ulong, code: c_uint) {
    (xlib.XUngrabKey)(display, code as _, 0, window);
    (xlib.XUngrabKey)(display, code as _, Mod2Mask, window);
}

unsafe extern "C" fn handle_error(_: *mut Display, _: *mut XErrorEvent) -> c_int {
    0
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

            let window = (xlib.XDefaultRootWindow)(display);
            (xlib.XSelectInput)(display, window, KeyPressMask);

            let fd = (xlib.XConnectionNumber)(display);
            let poll = Poll::new().map_err(|_| Error::EPoll)?;

            let (registration, ping) = Registration::new2();

            const X_TOKEN: Token = Token(0);
            const PING_TOKEN: Token = Token(1);

            poll.register(
                &EventedFd(&fd),
                X_TOKEN,
                Ready::readable() | Ready::writable(),
                PollOpt::edge(),
            )
            .map_err(|_| Error::EPoll)?;

            poll.register(
                &registration,
                PING_TOKEN,
                Ready::readable(),
                PollOpt::edge(),
            )
            .map_err(|_| Error::EPoll)?;

            struct XData(Xlib, *mut Display, c_ulong);
            unsafe impl Send for XData {}
            let xdata = XData(xlib, display, window);

            let join_handle = thread::spawn(move || -> Result<()> {
                let XData(xlib, display, window) = xdata;

                let mut result = Ok(());
                let mut events = Events::with_capacity(1024);
                let mut hotkeys = HashMap::new();

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
                                        let code =
                                            (xlib.XKeysymToKeycode)(display, key as _) as c_uint;

                                        if let Entry::Vacant(vacant) = hotkeys.entry(code) {
                                            (xlib.XGrabKey)(
                                                display,
                                                code as _,
                                                0,
                                                window,
                                                false as _,
                                                GrabModeAsync,
                                                GrabModeAsync,
                                            );

                                            (xlib.XGrabKey)(
                                                display,
                                                code as _,
                                                Mod2Mask,
                                                window,
                                                false as _,
                                                GrabModeAsync,
                                                GrabModeAsync,
                                            );

                                            vacant.insert(callback);
                                            promise.set(Ok(()));
                                        } else {
                                            promise.set(Err(Error::AlreadyRegistered));
                                        }
                                    }
                                    Message::Unregister(key, promise) => {
                                        let code =
                                            (xlib.XKeysymToKeycode)(display, key as _) as c_uint;

                                        if hotkeys.remove(&code).is_some() {
                                            unregister(&xlib, display, window, code);
                                            promise.set(Ok(()));
                                        } else {
                                            promise.set(Err(Error::NotRegistered));
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
                                }
                            }
                        }
                    }
                }

                for (code, _) in hotkeys {
                    unregister(&xlib, display, window, code);
                }

                (xlib.XCloseDisplay)(display);

                result
            });

            Ok(Hook {
                sender: sender,
                ping: ping,
                _registration: registration,
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

        self.ping
            .set_readiness(Ready::readable())
            .map_err(|_| Error::ThreadStopped)?;

        future.value().ok_or(Error::ThreadStopped)?
    }

    pub fn unregister(&self, hotkey: KeyCode) -> Result<()> {
        let (future, promise) = future_promise();

        self.sender
            .send(Message::Unregister(hotkey, promise))
            .map_err(|_| Error::ThreadStopped)?;
        self.ping
            .set_readiness(Ready::readable())
            .map_err(|_| Error::ThreadStopped)?;

        future.value().ok_or(Error::ThreadStopped)?
    }
}

#[test]
fn test() {
    let hook = Hook::new().unwrap();
    hook.register(KeyCode::NumPad0, || println!("A")).unwrap();
    thread::sleep(std::time::Duration::from_secs(5));
    hook.unregister(KeyCode::NumPad0).unwrap();
    hook.register(KeyCode::NumPad1, || println!("B")).unwrap();
    thread::sleep(std::time::Duration::from_secs(5));
}
