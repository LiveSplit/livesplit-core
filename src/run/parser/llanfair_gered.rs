//! Provides the parser for splits files used by Gered's Llanfair fork.

use core::mem::MaybeUninit;

#[cfg(feature = "std")]
use crate::util::byte_parsing::big_endian::strip_u32;
#[cfg(feature = "std")]
use crate::util::xml::helper::text_as_str_err;
use crate::{
    platform::prelude::*,
    util::xml::{
        helper::{
            end_tag, optional_attribute_escaped_err, parse_base, parse_children, single_child,
            text, text_err, text_parsed, Error as XmlError,
        },
        Reader,
    },
    RealTime, Run, Segment, Time, TimeSpan,
};
#[cfg(feature = "std")]
use image::{codecs::png, ColorType, ImageEncoder};
#[cfg(feature = "std")]
use snafu::OptionExt;

/// The Error type for splits files that couldn't be parsed by the Llanfair (Gered)
/// Parser.
#[derive(Debug, snafu::Snafu)]
#[snafu(context(suffix(false)))]
pub enum Error {
    /// The underlying XML format couldn't be parsed.
    Xml {
        /// The underlying error.
        source: XmlError,
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

impl From<XmlError> for Error {
    fn from(source: XmlError) -> Self {
        Self::Xml { source }
    }
}

impl From<core::num::ParseIntError> for Error {
    fn from(source: core::num::ParseIntError) -> Self {
        Self::Int { source }
    }
}

/// The Result type for the Llanfair (Gered) Parser.
pub type Result<T> = core::result::Result<T, Error>;

// FIXME: Generalized Type Ascription (GTA 6)
#[inline]
const fn type_hint<T>(v: Result<T>) -> Result<T> {
    v
}

fn time_span<F>(reader: &mut Reader<'_>, f: F) -> Result<()>
where
    F: FnOnce(TimeSpan),
{
    text_err(reader, |text| {
        let milliseconds = text.parse::<i64>()?;
        f(TimeSpan::from_milliseconds(milliseconds as f64));
        Ok(())
    })
}

fn time<F>(reader: &mut Reader<'_>, f: F) -> Result<()>
where
    F: FnOnce(Time),
{
    time_span(reader, |t| f(RealTime(Some(t)).into()))
}

#[cfg(feature = "std")]
fn image<F>(
    reader: &mut Reader<'_>,
    raw_buf: &mut Vec<MaybeUninit<u8>>,
    png_buf: &mut Vec<u8>,
    mut f: F,
) -> Result<()>
where
    F: FnMut(&[u8]),
{
    single_child(reader, "ImageIcon", |reader, _| {
        let (width, height, image) = text_as_str_err::<_, _, Error>(reader, |t| {
            let src = t.as_bytes();

            raw_buf.resize(
                base64_simd::STANDARD.estimated_decoded_length(src.len()),
                MaybeUninit::uninit(),
            );

            let decoded = base64_simd::STANDARD
                .decode(src, base64_simd::Out::from_uninit_slice(raw_buf))
                .map_err(|_| Error::Image)?;

            let (width, height);
            let mut cursor = decoded.get(0xD1..).ok_or(Error::Image)?;
            height = strip_u32(&mut cursor).ok_or(Error::Image)?;
            width = strip_u32(&mut cursor).ok_or(Error::Image)?;

            let len = (width as usize)
                .checked_mul(height as usize)
                .and_then(|b| b.checked_mul(4))
                .context(LengthOutOfBounds)?;

            Ok((
                width,
                height,
                decoded.get(0xFE..0xFE + len).ok_or(Error::Image)?,
            ))
        })?;

        png_buf.clear();
        png::PngEncoder::new(&mut *png_buf)
            .write_image(image, width, height, ColorType::Rgba8)
            .map_err(|_| Error::Image)?;

        f(png_buf);

        Ok(())
    })
}

fn parse_segment(
    total_time: &mut TimeSpan,
    reader: &mut Reader<'_>,
    _raw_buf: &mut Vec<MaybeUninit<u8>>,
    _png_buf: &mut Vec<u8>,
) -> Result<Segment> {
    single_child(reader, "Segment", |reader, _| {
        single_child(reader, "default", |reader, _| {
            let mut segment = Segment::new("");
            let mut defer_setting_run_time = false;

            parse_children(reader, |reader, tag, attributes| match tag.name() {
                "name" => text(reader, |t| segment.set_name(t)),
                "bestTime" => single_child(reader, "milliseconds", |reader, _| {
                    time(reader, |t| segment.set_best_segment_time(t))
                }),
                "runTime" => {
                    type_hint(optional_attribute_escaped_err(
                        attributes,
                        "reference",
                        |reference| {
                            if reference == "../bestTime" {
                                defer_setting_run_time = true;
                            }
                            Ok(())
                        },
                    ))?;
                    if !defer_setting_run_time {
                        single_child(reader, "milliseconds", |reader, _| {
                            time_span(reader, |t| {
                                *total_time += t;
                            })
                        })?;
                        segment.set_personal_best_split_time(RealTime(Some(*total_time)).into());
                        Ok(())
                    } else {
                        end_tag(reader)
                    }
                }
                #[cfg(feature = "std")]
                "icon" => image(reader, _raw_buf, _png_buf, |i| segment.set_icon(i)),
                _ => end_tag(reader),
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
pub fn parse(source: &str) -> Result<Run> {
    let reader = &mut Reader::new(source);

    let mut raw_buf = Vec::new();
    let mut png_buf = Vec::new();

    let mut run = Run::new();

    parse_base(reader, "Run", |reader, _| {
        single_child(reader, "Run", |reader, _| {
            single_child(reader, "default", |reader, _| {
                parse_children(reader, |reader, tag, _| match tag.name() {
                    "name" => text(reader, |t| run.set_game_name(t)),
                    "subTitle" => text(reader, |t| run.set_category_name(t)),
                    "delayedStart" => time_span(reader, |t| run.set_offset(TimeSpan::zero() - t)),
                    "numberOfAttempts" => text_parsed(reader, |t| run.set_attempt_count(t)),
                    "segments" => {
                        let mut total_time = TimeSpan::zero();
                        parse_children(reader, |reader, _, _| {
                            let segment =
                                parse_segment(&mut total_time, reader, &mut raw_buf, &mut png_buf)?;
                            run.push_segment(segment);
                            Ok(())
                        })
                    }
                    _ => end_tag(reader),
                })
            })
        })
    })?;

    Ok(run)
}
