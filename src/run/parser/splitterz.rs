//! Provides the parser for SplitterZ splits files.

use crate::{Lang, RealTime, Run, Segment, TimeSpan, platform::path::Path, timing};
use alloc::borrow::Cow;
use core::{num::ParseIntError, result::Result as StdResult};
use snafu::ResultExt;

/// The Error type for splits files that couldn't be parsed by the SplitterZ
/// Parser.
#[derive(Debug, snafu::Snafu)]
#[snafu(context(suffix(false)))]
pub enum Error {
    /// An empty splits file was provided.
    Empty,
    /// Expected the name of the category, but didn't find it.
    ExpectedCategoryName,
    /// Expected the attempt count, but didn't find it.
    ExpectedAttemptCount,
    /// Failed to parse the amount of attempts.
    ParseAttemptCount {
        /// The underlying error.
        source: ParseIntError,
    },
    /// Expected the name of the segment, but didn't find it.
    ExpectedSegmentName,
    /// Expected the split time, but didn't find it.
    ExpectedSplitTime,
    /// Failed to parse a split time.
    ParseSplitTime {
        /// The underlying error.
        source: timing::ParseError,
    },
    /// Expected the best segment time, but didn't find it.
    ExpectedBestSegment,
    /// Failed to parse a best segment time.
    ParseBestSegment {
        /// The underlying error.
        source: timing::ParseError,
    },
}

/// The Result type for the SplitterZ parser.
pub type Result<T> = StdResult<T, Error>;

fn unescape(text: &str) -> Cow<'_, str> {
    if text.contains('‡') {
        text.replace('‡', ",").into()
    } else {
        text.into()
    }
}

/// Attempts to parse a SplitterZ splits file. In addition to the source to
/// parse, you can specify the path to load additional files from the file
/// system. If you are using livesplit-core in a server-like environment, set
/// this to [`None`]. Only client-side applications should provide a path here.
pub fn parse(source: &str, #[allow(unused)] load_files_path: Option<&Path>) -> Result<Run> {
    let mut run = Run::new();

    #[cfg(feature = "std")]
    let mut icon_buf = Vec::new();
    #[cfg(feature = "std")]
    let mut path_buf = Default::default();

    let mut lines = source.lines();
    let line = lines.next().ok_or(Error::Empty)?;
    let mut splits = line.split(',');
    run.set_category_name(unescape(splits.next().ok_or(Error::ExpectedCategoryName)?));
    run.set_attempt_count(
        splits
            .next()
            .ok_or(Error::ExpectedAttemptCount)?
            .parse()
            .context(ParseAttemptCount)?,
    );

    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let mut splits = line.split(',');

        let mut segment = Segment::new(unescape(splits.next().ok_or(Error::ExpectedSegmentName)?));

        let time = TimeSpan::parse(
            splits.next().ok_or(Error::ExpectedSplitTime)?,
            Lang::English,
        )
        .context(ParseSplitTime)?;
        if time != TimeSpan::zero() {
            segment.set_personal_best_split_time(RealTime(Some(time)).into());
        }

        let time = TimeSpan::parse(
            splits.next().ok_or(Error::ExpectedBestSegment)?,
            Lang::English,
        )
        .context(ParseBestSegment)?;
        if time != TimeSpan::zero() {
            segment.set_best_segment_time(RealTime(Some(time)).into());
        }

        #[cfg(feature = "std")]
        if let Some(load_files_path) = load_files_path
            && let Some(icon_path) = splits.next()
            && !icon_path.is_empty()
            && let Ok(image) = crate::settings::Image::from_file(
                crate::platform::path::relative_to(
                    &mut path_buf,
                    load_files_path,
                    Path::new(unescape(icon_path).as_ref()),
                ),
                &mut icon_buf,
                crate::settings::Image::ICON,
            )
        {
            segment.set_icon(image);
        }

        run.push_segment(segment);
    }

    for line in lines {
        if let Some(counter) = line.split(',').next() {
            run.metadata_mut()
                .custom_variable_mut(unescape(counter))
                .permanent()
                .set_value("0");
        }
        // The other two lines are not that useful. The number is how much to
        // increment the counter each time and the other is a boolean that does
        // not seem to be exposed to the UI.
    }

    Ok(run)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counters() {
        const RUN: &str = r#"Run Title:,1
SegmentName,0:00:00.00,0.00
SegmentName,0:00:00.00,0.00
SegmentName,0:00:00.00,0.00

Counter,1,True
Counter2,1,True
"#;

        let run = parse(RUN, None).unwrap();
        assert_eq!(run.len(), 3);
        assert_eq!(run.metadata.custom_variable_value("Counter"), Some("0"));
        assert_eq!(run.metadata.custom_variable_value("Counter2"), Some("0"));
    }
}
