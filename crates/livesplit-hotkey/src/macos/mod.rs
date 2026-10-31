//! CoreGraphics keyboard taps, callback storage and native keyboard resolution.
//!
//! Public Hook construction checks privacy access before starting this worker.
//! Explicit permission requests live separately; callbacks never query or prompt
//! for grants. Callers retain Hook while using its thread-confined native loop.

mod ax;
mod carbon;
mod cf;
mod cg;
pub mod permission;
mod recovery;

use crate::{
    ConsumePreference, Hotkey, KeyCode, Modifiers, Result,
    macos::{
        carbon::{
            LMGetKbdType, TISCopyCurrentKeyboardInputSource,
            TISCopyCurrentKeyboardLayoutInputSource, TISGetInputSourceProperty, UCKeyAction,
            UCKeyTranslate, UCKeyTranslateBits, kTISPropertyUnicodeKeyLayoutData,
        },
        cf::{
            CFDataGetBytePtr, CFMachPortCreateRunLoopSource, CFRelease, CFRunLoopAddSource,
            CFRunLoopContainsSource, CFRunLoopGetCurrent, CFRunLoopRemoveSource, CFRunLoopRun,
            kCFAllocatorDefault, kCFRunLoopDefaultMode,
        },
        cg::{
            CGEventGetFlags, CGEventTapCreate, CGEventTapEnable, EventField, EventFlags, EventMask,
            EventRef, EventTapLocation, EventTapOptions, EventTapPlacement, EventTapProxy,
            EventType,
        },
    },
    permission::macos::TapMode,
};
use core::ptr::null_mut;
use std::{
    cell::Cell,
    collections::{HashMap, hash_map::Entry},
    ffi::c_void,
    fmt,
    sync::{Arc, Mutex, mpsc::channel},
    thread,
};

#[cfg(test)]
use std::sync::mpsc::Sender;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    CouldntCreateEventTap,
    /// The native dictionary for an explicit permission request could not be created.
    CouldntCreatePermissionOptions,
    CouldntCreateRunLoopSource,
    CouldntGetCurrentRunLoop,
    ThreadStoppedUnexpectedly,
}

impl From<Error> for crate::Error {
    fn from(e: Error) -> Self {
        Self::Platform(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::CouldntCreatePermissionOptions => "Failed creating permission request options.",
            Self::CouldntCreateEventTap => "Failed creating the event tap.",
            Self::CouldntCreateRunLoopSource => "Failed creating the run loop source.",
            Self::CouldntGetCurrentRunLoop => "Failed getting the current run loop.",
            Self::ThreadStoppedUnexpectedly => "The background thread stopped unexpectedly.",
        })
    }
}

struct Owned<T>(*mut T);

impl<T> Drop for Owned<T> {
    fn drop(&mut self) {
        // SAFETY: Owned is constructed only from a non-null retained CF
        // object. Each guard releases its own reference exactly once.
        unsafe { CFRelease(self.0.cast()) };
    }
}

/// Retained independently of the worker: native source invalidation can end
/// its loop while the caller still owns Hook and will later run its destructor.
struct RunLoop(cf::RunLoopRef);

impl RunLoop {
    fn current() -> Option<Self> {
        // SAFETY: CoreFoundation returns this thread's borrowed loop. Retaining
        // it gives the returned handle its own reference, independent of thread
        // lifetime. Null is handled before the retaining call.
        let loop_ref = unsafe { CFRunLoopGetCurrent() };
        if loop_ref.is_null() {
            return None;
        }
        // SAFETY: The current thread keeps this non-null loop alive while the
        // additional reference is acquired. Drop releases that reference once.
        unsafe { cf::CFRetain(loop_ref.cast()) };
        Some(Self(loop_ref))
    }
}

impl Drop for RunLoop {
    fn drop(&mut self) {
        // SAFETY: This handle owns one retained CF reference. CF reference
        // counting is thread-safe, including after the originating worker exits.
        unsafe { CFRelease(self.0.cast()) };
    }
}

// https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/RunLoopManagement/RunLoopManagement.html
// "The functions in Core Foundation are generally thread-safe and can be called from any thread."
// SAFETY: This opaque handle is used only with thread-safe CF run-loop
// functions; native tap/context access stays on the worker.
unsafe impl Send for RunLoop {}

// SAFETY: Sharing this handle does not expose or mutate the worker's callback
// data; CF run-loop control functions support cross-thread callers.
unsafe impl Sync for RunLoop {}

struct State {
    hotkeys: Mutex<HashMap<Hotkey, Box<dyn FnMut() + Send + 'static>>>,
    #[cfg(test)]
    observer: Option<Sender<Observation>>,
}

/// Native tests use an application-specific tap so synthetic events and slow
/// callbacks affect only their disposable receiver. Both targets share the
/// production worker, callback context, native source and teardown path.
#[derive(Clone, Copy)]
enum Target {
    Session,
    #[cfg(test)]
    Process(i32),
}

#[cfg(test)]
#[derive(Debug)]
enum Observation {
    Notification {
        event_type: EventType,
        attempted: bool,
        enabled: bool,
    },
    Stopped,
}

#[cfg(test)]
fn observe(state: &State, observation: Observation) {
    if let Some(observer) = &state.observer
        && let Err(error) = observer.send(observation)
    {
        eprintln!("native recovery observer disconnected: {error}");
    }
}

/// Callback data owned exclusively by the event-tap worker. The box is allocated
/// before creating the tap, and its address remains stable until the run-loop
/// source and tap have been released. Only that worker accesses the native port;
/// registration threads share the separate, synchronized hotkey state.
struct CallbackContext {
    state: Arc<State>,
    // Filled before scheduling the source, so no callback can observe null.
    tap: Cell<cf::MachPortRef>,
    recovery: recovery::Recovery,
}

/// A hook allows you to listen to hotkeys.
pub struct Hook {
    event_loop: RunLoop,
    state: Arc<State>,
}

impl Drop for Hook {
    fn drop(&mut self) {
        // SAFETY: Hook owns a retained loop reference even if native source
        // invalidation already ended the worker. CF loop control is thread-safe.
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
    pub fn new(consume: ConsumePreference) -> Result<Self> {
        let state = Arc::new(State {
            hotkeys: Mutex::new(HashMap::new()),
            #[cfg(test)]
            observer: None,
        });
        Self::with_state(consume, Target::Session, state)
    }

    fn with_state(consume: ConsumePreference, target: Target, state: Arc<State>) -> Result<Self> {
        let is_consuming = TapMode::from_preference(consume) == TapMode::Consume;
        let thread_state = state.clone();

        let (sender, receiver) = channel();

        // The code here is mostly based on:
        // https://github.com/kwhat/libuiohook/blob/f4bb19be8aee7d7ee5ead89b5a89dbf440e2a71a/src/darwin/input_hook.c#L1086

        // SAFETY: The worker owns the native port and run-loop source. Its
        // boxed callback data remains stable through creation and scheduling;
        // the source is removed before the port and context are released.
        thread::spawn(move || unsafe {
            let context = Box::new(CallbackContext {
                state: thread_state,
                tap: Cell::new(null_mut()),
                recovery: recovery::Recovery::new(),
            });
            let context_ptr: *const CallbackContext = &*context;

            // Hook's public constructor already checked the matching privacy
            // grant without prompting. Creating the tap may still fail if the
            // grant was revoked or native resources are unavailable.
            let options = if is_consuming {
                EventTapOptions::DEFAULT_TAP
            } else {
                EventTapOptions::LISTEN_ONLY
            };
            let port = match target {
                Target::Session => CGEventTapCreate(
                    EventTapLocation::SESSION,
                    EventTapPlacement::HEAD_INSERT_EVENT_TAP,
                    options,
                    EventMask::KEY_DOWN,
                    Some(callback),
                    context_ptr as *mut c_void,
                ),
                #[cfg(test)]
                Target::Process(pid) => cg::CGEventTapCreateForPid(
                    pid,
                    EventTapPlacement::HEAD_INSERT_EVENT_TAP,
                    options,
                    EventMask::KEY_DOWN,
                    Some(callback),
                    context_ptr as *mut c_void,
                ),
            };
            if port.is_null() {
                let _ = sender.send(Err(Error::CouldntCreateEventTap));
                return;
            }
            let port = Owned(port);
            context.tap.set(port.0);

            let source = CFMachPortCreateRunLoopSource(kCFAllocatorDefault, port.0, 0);
            if source.is_null() {
                let _ = sender.send(Err(Error::CouldntCreateRunLoopSource));
                return;
            }
            let source = Owned(source);

            let Some(control) = RunLoop::current() else {
                let _ = sender.send(Err(Error::CouldntGetCurrentRunLoop));
                return;
            };
            // The worker's thread-local CF reference keeps this raw borrow
            // alive after the retained control handle moves to the caller.
            let event_loop = control.0;
            CFRunLoopAddSource(event_loop, source.0, kCFRunLoopDefaultMode);

            if { sender }.send(Ok(control)).is_ok() {
                CFRunLoopRun();
            }

            if CFRunLoopContainsSource(event_loop, source.0, kCFRunLoopDefaultMode) {
                CFRunLoopRemoveSource(event_loop, source.0, kCFRunLoopDefaultMode);
            }
            CGEventTapEnable(port.0, false);
            // Release the source and port before publishing test completion or
            // dropping callback data. No native callback can outlive its box.
            drop(source);
            drop(port);
            #[cfg(test)]
            observe(&context.state, Observation::Stopped);
        });

        let event_loop = receiver
            .recv()
            .map_err(|_| Error::ThreadStoppedUnexpectedly)??;

        Ok(Hook { event_loop, state })
    }

    pub fn register<F>(&self, hotkey: Hotkey, callback: F) -> Result<()>
    where
        F: FnMut() + Send + 'static,
    {
        if let Entry::Vacant(vacant) = self.state.hotkeys.lock().unwrap().entry(hotkey) {
            vacant.insert(Box::new(callback));
            Ok(())
        } else {
            Err(crate::Error::AlreadyRegistered)
        }
    }

    pub fn unregister(&self, hotkey: Hotkey) -> Result<()> {
        let _ = self
            .state
            .hotkeys
            .lock()
            .unwrap()
            .remove(&hotkey)
            .ok_or(crate::Error::NotRegistered)?;
        Ok(())
    }

    pub fn try_resolve(&self, key_code: KeyCode) -> Option<String> {
        unsafe {
            let current_keyboard_raw = TISCopyCurrentKeyboardInputSource();
            if current_keyboard_raw.is_null() {
                return None;
            }
            let mut current_keyboard = Owned(current_keyboard_raw);

            let mut layout_data =
                TISGetInputSourceProperty(current_keyboard.0, kTISPropertyUnicodeKeyLayoutData);

            if layout_data.is_null() {
                let current_keyboard_raw = TISCopyCurrentKeyboardLayoutInputSource();
                if current_keyboard_raw.is_null() {
                    return None;
                }
                current_keyboard = Owned(current_keyboard_raw);

                layout_data =
                    TISGetInputSourceProperty(current_keyboard.0, kTISPropertyUnicodeKeyLayoutData);
                if layout_data.is_null() {
                    return None;
                }
            }

            let keyboard_layout = CFDataGetBytePtr(layout_data.cast());

            let key_code = match key_code {
                KeyCode::Backquote => 0x32,
                KeyCode::Backslash => 0x2A,
                KeyCode::BracketLeft => 0x21,
                KeyCode::BracketRight => 0x1E,
                KeyCode::Comma => 0x2B,
                KeyCode::Digit0 => 0x1D,
                KeyCode::Digit1 => 0x12,
                KeyCode::Digit2 => 0x13,
                KeyCode::Digit3 => 0x14,
                KeyCode::Digit4 => 0x15,
                KeyCode::Digit5 => 0x17,
                KeyCode::Digit6 => 0x16,
                KeyCode::Digit7 => 0x1A,
                KeyCode::Digit8 => 0x1C,
                KeyCode::Digit9 => 0x19,
                KeyCode::Equal => 0x18,
                KeyCode::IntlBackslash => 0x0A,
                KeyCode::IntlRo => 0x5E,
                KeyCode::IntlYen => 0x5D,
                KeyCode::KeyA => 0x00,
                KeyCode::KeyB => 0x0B,
                KeyCode::KeyC => 0x08,
                KeyCode::KeyD => 0x02,
                KeyCode::KeyE => 0x0E,
                KeyCode::KeyF => 0x03,
                KeyCode::KeyG => 0x05,
                KeyCode::KeyH => 0x04,
                KeyCode::KeyI => 0x22,
                KeyCode::KeyJ => 0x26,
                KeyCode::KeyK => 0x28,
                KeyCode::KeyL => 0x25,
                KeyCode::KeyM => 0x2E,
                KeyCode::KeyN => 0x2D,
                KeyCode::KeyO => 0x1F,
                KeyCode::KeyP => 0x23,
                KeyCode::KeyQ => 0x0C,
                KeyCode::KeyR => 0x0F,
                KeyCode::KeyS => 0x01,
                KeyCode::KeyT => 0x11,
                KeyCode::KeyU => 0x20,
                KeyCode::KeyV => 0x09,
                KeyCode::KeyW => 0x0D,
                KeyCode::KeyX => 0x07,
                KeyCode::KeyY => 0x10,
                KeyCode::KeyZ => 0x06,
                KeyCode::Minus => 0x1B,
                KeyCode::Period => 0x2F,
                KeyCode::Quote => 0x27,
                KeyCode::Semicolon => 0x29,
                KeyCode::Slash => 0x2C,
                _ => return None,
            };

            let mut chars = [0; 4];
            let mut len = 0;

            UCKeyTranslate(
                keyboard_layout.cast(),
                key_code,
                UCKeyAction::Display as _,
                0,
                LMGetKbdType() as _,
                UCKeyTranslateBits::NO_DEAD_KEYS_BIT.bits(),
                &mut 0,
                4,
                &mut len,
                chars.as_mut_ptr(),
            );

            if len == 0 {
                return None;
            }

            String::from_utf16(&chars[..len as usize]).ok()
        }
    }
}

/// # Safety
/// `user_info` must point to a live, immovable callback context. Its tap must be
/// valid when handling a disable notification; the worker installs it before
/// scheduling the source. Non-null key-down events must be valid CoreGraphics
/// events. Null and non-keyboard events are ignored; control notifications may
/// carry a null event.
/// The source must be removed and the tap released before dropping the context.
unsafe extern "C" fn callback(
    _: EventTapProxy,
    event_type: EventType,
    event: EventRef,
    user_info: *mut c_void,
) -> EventRef {
    // SAFETY: The worker keeps the boxed context alive until its native source
    // and tap are gone; callbacks execute only on that worker's run loop.
    let context = unsafe { &*user_info.cast::<CallbackContext>() };

    #[cfg(test)]
    let attempted = Cell::new(false);

    let response = context.recovery.dispatch(
        event_type,
        !event.is_null(),
        event,
        || {
            // SAFETY: The worker installs this port before scheduling callbacks
            // and releases it after removing the source. The policy spends its
            // sole timeout retry before this native call; no grant is requested.
            unsafe { CGEventTapEnable(context.tap.get(), true) };
            #[cfg(test)]
            attempted.set(true);
        },
        || {
            // SAFETY: The router admits only non-null key-down events. Both
            // the event and shared registration state outlive this callback.
            unsafe { dispatch_key(event, &context.state) }
        },
    );

    #[cfg(test)]
    if matches!(
        event_type,
        EventType::TAP_DISABLED_BY_TIMEOUT | EventType::TAP_DISABLED_BY_USER_INPUT
    ) {
        // SAFETY: As in native recovery, the worker's port remains alive for
        // this callback. Tests observe its state without sharing that port.
        let enabled = unsafe { cg::CGEventTapIsEnabled(context.tap.get()) };
        observe(
            &context.state,
            Observation::Notification {
                event_type,
                attempted: attempted.get(),
                enabled,
            },
        );
    }

    response
}

/// # Safety
/// `event` must be a live, non-null native key-down event. Registration state
/// stays valid for this invocation; its mutex serializes access to callbacks.
unsafe fn dispatch_key(event: EventRef, state: &State) -> EventRef {
    // SAFETY: Only non-null native key-down events reach keyboard accessors.
    let is_repeating =
        unsafe { cg::CGEventGetIntegerValueField(event, EventField::KEYBOARD_EVENT_AUTOREPEAT) };
    if is_repeating != 0 {
        return event;
    }

    // SAFETY: The same live key-down event was admitted above.
    let key_code =
        unsafe { cg::CGEventGetIntegerValueField(event, EventField::KEYBOARD_EVENT_KEYCODE) };
    let key_code = match key_code {
        0x00 => KeyCode::KeyA,
        0x01 => KeyCode::KeyS,
        0x02 => KeyCode::KeyD,
        0x03 => KeyCode::KeyF,
        0x04 => KeyCode::KeyH,
        0x05 => KeyCode::KeyG,
        0x06 => KeyCode::KeyZ,
        0x07 => KeyCode::KeyX,
        0x08 => KeyCode::KeyC,
        0x09 => KeyCode::KeyV,
        0x0A => KeyCode::IntlBackslash,
        0x0B => KeyCode::KeyB,
        0x0C => KeyCode::KeyQ,
        0x0D => KeyCode::KeyW,
        0x0E => KeyCode::KeyE,
        0x0F => KeyCode::KeyR,
        0x10 => KeyCode::KeyY,
        0x11 => KeyCode::KeyT,
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
        0x1E => KeyCode::BracketRight,
        0x1F => KeyCode::KeyO,
        0x20 => KeyCode::KeyU,
        0x21 => KeyCode::BracketLeft,
        0x22 => KeyCode::KeyI,
        0x23 => KeyCode::KeyP,
        0x24 => KeyCode::Enter,
        0x25 => KeyCode::KeyL,
        0x26 => KeyCode::KeyJ,
        0x27 => KeyCode::Quote,
        0x28 => KeyCode::KeyK,
        0x29 => KeyCode::Semicolon,
        0x2A => KeyCode::Backslash,
        0x2B => KeyCode::Comma,
        0x2C => KeyCode::Slash,
        0x2D => KeyCode::KeyN,
        0x2E => KeyCode::KeyM,
        0x2F => KeyCode::Period,
        0x30 => KeyCode::Tab,
        0x31 => KeyCode::Space,
        0x32 => KeyCode::Backquote,
        0x33 => KeyCode::Backspace,
        0x34 => KeyCode::NumpadEnter, // Not Chrome
        0x35 => KeyCode::Escape,
        0x36 => KeyCode::MetaRight,
        0x37 => KeyCode::MetaLeft,
        0x38 => KeyCode::ShiftLeft,
        0x39 => KeyCode::CapsLock,
        0x3A => KeyCode::AltLeft,
        0x3B => KeyCode::ControlLeft,
        0x3C => KeyCode::ShiftRight,
        0x3D => KeyCode::AltRight,
        0x3E => KeyCode::ControlRight,
        0x3F => KeyCode::Fn, // Not Chrome
        0x40 => KeyCode::F17,
        0x41 => KeyCode::NumpadDecimal,
        0x43 => KeyCode::NumpadMultiply,
        0x45 => KeyCode::NumpadAdd,
        0x47 => KeyCode::NumLock,
        0x48 => KeyCode::AudioVolumeUp,
        0x49 => KeyCode::AudioVolumeDown,
        0x4A => KeyCode::AudioVolumeMute,
        0x4B => KeyCode::NumpadDivide,
        0x4C => KeyCode::NumpadEnter,
        0x4E => KeyCode::NumpadSubtract,
        0x4F => KeyCode::F18,
        0x50 => KeyCode::F19,
        0x51 => KeyCode::NumpadEqual,
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
        0x5D => KeyCode::IntlYen,
        0x5E => KeyCode::IntlRo,
        0x5F => KeyCode::NumpadComma,
        0x60 => KeyCode::F5,
        0x61 => KeyCode::F6,
        0x62 => KeyCode::F7,
        0x63 => KeyCode::F3,
        0x64 => KeyCode::F8,
        0x65 => KeyCode::F9,
        0x66 => KeyCode::Lang2,
        0x67 => KeyCode::F11,
        0x68 => KeyCode::Lang1, // KanaMode in Safari
        0x69 => KeyCode::F13,
        0x6A => KeyCode::F16,
        0x6B => KeyCode::F14,
        0x6D => KeyCode::F10,
        0x6E => KeyCode::ContextMenu, // Missing on MDN
        0x6F => KeyCode::F12,
        0x71 => KeyCode::F15,
        // Apple hasn't been producing any keyboards with `Help` anymore since
        // 2007. So this can be considered Insert instead.
        0x72 => KeyCode::Insert,
        0x73 => KeyCode::Home,
        0x74 => KeyCode::PageUp,
        0x75 => KeyCode::Delete,
        0x76 => KeyCode::F4,
        0x77 => KeyCode::End,
        0x78 => KeyCode::F2,
        0x79 => KeyCode::PageDown,
        0x7A => KeyCode::F1,
        0x7B => KeyCode::ArrowLeft,
        0x7C => KeyCode::ArrowRight,
        0x7D => KeyCode::ArrowDown,
        0x7E => KeyCode::ArrowUp,
        _ => return event,
    };

    // SAFETY: The callback's key-down event remains borrowed for this invocation.
    let modifier_flags = unsafe { CGEventGetFlags(event) };
    let mut modifiers = Modifiers::empty();

    if modifier_flags
        .intersects(EventFlags::SHIFT | EventFlags::LEFT_SHIFT | EventFlags::RIGHT_SHIFT)
        && !matches!(key_code, KeyCode::ShiftLeft | KeyCode::ShiftRight)
    {
        modifiers.insert(Modifiers::SHIFT);
    }

    if modifier_flags
        .intersects(EventFlags::CONTROL | EventFlags::LEFT_CONTROL | EventFlags::RIGHT_CONTROL)
        && !matches!(key_code, KeyCode::ControlLeft | KeyCode::ControlRight)
    {
        modifiers.insert(Modifiers::CONTROL);
    }

    if modifier_flags
        .intersects(EventFlags::OPTION | EventFlags::LEFT_OPTION | EventFlags::RIGHT_OPTION)
        && !matches!(key_code, KeyCode::AltLeft | KeyCode::AltRight)
    {
        modifiers.insert(Modifiers::ALT);
    }

    if modifier_flags
        .intersects(EventFlags::COMMAND | EventFlags::LEFT_COMMAND | EventFlags::RIGHT_COMMAND)
        && !matches!(key_code, KeyCode::MetaLeft | KeyCode::MetaRight)
    {
        modifiers.insert(Modifiers::META);
    }

    if let Some(callback) = state
        .hotkeys
        .lock()
        .unwrap()
        .get_mut(&key_code.with_modifiers(modifiers))
    {
        callback();

        // If we handled the event and the hook is consuming, we should return
        // null so the system deletes the event. If the hook is not consuming
        // the return value will be ignored, so return null anyway.
        null_mut()
    } else {
        event
    }
}

#[cfg(test)]
mod tests;
