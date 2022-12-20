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
//! use std::path::Path;
//!
//! // Load the file.
//! let path = Path::new("path/to/splits_file");
//! let file = fs::read(path).expect("Failed reading the file.");
//!
//! // Actually parse the file. We also pass the path to load additional files from
//! // the file system, like segment icons.
//! let result = composite::parse(&file, Some(path));
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
    source_live_timer, speedrun_igt, splits_io, splitterino, splitterz, splitty,
    time_split_tracker, urn, wsplit, TimerKind,
};
use crate::{platform::path::Path, Run};
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
pub struct ParsedRun<'a> {
    /// The parsed run.
    pub run: Run,
    /// The parser that parsed it.
    pub kind: TimerKind<'a>,
}

impl ParsedRun<'_> {
    /// Returns an owned version of the parsed run.
    pub fn into_owned(self) -> ParsedRun<'static> {
        ParsedRun {
            run: self.run,
            kind: self.kind.into_owned(),
        }
    }
}

const fn parsed(run: Run, kind: TimerKind<'_>) -> ParsedRun<'_> {
    ParsedRun { run, kind }
}

/// Attempts to parse and fix a splits file by invoking the corresponding parser
/// for the file format detected. Additionally you can provide the path of the
/// splits file so additional files, like external images, can be loaded. If you
/// are using livesplit-core in a server-like environment, set this to `None`.
/// Only client-side applications should provide a path here. Unlike the normal
/// parsing function, it also fixes problems in the Run, such as decreasing
/// times and missing information.
pub fn parse_and_fix<'source>(
    source: &'source [u8],
    load_files_path: Option<&Path>,
) -> Result<ParsedRun<'source>> {
    let mut run = parse(source, load_files_path)?;
    run.run.fix_splits();
    Ok(run)
}

/// Attempts to parse a splits file by invoking the corresponding parser for the
/// file format detected. Additionally you can provide the path of the splits
/// file so additional files, like external images, can be loaded. If you are
/// using livesplit-core in a server-like environment, set this to `None`. Only
/// client-side applications should provide a path here.
pub fn parse<'source>(
    source: &'source [u8],
    load_files_path: Option<&Path>,
) -> Result<ParsedRun<'source>> {
    if let Ok(source) = simdutf8::basic::from_utf8(source) {
        if let Ok(run) = livesplit::parse(source) {
            return Ok(parsed(run, TimerKind::LiveSplit));
        }

        if let Ok(run) = wsplit::parse(source, load_files_path.is_some()) {
            return Ok(parsed(run, TimerKind::WSplit));
        }

        if let Ok(run) = splitterz::parse(source, load_files_path.is_some()) {
            return Ok(parsed(run, TimerKind::SplitterZ));
        }

        if let Ok(run) = shit_split::parse(source) {
            return Ok(parsed(run, TimerKind::ShitSplit));
        }

        if let Ok(run) = splitty::parse(source) {
            return Ok(parsed(run, TimerKind::Splitty));
        }

        if let Ok(run) = time_split_tracker::parse(source, load_files_path) {
            return Ok(parsed(run, TimerKind::TimeSplitTracker));
        }

        if let Ok(run) = portal2_live_timer::parse(source) {
            return Ok(parsed(run, TimerKind::Portal2LiveTimer));
        }

        if let Ok(run) = face_split::parse(source, load_files_path.is_some()) {
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

        // Splitterino, SourceLiveTimer, Flitter, and SpeedRunIGT need to be
        // before Urn because of a false positive due to the nature of parsing
        // JSON files.
        if let Ok(run) = splitterino::parse(source) {
            return Ok(parsed(run, TimerKind::Splitterino));
        }

        if let Ok(run) = flitter::parse(source) {
            return Ok(parsed(run, TimerKind::Flitter));
        }

        if let Ok(run) = source_live_timer::parse(source) {
            return Ok(parsed(run, TimerKind::SourceLiveTimer));
        }

        if let Ok(run) = speedrun_igt::parse(source) {
            return Ok(parsed(run, TimerKind::SpeedRunIGT));
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
