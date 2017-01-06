use std::path::PathBuf;
use std::io::{self, BufRead, SeekFrom, Seek};
use std::result::Result as StdResult;
use Run;
use super::{lss, splitterz, splitty, urn, wsplit};

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

pub fn parse<R>(mut source: R, path: Option<PathBuf>, load_icons: bool) -> Result<Run>
    where R: BufRead + Seek
{
    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = lss::parse(&mut source, path) {
        return Ok(run);
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = wsplit::parse(&mut source, load_icons) {
        return Ok(run);
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = splitterz::parse(&mut source, load_icons) {
        return Ok(run);
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = urn::parse(&mut source) {
        return Ok(run);
    }

    source.seek(SeekFrom::Start(0))?;
    if let Ok(run) = splitty::parse(&mut source) {
        return Ok(run);
    }

    Err(Error::NoParserParsedIt)
}
