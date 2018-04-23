//! Provides the parser for WSplit splits files.

use std::io::{self, BufRead};
use std::num::{ParseFloatError, ParseIntError};
use std::result::Result as StdResult;
use {Image, RealTime, Run, Segment, TimeSpan};

quick_error! {
    /// The Error type for splits files that couldn't be parsed by the WSplit
    /// Parser.
    #[derive(Debug)]
    pub enum Error {
        /// Expected the name of the segment, but didn't find it.
        ExpectedSegmentName {}
        /// Expected the old time, but didn't find it.
        ExpectedOldTime {}
        /// Expected the split time, but didn't find it.
        ExpectedPbTime {}
        /// Expected the best segment time, but didn't find it.
        ExpectedBestTime {}
        /// Failed to parse the amount of attempts.
        Attempt(err: ParseIntError) {
            from()
        }
        /// Failed to parse a time.
        Time(err: ParseFloatError) {
            from()
        }
        /// Failed to read from the source.
        Io(err: io::Error) {
            from()
        }
    }
}

/// The Result type for the WSplit Parser.
pub type Result<T> = StdResult<T, Error>;

/// Attempts to parse a WSplit splits file. In addition to the source to parse,
/// you need to specify if additional files for the icons should be loaded from
/// the file system. If you are using livesplit-core in a server-like
/// environment, set this to `false`. Only client-side applications should set
/// this to `true`.
pub fn parse<R: BufRead>(source: R, load_icons: bool) -> Result<Run> {
    let mut run = Run::new();
    let mut icon_buf = Vec::new();
    let mut icons_list = Vec::new();
    let mut old_run_exists = false;
    let mut goal = None;

    for line in source.lines() {
        let line = line?;
        if !line.is_empty() {
            if line.starts_with("Title=") {
                run.set_category_name(&line["Title=".len()..]);
            } else if line.starts_with("Attempts=") {
                run.set_attempt_count(line["Attempts=".len()..].parse()?);
            } else if line.starts_with("Offset=") {
                let offset = &line["Offset=".len()..];
                if !offset.is_empty() {
                    run.set_offset(TimeSpan::from_milliseconds(-offset.parse::<f64>()?));
                }
            } else if line.starts_with("Size=") {
                // Ignore
            } else if line.starts_with("Icons=") {
                if load_icons {
                    let icons = &line["Icons=".len()..];
                    icons_list.clear();
                    for path in icons.split(',') {
                        if path.len() >= 2 {
                            let path = &path[1..path.len() - 1];
                            if let Ok(image) = Image::from_file(path, &mut icon_buf) {
                                icons_list.push(image);
                                continue;
                            }
                        }
                        icons_list.push(Image::default());
                    }
                }
            } else if line.starts_with("Goal=") {
                goal = Some(line["Goal=".len()..].to_owned());
            } else {
                // must be a split Kappa
                let mut split_info = line.split(',');

                let segment_name = split_info.next().ok_or(Error::ExpectedSegmentName)?;
                let old_time = split_info.next().ok_or(Error::ExpectedOldTime)?;
                let pb_time = split_info.next().ok_or(Error::ExpectedPbTime)?;
                let best_time = split_info.next().ok_or(Error::ExpectedBestTime)?;

                let pb_time = TimeSpan::from_seconds(pb_time.parse()?);
                let best_time = TimeSpan::from_seconds(best_time.parse()?);
                let old_time = TimeSpan::from_seconds(old_time.parse()?);

                let mut segment = Segment::new(segment_name);

                if pb_time != TimeSpan::zero() {
                    segment.set_personal_best_split_time(RealTime(Some(pb_time)).into());
                }
                if best_time != TimeSpan::zero() {
                    segment.set_best_segment_time(RealTime(Some(best_time)).into());
                }
                if old_time != TimeSpan::zero() {
                    *segment.comparison_mut("Old Run") = RealTime(Some(old_time)).into();
                    old_run_exists = true;
                }

                run.push_segment(segment);
            }
        }
    }

    if let Some(goal) = goal {
        if run.category_name().is_empty() {
            run.set_category_name(goal);
        } else {
            let category_name = format!("{} (Goal: {})", run.category_name(), goal);
            run.set_category_name(category_name);
        }
    }

    if old_run_exists {
        run.add_custom_comparison("Old Run")
            .expect("WSplit: Old Run");
    }

    for (icon, segment) in icons_list.into_iter().zip(run.segments_mut().iter_mut()) {
        segment.set_icon(icon);
    }

    Ok(run)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn goal_parsing() {
        const RUN: &[u8] = br#"Title=WarioWare, Inc
Attempts=1
Offset=0
Size=374,61
Goal=sub 2h
Introduction,0,85.48,85.48
Jimmy,0,219.68,134.2
"#;

        let run = parse(RUN, false).unwrap();
        assert_eq!(run.category_name(), "WarioWare, Inc (Goal: sub 2h)");
        assert_eq!(run.len(), 2);
    }
}
