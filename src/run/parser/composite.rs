use std::path::PathBuf;
use std::io::{self, BufRead, Seek, SeekFrom};
use std::result::Result as StdResult;
use Run;
use super::{face_split, livesplit, llanfair, llanfair_gered, shit_split, splitterz, splitty,
            time_split_tracker, urn, wsplit, llanfair2, portal2_live_timer};

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

pub fn parse<R>(mut source: R, path: Option<PathBuf>, load_files: bool) -> Result<Run>
where
    R: BufRead + Seek,
{
    let files_path = if load_files { path.clone() } else { None };

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = livesplit::parse(&mut source, path) {
        return Ok(run);
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = wsplit::parse(&mut source, load_files) {
        return Ok(run);
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = splitterz::parse(&mut source, load_files) {
        return Ok(run);
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = shit_split::parse(&mut source) {
        return Ok(run);
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = splitty::parse(&mut source) {
        return Ok(run);
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = time_split_tracker::parse(&mut source, files_path) {
        return Ok(run);
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = portal2_live_timer::parse(&mut source) {
        return Ok(run);
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = face_split::parse(&mut source, load_files) {
        return Ok(run);
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = llanfair::parse(&mut source) {
        return Ok(run);
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = llanfair_gered::parse(&mut source) {
        return Ok(run);
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = llanfair2::parse(&mut source) {
        return Ok(run);
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = urn::parse(&mut source) {
        return Ok(run);
    }

    Err(Error::NoParserParsedIt)
}
