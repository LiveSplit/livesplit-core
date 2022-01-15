//! The auto splitting module provides a runtime for running auto splitters that
//! can control the timer. These auto splitters are provided as WebAssembly
//! modules.
//!
//! # Requirements for the Auto Splitters
//!
//! The auto splitters must provide an `update` function with the following
//! signature:
//!
//! ```rust
//! #[no_mangle]
//! pub extern "C" fn update() {}
//! ```
//!
//! This function is called periodically by the runtime at the configured tick
//! rate. The tick rate is 120 Hz by default, but can be changed by the auto
//! splitter.
//!
//! In addition the WebAssembly module is expected to export a memory called
//! `memory`.
//!
//! # API exposed to the Auto Splitters
//!
//! The following functions are provided to the auto splitters in the module
//! `env`:
//!
//! ```rust
//! # use core::num::NonZeroU64;
//!
//! #[repr(transparent)]
//! pub struct Address(pub u64);
//!
//! #[repr(transparent)]
//! pub struct NonZeroAddress(pub NonZeroU64);
//!
//! #[repr(transparent)]
//! pub struct ProcessId(NonZeroU64);
//!
//! #[repr(transparent)]
//! pub struct TimerState(u32);
//!
//! impl TimerState {
//!     /// The timer is not running.
//!     pub const NOT_RUNNING: Self = Self(0);
//!     /// The timer is running.
//!     pub const RUNNING: Self = Self(1);
//!     /// The timer started but got paused. This is separate from the game
//!     /// time being paused. Game time may even always be paused.
//!     pub const PAUSED: Self = Self(2);
//!     /// The timer has ended, but didn't get reset yet.
//!     pub const ENDED: Self = Self(3);
//! }
//!
//! extern "C" {
//!     /// Gets the state that the timer currently is in.
//!     pub fn timer_get_state() -> TimerState;
//!
//!     /// Starts the timer.
//!     pub fn timer_start();
//!     /// Splits the current segment.
//!     pub fn timer_split();
//!     /// Resets the timer.
//!     pub fn timer_reset();
//!     /// Sets a custom key value pair. This may be arbitrary information that
//!     /// the auto splitter wants to provide for visualization.
//!     pub fn timer_set_variable(
//!         key_ptr: *const u8,
//!         key_len: usize,
//!         value_ptr: *const u8,
//!         value_len: usize,
//!     );
//!
//!     /// Sets the game time.
//!     pub fn timer_set_game_time(secs: i64, nanos: i32);
//!     /// Pauses the game time. This does not pause the timer, only the
//!     /// automatic flow of time for the game time.
//!     pub fn timer_pause_game_time();
//!     /// Resumes the game time. This does not resume the timer, only the
//!     /// automatic flow of time for the game time.
//!     pub fn timer_resume_game_time();
//!
//!     /// Attaches to a process based on its name.
//!     pub fn process_attach(name_ptr: *const u8, name_len: usize) -> Option<ProcessId>;
//!     /// Detaches from a process.
//!     pub fn process_detach(process: ProcessId);
//!     /// Checks whether is a process is still open. You should detach from a
//!     /// process and stop using it if this returns `false`.
//!     pub fn process_is_open(process: ProcessId) -> bool;
//!     /// Reads memory from a process at the address given. This will write
//!     /// the memory to the buffer given. Returns `false` if this fails.
//!     pub fn process_read(
//!         process: ProcessId,
//!         address: Address,
//!         buf_ptr: *mut u8,
//!         buf_len: usize,
//!     ) -> bool;
//!     /// Gets the address of a module in a process.
//!     pub fn process_get_module_address(
//!         process: ProcessId,
//!         name_ptr: *const u8,
//!         name_len: usize,
//!     ) -> Option<NonZeroAddress>;
//!
//!     /// Sets the tick rate of the runtime. This influences the amount of
//!     /// times the `update` function is called per second.
//!     pub fn runtime_set_tick_rate(ticks_per_second: f64);
//!     /// Prints a log message for debugging purposes.
//!     pub fn runtime_print_message(text_ptr: *const u8, text_len: usize);
//! }
//! ```

use crate::timing::{SharedTimer, TimerPhase};
use crossbeam_channel::{bounded, unbounded, Sender};
use livesplit_auto_splitting::{
    CreationError, Runtime as ScriptRuntime, Timer as AutoSplitTimer, TimerState,
};
use snafu::Snafu;
use std::{
    path::PathBuf,
    thread::{self, JoinHandle},
};
use time::Duration;

/// An error that the [`Runtime`] can return.
#[derive(Debug, Snafu)]
pub enum Error {
    /// The runtime thread unexpectedly stopped.
    ThreadStopped,
    /// Failed loading the auto splitter.
    LoadFailed {
        /// The underlying error.
        source: CreationError,
    },
}

/// An auto splitter runtime that allows using an auto splitter provided as a
/// WebAssembly module to control a timer.
pub struct Runtime {
    sender: Sender<Request>,
    join_handle: Option<JoinHandle<Result<(), Error>>>,
}

impl Drop for Runtime {
    fn drop(&mut self) {
        self.sender.send(Request::End).ok();
        self.join_handle.take().unwrap().join().ok();
    }
}

impl Runtime {
    /// Starts the runtime. Doesn't actually load an auto splitter until
    /// [`load_script`][Runtime::load_script] is called.
    pub fn new(timer: SharedTimer) -> Self {
        let (sender, receiver) = unbounded();
        let join_handle = thread::spawn(move || -> Result<(), Error> {
            'back_to_not_having_a_runtime: loop {
                let mut runtime = loop {
                    match receiver.recv().map_err(|_| Error::ThreadStopped)? {
                        Request::LoadScript(script, ret) => {
                            match ScriptRuntime::new(&script, Timer(timer.clone())) {
                                Ok(r) => {
                                    ret.send(Ok(())).ok();
                                    break r;
                                }
                                Err(source) => ret.send(Err(Error::LoadFailed { source })).unwrap(),
                            };
                        }
                        Request::UnloadScript(ret) => {
                            log::warn!(target: "Auto Splitter", "Attempted to unload already unloaded script");
                            ret.send(()).ok();
                        }
                        Request::End => return Ok(()),
                    };
                };
                log::info!(target: "Auto Splitter", "Loaded script");
                loop {
                    if let Ok(request) = receiver.try_recv() {
                        match request {
                            Request::LoadScript(script, ret) => {
                                match ScriptRuntime::new(&script, Timer(timer.clone())) {
                                    Ok(r) => {
                                        ret.send(Ok(())).ok();
                                        runtime = r;
                                        log::info!(target: "Auto Splitter", "Reloaded script");
                                    }
                                    Err(source) => {
                                        ret.send(Err(Error::LoadFailed { source })).ok();
                                        log::info!(target: "Auto Splitter", "Failed to load");
                                    }
                                }
                            }
                            Request::UnloadScript(ret) => {
                                ret.send(()).ok();
                                log::info!(target: "Auto Splitter", "Unloaded script");
                                continue 'back_to_not_having_a_runtime;
                            }
                            Request::End => return Ok(()),
                        }
                    }
                    if let Err(e) = runtime.step() {
                        log::error!(target: "Auto Splitter", "Unloaded due to failure: {}", e);
                        continue 'back_to_not_having_a_runtime;
                    };
                    runtime.sleep();
                }
            }
        });

        Self {
            sender,
            join_handle: Some(join_handle),
        }
    }

    /// Attempts to load a wasm file containing an auto splitter module. This
    /// call will block until the auto splitter has either loaded successfully
    /// or failed.
    pub fn load_script(&self, script: PathBuf) -> Result<(), Error> {
        // FIXME: replace with `futures::channel::oneshot`
        let (sender, receiver) = bounded(1);
        self.sender
            .send(Request::LoadScript(script, sender))
            .map_err(|_| Error::ThreadStopped)?;
        receiver.recv().map_err(|_| Error::ThreadStopped)??;
        Ok(())
    }

    /// Unloads the current auto splitter. This will _not_ return an error if
    /// there isn't currently an auto splitter loaded, only if the runtime
    /// thread stops unexpectedly.
    pub fn unload_script(&self) -> Result<(), Error> {
        // FIXME: replace with `futures::channel::oneshot`
        let (sender, receiver) = bounded(1);
        self.sender
            .send(Request::UnloadScript(sender))
            .map_err(|_| Error::ThreadStopped)?;
        receiver.recv().map_err(|_| Error::ThreadStopped)
    }
}

enum Request {
    LoadScript(PathBuf, Sender<Result<(), Error>>),
    UnloadScript(Sender<()>),
    End,
}

// This newtype is required because SharedTimer is an Arc<RwLock<T>>, so we
// can't implement the trait directly on it.
struct Timer(SharedTimer);

impl AutoSplitTimer for Timer {
    fn state(&self) -> TimerState {
        match self.0.read().current_phase() {
            TimerPhase::NotRunning => TimerState::NotRunning,
            TimerPhase::Running => TimerState::Running,
            TimerPhase::Paused => TimerState::Paused,
            TimerPhase::Ended => TimerState::Ended,
        }
    }

    fn start(&mut self) {
        self.0.write().start()
    }

    fn split(&mut self) {
        self.0.write().split()
    }

    fn reset(&mut self) {
        self.0.write().reset(true)
    }

    fn set_game_time(&mut self, time: Duration) {
        self.0.write().set_game_time(time.into());
    }

    fn pause_game_time(&mut self) {
        self.0.write().pause_game_time()
    }

    fn resume_game_time(&mut self) {
        self.0.write().resume_game_time()
    }

    fn set_variable(&mut self, name: &str, value: &str) {
        self.0.write().set_custom_variable(name, value)
    }
}
