#![allow(non_snake_case)]

extern crate livesplit_core;
extern crate libc;

use libc::c_char;
use std::ffi::CStr;
use std::cell::{Cell, RefCell};
use std::mem;

pub mod timer;
pub mod hotkey_system;
pub mod run;
pub mod segment_list;
pub mod segment;
pub mod time;
pub mod time_span;
pub mod attempt;
pub mod run_metadata;
pub mod segment_history;
pub mod segment_history_iter;
pub mod segment_history_element;
pub mod title_component;
pub mod title_component_state;
pub mod splits_component;
pub mod splits_component_state;
pub mod timer_component;
pub mod timer_component_state;
pub mod previous_segment_component;
pub mod previous_segment_component_state;
pub mod sum_of_best_component;
pub mod sum_of_best_component_state;
pub mod possible_time_save_component;
pub mod possible_time_save_component_state;
pub mod run_editor;
pub mod shared_timer;
pub mod timer_read_lock;
pub mod timer_write_lock;

use segment_history_element::SegmentHistoryElement;
use livesplit_core::{Time, TimeSpan};

thread_local! {
    static OUTPUT_STR: RefCell<String> = RefCell::new(String::new());
    static OUTPUT_VEC: RefCell<Vec<u8>> = RefCell::new(Vec::new());
    static TIME_SPAN: Cell<TimeSpan> = Cell::default();
    static TIME: Cell<Time> = Cell::default();
    static SEGMENT_HISTORY_ELEMENT: Cell<SegmentHistoryElement> = Cell::new(unsafe { mem::uninitialized() });
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
    output_str_with(|o| { o.push_str(s.as_ref()); })
}

fn output_str_with<F>(f: F) -> *const c_char
    where F: FnOnce(&mut String)
{
    OUTPUT_STR.with(|output| {
        let mut output = output.borrow_mut();
        output.clear();
        f(&mut output);
        output.push('\0');
        output.as_ptr() as *const c_char
    })
}

fn output_vec<F>(f: F) -> *const c_char
    where F: FnOnce(&mut Vec<u8>)
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
    CStr::from_ptr(s).to_str().unwrap()
}

unsafe fn alloc<T>(data: T) -> *mut T {
    Box::into_raw(Box::new(data))
}

unsafe fn own<T>(data: *mut T) -> T {
    *Box::from_raw(data)
}

unsafe fn own_drop<T>(data: *mut T) {
    Box::from_raw(data);
}

unsafe fn acc_mut<T>(data: *mut T) -> &'static mut T {
    &mut *data
}

unsafe fn acc<T>(data: *const T) -> &'static T {
    &*data
}
