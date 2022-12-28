//! Provides the parser for LiveSplit splits files.

use super::super::ComparisonError;
use crate::{
    platform::prelude::*,
    util::xml::{
        helper::{
            attribute, attribute_escaped_err, end_tag, optional_attribute_escaped_err,
            parse_attributes, parse_base, parse_children, reencode_children, text,
            text_as_escaped_string_err, text_parsed, Error as XmlError,
        },
        Reader,
    },
    AtomicDateTime, DateTime, Run, RunMetadata, Segment, Time, TimeSpan,
};
use alloc::borrow::Cow;
use core::{mem::MaybeUninit, str};
use time::{Date, PrimitiveDateTime};

/// The Error type for splits files that couldn't be parsed by the LiveSplit
/// Parser.
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// The underlying XML format couldn't be parsed.
    Xml {
        /// The underlying error.
        source: XmlError,
    },
    /// Failed to parse an integer.
    ParseInt {
        /// The underlying error.
        source: core::num::ParseIntError,
    },
    /// Failed to parse a floating point number.
    ParseFloat {
        /// The underlying error.
        source: core::num::ParseFloatError,
    },
    /// Failed to parse a time.
    ParseTime {
        /// The underlying error.
        source: crate::timing::ParseError,
    },
    /// Failed to parse a date.
    ParseDate,
    /// Parsed comparison has an invalid name.
    InvalidComparisonName {
        /// The underlying error.
        source: ComparisonError,
    },
    /// Failed to parse a boolean.
    ParseBool,
}

impl From<XmlError> for Error {
    fn from(source: XmlError) -> Self {
        Self::Xml { source }
    }
}

impl From<core::num::ParseIntError> for Error {
    fn from(source: core::num::ParseIntError) -> Self {
        Self::ParseInt { source }
    }
}

impl From<core::num::ParseFloatError> for Error {
    fn from(source: core::num::ParseFloatError) -> Self {
        Self::ParseFloat { source }
    }
}

impl From<crate::timing::ParseError> for Error {
    fn from(source: crate::timing::ParseError) -> Self {
        Self::ParseTime { source }
    }
}

impl From<ComparisonError> for Error {
    fn from(source: ComparisonError) -> Self {
        Self::InvalidComparisonName { source }
    }
}

/// The Result type for the LiveSplit Parser.
pub type Result<T> = core::result::Result<T, Error>;

// FIXME: Generalized Type Ascription (GTA 6)
#[inline]
const fn type_hint<T>(v: Result<T>) -> Result<T> {
    v
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
struct Version(u32, u32, u32, u32);

fn parse_version(version: &str) -> Result<Version> {
    let splits = version.split('.');
    let mut v = [1, 0, 0, 0];
    for (d, s) in v.iter_mut().zip(splits) {
        *d = s.parse()?;
    }
    Ok(Version(v[0], v[1], v[2], v[3]))
}

fn parse_date_time(text: &str) -> Result<DateTime> {
    catch! {
        let (month, rem) = text.split_once('/')?;
        let (day, rem) = rem.split_once('/')?;
        let (year, rem) = rem.split_once(' ')?;
        let (hour, rem) = rem.split_once(':')?;
        let (minute, second) = rem.split_once(':')?;
        PrimitiveDateTime::new(
            Date::from_calendar_date(
                year.parse().ok()?,
                month
                    .parse::<u8>()
                    .ok()?
                    .try_into()
                    .ok()?,
                day.parse().ok()?,
            )
            .ok()?,
            time::Time::from_hms(
                hour.parse().ok()?,
                minute.parse().ok()?,
                second.parse().ok()?,
            )
            .ok()?,
        )
        .assume_utc()
    }
    .ok_or(Error::ParseDate)
}

fn image<F>(reader: &mut Reader<'_>, image_buf: &mut Vec<MaybeUninit<u8>>, f: F) -> Result<()>
where
    F: FnOnce(&[u8]),
{
    text_as_escaped_string_err(reader, |text| {
        if text.len() >= 216 {
            let src = &text.as_bytes()[212..];

            image_buf.resize(
                base64_simd::STANDARD.estimated_decoded_length(src.len()),
                MaybeUninit::uninit(),
            );

            if let Ok(decoded) =
                base64_simd::STANDARD.decode(src, base64_simd::Out::from_uninit_slice(image_buf))
            {
                f(&decoded[2..decoded.len() - 1]);
                return Ok(());
            }
        }
        f(&[]);
        Ok(())
    })
}

fn time_span<F>(reader: &mut Reader<'_>, f: F) -> Result<()>
where
    F: FnOnce(TimeSpan),
{
    text_as_escaped_string_err(reader, |text| {
        let time_span = || -> Result<TimeSpan> {
            if let Some((before_dot, after_dot)) = text.split_once('.') {
                if after_dot.contains(':') {
                    let days = TimeSpan::from_days(before_dot.parse()?);
                    let time = after_dot.parse()?;
                    return Ok(days + time);
                }
            }
            text.parse().map_err(Into::into)
        }()?;
        f(time_span);
        Ok(())
    })
}

fn time_span_opt<F>(reader: &mut Reader<'_>, f: F) -> Result<()>
where
    F: FnOnce(Option<TimeSpan>),
{
    text_as_escaped_string_err(reader, |text| {
        let time_span = || -> Result<Option<TimeSpan>> {
            if text.is_empty() {
                return Ok(None);
            }
            if let Some((before_dot, after_dot)) = text.split_once('.') {
                if after_dot.contains(':') {
                    let days = TimeSpan::from_days(before_dot.parse()?);
                    let time = after_dot.parse()?;
                    return Ok(Some(days + time));
                }
            }
            Ok(Some(text.parse()?))
        }()?;
        f(time_span);
        Ok(())
    })
}

fn time<F>(reader: &mut Reader<'_>, f: F) -> Result<()>
where
    F: FnOnce(Time),
{
    let mut time = Time::new();

    parse_children(reader, |reader, tag, _| {
        if tag.name() == "RealTime" {
            time_span_opt(reader, |t| time.real_time = t)
        } else if tag.name() == "GameTime" {
            time_span_opt(reader, |t| time.game_time = t)
        } else {
            end_tag(reader)
        }
    })?;

    f(time);

    Ok(())
}

fn time_old<F>(reader: &mut Reader<'_>, f: F) -> Result<()>
where
    F: FnOnce(Time),
{
    time_span_opt(reader, |t| f(Time::new().with_real_time(t)))
}

fn parse_bool(value: &str) -> Result<bool> {
    match value {
        "True" => Ok(true),
        "False" => Ok(false),
        _ => Err(Error::ParseBool),
    }
}

fn parse_metadata(
    version: Version,
    reader: &mut Reader<'_>,
    metadata: &mut RunMetadata,
) -> Result<()> {
    if version >= Version(1, 6, 0, 0) {
        parse_children(reader, |reader, tag, attributes| match tag.name() {
            "Run" => {
                type_hint(attribute(attributes, "id", |t| metadata.set_run_id(t)))?;
                end_tag(reader)
            }
            "Platform" => {
                type_hint(attribute_escaped_err(attributes, "usesEmulator", |t| {
                    metadata.set_emulator_usage(parse_bool(t)?);
                    Ok(())
                }))?;
                text(reader, |t| metadata.set_platform_name(t))
            }
            "Region" => text(reader, |t| metadata.set_region_name(t)),
            "Variables" | "SpeedrunComVariables" => {
                parse_children(reader, |reader, _, attributes| {
                    let mut name = Cow::Borrowed("");
                    let mut value = Cow::Borrowed("");
                    type_hint(attribute(attributes, "name", |t| name = t))?;
                    type_hint(text(reader, |t| value = t))?;
                    metadata.set_speedrun_com_variable(name, value);
                    Ok(())
                })
            }
            "CustomVariables" => parse_children(reader, |reader, _, attributes| {
                let mut name = Cow::Borrowed("");
                type_hint(attribute(attributes, "name", |t| name = t))?;
                let var = metadata.custom_variable_mut(name).permanent();
                type_hint(text(reader, |t| var.set_value(t)))?;
                Ok(())
            }),
            _ => end_tag(reader),
        })
    } else {
        end_tag(reader)
    }
}

fn parse_segment(
    version: Version,
    reader: &mut Reader<'_>,
    image_buf: &mut Vec<MaybeUninit<u8>>,
    run: &mut Run,
) -> Result<Segment> {
    let mut segment = Segment::new("");

    parse_children(reader, |reader, tag, _| match tag.name() {
        "Name" => text(reader, |t| segment.set_name(t)),
        "Icon" => image(reader, image_buf, |i| segment.set_icon(i)),
        "SplitTimes" => {
            if version >= Version(1, 3, 0, 0) {
                parse_children(reader, |reader, tag, attributes| {
                    if tag.name() == "SplitTime" {
                        let mut comparison = Cow::Borrowed("");
                        type_hint(attribute(attributes, "name", |t| comparison = t))?;
                        if version >= Version(1, 4, 1, 0) {
                            time(reader, |t| *segment.comparison_mut(&comparison) = t)?;
                        } else {
                            time_old(reader, |t| *segment.comparison_mut(&comparison) = t)?;
                        }
                        if let Err(ComparisonError::NameStartsWithRace) =
                            run.add_custom_comparison(comparison)
                        {
                            return Err(ComparisonError::NameStartsWithRace.into());
                        }
                        Ok(())
                    } else {
                        end_tag(reader)
                    }
                })
            } else {
                end_tag(reader)
            }
        }
        "PersonalBestSplitTime" => {
            if version < Version(1, 3, 0, 0) {
                time_old(reader, |t| segment.set_personal_best_split_time(t))
            } else {
                end_tag(reader)
            }
        }
        "BestSegmentTime" => {
            if version >= Version(1, 4, 1, 0) {
                time(reader, |t| segment.set_best_segment_time(t))
            } else {
                time_old(reader, |t| segment.set_best_segment_time(t))
            }
        }
        "SegmentHistory" => parse_children(reader, |reader, _, attributes| {
            let mut index = 0;
            type_hint(attribute_escaped_err(attributes, "id", |t| {
                index = t.parse()?;
                Ok(())
            }))?;
            if version >= Version(1, 4, 1, 0) {
                time(reader, |t| segment.segment_history_mut().insert(index, t))
            } else {
                time_old(reader, |t| segment.segment_history_mut().insert(index, t))
            }
        }),
        _ => end_tag(reader),
    })?;

    Ok(segment)
}

fn parse_run_history(version: Version, reader: &mut Reader<'_>, run: &mut Run) -> Result<()> {
    if version >= Version(1, 5, 0, 0) {
        end_tag(reader)
    } else if version >= Version(1, 4, 1, 0) {
        parse_children(reader, |reader, _, attributes| {
            let mut index = 0;
            type_hint(attribute_escaped_err(attributes, "id", |t| {
                index = t.parse()?;
                Ok(())
            }))?;
            time(reader, |time| {
                run.add_attempt_with_index(time, index, None, None, None)
            })
        })
    } else {
        parse_children(reader, |reader, _, attributes| {
            let mut index = 0;
            type_hint(attribute_escaped_err(attributes, "id", |t| {
                index = t.parse()?;
                Ok(())
            }))?;
            time_old(reader, |time| {
                run.add_attempt_with_index(time, index, None, None, None)
            })
        })
    }
}

fn parse_attempt_history(version: Version, reader: &mut Reader<'_>, run: &mut Run) -> Result<()> {
    if version >= Version(1, 5, 0, 0) {
        parse_children(reader, |reader, _, attributes| {
            let mut time = Time::new();
            let mut pause_time = None;
            let mut index = None;
            let (mut started, mut started_synced) = (None, false);
            let (mut ended, mut ended_synced) = (None, false);

            type_hint(parse_attributes(attributes, |k, v| {
                match k {
                    "id" => index = Some(v.escaped().parse()?),
                    "started" => started = Some(parse_date_time(v.escaped())?),
                    "isStartedSynced" => started_synced = parse_bool(v.escaped())?,
                    "ended" => ended = Some(parse_date_time(v.escaped())?),
                    "isEndedSynced" => ended_synced = parse_bool(v.escaped())?,
                    _ => {}
                }
                Ok(true)
            }))?;

            let index = index.ok_or(Error::Xml {
                source: XmlError::AttributeNotFound,
            })?;

            parse_children(reader, |reader, tag, _| match tag.name() {
                "RealTime" => time_span_opt(reader, |t| time.real_time = t),
                "GameTime" => time_span_opt(reader, |t| time.game_time = t),
                "PauseTime" => time_span_opt(reader, |t| pause_time = t),
                _ => end_tag(reader),
            })?;

            let started = started.map(|t| AtomicDateTime::new(t, started_synced));
            let ended = if version <= Version(1, 7, 0, 0)
                && catch! { ended? < started?.time }.unwrap_or(false)
            {
                None
            } else {
                ended.map(|t| AtomicDateTime::new(t, ended_synced))
            };

            run.add_attempt_with_index(time, index, started, ended, pause_time);

            Ok(())
        })
    } else {
        end_tag(reader)
    }
}

/// Attempts to parse a LiveSplit splits file.
pub fn parse(source: &str) -> Result<Run> {
    let mut reader = Reader::new(source);

    let mut image_buf = Vec::new();

    let mut run = Run::new();

    let mut required_flags = 0u8;

    parse_base(&mut reader, "Run", |reader, attributes| {
        let mut version = Version(1, 0, 0, 0);
        type_hint(optional_attribute_escaped_err(attributes, "version", |t| {
            version = parse_version(t)?;
            Ok(())
        }))?;

        parse_children(reader, |reader, tag, _| match tag.name() {
            "GameIcon" => {
                required_flags |= 1;
                image(reader, &mut image_buf, |i| run.set_game_icon(i))
            }
            "GameName" => {
                required_flags |= 1 << 1;
                text(reader, |t| run.set_game_name(t))
            }
            "CategoryName" => {
                required_flags |= 1 << 2;
                text(reader, |t| run.set_category_name(t))
            }
            "Offset" => {
                required_flags |= 1 << 3;
                time_span(reader, |t| run.set_offset(t))
            }
            "AttemptCount" => {
                required_flags |= 1 << 4;
                text_parsed(reader, |t| run.set_attempt_count(t))
            }
            "AttemptHistory" => parse_attempt_history(version, reader, &mut run),
            "RunHistory" => parse_run_history(version, reader, &mut run),
            "Metadata" => parse_metadata(version, reader, run.metadata_mut()),
            "Segments" => {
                required_flags |= 1 << 5;
                parse_children(reader, |reader, tag, _| {
                    if tag.name() == "Segment" {
                        let segment = parse_segment(version, reader, &mut image_buf, &mut run)?;
                        run.push_segment(segment);
                        Ok(())
                    } else {
                        end_tag(reader)
                    }
                })
            }
            "AutoSplitterSettings" => {
                let settings = run.auto_splitter_settings_mut();
                reencode_children(reader, settings).map_err(Into::into)
            }
            _ => end_tag(reader),
        })
    })?;

    if required_flags != (1 << 6) - 1 {
        return Err(Error::Xml {
            source: XmlError::ElementNotFound,
        });
    }

    Ok(run)
}
