#![allow(non_snake_case)]

extern crate livesplit_core;
extern crate libc;

use livesplit_core::{Segment, Run, Timer};
use libc::c_char;
use std::ffi::CStr;

unsafe fn get_str(s: *const c_char) -> &'static str {
    CStr::from_ptr(s).to_str().unwrap()
}

unsafe fn alloc<T>(data: T) -> *mut T {
    Box::into_raw(Box::new(data))
}

unsafe fn own<T>(data: *mut T) -> T {
    *Box::from_raw(data)
}

unsafe fn acc<T>(data: *mut T) -> &'static mut T {
    &mut *data
}

#[no_mangle]
pub unsafe extern "C" fn Segment_new(name: *const c_char) -> *mut Segment {
    alloc(Segment::new(get_str(name)))
}

#[no_mangle]
pub unsafe extern "C" fn SegmentList_new() -> *mut Vec<Segment> {
    alloc(Vec::new())
}

#[no_mangle]
pub unsafe extern "C" fn SegmentList_push(this: *mut Vec<Segment>, segment_drop: *mut Segment) {
    acc(this).push(own(segment_drop));
}

#[no_mangle]
pub unsafe extern "C" fn Run_new(segments_drop: *mut Vec<Segment>) -> *mut Run {
    alloc(Run::new(own(segments_drop)))
}

#[no_mangle]
pub unsafe extern "C" fn Timer_new(run_drop: *mut Run) -> *mut Timer {
    alloc(Timer::new(own(run_drop)))
}

#[no_mangle]
pub unsafe extern "C" fn Timer_drop(this_drop: *mut Timer) {
    own(this_drop);
}

#[no_mangle]
pub unsafe extern "C" fn Timer_start(this: *mut Timer) {
    acc(this).start();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_split(this: *mut Timer) {
    acc(this).split();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_skip_split(this: *mut Timer) {
    acc(this).skip_split();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_undo_split(this: *mut Timer) {
    acc(this).undo_split();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_reset(this: *mut Timer, update_splits: bool) {
    acc(this).reset(update_splits);
}

#[no_mangle]
pub unsafe extern "C" fn Timer_pause(this: *mut Timer) {
    acc(this).pause();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_print_debug(this: *mut Timer) {
    println!("{:#?}", acc(this));
}
