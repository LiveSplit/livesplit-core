use std::io::{self, Read, Cursor, Seek, SeekFrom};
use std::num::ParseIntError;
use std::result::Result as StdResult;
use {Run, TimeSpan, Time, RealTime, Segment};
use base64::{self, STANDARD};
use byteorder::{ReadBytesExt, BigEndian as BE};
use imagelib::{png, Rgba, ImageBuffer, ColorType};
use super::xml_util::{self, text};
use sxd_document::dom::Element;
use sxd_document::parser::{Error as XmlError, parse as parse_xml};

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
        ElementNotFound
        AttributeNotFound
    }
}

pub type Result<T> = StdResult<T, Error>;

fn child<'d>(element: &Element<'d>, name: &str) -> Result<Element<'d>> {
    xml_util::child(element, name).ok_or(Error::ElementNotFound)
}

fn attribute<'d>(element: &Element<'d>, attribute: &str) -> Result<&'d str> {
    xml_util::attribute(element, attribute).ok_or(Error::AttributeNotFound)
}

fn time_span(element: &Element, buf: &mut String) -> Result<TimeSpan> {
    let text = text(element, buf);
    let milliseconds = text.parse::<i64>()?;
    Ok(TimeSpan::from_milliseconds(milliseconds as f64))
}

fn time(element: &Element, buf: &mut String) -> Result<Time> {
    Ok(RealTime(Some(time_span(element, buf)?)).into())
}

fn image<'b>(node: &Element,
             buf: &mut Vec<u8>,
             buf2: &'b mut Vec<u8>,
             str_buf: &mut String)
             -> Result<&'b [u8]> {
    let node = child(&node, "icon")?;
    let node = child(&node, "ImageIcon")?;

    buf.clear();
    base64::decode_config_buf(text(&node, str_buf), STANDARD, buf)
        .map_err(|_| Error::ElementNotFound)?;

    let (width, height);
    {
        let mut cursor = Cursor::new(&buf);
        cursor.seek(SeekFrom::Current(0xD1))?;
        height = cursor.read_u32::<BE>()?;
        width = cursor.read_u32::<BE>()?;
    }

    let len = width as usize * height as usize * 4;

    if buf.len() < 0xFE + len {
        return Err(Error::ElementNotFound);
    }

    let buf = &buf[0xFE..][..len];
    let image = ImageBuffer::<Rgba<_>, _>::from_raw(width, height, buf)
        .ok_or(Error::ElementNotFound)?;

    buf2.clear();
    png::PNGEncoder::new(&mut *buf2)
        .encode(image.as_ref(), width, height, ColorType::RGBA(8))
        .map_err(|_| Error::ElementNotFound)?;

    Ok(buf2)
}

pub fn parse<R: Read>(mut source: R) -> Result<Run> {
    let buf = &mut String::new();
    let mut byte_buf = Vec::new();
    let mut byte_buf2 = Vec::new();
    source.read_to_string(buf)?;
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

    let node = child(&node, "Run")?;
    let node = child(&node, "default")?;

    run.set_game_name(text(&child(&node, "name")?, buf));
    run.set_category_name(text(&child(&node, "subTitle")?, buf));
    run.set_offset(TimeSpan::zero() - time_span(&child(&node, "delayedStart")?, buf)?);
    run.set_attempt_count(text(&child(&node, "numberOfAttempts")?, buf).parse()?);

    let segments = child(&node, "segments")?;

    let mut total_time = TimeSpan::zero();

    for node in segments
            .children()
            .into_iter()
            .filter_map(|c| c.element()) {
        let node = child(&node, "Segment")?;
        let node = child(&node, "default")?;

        let mut segment = Segment::new(text(&child(&node, "name")?, buf));

        if let Ok(node) = child(&node, "bestTime") {
            if let Ok(node) = child(&node, "milliseconds") {
                segment.set_best_segment_time(time(&node, buf)?);
            }
        }

        if let Ok(node) = child(&node, "runTime") {
            if let Ok(node) = child(&node, "milliseconds") {
                total_time += time_span(&node, buf)?;
            } else if let Ok("../bestTime") = attribute(&node, "reference") {
                total_time += segment
                    .best_segment_time()
                    .real_time
                    .ok_or(Error::ElementNotFound)?;
            }
            segment.set_personal_best_split_time(RealTime(Some(total_time)).into());
        }

        if let Ok(image) = image(&node, &mut byte_buf, &mut byte_buf2, buf) {
            segment.set_icon(image);
        }

        run.push_segment(segment);
    }

    Ok(run)
}
