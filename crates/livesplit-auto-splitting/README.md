# <img src="https://raw.githubusercontent.com/LiveSplit/LiveSplit/master/LiveSplit/Resources/Icon.png" alt="LiveSplit" height="42" width="45" align="top"/> livesplit-auto-splitting

livesplit-auto-splitting is a library that provides a runtime for running
auto splitters that can control a speedrun timer. These auto splitters are
provided as WebAssembly modules.

## Requirements for the Auto Splitters

The auto splitters must provide an `update` function with the following
signature:

```rust
#[no_mangle]
pub extern "C" fn update() {}
```

This function is called periodically by the runtime at the configured tick
rate. The tick rate is 120 Hz by default, but can be changed by the auto
splitter.

In addition the WebAssembly module is expected to export a memory called
`memory`.

## API exposed to the Auto Splitters

The following functions are provided to the auto splitters in the module
`env`:

```rust
#[repr(transparent)]
pub struct Address(pub u64);

#[repr(transparent)]
pub struct NonZeroAddress(pub NonZeroU64);

#[repr(transparent)]
pub struct ProcessId(NonZeroU64);

#[repr(transparent)]
pub struct TimerState(u32);

impl TimerState {
    /// The timer is not running.
    pub const NOT_RUNNING: Self = Self(0);
    /// The timer is running.
    pub const RUNNING: Self = Self(1);
    /// The timer started but got paused. This is separate from the game
    /// time being paused. Game time may even always be paused.
    pub const PAUSED: Self = Self(2);
    /// The timer has ended, but didn't get reset yet.
    pub const ENDED: Self = Self(3);
}

extern "C" {
    /// Gets the state that the timer currently is in.
    pub fn timer_get_state() -> TimerState;

    /// Starts the timer.
    pub fn timer_start();
    /// Splits the current segment.
    pub fn timer_split();
    /// Resets the timer.
    pub fn timer_reset();
    /// Sets a custom key value pair. This may be arbitrary information that
    /// the auto splitter wants to provide for visualization.
    pub fn timer_set_variable(
        key_ptr: *const u8,
        key_len: usize,
        value_ptr: *const u8,
        value_len: usize,
    );

    /// Sets the game time.
    pub fn timer_set_game_time(secs: i64, nanos: i32);
    /// Pauses the game time. This does not pause the timer, only the
    /// automatic flow of time for the game time.
    pub fn timer_pause_game_time();
    /// Resumes the game time. This does not resume the timer, only the
    /// automatic flow of time for the game time.
    pub fn timer_resume_game_time();

    /// Attaches to a process based on its name.
    pub fn process_attach(name_ptr: *const u8, name_len: usize) -> Option<ProcessId>;
    /// Detaches from a process.
    pub fn process_detach(process: ProcessId);
    /// Checks whether is a process is still open. You should detach from a
    /// process and stop using it if this returns `false`.
    pub fn process_is_open(process: ProcessId) -> bool;
    /// Reads memory from a process at the address given. This will write
    /// the memory to the buffer given. Returns `false` if this fails.
    pub fn process_read(
        process: ProcessId,
        address: Address,
        buf_ptr: *mut u8,
        buf_len: usize,
    ) -> bool;
    /// Gets the address of a module in a process.
    pub fn process_get_module_address(
        process: ProcessId,
        name_ptr: *const u8,
        name_len: usize,
    ) -> Option<NonZeroAddress>;
    /// Gets the size of a module in a process.
    pub fn process_get_module_size(
        process: ProcessId,
        name_ptr: *const u8,
        name_len: usize,
    ) -> Option<NonZeroU64>;

    /// Sets the tick rate of the runtime. This influences the amount of
    /// times the `update` function is called per second.
    pub fn runtime_set_tick_rate(ticks_per_second: f64);
    /// Prints a log message for debugging purposes.
    pub fn runtime_print_message(text_ptr: *const u8, text_len: usize);

    /// Adds a new setting that the user can modify. This will return either
    /// the specified default value or the value that the user has set.
    pub fn user_settings_add_bool(
        key_ptr: *const u8,
        key_len: usize,
        description_ptr: *const u8,
        description_len: usize,
        default_value: bool,
    ) -> bool;
}
```

On top of the runtime's API, there's also unstable `WASI` support via the
`unstable` feature.
