//! Provides the parser for splits files used by the Llanfair Rewrite.

use super::xml_util::{end_tag, parse_base, parse_children, single_child, text, text_err,
                      text_parsed};
use byteorder::{ByteOrder, BE};
use imagelib::{png, ColorType, ImageBuffer, Rgba};
use quick_xml::Reader;
use std::cmp::min;
use std::io::BufRead;
use {RealTime, Run, Segment, Time, TimeSpan};

pub use super::xml_util::{Error, Result};

fn time_span<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, mut f: F) -> Result<()>
where
    R: BufRead,
    F: FnMut(TimeSpan),
{
    single_child(reader, buf, b"value", |reader, tag| {
        text_err(reader, tag.into_buf(), |text| {
            let milliseconds = text.parse::<i64>()?;
            f(TimeSpan::from_milliseconds(milliseconds as f64));
            Ok(())
        })
    })
}

fn time<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, mut f: F) -> Result<()>
where
    R: BufRead,
    F: FnMut(Time),
{
    time_span(reader, buf, |t| f(RealTime(Some(t)).into()))
}

fn image<R, F>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    image_buf: &mut Vec<u8>,
    mut f: F,
) -> Result<()>
where
    R: BufRead,
    F: FnMut(&[u8]),
{
    let (mut width, mut height) = (None, None);
    image_buf.clear();

    single_child(reader, buf, b"javax.swing.ImageIcon", |reader, tag| {
        parse_children(reader, tag.into_buf(), |reader, tag| {
            if tag.name() == b"default" {
                parse_children(reader, tag.into_buf(), |reader, tag| {
                    if tag.name() == b"height" {
                        text_parsed(reader, tag.into_buf(), |t: u32| height = Some(t))
                    } else if tag.name() == b"width" {
                        text_parsed(reader, tag.into_buf(), |t: u32| width = Some(t))
                    } else {
                        end_tag(reader, tag.into_buf())
                    }
                })
            } else if tag.name() == b"int-array" {
                image_buf.clear();
                if let (Some(width), Some(height)) = (width, height) {
                    let len = width as usize * height as usize * 4;
                    image_buf.reserve(min(len, 32 << 20));
                }

                let mut tmp = [0; 4];

                parse_children(reader, tag.into_buf(), |reader, tag| {
                    text_parsed(reader, tag.into_buf(), |value: i32| {
                        BE::write_i32(&mut tmp, value);
                        image_buf.extend_from_slice(&[tmp[1], tmp[2], tmp[3], tmp[0]]);
                    })
                })
            } else {
                end_tag(reader, tag.into_buf())
            }
        })
    })?;

    let height = height.ok_or(Error::ElementNotFound)?;
    let width = width.ok_or(Error::ElementNotFound)?;

    let image_buf = image_buf.as_slice();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, image_buf)
        .ok_or(Error::ElementNotFound)?;

    buf.clear();
    png::PNGEncoder::new(&mut *buf)
        .encode(image.as_ref(), width, height, ColorType::RGBA(8))
        .map_err(|_| Error::ElementNotFound)?;

    f(buf);

    Ok(())
}

fn parse_segment<R: BufRead>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    image_buf: &mut Vec<u8>,
) -> Result<Segment> {
    let mut segment = Segment::new("");

    parse_children(reader, buf, |reader, tag| {
        if tag.name() == b"name" {
            text(reader, tag.into_buf(), |t| segment.set_name(t))
        } else if tag.name() == b"icon" {
            image(reader, tag.into_buf(), image_buf, |i| segment.set_icon(i))
        } else if tag.name() == b"time" {
            time(reader, tag.into_buf(), |t| {
                segment.set_personal_best_split_time(t)
            })
        } else if tag.name() == b"best" {
            time(reader, tag.into_buf(), |t| segment.set_best_segment_time(t))
        } else {
            end_tag(reader, tag.into_buf())
        }
    })?;

    Ok(segment)
}

/// Attempts to parse a splits file used by the Llanfair Rewrite.
pub fn parse<R: BufRead>(source: R) -> Result<Run> {
    let reader = &mut Reader::from_reader(source);
    reader.expand_empty_elements(true);
    reader.trim_text(true);

    let mut buf = Vec::with_capacity(4096);
    let mut image_buf = Vec::with_capacity(4096);

    let mut run = Run::new();

    parse_base(reader, &mut buf, b"run", |reader, tag| {
        parse_children(reader, tag.into_buf(), |reader, tag| {
            if tag.name() == b"game" {
                text(reader, tag.into_buf(), |t| run.set_game_name(t))
            } else if tag.name() == b"category" {
                text(reader, tag.into_buf(), |t| run.set_category_name(t))
            } else if tag.name() == b"platform" {
                text(reader, tag.into_buf(), |t| {
                    run.metadata_mut().set_platform_name(t)
                })
            } else if tag.name() == b"region" {
                text(reader, tag.into_buf(), |t| {
                    run.metadata_mut().set_region_name(t)
                })
            } else if tag.name() == b"emulated" {
                text(reader, tag.into_buf(), |t| {
                    run.metadata_mut().set_emulator_usage(t == "true")
                })
            } else if tag.name() == b"segments" {
                parse_children(reader, tag.into_buf(), |reader, tag| {
                    if tag.name() == b"segment" {
                        let segment = parse_segment(reader, tag.into_buf(), &mut image_buf)?;
                        run.push_segment(segment);
                        Ok(())
                    } else {
                        end_tag(reader, tag.into_buf())
                    }
                })
            } else {
                end_tag(reader, tag.into_buf())
            }
        })
    })?;

    Ok(run)
}
