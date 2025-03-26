# <img src="https://raw.githubusercontent.com/LiveSplit/LiveSplit/master/res/Icon.svg" alt="LiveSplit" height="42" width="45" align="top"/> livesplit-auto-splitting

livesplit-auto-splitting is a library that provides a runtime for running
auto splitters that can control a speedrun timer. These auto splitters are
provided as WebAssembly modules.

## Requirements for the Auto Splitters

The auto splitters must provide an `update` function with the following
signature:

```rust
#[unsafe(no_mangle)]
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
pub struct AttachedProcess(NonZeroU64);

#[repr(transparent)]
pub struct ProcessId(u64);

#[repr(transparent)]
pub struct SettingsMap(NonZeroU64);

#[repr(transparent)]
pub struct SettingsList(NonZeroU64);

#[repr(transparent)]
pub struct SettingValue(NonZeroU64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct SettingValueType(u32);

impl SettingValueType {
    /// The setting value is a settings map.
    pub const MAP: Self = Self(1);
    /// The setting value is a settings list.
    pub const LIST: Self = Self(2);
    /// The setting value is a boolean.
    pub const BOOL: Self = Self(3);
    /// The setting value is a 64-bit signed integer.
    pub const I64: Self = Self(4);
    /// The setting value is a 64-bit floating point number.
    pub const F64: Self = Self(5);
    /// The setting value is a string.
    pub const STRING: Self = Self(6);
}

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

#[repr(transparent)]
pub struct MemoryRangeFlags(NonZeroU64);

impl MemoryRangeFlags {
    /// The memory range is readable.
    pub const READ: Self = Self(match NonZeroU64::new(1 << 1) { Some(v) => v, None => panic!() });
    /// The memory range is writable.
    pub const WRITE: Self = Self(match NonZeroU64::new(1 << 2) { Some(v) => v, None => panic!() });
    /// The memory range is executable.
    pub const EXECUTE: Self = Self(match NonZeroU64::new(1 << 3) { Some(v) => v, None => panic!() });
    /// The memory range has a file path.
    pub const PATH: Self = Self(match NonZeroU64::new(1 << 4) { Some(v) => v, None => panic!() });
}

unsafe extern "C" {
    /// Gets the state that the timer currently is in.
    pub safe fn timer_get_state() -> TimerState;

    /// Starts the timer.
    pub safe fn timer_start();
    /// Splits the current segment.
    pub safe fn timer_split();
    /// Skips the current split.
    pub safe fn timer_skip_split();
    /// Undoes the previous split.
    pub safe fn timer_undo_split();
    /// Resets the timer.
    pub safe fn timer_reset();
    /// Accesses the index of the split the attempt is currently on. If there's
    /// no attempt in progress, `-1` is returned instead. This returns an
    /// index that is equal to the amount of segments when the attempt is
    /// finished, but has not been reset. So you need to be careful when using
    /// this value for indexing.
    pub safe fn timer_current_split_index() -> i32;
    /// Sets a custom key value pair. This may be arbitrary information that the
    /// auto splitter wants to provide for visualization. The pointers need to
    /// point to valid UTF-8 encoded text with the respective given length.
    pub fn timer_set_variable(
        key_ptr: *const u8,
        key_len: usize,
        value_ptr: *const u8,
        value_len: usize,
    );

    /// Sets the game time.
    pub safe fn timer_set_game_time(secs: i64, nanos: i32);
    /// Pauses the game time. This does not pause the timer, only the
    /// automatic flow of time for the game time.
    pub safe fn timer_pause_game_time();
    /// Resumes the game time. This does not resume the timer, only the
    /// automatic flow of time for the game time.
    pub safe fn timer_resume_game_time();

    /// Attaches to a process based on its name. The pointer needs to point to
    /// valid UTF-8 encoded text with the given length. If multiple processes
    /// with the same name are running, the process that most recently started
    /// is being attached to.
    pub fn process_attach(name_ptr: *const u8, name_len: usize) -> Option<AttachedProcess>;
    /// Attaches to a process based on its process id.
    pub safe fn process_attach_by_pid(pid: ProcessId) -> Option<AttachedProcess>;
    /// Detaches from a process.
    pub fn process_detach(process: AttachedProcess);
    /// Lists processes based on their name. The name pointer needs to point to
    /// valid UTF-8 encoded text with the given length. Returns `false` if
    /// listing the processes failed. If it was successful, the buffer is now
    /// filled with the process ids. They are in no specific order. The
    /// `list_len_ptr` will be updated to the amount of process ids that were
    /// found. If this is larger than the original value provided, the buffer
    /// provided was too small and not all process ids could be stored. This is
    /// still considered successful and can optionally be treated as an error
    /// condition by the caller by checking if the length increased and
    /// potentially reallocating a larger buffer. If the length decreased after
    /// the call, the buffer was larger than needed and the remaining entries
    /// are untouched.
    pub fn process_list_by_name(
        name_ptr: *const u8,
        name_len: usize,
        list_ptr: *mut ProcessId,
        list_len_ptr: *mut usize,
    ) -> bool;
    /// Checks whether a process is still open. You should detach from a
    /// process and stop using it if this returns `false`.
    pub fn process_is_open(process: AttachedProcess) -> bool;
    /// Reads memory from a process at the address given. This will write
    /// the memory to the buffer given. Returns `false` if this fails.
    pub fn process_read(
        process: AttachedProcess,
        address: Address,
        buf_ptr: *mut u8,
        buf_len: usize,
    ) -> bool;
    /// Gets the address of a module in a process. The pointer needs to point to
    /// valid UTF-8 encoded text with the given length.
    pub fn process_get_module_address(
        process: AttachedProcess,
        name_ptr: *const u8,
        name_len: usize,
    ) -> Option<NonZeroAddress>;
    /// Gets the size of a module in a process. The pointer needs to point to
    /// valid UTF-8 encoded text with the given length.
    pub fn process_get_module_size(
        process: AttachedProcess,
        name_ptr: *const u8,
        name_len: usize,
    ) -> Option<NonZeroU64>;
    /// Stores the file system path of a module in a process in the buffer
    /// given. The pointer to the module name needs to point to valid UTF-8
    /// encoded text with the given length. The path is a path that is
    /// accessible through the WASI file system, so a Windows path of
    /// `C:\foo\bar.exe` would be returned as `/mnt/c/foo/bar.exe`. Returns
    /// `false` if the buffer is too small. After this call, no matter whether
    /// it was successful or not, the `buf_len_ptr` will be set to the required
    /// buffer size. If `false` is returned and the `buf_len_ptr` got set to 0,
    /// the path or the module does not exist or it failed to get read. The path
    /// is guaranteed to be valid UTF-8 and is not nul-terminated.
    pub fn process_get_module_path(
        process: AttachedProcess,
        name_ptr: *const u8,
        name_len: usize,
        buf_ptr: *mut u8,
        buf_len_ptr: *mut usize,
    ) -> bool;
    /// Stores the file system path of the executable in the buffer given. The
    /// path is a path that is accessible through the WASI file system, so a
    /// Windows path of `C:\foo\bar.exe` would be returned as
    /// `/mnt/c/foo/bar.exe`. Returns `false` if the buffer is too small. After
    /// this call, no matter whether it was successful or not, the `buf_len_ptr`
    /// will be set to the required buffer size. If `false` is returned and the
    /// `buf_len_ptr` got set to 0, the path does not exist or failed to get
    /// read. The path is guaranteed to be valid UTF-8 and is not
    /// nul-terminated.
    pub fn process_get_path(
        process: AttachedProcess,
        buf_ptr: *mut u8,
        buf_len_ptr: *mut usize,
    ) -> bool;
    /// Gets the number of memory ranges in a given process.
    pub fn process_get_memory_range_count(process: AttachedProcess) -> Option<NonZeroU64>;
    /// Gets the start address of a memory range by its index.
    pub fn process_get_memory_range_address(
        process: AttachedProcess,
        idx: u64,
    ) -> Option<NonZeroAddress>;
    /// Gets the size of a memory range by its index.
    pub fn process_get_memory_range_size(process: AttachedProcess, idx: u64) -> Option<NonZeroU64>;
    /// Gets the flags of a memory range by its index.
    pub fn process_get_memory_range_flags(process: AttachedProcess, idx: u64) -> Option<MemoryRangeFlags>;

    /// Sets the tick rate of the runtime. This influences the amount of
    /// times the `update` function is called per second.
    pub safe fn runtime_set_tick_rate(ticks_per_second: f64);
    /// Prints a log message for debugging purposes. The pointer needs to point
    /// to valid UTF-8 encoded text with the given length.
    pub fn runtime_print_message(text_ptr: *const u8, text_len: usize);
    /// Stores the name of the operating system that the runtime is running
    /// on in the buffer given. Returns `false` if the buffer is too small.
    /// After this call, no matter whether it was successful or not, the
    /// `buf_len_ptr` will be set to the required buffer size. The name is
    /// guaranteed to be valid UTF-8 and is not nul-terminated.
    /// Example values: `windows`, `linux`, `macos`
    pub fn runtime_get_os(buf_ptr: *mut u8, buf_len_ptr: *mut usize) -> bool;
    /// Stores the name of the architecture that the runtime is running on
    /// in the buffer given. Returns `false` if the buffer is too small.
    /// After this call, no matter whether it was successful or not, the
    /// `buf_len_ptr` will be set to the required buffer size. The name is
    /// guaranteed to be valid UTF-8 and is not nul-terminated.
    /// Example values: `x86`, `x86_64`, `arm`, `aarch64`
    pub fn runtime_get_arch(buf_ptr: *mut u8, buf_len_ptr: *mut usize) -> bool;

    /// Adds a new boolean setting that the user can modify. This will return
    /// either the specified default value or the value that the user has set.
    /// The key is used to store the setting and needs to be unique across all
    /// types of settings. The pointers need to point to valid UTF-8 encoded
    /// text with the respective given length.
    pub fn user_settings_add_bool(
        key_ptr: *const u8,
        key_len: usize,
        description_ptr: *const u8,
        description_len: usize,
        default_value: bool,
    ) -> bool;
    /// Adds a new title to the user settings. This is used to group settings
    /// together. The heading level determines the size of the title. The top
    /// level titles use a heading level of 0. The key needs to be unique across
    /// all types of settings. The pointers need to point to valid UTF-8 encoded
    /// text with the respective given length.
    pub fn user_settings_add_title(
        key_ptr: *const u8,
        key_len: usize,
        description_ptr: *const u8,
        description_len: usize,
        heading_level: u32,
    );
    /// Adds a new choice setting that the user can modify. This allows the user
    /// to choose between various options. The key is used to store the setting
    /// in the settings map and needs to be unique across all types of settings.
    /// The description is what's shown to the user. The key of the default
    /// option to show needs to be specified. The pointers need to point to
    /// valid UTF-8 encoded text with the respective given length.
    pub fn user_settings_add_choice(
        key_ptr: *const u8,
        key_len: usize,
        description_ptr: *const u8,
        description_len: usize,
        default_option_key_ptr: *const u8,
        default_option_key_len: usize,
    );
    /// Adds a new option to a choice setting. The key needs to match the key of
    /// the choice setting that it's supposed to be added to. The option key is
    /// used as the value to store when the user chooses this option. The
    /// description is what's shown to the user. The pointers need to point to
    /// valid UTF-8 encoded text with the respective given length. Returns
    /// `true` if the option is at this point in time chosen by the user.
    pub fn user_settings_add_choice_option(
        key_ptr: *const u8,
        key_len: usize,
        option_key_ptr: *const u8,
        option_key_len: usize,
        option_description_ptr: *const u8,
        option_description_len: usize,
    ) -> bool;
    /// Adds a new file select setting that the user can modify. This allows the
    /// user to choose a file from the file system. The key is used to store the
    /// path of the file in the settings map and needs to be unique across all
    /// types of settings. The description is what's shown to the user. The
    /// pointers need to point to valid UTF-8 encoded text with the respective
    /// given length. The path is a path that is accessible through the WASI
    /// file system, so a Windows path of `C:\foo\bar.exe` would be stored as
    /// `/mnt/c/foo/bar.exe`.
    pub fn user_settings_add_file_select(
        key_ptr: *const u8,
        key_len: usize,
        description_ptr: *const u8,
        description_len: usize,
    );
    /// Adds a filter to a file select setting. The key needs to match the key
    /// of the file select setting that it's supposed to be added to. The
    /// description is what's shown to the user for the specific filter. The
    /// description is optional. You may provide a null pointer if you don't
    /// want to specify a description. The pattern is a [glob
    /// pattern](https://en.wikipedia.org/wiki/Glob_(programming)) that is used
    /// to filter the files. The pattern generally only supports `*` wildcards,
    /// not `?` or brackets. This may however differ between frontends.
    /// Additionally `;` can't be used in Windows's native file dialog if it's
    /// part of the pattern. Multiple patterns may be specified by separating
    /// them with ASCII space characters. There are operating systems where glob
    /// patterns are not supported. A best effort lookup of the fitting MIME
    /// type may be used by a frontend on those operating systems instead. The
    /// pointers need to point to valid UTF-8 encoded text with the respective
    /// given length.
    pub fn user_settings_add_file_select_name_filter(
        key_ptr: *const u8,
        key_len: usize,
        description_ptr: *const u8,
        description_len: usize,
        pattern_ptr: *const u8,
        pattern_len: usize,
    );
    /// Adds a filter to a file select setting. The key needs to match the key
    /// of the file select setting that it's supposed to be added to. The MIME
    /// type is what's used to filter the files. Most operating systems do not
    /// support MIME types, but the frontends are encouraged to look up the file
    /// extensions that are associated with the MIME type and use those as a
    /// filter in those cases. You may also use wildcards as part of the MIME
    /// types such as `image/*`. The support likely also varies between
    /// frontends however. The pointers need to point to valid UTF-8 encoded
    /// text with the respective given length.
    pub fn user_settings_add_file_select_mime_filter(
        key_ptr: *const u8,
        key_len: usize,
        mime_type_ptr: *const u8,
        mime_type_len: usize,
    );
    /// Adds a tooltip to a setting based on its key. A tooltip is useful for
    /// explaining the purpose of a setting to the user. The pointers need to
    /// point to valid UTF-8 encoded text with the respective given length.
    pub fn user_settings_set_tooltip(
        key_ptr: *const u8,
        key_len: usize,
        tooltip_ptr: *const u8,
        tooltip_len: usize,
    );

    /// Creates a new settings map. You own the settings map and are responsible
    /// for freeing it.
    pub safe fn settings_map_new() -> SettingsMap;
    /// Frees a settings map.
    pub fn settings_map_free(map: SettingsMap);
    /// Loads a copy of the currently set global settings map. Any changes to it
    /// are only perceived if it's stored back. You own the settings map and are
    /// responsible for freeing it.
    pub safe fn settings_map_load() -> SettingsMap;
    /// Stores a copy of the settings map as the new global settings map. This
    /// will overwrite the previous global settings map. You still retain
    /// ownership of the map, which means you still need to free it. There's a
    /// chance that the settings map was changed in the meantime, so those
    /// changes could get lost. Prefer using `settings_map_store_if_unchanged`
    /// if you want to avoid that.
    pub fn settings_map_store(map: SettingsMap);
    /// Stores a copy of the new settings map as the new global settings map if
    /// the map has not changed in the meantime. This is done by comparing the
    /// old map. You still retain ownership of both maps, which means you still
    /// need to free them. Returns `true` if the map was stored successfully.
    /// Returns `false` if the map was changed in the meantime.
    pub fn settings_map_store_if_unchanged(old_map: SettingsMap, new_map: SettingsMap) -> bool;
    /// Copies a settings map. No changes inside the copy affect the original
    /// settings map. You own the new settings map and are responsible for
    /// freeing it.
    pub fn settings_map_copy(map: SettingsMap) -> SettingsMap;
    /// Inserts a copy of the setting value into the settings map based on the
    /// key. If the key already exists, it will be overwritten. You still retain
    /// ownership of the setting value, which means you still need to free it.
    /// The pointer needs to point to valid UTF-8 encoded text with the given
    /// length.
    pub fn settings_map_insert(
        map: SettingsMap,
        key_ptr: *const u8,
        key_len: usize,
        value: SettingValue,
    );
    /// Gets a copy of the setting value from the settings map based on the key.
    /// Returns `None` if the key does not exist. Any changes to it are only
    /// perceived if it's stored back. You own the setting value and are
    /// responsible for freeing it. The pointer needs to point to valid UTF-8
    /// encoded text with the given length.
    pub fn settings_map_get(
        map: SettingsMap,
        key_ptr: *const u8,
        key_len: usize,
    ) -> Option<SettingValue>;
    /// Gets the length of a settings map.
    pub fn settings_map_len(map: SettingsMap) -> u64;
    /// Gets the key of a setting value from the settings map based on the index
    /// by storing it into the buffer provided. Returns `false` if the buffer is
    /// too small. After this call, no matter whether it was successful or not,
    /// the `buf_len_ptr` will be set to the required buffer size. If `false` is
    /// returned and the `buf_len_ptr` got set to 0, the index is out of bounds.
    /// The key is guaranteed to be valid UTF-8 and is not nul-terminated.
    pub fn settings_map_get_key_by_index(
        map: SettingsMap,
        idx: u64,
        buf_ptr: *mut u8,
        buf_len_ptr: *mut usize,
    ) -> bool;
    /// Gets a copy of the setting value from the settings map based on the
    /// index. Returns `None` if the index is out of bounds. Any changes to it
    /// are only perceived if it's stored back. You own the setting value and
    /// are responsible for freeing it.
    pub fn settings_map_get_value_by_index(map: SettingsMap, idx: u64) -> Option<SettingValue>;

    /// Creates a new settings list. You own the settings list and are
    /// responsible for freeing it.
    pub safe fn settings_list_new() -> SettingsList;
    /// Frees a settings list.
    pub fn settings_list_free(list: SettingsList);
    /// Copies a settings list. No changes inside the copy affect the original
    /// settings list. You own the new settings list and are responsible for
    /// freeing it.
    pub fn settings_list_copy(list: SettingsList) -> SettingsList;
    /// Gets the length of a settings list.
    pub fn settings_list_len(list: SettingsList) -> u64;
    /// Gets a copy of the setting value from the settings list based on the
    /// index. Returns `None` if the index is out of bounds. Any changes to it
    /// are only perceived if it's stored back. You own the setting value and
    /// are responsible for freeing it.
    pub fn settings_list_get(list: SettingsList, idx: u64) -> Option<SettingValue>;
    /// Pushes a copy of the setting value to the end of the settings list. You
    /// still retain ownership of the setting value, which means you still need
    /// to free it.
    pub fn settings_list_push(list: SettingsList, value: SettingValue);
    /// Inserts a copy of the setting value into the settings list at the index
    /// given. Returns `false` if the index is out of bounds. No matter what
    /// happens, you still retain ownership of the setting value, which means
    /// you still need to free it.
    pub fn settings_list_insert(list: SettingsList, idx: u64, value: SettingValue) -> bool;

    /// Creates a new setting value from a settings map. The value is a copy of
    /// the settings map. Any changes to the original settings map afterwards
    /// are not going to be perceived by the setting value. You own the setting
    /// value and are responsible for freeing it. You also retain ownership of
    /// the settings map, which means you still need to free it.
    pub fn setting_value_new_map(value: SettingsMap) -> SettingValue;
    /// Creates a new setting value from a settings list. The value is a copy of
    /// the settings list. Any changes to the original settings list afterwards
    /// are not going to be perceived by the setting value. You own the setting
    /// value and are responsible for freeing it. You also retain ownership of
    /// the settings list, which means you still need to free it.
    pub fn setting_value_new_list(value: SettingsList) -> SettingValue;
    /// Creates a new boolean setting value. You own the setting value and are
    /// responsible for freeing it.
    pub safe fn setting_value_new_bool(value: bool) -> SettingValue;
    /// Creates a new 64-bit signed integer setting value. You own the setting
    /// value and are responsible for freeing it.
    pub safe fn setting_value_new_i64(value: i64) -> SettingValue;
    /// Creates a new 64-bit floating point setting value. You own the setting
    /// value and are responsible for freeing it.
    pub safe fn setting_value_new_f64(value: f64) -> SettingValue;
    /// Creates a new string setting value. The pointer needs to point to valid
    /// UTF-8 encoded text with the given length. You own the setting value and
    /// are responsible for freeing it.
    pub fn setting_value_new_string(value_ptr: *const u8, value_len: usize) -> SettingValue;
    /// Frees a setting value.
    pub fn setting_value_free(value: SettingValue);
    /// Copies a setting value. No changes inside the copy affect the original
    /// setting value. You own the new setting value and are responsible for
    /// freeing it.
    pub fn setting_value_copy(value: SettingValue) -> SettingValue;
    /// Gets the type of a setting value.
    pub fn setting_value_get_type(value: SettingValue) -> SettingValueType;
    /// Gets the value of a setting value as a settings map by storing it into
    /// the pointer provided. Returns `false` if the setting value is not a
    /// settings map. No value is stored into the pointer in that case. No
    /// matter what happens, you still retain ownership of the setting value,
    /// which means you still need to free it. You own the settings map and are
    /// responsible for freeing it.
    pub fn setting_value_get_map(value: SettingValue, value_ptr: *mut SettingsMap) -> bool;
    /// Gets the value of a setting value as a settings list by storing it into
    /// the pointer provided. Returns `false` if the setting value is not a
    /// settings list. No value is stored into the pointer in that case. No
    /// matter what happens, you still retain ownership of the setting value,
    /// which means you still need to free it. You own the settings list and are
    /// responsible for freeing it.
    pub fn setting_value_get_list(value: SettingValue, value_ptr: *mut SettingsList) -> bool;
    /// Gets the value of a boolean setting value by storing it into the pointer
    /// provided. Returns `false` if the setting value is not a boolean. No
    /// value is stored into the pointer in that case. No matter what happens,
    /// you still retain ownership of the setting value, which means you still
    /// need to free it.
    pub fn setting_value_get_bool(value: SettingValue, value_ptr: *mut bool) -> bool;
    /// Gets the value of a 64-bit signed integer setting value by storing it
    /// into the pointer provided. Returns `false` if the setting value is not a
    /// 64-bit signed integer. No value is stored into the pointer in that case.
    /// No matter what happens, you still retain ownership of the setting value,
    /// which means you still need to free it.
    pub fn setting_value_get_i64(value: SettingValue, value_ptr: *mut i64) -> bool;
    /// Gets the value of a 64-bit floating point setting value by storing it
    /// into the pointer provided. Returns `false` if the setting value is not a
    /// 64-bit floating point number. No value is stored into the pointer in
    /// that case. No matter what happens, you still retain ownership of the
    /// setting value, which means you still need to free it.
    pub fn setting_value_get_f64(value: SettingValue, value_ptr: *mut f64) -> bool;
    /// Gets the value of a string setting value by storing it into the buffer
    /// provided. Returns `false` if the buffer is too small or if the setting
    /// value is not a string. After this call, no matter whether it was
    /// successful or not, the `buf_len_ptr` will be set to the required buffer
    /// size. If `false` is returned and the `buf_len_ptr` got set to 0, the
    /// setting value is not a string. The string is guaranteed to be valid
    /// UTF-8 and is not nul-terminated. No matter what happens, you still
    /// retain ownership of the setting value, which means you still need to
    /// free it.
    pub fn setting_value_get_string(
        value: SettingValue,
        buf_ptr: *mut u8,
        buf_len_ptr: *mut usize,
    ) -> bool;
}
```

On top of the runtime's API, there's also WASI 0.1 support. Considering WASI
itself is still in preview, the API is subject to change. Auto splitters using
WASI may need to be recompiled in the future. Limitations of the WASI support:

- `stdout` and `stdin` are unbound. Those streams currently do nothing.
- `stderr` is available for logging purposes. It is line buffered. Only
  completed lines or flushing it will cause the output to be logged.
- The file system is currently almost entirely empty. The host's file system is
  accessible through `/mnt`. It is entirely read-only. Windows paths are mapped
  to `/mnt/c`, `/mnt/d`, etc. to match WSL.
- There are no environment variables.
- There are no command line arguments.
- There is no networking.
- There is no threading.
- Time and random numbers are available.
