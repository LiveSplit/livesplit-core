#![allow(non_snake_case)]

extern crate livesplit_core;
extern crate libc;

use livesplit_core::{Segment, Run, Timer, parser, saver};
use livesplit_core::component::{timer, title, splits, previous_segment};
use libc::c_char;
use std::ffi::CStr;
use std::cell::RefCell;
use std::io::Cursor;
use std::ptr;
// use std::fmt::Write;

thread_local!{
    // static OUTPUT_STR: RefCell<String> = RefCell::new(String::new());
    static OUTPUT_VEC: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}

// fn output_str<F>(f: F) -> *const c_char
//     where F: FnOnce(&mut String)
// {
//     OUTPUT_STR.with(|output| {
//         let mut output = output.borrow_mut();
//         output.clear();
//         f(&mut output);
//         output.push('\0');
//         output.as_ptr() as *const c_char
//     })
// }

fn output_vec<F>(f: F) -> *const u8
    where F: FnOnce(&mut Vec<u8>)
{
    OUTPUT_VEC.with(|output| {
        let mut output = output.borrow_mut();
        output.clear();
        f(&mut output);
        output.push(0);
        output.as_ptr()
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

unsafe fn acc_mut<T>(data: *mut T) -> &'static mut T {
    &mut *data
}

unsafe fn acc<T>(data: *const T) -> &'static T {
    &*data
}

#[no_mangle]
pub unsafe extern "C" fn Segment_new(name: *const c_char) -> *mut Segment {
    alloc(Segment::new(str(name)))
}

#[no_mangle]
pub unsafe extern "C" fn SegmentList_new() -> *mut Vec<Segment> {
    alloc(Vec::new())
}

#[no_mangle]
pub unsafe extern "C" fn SegmentList_push(this: *mut Vec<Segment>, segment_drop: *mut Segment) {
    acc_mut(this).push(own(segment_drop));
}

#[no_mangle]
pub unsafe extern "C" fn Run_new(segments_drop: *mut Vec<Segment>) -> *mut Run {
    alloc(Run::new(own(segments_drop)))
}

#[no_mangle]
pub unsafe extern "C" fn Run_from_file(lss: *const c_char) -> *mut Run {
    match parser::lss::parse(Cursor::new(str(lss)), None) {
        Ok(run) => alloc(run),
        Err(e) => {
            println!("{:?}", e);
            match parser::wsplit::parse(Cursor::new(str(lss)), None) {
                Ok(run) => alloc(run),
                Err(e) => {
                    println!("{:?}", e);
                    ptr::null_mut()
                }
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Run_set_game(this: *mut Run, game: *const c_char) {
    acc_mut(this).set_game_name(str(game));
}

#[no_mangle]
pub unsafe extern "C" fn Run_set_category(this: *mut Run, category: *const c_char) {
    acc_mut(this).set_category_name(str(category));
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
    acc_mut(this).start();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_split(this: *mut Timer) {
    acc_mut(this).split();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_skip_split(this: *mut Timer) {
    acc_mut(this).skip_split();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_undo_split(this: *mut Timer) {
    acc_mut(this).undo_split();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_reset(this: *mut Timer, update_splits: bool) {
    acc_mut(this).reset(update_splits);
}

#[no_mangle]
pub unsafe extern "C" fn Timer_pause(this: *mut Timer) {
    acc_mut(this).pause();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_print_debug(this: *mut Timer) {
    println!("{:#?}", acc_mut(this));
}

#[no_mangle]
pub unsafe extern "C" fn Timer_save_run_as_lss(this: *const Timer) -> *const u8 {
    output_vec(|o| {
        saver::lss::save(acc(this).run(), o).unwrap();
    })
}

#[no_mangle]
pub unsafe extern "C" fn TimerComponent_new() -> *mut timer::Component {
    alloc(timer::Component::new())
}

#[no_mangle]
pub unsafe extern "C" fn TimerComponent_drop(this_drop: *mut timer::Component) {
    own(this_drop);
}

#[no_mangle]
pub unsafe extern "C" fn TimerComponent_state(this: *const timer::Component,
                                              timer: *const Timer)
                                              -> *const u8 {
    output_vec(|o| {
        acc(this).state(acc(timer)).write_json(o).unwrap();
    })
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponent_new() -> *mut title::Component {
    alloc(title::Component::new())
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponent_drop(this_drop: *mut title::Component) {
    own(this_drop);
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponent_state(this: *mut title::Component,
                                              timer: *const Timer)
                                              -> *const u8 {
    output_vec(|o| {
        acc_mut(this).state(acc(timer)).write_json(o).unwrap();
    })
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_new() -> *mut splits::Component {
    alloc(splits::Component::new())
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_drop(this_drop: *mut splits::Component) {
    own(this_drop);
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_state(this: *mut splits::Component,
                                               timer: *const Timer)
                                               -> *const u8 {
    output_vec(|o| {
        acc_mut(this).state(acc(timer)).write_json(o).unwrap();
    })
}

#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponent_new() -> *mut previous_segment::Component {
    alloc(previous_segment::Component::new())
}

#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponent_drop(this_drop: *mut previous_segment::Component) {
    own(this_drop);
}

#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponent_state(this: *const previous_segment::Component,
                                                        timer: *const Timer)
                                                        -> *const u8 {
    output_vec(|o| {
        acc(this).state(acc(timer)).write_json(o).unwrap();
    })
}
