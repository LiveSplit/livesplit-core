use livesplit_core::{Attempt, Run, RunMetadata, Segment, TimeSpan};
use livesplit_core::run::{parser, saver};
use super::{acc, acc_mut, alloc, output_str, output_time_span, output_vec, own, own_drop, str};
use std::io::{BufReader, Cursor};
use std::slice;
use std::path::PathBuf;
use std::fs::File;
use libc::c_char;
use segment::OwnedSegment;
use parse_run_result::OwnedParseRunResult;

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
pub unsafe extern "C" fn Run_parse(
    data: *const u8,
    length: usize,
    path: *const c_char,
    load_files: bool,
) -> OwnedParseRunResult {
    let path = str(path);
    let path = if !path.is_empty() {
        Some(PathBuf::from(path))
    } else {
        None
    };

    alloc(parser::composite::parse(
        Cursor::new(slice::from_raw_parts(data, length)),
        path,
        load_files,
    ))
}

/// On Unix you pass a file descriptor to this function.
/// On Windows you pass a file handle to this function.
/// The file descriptor / handle does not get closed.
#[no_mangle]
pub unsafe extern "C" fn Run_parse_file_handle(
    handle: i64,
    path: *const c_char,
    load_files: bool,
) -> OwnedParseRunResult {
    let path = str(path);
    let path = if !path.is_empty() {
        Some(PathBuf::from(path))
    } else {
        None
    };

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

    let file = get_file(handle);

    let run = alloc(parser::composite::parse(
        BufReader::new(&file),
        path,
        load_files,
    ));

    #[cfg(unix)]
    fn release_file(file: File) {
        use std::os::unix::io::IntoRawFd;
        file.into_raw_fd();
    }

    #[cfg(windows)]
    fn release_file(file: File) {
        use std::os::windows::io::IntoRawHandle;
        file.into_raw_handle();
    }

    #[cfg(not(any(windows, unix)))]
    fn release_file(_: File) {}

    release_file(file);

    run
}

#[no_mangle]
pub unsafe extern "C" fn Run_clone(this: *const Run) -> OwnedRun {
    alloc(acc(this).clone())
}

#[no_mangle]
pub unsafe extern "C" fn Run_push_segment(this: *mut Run, segment: OwnedSegment) {
    acc_mut(this).push_segment(own(segment));
}

#[no_mangle]
pub unsafe extern "C" fn Run_game_name(this: *const Run) -> *const c_char {
    output_str(acc(this).game_name())
}

#[no_mangle]
pub unsafe extern "C" fn Run_set_game_name(this: *mut Run, game: *const c_char) {
    acc_mut(this).set_game_name(str(game));
}

#[no_mangle]
pub unsafe extern "C" fn Run_game_icon(this: *const Run) -> *const c_char {
    output_str(acc(this).game_icon().url())
}

#[no_mangle]
pub unsafe extern "C" fn Run_category_name(this: *const Run) -> *const c_char {
    output_str(acc(this).category_name())
}

#[no_mangle]
pub unsafe extern "C" fn Run_set_category_name(this: *mut Run, category: *const c_char) {
    acc_mut(this).set_category_name(str(category));
}

#[no_mangle]
pub unsafe extern "C" fn Run_extended_file_name(
    this: *const Run,
    use_extended_category_name: bool,
) -> *const c_char {
    output_str(acc(this).extended_file_name(use_extended_category_name))
}

#[no_mangle]
pub unsafe extern "C" fn Run_extended_name(
    this: *const Run,
    use_extended_category_name: bool,
) -> *const c_char {
    output_str(acc(this).extended_name(use_extended_category_name))
}

#[no_mangle]
pub unsafe extern "C" fn Run_extended_category_name(
    this: *const Run,
    show_region: bool,
    show_platform: bool,
    show_variables: bool,
) -> *const c_char {
    output_str(acc(this).extended_category_name(
        show_region,
        show_platform,
        show_variables,
    ))
}

#[no_mangle]
pub unsafe extern "C" fn Run_attempt_count(this: *const Run) -> u32 {
    acc(this).attempt_count()
}

#[no_mangle]
pub unsafe extern "C" fn Run_metadata(this: *const Run) -> *const RunMetadata {
    acc(this).metadata()
}

#[no_mangle]
pub unsafe extern "C" fn Run_stop_time(this: *const Run) -> *const TimeSpan {
    output_time_span(acc(this).stop_time())
}

#[no_mangle]
pub unsafe extern "C" fn Run_offset(this: *const Run) -> *const TimeSpan {
    output_time_span(acc(this).offset())
}

#[no_mangle]
pub unsafe extern "C" fn Run_len(this: *const Run) -> usize {
    acc(this).len()
}

#[no_mangle]
pub unsafe extern "C" fn Run_segment(this: *const Run, index: usize) -> *const Segment {
    acc(this).segment(index)
}

#[no_mangle]
pub unsafe extern "C" fn Run_attempt_history_len(this: *const Run) -> usize {
    acc(this).attempt_history().len()
}

#[no_mangle]
pub unsafe extern "C" fn Run_attempt_history_index(
    this: *const Run,
    index: usize,
) -> *const Attempt {
    &acc(this).attempt_history()[index]
}

#[no_mangle]
pub unsafe extern "C" fn Run_save_as_lss(this: *const Run) -> *const c_char {
    output_vec(|o| {
        saver::livesplit::save(acc(this), o).unwrap();
    })
}

#[no_mangle]
pub unsafe extern "C" fn Run_custom_comparisons_len(this: *const Run) -> usize {
    acc(this).custom_comparisons().len()
}

#[no_mangle]
pub unsafe extern "C" fn Run_custom_comparison(this: *const Run, index: usize) -> *const c_char {
    output_str(&acc(this).custom_comparisons()[index])
}

#[no_mangle]
pub unsafe extern "C" fn Run_auto_splitter_settings(this: *const Run) -> *const c_char {
    output_vec(|o| o.extend_from_slice(acc(this).auto_splitter_settings()))
}
