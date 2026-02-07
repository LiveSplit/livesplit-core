//! Provides the parser for LibreSplit (formerly Urn) splits files.

use crate::{
    Lang, Run, Segment, Time, TimeSpan,
    comparison::world_record,
    platform::{path::Path, prelude::*},
    timing::formatter::{self, TimeFormatter},
};
use alloc::borrow::Cow;
use core::result::Result as StdResult;
use serde_derive::Deserialize;
use serde_json::Error as JsonError;

/// The Error type for splits files that couldn't be parsed by the LibreSplit
/// Parser.
#[derive(Debug, snafu::Snafu)]
#[snafu(context(suffix(false)))]
pub enum Error {
    /// Failed to parse JSON.
    Json {
        /// The underlying error.
        #[cfg_attr(not(feature = "std"), snafu(source(false)))]
        source: JsonError,
    },
}

/// The Result type for the LibreSplit Parser.
pub type Result<T> = StdResult<T, Error>;

#[derive(Deserialize)]
struct Splits<'a> {
    #[serde(borrow)]
    title: Option<Cow<'a, str>>,
    attempt_count: Option<u32>,
    start_delay: Option<TimeSpan>,
    world_record: Option<TimeSpan>,
    splits: Option<Vec<Split<'a>>>,
}

#[derive(Deserialize)]
struct Split<'a> {
    #[serde(borrow)]
    title: Option<Cow<'a, str>>,
    /// The icon can either be a file path or a URL. We don't currently support
    /// URLs.
    #[cfg(feature = "std")]
    #[serde(borrow)]
    icon: Option<Cow<'a, str>>,
    time: Option<TimeSpan>,
    best_time: Option<TimeSpan>,
    best_segment: Option<TimeSpan>,
}

fn parse_time(real_time: TimeSpan) -> Time {
    // Empty Time is stored as zero
    let real_time = if real_time != TimeSpan::zero() {
        Some(real_time)
    } else {
        None
    };

    Time::new().with_real_time(real_time)
}

/// Attempts to parse an LibreSplit (formerly Urn) splits file. In addition to
/// the source to parse, you can specify the path to load additional files from
/// the file system. If you are using livesplit-core in a server-like
/// environment, set this to [`None`]. Only client-side applications should
/// provide a path here.
pub fn parse(source: &str, #[allow(unused)] load_files_path: Option<&Path>) -> Result<Run> {
    let splits: Splits<'_> =
        serde_json::from_str(source).map_err(|source| Error::Json { source })?;

    let mut run = Run::new();

    #[cfg(feature = "std")]
    let mut icon_buf = Vec::new();
    #[cfg(feature = "std")]
    let mut path_buf = Default::default();

    if let Some(title) = splits.title {
        run.set_category_name(title);
    }
    if let Some(attempt_count) = splits.attempt_count {
        run.set_attempt_count(attempt_count);
    }
    if let Some(start_delay) = splits.start_delay {
        run.set_offset(-start_delay);
    }
    if let Some(world_record) = splits.world_record {
        run.metadata_mut()
            .custom_variable_mut(world_record::NAME)
            .permanent()
            // FIXME: This should probably depend on the locale or:
            // FIXME: Custom variables should support TimeSpans directly.
            .set_value(
                formatter::Regular::new()
                    .format(Some(world_record), Lang::English)
                    .to_string(),
            );
    }

    // Best Split Times can be used for the Segment History Every single best
    // split time should be included as its own run, since the best split times
    // could be apart from each other less than the best segments, so we have to
    // assume they are from different runs.
    let mut attempt_history_index = 1;

    if let Some(splits) = splits.splits {
        for split in splits {
            let mut segment = Segment::new(split.title.unwrap_or_default());
            if let Some(time) = split.time {
                segment.set_personal_best_split_time(parse_time(time));
            }
            if let Some(best_segment) = split.best_segment {
                segment.set_best_segment_time(parse_time(best_segment));
            }

            if let Some(best_time) = split.best_time {
                let best_split_time = parse_time(best_time);
                if best_split_time.real_time.is_some() {
                    run.add_attempt_with_index(
                        Time::default(),
                        attempt_history_index,
                        None,
                        None,
                        None,
                    );

                    // Insert a new run that skips to the current split
                    for already_inserted_segment in run.segments_mut() {
                        already_inserted_segment
                            .segment_history_mut()
                            .insert(attempt_history_index, Time::default());
                    }

                    segment
                        .segment_history_mut()
                        .insert(attempt_history_index, best_split_time);

                    attempt_history_index += 1;
                }
            }

            #[cfg(feature = "std")]
            if let Some(load_files_path) = load_files_path
                && let Some(icon_path) = split.icon
                && !icon_path.is_empty()
                && let Ok(image) = crate::settings::Image::from_file(
                    crate::platform::path::relative_to(
                        &mut path_buf,
                        load_files_path,
                        Path::new(icon_path.as_ref()),
                    ),
                    &mut icon_buf,
                    crate::settings::Image::ICON,
                )
            {
                segment.set_icon(image);
            }

            run.push_segment(segment);
        }
    }

    Ok(run)
}
