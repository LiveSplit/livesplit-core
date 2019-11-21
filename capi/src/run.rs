//! A Run stores the split times for a specific game and category of a runner.

use super::{get_file, output_str, output_time_span, output_vec, release_file, str};
use crate::parse_run_result::OwnedParseRunResult;
use crate::segment::OwnedSegment;
use livesplit_core::run::{parser, saver};
use livesplit_core::{Attempt, Run, RunMetadata, Segment, TimeSpan};
use std::io::{BufReader, Cursor};
use std::os::raw::c_char;
use std::path::PathBuf;
use std::slice;

/// type
pub type OwnedRun = Box<Run>;
/// type
pub type NullableOwnedRun = Option<OwnedRun>;

/// Creates a new Run object with no segments.
#[no_mangle]
pub extern "C" fn Run_new() -> OwnedRun {
    Box::new(Run::new())
}

/// drop
#[no_mangle]
pub extern "C" fn Run_drop(this: OwnedRun) {
    drop(this);
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

    Box::new(parser::composite::parse(
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

    let file = get_file(handle);

    let run = Box::new(parser::composite::parse(
        BufReader::new(&file),
        path,
        load_files,
    ));

    release_file(file);

    run
}

/// Clones the Run object.
#[no_mangle]
pub extern "C" fn Run_clone(this: &Run) -> OwnedRun {
    Box::new(this.clone())
}

/// Pushes the segment provided to the end of the list of segments of this Run.
#[no_mangle]
pub extern "C" fn Run_push_segment(this: &mut Run, segment: OwnedSegment) {
    this.push_segment(*segment);
}

/// Accesses the name of the game this Run is for.
#[no_mangle]
pub extern "C" fn Run_game_name(this: &Run) -> *const c_char {
    output_str(this.game_name())
}

/// Sets the name of the game this Run is for.
#[no_mangle]
pub unsafe extern "C" fn Run_set_game_name(this: &mut Run, game: *const c_char) {
    this.set_game_name(str(game));
}

/// Accesses the game icon's data. If there is no game icon, this returns an
/// empty buffer.
#[no_mangle]
pub extern "C" fn Run_game_icon_ptr(this: &Run) -> *const u8 {
    this.game_icon().data().as_ptr()
}

/// Accesses the amount of bytes the game icon's data takes up.
#[no_mangle]
pub extern "C" fn Run_game_icon_len(this: &Run) -> usize {
    this.game_icon().data().len()
}

/// Accesses the name of the category this Run is for.
#[no_mangle]
pub extern "C" fn Run_category_name(this: &Run) -> *const c_char {
    output_str(this.category_name())
}

/// Sets the name of the category this Run is for.
#[no_mangle]
pub unsafe extern "C" fn Run_set_category_name(this: &mut Run, category: *const c_char) {
    this.set_category_name(str(category));
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
pub extern "C" fn Run_extended_file_name(
    this: &Run,
    use_extended_category_name: bool,
) -> *const c_char {
    output_str(this.extended_file_name(use_extended_category_name))
}

/// Returns a name suitable for this Run that is built the following way:
///
/// Game Name - Category Name
///
/// If either is empty, the dash is omitted. If an extended category name is
/// used, the variables of the category are appended in a parenthesis.
#[no_mangle]
pub extern "C" fn Run_extended_name(this: &Run, use_extended_category_name: bool) -> *const c_char {
    output_str(this.extended_name(use_extended_category_name))
}

/// Returns an extended category name that possibly includes the region,
/// platform and variables, depending on the arguments provided. An extended
/// category name may look like this:
///
/// Any% (No Tuner, JPN, Wii Emulator)
#[no_mangle]
pub extern "C" fn Run_extended_category_name(
    this: &Run,
    show_region: bool,
    show_platform: bool,
    show_variables: bool,
) -> *const c_char {
    output_str(this.extended_category_name(show_region, show_platform, show_variables))
}

/// Returns the amount of runs that have been attempted with these splits.
#[no_mangle]
pub extern "C" fn Run_attempt_count(this: &Run) -> u32 {
    this.attempt_count()
}

/// Accesses additional metadata of this Run, like the platform and region
/// of the game.
#[no_mangle]
pub extern "C" fn Run_metadata(this: &Run) -> &RunMetadata {
    this.metadata()
}

/// Accesses the time an attempt of this Run should start at.
#[no_mangle]
pub extern "C" fn Run_offset(this: &Run) -> *const TimeSpan {
    output_time_span(this.offset())
}

/// Returns the amount of segments stored in this Run.
#[no_mangle]
pub extern "C" fn Run_len(this: &Run) -> usize {
    this.len()
}

/// Marks the Run as modified, so that it is known that there are changes
/// that should be saved.
#[no_mangle]
pub extern "C" fn Run_mark_as_modified(this: &mut Run) {
    this.mark_as_modified();
}

/// Returns whether the Run has been modified and should be saved so that the
/// changes don't get lost.
#[no_mangle]
pub extern "C" fn Run_has_been_modified(this: &Run) -> bool {
    this.has_been_modified()
}

/// Accesses a certain segment of this Run. You may not provide an out of bounds
/// index.
#[no_mangle]
pub extern "C" fn Run_segment(this: &Run, index: usize) -> &Segment {
    this.segment(index)
}

/// Returns the amount attempt history elements are stored in this Run.
#[no_mangle]
pub extern "C" fn Run_attempt_history_len(this: &Run) -> usize {
    this.attempt_history().len()
}

/// Accesses the an attempt history element by its index. This does not store
/// the actual segment times, just the overall attempt information. Information
/// about the individual segments is stored within each segment. You may not
/// provide an out of bounds index.
#[no_mangle]
pub extern "C" fn Run_attempt_history_index(this: &Run, index: usize) -> &Attempt {
    &this.attempt_history()[index]
}

/// Saves a Run as a LiveSplit splits file (*.lss). If the run is actively in
/// use by a timer, use the appropriate method on the timer instead, in order to
/// properly save the current attempt as well.
#[no_mangle]
pub extern "C" fn Run_save_as_lss(this: &Run) -> *const c_char {
    output_vec(|o| {
        saver::livesplit::save_run(this, o).unwrap();
    })
}

/// Returns the amount of custom comparisons stored in this Run.
#[no_mangle]
pub extern "C" fn Run_custom_comparisons_len(this: &Run) -> usize {
    this.custom_comparisons().len()
}

/// Accesses a custom comparison stored in this Run by its index. This includes
/// `Personal Best` but excludes all the other Comparison Generators. You may
/// not provide an out of bounds index.
#[no_mangle]
pub extern "C" fn Run_custom_comparison(this: &Run, index: usize) -> *const c_char {
    output_str(&this.custom_comparisons()[index])
}

/// Accesses the Auto Splitter Settings that are encoded as XML.
#[no_mangle]
pub extern "C" fn Run_auto_splitter_settings(this: &Run) -> *const c_char {
    output_vec(|o| o.extend_from_slice(this.auto_splitter_settings()))
}
