//! The composite parser can be used, if the file type of the splits file is not
//! known, which tries to figure out which splits file format is used and parses
//! it with the parser for that format.
//!
//! # Examples
//!
//! Using the composite parser to parse a splits file of an unknown file format.
//!
//! ```no_run
//! use livesplit_core::run::parser::composite;
//! use std::fs;
//! use std::path::PathBuf;
//!
//! // Load the file.
//! let path = PathBuf::from("path/to/splits_file");
//! let file = fs::read(&path).expect("Failed reading the file.");
//!
//! // We want to load additional files from the file system, like segment icons.
//! let load_files = true;
//!
//! // Actually parse the file.
//! let result = composite::parse(&file, Some(path), load_files);
//! let parsed = result.expect("Not a valid splits file.");
//!
//! // Print out the detected file format.
//! println!("Splits File Format: {}", parsed.kind);
//!
//! // Get out the Run object.
//! let run = parsed.run;
//! ```

use super::{
    face_split, flitter, livesplit, llanfair, llanfair_gered, portal2_live_timer, shit_split,
    source_live_timer, splits_io, splitterino, splitterz, splitty, time_split_tracker, urn, wsplit,
    TimerKind,
};
use crate::{platform::path::PathBuf, Run};
use core::{result::Result as StdResult, str};

/// The Error type for splits files that couldn't be parsed by the Composite
/// Parser.
#[derive(Debug, snafu::Snafu)]
#[snafu(context(suffix(false)))]
pub enum Error {
    /// No parser was able to parse the splits file.
    NoParserParsedIt,
}

/// The Result type for the Composite Parser.
pub type Result<T> = StdResult<T, Error>;

/// A run parsed by the Composite Parser. This contains the Run itself and
/// information about which parser parsed it.
pub struct ParsedRun {
    /// The parsed run.
    pub run: Run,
    /// The parser that parsed it.
    pub kind: TimerKind,
}

const fn parsed(run: Run, kind: TimerKind) -> ParsedRun {
    ParsedRun { run, kind }
}

/// Attempts to parse and fix a splits file by invoking the corresponding parser
/// for the file format detected. A path to the splits file can be provided,
/// which helps saving the splits file again later. Additionally you need to
/// specify if additional files, like external images are allowed to be loaded.
/// If you are using livesplit-core in a server-like environment, set this to
/// `false`. Only client-side applications should set this to `true`. Unlike the
/// normal parsing function, it also fixes problems in the Run, such as
/// decreasing times and missing information.
pub fn parse_and_fix<R>(
    source: &[u8],
    path: Option<PathBuf>,
    load_files: bool,
) -> Result<ParsedRun> {
    let mut run = parse(source, path, load_files)?;
    run.run.fix_splits();
    Ok(run)
}

/// Attempts to parse a splits file by invoking the corresponding parser for the
/// file format detected. A path to the splits file can be provided, which helps
/// saving the splits file again later. Additionally you need to specify if
/// additional files, like external images are allowed to be loaded. If you are
/// using livesplit-core in a server-like environment, set this to `false`. Only
/// client-side applications should set this to `true`.
pub fn parse(source: &[u8], path: Option<PathBuf>, load_files: bool) -> Result<ParsedRun> {
    if let Ok(source) = simdutf8::basic::from_utf8(source) {
        let files_path = if load_files { path.clone() } else { None };

        if let Ok(run) = livesplit::parse(source, path) {
            return Ok(parsed(run, TimerKind::LiveSplit));
        }

        if let Ok(run) = wsplit::parse(source, load_files) {
            return Ok(parsed(run, TimerKind::WSplit));
        }

        if let Ok(run) = splitterz::parse(source, load_files) {
            return Ok(parsed(run, TimerKind::SplitterZ));
        }

        if let Ok(run) = shit_split::parse(source) {
            return Ok(parsed(run, TimerKind::ShitSplit));
        }

        if let Ok(run) = splitty::parse(source) {
            return Ok(parsed(run, TimerKind::Splitty));
        }

        if let Ok(run) = time_split_tracker::parse(source, files_path) {
            return Ok(parsed(run, TimerKind::TimeSplitTracker));
        }

        if let Ok(run) = portal2_live_timer::parse(source) {
            return Ok(parsed(run, TimerKind::Portal2LiveTimer));
        }

        if let Ok(run) = face_split::parse(source, load_files) {
            return Ok(parsed(run, TimerKind::FaceSplit));
        }

        // Should be parsed after LiveSplit's parser, as it also parses all
        // LiveSplit files with the current implementation.
        if let Ok(run) = llanfair_gered::parse(source) {
            return Ok(parsed(run, TimerKind::LlanfairGered));
        }

        if let Ok((run, timer)) = splits_io::parse(source) {
            return Ok(parsed(run, TimerKind::Generic(timer)));
        }

        // Splitterino, SourceLiveTimer and Flitter need to be before Urn because of
        // a false positive due to the nature of parsing json files.
        if let Ok(run) = splitterino::parse(source) {
            return Ok(parsed(run, TimerKind::Splitterino));
        }

        if let Ok(run) = flitter::parse(source) {
            return Ok(parsed(run, TimerKind::Flitter));
        }

        if let Ok(run) = source_live_timer::parse(source) {
            return Ok(parsed(run, TimerKind::SourceLiveTimer));
        }

        // Urn accepts entirely empty JSON files.
        if let Ok(run) = urn::parse(source) {
            return Ok(parsed(run, TimerKind::Urn));
        }
    }

    if let Ok(run) = llanfair::parse(source) {
        return Ok(parsed(run, TimerKind::Llanfair));
    }

    Err(Error::NoParserParsedIt)
}
