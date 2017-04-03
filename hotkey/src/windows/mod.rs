extern crate winapi;
extern crate kernel32;
extern crate user32;

use std::cell::RefCell;
use std::{ptr, mem, thread};
use std::sync::mpsc::{channel, Sender};
use self::winapi::{c_int, WPARAM, LPARAM, LRESULT, WH_KEYBOARD_LL, HHOOK, WM_KEYDOWN, WM_KEYUP,
                   KBDLLHOOKSTRUCT, UINT, DWORD};
use self::user32::{CallNextHookEx, UnhookWindowsHookEx, SetWindowsHookExW, GetMessageW,
                   PostThreadMessageW};
use self::kernel32::{GetModuleHandleW, GetCurrentThreadId};
use KeyEvent;

const MSG_EXIT: UINT = 0x400;

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum KeyCode {
    LButton = 0x01,
    RButton = 0x02,
    Cancel = 0x03,
    MButton = 0x04,
    XButton1 = 0x05,
    XButton2 = 0x06,
    Back = 0x08,
    Tab = 0x09,
    Clear = 0x0C,
    Return = 0x0D,
    Shift = 0x10,
    Control = 0x11,
    Menu = 0x12,
    Pause = 0x13,
    Capital = 0x14,
    Kana = 0x15,
    Junja = 0x17,
    Final = 0x18,
    Kanji = 0x19,
    Escape = 0x1B,
    Convert = 0x1C,
    NonConvert = 0x1D,
    Accept = 0x1E,
    ModeChange = 0x1F,
    Space = 0x20,
    Prior = 0x21,
    Next = 0x22,
    End = 0x23,
    Home = 0x24,
    Left = 0x25,
    Up = 0x26,
    Right = 0x27,
    Down = 0x28,
    Select = 0x29,
    Print = 0x2A,
    Execute = 0x2B,
    Snapshot = 0x2C,
    Insert = 0x2D,
    Delete = 0x2E,
    Help = 0x2F,
    Num0 = 0x30,
    Num1 = 0x31,
    Num2 = 0x32,
    Num3 = 0x33,
    Num4 = 0x34,
    Num5 = 0x35,
    Num6 = 0x36,
    Num7 = 0x37,
    Num8 = 0x38,
    Num9 = 0x39,
    A = 0x41,
    B = 0x42,
    C = 0x43,
    D = 0x44,
    E = 0x45,
    F = 0x46,
    G = 0x47,
    H = 0x48,
    I = 0x49,
    J = 0x4A,
    K = 0x4B,
    L = 0x4C,
    M = 0x4D,
    N = 0x4E,
    O = 0x4F,
    P = 0x50,
    Q = 0x51,
    R = 0x52,
    S = 0x53,
    T = 0x54,
    U = 0x55,
    V = 0x56,
    W = 0x57,
    X = 0x58,
    Y = 0x59,
    Z = 0x5A,
    LWin = 0x5B,
    RWin = 0x5C,
    Apps = 0x5D,
    Sleep = 0x5F,
    NumPad0 = 0x60,
    NumPad1 = 0x61,
    NumPad2 = 0x62,
    NumPad3 = 0x63,
    NumPad4 = 0x64,
    NumPad5 = 0x65,
    NumPad6 = 0x66,
    NumPad7 = 0x67,
    NumPad8 = 0x68,
    NumPad9 = 0x69,
    Multiply = 0x6A,
    Add = 0x6B,
    Separator = 0x6C,
    Subtract = 0x6D,
    Decimal = 0x6E,
    Divide = 0x6F,
    F1 = 0x70,
    F2 = 0x71,
    F3 = 0x72,
    F4 = 0x73,
    F5 = 0x74,
    F6 = 0x75,
    F7 = 0x76,
    F8 = 0x77,
    F9 = 0x78,
    F10 = 0x79,
    F11 = 0x7A,
    F12 = 0x7B,
    F13 = 0x7C,
    F14 = 0x7D,
    F15 = 0x7E,
    F16 = 0x7F,
    F17 = 0x80,
    F18 = 0x81,
    F19 = 0x82,
    F20 = 0x83,
    F21 = 0x84,
    F22 = 0x85,
    F23 = 0x86,
    F24 = 0x87,
    NumLock = 0x90,
    Scroll = 0x91,
    LShift = 0xA0,
    RShift = 0xA1,
    LControl = 0xA2,
    RControl = 0xA3,
    LMenu = 0xA4,
    RMenu = 0xA5,
    BrowserBack = 0xA6,
    BrowserForward = 0xA7,
    BrowserRefresh = 0xA8,
    BrowserStop = 0xA9,
    BrowserSearch = 0xAA,
    BrowserFavorites = 0xAB,
    BrowserHome = 0xAC,
    VolumeMute = 0xAD,
    VolumeDown = 0xAE,
    VolumeUp = 0xAF,
    MediaNextTrack = 0xB0,
    MediaPrevTrack = 0xB1,
    MediaStop = 0xB2,
    MediaPlayPause = 0xB3,
    LaunchMail = 0xB4,
    LaunchMediaSelect = 0xB5,
    LaunchApp1 = 0xB6,
    LaunchApp2 = 0xB7,
    Oem1 = 0xBA,
    OemPlus = 0xBB,
    OemComma = 0xBC,
    OemMinus = 0xBD,
    OemPeriod = 0xBE,
    Oem2 = 0xBF,
    Oem3 = 0xC0,
    Oem4 = 0xDB,
    Oem5 = 0xDC,
    Oem6 = 0xDD,
    Oem7 = 0xDE,
    Oem8 = 0xDF,
    Oem102 = 0xE2,
    ProcessKey = 0xE5,
    Packet = 0xE7,
    Attn = 0xF6,
    CrSel = 0xF7,
    ExSel = 0xF8,
    ErEof = 0xF9,
    Play = 0xFA,
    Zoom = 0xFB,
    NoName = 0xFC,
    Pa1 = 0xFD,
    OemClear = 0xFE,
}

pub struct Hook {
    thread_id: DWORD,
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
    events: Sender<KeyEvent>,
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
                state.events
                    .send(KeyEvent::KeyDown(key_code))
                    .expect("Callback Thread disconnected");
            } else if event == WM_KEYUP {
                state.events.send(KeyEvent::KeyUp(key_code)).expect("Callback Thread disconnected");
            }
        }

        CallNextHookEx(state.hook, code, wparam, lparam)
    })
}

pub fn register_hook<F>(mut callback: F) -> Result<Hook, ()>
    where F: FnMut(KeyEvent) + Send + 'static
{
    let (initialized_tx, initialized_rx) = channel();
    let (events_tx, events_rx) = channel();

    thread::spawn(move || {
        let mut hook = ptr::null_mut();

        STATE.with(|state| {
                hook = unsafe {
                    SetWindowsHookExW(WH_KEYBOARD_LL,
                                      Some(callback_proc),
                                      GetModuleHandleW(ptr::null()),
                                      0)
                };

                if hook != ptr::null_mut() {
                    initialized_tx.send(Ok(unsafe { GetCurrentThreadId() }))
                        .map_err(|_| ())?;
                } else {
                    initialized_tx.send(Err(())).map_err(|_| ())?;
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
                return Err(());
            } else {
                break;
            }
        }

        unsafe {
            UnhookWindowsHookEx(hook);
        }

        Ok(())
    });


    thread::spawn(move || while let Ok(event) = events_rx.recv() {
        callback(event);
    });

    let thread_id = initialized_rx.recv().map_err(|_| ())??;

    Ok(Hook { thread_id: thread_id })
}
