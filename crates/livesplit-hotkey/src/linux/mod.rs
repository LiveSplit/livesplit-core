use std::thread::JoinHandle;

use crate::{Hotkey, KeyCode};
use crossbeam_channel::Sender;
use mio::Waker;
use nix::unistd::{getgroups, Group};
use promising_future::{future_promise, Promise};

mod evdev_impl;
mod x11_impl;

/// The error type for this crate.
#[derive(Debug, Copy, Clone, snafu::Snafu)]
#[non_exhaustive]
pub enum Error {
    /// The hotkey was already registered.
    AlreadyRegistered,
    /// The hotkey to unregister was not registered.
    NotRegistered,
    /// Failed fetching events from evdev.
    EvDev,
    /// Failed polling the event file descriptors.
    EPoll,
    /// Failed dynamically linking to X11.
    NoXLib,
    /// Failed opening a connection to the X11 server.
    OpenXServerConnection,
    /// The background thread stopped unexpectedly.
    ThreadStopped,
}

/// The result type for this crate.
pub type Result<T> = std::result::Result<T, Error>;

enum Message {
    Register(
        Hotkey,
        Box<dyn FnMut() + Send + 'static>,
        Promise<Result<()>>,
    ),
    Unregister(Hotkey, Promise<Result<()>>),
    Resolve(KeyCode, Promise<Option<char>>),
    End,
}

/// A hook allows you to listen to hotkeys.
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

fn can_use_evdev() -> Option<()> {
    let group = Group::from_name("input").ok()??.gid;
    let groups = getgroups().ok()?;
    groups.into_iter().find(|&g| g == group).map(drop)
}

impl Hook {
    /// Creates a new hook.
    pub fn new() -> Result<Self> {
        if can_use_evdev().is_some() {
            evdev_impl::new()
        } else {
            x11_impl::new()
        }
    }

    /// Registers a hotkey to listen to.
    pub fn register<F>(&self, hotkey: Hotkey, callback: F) -> Result<()>
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

    /// Unregisters a previously registered hotkey.
    pub fn unregister(&self, hotkey: Hotkey) -> Result<()> {
        let (future, promise) = future_promise();

        self.sender
            .send(Message::Unregister(hotkey, promise))
            .map_err(|_| Error::ThreadStopped)?;

        self.waker.wake().map_err(|_| Error::ThreadStopped)?;

        future.value().ok_or(Error::ThreadStopped)?
    }

    pub(crate) fn try_resolve(&self, key_code: KeyCode) -> Option<String> {
        let (future, promise) = future_promise();

        self.sender.send(Message::Resolve(key_code, promise)).ok()?;

        self.waker.wake().ok()?;

        Some(char::to_string(&future.value()??))
    }
}
