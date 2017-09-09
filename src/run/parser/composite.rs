use std::path::PathBuf;
use std::io::{self, BufRead, Seek, SeekFrom};
use std::result::Result as StdResult;
use Run;
use super::{face_split, livesplit, llanfair, llanfair_gered, shit_split, splitterz, splitty,
            time_split_tracker, urn, wsplit, TimerKind, llanfair2, portal2_live_timer};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Seek(err: io::Error) {
            from()
        }
        NoParserParsedIt
    }
}

pub type Result<T> = StdResult<T, Error>;

pub struct ParsedRun {
    pub run: Run,
    pub kind: TimerKind,
}

fn parsed(run: Run, kind: TimerKind) -> ParsedRun {
    ParsedRun { run, kind }
}

pub fn parse<R>(mut source: R, path: Option<PathBuf>, load_files: bool) -> Result<ParsedRun>
where
    R: BufRead + Seek,
{
    let files_path = if load_files { path.clone() } else { None };

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = livesplit::parse(&mut source, path) {
        return Ok(parsed(run, TimerKind::LiveSplit));
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = wsplit::parse(&mut source, load_files) {
        return Ok(parsed(run, TimerKind::WSplit));
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = splitterz::parse(&mut source, load_files) {
        return Ok(parsed(run, TimerKind::SplitterZ));
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = shit_split::parse(&mut source) {
        return Ok(parsed(run, TimerKind::ShitSplit));
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = splitty::parse(&mut source) {
        return Ok(parsed(run, TimerKind::Splitty));
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = time_split_tracker::parse(&mut source, files_path) {
        return Ok(parsed(run, TimerKind::TimeSplitTracker));
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = portal2_live_timer::parse(&mut source) {
        return Ok(parsed(run, TimerKind::Portal2LiveTimer));
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = face_split::parse(&mut source, load_files) {
        return Ok(parsed(run, TimerKind::FaceSplit));
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = llanfair::parse(&mut source) {
        return Ok(parsed(run, TimerKind::Llanfair));
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = llanfair_gered::parse(&mut source) {
        return Ok(parsed(run, TimerKind::LlanfairGered));
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = llanfair2::parse(&mut source) {
        return Ok(parsed(run, TimerKind::Llanfair2));
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = urn::parse(&mut source) {
        return Ok(parsed(run, TimerKind::Urn));
    }

    Err(Error::NoParserParsedIt)
}
