//! Provides the parser for Llanfair splits files.

use crate::{
    util::byte_parsing::{
        big_endian::{strip_u16, strip_u32, strip_u64},
        strip_slice, strip_u8,
    },
    RealTime, Run, Segment, Time, TimeSpan,
};
use core::{result::Result as StdResult, str};
#[cfg(feature = "std")]
use image::{ExtendedColorType, ImageBuffer, ImageEncoder, Rgba};
use snafu::{OptionExt, ResultExt};

/// The Error type for splits files that couldn't be parsed by the Llanfair
/// Parser.
#[derive(Debug, snafu::Snafu)]
#[snafu(context(suffix(false)))]
pub enum Error {
    /// Failed to read the header.
    ReadHeader,
    /// The Header doesn't match the header of a Llanfair file.
    InvalidHeader,
    /// Failed to determine the length of the file.
    DetermineLength,
    /// Failed to skip ahead to the goal.
    SkipToGoal,
    /// Failed to read the goal.
    ReadGoal {
        /// The underlying error.
        source: StringError,
    },
    /// Failed to skip ahead to the title.
    SkipToTitle,
    /// Failed to read the title.
    ReadTitle {
        /// The underlying error.
        source: StringError,
    },
    /// Failed to read the amount of segments.
    ReadSegmentCount,
    /// Failed to read the next segment.
    ReadSegment,
    /// Failed to read the best segment time of a segment.
    ReadBestSegment,
    /// Failed to read the icon of a segment.
    ReadIcon,
    /// The icon of a segment has invalid dimensions.
    #[snafu(display("The dimensions {width}x{height} of a segment's icon are not valid."))]
    InvalidIconDimensions {
        /// The width of the icon.
        width: u32,
        /// The height of the icon.
        height: u32,
    },
    /// Failed to read the data of a segment's icon.
    ReadImageData,
    /// Failed to read the name of a segment.
    ReadSegmentName {
        /// The underlying error.
        source: StringError,
    },
    /// Failed to read the segment time of a segment.
    ReadSegmentTime,
}

/// An error type that indicates that a string failed to be parsed.
#[derive(Debug, snafu::Snafu)]
#[snafu(context(suffix(false)))]
pub enum StringError {
    /// Failed to read the length of the string.
    ReadLength,
    /// The string was larger than the total remaining splits file.
    LengthOutOfBounds,
    /// The string is not encoded as valid UTF-8.
    Validate,
}

/// The Result type for the Llanfair Parser.
pub type Result<T> = StdResult<T, Error>;

fn to_time(milliseconds: u64) -> Time {
    if milliseconds == 0 {
        Time::default()
    } else {
        RealTime(Some(TimeSpan::from_milliseconds(milliseconds as f64))).into()
    }
}

fn read_string<'a>(cursor: &mut &'a [u8]) -> StdResult<&'a str, StringError> {
    let str_length = strip_u16(cursor).context(ReadLength)? as usize;
    let str_data = strip_slice(cursor, str_length).context(LengthOutOfBounds)?;
    simdutf8::basic::from_utf8(str_data).ok().context(Validate)
}

/// Attempts to parse a Llanfair splits file.
pub fn parse(source: &[u8]) -> Result<Run> {
    #[cfg(feature = "std")]
    let mut buf = Vec::new();

    // The protocol is documented here:
    // https://docs.oracle.com/javase/7/docs/platform/serialization/spec/protocol.html

    // AC ED  Magic
    // 00 05  Version
    // 73     New Object
    // 72     New Class Declaration
    // 00 16  Length of Class Name
    const HEADER: &[u8; 30] = b"\xAC\xED\0\x05\x73\x72\0\x16org.fenix.llanfair.Run";

    if !source.starts_with(HEADER) {
        return Err(Error::InvalidHeader);
    }

    let mut run = Run::new();

    // Skip to the goal string
    let mut cursor = source.get(0xc5..).context(SkipToGoal)?;
    run.set_category_name(read_string(&mut cursor).context(ReadGoal)?);

    // Skip to the title string
    cursor = cursor.get(1..).context(SkipToTitle)?;
    run.set_game_name(read_string(&mut cursor).context(ReadTitle)?);

    cursor = cursor.get(0x6..).context(ReadSegmentCount)?;
    let segment_count = strip_u32(&mut cursor).context(ReadSegmentCount)?;

    // The object header changes if there is no instance of one of the object
    // used by the Run class. The 2 objects that can be affected are the Time
    // object and the ImageIcon object. The next step of the import algorithm is
    // to check for their presence.
    let (mut time_encountered, mut icon_encountered) = (false, false);

    let mut aggregate_time_ms = 0;

    // Seek to the first byte of the first segment
    cursor = cursor.get(0x8F..).context(ReadSegment)?;

    run.segments_mut().reserve(segment_count as usize);

    for _ in 0..segment_count {
        #[cfg(feature = "std")]
        let mut icon = None;
        let mut best_segment_ms = 0;

        let id = strip_u8(&mut cursor).context(ReadSegment)?;
        if id != 0x70 {
            if !time_encountered {
                time_encountered = true;

                // Seek past the object declaration
                cursor = cursor.get(0x36..).context(ReadSegment)?;
            } else {
                cursor = cursor.get(0x5..).context(ReadSegment)?;
            }

            best_segment_ms = strip_u64(&mut cursor).context(ReadBestSegment)?;
        }

        let id = strip_u8(&mut cursor).context(ReadIcon)?;
        if id != 0x70 {
            let seek_offset_base = if !icon_encountered {
                icon_encountered = true;
                cursor = cursor.get(0xBC..).context(ReadIcon)?;
                0x25
            } else {
                cursor = cursor.get(0x5..).context(ReadIcon)?;
                0x18
            };
            let height = strip_u32(&mut cursor).context(ReadIcon)?;
            let width = strip_u32(&mut cursor).context(ReadIcon)?;

            cursor = cursor.get(seek_offset_base..).context(ReadIcon)?;

            let len = (width as usize)
                .checked_mul(height as usize)
                .and_then(|b| b.checked_mul(4))
                .context(InvalidIconDimensions { width, height })?;

            if cursor.len() < len {
                return Err(Error::InvalidIconDimensions { width, height });
            }

            let (_image_data, rem) = cursor.split_at(len);
            cursor = rem;

            #[cfg(feature = "std")]
            if let Some(image) = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, _image_data) {
                buf.clear();
                if crate::util::image::create_reencoder(&mut buf)
                    .write_image(image.as_ref(), width, height, ExtendedColorType::Rgba8)
                    .is_ok()
                {
                    icon = Some(crate::settings::Image::new(
                        buf.as_slice().into(),
                        crate::settings::Image::ICON,
                    ));
                }
            }
        }

        // Skip to the segment name
        cursor = cursor.get(1..).context(ReadSegment)?;
        let mut segment = Segment::new(read_string(&mut cursor).context(ReadSegmentName)?);

        #[cfg(feature = "std")]
        if let Some(icon) = icon {
            segment.set_icon(icon);
        }

        // Read the segment time
        let id = strip_u8(&mut cursor).context(ReadSegmentTime)?;
        let segment_time_ms = match id {
            0x71 => {
                cursor = cursor.get(0x4..).context(ReadSegmentTime)?;
                best_segment_ms
            }
            0x70 => 0,
            _ => {
                // Since there is always a best segment when there is a best
                // time in Llanfair, I assume that there will never be another
                // Time object declaration before this data.
                cursor = cursor.get(0x5..).context(ReadSegmentTime)?;
                strip_u64(&mut cursor).context(ReadSegmentTime)?
            }
        };

        if segment_time_ms != 0 {
            aggregate_time_ms += segment_time_ms;
            let split_time = to_time(aggregate_time_ms);
            segment.set_personal_best_split_time(split_time);
        }

        segment.set_best_segment_time(to_time(best_segment_ms));

        run.push_segment(segment);

        // Seek to the beginning of the next segment name
        cursor = cursor.get(0x6..).context(ReadSegment)?;
    }

    Ok(run)
}
