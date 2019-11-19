//! Provides the parser for splits files used by Gered's Llanfair fork.

use crate::xml_util::{
    end_tag, optional_attribute_err, parse_base, parse_children, single_child, text,
    text_as_bytes_err, text_err, text_parsed,
};
use crate::{RealTime, Run, Segment, Time, TimeSpan};
use base64::{self, STANDARD};
use byteorder::{ReadBytesExt, BE};
use image::{png, ColorType, ImageBuffer, Rgba};
use quick_xml::Reader;
use snafu::OptionExt;
use std::io::{BufRead, Cursor, Seek, SeekFrom};

use crate::xml_util::Error as XmlError;

/// The Error type for splits files that couldn't be parsed by the Llanfair (Gered)
/// Parser.
#[derive(Debug, snafu::Snafu, derive_more::From)]
pub enum Error {
    /// The underlying XML format couldn't be parsed.
    Xml {
        /// The underlying error.
        source: XmlError,
    },
    /// Failed to decode a string slice as UTF-8.
    Utf8Str {
        /// The underlying error.
        source: core::str::Utf8Error,
    },
    /// Failed to decode a string as UTF-8.
    Utf8String {
        /// The underlying error.
        source: alloc::string::FromUtf8Error,
    },
    /// Failed to parse an integer.
    Int {
        /// The underlying error.
        source: core::num::ParseIntError,
    },
    /// The length of a buffer was too large.
    LengthOutOfBounds,
    /// Failed to parse an image.
    Image,
}

/// The Result type for the Llanfair (Gered) Parser.
pub type Result<T> = core::result::Result<T, Error>;

// FIXME: Generalized Type Ascription (GTA 6)
#[inline]
fn type_hint<T>(v: Result<T>) -> Result<T> {
    v
}

fn time_span<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(TimeSpan),
{
    text_err(reader, buf, |text| {
        let milliseconds = text.parse::<i64>()?;
        f(TimeSpan::from_milliseconds(milliseconds as f64));
        Ok(())
    })
}

fn time<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(Time),
{
    time_span(reader, buf, |t| f(RealTime(Some(t)).into()))
}

fn image<R, F>(
    reader: &mut Reader<R>,
    tag_buf: &mut Vec<u8>,
    buf: &mut Vec<u8>,
    mut f: F,
) -> Result<()>
where
    R: BufRead,
    F: FnMut(&[u8]),
{
    single_child(reader, tag_buf, b"ImageIcon", |reader, tag| {
        let tag_buf = tag.into_buf();
        let (width, height, image) = text_as_bytes_err(reader, tag_buf, |t| {
            buf.clear();
            base64::decode_config_buf(&t, STANDARD, buf).map_err(|_| Error::Image)?;

            let (width, height);
            {
                let mut cursor = Cursor::new(&buf);
                cursor
                    .seek(SeekFrom::Current(0xD1))
                    .map_err(|_| Error::Image)?;
                height = cursor.read_u32::<BE>().map_err(|_| Error::Image)?;
                width = cursor.read_u32::<BE>().map_err(|_| Error::Image)?;
            }

            let len = (width as usize)
                .checked_mul(height as usize)
                .and_then(|b| b.checked_mul(4))
                .context(LengthOutOfBounds)?;

            if buf.len() < 0xFE + len {
                return Err(Error::Image);
            }

            let buf = &buf[0xFE..][..len];
            let image = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, buf).context(Image)?;

            Ok((width, height, image))
        })?;

        tag_buf.clear();
        png::PNGEncoder::new(&mut *tag_buf)
            .encode(image.as_ref(), width, height, ColorType::RGBA(8))
            .map_err(|_| Error::Image)?;

        f(tag_buf);

        Ok(())
    })
}

fn parse_segment<R>(
    total_time: &mut TimeSpan,
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    buf2: &mut Vec<u8>,
) -> Result<Segment>
where
    R: BufRead,
{
    single_child(reader, buf, b"Segment", |reader, tag| {
        single_child(reader, tag.into_buf(), b"default", |reader, tag| {
            let mut segment = Segment::new("");
            let mut defer_setting_run_time = false;

            parse_children(reader, tag.into_buf(), |reader, tag| {
                if tag.name() == b"name" {
                    text(reader, tag.into_buf(), |t| segment.set_name(t))
                } else if tag.name() == b"bestTime" {
                    single_child(reader, tag.into_buf(), b"milliseconds", |reader, tag| {
                        time(reader, tag.into_buf(), |t| {
                            segment.set_best_segment_time(t);
                        })
                    })
                } else if tag.name() == b"runTime" {
                    type_hint(optional_attribute_err(&tag, b"reference", |reference| {
                        if reference == "../bestTime" {
                            defer_setting_run_time = true;
                        }
                        Ok(())
                    }))?;
                    if !defer_setting_run_time {
                        single_child(reader, tag.into_buf(), b"milliseconds", |reader, tag| {
                            time_span(reader, tag.into_buf(), |t| {
                                *total_time += t;
                            })
                        })?;
                        segment.set_personal_best_split_time(RealTime(Some(*total_time)).into());
                        Ok(())
                    } else {
                        end_tag(reader, tag.into_buf())
                    }
                } else if tag.name() == b"icon" {
                    image(reader, tag.into_buf(), buf2, |i| {
                        segment.set_icon(i);
                    })
                } else {
                    end_tag(reader, tag.into_buf())
                }
            })?;

            if defer_setting_run_time {
                *total_time += segment.best_segment_time().real_time.ok_or(Error::Xml {
                    source: XmlError::ElementNotFound,
                })?;
                segment.set_personal_best_split_time(RealTime(Some(*total_time)).into());
            }

            Ok(segment)
        })
    })
}

/// Attempts to parse a splits file used by Gered's Llanfair fork.
pub fn parse<R: BufRead>(source: R) -> Result<Run> {
    let reader = &mut Reader::from_reader(source);
    reader.expand_empty_elements(true);
    reader.trim_text(true);

    let mut buf = Vec::with_capacity(4096);
    let mut buf2 = Vec::with_capacity(4096);

    let mut run = Run::new();

    parse_base(reader, &mut buf, b"Run", |reader, tag| {
        single_child(reader, tag.into_buf(), b"Run", |reader, tag| {
            single_child(reader, tag.into_buf(), b"default", |reader, tag| {
                parse_children(reader, tag.into_buf(), |reader, tag| {
                    if tag.name() == b"name" {
                        text(reader, tag.into_buf(), |t| run.set_game_name(t))
                    } else if tag.name() == b"subTitle" {
                        text(reader, tag.into_buf(), |t| run.set_category_name(t))
                    } else if tag.name() == b"delayedStart" {
                        time_span(reader, tag.into_buf(), |t| {
                            run.set_offset(TimeSpan::zero() - t);
                        })
                    } else if tag.name() == b"numberOfAttempts" {
                        text_parsed(reader, tag.into_buf(), |t| run.set_attempt_count(t))
                    } else if tag.name() == b"segments" {
                        let mut total_time = TimeSpan::zero();
                        parse_children(reader, tag.into_buf(), |reader, tag| {
                            let segment =
                                parse_segment(&mut total_time, reader, tag.into_buf(), &mut buf2)?;
                            run.push_segment(segment);
                            Ok(())
                        })
                    } else {
                        end_tag(reader, tag.into_buf())
                    }
                })
            })
        })
    })?;

    Ok(run)
}
