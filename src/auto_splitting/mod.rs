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
//! pub struct Process(NonZeroU64);
//!
//! #[repr(transparent)]
//! pub struct ProcessId(u64);
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
//! #[repr(transparent)]
//! pub struct MemoryRangeFlags(NonZeroU64);
//!
//! impl MemoryRangeFlags {
//!     /// The memory range is readable.
//!     pub const READ: Self = Self(match NonZeroU64::new(1 << 1) { Some(v) => v, None => panic!() });
//!     /// The memory range is writable.
//!     pub const WRITE: Self = Self(match NonZeroU64::new(1 << 2) { Some(v) => v, None => panic!() });
//!     /// The memory range is executable.
//!     pub const EXECUTE: Self = Self(match NonZeroU64::new(1 << 3) { Some(v) => v, None => panic!() });
//!     /// The memory range has a file path.
//!     pub const PATH: Self = Self(match NonZeroU64::new(1 << 4) { Some(v) => v, None => panic!() });
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
//!     /// Skips the current split.
//!     pub fn timer_skip_split();
//!     /// Undoes the previous split.
//!     pub fn timer_undo_split();
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
//!     pub fn process_attach(name_ptr: *const u8, name_len: usize) -> Option<Process>;
//!     /// Attaches to a process based on its process id.
//!     pub fn process_attach_by_pid(pid: ProcessId) -> Option<Process>;
//!     /// Detaches from a process.
//!     pub fn process_detach(process: Process);
//!     /// Lists processes based on their name. Returns `false` if listing the
//!     /// processes failed. If it was successful, the buffer is now filled
//!     /// with the process ids. They are in no specific order. The
//!     /// `list_len_ptr` will be updated to the amount of process ids that
//!     /// were found. If this is larger than the original value provided, the
//!     /// buffer provided was too small and not all process ids could be
//!     /// stored. This is still considered successful and can optionally be
//!     /// treated as an error condition by the caller by checking if the
//!     /// length increased and potentially reallocating a larger buffer. If
//!     /// the length decreased after the call, the buffer was larger than
//!     /// needed and the remaining entries are untouched.
//!     pub fn process_list_by_name(
//!         name_ptr: *const u8,
//!         name_len: usize,
//!         list_ptr: *mut ProcessId,
//!         list_len_ptr: *mut usize,
//!     ) -> bool;
//!     /// Checks whether is a process is still open. You should detach from a
//!     /// process and stop using it if this returns `false`.
//!     pub fn process_is_open(process: Process) -> bool;
//!     /// Reads memory from a process at the address given. This will write
//!     /// the memory to the buffer given. Returns `false` if this fails.
//!     pub fn process_read(
//!         process: Process,
//!         address: Address,
//!         buf_ptr: *mut u8,
//!         buf_len: usize,
//!     ) -> bool;
//!
//!     /// Gets the address of a module in a process.
//!     pub fn process_get_module_address(
//!         process: Process,
//!         name_ptr: *const u8,
//!         name_len: usize,
//!     ) -> Option<NonZeroAddress>;
//!     /// Gets the size of a module in a process.
//!     pub fn process_get_module_size(
//!         process: Process,
//!         name_ptr: *const u8,
//!         name_len: usize,
//!     ) -> Option<NonZeroU64>;
//!
//!     /// Gets the number of memory ranges in a given process.
//!     pub fn process_get_memory_range_count(process: Process) -> Option<NonZeroU64>;
//!     /// Gets the start address of a memory range by its index.
//!     pub fn process_get_memory_range_address(
//!         process: Process,
//!         idx: u64,
//!     ) -> Option<NonZeroAddress>;
//!     /// Gets the size of a memory range by its index.
//!     pub fn process_get_memory_range_size(process: Process, idx: u64) -> Option<NonZeroU64>;
//!     /// Gets the flags of a memory range by its index.
//!     pub fn process_get_memory_range_flags(
//!         process: Process,
//!         idx: u64,
//!     ) -> Option<MemoryRangeFlags>;
//!
//!     /// Stores the file system path of the executable in the buffer given. The
//!     /// path is a path that is accessible through the WASI file system, so a
//!     /// Windows path of `C:\foo\bar.exe` would be returned as
//!     /// `/mnt/c/foo/bar.exe`. Returns `false` if the buffer is too small. After
//!     /// this call, no matter whether it was successful or not, the
//!     /// `buf_len_ptr` will be set to the required buffer size. The path is
//!     /// guaranteed to be valid UTF-8 and is not nul-terminated.
//!     pub fn process_get_path(
//!         process: Process,
//!         buf_ptr: *mut u8,
//!         buf_len_ptr: *mut usize,
//!     ) -> bool;
//!
//!     /// Sets the tick rate of the runtime. This influences the amount of
//!     /// times the `update` function is called per second.
//!     pub fn runtime_set_tick_rate(ticks_per_second: f64);
//!     /// Prints a log message for debugging purposes.
//!     pub fn runtime_print_message(text_ptr: *const u8, text_len: usize);
//!     /// Stores the name of the operating system that the runtime is running
//!     /// on in the buffer given. Returns `false` if the buffer is too small.
//!     /// After this call, no matter whether it was successful or not, the
//!     /// `buf_len_ptr` will be set to the required buffer size. The name is
//!     /// guaranteed to be valid UTF-8 and is not nul-terminated.
//!     /// Example values: `windows`, `linux`, `macos`
//!     pub fn runtime_get_os(buf_ptr: *mut u8, buf_len_ptr: *mut usize) -> bool;
//!     /// Stores the name of the architecture that the runtime is running on
//!     /// in the buffer given. Returns `false` if the buffer is too small.
//!     /// After this call, no matter whether it was successful or not, the
//!     /// `buf_len_ptr` will be set to the required buffer size. The name is
//!     /// guaranteed to be valid UTF-8 and is not nul-terminated.
//!     /// Example values: `x86`, `x86_64`, `arm`, `aarch64`
//!     pub fn runtime_get_arch(buf_ptr: *mut u8, buf_len_ptr: *mut usize) -> bool;
//!
//!     /// Adds a new boolean setting that the user can modify. This will return
//!     /// either the specified default value or the value that the user has set.
//!     /// The key is used to store the setting and needs to be unique across all
//!     /// types of settings.
//!     pub fn user_settings_add_bool(
//!         key_ptr: *const u8,
//!         key_len: usize,
//!         description_ptr: *const u8,
//!         description_len: usize,
//!         default_value: bool,
//!     ) -> bool;
//!     /// Adds a new title to the user settings. This is used to group settings
//!     /// together. The heading level determines the size of the title. The top
//!     /// level titles use a heading level of 0. The key needs to be unique across
//!     /// all types of settings.
//!     pub fn user_settings_add_title(
//!         key_ptr: *const u8,
//!         key_len: usize,
//!         description_ptr: *const u8,
//!         description_len: usize,
//!         heading_level: u32,
//!     );
//!     /// Adds a tooltip to a setting based on its key. A tooltip is useful for
//!     /// explaining the purpose of a setting to the user.
//!     pub fn user_settings_set_tooltip(
//!         key_ptr: *const u8,
//!         key_len: usize,
//!         tooltip_ptr: *const u8,
//!         tooltip_len: usize,
//!     );
//!     /// Gets the AutoSplitterSettings as a settings object
//!     /// and puts it in the obj_ptr.
//!     /// This can be queried by functions such as
//!     /// settings_object_as_bool, settings_object_list_get,
//!     /// and settings_object_dict_get.
//!     pub fn get_auto_splitter_settings(obj_ptr: *mut u64) -> bool;
//!     /// Query a settings object as a bool:
//!     ///   1 for Some(true)
//!     ///   0 for Some(false)
//!     ///   -1 for None
//!     pub fn settings_object_as_bool(obj: u64) -> i32;
//!     /// Query a settings object as a list, get the length:
//!     ///   non-negative values for Some(len)
//!     ///   -1 for None
//!     pub fn settings_object_list_len(obj: u64) -> i32;
//!     /// Query a settings object as a list, get an element by index,
//!     /// putting the element at elem_ptr.
//!     pub fn settings_object_list_get(obj: u64, index: u32, elem_ptr: *mut u64) -> bool;
//!     /// Query a settings object as a dict, get a value by key,
//!     /// putting the value at value_ptr.
//!     pub fn settings_object_dict_get(obj: u64, key_ptr: *const u8, key_len: usize, value_ptr: *mut u64) -> bool;
//!     /// Query a settings object as a string, storing the
//!     /// contents in the buffer given.
//!     /// Returns `false` if the buffer is too small.
//!     /// After this call, whether it was successful or not,
//!     /// the `buf_len_ptr` will be set to the required buffer size.
//!     pub fn settings_object_as_str(obj: u64, buf_ptr: *mut u8, buf_len_ptr: *mut usize) -> bool;
//! }
//! ```
//!
//! On top of the runtime's API, there's also WASI support. Considering WASI
//! itself is still in preview, the API is subject to change. Auto splitters
//! using WASI may need to be recompiled in the future. Limitations of the WASI
//! support:
//!
//! - `stdout` / `stderr` / `stdin` are unbound. Those streams currently do
//!   nothing.
//! - The file system is currently almost entirely empty. The host's file system
//!   is accessible through `/mnt`. It is entirely read-only. Windows paths are
//!   mapped to `/mnt/c`, `/mnt/d`, etc. to match WSL.
//! - There are no environment variables.
//! - There are no command line arguments.
//! - There is no networking.
//! - There is no threading.

use crate::timing::{SharedTimer, TimerPhase};
use livesplit_auto_splitting::{
    Config, CreationError, InterruptHandle, Runtime as ScriptRuntime, Timer as AutoSplitTimer,
    TimerState,
};
pub use livesplit_auto_splitting::{SettingValue, SettingsStore, UserSetting, UserSettingKind};
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
    /// Tried reloading the auto splitter when no auto splitter is loaded
    ImpossibleReload,
    /// Failed reading the auto splitter file.
    ReadFileFailed {
        /// The underlying error.
        source: io::Error,
    },
    /// Failed reading the auto splitter settings.
    SettingsLoadFailed,
    /// The asked setting was not found.
    SettingNotFound,
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

    /// Attempts to reload the currently loaded auto splitter.
    /// This call will block until the auto splitter has either reloaded successfully
    /// or failed to reload.
    pub async fn reload_script(&self) -> Result<(), Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .send(Request::ReloadScript(sender))
            .map_err(|_| Error::ThreadStopped)?;

        receiver.await.map_err(|_| Error::ThreadStopped)??;

        Ok(())
    }

    /// Attempts to reload the currently loaded auto splitter.
    /// This call will block until the auto splitter has either reloaded successfully
    /// or failed to reload.
    pub fn reload_script_blocking(&self) -> Result<(), Error> {
        runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .unwrap()
            .block_on(self.reload_script())
    }

    /// Get the custom auto splitter settings
    pub async fn get_settings(&self) -> Result<Vec<UserSetting>, Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .send(Request::GetSettings(sender))
            .map_err(|_| Error::ThreadStopped)?;

        let result = receiver.await;

        match result {
            Ok(settings) => match settings {
                Some(settings) => Ok(settings),
                None => Err(Error::SettingsLoadFailed),
            },
            Err(_) => Err(Error::ThreadStopped),
        }
    }

    /// Get the custom auto splitter settings
    pub fn get_settings_blocking(&self) -> Result<Vec<UserSetting>, Error> {
        runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .unwrap()
            .block_on(self.get_settings())
    }

    /// Get the value for a custom auto splitter setting
    pub async fn get_setting_value(&self, key: String) -> Result<SettingValue, Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .send(Request::GetSettingValue(key, sender))
            .map_err(|_| Error::ThreadStopped)?;

        let result = receiver.await;

        match result {
            Ok(setting_value) => match setting_value {
                Some(setting_value) => Ok(setting_value),
                None => Err(Error::SettingNotFound),
            },
            Err(_) => Err(Error::ThreadStopped),
        }
    }

    /// Get the value for a custom auto splitter setting
    pub fn get_setting_value_blocking(&self, key: String) -> Result<SettingValue, Error> {
        runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .unwrap()
            .block_on(self.get_setting_value(key))
    }

    /// Set the value for a custom auto splitter setting
    pub async fn set_setting_value(&self, key: String, value: SettingValue) -> Result<(), Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .send(Request::SetSettingValue(key, value, sender))
            .map_err(|_| Error::ThreadStopped)?;

        receiver.await.map_err(|_| Error::ThreadStopped)
    }

    /// Set the value for a custom auto splitter setting
    pub fn set_setting_value_blocking(
        &self,
        key: String,
        value: SettingValue,
    ) -> Result<(), Error> {
        runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .unwrap()
            .block_on(self.set_setting_value(key, value))
    }
}

enum Request {
    LoadScript(Vec<u8>, oneshot::Sender<Result<(), Error>>),
    UnloadScript(oneshot::Sender<()>),
    ReloadScript(oneshot::Sender<Result<(), Error>>),
    GetSettings(oneshot::Sender<Option<Vec<UserSetting>>>),
    GetSettingValue(String, oneshot::Sender<Option<SettingValue>>),
    SetSettingValue(String, SettingValue, oneshot::Sender<()>),
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

    fn skip_split(&mut self) {
        self.0.write().unwrap().skip_split()
    }

    fn undo_split(&mut self) {
        self.0.write().unwrap().undo_split()
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

        let mut script_path;

        let mut runtime = loop {
            match receiver.recv().await {
                Some(Request::LoadScript(script, ret)) => {
                    let mut config = Config::default();
                    if let Ok(t) = timer.read() {
                        let s = t.run().auto_splitter_settings().to_string();
                        let ss = SettingsStore::new_auto_splitter_settings(s);
                        config.settings_store = Some(ss)
                    }
                    match ScriptRuntime::new(&script, Timer(timer.clone()), config) {
                        Ok(r) => {
                            ret.send(Ok(())).ok();
                            script_path = script;
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
                Some(Request::ReloadScript(ret)) => {
                    log::warn!(target: "Auto Splitter", "Attempted to reload a non existing script");
                    ret.send(Err(Error::ImpossibleReload)).ok();
                }
                Some(Request::GetSettings(ret)) => {
                    log::warn!(target: "Auto Splitter", "Attempted to get the settings when no script is loaded");
                    ret.send(None).ok();
                }
                Some(Request::GetSettingValue(_, ret)) => {
                    log::warn!(target: "Auto Splitter", "Attempted to get a setting value when no script is loaded");
                    ret.send(None).ok();
                }
                Some(Request::SetSettingValue(_, _, ret)) => {
                    log::warn!(target: "Auto Splitter", "Attempted to set a setting value when no script is loaded");
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
                        let mut config = Config::default();
                        config.settings_store = Some(runtime.settings_store().clone());

                        match ScriptRuntime::new(&script, Timer(timer.clone()), config) {
                            Ok(r) => {
                                ret.send(Ok(())).ok();
                                runtime = r;
                                script_path = script;
                                log::info!(target: "Auto Splitter", "Loaded new script");
                            }
                            Err(source) => {
                                ret.send(Err(Error::LoadFailed { source })).ok();
                                log::info!(target: "Auto Splitter", "Failed to load the new script");
                            }
                        }
                    }
                    Request::UnloadScript(ret) => {
                        ret.send(()).ok();
                        log::info!(target: "Auto Splitter", "Unloaded script");
                        continue 'back_to_not_having_a_runtime;
                    }
                    Request::ReloadScript(ret) => {
                        let mut config = Config::default();
                        config.settings_store = Some(runtime.settings_store().clone());

                        match ScriptRuntime::new(&script_path, Timer(timer.clone()), config) {
                            Ok(r) => {
                                ret.send(Ok(())).ok();
                                runtime = r;
                                log::info!(target: "Auto Splitter", "Reloaded script");
                            }
                            Err(source) => {
                                ret.send(Err(Error::LoadFailed { source })).ok();
                                log::info!(target: "Auto Splitter", "Failed to reload the script");
                            }
                        }
                    }
                    Request::GetSettings(ret) => {
                        ret.send(Some(runtime.user_settings().to_vec())).ok();
                        log::info!(target: "Auto Splitter", "Getting the settings");
                    }
                    Request::GetSettingValue(key, ret) => {
                        let setting_value = runtime.settings_store().get(key.as_str());

                        let user_setting_value = match runtime
                            .user_settings()
                            .iter()
                            .find(|x| x.key == key.clone().into_boxed_str())
                        {
                            Some(user_setting) => match user_setting.kind {
                                UserSettingKind::Bool { default_value } => {
                                    Some(SettingValue::Bool(default_value))
                                }
                                UserSettingKind::Title { heading_level: _ } => None,
                            },
                            None => None,
                        };

                        if setting_value.is_some() {
                            ret.send(setting_value.cloned()).ok();
                        } else {
                            ret.send(user_setting_value).ok();
                        }

                        log::info!(target: "Auto Splitter", "Getting value for {}", key);
                    }
                    Request::SetSettingValue(key, value, ret) => {
                        runtime
                            .settings_store_mut()
                            .set(key.clone().into_boxed_str(), value);
                        ret.send(()).ok();
                        log::info!(target: "Auto Splitter", "Setting value for {}", key);
                    }
                },
                Ok(None) => return,
                Err(_) => match runtime.update() {
                    Ok(()) => {
                        next_step = next_step
                            .into_std()
                            .checked_add(runtime.tick_rate())
                            .map_or(next_step, |t| t.into());

                        timeout_sender.send(Some(next_step)).ok();
                    }
                    Err(e) => {
                        log::error!(target: "Auto Splitter", "Unloaded, because the script trapped: {:?}", e);
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
