mod key_code;
pub use self::key_code::KeyCode;

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

unsafe fn grab_all(xlib: &Xlib, display: *mut Display, keylist: Vec<c_uint>) {
    ungrab_all(xlib, display);
    let screencount = (xlib.XScreenCount)(display);
    for screen in 0..screencount {
        let rootwindow = (xlib.XRootWindow)(display, screen);
        for code in &keylist {
            (xlib.XGrabKey)(
                display,
                *code as _,
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
                                            vacant.insert(callback);
                                            promise.set(Ok(()));
                                        } else {
                                            promise.set(Err(Error::AlreadyRegistered));
                                        }
                                        let keys = hotkeys.keys().copied().collect();
                                        grab_all(&xlib, display, keys);
                                    }
                                    Message::Unregister(key, promise) => {
                                        let code =
                                            (xlib.XKeysymToKeycode)(display, key as _) as c_uint;

                                        if hotkeys.remove(&code).is_some() {
                                            promise.set(Ok(()));
                                        } else {
                                            promise.set(Err(Error::NotRegistered));
                                        }
                                        let keys = hotkeys.keys().copied().collect();
                                        grab_all(&xlib, display, keys);
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

#[test]
fn test() {
    let hook = Hook::new().unwrap();
    hook.register(KeyCode::NumPad1, || println!("A")).unwrap();
    println!("Press NumPad1");
    thread::sleep(std::time::Duration::from_secs(5));
    hook.unregister(KeyCode::NumPad1).unwrap();
    hook.register(KeyCode::NumPad4, || println!("B")).unwrap();
    println!("Press NumPad4");
    thread::sleep(std::time::Duration::from_secs(5));
    hook.unregister(KeyCode::NumPad4).unwrap();
    hook.register(KeyCode::NumPad1, || println!("C")).unwrap();
    println!("Press NumPad1");
    thread::sleep(std::time::Duration::from_secs(5));
    hook.unregister(KeyCode::NumPad1).unwrap();
}
