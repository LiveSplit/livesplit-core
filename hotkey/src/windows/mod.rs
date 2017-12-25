extern crate parking_lot;
extern crate winapi;

mod key_code;
pub use self::key_code::KeyCode;

use std::cell::RefCell;
use std::{mem, ptr, thread};
use std::sync::mpsc::{channel, Sender};
use self::winapi::ctypes::c_int;
use self::winapi::shared::minwindef::{DWORD, LPARAM, LRESULT, UINT, WPARAM};
use self::winapi::shared::windef::HHOOK;
use self::winapi::um::libloaderapi::GetModuleHandleW;
use self::winapi::um::processthreadsapi::GetCurrentThreadId;
use self::winapi::um::winuser::{CallNextHookEx, GetMessageW, PostThreadMessageW, SetWindowsHookExW,
                                UnhookWindowsHookEx};
use self::winapi::um::winuser::{KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_KEYDOWN};
use std::sync::Arc;
use std::collections::hash_map::{Entry, HashMap};
use self::parking_lot::Mutex;

const MSG_EXIT: UINT = 0x400;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        AlreadyRegistered {}
        NotRegistered {}
        WindowsHook {}
        ThreadStopped {}
        MessageLoop {}
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

pub struct Hook {
    thread_id: DWORD,
    hotkeys: Arc<Mutex<HashMap<KeyCode, Box<FnMut() + Send + 'static>>>>,
}

impl Drop for Hook {
    fn drop(&mut self) {
        unsafe {
            PostThreadMessageW(self.thread_id, MSG_EXIT, 0, 0);
        }
    }
}

struct State {
    hook: HHOOK,
    events: Sender<KeyCode>,
}

thread_local! {
    static STATE: RefCell<Option<State>> = RefCell::new(None);
}

unsafe extern "system" fn callback_proc(code: c_int, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let state = state.as_mut().expect("State should be initialized by now");

        if code >= 0 {
            let key_code = mem::transmute((*(lparam as *const KBDLLHOOKSTRUCT)).vkCode as u8);
            let event = wparam as UINT;
            if event == WM_KEYDOWN {
                state
                    .events
                    .send(key_code)
                    .expect("Callback Thread disconnected");
            }
        }

        CallNextHookEx(state.hook, code, wparam, lparam)
    })
}

impl Hook {
    pub fn new() -> Result<Self> {
        let hotkeys = Arc::new(Mutex::new(HashMap::<
            KeyCode,
            Box<FnMut() + Send + 'static>,
        >::new()));

        let (initialized_tx, initialized_rx) = channel();
        let (events_tx, events_rx) = channel();

        thread::spawn(move || {
            let mut hook = ptr::null_mut();

            STATE.with(|state| {
                hook = unsafe {
                    SetWindowsHookExW(
                        WH_KEYBOARD_LL,
                        Some(callback_proc),
                        GetModuleHandleW(ptr::null()),
                        0,
                    )
                };

                if hook != ptr::null_mut() {
                    initialized_tx
                        .send(Ok(unsafe { GetCurrentThreadId() }))
                        .map_err(|_| Error::ThreadStopped)?;
                } else {
                    initialized_tx
                        .send(Err(Error::WindowsHook))
                        .map_err(|_| Error::ThreadStopped)?;
                }

                *state.borrow_mut() = Some(State {
                    hook: hook,
                    events: events_tx,
                });

                Ok(())
            })?;

            let mut msg = unsafe { mem::uninitialized() };
            loop {
                let ret = unsafe { GetMessageW(&mut msg, ptr::null_mut(), 0, 0) };

                if msg.message == MSG_EXIT {
                    break;
                } else if ret < 0 {
                    return Err(Error::MessageLoop);
                } else {
                    break;
                }
            }

            unsafe {
                UnhookWindowsHookEx(hook);
            }

            Ok(())
        });

        let hotkey_map = hotkeys.clone();

        thread::spawn(move || {
            while let Ok(key) = events_rx.recv() {
                if let Some(callback) = hotkey_map.lock().get_mut(&key) {
                    callback();
                }
            }
        });

        let thread_id = initialized_rx.recv().map_err(|_| Error::ThreadStopped)??;

        Ok(Hook {
            thread_id: thread_id,
            hotkeys: hotkeys,
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

#[test]
fn test() {
    let hook = Hook::new().unwrap();
    hook.register(KeyCode::NumPad0, || println!("A")).unwrap();
    thread::sleep(::std::time::Duration::from_secs(5));
    hook.unregister(KeyCode::NumPad0).unwrap();
    hook.register(KeyCode::NumPad1, || println!("B")).unwrap();
    thread::sleep(::std::time::Duration::from_secs(5));
}
