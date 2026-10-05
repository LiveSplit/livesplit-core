#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::needless_pass_by_ref_mut,
    missing_docs
)]
#![allow(clippy::missing_safety_doc, non_camel_case_types, non_snake_case)]

//! mod

#[cfg(any(
    feature = "attempt",
    feature = "run",
    feature = "run-metadata-custom-variables-iter",
    feature = "run-metadata-speedrun-com-variables-iter",
    feature = "segment",
    feature = "segment-history-element",
    feature = "segment-history-iter",
    feature = "timer",
))]
use std::cell::Cell;
use std::{cell::RefCell, ffi::CStr, os::raw::c_char, slice};
#[cfg(any(feature = "hotkey-config", feature = "layout", feature = "parsing"))]
use std::{fs::File, mem::ManuallyDrop};

#[cfg(feature = "analysis")]
pub mod analysis;
#[cfg(feature = "atomic-date-time")]
pub mod atomic_date_time;
#[cfg(feature = "attempt")]
pub mod attempt;
#[cfg(feature = "auto-splitting")]
pub mod auto_splitting_runtime;
#[cfg(feature = "blank-space-component")]
pub mod blank_space_component;
#[cfg(feature = "blank-space-component-state")]
pub mod blank_space_component_state;
#[cfg(feature = "carousel-component")]
pub mod carousel_component;
#[cfg(feature = "carousel-component-state")]
pub mod carousel_component_state;
#[cfg(feature = "command-sink")]
pub mod command_sink;
#[cfg(feature = "component")]
pub mod component;
#[cfg(feature = "current-comparison-component")]
pub mod current_comparison_component;
#[cfg(feature = "current-pace-component")]
pub mod current_pace_component;
#[cfg(feature = "delta-component")]
pub mod delta_component;
#[cfg(feature = "detailed-timer-component")]
pub mod detailed_timer_component;
#[cfg(feature = "detailed-timer-component-state")]
pub mod detailed_timer_component_state;
#[cfg(feature = "fuzzy-list")]
pub mod fuzzy_list;
#[cfg(feature = "general-layout-settings")]
pub mod general_layout_settings;
#[cfg(feature = "graph-component")]
pub mod graph_component;
#[cfg(feature = "graph-component-state")]
pub mod graph_component_state;
#[cfg(feature = "group-component")]
pub mod group_component;
#[cfg(feature = "group-component-state")]
pub mod group_component_state;
#[cfg(feature = "hotkey-config")]
pub mod hotkey_config;
#[cfg(feature = "hotkey-system")]
pub mod hotkey_system;
#[cfg(feature = "image-cache")]
pub mod image_cache;
#[cfg(feature = "key-value-component-state")]
pub mod key_value_component_state;
#[cfg(feature = "lang")]
pub mod lang;
#[cfg(feature = "layout")]
pub mod layout;
#[cfg(feature = "layout-editor")]
pub mod layout_editor;
#[cfg(feature = "layout-editor-state")]
pub mod layout_editor_state;
#[cfg(feature = "layout-state")]
pub mod layout_state;
#[cfg(feature = "linked-layout")]
pub mod linked_layout;
#[cfg(feature = "parse-run-result")]
pub mod parse_run_result;
#[cfg(feature = "pb-chance-component")]
pub mod pb_chance_component;
#[cfg(feature = "possible-time-save-component")]
pub mod possible_time_save_component;
#[cfg(feature = "potential-clean-up")]
pub mod potential_clean_up;
#[cfg(feature = "previous-segment-component")]
pub mod previous_segment_component;
#[cfg(feature = "run")]
pub mod run;
#[cfg(feature = "run-editor")]
pub mod run_editor;
#[cfg(feature = "run-metadata")]
pub mod run_metadata;
#[cfg(feature = "run-metadata-custom-variable")]
pub mod run_metadata_custom_variable;
#[cfg(feature = "run-metadata-custom-variables-iter")]
pub mod run_metadata_custom_variables_iter;
#[cfg(feature = "run-metadata-speedrun-com-variable")]
pub mod run_metadata_speedrun_com_variable;
#[cfg(feature = "run-metadata-speedrun-com-variables-iter")]
pub mod run_metadata_speedrun_com_variables_iter;
#[cfg(feature = "segment")]
pub mod segment;
#[cfg(feature = "segment-group")]
pub mod segment_group;
#[cfg(feature = "segment-history")]
pub mod segment_history;
#[cfg(feature = "segment-history-element")]
pub mod segment_history_element;
#[cfg(feature = "segment-history-iter")]
pub mod segment_history_iter;
#[cfg(feature = "segment-time-component")]
pub mod segment_time_component;
#[cfg(feature = "separator-component")]
pub mod separator_component;
#[cfg(feature = "separator-component-state")]
pub mod separator_component_state;
#[cfg(all(target_family = "wasm", feature = "server-protocol"))]
pub mod server_protocol;
#[cfg(feature = "setting-value")]
pub mod setting_value;
#[cfg(feature = "shared-timer")]
pub mod shared_timer;
#[cfg(feature = "software-rendering")]
pub mod software_renderer;
#[cfg(feature = "splits-component")]
pub mod splits_component;
#[cfg(feature = "splits-component-state")]
pub mod splits_component_state;
#[cfg(feature = "sum-of-best-cleaner")]
pub mod sum_of_best_cleaner;
#[cfg(feature = "sum-of-best-component")]
pub mod sum_of_best_component;
#[cfg(feature = "text-component")]
pub mod text_component;
#[cfg(feature = "text-component-state")]
pub mod text_component_state;
#[cfg(feature = "time")]
pub mod time;
#[cfg(feature = "time-span")]
pub mod time_span;
#[cfg(feature = "timer")]
pub mod timer;
#[cfg(feature = "timer-component")]
pub mod timer_component;
#[cfg(feature = "timer-component-state")]
pub mod timer_component_state;
#[cfg(feature = "timer-read-lock")]
pub mod timer_read_lock;
#[cfg(feature = "timer-write-lock")]
pub mod timer_write_lock;
#[cfg(feature = "title-component")]
pub mod title_component;
#[cfg(feature = "title-component-state")]
pub mod title_component_state;
#[cfg(feature = "total-playtime-component")]
pub mod total_playtime_component;
#[cfg(all(target_family = "wasm", feature = "web-command-sink"))]
pub mod web_command_sink;
#[cfg(all(target_family = "wasm", feature = "web-rendering"))]
pub mod web_rendering;
#[cfg(all(target_family = "wasm", feature = "therun-gg"))]
pub mod web_therun_gg;

#[cfg(feature = "run-metadata-custom-variables-iter")]
use crate::run_metadata_custom_variable::RunMetadataCustomVariable;
#[cfg(feature = "run-metadata-speedrun-com-variables-iter")]
use crate::run_metadata_speedrun_com_variable::RunMetadataSpeedrunComVariable;
#[cfg(feature = "segment-history-iter")]
use crate::segment_history_element::SegmentHistoryElement;
#[cfg(any(
    feature = "attempt",
    feature = "run",
    feature = "segment",
    feature = "segment-history-element",
    feature = "segment-history-iter",
    feature = "timer",
))]
use livesplit_core::{Time, TimeSpan};

/// type
pub type Json = *const c_char;
/// type
#[expect(non_camel_case_types)]
pub type Nullablec_char = c_char;

thread_local! {
    static OUTPUT_VEC: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    #[cfg(any(feature = "attempt", feature = "run", feature = "timer"))]
    static TIME_SPAN: Cell<TimeSpan> = const { Cell::new(TimeSpan::zero()) };
    #[cfg(any(feature = "attempt", feature = "segment", feature = "segment-history-element", feature = "timer"))]
    static TIME: Cell<Time> = const { Cell::new(Time::new()) };
    #[cfg(feature = "segment-history-iter")]
    static SEGMENT_HISTORY_ELEMENT: Cell<SegmentHistoryElement> = const { Cell::new((0, Time::new())) };
    #[cfg(feature = "run-metadata-speedrun-com-variables-iter")]
    static RUN_METADATA_SPEEDRUN_COM_VARIABLE: Cell<RunMetadataSpeedrunComVariable> = const { Cell::new(("", std::ptr::null())) };
    #[cfg(feature = "run-metadata-custom-variables-iter")]
    static RUN_METADATA_CUSTOM_VARIABLE: Cell<RunMetadataCustomVariable> = const { Cell::new(("", std::ptr::null())) };
}

#[cfg(any(feature = "attempt", feature = "run", feature = "timer"))]
fn output_time_span(time_span: TimeSpan) -> *const TimeSpan {
    TIME_SPAN.with(|output| {
        output.set(time_span);
        output.as_ptr() as *const TimeSpan
    })
}

#[cfg(any(
    feature = "attempt",
    feature = "segment",
    feature = "segment-history-element",
    feature = "timer",
))]
fn output_time(time: Time) -> *const Time {
    TIME.with(|output| {
        output.set(time);
        output.as_ptr() as *const Time
    })
}

fn output_str<S: AsRef<str>>(s: S) -> *const c_char {
    output_vec(|o| {
        o.extend_from_slice(s.as_ref().as_bytes());
    })
}

#[cfg(feature = "parsing")]
fn with_vec<F, R>(f: F) -> R
where
    F: FnOnce(&mut Vec<u8>) -> R,
{
    OUTPUT_VEC.with_borrow_mut(|output| {
        output.clear();
        f(output)
    })
}

fn output_vec<F>(f: F) -> *const c_char
where
    F: FnOnce(&mut Vec<u8>),
{
    OUTPUT_VEC.with_borrow_mut(|output| {
        output.clear();
        f(output);
        output.push(0);
        output.as_ptr() as *const c_char
    })
}

unsafe fn slice<T>(ptr: *const T, len: usize) -> &'static [T] {
    if len == 0 {
        &[]
    } else {
        // SAFETY: The caller guarantees that `ptr` is valid for `len`.
        unsafe { slice::from_raw_parts(ptr, len) }
    }
}

#[cfg(feature = "software-rendering")]
unsafe fn slice_mut<T>(ptr: *mut T, len: usize) -> &'static mut [T] {
    if len == 0 {
        &mut []
    } else {
        // SAFETY: The caller guarantees that `ptr` is valid for `len`.
        unsafe { slice::from_raw_parts_mut(ptr, len) }
    }
}

unsafe fn str(s: *const c_char) -> &'static str {
    if s.is_null() {
        ""
    } else {
        // SAFETY: The caller guarantees that `s` is valid.
        let bytes = unsafe { CStr::from_ptr(s as _).to_bytes() };

        // Depending on where the C API is used, you may be able to fully trust
        // that the caller always passes valid UTF-8. On the web we use the
        // `TextEncoder` which always produces valid UTF-8.
        #[cfg(any(
            feature = "assume-str-parameters-are-utf8",
            all(target_family = "wasm", feature = "wasm-web"),
        ))]
        {
            // SAFETY: The caller guarantees that `s` is valid UTF-8.
            unsafe { std::str::from_utf8_unchecked(bytes) }
        }
        #[cfg(not(any(
            feature = "assume-str-parameters-are-utf8",
            all(target_family = "wasm", feature = "wasm-web"),
        )))]
        {
            simdutf8::basic::from_utf8(bytes).unwrap()
        }
    }
}

// raw file descriptor handling
#[cfg(all(
    unix,
    any(feature = "hotkey-config", feature = "layout", feature = "parsing"),
))]
unsafe fn get_file(fd: i64) -> ManuallyDrop<File> {
    use std::os::unix::io::FromRawFd;
    // SAFETY: The caller guarantees that `fd` is valid.
    ManuallyDrop::new(unsafe { File::from_raw_fd(fd as _) })
}

#[cfg(all(
    windows,
    any(feature = "hotkey-config", feature = "layout", feature = "parsing"),
))]
unsafe fn get_file(handle: i64) -> ManuallyDrop<File> {
    use std::os::windows::io::FromRawHandle;
    // SAFETY: The caller guarantees that `handle` is valid.
    ManuallyDrop::new(unsafe { File::from_raw_handle(handle as *mut () as _) })
}

#[cfg(all(
    not(any(windows, unix)),
    any(feature = "hotkey-config", feature = "layout", feature = "parsing"),
))]
unsafe fn get_file(_: i64) -> ManuallyDrop<File> {
    panic!("File Descriptor Parsing is not implemented for this platform");
}

/// Allocate memory.
#[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
#[unsafe(no_mangle)]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    if size == 0 {
        std::ptr::NonNull::dangling().as_ptr()
    } else {
        // SAFETY: We checked that the size is not 0, so this is safe.
        unsafe { std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(size, 1)) }
    }
}

/// Deallocate memory.
#[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, cap: usize) {
    if cap != 0 {
        // SAFETY: We checked that the capacity is not 0 and the caller
        // guarantees that the pointer is valid to be deallocated for the given
        // capacity.
        unsafe {
            std::alloc::dealloc(ptr, std::alloc::Layout::from_size_align_unchecked(cap, 1));
        }
    }
}

/// Returns the byte length of the last nul-terminated string returned on the
/// current thread. The length excludes the nul-terminator.
#[unsafe(no_mangle)]
pub extern "C" fn get_buf_len() -> usize {
    OUTPUT_VEC.with_borrow(|v| v.len() - 1)
}
