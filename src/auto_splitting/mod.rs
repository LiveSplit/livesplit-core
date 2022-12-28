//! The auto splitting module provides a runtime for running auto splitters that
//! can control the [`Timer`](crate::timing::Timer). These auto splitters are
//! provided as WebAssembly modules.
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
//!     /// Gets the size of a module in a process.
//!     pub fn process_get_module_size(
//!         process: ProcessId,
//!         name_ptr: *const u8,
//!         name_len: usize,
//!     ) -> Option<NonZeroU64>;
//!
//!     /// Sets the tick rate of the runtime. This influences the amount of
//!     /// times the `update` function is called per second.
//!     pub fn runtime_set_tick_rate(ticks_per_second: f64);
//!     /// Prints a log message for debugging purposes.
//!     pub fn runtime_print_message(text_ptr: *const u8, text_len: usize);
//!
//!     /// Adds a new setting that the user can modify. This will return either
//!     /// the specified default value or the value that the user has set.
//!     pub fn user_settings_add_bool(
//!         key_ptr: *const u8,
//!         key_len: usize,
//!         description_ptr: *const u8,
//!         description_len: usize,
//!         default_value: bool,
//!     ) -> bool;
//! }
//! ```

use crate::timing::{SharedTimer, TimerPhase};
use livesplit_auto_splitting::{
    CreationError, InterruptHandle, Runtime as ScriptRuntime, SettingsStore,
    Timer as AutoSplitTimer, TimerState,
};
use snafu::Snafu;
use std::{fmt, fs, io, path::PathBuf, thread, time::Duration};
use tokio::{
    runtime,
    sync::{mpsc, oneshot, watch},
    time::{timeout_at, Instant},
};

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
    /// Failed reading the auto splitter file.
    ReadFileFailed {
        /// The underlying error.
        source: io::Error,
    },
}

/// An auto splitter runtime that allows using an auto splitter provided as a
/// WebAssembly module to control a timer.
pub struct Runtime {
    interrupt_receiver: watch::Receiver<Option<InterruptHandle>>,
    sender: mpsc::UnboundedSender<Request>,
}

impl Drop for Runtime {
    fn drop(&mut self) {
        if let Some(handle) = &*self.interrupt_receiver.borrow() {
            handle.interrupt();
        }
    }
}

impl Runtime {
    /// Starts the runtime. Doesn't actually load an auto splitter until
    /// [`load_script`][Runtime::load_script] is called.
    pub fn new(timer: SharedTimer) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let (interrupt_sender, interrupt_receiver) = watch::channel(None);
        let (timeout_sender, timeout_receiver) = watch::channel(None);

        thread::Builder::new()
            .name("Auto Splitting Runtime".into())
            .spawn(move || {
                runtime::Builder::new_current_thread()
                    .enable_time()
                    .build()
                    .unwrap()
                    .block_on(run(receiver, timer, timeout_sender, interrupt_sender))
            })
            .unwrap();

        thread::Builder::new()
            .name("Auto Splitting Watchdog".into())
            .spawn({
                let interrupt_receiver = interrupt_receiver.clone();
                move || {
                    runtime::Builder::new_current_thread()
                        .enable_time()
                        .build()
                        .unwrap()
                        .block_on(watchdog(timeout_receiver, interrupt_receiver))
                }
            })
            .unwrap();

        Self {
            interrupt_receiver,
            sender,
        }
    }

    /// Attempts to load a wasm file containing an auto splitter module. This
    /// call will block until the auto splitter has either loaded successfully
    /// or failed.
    pub async fn load_script(&self, script: PathBuf) -> Result<(), Error> {
        let (sender, receiver) = oneshot::channel();
        let script = fs::read(script).map_err(|e| Error::ReadFileFailed { source: e })?;
        self.sender
            .send(Request::LoadScript(script, sender))
            .map_err(|_| Error::ThreadStopped)?;

        receiver.await.map_err(|_| Error::ThreadStopped)??;

        Ok(())
    }

    /// Attempts to load a wasm file containing an auto splitter module. This
    /// call will block until the auto splitter has either loaded successfully
    /// or failed.
    pub fn load_script_blocking(&self, script: PathBuf) -> Result<(), Error> {
        runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .unwrap()
            .block_on(self.load_script(script))
    }

    /// Unloads the current auto splitter. This will _not_ return an error if
    /// there isn't currently an auto splitter loaded, only if the runtime
    /// thread stops unexpectedly.
    pub async fn unload_script(&self) -> Result<(), Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .send(Request::UnloadScript(sender))
            .map_err(|_| Error::ThreadStopped)?;

        receiver.await.map_err(|_| Error::ThreadStopped)
    }

    /// Unloads the current auto splitter. This will _not_ return an error if
    /// there isn't currently an auto splitter loaded, only if the runtime
    /// thread stops unexpectedly.
    pub fn unload_script_blocking(&self) -> Result<(), Error> {
        runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .unwrap()
            .block_on(self.unload_script())
    }
}

enum Request {
    LoadScript(Vec<u8>, oneshot::Sender<Result<(), Error>>),
    UnloadScript(oneshot::Sender<()>),
}

// This newtype is required because [`SharedTimer`](crate::timing::SharedTimer)
// is an Arc<RwLock<T>>, so we can't implement the trait directly on it.
struct Timer(SharedTimer);

impl AutoSplitTimer for Timer {
    fn state(&self) -> TimerState {
        match self.0.read().unwrap().current_phase() {
            TimerPhase::NotRunning => TimerState::NotRunning,
            TimerPhase::Running => TimerState::Running,
            TimerPhase::Paused => TimerState::Paused,
            TimerPhase::Ended => TimerState::Ended,
        }
    }

    fn start(&mut self) {
        self.0.write().unwrap().start()
    }

    fn split(&mut self) {
        self.0.write().unwrap().split()
    }

    fn reset(&mut self) {
        self.0.write().unwrap().reset(true)
    }

    fn set_game_time(&mut self, time: time::Duration) {
        self.0.write().unwrap().set_game_time(time.into());
    }

    fn pause_game_time(&mut self) {
        self.0.write().unwrap().pause_game_time()
    }

    fn resume_game_time(&mut self) {
        self.0.write().unwrap().resume_game_time()
    }

    fn set_variable(&mut self, name: &str, value: &str) {
        self.0.write().unwrap().set_custom_variable(name, value)
    }

    fn log(&mut self, message: fmt::Arguments<'_>) {
        log::info!(target: "Auto Splitter", "{message}");
    }
}

async fn run(
    mut receiver: mpsc::UnboundedReceiver<Request>,
    timer: SharedTimer,
    timeout_sender: watch::Sender<Option<Instant>>,
    interrupt_sender: watch::Sender<Option<InterruptHandle>>,
) {
    'back_to_not_having_a_runtime: loop {
        interrupt_sender.send(None).ok();
        timeout_sender.send(None).ok();

        let mut runtime = loop {
            match receiver.recv().await {
                Some(Request::LoadScript(script, ret)) => {
                    match ScriptRuntime::new(&script, Timer(timer.clone()), SettingsStore::new()) {
                        Ok(r) => {
                            ret.send(Ok(())).ok();
                            break r;
                        }
                        Err(source) => {
                            ret.send(Err(Error::LoadFailed { source })).ok();
                        }
                    };
                }
                Some(Request::UnloadScript(ret)) => {
                    log::warn!(target: "Auto Splitter", "Attempted to unload already unloaded script");
                    ret.send(()).ok();
                }
                None => {
                    return;
                }
            };
        };

        log::info!(target: "Auto Splitter", "Loaded script");
        let mut next_step = Instant::now();
        interrupt_sender.send(Some(runtime.interrupt_handle())).ok();
        timeout_sender.send(Some(next_step)).ok();

        loop {
            match timeout_at(next_step, receiver.recv()).await {
                Ok(Some(request)) => match request {
                    Request::LoadScript(script, ret) => {
                        match ScriptRuntime::new(
                            &script,
                            Timer(timer.clone()),
                            SettingsStore::new(),
                        ) {
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
                },
                Ok(None) => return,
                Err(_) => match runtime.update() {
                    Ok(tick_rate) => {
                        next_step += tick_rate;
                        timeout_sender.send(Some(next_step)).ok();
                    }
                    Err(e) => {
                        log::error!(target: "Auto Splitter", "Unloaded due to failure: {:?}", e);
                        continue 'back_to_not_having_a_runtime;
                    }
                },
            }
        }
    }
}

async fn watchdog(
    mut timeout_receiver: watch::Receiver<Option<Instant>>,
    interrupt_receiver: watch::Receiver<Option<InterruptHandle>>,
) {
    const TIMEOUT: Duration = Duration::from_secs(5);

    loop {
        let instant = *timeout_receiver.borrow();
        match instant {
            Some(time) => match timeout_at(time + TIMEOUT, timeout_receiver.changed()).await {
                Ok(Ok(_)) => {}
                Ok(Err(_)) => return,
                Err(_) => {
                    if let Some(handle) = &*interrupt_receiver.borrow() {
                        handle.interrupt();
                    }
                }
            },
            None => {
                if timeout_receiver.changed().await.is_err() {
                    return;
                }
            }
        }
    }
}
