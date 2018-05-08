#![allow(non_snake_case, non_camel_case_types)]
#![warn(missing_docs)]

//! mod

extern crate livesplit_core;

use std::cell::{Cell, RefCell};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

pub mod atomic_date_time;
pub mod attempt;
pub mod blank_space_component;
pub mod blank_space_component_state;
pub mod component;
pub mod current_comparison_component;
pub mod current_comparison_component_state;
pub mod current_pace_component;
pub mod current_pace_component_state;
pub mod delta_component;
pub mod delta_component_state;
pub mod detailed_timer_component;
pub mod detailed_timer_component_state;
pub mod general_layout_settings;
pub mod graph_component;
pub mod graph_component_state;
pub mod hotkey_system;
pub mod layout;
pub mod layout_editor;
pub mod parse_run_result;
pub mod possible_time_save_component;
pub mod possible_time_save_component_state;
pub mod potential_clean_up;
pub mod previous_segment_component;
pub mod previous_segment_component_state;
pub mod run;
pub mod run_editor;
pub mod run_metadata;
pub mod run_metadata_variable;
pub mod run_metadata_variables_iter;
pub mod segment;
pub mod segment_history;
pub mod segment_history_element;
pub mod segment_history_iter;
pub mod separator_component;
pub mod setting_value;
pub mod shared_timer;
pub mod splits_component;
pub mod splits_component_state;
pub mod sum_of_best_cleaner;
pub mod sum_of_best_component;
pub mod sum_of_best_component_state;
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
pub mod total_playtime_component_state;

use livesplit_core::{Time, TimeSpan};
use run_metadata_variable::RunMetadataVariable;
use segment_history_element::SegmentHistoryElement;

/// type
pub type Json = *const c_char;
/// type
pub type Nullablec_char = c_char;

thread_local! {
    static OUTPUT_VEC: RefCell<Vec<u8>> = RefCell::new(Vec::new());
    static TIME_SPAN: Cell<TimeSpan> = Cell::default();
    static TIME: Cell<Time> = Cell::default();
    static SEGMENT_HISTORY_ELEMENT: Cell<SegmentHistoryElement> = Cell::default();
    static RUN_METADATA_VARIABLE: Cell<RunMetadataVariable> = Cell::new((ptr::null(), ptr::null()));
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
