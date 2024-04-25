#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::needless_pass_by_ref_mut,
    missing_docs,
    rust_2018_idioms
)]
#![allow(clippy::missing_safety_doc, non_camel_case_types, non_snake_case)]

//! mod

use std::{
    cell::{Cell, RefCell},
    ffi::CStr,
    fs::File,
    mem::ManuallyDrop,
    os::raw::c_char,
    ptr, slice,
};

pub mod analysis;
pub mod atomic_date_time;
pub mod attempt;
pub mod auto_splitting_runtime;
pub mod blank_space_component;
pub mod blank_space_component_state;
pub mod component;
pub mod current_comparison_component;
pub mod current_pace_component;
pub mod delta_component;
pub mod detailed_timer_component;
pub mod detailed_timer_component_state;
pub mod fuzzy_list;
pub mod general_layout_settings;
pub mod graph_component;
pub mod graph_component_state;
pub mod hotkey_config;
pub mod hotkey_system;
pub mod image_cache;
pub mod key_value_component_state;
pub mod layout;
pub mod layout_editor;
pub mod layout_editor_state;
pub mod layout_state;
pub mod linked_layout;
pub mod parse_run_result;
pub mod pb_chance_component;
pub mod possible_time_save_component;
pub mod potential_clean_up;
pub mod previous_segment_component;
pub mod run;
pub mod run_editor;
pub mod run_metadata;
pub mod run_metadata_custom_variable;
pub mod run_metadata_custom_variables_iter;
pub mod run_metadata_speedrun_com_variable;
pub mod run_metadata_speedrun_com_variables_iter;
pub mod segment;
pub mod segment_history;
pub mod segment_history_element;
pub mod segment_history_iter;
pub mod segment_time_component;
pub mod separator_component;
pub mod separator_component_state;
pub mod setting_value;
pub mod shared_timer;
pub mod software_renderer;
pub mod splits_component;
pub mod splits_component_state;
pub mod sum_of_best_cleaner;
pub mod sum_of_best_component;
pub mod text_component;
pub mod text_component_state;
pub mod time;
pub mod time_span;
pub mod timer;
pub mod timer_component;
pub mod timer_component_state;
pub mod timer_read_lock;
pub mod timer_write_lock;
pub mod title_component;
pub mod title_component_state;
pub mod total_playtime_component;
#[cfg(all(target_family = "wasm", feature = "web-rendering"))]
pub mod web_rendering;

use crate::{
    run_metadata_custom_variable::RunMetadataCustomVariable,
    run_metadata_speedrun_com_variable::RunMetadataSpeedrunComVariable,
    segment_history_element::SegmentHistoryElement,
};
use livesplit_core::{Time, TimeSpan};

/// type
pub type Json = *const c_char;
/// type
pub type Nullablec_char = c_char;

thread_local! {
    static OUTPUT_VEC: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    static TIME_SPAN: Cell<TimeSpan> = const { Cell::new(TimeSpan::zero()) };
    static TIME: Cell<Time> = const { Cell::new(Time::new()) };
    static SEGMENT_HISTORY_ELEMENT: Cell<SegmentHistoryElement> = const { Cell::new((0, Time::new())) };
    static RUN_METADATA_SPEEDRUN_COM_VARIABLE: Cell<RunMetadataSpeedrunComVariable> = const { Cell::new(("", ptr::null())) };
    static RUN_METADATA_CUSTOM_VARIABLE: Cell<RunMetadataCustomVariable> = const { Cell::new(("", ptr::null())) };
}

fn output_time_span(time_span: TimeSpan) -> *const TimeSpan {
    TIME_SPAN.with(|output| {
        output.set(time_span);
        output.as_ptr() as *const TimeSpan
    })
}

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
        slice::from_raw_parts(ptr, len)
    }
}

unsafe fn slice_mut<T>(ptr: *mut T, len: usize) -> &'static mut [T] {
    if len == 0 {
        &mut []
    } else {
        slice::from_raw_parts_mut(ptr, len)
    }
}

unsafe fn str(s: *const c_char) -> &'static str {
    if s.is_null() {
        ""
    } else {
        let bytes = CStr::from_ptr(s as _).to_bytes();

        // Depending on where the C API is used, you may be able to fully trust
        // that the caller always passes valid UTF-8. On the web we use the
        // `TextEncoder` which always produces valid UTF-8.
        #[cfg(any(
            feature = "assume-str-parameters-are-utf8",
            all(target_family = "wasm", feature = "wasm-web"),
        ))]
        {
            std::str::from_utf8_unchecked(bytes)
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
#[cfg(unix)]
unsafe fn get_file(fd: i64) -> ManuallyDrop<File> {
    use std::os::unix::io::FromRawFd;
    ManuallyDrop::new(File::from_raw_fd(fd as _))
}

#[cfg(windows)]
unsafe fn get_file(handle: i64) -> ManuallyDrop<File> {
    use std::os::windows::io::FromRawHandle;
    ManuallyDrop::new(File::from_raw_handle(handle as *mut () as _))
}

#[cfg(not(any(windows, unix)))]
unsafe fn get_file(_: i64) -> ManuallyDrop<File> {
    panic!("File Descriptor Parsing is not implemented for this platform");
}

/// Allocate memory.
#[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    if size == 0 {
        std::ptr::NonNull::dangling().as_ptr()
    } else {
        unsafe { std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(size, 1)) }
    }
}

/// Deallocate memory.
#[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, cap: usize) {
    if cap != 0 {
        std::alloc::dealloc(ptr, std::alloc::Layout::from_size_align_unchecked(cap, 1));
    }
}

/// Returns the byte length of the last nul-terminated string returned on the
/// current thread. The length excludes the nul-terminator.
#[no_mangle]
pub extern "C" fn get_buf_len() -> usize {
    OUTPUT_VEC.with_borrow(|v| v.len() - 1)
}
