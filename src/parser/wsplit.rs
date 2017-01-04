use std::path::PathBuf;
use std::io::{self, BufRead};
use std::result::Result as StdResult;
use std::num::{ParseFloatError, ParseIntError};
use {Run, TimeSpan, Time, Segment};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        ExpectedName
        ExpectedOldTime
        ExpectedPbTime
        ExpectedBestTime
        Attempt(err: ParseIntError) {
            from()
        }
        Time(err: ParseFloatError) {
            from()
        }
        Io(err: io::Error) {
            from()
        }
    }
}

pub type Result<T> = StdResult<T, Error>;

pub fn parse<R: BufRead>(source: R, path: Option<PathBuf>) -> Result<Run> {

    let mut run = Run::new(Vec::new());
    // let mut icons_list = Vec::new();
    let mut old_run_exists = false;

    for line in source.lines() {
        let line = line?;
        let line = line.trim();
        if !line.is_empty() {
            if line.starts_with("Title=") {
                run.set_category_name(&line["Title=".len()..]);
            } else if line.starts_with("Attempts=") {
                run.set_attempt_count(line["Attempts=".len()..].parse()?);
            } else if line.starts_with("Offset=") {
                let offset = &line["Offset=".len()..];
                if !offset.is_empty() {
                    run.set_offset(TimeSpan::from_seconds(offset.parse::<f64>()? * -1000.0));
                }
            } else if line.starts_with("Size=") {
                // Ignore
            } else if line.starts_with("Icons=") {
                // TODO Implement Icons
            } else {
                // must be a split Kappa
                let mut split_info = line.split(',');

                let name = split_info.next().ok_or(Error::ExpectedName)?;
                let old_time = split_info.next().ok_or(Error::ExpectedOldTime)?;
                let pb_time = split_info.next().ok_or(Error::ExpectedPbTime)?;
                let best_time = split_info.next().ok_or(Error::ExpectedBestTime)?;

                let mut segment = Segment::new(name);
                let pb_real_time = TimeSpan::from_seconds(pb_time.parse()?);
                let best_real_time = TimeSpan::from_seconds(best_time.parse()?);
                let old_real_time = TimeSpan::from_seconds(old_time.parse()?);

                let mut pb_time = Time::new();
                let mut best_time = Time::new();
                let mut old_time = Time::new();

                if pb_real_time != TimeSpan::zero() {
                    pb_time.real_time = Some(pb_real_time);
                }
                if best_real_time != TimeSpan::zero() {
                    best_time.real_time = Some(best_real_time);
                }
                if old_real_time != TimeSpan::zero() {
                    old_time.real_time = Some(old_real_time);
                    *segment.comparison_mut("Old Run") = old_time;
                    old_run_exists = true;
                }

                segment.set_personal_best_split_time(pb_time);
                segment.set_best_segment_time(best_time);
                run.push_segment(segment);
            }
        }
    }

    if old_run_exists {
        run.add_custom_comparison("Old Run");
    }

    // TODO Insert Icons

    run.set_path(path);
    Ok(run)
}