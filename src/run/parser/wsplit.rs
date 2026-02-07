//! Provides the parser for WSplit splits files.

use crate::{RealTime, Run, Segment, TimeSpan, platform::path::Path};
use core::{
    num::{ParseFloatError, ParseIntError},
    result::Result as StdResult,
};
use snafu::ResultExt;

/// The Error type for splits files that couldn't be parsed by the WSplit
/// Parser.
#[derive(Debug, snafu::Snafu)]
#[snafu(context(suffix(false)))]
pub enum Error {
    /// Expected the name of the segment, but didn't find it.
    ExpectedSegmentName,
    /// Expected the old time, but didn't find it.
    ExpectedOldTime,
    /// Expected the split time, but didn't find it.
    ExpectedPbTime,
    /// Expected the best segment time, but didn't find it.
    ExpectedBestTime,
    /// Failed to parse the amount of attempts.
    Attempt {
        /// The underlying error.
        source: ParseIntError,
    },
    /// Failed to parse the time the timer starts at.
    Offset {
        /// The underlying error.
        source: ParseFloatError,
    },
    /// Failed to parse the personal best split time of a segment.
    PbTime {
        /// The underlying error.
        source: ParseFloatError,
    },
    /// Failed to parse the best segment time of a segment.
    BestSegment {
        /// The underlying error.
        source: ParseFloatError,
    },
    /// Failed to parse the "Old Run" time of a segment.
    OldTime {
        /// The underlying error.
        source: ParseFloatError,
    },
}

/// The Result type for the WSplit Parser.
pub type Result<T> = StdResult<T, Error>;

/// Attempts to parse a WSplit splits file. In addition to the source to parse,
/// you can specify the path to load additional files from the file system. If
/// you are using livesplit-core in a server-like environment, set this to
/// `None`. Only client-side applications should provide a path here.
pub fn parse(source: &str, #[allow(unused)] load_files_path: Option<&Path>) -> Result<Run> {
    let mut run = Run::new();
    #[cfg(feature = "std")]
    let mut icon_buf = Vec::new();
    #[cfg(feature = "std")]
    let mut path_buf = Default::default();
    #[cfg(feature = "std")]
    let mut icons_list = Vec::new();
    let mut old_run_exists = false;
    let mut goal = None;

    for line in source.lines() {
        if line.is_empty() {
            continue;
        }

        if let Some(title) = line.strip_prefix("Title=") {
            run.set_category_name(title);
        } else if let Some(attempts) = line.strip_prefix("Attempts=") {
            run.set_attempt_count(attempts.parse().context(Attempt)?);
        } else if let Some(offset) = line.strip_prefix("Offset=") {
            if !offset.is_empty() {
                run.set_offset(TimeSpan::from_milliseconds(
                    -offset.parse::<f64>().context(Offset)?,
                ));
            }
        } else if line.starts_with("Size=") {
            // Ignore
        } else if let Some(_icons) = line.strip_prefix("Icons=") {
            #[cfg(feature = "std")]
            if let Some(load_files_path) = load_files_path {
                icons_list.clear();
                for path in _icons.split(',') {
                    icons_list.push(
                        if let Some(path) = path.strip_prefix('"')
                            && let Some(path) = path.strip_suffix('"')
                            && let Ok(image) = crate::settings::Image::from_file(
                                crate::platform::path::relative_to(
                                    &mut path_buf,
                                    load_files_path,
                                    Path::new(path),
                                ),
                                &mut icon_buf,
                                crate::settings::Image::ICON,
                            )
                        {
                            image
                        } else {
                            Default::default()
                        },
                    );
                }
            }
        } else if let Some(goal_value) = line.strip_prefix("Goal=") {
            if !goal_value.is_empty() {
                goal = Some(goal_value);
            }
        } else {
            // must be a split Kappa
            let mut split_info = line.split(',');

            let segment_name = split_info.next().ok_or(Error::ExpectedSegmentName)?;
            let old_time = split_info.next().ok_or(Error::ExpectedOldTime)?;
            let pb_time = split_info.next().ok_or(Error::ExpectedPbTime)?;
            let best_time = split_info.next().ok_or(Error::ExpectedBestTime)?;

            let pb_time = TimeSpan::from_seconds(pb_time.parse().context(PbTime)?);
            let best_time = TimeSpan::from_seconds(best_time.parse().context(BestSegment)?);
            let old_time = TimeSpan::from_seconds(old_time.parse().context(OldTime)?);

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

    if let Some(goal) = goal {
        if run.category_name().is_empty() {
            run.set_category_name(goal);
        } else {
            run.metadata_mut()
                .custom_variable_mut("Goal")
                .permanent()
                .set_value(goal);
        }
    }

    if old_run_exists {
        run.add_custom_comparison("Old Run").unwrap();
    }

    #[cfg(feature = "std")]
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
        const RUN: &str = r#"Title=WarioWare, Inc
Attempts=1
Offset=0
Size=374,61
Goal=sub 2h
Introduction,0,85.48,85.48
Jimmy,0,219.68,134.2
"#;

        let run = parse(RUN, None).unwrap();
        assert_eq!(run.category_name(), "WarioWare, Inc");
        assert_eq!(run.metadata().custom_variable_value("Goal"), Some("sub 2h"));
        assert_eq!(run.len(), 2);
    }

    #[test]
    fn skip_goal_if_its_empty() {
        const RUN: &str = r#"Title=WarioWare, Inc
Attempts=1
Offset=0
Size=374,61
Goal=
Introduction,0,85.48,85.48
Jimmy,0,219.68,134.2
"#;

        let run = parse(RUN, None).unwrap();
        assert_eq!(run.category_name(), "WarioWare, Inc");
        assert_eq!(run.len(), 2);
    }
}
