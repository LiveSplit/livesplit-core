//! livesplit-auto-splitting is a library that provides a runtime for running
//! auto splitters that can control a speedrun timer. These auto splitters are
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
//! #[repr(transparent)]
//! pub struct MemoryRangeFlags(NonZeroU64);
//!
//! impl MemoryRangeFlags {
//!     /// The memory range is readable.
//!     pub const READ: Self = Self(NonZeroU64::new(1 << 1).unwrap());
//!     /// The memory range is writable.
//!     pub const WRITE: Self = Self(NonZeroU64::new(1 << 2).unwrap());
//!     /// The memory range is executable.
//!     pub const EXECUTE: Self = Self(NonZeroU64::new(1 << 3).unwrap());
//!     /// The memory range has a file path.
//!     pub const PATH: Self = Self(NonZeroU64::new(1 << 4).unwrap());
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
//!
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
//!     /// Gets the number of memory ranges in a given process.
//!     pub fn process_get_memory_range_count(process: ProcessId) -> Option<NonZeroU64>;
//!     /// Gets the start address of a memory range by its index.
//!     pub fn process_get_memory_range_address(
//!         process: ProcessId,
//!         idx: u64,
//!     ) -> Option<NonZeroAddress>;
//!     /// Gets the size of a memory range by its index.
//!     pub fn process_get_memory_range_size(process: ProcessId, idx: u64) -> Option<NonZeroU64>;
//!     /// Gets the flags of a memory range by its index.
//!     pub fn process_get_memory_range_flags(
//!         process: ProcessId,
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
//!         process: ProcessId,
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
//!
//! On top of the runtime's API, there's also WASI support. Considering WASI
//! itself is still in preview, the API is subject to change. Auto splitters
//! using WASI may need to be recompiled in the future. Limitations of the WASI
//! support:
//!
//! - `stout` / `stderr` / `stdin` are unbound. Those streams currently do
//!   nothing.
//! - The file system is currently almost entirely empty. The host's file system
//!   is accessible through `/mnt`. It is entirely read-only. Windows paths are
//!   mapped to `/mnt/c`, `/mnt/d`, etc. to match WSL.
//! - There are no environment variables.
//! - There are no command line arguments.
//! - There is no networking.
//! - There is no threading.

#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::missing_const_for_fn,
    missing_docs,
    rust_2018_idioms
)]

mod process;
mod runtime;
mod settings;
mod timer;

pub use runtime::{CreationError, InterruptHandle, Runtime};
pub use settings::{SettingValue, SettingsStore, UserSetting};
pub use time;
pub use timer::{Timer, TimerState};
