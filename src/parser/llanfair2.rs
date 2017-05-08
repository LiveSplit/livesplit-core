use {Run, TimeSpan, Time, RealTime, Segment};
use byteorder::{BigEndian, ByteOrder};
use imagelib::{png, Rgba, ImageBuffer, ColorType};
use std::cmp::min;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::result::Result as StdResult;
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
    }
}

pub type Result<T> = StdResult<T, Error>;

fn child<'d>(element: &Element<'d>, name: &str) -> Result<Element<'d>> {
    xml_util::child(element, name).ok_or(Error::ElementNotFound)
}

fn time_span(element: &Element, buf: &mut String) -> Result<TimeSpan> {
    let node = child(&element, "value")?;
    let text = text(&node, buf);
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
    let node = child(&node, "javax.swing.ImageIcon")?;

    let default = child(&node, "default")?;
    let height = text(&child(&default, "height")?, str_buf)
        .parse::<u32>()?;
    let width = text(&child(&default, "width")?, str_buf).parse::<u32>()?;

    buf.clear();
    let len = width as usize * height as usize * 4;
    buf.reserve(min(len, 32 << 20));

    let array = child(&node, "int-array")?;
    let mut tmp = [0; 4];

    for node in array.children().into_iter().filter_map(|c| c.element()) {
        let value = text(&node, str_buf).parse::<i32>()?;
        BigEndian::write_i32(&mut tmp, value);
        buf.extend_from_slice(&[tmp[1], tmp[2], tmp[3], tmp[0]]);
    }

    let buf = buf.as_slice();
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

    if let Ok(node) = child(&node, "game") {
        run.set_game_name(text(&node, buf));
    }
    if let Ok(node) = child(&node, "category") {
        run.set_category_name(text(&node, buf));
    }
    if let Ok(node) = child(&node, "platform") {
        run.metadata_mut().set_platform_name(text(&node, buf));
    }
    if let Ok(node) = child(&node, "region") {
        run.metadata_mut().set_region_name(text(&node, buf));
    }
    run.metadata_mut()
        .set_emulator_usage(text(&child(&node, "emulated")?, buf) == "true");

    let segments = child(&node, "segments")?;

    for node in segments
            .children()
            .into_iter()
            .filter_map(|c| c.element()) {
        let mut segment = Segment::new(child(&node, "name").ok().map_or("", |n| text(&n, buf)));

        if let Ok(image) = image(&node, &mut byte_buf, &mut byte_buf2, buf) {
            segment.set_icon(image);
        }

        if let Ok(node) = child(&node, "time") {
            segment.set_personal_best_split_time(time(&node, buf)?);
        }

        if let Ok(node) = child(&node, "best") {
            segment.set_best_segment_time(time(&node, buf)?);
        }

        run.push_segment(segment);
    }

    Ok(run)
}
