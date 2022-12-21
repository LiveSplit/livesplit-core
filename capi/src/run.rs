//! A Run stores the split times for a specific game and category of a runner.

use super::{get_file, output_str, output_time_span, output_vec, str};
use crate::{parse_run_result::OwnedParseRunResult, segment::OwnedSegment, with_vec};
use livesplit_core::{
    run::{
        parser,
        saver::{self, livesplit::IoWrite},
    },
    Attempt, Run, RunMetadata, Segment, TimeSpan,
};
use std::{
    io::{Read, Write},
    os::raw::c_char,
    path::Path,
    slice,
};

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
/// parser for the file format detected. Additionally you can provide the path
/// of the splits file so additional files, like external images, can be loaded.
/// If you are using livesplit-core in a server-like environment, set this to
/// <NULL>. Only client-side applications should provide a path here. Unlike the
/// normal parsing function, it also fixes problems in the Run, such as
/// decreasing times and missing information.
#[no_mangle]
pub unsafe extern "C" fn Run_parse(
    data: *const u8,
    length: usize,
    load_files_path: *const c_char,
) -> OwnedParseRunResult {
    let load_files_path = str(load_files_path);
    let load_files_path = if !load_files_path.is_empty() {
        Some(Path::new(load_files_path))
    } else {
        None
    };

    Box::new(parser::composite::parse(slice::from_raw_parts(data, length), load_files_path).ok())
}

/// Attempts to parse a splits file from a file by invoking the corresponding
/// parser for the file format detected. Additionally you can provide the path
/// of the splits file so additional files, like external images, can be loaded.
/// If you are using livesplit-core in a server-like environment, set this to
/// <NULL>. Only client-side applications should provide a path here. Unlike the
/// normal parsing function, it also fixes problems in the Run, such as
/// decreasing times and missing information. On Unix you pass a file descriptor
/// to this function. On Windows you pass a file handle to this function. The
/// file descriptor / handle does not get closed.
#[no_mangle]
pub unsafe extern "C" fn Run_parse_file_handle(
    handle: i64,
    load_files_path: *const c_char,
) -> OwnedParseRunResult {
    let load_files_path = str(load_files_path);
    let load_files_path = if !load_files_path.is_empty() {
        Some(Path::new(load_files_path))
    } else {
        None
    };

    let mut file = get_file(handle);

    with_vec(|buf| {
        Box::new(
            file.read_to_end(buf)
                .ok()
                .and_then(|_| parser::composite::parse(buf, load_files_path).ok())
                .map(|p| p.into_owned()),
        )
    })
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
    output_vec(|o| {
        let _ = write!(
            o,
            "{}",
            this.extended_category_name(show_region, show_platform, show_variables),
        );
    })
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
        saver::livesplit::save_run(this, IoWrite(o)).unwrap();
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
    output_str(this.auto_splitter_settings())
}
