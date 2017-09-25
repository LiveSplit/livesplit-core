use std::io::Write;
use std::result::Result as StdResult;
use std::fmt::Display;
use std::borrow::Cow;
use std::mem::replace;
use {Image, Run, Time, TimeSpan, base64};
use time::formatter::{Complete, TimeFormatter};
use chrono::{DateTime, Utc};
use byteorder::{WriteBytesExt, LE};
use quick_xml::Writer;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::errors::Error as XmlError;

static LSS_IMAGE_HEADER: &[u8; 156] = include_bytes!("lss_image_header.bin");

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Xml(err: XmlError) {
            from()
        }
    }
}

pub type Result<T> = StdResult<T, Error>;

fn new_tag(name: &[u8]) -> BytesStart {
    BytesStart::borrowed(name, name.len())
}

fn write_start<W: Write>(writer: &mut Writer<W>, tag: BytesStart) -> Result<()> {
    writer.write_event(Event::Start(tag))?;
    Ok(())
}

fn write_end<W: Write>(writer: &mut Writer<W>, tag: &[u8]) -> Result<()> {
    writer.write_event(Event::End(BytesEnd::borrowed(tag)))?;
    Ok(())
}

fn split_tag<'a>(tag: &'a BytesStart<'a>) -> (BytesStart<'a>, BytesEnd<'a>) {
    (
        BytesStart::borrowed(&tag, tag.name().len()),
        BytesEnd::borrowed(tag.name()),
    )
}

fn bool(value: bool) -> &'static [u8] {
    if value {
        b"True"
    } else {
        b"False"
    }
}

fn scoped<W, F>(writer: &mut Writer<W>, tag: BytesStart, is_empty: bool, scope: F) -> Result<()>
where
    W: Write,
    F: FnOnce(&mut Writer<W>) -> Result<()>,
{
    if is_empty {
        writer.write_event(Event::Empty(tag))?;
    } else {
        let (start, end) = split_tag(&tag);
        writer.write_event(Event::Start(start))?;
        scope(writer)?;
        writer.write_event(Event::End(end))?;
    }
    Ok(())
}

fn scoped_iter<W, F, I>(
    writer: &mut Writer<W>,
    tag: BytesStart,
    iter: I,
    mut scope: F,
) -> Result<()>
where
    W: Write,
    I: IntoIterator,
    F: FnMut(&mut Writer<W>, <I as IntoIterator>::Item) -> Result<()>,
{
    let mut iter = iter.into_iter().peekable();
    scoped(writer, tag, iter.peek().is_none(), |writer| {
        for item in iter {
            scope(writer, item)?;
        }
        Ok(())
    })
}

fn text<W: Write, T: AsRef<[u8]>>(writer: &mut Writer<W>, tag: BytesStart, text: T) -> Result<()> {
    let text = text.as_ref();
    scoped(writer, tag, text.is_empty(), |writer| {
        writer.write_event(Event::Text(BytesText::borrowed(text)))?;
        Ok(())
    })
}

fn vec_as_string<F, R>(vec: &mut Vec<u8>, f: F) -> R
where
    F: FnOnce(&mut String) -> R,
{
    let taken = replace(vec, Vec::new());
    let mut string = String::from_utf8(taken).unwrap();
    let result = f(&mut string);
    let bytes = string.into_bytes();
    replace(vec, bytes);
    result
}

fn image<W: Write>(
    writer: &mut Writer<W>,
    tag: BytesStart,
    image: &Image,
    buf: &mut Vec<u8>,
    image_buf: &mut Cow<[u8]>,
) -> Result<()> {
    let url = image.url();
    if url.starts_with("data:;base64,") {
        let src = &url["data:;base64,".len()..];
        buf.clear();
        if base64::decode_config_buf(src, base64::STANDARD, buf).is_ok() {
            let len = buf.len();
            let image_buf = image_buf.to_mut();
            image_buf.truncate(LSS_IMAGE_HEADER.len());
            image_buf.reserve(len + 6);
            image_buf.write_u32::<LE>(len as u32).unwrap();
            image_buf.push(0x2);
            image_buf.append(buf);
            image_buf.push(0xB);
            buf.clear();
            vec_as_string(buf, |s| {
                base64::encode_config_buf(image_buf, base64::STANDARD, s)
            });
            return text(writer, tag, buf);
        }
    }
    writer.write_event(Event::Empty(tag))?;
    Ok(())
}

fn fmt_date(date: DateTime<Utc>, buf: &mut Vec<u8>) -> &[u8] {
    fmt_buf(date.format("%m/%d/%Y %T"), buf)
}

fn fmt_buf<D: Display>(value: D, buf: &mut Vec<u8>) -> &[u8] {
    buf.clear();
    write!(buf, "{}", value).unwrap();
    buf
}

fn write_display<W: Write, D: Display>(
    writer: &mut Writer<W>,
    tag: BytesStart,
    value: D,
    buf: &mut Vec<u8>,
) -> Result<()> {
    text(writer, tag, fmt_buf(value, buf))
}

fn time_span<W: Write>(
    writer: &mut Writer<W>,
    tag: BytesStart,
    time: TimeSpan,
    buf: &mut Vec<u8>,
) -> Result<()> {
    write_display(writer, tag, Complete.format(time), buf)
}

fn time_inner<W: Write>(writer: &mut Writer<W>, time: Time, buf: &mut Vec<u8>) -> Result<()> {
    if let Some(time) = time.real_time {
        time_span(writer, new_tag(b"RealTime"), time, buf)?;
    }

    if let Some(time) = time.game_time {
        time_span(writer, new_tag(b"GameTime"), time, buf)?;
    }

    Ok(())
}

fn time<W: Write>(
    writer: &mut Writer<W>,
    tag: BytesStart,
    time: Time,
    buf: &mut Vec<u8>,
) -> Result<()> {
    scoped(
        writer,
        tag,
        time.real_time.is_none() && time.game_time.is_none(),
        |writer| time_inner(writer, time, buf),
    )
}

pub fn save<W: Write>(run: &Run, writer: W) -> Result<()> {
    let writer = &mut Writer::new(writer);

    let buf = &mut Vec::new();
    let image_buf = &mut Cow::Borrowed(&LSS_IMAGE_HEADER[..]);

    writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), None)))?;
    writer.write_event(Event::Start(
        BytesStart::borrowed(br#"Run version="1.7.0""#, 3),
    ))?;

    image(
        writer,
        new_tag(b"GameIcon"),
        run.game_icon(),
        buf,
        image_buf,
    )?;
    text(writer, new_tag(b"GameName"), run.game_name())?;
    text(writer, new_tag(b"CategoryName"), run.category_name())?;

    write_start(writer, new_tag(b"Metadata"))?;
    let metadata = run.metadata();

    let mut tag = new_tag(b"Run");
    tag.push_attribute((&b"id"[..], metadata.run_id().as_bytes()));
    writer.write_event(Event::Empty(tag))?;

    tag = new_tag(b"Platform");
    tag.push_attribute((&b"usesEmulator"[..], bool(metadata.uses_emulator())));
    text(writer, tag, metadata.platform_name())?;

    text(writer, new_tag(b"Region"), metadata.region_name())?;

    scoped_iter(
        writer,
        new_tag(b"Variables"),
        metadata.variables(),
        |writer, (name, value)| {
            let mut tag = new_tag(b"Variable");
            tag.push_attribute((&b"name"[..], name.as_bytes()));
            text(writer, tag, value)
        },
    )?;
    write_end(writer, b"Metadata")?;

    time_span(writer, new_tag(b"Offset"), run.offset(), buf)?;
    write_display(writer, new_tag(b"AttemptCount"), run.attempt_count(), buf)?;

    scoped_iter(
        writer,
        new_tag(b"AttemptHistory"),
        run.attempt_history(),
        |writer, attempt| {
            let mut tag = new_tag(b"Attempt");
            tag.push_attribute((&b"id"[..], fmt_buf(attempt.index(), buf)));

            if let Some(started) = attempt.started() {
                tag.push_attribute((&b"started"[..], fmt_date(started.time, buf)));
                tag.push_attribute((
                    &b"isStartedSynced"[..],
                    bool(started.synced_with_atomic_clock),
                ));
            }

            if let Some(ended) = attempt.ended() {
                tag.push_attribute((&b"ended"[..], fmt_date(ended.time, buf)));
                tag.push_attribute((
                    &b"isEndedSynced"[..],
                    bool(ended.synced_with_atomic_clock),
                ));
            }

            let is_empty = attempt.time().real_time.is_none() &&
                attempt.time().game_time.is_none() &&
                attempt.pause_time().is_none();

            scoped(writer, tag, is_empty, |writer| {
                time_inner(writer, attempt.time(), buf)?;

                if let Some(pause_time) = attempt.pause_time() {
                    time_span(writer, new_tag(b"PauseTime"), pause_time, buf)?;
                }

                Ok(())
            })
        },
    )?;

    scoped_iter(
        writer,
        new_tag(b"Segments"),
        run.segments(),
        |writer, segment| {
            write_start(writer, new_tag(b"Segment"))?;

            text(writer, new_tag(b"Name"), segment.name())?;
            image(writer, new_tag(b"Icon"), segment.icon(), buf, image_buf)?;

            scoped_iter(
                writer,
                new_tag(b"SplitTimes"),
                run.custom_comparisons(),
                |writer, comparison| {
                    let mut tag = new_tag(b"SplitTime");
                    tag.push_attribute((&b"name"[..], comparison.as_bytes()));
                    time(writer, tag, segment.comparison(comparison), buf)
                },
            )?;

            time(
                writer,
                new_tag(b"BestSegmentTime"),
                segment.best_segment_time(),
                buf,
            )?;

            scoped_iter(
                writer,
                new_tag(b"SegmentHistory"),
                segment.segment_history(),
                |writer, &(index, history_time)| {
                    let mut tag = new_tag(b"Time");
                    tag.push_attribute((&b"id"[..], fmt_buf(index, buf)));
                    time(writer, tag, history_time, buf)
                },
            )?;

            write_end(writer, b"Segment")
        },
    )?;

    scoped_iter(
        writer,
        new_tag(b"AutoSplitterSettings"),
        run.auto_splitter_settings(),
        |writer, event| {
            writer.write_event(event)?;
            Ok(())
        },
    )?;

    write_end(writer, b"Run")?;
    Ok(())
}
