//! livesplit-core supports autosplitters written in a variety of languages by
//! interpreting WebAssembly modules with [wasmtime](https://github.com/bytecodealliance/wasmtime).
//! A WASM module which provides the right set of functions can be loaded into
//! livesplit's runtime and directly control the timer.
//!
//! Here is the current interface that an autosplitter must expose:
//! ```
//! pub extern "C" fn register() {} // called on load
//! pub extern "C" fn update() {}   // called periodically
//! ```
//!
//! and here are the functions made available to it to control the timer:
//! ```
//! extern "C" {
//!    pub fn start();
//!    pub fn split();
//!    pub fn reset();
//!    pub fn print_message(ptr: u32, len: u32);
//!    pub fn set_variable(name_ptr: u32, name_len: u32, value_ptr: u32, value_len: u32);
//!    // game time will increment on its own unless paused by the autosplitter
//!    pub fn set_game_time(seconds: f64);
//!    pub fn pause_game_time();
//!    pub fn resume_game_time();
//!    pub fn set_tick_rate(ticks_per_sec: f64);
//!    // returns a value in [0,3] representing "not started", "running",
//!    // "paused", and "ended" respectively
//!    pub fn get_timer_state() -> u32;
//!    // autosplitters can request to attach to a process by name. this returns
//!    // an opaque handle (or zero on failure) which can be used to query for
//!    // modules/dlls by name and to read that processes memory.
//!    pub fn attach(ptr: u32, len: u32) -> u64;
//!    pub fn detach(handle: u64);
//!    // returns the base address of a module or zero if the module isn't found
//!    pub fn get_module(handle: u64, ptr: u32, len: u32) -> u64;
//!    // returns a boolean, 0=failure
//!    pub fn read_mem(handle: u64, address: u64, buf: u32, buf_len: u32) -> u32;
//! }
//! ```
// (TODO: link to an example autosplitter and/or a helper crate for writing
// them)

use {
    crate::timing::{SharedTimer, TimerPhase},
    crossbeam_channel::{bounded, unbounded, Sender},
    livesplit_auto_splitting::{
        Runtime as ScriptRuntime, Timer as AutoSplitTimer, TimerState,
    },
    std::{
        path::PathBuf,
        thread::{self, JoinHandle},
    },
    time::Duration,
};

/// Ways in which the autosplitter runtime can fail
#[derive(Debug, Copy, Clone, snafu::Snafu)]
pub enum Error {
    /// The autosplitter runtime's thread died
    ThreadStopped,
    /// The wasm module for the autosplitter failed to load
    LoadFailed,
}

type Result<T> = std::result::Result<T, Error>;

/// The autosplitter runtime is a thread running an event loop. It holds a
/// shared timer that can be updated by the wasm autosplitter.
///
/// The only communication possible with the runtime is to load or unload an
/// autosplitter. For passing arbitrary data such as death counts or collectables
/// from inside the autosplitter, use `set_variable()`.
pub struct Runtime {
    sender: Sender<Request>,
    join_handle: Option<JoinHandle<Result<()>>>,
}

impl Drop for Runtime {
    fn drop(&mut self) {
        self.sender.send(Request::End).ok();
        self.join_handle.take().unwrap().join().ok();
    }
}

impl Runtime {
    /// Starts the runtime. Doesn't actually load an autosplitter until
    /// [`load_script`][Runtime::load_script] is called.
    pub fn new(timer: SharedTimer) -> Self {
        let (sender, receiver) = unbounded();
        let join_handle = thread::spawn(move || -> Result<()> {
            'back_to_not_having_a_runtime: loop {
                let mut runtime = loop {
                    match receiver.recv().map_err(|_| Error::ThreadStopped)? {
                        Request::LoadScript(script, ret) => {
                            match ScriptRuntime::new(&script, AST(timer.clone())) {
                                Ok(r) => {
                                    ret.send(Ok(())).ok();
                                    break r;
                                }
                                Err(_) => ret.send(Err(Error::LoadFailed)).unwrap(),
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
                                match ScriptRuntime::new(&script, AST(timer.clone())) {
                                    Ok(r) => {
                                        ret.send(Ok(())).ok();
                                        runtime = r;
                                        log::info!(target: "Auto Splitter", "Reloaded script");
                                    }
                                    Err(_) => {
                                        ret.send(Err(Error::LoadFailed)).ok();
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

    /// Attempt to load a wasm file containing an autosplitter module. This call
    /// will block until the autosplitter has either loaded successfully or
    /// failed.
    pub fn load_script(&self, script: PathBuf) -> Result<()> {
        // TODO: replace with `futures::channel::oneshot`
        let (sender, receiver) = bounded(1);
        self.sender
            .send(Request::LoadScript(script, sender))
            .map_err(|_| Error::ThreadStopped)?;
        receiver
            .recv()
            .map_err(|_| Error::ThreadStopped)
            .and_then(std::convert::identity)
    }

    /// Unload the current autosplitter. This will _not_ return an error if
    /// there isn't currently an autosplitter loaded, only if the runtime thread
    /// stops unexpectedly.
    pub fn unload_script(&self) -> Result<()> {
        // TODO: replace with `futures:channel::oneshot`
        let (sender, receiver) = bounded(1);
        self.sender
            .send(Request::UnloadScript(sender))
            .map_err(|_| Error::ThreadStopped)?;
        receiver.recv().map_err(|_| Error::ThreadStopped)
    }
}

enum Request {
    LoadScript(PathBuf, Sender<Result<()>>),
    UnloadScript(Sender<()>),
    End,
}

// This newtype is required because SharedTimer is an Arc<RwLock<T>>, so we
// can't impl the autosplitter Timer trait directly on it
struct AST(SharedTimer);

impl AutoSplitTimer for AST {
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
