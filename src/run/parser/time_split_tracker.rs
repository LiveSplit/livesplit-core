use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::PathBuf;
use std::result::Result as StdResult;
use std::num::ParseIntError;
use chrono::{TimeZone, Utc};
use {time, AtomicDateTime, Image, RealTime, Run, Segment, Time, TimeSpan};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Empty
        ExpectedAttemptCount
        ExpectedOffset
        ExpectedTitleLine
        ExpectedCategoryName
        ExpectedSplitName
        ExpectedBestSegment
        ExpectedComparisonTime
        ExpectedIconLine
        Int(err: ParseIntError) {
            from()
        }
        Time(err: time::ParseError) {
            from()
        }
        Io(err: io::Error) {
            from()
        }
    }
}

pub type Result<T> = StdResult<T, Error>;

fn parse_time_optional(time: &str) -> Result<Option<TimeSpan>> {
    let time: TimeSpan = time.parse()?;
    if time == TimeSpan::zero() {
        Ok(None)
    } else {
        Ok(Some(time))
    }
}

pub fn parse<R: BufRead>(source: R, path_for_loading_other_files: Option<PathBuf>) -> Result<Run> {
    let mut run = Run::new();
    let mut buf = Vec::new();
    let path = path_for_loading_other_files;

    let mut lines = source.lines();

    let line = lines.next().ok_or(Error::Empty)??;
    let mut splits = line.split('\t');
    run.attempt_count = splits.next().ok_or(Error::ExpectedAttemptCount)?.parse()?;
    run.offset = splits.next().ok_or(Error::ExpectedOffset)?.parse()?;
    if let (&Some(ref path), Some(file)) = (&path, splits.next()) {
        let path = path.with_file_name(file);
        if let Ok(image) = Image::from_file(path, &mut buf) {
            run.game_icon = image;
        }
    }

    let line = lines.next().ok_or(Error::ExpectedTitleLine)??;
    let mut splits = line.split('\t');
    run.category_name = splits.next().ok_or(Error::ExpectedCategoryName)?.to_string();
    splits.next(); // Skip one element
    let comparisons = splits.collect::<Vec<_>>();

    while let Some(line) = lines.next() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let mut splits = line.split('\t');
        let mut segment = Segment::new(splits.next().ok_or(Error::ExpectedSplitName)?);
        let best_segment = parse_time_optional(splits.next().ok_or(Error::ExpectedBestSegment)?)?;
        segment.set_best_segment_time(RealTime(best_segment).into());

        let mut pb_time = Time::new();
        for comparison in &comparisons {
            let time = segment.comparison_mut(comparison);
            pb_time.real_time =
                parse_time_optional(splits.next().ok_or(Error::ExpectedComparisonTime)?)?;
            time.real_time = pb_time.real_time;
        }
        segment.set_personal_best_split_time(pb_time);

        let line = lines.next().ok_or(Error::ExpectedIconLine)??;

        if let Some(ref path) = path {
            let file = line.trim_right();
            if !file.is_empty() {
                let path = path.with_file_name(file);
                if let Ok(image) = Image::from_file(path, &mut buf) {
                    segment.set_icon(image);
                }
            }
        }

        run.segments.push(segment);
    }

    parse_history(&mut run, path).ok();

    for comparison in comparisons {
        run.add_custom_comparison(comparison);
    }

    Ok(run)
}

fn parse_history(run: &mut Run, path: Option<PathBuf>) -> StdResult<(), ()> {
    if let Some(mut path) = path {
        path.set_extension("");
        let mut path = path.into_os_string();
        path.push("-RunLog.txt");
        let path = PathBuf::from(path);

        let lines = BufReader::new(File::open(path).map_err(|_| ())?).lines();
        let mut attempt_id = 1;

        for line in lines.skip(1) {
            let line = line.map_err(|_| ())?;
            let mut splits = line.split('\t');
            let time_stamp = splits.next().ok_or(())?;
            let started = Utc.datetime_from_str(time_stamp, "%Y/%m/%d %R")
                .map_err(|_| ())?;
            let completed = splits.next().ok_or(())? == "C";
            let split_times: Vec<_> = splits
                .map(parse_time_optional)
                .collect::<Result<_>>()
                .map_err(|_| ())?;
            let mut final_time = Time::default();
            let mut ended = None;
            if completed {
                if let Some(&last_split_time) = split_times.last() {
                    final_time.real_time = last_split_time;
                    if let Some(final_time) = final_time.real_time {
                        let ended_date = started + final_time.to_duration();
                        ended = Some(AtomicDateTime::new(ended_date, false));
                    }
                }
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
                run.segments.iter_mut().zip(split_times.into_iter())
            {

                let mut segment_time = Time::default();
                if let Some(current_split) = current_split {
                    segment_time.real_time = Some(current_split - last_split);
                    last_split = current_split;
                }

                segment
                    .segment_history_mut()
                    .insert(attempt_id, segment_time);
                if TimeSpan::option_op(
                    segment_time.real_time,
                    segment.best_segment_time().real_time,
                    |a, b| a < b,
                ).unwrap_or(false)
                {
                    segment.set_best_segment_time(segment_time);
                }
            }

            attempt_id += 1;
        }
    }
    Ok(())
}
