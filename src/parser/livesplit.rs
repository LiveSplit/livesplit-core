use std::io::{self, Read};
use std::result::Result as StdResult;
use std::num::{ParseIntError, ParseFloatError};
use std::path::PathBuf;
use base64;
use sxd_document::dom::Element;
use sxd_document::parser::{Error as XmlError, parse as parse_xml};
use chrono::{DateTime, UTC, TimeZone, ParseError as ChronoError};
use super::bom_consumer::BomConsumer;
use {Run, time_span, TimeSpan, Time, AtomicDateTime, Segment};
use comparison::personal_best;
use super::xml_util::{self, text};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Xml(err: (usize, Vec<XmlError>)) {
            from()
        }
        Io(err: io::Error) {
            from()
        }
        Int(err: ParseIntError) {
            from()
        }
        Float(err: ParseFloatError) {
            from()
        }
        Bool
        ElementNotFound
        AttributeNotFound
        Time(err: time_span::ParseError) {
            from()
        }
        Date(err: ChronoError) {
            from()
        }
    }
}

pub type Result<T> = StdResult<T, Error>;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
struct Version(u32, u32, u32, u32);

fn child<'d>(element: &Element<'d>, name: &str) -> Result<Element<'d>> {
    xml_util::child(element, name).ok_or(Error::ElementNotFound)
}

fn attribute<'d>(element: &Element<'d>, attribute: &str) -> Result<&'d str> {
    xml_util::attribute(element, attribute).ok_or(Error::AttributeNotFound)
}

fn time_span(element: &Element, buf: &mut String) -> Result<TimeSpan> {
    let text = text(element, buf);
    if let (Some(dot_index), Some(colon_index)) = (text.find('.'), text.find(':')) {
        if dot_index < colon_index {
            let days = TimeSpan::from_days(text[..dot_index].parse()?);
            let time = text[dot_index + 1..].parse()?;
            return Ok(days + time);
        }
    }
    text.parse().map_err(Into::into)
}

fn time_span_opt(element: &Element, buf: &mut String) -> Result<Option<TimeSpan>> {
    let text = text(element, buf);
    if text.trim().is_empty() {
        Ok(None)
    } else {
        if let (Some(dot_index), Some(colon_index)) = (text.find('.'), text.find(':')) {
            if dot_index < colon_index {
                let days = TimeSpan::from_days(text[..dot_index].parse()?);
                let time = text[dot_index + 1..].parse()?;
                return Ok(Some(days + time));
            }
        }
        Ok(Some(text.parse()?))
    }
}

fn time(element: &Element, buf: &mut String) -> Result<Time> {
    let mut time = Time::new();

    if let Ok(element) = child(element, "RealTime") {
        time = time.with_real_time(time_span_opt(&element, buf)?);
    }

    if let Ok(element) = child(element, "GameTime") {
        time = time.with_game_time(time_span_opt(&element, buf)?);
    }

    Ok(time)
}

fn time_old(element: &Element, buf: &mut String) -> Result<Time> {
    Ok(Time::new().with_real_time(time_span_opt(element, buf)?))
}

fn parse_bool<S: AsRef<str>>(text: S) -> Result<bool> {
    match text.as_ref() {
        "True" => Ok(true),
        "False" => Ok(false),
        _ => Err(Error::Bool),
    }
}

fn image<'b>(element: &Element, buf: &'b mut Vec<u8>, str_buf: &mut String) -> &'b [u8] {
    buf.clear();
    let text = text(element, str_buf);
    if text.len() >= 216 {
        if let Ok(data) = base64::decode(&text[212..]) {
            buf.extend_from_slice(&data[2..data.len() - 1]);
        }
    }
    buf
}

fn parse_version<S: AsRef<str>>(version: S) -> Result<Version> {
    let splits = version.as_ref().split('.');
    let mut v = [1, 0, 0, 0];
    for (d, s) in v.iter_mut().zip(splits) {
        *d = s.parse()?;
    }
    Ok(Version(v[0], v[1], v[2], v[3]))
}

fn parse_date_time<S: AsRef<str>>(text: S) -> Result<DateTime<UTC>> {
    UTC.datetime_from_str(text.as_ref(), "%m/%d/%Y %T")
        .map_err(Into::into)
}

fn parse_attempt_history(version: Version,
                         element: &Element,
                         run: &mut Run,
                         buf: &mut String)
                         -> Result<()> {
    if version >= Version(1, 5, 0, 0) {
        let attempt_history = child(element, "AttemptHistory")?;
        for attempt in attempt_history
                .children()
                .into_iter()
                .filter_map(|c| c.element()) {
            let time = time(&attempt, buf)?;
            let index = attribute(&attempt, "id")?.parse()?;

            let (mut started, mut started_synced) = (None, false);
            let (mut ended, mut ended_synced) = (None, false);

            if let Ok(attr) = attribute(&attempt, "started") {
                started = Some(parse_date_time(attr)?);
                if let Ok(synced) = attribute(&attempt, "isStartedSynced") {
                    started_synced = parse_bool(synced)?;
                }
            }

            if let Ok(attr) = attribute(&attempt, "ended") {
                ended = Some(parse_date_time(attr)?);
                if let Ok(synced) = attribute(&attempt, "isEndedSynced") {
                    ended_synced = parse_bool(synced)?;
                }
            }

            let mut pause_time = None;
            if version >= Version(1, 7, 0, 0) {
                if let Ok(element) = child(&attempt, "PauseTime") {
                    pause_time = time_span_opt(&element, buf)?;
                }
            }

            let started = started.map(|t| AtomicDateTime::new(t, started_synced));
            let ended = ended.map(|t| AtomicDateTime::new(t, ended_synced));

            run.add_attempt_with_index(time, index, started, ended, pause_time);
        }
    } else if version >= Version(1, 4, 1, 0) {
        let run_history = child(element, "RunHistory")?;
        for attempt in run_history
                .children()
                .into_iter()
                .filter_map(|c| c.element()) {
            let time = time(&attempt, buf)?;
            let index = attribute(&attempt, "id")?.parse()?;

            run.add_attempt_with_index(time, index, None, None, None);
        }
    } else {
        let run_history = child(element, "RunHistory")?;
        for attempt in run_history
                .children()
                .into_iter()
                .filter_map(|c| c.element()) {
            let time = time_old(&attempt, buf)?;
            let index = attribute(&attempt, "id")?.parse()?;

            run.add_attempt_with_index(time, index, None, None, None);
        }
    }

    Ok(())
}

pub fn parse<R: Read>(source: R, path: Option<PathBuf>) -> Result<Run> {
    let icon_buf = &mut Vec::new();
    let buf = &mut String::new();
    BomConsumer::from(source).read_to_string(buf)?;
    let package = parse_xml(buf)?;

    let node = package
        .as_document()
        .root()
        .children()
        .into_iter()
        .filter_map(|c| c.element())
        .next()
        .unwrap();

    let mut run = Run::new();

    let version = if let Ok(version) = attribute(&node, "version") {
        parse_version(version)?
    } else {
        Version(1, 0, 0, 0)
    };

    if version >= Version(1, 6, 0, 0) {
        let metadata = run.metadata_mut();
        let node = child(&node, "Metadata")?;

        metadata.set_run_id(attribute(&child(&node, "Run")?, "id")?);
        let platform = child(&node, "Platform")?;
        metadata.set_platform_name(text(&platform, buf));
        metadata.set_emulator_usage(parse_bool(attribute(&platform, "usesEmulator")?)?);
        metadata.set_region_name(text(&child(&node, "Region")?, buf));

        let variables = child(&node, "Variables")?;
        for variable in variables.children().into_iter().filter_map(|c| c.element()) {
            let name = attribute(&variable, "name")?;
            let value = text(&variable, buf);
            metadata.add_variable(name, value);
        }
    }

    run.set_game_icon(image(&child(&node, "GameIcon")?, icon_buf, buf));
    run.set_game_name(text(&child(&node, "GameName")?, buf));
    run.set_category_name(text(&child(&node, "CategoryName")?, buf));
    run.set_offset(time_span(&child(&node, "Offset")?, buf)?);
    run.set_attempt_count(text(&child(&node, "AttemptCount")?, buf).parse()?);

    parse_attempt_history(version, &node, &mut run, buf)?;

    let segments = child(&node, "Segments")?;

    for node in segments.children().into_iter().filter_map(|c| c.element()) {
        let mut segment = Segment::new(text(&child(&node, "Name")?, buf));
        segment.set_icon(image(&child(&node, "Icon")?, icon_buf, buf));

        if version >= Version(1, 3, 0, 0) {
            let node = child(&node, "SplitTimes")?;
            for node in node.children().into_iter().filter_map(|c| c.element()) {
                let comparison_name = attribute(&node, "name")?;
                if !node.children().is_empty() {
                    *segment.comparison_mut(comparison_name) = if version >= Version(1, 4, 1, 0) {
                        time(&node, buf)?
                    } else {
                        time_old(&node, buf)?
                    };
                }
                run.add_custom_comparison(comparison_name);
            }
        } else {
            let node = child(&node, "PersonalBestSplitTime")?;
            if !node.children().is_empty() {
                *segment.comparison_mut(personal_best::NAME) = time_old(&node, buf)?;
            }
        }

        let gold_split = child(&node, "BestSegmentTime")?;
        if !gold_split.children().is_empty() {
            segment.set_best_segment_time(if version >= Version(1, 4, 1, 0) {
                                              time(&gold_split, buf)?
                                          } else {
                                              time_old(&gold_split, buf)?
                                          });
        }

        let history = child(&node, "SegmentHistory")?;
        for node in history.children().into_iter().filter_map(|c| c.element()) {
            let index = attribute(&node, "id")?.parse()?;
            let time = if version >= Version(1, 4, 1, 0) {
                time(&node, buf)?
            } else {
                time_old(&node, buf)?
            };

            segment.segment_history_mut().insert(index, time);
        }

        run.push_segment(segment);
    }

    if version >= Version(1, 4, 2, 0) {
        let _settings = child(&node, "AutoSplitterSettings")?;
        // TODO Store this somehow
    }

    run.set_path(path);

    Ok(run)
}
