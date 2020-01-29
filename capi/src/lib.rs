#![allow(non_snake_case, non_camel_case_types)]
#![warn(missing_docs)]

//! mod

use std::cell::{Cell, RefCell};
use std::ffi::CStr;
use std::fs::File;
use std::os::raw::c_char;
use std::ptr;

pub mod analysis;
pub mod atomic_date_time;
pub mod attempt;
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
pub mod key_value_component_state;
pub mod layout;
pub mod layout_editor;
pub mod layout_state;
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

use crate::run_metadata_custom_variable::RunMetadataCustomVariable;
use crate::run_metadata_speedrun_com_variable::RunMetadataSpeedrunComVariable;
use crate::segment_history_element::SegmentHistoryElement;
use livesplit_core::{Time, TimeSpan};

/// type
pub type Json = *const c_char;
/// type
pub type Nullablec_char = c_char;

thread_local! {
    static OUTPUT_VEC: RefCell<Vec<u8>> = RefCell::new(Vec::new());
    static TIME_SPAN: Cell<TimeSpan> = Cell::default();
    static TIME: Cell<Time> = Cell::default();
    static SEGMENT_HISTORY_ELEMENT: Cell<SegmentHistoryElement> = Cell::default();
    static RUN_METADATA_SPEEDRUN_COM_VARIABLE: Cell<RunMetadataSpeedrunComVariable> = Cell::new((ptr::null(), ptr::null()));
    static RUN_METADATA_CUSTOM_VARIABLE: Cell<RunMetadataCustomVariable> = Cell::new((ptr::null(), ptr::null()));
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

fn output_vec<F>(f: F) -> *const c_char
where
    F: FnOnce(&mut Vec<u8>),
{
    OUTPUT_VEC.with(|output| {
        let mut output = output.borrow_mut();
        output.clear();
        f(&mut output);
        output.push(0);
        output.as_ptr() as *const c_char
    })
}

unsafe fn str(s: *const c_char) -> &'static str {
    if s.is_null() {
        ""
    } else {
        CStr::from_ptr(s as _).to_str().unwrap()
    }
}

// raw file descriptor handling
#[cfg(unix)]
unsafe fn get_file(fd: i64) -> File {
    use std::os::unix::io::FromRawFd;
    File::from_raw_fd(fd as _)
}

#[cfg(windows)]
unsafe fn get_file(handle: i64) -> File {
    use std::os::windows::io::FromRawHandle;
    File::from_raw_handle(handle as *mut () as _)
}

#[cfg(not(any(windows, unix)))]
unsafe fn get_file(_: i64) -> File {
    panic!("File Descriptor Parsing is not implemented for this platform");
}

#[cfg(unix)]
unsafe fn release_file(file: File) {
    use std::os::unix::io::IntoRawFd;
    file.into_raw_fd();
}

#[cfg(windows)]
unsafe fn release_file(file: File) {
    use std::os::windows::io::IntoRawHandle;
    file.into_raw_handle();
}

#[cfg(not(any(windows, unix)))]
unsafe fn release_file(_: File) {}

/// Allocate memory.
#[cfg(all(
    target_arch = "wasm32",
    not(any(target_os = "wasi", feature = "wasm-web")),
))]
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}

/// Deallocate memory.
#[cfg(all(
    target_arch = "wasm32",
    not(any(target_os = "wasi", feature = "wasm-web")),
))]
#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, cap: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}
