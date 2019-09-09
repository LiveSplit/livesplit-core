mod key_code;
pub use self::key_code::KeyCode;

use parking_lot::Mutex;
use std::cell::RefCell;
use std::collections::hash_map::{Entry, HashMap};
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::{mem, ptr, thread};
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{DWORD, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HHOOK;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::processthreadsapi::GetCurrentThreadId;
use winapi::um::winuser::{
    CallNextHookEx, GetMessageW, PostThreadMessageW, SetWindowsHookExW, UnhookWindowsHookEx,
};
use winapi::um::winuser::{KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_KEYDOWN};

const MSG_EXIT: UINT = 0x400;

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    AlreadyRegistered,
    NotRegistered,
    WindowsHook,
    ThreadStopped,
    MessageLoop,
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Hook {
    thread_id: DWORD,
    hotkeys: Arc<Mutex<HashMap<KeyCode, Box<dyn FnMut() + Send + 'static>>>>,
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
            Box<dyn FnMut() + Send + 'static>,
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

                if !hook.is_null() {
                    initialized_tx
                        .send(Ok(unsafe { GetCurrentThreadId() }))
                        .map_err(|_| Error::ThreadStopped)?;
                } else {
                    initialized_tx
                        .send(Err(Error::WindowsHook))
                        .map_err(|_| Error::ThreadStopped)?;
                }

                *state.borrow_mut() = Some(State {
                    hook,
                    events: events_tx,
                });

                Ok(())
            })?;

            loop {
                let mut msg = mem::MaybeUninit::uninit();
                let ret = unsafe { GetMessageW(msg.as_mut_ptr(), ptr::null_mut(), 0, 0) };
                let msg = unsafe { msg.assume_init() };
                if msg.message == MSG_EXIT {
                    break;
                } else if ret < 0 {
                    return Err(Error::MessageLoop);
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

        Ok(Hook { thread_id, hotkeys })
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
    thread::sleep(std::time::Duration::from_secs(5));
    hook.unregister(KeyCode::NumPad0).unwrap();
    hook.register(KeyCode::NumPad1, || println!("B")).unwrap();
    thread::sleep(std::time::Duration::from_secs(5));
}
