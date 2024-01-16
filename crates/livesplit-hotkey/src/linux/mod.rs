use std::{fmt, thread::JoinHandle};

use crate::{ConsumePreference, Hotkey, KeyCode, Result};
use crossbeam_channel::Sender;
use mio::Waker;
use nix::unistd::{getgroups, Group};
use promising_future::{future_promise, Promise};

mod evdev_impl;
mod x11_impl;

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum Error {
    EvDev,
    EPoll,
    NoXLib,
    OpenXServerConnection,
    ThreadStopped,
}

impl From<Error> for crate::Error {
    fn from(e: Error) -> Self {
        Self::Platform(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::EvDev => "Failed fetching events from evdev.",
            Self::EPoll => "Failed polling the event file descriptors.",
            Self::NoXLib => "Failed dynamically linking to X11.",
            Self::OpenXServerConnection => "Failed opening a connection to the X11 server.",
            Self::ThreadStopped => "The background thread stopped unexpectedly.",
        })
    }
}

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
    pub fn new(consume: ConsumePreference) -> Result<Self> {
        if matches!(consume, ConsumePreference::PreferConsume) {
            if let Ok(x11) = x11_impl::new() {
                return Ok(x11);
            }
        }

        if !matches!(consume, ConsumePreference::MustConsume) && can_use_evdev().is_some() {
            evdev_impl::new().map_err(Into::into)
        } else if !matches!(
            consume,
            ConsumePreference::MustNotConsume | ConsumePreference::PreferConsume
        ) {
            x11_impl::new().map_err(Into::into)
        } else {
            Err(crate::Error::UnmatchedPreference)
        }
    }

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

    pub fn unregister(&self, hotkey: Hotkey) -> Result<()> {
        let (future, promise) = future_promise();

        self.sender
            .send(Message::Unregister(hotkey, promise))
            .map_err(|_| Error::ThreadStopped)?;

        self.waker.wake().map_err(|_| Error::ThreadStopped)?;

        future.value().ok_or(Error::ThreadStopped)?
    }

    pub fn try_resolve(&self, key_code: KeyCode) -> Option<String> {
        let (future, promise) = future_promise();

        self.sender.send(Message::Resolve(key_code, promise)).ok()?;

        self.waker.wake().ok()?;

        Some(char::to_string(&future.value()??))
    }
}
