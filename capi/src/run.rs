//! A Run stores the split times for a specific game and category of a runner.

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

/// type
pub type OwnedRun = *mut Run;
/// type
pub type NullableOwnedRun = *mut Run;

/// Creates a new Run object with no segments.
#[no_mangle]
pub unsafe extern "C" fn Run_new() -> OwnedRun {
    alloc(Run::new())
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn Run_drop(this: OwnedRun) {
    own_drop(this);
}

/// Attempts to parse a splits file from an array by invoking the corresponding
/// parser for the file format detected. A path to the splits file can be
/// provided, which helps saving the splits file again later. Additionally you
/// need to specify if additional files, like external images are allowed to be
/// loaded. If you are using livesplit-core in a server-like environment, set
/// this to <FALSE>. Only client-side applications should set this to <TRUE>.
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

/// Attempts to parse a splits file from a file by invoking the corresponding
/// parser for the file format detected. A path to the splits file can be
/// provided, which helps saving the splits file again later. Additionally you
/// need to specify if additional files, like external images are allowed to be
/// loaded. If you are using livesplit-core in a server-like environment, set
/// this to <FALSE>. Only client-side applications should set this to <TRUE>. On
/// Unix you pass a file descriptor to this function. On Windows you pass a file
/// handle to this function. The file descriptor / handle does not get closed.
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

/// Clones the Run object.
#[no_mangle]
pub unsafe extern "C" fn Run_clone(this: *const Run) -> OwnedRun {
    alloc(acc(this).clone())
}

/// Pushes the segment provided to the end of the list of segments of this Run.
#[no_mangle]
pub unsafe extern "C" fn Run_push_segment(this: *mut Run, segment: OwnedSegment) {
    acc_mut(this).push_segment(own(segment));
}

/// Accesses the name of the game this Run is for.
#[no_mangle]
pub unsafe extern "C" fn Run_game_name(this: *const Run) -> *const c_char {
    output_str(acc(this).game_name())
}

/// Sets the name of the game this Run is for.
#[no_mangle]
pub unsafe extern "C" fn Run_set_game_name(this: *mut Run, game: *const c_char) {
    acc_mut(this).set_game_name(str(game));
}

/// Accesses the Data URL storing the game icon's data. If there is no game
/// icon, this returns an empty string instead of a URL.
#[no_mangle]
pub unsafe extern "C" fn Run_game_icon(this: *const Run) -> *const c_char {
    output_str(acc(this).game_icon().url())
}

/// Accesses the name of the category this Run is for.
#[no_mangle]
pub unsafe extern "C" fn Run_category_name(this: *const Run) -> *const c_char {
    output_str(acc(this).category_name())
}

/// Sets the name of the category this Run is for.
#[no_mangle]
pub unsafe extern "C" fn Run_set_category_name(this: *mut Run, category: *const c_char) {
    acc_mut(this).set_category_name(str(category));
}

/// Returns a file name (without the extension) suitable for this Run that
/// is built the following way:
///
/// Game Name - Category Name
///
/// If either is empty, the dash is omitted. Special characters that cause
/// problems in file names are also omitted. If an extended category name is
/// used, the variables of the category are appended in a parenthesis.
#[no_mangle]
pub unsafe extern "C" fn Run_extended_file_name(
    this: *const Run,
    use_extended_category_name: bool,
) -> *const c_char {
    output_str(acc(this).extended_file_name(use_extended_category_name))
}

/// Returns a name suitable for this Run that is built the following way:
///
/// Game Name - Category Name
///
/// If either is empty, the dash is omitted. If an extended category name is
/// used, the variables of the category are appended in a parenthesis.
#[no_mangle]
pub unsafe extern "C" fn Run_extended_name(
    this: *const Run,
    use_extended_category_name: bool,
) -> *const c_char {
    output_str(acc(this).extended_name(use_extended_category_name))
}

/// Returns an extended category name that possibly includes the region,
/// platform and variables, depending on the arguments provided. An extended
/// category name may look like this:
///
/// Any% (No Tuner, JPN, Wii Emulator)
#[no_mangle]
pub unsafe extern "C" fn Run_extended_category_name(
    this: *const Run,
    show_region: bool,
    show_platform: bool,
    show_variables: bool,
) -> *const c_char {
    output_str(acc(this).extended_category_name(show_region, show_platform, show_variables))
}

/// Returns the amount of runs that have been attempted with these splits.
#[no_mangle]
pub unsafe extern "C" fn Run_attempt_count(this: *const Run) -> u32 {
    acc(this).attempt_count()
}

/// Accesses additional metadata of this Run, like the platform and region
/// of the game.
#[no_mangle]
pub unsafe extern "C" fn Run_metadata(this: *const Run) -> *const RunMetadata {
    acc(this).metadata()
}

/// Accesses the time an attempt of this Run should start at.
#[no_mangle]
pub unsafe extern "C" fn Run_offset(this: *const Run) -> *const TimeSpan {
    output_time_span(acc(this).offset())
}

/// Returns the amount of segments stored in this Run.
#[no_mangle]
pub unsafe extern "C" fn Run_len(this: *const Run) -> usize {
    acc(this).len()
}

/// Accesses a certain segment of this Run. You may not provide an out of bounds
/// index.
#[no_mangle]
pub unsafe extern "C" fn Run_segment(this: *const Run, index: usize) -> *const Segment {
    acc(this).segment(index)
}

/// Returns the amount attempt history elements are stored in this Run.
#[no_mangle]
pub unsafe extern "C" fn Run_attempt_history_len(this: *const Run) -> usize {
    acc(this).attempt_history().len()
}

/// Accesses the an attempt history element by its index. This does not store
/// the actual segment times, just the overall attempt information. Information
/// about the individual segments is stored within each segment. You may not
/// provide an out of bounds index.
#[no_mangle]
pub unsafe extern "C" fn Run_attempt_history_index(
    this: *const Run,
    index: usize,
) -> *const Attempt {
    &acc(this).attempt_history()[index]
}

/// Saves the Run as a LiveSplit splits file (*.lss).
#[no_mangle]
pub unsafe extern "C" fn Run_save_as_lss(this: *const Run) -> *const c_char {
    output_vec(|o| {
        saver::livesplit::save(acc(this), o).unwrap();
    })
}

/// Returns the amount of custom comparisons stored in this Run.
#[no_mangle]
pub unsafe extern "C" fn Run_custom_comparisons_len(this: *const Run) -> usize {
    acc(this).custom_comparisons().len()
}

/// Accesses a custom comparison stored in this Run by its index. This includes
/// `Personal Best` but excludes all the other Comparison Generators. You may
/// not provide an out of bounds index.
#[no_mangle]
pub unsafe extern "C" fn Run_custom_comparison(this: *const Run, index: usize) -> *const c_char {
    output_str(&acc(this).custom_comparisons()[index])
}

/// Accesses the Auto Splitter Settings that are encoded as XML.
#[no_mangle]
pub unsafe extern "C" fn Run_auto_splitter_settings(this: *const Run) -> *const c_char {
    output_vec(|o| o.extend_from_slice(acc(this).auto_splitter_settings()))
}
