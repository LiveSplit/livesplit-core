//! Provides the parser for LiveSplit splits files.

use super::super::run::ComparisonError;
use super::xml_util::{
    attribute, attribute_err, end_tag, optional_attribute_err, parse_attributes, parse_base,
    parse_children, reencode_children, text, text_as_bytes_err, text_err, text_parsed,
};
use chrono::{DateTime, TimeZone, Utc};
use quick_xml::Reader;
use std::io::BufRead;
use std::path::PathBuf;
use std::str;
use {base64, AtomicDateTime, Run, RunMetadata, Segment, Time, TimeSpan};

pub use super::xml_util::{Error, Result};

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
struct Version(u32, u32, u32, u32);

fn parse_version<S: AsRef<str>>(version: S) -> Result<Version> {
    let splits = version.as_ref().split('.');
    let mut v = [1, 0, 0, 0];
    for (d, s) in v.iter_mut().zip(splits) {
        *d = s.parse()?;
    }
    Ok(Version(v[0], v[1], v[2], v[3]))
}

fn parse_date_time<S: AsRef<str>>(text: S) -> Result<DateTime<Utc>> {
    Utc.datetime_from_str(text.as_ref(), "%m/%d/%Y %T")
        .map_err(Into::into)
}

fn image<R, F>(
    reader: &mut Reader<R>,
    result: &mut Vec<u8>,
    image_buf: &mut Vec<u8>,
    f: F,
) -> Result<()>
where
    R: BufRead,
    F: FnOnce(&[u8]),
{
    text_as_bytes_err(reader, result, |text| {
        if text.len() >= 216 {
            image_buf.clear();
            if base64::decode_config_buf(&text[212..], base64::STANDARD, image_buf).is_ok() {
                f(&image_buf[2..image_buf.len() - 1]);
                return Ok(());
            }
        }
        f(&[]);
        Ok(())
    })
}

fn time_span<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(TimeSpan),
{
    text_err(reader, buf, |text| {
        let time_span = || -> Result<TimeSpan> {
            if let (Some(dot_index), Some(colon_index)) = (text.find('.'), text.find(':')) {
                if dot_index < colon_index {
                    let days = TimeSpan::from_days(text[..dot_index].parse()?);
                    let time = text[dot_index + 1..].parse()?;
                    return Ok(days + time);
                }
            }
            text.parse().map_err(Into::into)
        }()?;
        f(time_span);
        Ok(())
    })
}

fn time_span_opt<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(Option<TimeSpan>),
{
    text_err(reader, buf, |text| {
        let time_span = || -> Result<Option<TimeSpan>> {
            if text.is_empty() {
                return Ok(None);
            }
            if let (Some(dot_index), Some(colon_index)) = (text.find('.'), text.find(':')) {
                if dot_index < colon_index {
                    let days = TimeSpan::from_days(text[..dot_index].parse()?);
                    let time = text[dot_index + 1..].parse()?;
                    return Ok(Some(days + time));
                }
            }
            Ok(Some(text.parse()?))
        }()?;
        f(time_span);
        Ok(())
    })
}

fn time<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(Time),
{
    let mut time = Time::new();

    parse_children(reader, buf, |reader, tag| {
        if tag.name() == b"RealTime" {
            time_span_opt(reader, tag.into_buf(), |t| {
                time.real_time = t;
            })
        } else if tag.name() == b"GameTime" {
            time_span_opt(reader, tag.into_buf(), |t| {
                time.game_time = t;
            })
        } else {
            end_tag(reader, tag.into_buf())
        }
    })?;

    f(time);

    Ok(())
}

fn time_old<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(Time),
{
    time_span_opt(reader, buf, |t| f(Time::new().with_real_time(t)))
}

fn parse_bool<S: AsRef<str>>(text: S) -> Result<bool> {
    match text.as_ref() {
        "True" => Ok(true),
        "False" => Ok(false),
        _ => Err(Error::Bool),
    }
}

fn parse_metadata<R: BufRead>(
    version: Version,
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    metadata: &mut RunMetadata,
) -> Result<()> {
    if version >= Version(1, 6, 0, 0) {
        parse_children(reader, buf, |reader, tag| {
            if tag.name() == b"Run" {
                attribute(&tag, b"id", |t| metadata.set_run_id(t))?;
                end_tag(reader, tag.into_buf())
            } else if tag.name() == b"Platform" {
                attribute_err(&tag, b"usesEmulator", |t| {
                    metadata.set_emulator_usage(parse_bool(t)?);
                    Ok(())
                })?;
                text(reader, tag.into_buf(), |t| metadata.set_platform_name(t))
            } else if tag.name() == b"Region" {
                text(reader, tag.into_buf(), |t| metadata.set_region_name(t))
            } else if tag.name() == b"Variables" {
                parse_children(reader, tag.into_buf(), |reader, tag| {
                    let mut name = String::new();
                    let mut value = String::new();
                    attribute(&tag, b"name", |t| {
                        name = t.into_owned();
                    })?;
                    text(reader, tag.into_buf(), |t| {
                        value = t.into_owned();
                    })?;
                    metadata.set_variable(name, value);
                    Ok(())
                })
            } else {
                end_tag(reader, tag.into_buf())
            }
        })
    } else {
        end_tag(reader, buf)
    }
}

fn parse_segment<R: BufRead>(
    version: Version,
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    buf2: &mut Vec<u8>,
    run: &mut Run,
) -> Result<Segment> {
    let mut segment = Segment::new("");

    parse_children(reader, buf, |reader, tag| {
        if tag.name() == b"Name" {
            text(reader, tag.into_buf(), |t| segment.set_name(t))
        } else if tag.name() == b"Icon" {
            image(reader, tag.into_buf(), buf2, |i| segment.set_icon(i))
        } else if tag.name() == b"SplitTimes" {
            if version >= Version(1, 3, 0, 0) {
                parse_children(reader, tag.into_buf(), |reader, tag| {
                    if tag.name() == b"SplitTime" {
                        let mut comparison = String::new();
                        attribute(&tag, b"name", |t| {
                            comparison = t.into_owned();
                        })?;
                        if version >= Version(1, 4, 1, 0) {
                            time(reader, tag.into_buf(), |t| {
                                *segment.comparison_mut(&comparison) = t;
                            })?;
                        } else {
                            time_old(reader, tag.into_buf(), |t| {
                                *segment.comparison_mut(&comparison) = t;
                            })?;
                        }
                        if let Err(ComparisonError::NameStartsWithRace) =
                            run.add_custom_comparison(comparison)
                        {
                            return Err(ComparisonError::NameStartsWithRace.into());
                        }
                        Ok(())
                    } else {
                        end_tag(reader, tag.into_buf())
                    }
                })
            } else {
                end_tag(reader, tag.into_buf())
            }
        } else if tag.name() == b"PersonalBestSplitTime" {
            if version < Version(1, 3, 0, 0) {
                time_old(reader, tag.into_buf(), |t| {
                    segment.set_personal_best_split_time(t);
                })
            } else {
                end_tag(reader, tag.into_buf())
            }
        } else if tag.name() == b"BestSegmentTime" {
            if version >= Version(1, 4, 1, 0) {
                time(reader, tag.into_buf(), |t| {
                    segment.set_best_segment_time(t);
                })
            } else {
                time_old(reader, tag.into_buf(), |t| {
                    segment.set_best_segment_time(t);
                })
            }
        } else if tag.name() == b"SegmentHistory" {
            parse_children(reader, tag.into_buf(), |reader, tag| {
                let mut index = 0;
                attribute_err(&tag, b"id", |t| {
                    index = t.parse()?;
                    Ok(())
                })?;
                if version >= Version(1, 4, 1, 0) {
                    time(reader, tag.into_buf(), |t| {
                        segment.segment_history_mut().insert(index, t);
                    })
                } else {
                    time_old(reader, tag.into_buf(), |t| {
                        segment.segment_history_mut().insert(index, t);
                    })
                }
            })
        } else {
            end_tag(reader, tag.into_buf())
        }
    })?;

    Ok(segment)
}

fn parse_run_history<R: BufRead>(
    version: Version,
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    run: &mut Run,
) -> Result<()> {
    if version >= Version(1, 5, 0, 0) {
        end_tag(reader, buf)
    } else if version >= Version(1, 4, 1, 0) {
        parse_children(reader, buf, |reader, tag| {
            let mut index = 0;
            attribute_err(&tag, b"id", |t| {
                index = t.parse()?;
                Ok(())
            })?;
            time(reader, tag.into_buf(), |time| {
                run.add_attempt_with_index(time, index, None, None, None);
            })
        })
    } else {
        parse_children(reader, buf, |reader, tag| {
            let mut index = 0;
            attribute_err(&tag, b"id", |t| {
                index = t.parse()?;
                Ok(())
            })?;
            time_old(reader, tag.into_buf(), |time| {
                run.add_attempt_with_index(time, index, None, None, None);
            })
        })
    }
}

fn parse_attempt_history<R: BufRead>(
    version: Version,
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    run: &mut Run,
) -> Result<()> {
    if version >= Version(1, 5, 0, 0) {
        parse_children(reader, buf, |reader, tag| {
            let mut time = Time::new();
            let mut pause_time = None;
            let mut index = None;
            let (mut started, mut started_synced) = (None, false);
            let (mut ended, mut ended_synced) = (None, false);

            parse_attributes(&tag, |k, v| {
                if k == b"id" {
                    index = Some(v.get()?.parse()?);
                } else if k == b"started" {
                    started = Some(parse_date_time(v.get()?)?);
                } else if k == b"isStartedSynced" {
                    started_synced = parse_bool(v.get()?)?;
                } else if k == b"ended" {
                    ended = Some(parse_date_time(v.get()?)?);
                } else if k == b"isEndedSynced" {
                    ended_synced = parse_bool(v.get()?)?;
                }
                Ok(true)
            })?;

            let index = index.ok_or(Error::AttributeNotFound)?;

            parse_children(reader, tag.into_buf(), |reader, tag| {
                if tag.name() == b"RealTime" {
                    time_span_opt(reader, tag.into_buf(), |t| {
                        time.real_time = t;
                    })
                } else if tag.name() == b"GameTime" {
                    time_span_opt(reader, tag.into_buf(), |t| {
                        time.game_time = t;
                    })
                } else if tag.name() == b"PauseTime" {
                    time_span_opt(reader, tag.into_buf(), |t| {
                        pause_time = t;
                    })
                } else {
                    end_tag(reader, tag.into_buf())
                }
            })?;

            let started = started.map(|t| AtomicDateTime::new(t, started_synced));
            let ended = ended.map(|t| AtomicDateTime::new(t, ended_synced));

            run.add_attempt_with_index(time, index, started, ended, pause_time);

            Ok(())
        })
    } else {
        end_tag(reader, buf)
    }
}

/// Attempts to parse a LiveSplit splits file. In addition to the source to
/// parse, you can provide a path to the splits file, which helps saving the
/// splits file again later.
pub fn parse<R: BufRead>(source: R, path: Option<PathBuf>) -> Result<Run> {
    let reader = &mut Reader::from_reader(source);
    reader.expand_empty_elements(true);
    reader.trim_text(true);

    let mut buf = Vec::with_capacity(4096);
    let mut buf2 = Vec::with_capacity(4096);

    let mut run = Run::new();

    let mut required_flags = 0u8;

    parse_base(reader, &mut buf, b"Run", |reader, tag| {
        let mut version = Version(1, 0, 0, 0);
        optional_attribute_err(&tag, b"version", |t| {
            version = parse_version(t)?;
            Ok(())
        })?;

        parse_children(reader, tag.into_buf(), |reader, tag| {
            if tag.name() == b"GameIcon" {
                required_flags |= 1 << 0;
                image(reader, tag.into_buf(), &mut buf2, |i| run.set_game_icon(i))
            } else if tag.name() == b"GameName" {
                required_flags |= 1 << 1;
                text(reader, tag.into_buf(), |t| run.set_game_name(t))
            } else if tag.name() == b"CategoryName" {
                required_flags |= 1 << 2;
                text(reader, tag.into_buf(), |t| run.set_category_name(t))
            } else if tag.name() == b"Offset" {
                required_flags |= 1 << 3;
                time_span(reader, tag.into_buf(), |t| run.set_offset(t))
            } else if tag.name() == b"AttemptCount" {
                required_flags |= 1 << 4;
                text_parsed(reader, tag.into_buf(), |t| run.set_attempt_count(t))
            } else if tag.name() == b"AttemptHistory" {
                parse_attempt_history(version, reader, tag.into_buf(), &mut run)
            } else if tag.name() == b"RunHistory" {
                parse_run_history(version, reader, tag.into_buf(), &mut run)
            } else if tag.name() == b"Metadata" {
                parse_metadata(version, reader, tag.into_buf(), run.metadata_mut())
            } else if tag.name() == b"Segments" {
                required_flags |= 1 << 5;
                parse_children(reader, tag.into_buf(), |reader, tag| {
                    if tag.name() == b"Segment" {
                        let segment =
                            parse_segment(version, reader, tag.into_buf(), &mut buf2, &mut run)?;
                        run.push_segment(segment);
                        Ok(())
                    } else {
                        end_tag(reader, tag.into_buf())
                    }
                })
            } else if tag.name() == b"AutoSplitterSettings" {
                let settings = run.auto_splitter_settings_mut();
                reencode_children(reader, tag.into_buf(), settings)
            } else {
                end_tag(reader, tag.into_buf())
            }
        })
    })?;

    if required_flags != (1 << 6) - 1 {
        return Err(Error::ElementNotFound);
    }

    run.set_path(path);

    Ok(run)
}
