//! Provides the parser for FaceSplit splits files.

use crate::{settings::Image, timing, RealTime, Run, Segment, Time, TimeSpan};
use alloc::borrow::Cow;
use core::num::ParseIntError;
use core::result::Result as StdResult;
use snafu::{OptionExt, ResultExt};
use std::io::{self, BufRead};

/// The Error type for splits files that couldn't be parsed by the FaceSplit
/// Parser.
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// Expected the title, but didn't find it.
    ExpectedTitle,
    /// Failed to read the title.
    ReadTitle {
        /// The underlying error.
        source: io::Error,
    },
    /// Expected the goal, but didn't find it.
    ExpectedGoal,
    /// Failed to read the goal.
    ReadGoal {
        /// The underlying error.
        source: io::Error,
    },
    /// Expected the attempt count, but didn't find it.
    ExpectedAttemptCount,
    /// Failed to read the attempt count.
    ReadAttemptCount {
        /// The underlying error.
        source: io::Error,
    },
    /// Failed to parse the attempt count.
    ParseAttemptCount {
        /// The underlying error.
        source: ParseIntError,
    },
    /// Failed to read the next segment.
    ReadSegment {
        /// The underlying error.
        source: io::Error,
    },
    /// Expected the name of a segment, but didn't find it.
    ExpectedSegmentName,
    /// Expected the split time of a segment, but didn't find it.
    ExpectedSplitTime,
    /// Failed to parse the split time of a segment.
    ParseSplitTime {
        /// The underlying error.
        source: timing::ParseError,
    },
    /// Expected the best segment time of a segment, but didn't find it.
    ExpectedBestSegmentTime,
    /// Failed to parse the best segment time of a segment.
    ParseBestSegmentTime {
        /// The underlying error.
        source: timing::ParseError,
    },
}

/// The Result type for the FaceSplit Parser.
pub type Result<T> = StdResult<T, Error>;

fn parse_time(time: &str) -> StdResult<Time, timing::ParseError> {
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

    run.set_category_name(lines.next().context(ExpectedTitle)?.context(ReadTitle)?);

    let goal = lines.next().context(ExpectedGoal)?.context(ReadGoal)?;
    if !goal.trim_start().is_empty() {
        run.metadata_mut()
            .custom_variable_mut("Goal")
            .permanent()
            .set_value(goal);
    }

    run.set_attempt_count(
        lines
            .next()
            .context(ExpectedAttemptCount)?
            .context(ReadAttemptCount)?
            .parse()
            .context(ParseAttemptCount)?,
    );
    lines.next(); // FIXME: Store runs completed somehow

    for line in lines {
        let line = line.context(ReadSegment)?;
        let mut splits = line.splitn(5, '-');

        let segment_name = replace(splits.next().context(ExpectedSegmentName)?, r#""?""#, "-");
        let mut segment = Segment::new(segment_name);

        let split_time =
            parse_time(splits.next().context(ExpectedSplitTime)?).context(ParseSplitTime)?;
        segment.set_personal_best_split_time(split_time);

        let best_segment = parse_time(splits.next().context(ExpectedBestSegmentTime)?)
            .context(ParseBestSegmentTime)?;
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
