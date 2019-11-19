//! Provides the parser for Time Split Tracker splits files.

use super::super::ComparisonError;
use crate::{settings::Image, timing, AtomicDateTime, RealTime, Run, Segment, Time, TimeSpan};
use chrono::{TimeZone, Utc};
use core::num::ParseIntError;
use core::result::Result as StdResult;
use snafu::{OptionExt, ResultExt};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

/// The Error type for splits files that couldn't be parsed by the Time
/// Split Tracker Parser.
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// An empty splits file was provided.
    Empty,
    /// Failed to read the initial information line.
    ReadInitialLine {
        /// The underlying error.
        source: io::Error,
    },
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
    /// Failed to read the line containing the title.
    ReadTitleLine {
        /// The underlying error.
        source: io::Error,
    },
    /// Expected the name of the category, but didn't find it.
    ExpectedCategoryName,
    /// Failed to read a segment line.
    ReadSegmentLine {
        /// The underlying error.
        source: io::Error,
    },
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
    /// Failed to read a line containing the icon of a segment.
    ReadIconLine {
        /// The underlying error.
        source: io::Error,
    },
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
pub fn parse<R: BufRead>(source: R, path_for_loading_other_files: Option<PathBuf>) -> Result<Run> {
    let mut run = Run::new();
    let mut buf = Vec::new();
    let path = path_for_loading_other_files;

    let mut lines = source.lines();

    let line = lines.next().context(Empty)?.context(ReadInitialLine)?;
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

    catch! {
        let path = path.as_ref()?.with_file_name(splits.next()?);
        let image = Image::from_file(path, &mut buf).ok()?;
        run.set_game_icon(image);
    };

    let line = lines
        .next()
        .context(ExpectedTitleLine)?
        .context(ReadTitleLine)?;
    let mut splits = line.split('\t');
    run.set_category_name(splits.next().context(ExpectedCategoryName)?);
    splits.next(); // Skip one element
    let mut comparisons = splits.map(ToOwned::to_owned).collect::<Vec<_>>();

    for comparison in &mut comparisons {
        let mut name = comparison.to_owned();
        let mut good_name = name.to_owned();
        let mut number = 2;
        loop {
            match run.add_custom_comparison(good_name.clone()) {
                Ok(_) => break,
                Err(ComparisonError::DuplicateName) => {
                    good_name = format!("{}{}", name, number);
                    number += 1;
                }
                Err(ComparisonError::NameStartsWithRace) => {
                    name = name[6..].to_string();
                    good_name = name.to_owned();
                }
            }
        }
        *comparison = good_name;
    }

    while let Some(line) = lines.next() {
        let line = line.context(ReadSegmentLine)?;
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

        let line = lines
            .next()
            .context(ExpectedIconLine)?
            .context(ReadIconLine)?;

        catch! {
            let file = line.trim_end();
            if !file.is_empty() {
                let path = path.as_ref()?.with_file_name(file);
                let image = Image::from_file(path, &mut buf).ok()?;
                segment.set_icon(image);
            }
        };

        run.push_segment(segment);
    }

    parse_history(&mut run, path).ok();

    Ok(run)
}

fn parse_history(run: &mut Run, path: Option<PathBuf>) -> StdResult<(), ()> {
    if let Some(mut path) = path {
        path.set_extension("");
        let mut path = path.into_os_string();
        path.push("-RunLog.txt");
        let path = PathBuf::from(path);

        let lines = BufReader::new(File::open(path).map_err(drop)?).lines();
        let mut attempt_id = 1;

        for line in lines.skip(1) {
            let line = line.map_err(drop)?;
            let mut splits = line.split('\t');
            let time_stamp = splits.next().ok_or(())?;
            let started = Utc
                .datetime_from_str(time_stamp, "%Y/%m/%d %R")
                .map_err(drop)?;
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
