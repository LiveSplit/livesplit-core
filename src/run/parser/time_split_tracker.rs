//! Provides the parser for Time Split Tracker splits files.

use super::super::ComparisonError;
use crate::{
    comparison::RACE_COMPARISON_PREFIX,
    platform::{
        path::{Path, PathBuf},
        prelude::*,
    },
    timing, RealTime, Run, Segment, Time, TimeSpan,
};
#[cfg(feature = "std")]
use crate::{settings::Image, AtomicDateTime};
use alloc::borrow::Cow;
use core::{fmt::Write, num::ParseIntError, result::Result as StdResult};
use snafu::{OptionExt, ResultExt};

/// The Error type for splits files that couldn't be parsed by the Time
/// Split Tracker Parser.
#[derive(Debug, snafu::Snafu)]
#[snafu(context(suffix(false)))]
pub enum Error {
    /// An empty splits file was provided.
    Empty,
    /// Expected the attempt count, but didn't find it.
    ExpectedAttemptCount,
    /// Failed to parse the attempt count.
    ParseAttemptCount {
        /// The underlying error.
        source: ParseIntError,
    },
    /// Expected the start time offset, but didn't find it.
    ExpectedOffset,
    /// Failed to parse the start time offset.
    ParseOffset {
        /// The underlying error.
        source: timing::ParseError,
    },
    /// Expected the line containing the title, but didn't find it.
    ExpectedTitleLine,
    /// Expected the name of the category, but didn't find it.
    ExpectedCategoryName,
    /// Expected the name of a segment, but didn't find it.
    ExpectedSegmentName,
    /// Expected the best segment time of a segment, but didn't find it.
    ExpectedBestSegment,
    /// Failed to parse the best segment time of a segment.
    ParseBestSegment {
        /// The underlying error.
        source: timing::ParseError,
    },
    /// Expected the time for a comparison of a segment, but didn't find it.
    ExpectedComparisonTime,
    /// Failed to parse the time for a comparison of a segment.
    ParseComparisonTime {
        /// The underlying error.
        source: timing::ParseError,
    },
    /// Expected a line containing the icon of a segment, but didn't find it.
    ExpectedIconLine,
}

/// The Result type for the Time Split Tracker parser.
pub type Result<T> = StdResult<T, Error>;

fn parse_time_optional(time: &str) -> StdResult<Option<TimeSpan>, timing::ParseError> {
    let time: TimeSpan = time.parse()?;
    if time == TimeSpan::zero() {
        Ok(None)
    } else {
        Ok(Some(time))
    }
}

/// Attempts to parse a Time Split Tracker splits file. In addition to the
/// source to parse, you can specify the path of the splits file, which is then
/// use to load the run log file from the file system. This is entirely
/// optional. If you are using livesplit-core in a server-like environment, set
/// this to `None`. Only client-side applications should provide the path here.
pub fn parse(
    source: &str,
    #[allow(unused)] path_for_loading_other_files: Option<&Path>,
) -> Result<Run> {
    let mut run = Run::new();
    #[cfg(feature = "std")]
    let mut buf = Vec::new();

    let mut lines = source.lines();

    let line = lines.next().context(Empty)?;
    let mut splits = line.split('\t');

    let attempt_count = splits.next().context(ExpectedAttemptCount)?;
    if !attempt_count.is_empty() {
        run.set_attempt_count(attempt_count.parse().context(ParseAttemptCount)?);
    }

    run.set_offset(
        splits
            .next()
            .context(ExpectedOffset)?
            .parse()
            .context(ParseOffset)?,
    );

    #[cfg(feature = "std")]
    let mut path = path_for_loading_other_files.map(Path::to_path_buf);

    #[cfg(feature = "std")]
    catch! {
        let path = path.as_mut()?;
        path.set_file_name(splits.next()?);
        let image = Image::from_file(path, &mut buf).ok()?;
        run.set_game_icon(image);
    };

    let line = lines.next().context(ExpectedTitleLine)?;
    let mut splits = line.split('\t');
    run.set_category_name(splits.next().context(ExpectedCategoryName)?);
    splits.next(); // Skip one element

    let mut comparisons = splits.map(Cow::Borrowed).collect::<Vec<_>>();

    for comparison in &mut comparisons {
        let orig_len = comparison.len();
        let mut number = 2;
        loop {
            match run.add_custom_comparison(&**comparison) {
                Ok(_) => break,
                Err(ComparisonError::DuplicateName) => {
                    let comparison = comparison.to_mut();
                    comparison.drain(orig_len..);
                    let _ = write!(comparison, " {number}");
                    number += 1;
                }
                Err(ComparisonError::NameStartsWithRace) => {
                    let comparison = comparison.to_mut();
                    // After removing the `[Race]`, there might be some
                    // whitespace we want to trim too.
                    let len_after_trimming = comparison[RACE_COMPARISON_PREFIX.len()..]
                        .trim_start()
                        .len();
                    let shrunk_by = comparison.len() - len_after_trimming;
                    comparison.drain(..shrunk_by);
                }
            }
        }
    }

    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }

        let mut splits = line.split('\t');
        let mut segment = Segment::new(splits.next().context(ExpectedSegmentName)?);
        let best_segment = parse_time_optional(splits.next().context(ExpectedBestSegment)?)
            .context(ParseBestSegment)?;
        segment.set_best_segment_time(RealTime(best_segment).into());

        let mut pb_time = Time::new();
        for comparison in &comparisons {
            let time = segment.comparison_mut(comparison);
            pb_time.real_time = parse_time_optional(splits.next().context(ExpectedComparisonTime)?)
                .context(ParseComparisonTime)?;
            time.real_time = pb_time.real_time;
        }
        segment.set_personal_best_split_time(pb_time);

        let _line = lines.next().context(ExpectedIconLine)?;

        #[cfg(feature = "std")]
        catch! {
            let file = _line.trim_end();
            if !file.is_empty() {
                let path = path.as_mut()?;
                path.set_file_name(file);
                let image = Image::from_file(path, &mut buf).ok()?;
                segment.set_icon(image);
            }
        };

        run.push_segment(segment);
    }

    #[cfg(feature = "std")]
    parse_history(&mut run, path).ok();

    Ok(run)
}

#[cfg(feature = "std")]
fn parse_history(run: &mut Run, path: Option<PathBuf>) -> StdResult<(), ()> {
    if let Some(mut path) = path {
        path.set_extension("");
        let mut path = path.into_os_string();
        path.push("-RunLog.txt");
        let path = PathBuf::from(path);

        let file = std::fs::read_to_string(path).map_err(drop)?;
        let mut lines = file.lines();
        let mut attempt_id = 1;

        lines.next(); // Skip the first line

        for line in lines {
            let mut splits = line.split('\t');
            let time_stamp = splits.next().ok_or(())?;
            let started = parse_date(time_stamp).ok_or(())?;
            let completed = splits.next().ok_or(())? == "C";
            let split_times: Vec<_> = splits
                .map(parse_time_optional)
                .collect::<StdResult<_, _>>()
                .map_err(drop)?;
            let mut final_time = Time::default();
            let mut ended = None;
            if completed {
                catch! {
                    let last_split_time = split_times.last()?;
                    final_time.real_time = *last_split_time;
                    let final_time = final_time.real_time?;
                    let ended_date = started + final_time.to_duration();
                    ended = Some(AtomicDateTime::new(ended_date, false));
                };
            }

            run.add_attempt_with_index(
                final_time,
                attempt_id,
                Some(AtomicDateTime::new(started, false)),
                ended,
                None,
            );

            let mut last_split = TimeSpan::zero();
            for (segment, current_split) in
                run.segments_mut().iter_mut().zip(split_times.into_iter())
            {
                let mut segment_time = Time::default();
                if let Some(current_split) = current_split {
                    segment_time.real_time = Some(current_split - last_split);
                    last_split = current_split;
                }

                segment
                    .segment_history_mut()
                    .insert(attempt_id, segment_time);

                if catch! {
                    segment_time.real_time? < segment.best_segment_time().real_time?
                }
                .unwrap_or(false)
                {
                    segment.set_best_segment_time(segment_time);
                }
            }

            attempt_id += 1;
        }
    }
    Ok(())
}

#[cfg(feature = "std")]
fn parse_date(text: &str) -> Option<crate::DateTime> {
    let (year, rem) = text.split_once('/')?;
    let (month, rem) = rem.split_once('/')?;
    let (day, rem) = rem.split_once(' ')?;
    let (hour, minute) = rem.split_once(':')?;
    Some(
        time::PrimitiveDateTime::new(
            time::Date::from_calendar_date(
                year.parse().ok()?,
                month.parse::<u8>().ok()?.try_into().ok()?,
                day.parse().ok()?,
            )
            .ok()?,
            time::Time::from_hms(hour.parse().ok()?, minute.parse().ok()?, 0).ok()?,
        )
        .assume_utc(),
    )
}
