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
//! use std::fs::File;
//! use std::io::BufReader;
//! use std::path::PathBuf;
//!
//! // Load the file.
//! let path = PathBuf::from("path/to/splits_file");
//! let file = BufReader::new(File::open(&path).expect("File not found"));
//!
//! // We want to load additional files from the file system, like segment icons.
//! let load_files = true;
//!
//! // Actually parse the file.
//! let result = composite::parse(file, Some(path), load_files);
//! let parsed = result.expect("Not a valid splits file");
//!
//! // Print out the detected file format.
//! println!("Splits File Format: {}", parsed.kind);
//!
//! // Get out the Run object.
//! let run = parsed.run;
//! ```

use super::{
    face_split, flitter, livesplit, llanfair, llanfair2, llanfair_gered, portal2_live_timer,
    shit_split, source_live_timer, splits_io, splitterino, splitterz, splitty, time_split_tracker,
    urn, worstrun, wsplit, TimerKind,
};
use crate::Run;
use core::result::Result as StdResult;
use snafu::ResultExt;
use std::io::{self, BufRead, Seek, SeekFrom};
use std::path::PathBuf;

/// The Error type for splits files that couldn't be parsed by the Composite
/// Parser.
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// Failed to seek back when trying to parse with a different parser.
    SeekBack {
        /// The underlying error.
        source: io::Error,
    },
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

fn parsed(run: Run, kind: TimerKind) -> ParsedRun {
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
pub fn parse_and_fix<R>(source: R, path: Option<PathBuf>, load_files: bool) -> Result<ParsedRun>
where
    R: BufRead + Seek,
{
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
pub fn parse<R>(mut source: R, path: Option<PathBuf>, load_files: bool) -> Result<ParsedRun>
where
    R: BufRead + Seek,
{
    let files_path = if load_files { path.clone() } else { None };

    source.seek(SeekFrom::Start(0)).context(SeekBack)?;
    if let Ok(run) = livesplit::parse(&mut source, path) {
        return Ok(parsed(run, TimerKind::LiveSplit));
    }

    source.seek(SeekFrom::Start(0)).context(SeekBack)?;
    if let Ok(run) = wsplit::parse(&mut source, load_files) {
        return Ok(parsed(run, TimerKind::WSplit));
    }

    source.seek(SeekFrom::Start(0)).context(SeekBack)?;
    if let Ok(run) = splitterz::parse(&mut source, load_files) {
        return Ok(parsed(run, TimerKind::SplitterZ));
    }

    source.seek(SeekFrom::Start(0)).context(SeekBack)?;
    if let Ok(run) = shit_split::parse(&mut source) {
        return Ok(parsed(run, TimerKind::ShitSplit));
    }

    source.seek(SeekFrom::Start(0)).context(SeekBack)?;
    if let Ok(run) = splitty::parse(&mut source) {
        return Ok(parsed(run, TimerKind::Splitty));
    }

    source.seek(SeekFrom::Start(0)).context(SeekBack)?;
    if let Ok(run) = time_split_tracker::parse(&mut source, files_path) {
        return Ok(parsed(run, TimerKind::TimeSplitTracker));
    }

    source.seek(SeekFrom::Start(0)).context(SeekBack)?;
    if let Ok(run) = portal2_live_timer::parse(&mut source) {
        return Ok(parsed(run, TimerKind::Portal2LiveTimer));
    }

    source.seek(SeekFrom::Start(0)).context(SeekBack)?;
    if let Ok(run) = face_split::parse(&mut source, load_files) {
        return Ok(parsed(run, TimerKind::FaceSplit));
    }

    source.seek(SeekFrom::Start(0)).context(SeekBack)?;
    if let Ok(run) = llanfair::parse(&mut source) {
        return Ok(parsed(run, TimerKind::Llanfair));
    }

    // Should be parsed after LiveSplit's parser, as it also parses all
    // LiveSplit files with the current implementation.
    source.seek(SeekFrom::Start(0)).context(SeekBack)?;
    if let Ok(run) = llanfair_gered::parse(&mut source) {
        return Ok(parsed(run, TimerKind::LlanfairGered));
    }

    // Llanfair 2's format is almost entirely optional so it should be parsed
    // after all other XML based formats.
    source.seek(SeekFrom::Start(0)).context(SeekBack)?;
    if let Ok(run) = llanfair2::parse(&mut source) {
        return Ok(parsed(run, TimerKind::Llanfair2));
    }

    source.seek(SeekFrom::Start(0)).context(SeekBack)?;
    if let Ok((run, timer)) = splits_io::parse(&mut source) {
        return Ok(parsed(run, TimerKind::Generic(timer)));
    }

    // Splitterino, SourceLiveTimer and Flitter need to be before Urn because of
    // a false positive due to the nature of parsing json files.
    source.seek(SeekFrom::Start(0)).context(SeekBack)?;
    if let Ok(run) = splitterino::parse(&mut source) {
        return Ok(parsed(run, TimerKind::Splitterino));
    }

    source.seek(SeekFrom::Start(0)).context(SeekBack)?;
    if let Ok(run) = flitter::parse(&mut source) {
        return Ok(parsed(run, TimerKind::Flitter));
    }

    source.seek(SeekFrom::Start(0)).context(SeekBack)?;
    if let Ok(run) = source_live_timer::parse(&mut source) {
        return Ok(parsed(run, TimerKind::SourceLiveTimer));
    }

    // Both worstrun and Urn accept entirely empty JSON files. Therefore it's
    // very hard to determine which format we should be parsing those files as.
    // We poke at the format first to see if there's a game and category key in
    // there. If there is then we assume it's a worstrun file. This is somewhat
    // suboptimal as we parse worstrun files that don't have those keys (they
    // are optional) as Urn files.
    source.seek(SeekFrom::Start(0)).context(SeekBack)?;
    if worstrun::poke(&mut source) {
        source.seek(SeekFrom::Start(0)).context(SeekBack)?;
        if let Ok(run) = worstrun::parse(&mut source) {
            return Ok(parsed(run, TimerKind::Worstrun));
        }
    }

    source.seek(SeekFrom::Start(0)).context(SeekBack)?;
    if let Ok(run) = urn::parse(&mut source) {
        return Ok(parsed(run, TimerKind::Urn));
    }

    Err(Error::NoParserParsedIt)
}
