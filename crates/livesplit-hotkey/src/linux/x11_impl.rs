use std::{
    collections::HashMap,
    mem::MaybeUninit,
    os::raw::{c_int, c_uint},
    ptr,
    sync::mpsc::channel,
    thread,
};

use mio::{unix::SourceFd, Events, Interest, Poll, Token, Waker};
use x11_dl::xlib::{
    AnyKey, AnyModifier, ControlMask, Display, GrabModeAsync, KeyPress, Mod1Mask, Mod4Mask,
    ShiftMask, XErrorEvent, XKeyEvent, Xlib,
};

use super::Message;
use crate::{Error, Hook, KeyCode, Modifiers, Result};

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

unsafe fn grab_all(xlib: &Xlib, display: *mut Display, keylist: &[(c_uint, Modifiers)]) {
    ungrab_all(xlib, display);
    let screencount = (xlib.XScreenCount)(display);
    for screen in 0..screencount {
        let rootwindow = (xlib.XRootWindow)(display, screen);
        for &(code, modifiers) in keylist {
            let mut mod_mask = 0;
            if modifiers.contains(Modifiers::SHIFT) {
                mod_mask |= ShiftMask;
            }
            if modifiers.contains(Modifiers::CONTROL) {
                mod_mask |= ControlMask;
            }
            if modifiers.contains(Modifiers::ALT) {
                mod_mask |= Mod1Mask;
            }
            if modifiers.contains(Modifiers::META) {
                mod_mask |= Mod4Mask;
            }
            (xlib.XGrabKey)(
                display,
                code as c_int,
                mod_mask,
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

const fn code_for(key: KeyCode) -> Option<c_uint> {
    match super::evdev_impl::code_for(key) {
        Some(code) => Some(code.0 as c_uint + 8),
        None => None,
    }
}

const X_TOKEN: Token = Token(0);
const PING_TOKEN: Token = Token(1);

pub fn new() -> Result<Hook> {
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
            // Force the whole XData to be moved.
            let xdata = xdata;
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
                                    promise.set(
                                        if code_for(key.key_code)
                                            .and_then(|k| {
                                                hotkeys.insert((k, key.modifiers), callback)
                                            })
                                            .is_some()
                                        {
                                            Err(Error::AlreadyRegistered)
                                        } else {
                                            let keys = hotkeys.keys().copied().collect::<Vec<_>>();
                                            grab_all(&xlib, display, &keys);
                                            Ok(())
                                        },
                                    );
                                }
                                Message::Unregister(key, promise) => {
                                    let res = code_for(key.key_code)
                                        .and_then(|k| hotkeys.remove(&(k, key.modifiers)).map(drop))
                                        .ok_or(Error::NotRegistered);
                                    if res.is_ok() {
                                        let keys = hotkeys.keys().copied().collect::<Vec<_>>();
                                        grab_all(&xlib, display, &keys);
                                    }
                                    promise.set(res);
                                }
                                Message::End => {
                                    break 'event_loop;
                                }
                            }
                        }
                    } else if mio_event.token() == X_TOKEN {
                        while (xlib.XPending)(display) != 0 {
                            let mut event = MaybeUninit::uninit();
                            let err_code = (xlib.XNextEvent)(display, event.as_mut_ptr());
                            if err_code == 0 {
                                let event = event.assume_init();
                                if event.get_type() == KeyPress {
                                    let event: &XKeyEvent = event.as_ref();

                                    let mut modifiers = Modifiers::empty();
                                    if event.state & ShiftMask != 0 {
                                        modifiers.insert(Modifiers::SHIFT);
                                    }
                                    if event.state & ControlMask != 0 {
                                        modifiers.insert(Modifiers::CONTROL);
                                    }
                                    if event.state & Mod1Mask != 0 {
                                        modifiers.insert(Modifiers::ALT);
                                    }
                                    if event.state & Mod4Mask != 0 {
                                        modifiers.insert(Modifiers::META);
                                    }

                                    if let Some(callback) =
                                        hotkeys.get_mut(&(event.keycode, modifiers))
                                    {
                                        callback();
                                    }
                                    // FIXME: We should check else here: these amount to lost
                                    // keypresses.
                                }
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
