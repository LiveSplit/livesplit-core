//! Provides the parser for LiveSplit splits files.

use crate::{
    platform::prelude::*,
    run::{AddComparisonError, LinkedLayout},
    settings::Image,
    util::{
        ascii_char::AsciiChar,
        xml::{
            helper::{
                attribute, attribute_escaped_err, end_tag, image, optional_attribute_escaped_err,
                parse_attributes, parse_base, parse_children, text, text_as_escaped_string_err,
                text_parsed, Error as XmlError,
            },
            Reader,
        },
    },
    AtomicDateTime, DateTime, Run, RunMetadata, Segment, Time, TimeSpan,
};
use alloc::borrow::Cow;
use core::fmt::{Display, Formatter};
use core::{mem::MaybeUninit, str};
use time::{Date, Duration, PrimitiveDateTime};
#[cfg(feature = "auto-splitting")]
use {
    crate::run::auto_splitter_settings::AutoSplitterSettings, crate::util::xml::Attributes,
    livesplit_auto_splitting::settings,
};

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
    /// Failed to parse a time that contains days.
    ParseExtendedTime,
    /// Failed to parse a date.
    ParseDate,
    /// Parsed comparison has an invalid name.
    InvalidComparisonName {
        /// The underlying error.
        source: AddComparisonError,
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

impl From<AddComparisonError> for Error {
    fn from(source: AddComparisonError) -> Self {
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

/// The version type for the LiveSplit parser
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct Version(pub u32, pub u32, pub u32, pub u32);

impl Default for Version {
    fn default() -> Self {
        Version(1, 0, 0, 0)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0, self.1, self.2, self.3)
    }
}

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

fn time_span<F>(reader: &mut Reader<'_>, f: F) -> Result<()>
where
    F: FnOnce(TimeSpan),
{
    text_as_escaped_string_err(reader, |text| {
        f(parse_time_span(text)?);
        Ok(())
    })
}

fn time_span_opt<F>(reader: &mut Reader<'_>, f: F) -> Result<()>
where
    F: FnOnce(Option<TimeSpan>),
{
    text_as_escaped_string_err(reader, |text| {
        f(if text.is_empty() {
            None
        } else {
            Some(parse_time_span(text)?)
        });
        Ok(())
    })
}

fn parse_time_span(text: &str) -> Result<TimeSpan> {
    if let Some((before_dot, after_dot)) = AsciiChar::DOT.split_once(text) {
        if AsciiChar::COLON.contains(after_dot) {
            const SECS_PER_DAY: i64 = 24 * 60 * 60;

            let days_secs = before_dot
                .parse::<i64>()
                .ok()
                .and_then(|s| s.checked_mul(SECS_PER_DAY))
                .ok_or(Error::ParseExtendedTime)?;

            let days: TimeSpan = Duration::seconds(days_secs).into();

            let time: TimeSpan = after_dot.parse()?;

            if time < TimeSpan::zero() {
                return Err(Error::ParseExtendedTime);
            }

            return Ok(if days < TimeSpan::zero() {
                days - time
            } else {
                days + time
            });
        }
    }
    text.parse().map_err(Into::into)
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
        "Icon" => image(reader, image_buf, |i| {
            segment.set_icon(Image::new(i.into(), Image::ICON))
        }),
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
                        if let Err(AddComparisonError::NameStartsWithRace) =
                            run.add_custom_comparison(comparison)
                        {
                            return Err(AddComparisonError::NameStartsWithRace.into());
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

fn parse_auto_splitter_settings(
    _version: Version,
    reader: &mut Reader<'_>,
    run: &mut Run,
) -> Result<()> {
    crate::util::xml::helper::reencode_children(reader, run.auto_splitter_settings_mut())
        .map_err(Into::<Error>::into)?;

    #[cfg(feature = "auto-splitting")]
    let mut reader = Reader::new(run.auto_splitter_settings());

    #[cfg(feature = "auto-splitting")]
    let mut any_parsed = false;
    #[cfg(feature = "auto-splitting")]
    let mut settings = AutoSplitterSettings::default();

    #[cfg(feature = "auto-splitting")]
    // The compiler seems to throw a warning that 'attributes' isn't used by default, it actually is though
    #[allow(unused_variables)]
    parse_children(&mut reader, |reader, tag, attributes| match tag.name() {
        "Version" => type_hint(text(reader, |t| {
            any_parsed = true;
            settings.set_version(parse_version(t.as_ref()).unwrap_or_default())
        })),
        "ScriptPath" => type_hint(text(reader, |t| {
            any_parsed = true;
            settings.set_script_path(t.to_string())
        })),
        "CustomSettings" => {
            any_parsed = true;
            settings.set_custom_settings(parse_settings_map(reader));
            Ok(())
        }
        _ => Ok(()),
    })
    .ok();

    #[cfg(feature = "auto-splitting")]
    if any_parsed {
        run.parsed_auto_splitter_settings = Some(settings);
    }

    Ok(())
}

#[cfg(feature = "auto-splitting")]
fn parse_settings_map(reader: &mut Reader<'_>) -> settings::Map {
    let mut settings_map = settings::Map::new();

    parse_children(reader, |reader, _tag, attributes| {
        if let (Some(id), Some(value)) = parse_settings_entry(reader, attributes) {
            settings_map.insert(id.into(), value);
        }
        Ok::<(), Error>(())
    })
    .ok();

    settings_map
}

#[cfg(feature = "auto-splitting")]
fn parse_settings_list(reader: &mut Reader<'_>) -> settings::List {
    let mut settings_list = settings::List::new();

    parse_children(reader, |reader, _tag, attributes| {
        if let (_, Some(value)) = parse_settings_entry(reader, attributes) {
            settings_list.push(value);
        }
        Ok::<(), Error>(())
    })
    .ok();

    settings_list
}

#[cfg(feature = "auto-splitting")]
fn parse_settings_entry(
    reader: &mut Reader<'_>,
    attributes: Attributes<'_>,
) -> (Option<String>, Option<settings::Value>) {
    let mut id = None;
    let mut setting_type = None;
    let mut string_value = None;
    type_hint(parse_attributes(attributes, |k, v| {
        match k {
            "id" => id = Some(v.unescape_str()),
            "type" => setting_type = Some(v.unescape_str()),
            "value" => string_value = Some(v.unescape_str()),
            _ => {}
        }
        Ok(true)
    }))
    .ok();
    let Some(setting_type) = setting_type else {
        return (id, None);
    };
    let value = match setting_type.as_str() {
        "bool" => {
            let mut b = bool::default();
            type_hint(text(reader, |t| {
                b = parse_bool(t.as_ref()).unwrap_or_default();
            }))
            .ok();
            Some(settings::Value::Bool(b))
        }
        "i64" => {
            let mut i = i64::default();
            type_hint(text(reader, |t| {
                i = t.as_ref().parse().unwrap_or_default();
            }))
            .ok();
            Some(settings::Value::I64(i))
        }
        "f64" => {
            let mut f = f64::default();
            type_hint(text(reader, |t| {
                f = t.as_ref().parse().unwrap_or_default();
            }))
            .ok();
            Some(settings::Value::F64(f))
        }
        "string" => {
            let mut s = String::default();
            type_hint(text(reader, |t| {
                s = t.to_string();
            }))
            .ok();
            Some(settings::Value::String(string_value.unwrap_or(s).into()))
        }
        "map" => Some(settings::Value::Map(parse_settings_map(reader))),
        "list" => Some(settings::Value::List(parse_settings_list(reader))),
        _ => None,
    };
    (id, value)
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
                image(reader, &mut image_buf, |i| {
                    run.set_game_icon(Image::new(i.into(), Image::ICON))
                })
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
            "AutoSplitterSettings" => parse_auto_splitter_settings(version, reader, &mut run),
            "LayoutPath" => text(reader, |t| {
                run.set_linked_layout(if t == "?default" {
                    Some(LinkedLayout::Default)
                } else if t.is_empty() {
                    None
                } else {
                    Some(LinkedLayout::Path(t.into_owned()))
                });
            }),
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

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "auto-splitting")]
    use livesplit_auto_splitting::settings;

    #[test]
    fn time_span_parsing() {
        assert_eq!(
            parse_time_span("1.23:34:56.789")
                .unwrap()
                .to_seconds_and_subsec_nanoseconds(),
            (171296, 789000000)
        );
        assert_eq!(
            parse_time_span("-1.23:34:56.789")
                .unwrap()
                .to_seconds_and_subsec_nanoseconds(),
            (-171296, -789000000)
        );
        parse_time_span("-1.-23:34:56.789").unwrap_err();
        parse_time_span("1.-23:34:56.789").unwrap_err();
        parse_time_span("-123.45.23:34:56.789").unwrap_err();
        parse_time_span("NaN.23:34:56.789").unwrap_err();
        parse_time_span("Inf.23:34:56.789").unwrap_err();
    }

    #[cfg(feature = "auto-splitting")]
    #[test]
    fn test_parse_settings() {
        assert_eq!(
            parse_settings_map(&mut Reader::new(
                r#"
                <Setting id="start" type="bool">True</Setting>
                <Setting id="split" type="bool">True</Setting>
                <Setting id="remove_loads" type="bool">True</Setting>
            "#
            )),
            {
                let mut m = settings::Map::new();
                m.insert("start".into(), settings::Value::Bool(true));
                m.insert("split".into(), settings::Value::Bool(true));
                m.insert("remove_loads".into(), settings::Value::Bool(true));
                m
            },
        );

        assert_eq!(
            parse_settings_list(&mut Reader::new(
                r#"<Setting type="string" value="KingsPass" />"#
            )),
            {
                let mut l = settings::List::new();
                l.push(settings::Value::String("KingsPass".into()));
                l
            },
        );

        assert_eq!(
            parse_settings_map(&mut Reader::new(
                r#"<Setting id="splits_0_item" type="string" value="KingsPass" />"#
            )),
            {
                let mut m = settings::Map::new();
                m.insert(
                    "splits_0_item".into(),
                    settings::Value::String("KingsPass".into()),
                );
                m
            },
        );

        assert_eq!(
            parse_settings_map(&mut Reader::new(
                r#"
                <Setting id="inner_map" type="map">
                    <Setting id="first" type="bool">True</Setting>
                    <Setting id="second" type="string" value="bar" />
                </Setting>
            "#
            )),
            {
                let mut map = settings::Map::new();

                let mut inner_map = settings::Map::new();
                inner_map.insert("first".into(), settings::Value::Bool(true));
                inner_map.insert("second".into(), settings::Value::String("bar".into()));

                map.insert("inner_map".into(), settings::Value::Map(inner_map));
                map
            },
        );

        assert_eq!(
            parse_settings_map(&mut Reader::new(
                r#"
                <Setting id="lolol" type="map">
                    <Setting id="first" type="bool">True</Setting>
                    <Setting id="second" type="string" value="bar"></Setting>
                    <Setting id="recursive" type="map">
                        <Setting id="first" type="bool">True</Setting>
                        <Setting id="second" type="string" value="bar"></Setting>
                    </Setting>
                </Setting>
            "#
            )),
            {
                let mut map = settings::Map::new();

                let mut inner_map = settings::Map::new();
                inner_map.insert("first".into(), settings::Value::Bool(true));
                inner_map.insert("second".into(), settings::Value::String("bar".into()));
                inner_map.insert("recursive".into(), settings::Value::Map(inner_map.clone()));

                map.insert("lolol".into(), settings::Value::Map(inner_map));
                map
            },
        );

        assert_eq!(
            parse_settings_map(&mut Reader::new(
                r#"
                <Setting id="lolol" type="map">
                    <Setting id="first" type="bool">True</Setting>
                    <Setting id="second" type="string" value="bar" />
                    <Setting id="recursive" type="map">
                        <Setting id="first" type="bool">True</Setting>
                        <Setting id="second" type="string" value="bar" />
                    </Setting>
                </Setting>
            "#
            )),
            {
                let mut map = settings::Map::new();

                let mut inner_map = settings::Map::new();
                inner_map.insert("first".into(), settings::Value::Bool(true));
                inner_map.insert("second".into(), settings::Value::String("bar".into()));
                inner_map.insert("recursive".into(), settings::Value::Map(inner_map.clone()));

                map.insert("lolol".into(), settings::Value::Map(inner_map));
                map
            },
        );

        assert_eq!(
            parse_settings_map(&mut Reader::new(
                r#"
                <Setting id="level32_bool" type="bool">True</Setting>
                <Setting id="other_setting" type="bool">True</Setting>
                <Setting id="level12_bool" type="bool">True</Setting>
                <Setting id="lolol" type="map">
                    <Setting id="first" type="bool">True</Setting>
                    <Setting id="second" type="string" value="bar" />
                    <Setting id="recursive" type="map">
                        <Setting id="first" type="bool">True</Setting>
                        <Setting id="second" type="string" value="bar" />
                    </Setting>
                </Setting>
                <Setting id="okok" type="string" value="hello, you seem to like true!" />
            "#
            )),
            {
                let mut map = settings::Map::new();
                map.insert("level32_bool".into(), settings::Value::Bool(true));
                map.insert("other_setting".into(), settings::Value::Bool(true));
                map.insert("level12_bool".into(), settings::Value::Bool(true));

                let mut inner_map = settings::Map::new();
                inner_map.insert("first".into(), settings::Value::Bool(true));
                inner_map.insert("second".into(), settings::Value::String("bar".into()));
                inner_map.insert("recursive".into(), settings::Value::Map(inner_map.clone()));

                map.insert("lolol".into(), settings::Value::Map(inner_map));
                map.insert(
                    "okok".into(),
                    settings::Value::String("hello, you seem to like true!".into()),
                );
                map
            },
        );
    }
}
