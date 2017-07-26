use std::borrow::Cow;
use std::io::{self, BufRead};
use std::result::Result as StdResult;
use std::num::ParseIntError;
use {Run, time, Image, TimeSpan, Time, RealTime, Segment};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        ExpectedTitle
        ExpectedAttemptCount
        ExpectedSplitName
        ExpectedSplitTime
        ExpectedBestSegmentTime
        Attempt(err: ParseIntError) {
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
            splits.next().ok_or(Error::ExpectedSplitName)?,
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
