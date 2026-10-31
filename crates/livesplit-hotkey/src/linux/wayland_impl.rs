use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    os::fd::{AsFd, AsRawFd},
    thread,
};

use mio::{Events, Interest, Poll, Token, Waker, unix::SourceFd};
use promising_future::Promise;
use wayland_client::{
    Connection, Dispatch, QueueHandle, WEnum,
    globals::{GlobalListContents, registry_queue_init},
    protocol::{wl_keyboard, wl_registry, wl_seat},
};
use xkbcommon_dl::{
    XkbCommon, xkb_context, xkb_context_flags, xkb_keymap, xkb_keymap_compile_flags,
    xkb_keymap_format,
};

use super::{Error, Hook, Message, evdev_impl};
use crate::{Hotkey, KeyCode, Modifiers, Result};

mod protocol {
    #[allow(clippy::single_component_path_imports)]
    use wayland_client;
    use wayland_client::protocol::*;

    pub mod __interfaces {
        use wayland_client::protocol::__interfaces::*;

        wayland_scanner::generate_interfaces!("protocols/xx-hotkey-v1.xml");
    }

    use self::__interfaces::*;
    wayland_scanner::generate_client_code!("protocols/xx-hotkey-v1.xml");
}

use self::protocol::{
    xx_hotkey_manager_v1::{Modifiers as ProtocolModifiers, XxHotkeyManagerV1},
    xx_hotkey_v1::{self, XxHotkeyV1},
};

const WAYLAND_TOKEN: Token = Token(0);
const PING_TOKEN: Token = Token(1);

struct KeyboardMap {
    lib: &'static XkbCommon,
    context: *mut xkb_context,
    keymap: *mut xkb_keymap,
}

// The xkb objects are only ever used by the Wayland hotkey thread.
unsafe impl Send for KeyboardMap {}

impl KeyboardMap {
    fn from_keymap(mut fd: File, size: u32) -> Option<Self> {
        let lib = xkbcommon_dl::xkbcommon_option()?;
        let mut bytes = Vec::with_capacity(size as usize);
        fd.by_ref().take(size as u64).read_to_end(&mut bytes).ok()?;

        let context = unsafe { (lib.xkb_context_new)(xkb_context_flags::XKB_CONTEXT_NO_FLAGS) };
        if context.is_null() {
            return None;
        }

        let keymap = unsafe {
            (lib.xkb_keymap_new_from_buffer)(
                context,
                bytes.as_ptr().cast(),
                bytes.len(),
                xkb_keymap_format::XKB_KEYMAP_FORMAT_TEXT_V1,
                xkb_keymap_compile_flags::XKB_KEYMAP_COMPILE_NO_FLAGS,
            )
        };
        if keymap.is_null() {
            unsafe { (lib.xkb_context_unref)(context) };
            return None;
        }

        Some(Self {
            lib,
            context,
            keymap,
        })
    }

    fn keysym_for(&self, key_code: KeyCode) -> Option<u32> {
        // XKB keycodes use the X11 numbering, which is evdev + 8.
        let keycode = evdev_impl::code_for(key_code)?.0 as u32 + 8;
        let mut syms = std::ptr::null();
        let count = unsafe {
            (self.lib.xkb_keymap_key_get_syms_by_level)(self.keymap, keycode, 0, 0, &mut syms)
        };
        if count <= 0 || syms.is_null() {
            return None;
        }
        Some(unsafe { *syms })
    }

    fn resolve(&self, key_code: KeyCode) -> Option<char> {
        let keysym = self.keysym_for(key_code)?;
        let code_point = unsafe { (self.lib.xkb_keysym_to_utf32)(keysym) };
        char::from_u32(code_point)
    }
}

impl Drop for KeyboardMap {
    fn drop(&mut self) {
        unsafe {
            (self.lib.xkb_keymap_unref)(self.keymap);
            (self.lib.xkb_context_unref)(self.context);
        }
    }
}

struct Binding {
    proxy: XxHotkeyV1,
    callback: Box<dyn FnMut() + Send + 'static>,
    pending: Option<Promise<Result<()>>>,
    keysym: u32,
}

struct State {
    manager: XxHotkeyManagerV1,
    _seat: Option<wl_seat::WlSeat>,
    keyboard: Option<wl_keyboard::WlKeyboard>,
    keymap: Option<KeyboardMap>,
    bindings: HashMap<Hotkey, Binding>,
}

impl State {
    fn register(
        &mut self,
        qh: &QueueHandle<Self>,
        hotkey: Hotkey,
        callback: Box<dyn FnMut() + Send + 'static>,
        promise: Promise<Result<()>>,
    ) {
        if self.bindings.contains_key(&hotkey) {
            promise.set(Err(crate::Error::AlreadyRegistered));
            return;
        }

        let Some(keysym) = self
            .keymap
            .as_ref()
            .and_then(|keymap| keymap.keysym_for(hotkey.key_code))
        else {
            // Keep the behavior of the existing Linux backends for keys that
            // cannot be represented by the platform implementation.
            promise.set(Ok(()));
            return;
        };

        let proxy = self.manager.create_hotkey(qh, hotkey);
        proxy.set_description(hotkey.to_string());
        proxy.set_key_trigger(keysym, protocol_modifiers(hotkey.modifiers));
        proxy.commit();

        self.bindings.insert(
            hotkey,
            Binding {
                proxy,
                callback,
                pending: Some(promise),
                keysym,
            },
        );
    }

    fn unregister(&mut self, hotkey: Hotkey, promise: Promise<Result<()>>) {
        promise.set(match self.bindings.remove(&hotkey) {
            Some(binding) => {
                binding.proxy.destroy();
                Ok(())
            }
            None => Err(crate::Error::NotRegistered),
        });
    }

    fn update_keymap(&mut self, keymap: KeyboardMap) {
        self.keymap = Some(keymap);
        let updates: Vec<_> = self
            .bindings
            .keys()
            .filter_map(|&hotkey| {
                let keysym = self.keymap.as_ref()?.keysym_for(hotkey.key_code)?;
                Some((hotkey, keysym))
            })
            .collect();

        for (hotkey, keysym) in updates {
            let binding = self.bindings.get_mut(&hotkey).unwrap();
            if binding.keysym != keysym {
                binding.keysym = keysym;
                binding
                    .proxy
                    .set_key_trigger(keysym, protocol_modifiers(hotkey.modifiers));
                binding.proxy.commit();
            }
        }
    }
}

fn protocol_modifiers(modifiers: Modifiers) -> ProtocolModifiers {
    let mut result = ProtocolModifiers::empty();
    if modifiers.contains(Modifiers::SHIFT) {
        result.insert(ProtocolModifiers::Shift);
    }
    if modifiers.contains(Modifiers::CONTROL) {
        result.insert(ProtocolModifiers::Ctrl);
    }
    if modifiers.contains(Modifiers::ALT) {
        result.insert(ProtocolModifiers::Alt);
    }
    if modifiers.contains(Modifiers::META) {
        result.insert(ProtocolModifiers::Super);
    }
    result
}

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
    fn event(
        _: &mut Self,
        _: &wl_registry::WlRegistry,
        _: wl_registry::Event,
        _: &GlobalListContents,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_seat::WlSeat, ()> for State {
    fn event(
        state: &mut Self,
        seat: &wl_seat::WlSeat,
        event: wl_seat::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_seat::Event::Capabilities {
            capabilities: WEnum::Value(capabilities),
        } = event
            && capabilities.contains(wl_seat::Capability::Keyboard)
            && state.keyboard.is_none()
        {
            state.keyboard = Some(seat.get_keyboard(qh, ()));
        }
    }
}

impl Dispatch<wl_keyboard::WlKeyboard, ()> for State {
    fn event(
        state: &mut Self,
        _: &wl_keyboard::WlKeyboard,
        event: wl_keyboard::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let wl_keyboard::Event::Keymap {
            format: WEnum::Value(wl_keyboard::KeymapFormat::XkbV1),
            fd,
            size,
        } = event
            && let Some(keymap) = KeyboardMap::from_keymap(fd.into(), size)
        {
            state.update_keymap(keymap);
        }
    }
}

impl Dispatch<XxHotkeyManagerV1, ()> for State {
    fn event(
        _: &mut Self,
        _: &XxHotkeyManagerV1,
        _: <XxHotkeyManagerV1 as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<XxHotkeyV1, Hotkey> for State {
    fn event(
        state: &mut Self,
        proxy: &XxHotkeyV1,
        event: xx_hotkey_v1::Event,
        hotkey: &Hotkey,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            xx_hotkey_v1::Event::Bound => {
                if let Some(promise) = state
                    .bindings
                    .get_mut(hotkey)
                    .and_then(|binding| binding.pending.take())
                {
                    promise.set(Ok(()));
                }
            }
            xx_hotkey_v1::Event::Denied { .. } => {
                let is_initial_commit = state
                    .bindings
                    .get(hotkey)
                    .is_some_and(|binding| binding.pending.is_some());
                if is_initial_commit && let Some(mut binding) = state.bindings.remove(hotkey) {
                    proxy.destroy();
                    binding
                        .pending
                        .take()
                        .unwrap()
                        .set(Err(Error::WaylandHotkeyDenied.into()));
                }
            }
            xx_hotkey_v1::Event::Revoked { .. } => {
                state.bindings.remove(hotkey);
                proxy.destroy();
            }
            xx_hotkey_v1::Event::Triggered { .. } => {
                if let Some(binding) = state.bindings.get_mut(hotkey) {
                    (binding.callback)();
                }
            }
            xx_hotkey_v1::Event::Released { .. } => {}
        }
    }
}

pub fn new() -> Result<Hook> {
    let connection = Connection::connect_to_env().map_err(|_| Error::NoWaylandConnection)?;
    let (globals, mut event_queue) =
        registry_queue_init::<State>(&connection).map_err(|_| Error::Wayland)?;
    let qh = event_queue.handle();
    let manager = globals
        .bind::<XxHotkeyManagerV1, _, _>(&qh, 1..=1, ())
        .map_err(|_| Error::WaylandHotkeyProtocolUnavailable)?;
    let seat = globals
        .bind::<wl_seat::WlSeat, _, _>(&qh, 1..=1, ())
        .map_err(|_| Error::WaylandKeymapUnavailable)?;
    let mut state = State {
        manager,
        _seat: Some(seat),
        keyboard: None,
        keymap: None,
        bindings: HashMap::new(),
    };

    // The first trip receives seat capabilities and requests the keyboard;
    // the second receives its initial keymap.
    event_queue
        .roundtrip(&mut state)
        .map_err(|_| Error::Wayland)?;
    event_queue
        .roundtrip(&mut state)
        .map_err(|_| Error::Wayland)?;
    if state.keymap.is_none() {
        return Err(Error::WaylandKeymapUnavailable.into());
    }

    let (sender, receiver) = crossbeam_channel::unbounded();
    let mut poll = Poll::new().map_err(|_| Error::EPoll)?;
    let fd = event_queue.as_fd().as_raw_fd();
    poll.registry()
        .register(&mut SourceFd(&fd), WAYLAND_TOKEN, Interest::READABLE)
        .map_err(|_| Error::EPoll)?;
    let waker = Waker::new(poll.registry(), PING_TOKEN).map_err(|_| Error::EPoll)?;
    let thread_qh = qh;

    let join_handle = thread::spawn(move || -> Result<()> {
        let mut events = Events::with_capacity(16);

        'event_loop: loop {
            event_queue
                .dispatch_pending(&mut state)
                .map_err(|_| Error::Wayland)?;
            event_queue.flush().map_err(|_| Error::Wayland)?;

            let read_guard = loop {
                if let Some(read_guard) = event_queue.prepare_read() {
                    break read_guard;
                }
                event_queue
                    .dispatch_pending(&mut state)
                    .map_err(|_| Error::Wayland)?;
            };

            poll.poll(&mut events, None).map_err(|_| Error::EPoll)?;
            let wayland_ready = events
                .iter()
                .any(|event| event.token() == WAYLAND_TOKEN && event.is_readable());
            let messages_ready = events.iter().any(|event| event.token() == PING_TOKEN);

            if wayland_ready {
                read_guard.read().map_err(|_| Error::Wayland)?;
                event_queue
                    .dispatch_pending(&mut state)
                    .map_err(|_| Error::Wayland)?;
            } else {
                drop(read_guard);
            }

            if messages_ready {
                for message in receiver.try_iter() {
                    match message {
                        Message::Register(hotkey, callback, promise) => {
                            state.register(&thread_qh, hotkey, callback, promise);
                        }
                        Message::Unregister(hotkey, promise) => {
                            state.unregister(hotkey, promise);
                        }
                        Message::Resolve(key_code, promise) => {
                            promise.set(
                                state
                                    .keymap
                                    .as_ref()
                                    .and_then(|keymap| keymap.resolve(key_code)),
                            );
                        }
                        Message::End => break 'event_loop,
                    }
                }
            }
        }

        Ok(())
    });

    Ok(Hook {
        sender,
        waker,
        join_handle: Some(join_handle),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_modifiers() {
        let modifiers = protocol_modifiers(
            Modifiers::SHIFT | Modifiers::CONTROL | Modifiers::ALT | Modifiers::META,
        );
        assert!(modifiers.contains(ProtocolModifiers::Shift));
        assert!(modifiers.contains(ProtocolModifiers::Ctrl));
        assert!(modifiers.contains(ProtocolModifiers::Alt));
        assert!(modifiers.contains(ProtocolModifiers::Super));
    }
}
