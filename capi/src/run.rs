use livesplit_core::{Attempt, Run, RunMetadata, Segment, TimeSpan};
use livesplit_core::run::{parser, saver};
use super::{acc, acc_mut, alloc, output_str, output_time_span, output_vec, own, own_drop, str};
use std::io::Cursor;
use std::{ptr, slice};
use libc::c_char;
use segment::OwnedSegment;

pub type OwnedRun = *mut Run;
pub type NullableOwnedRun = *mut Run;

#[no_mangle]
pub unsafe extern "C" fn Run_new() -> OwnedRun {
    alloc(Run::new())
}

#[no_mangle]
pub unsafe extern "C" fn Run_drop(this: OwnedRun) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn Run_parse(data: *const u8, length: usize) -> NullableOwnedRun {
    match parser::composite::parse(
        Cursor::new(slice::from_raw_parts(data, length)),
        None,
        false,
    ) {
        Ok(run) => alloc(run),
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn Run_clone(this: *const Run) -> OwnedRun {
    alloc(acc(&this).clone())
}

#[no_mangle]
pub unsafe extern "C" fn Run_push_segment(this: *mut Run, segment: OwnedSegment) {
    acc_mut(&this).push_segment(own(segment));
}

#[no_mangle]
pub unsafe extern "C" fn Run_game_name(this: *const Run) -> *const c_char {
    output_str(acc(&this).game_name())
}

#[no_mangle]
pub unsafe extern "C" fn Run_set_game_name(this: *mut Run, game: *const c_char) {
    acc_mut(&this).set_game_name(str(&game));
}

#[no_mangle]
pub unsafe extern "C" fn Run_game_icon(this: *const Run) -> *const c_char {
    output_str(acc(&this).game_icon().url())
}

#[no_mangle]
pub unsafe extern "C" fn Run_category_name(this: *const Run) -> *const c_char {
    output_str(acc(&this).category_name())
}

#[no_mangle]
pub unsafe extern "C" fn Run_set_category_name(this: *mut Run, category: *const c_char) {
    acc_mut(&this).set_category_name(str(&category));
}

#[no_mangle]
pub unsafe extern "C" fn Run_extended_file_name(
    this: *const Run,
    use_extended_category_name: bool,
) -> *const c_char {
    output_str(acc(&this).extended_file_name(use_extended_category_name))
}

#[no_mangle]
pub unsafe extern "C" fn Run_extended_name(
    this: *const Run,
    use_extended_category_name: bool,
) -> *const c_char {
    output_str(acc(&this).extended_name(use_extended_category_name))
}

#[no_mangle]
pub unsafe extern "C" fn Run_extended_category_name(
    this: *const Run,
    show_region: bool,
    show_platform: bool,
    show_variables: bool,
) -> *const c_char {
    output_str(acc(&this).extended_category_name(
        show_region,
        show_platform,
        show_variables,
    ))
}

#[no_mangle]
pub unsafe extern "C" fn Run_attempt_count(this: *const Run) -> u32 {
    acc(&this).attempt_count()
}

#[no_mangle]
pub unsafe extern "C" fn Run_metadata(this: *const Run) -> *const RunMetadata {
    acc(&this).metadata()
}

#[no_mangle]
pub unsafe extern "C" fn Run_offset(this: *const Run) -> *const TimeSpan {
    output_time_span(acc(&this).offset())
}

#[no_mangle]
pub unsafe extern "C" fn Run_len(this: *const Run) -> usize {
    acc(&this).len()
}

#[no_mangle]
pub unsafe extern "C" fn Run_segment(this: *const Run, index: usize) -> *const Segment {
    acc(&this).segment(index)
}

#[no_mangle]
pub unsafe extern "C" fn Run_attempt_history_len(this: *const Run) -> usize {
    acc(&this).attempt_history().len()
}

#[no_mangle]
pub unsafe extern "C" fn Run_attempt_history_index(
    this: *const Run,
    index: usize,
) -> *const Attempt {
    &acc(&this).attempt_history()[index]
}

#[no_mangle]
pub unsafe extern "C" fn Run_save_as_lss(this: *const Run) -> *const c_char {
    output_vec(|o| { saver::livesplit::save(acc(&this), o).unwrap(); })
}
