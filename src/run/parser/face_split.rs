//! Provides the parser for FaceSplit splits files.

use std::borrow::Cow;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::result::Result as StdResult;
use {timing, Image, RealTime, Run, Segment, Time, TimeSpan};

quick_error! {
    /// The Error type for splits files that couldn't be parsed by the FaceSplit
    /// Parser.
    #[derive(Debug)]
    pub enum Error {
        /// Expected the title, but didn't find it.
        ExpectedTitle {}
        /// Expected the attempt count, but didn't find it.
        ExpectedAttemptCount {}
        /// Expected the name of the segment, but didn't find it.
        ExpectedSegmentName {}
        /// Expected the split time, but didn't find it.
        ExpectedSplitTime {}
        /// Expected the best segment time, but didn't find it.
        ExpectedBestSegmentTime {}
        /// Failed to parse the amount of attempts.
        Attempt(err: ParseIntError) {
            from()
        }
        /// Failed to parse a time.
        Time(err: timing::ParseError) {
            from()
        }
        /// Failed to read from the source.
        Io(err: io::Error) {
            from()
        }
    }
}

/// The Result type for the FaceSplit Parser.
pub type Result<T> = StdResult<T, Error>;

fn parse_time(time: &str) -> Result<Time> {
    // Replace "," by "." as "," wouldn't parse
    let time: TimeSpan = replace(time, ",", ".").parse()?;
    // Skipped is stored as a zero time in FaceSplit Splits
    if time == TimeSpan::zero() {
        Ok(Time::default())
    } else {
        Ok(RealTime(Some(time)).into())
    }
}

fn replace<'a>(text: &'a str, a: &'a str, b: &str) -> Cow<'a, str> {
    if text.contains(a) {
        text.replace(a, b).into()
    } else {
        text.into()
    }
}

/// Attempts to parse a FaceSplit splits file. In addition to the source to
/// parse, you need to specify if additional files for the icons should be
/// loaded from the file system. If you are using livesplit-core in a
/// server-like environment, set this to `false`. Only client-side applications
/// should set this to `true`.
pub fn parse<R: BufRead>(source: R, load_icons: bool) -> Result<Run> {
    let mut run = Run::new();
    let mut icon_buf = Vec::new();
    let mut lines = source.lines();

    run.set_category_name(lines.next().ok_or(Error::ExpectedTitle)??);
    lines.next(); // TODO Store Goal
    run.set_attempt_count(lines.next().ok_or(Error::ExpectedAttemptCount)??.parse()?);
    lines.next(); // TODO Store runs completed somehow

    for line in lines {
        let line = line?;
        let mut splits = line.splitn(5, '-');

        let segment_name = replace(
            splits.next().ok_or(Error::ExpectedSegmentName)?,
            r#""?""#,
            "-",
        );
        let mut segment = Segment::new(segment_name);

        let split_time = parse_time(splits.next().ok_or(Error::ExpectedSplitTime)?)?;
        segment.set_personal_best_split_time(split_time);

        let best_segment = parse_time(splits.next().ok_or(Error::ExpectedBestSegmentTime)?)?;
        segment.set_best_segment_time(best_segment);

        splits.next(); // Skip Segment Time

        if load_icons {
            if let Some(icon_path) = splits.next() {
                if !icon_path.is_empty() {
                    if let Ok(image) = Image::from_file(icon_path, &mut icon_buf) {
                        segment.set_icon(image);
                    }
                }
            }
        }

        run.push_segment(segment);
    }

    Ok(run)
}
