//! Provides the parser for Llanfair splits files.

use crate::{
    RealTime, Run, Segment, Time, TimeSpan,
    util::byte_parsing::{
        big_endian::{strip_u16, strip_u32, strip_u64},
        strip_slice, strip_u8,
    },
};
use core::{fmt, result::Result as StdResult, str};
#[cfg(feature = "std")]
use image::{ExtendedColorType, ImageBuffer, ImageEncoder, Rgba};

/// The Error type for splits files that couldn't be parsed by the Llanfair
/// Parser.
#[derive(Debug)]
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadHeader => f.write_str("Failed to read the header."),
            Self::InvalidHeader => {
                f.write_str("The Header doesn't match the header of a Llanfair file.")
            }
            Self::DetermineLength => f.write_str("Failed to determine the length of the file."),
            Self::SkipToGoal => f.write_str("Failed to skip ahead to the goal."),
            Self::ReadGoal { .. } => f.write_str("Failed to read the goal."),
            Self::SkipToTitle => f.write_str("Failed to skip ahead to the title."),
            Self::ReadTitle { .. } => f.write_str("Failed to read the title."),
            Self::ReadSegmentCount => f.write_str("Failed to read the amount of segments."),
            Self::ReadSegment => f.write_str("Failed to read the next segment."),
            Self::ReadBestSegment => {
                f.write_str("Failed to read the best segment time of a segment.")
            }
            Self::ReadIcon => f.write_str("Failed to read the icon of a segment."),
            Self::InvalidIconDimensions { width, height } => write!(
                f,
                "The dimensions {width}x{height} of a segment's icon are not valid."
            ),
            Self::ReadImageData => f.write_str("Failed to read the data of a segment's icon."),
            Self::ReadSegmentName { .. } => f.write_str("Failed to read the name of a segment."),
            Self::ReadSegmentTime => f.write_str("Failed to read the segment time of a segment."),
        }
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::ReadGoal { source }
            | Self::ReadTitle { source }
            | Self::ReadSegmentName { source } => Some(source),
            _ => None,
        }
    }
}

/// An error type that indicates that a string failed to be parsed.
#[derive(Debug)]
pub enum StringError {
    /// Failed to read the length of the string.
    ReadLength,
    /// The string was larger than the total remaining splits file.
    LengthOutOfBounds,
    /// The string is not encoded as valid UTF-8.
    Validate,
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadLength => f.write_str("Failed to read the length of the string."),
            Self::LengthOutOfBounds => {
                f.write_str("The string was larger than the total remaining splits file.")
            }
            Self::Validate => f.write_str("The string is not encoded as valid UTF-8."),
        }
    }
}

impl core::error::Error for StringError {}

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
    let str_length = strip_u16(cursor).ok_or(StringError::ReadLength)? as usize;
    let str_data = strip_slice(cursor, str_length).ok_or(StringError::LengthOutOfBounds)?;
    simdutf8::basic::from_utf8(str_data).map_err(|_| StringError::Validate)
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
    let mut cursor = source.get(0xc5..).ok_or(Error::SkipToGoal)?;
    run.set_category_name(read_string(&mut cursor).map_err(|source| Error::ReadGoal { source })?);

    // Skip to the title string
    cursor = cursor.get(1..).ok_or(Error::SkipToTitle)?;
    run.set_game_name(read_string(&mut cursor).map_err(|source| Error::ReadTitle { source })?);

    cursor = cursor.get(0x6..).ok_or(Error::ReadSegmentCount)?;
    let segment_count = strip_u32(&mut cursor).ok_or(Error::ReadSegmentCount)?;

    // The object header changes if there is no instance of one of the object
    // used by the Run class. The 2 objects that can be affected are the Time
    // object and the ImageIcon object. The next step of the import algorithm is
    // to check for their presence.
    let (mut time_encountered, mut icon_encountered) = (false, false);

    let mut aggregate_time_ms = 0;

    // Seek to the first byte of the first segment
    cursor = cursor.get(0x8F..).ok_or(Error::ReadSegment)?;

    for _ in 0..segment_count {
        #[cfg(feature = "std")]
        let mut icon = None;
        let mut best_segment_ms = 0;

        let id = strip_u8(&mut cursor).ok_or(Error::ReadSegment)?;
        if id != 0x70 {
            if !time_encountered {
                time_encountered = true;

                // Seek past the object declaration
                cursor = cursor.get(0x36..).ok_or(Error::ReadSegment)?;
            } else {
                cursor = cursor.get(0x5..).ok_or(Error::ReadSegment)?;
            }

            best_segment_ms = strip_u64(&mut cursor).ok_or(Error::ReadBestSegment)?;
        }

        let id = strip_u8(&mut cursor).ok_or(Error::ReadIcon)?;
        if id != 0x70 {
            let seek_offset_base = if !icon_encountered {
                icon_encountered = true;
                cursor = cursor.get(0xBC..).ok_or(Error::ReadIcon)?;
                0x25
            } else {
                cursor = cursor.get(0x5..).ok_or(Error::ReadIcon)?;
                0x18
            };
            let height = strip_u32(&mut cursor).ok_or(Error::ReadIcon)?;
            let width = strip_u32(&mut cursor).ok_or(Error::ReadIcon)?;

            cursor = cursor.get(seek_offset_base..).ok_or(Error::ReadIcon)?;

            let len = (width as usize)
                .checked_mul(height as usize)
                .and_then(|b| b.checked_mul(4))
                .ok_or(Error::InvalidIconDimensions { width, height })?;

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
        cursor = cursor.get(1..).ok_or(Error::ReadSegment)?;
        let mut segment = Segment::new(
            read_string(&mut cursor).map_err(|source| Error::ReadSegmentName { source })?,
        );

        #[cfg(feature = "std")]
        if let Some(icon) = icon {
            segment.set_icon(icon);
        }

        // Read the segment time
        let id = strip_u8(&mut cursor).ok_or(Error::ReadSegmentTime)?;
        let segment_time_ms = match id {
            0x71 => {
                cursor = cursor.get(0x4..).ok_or(Error::ReadSegmentTime)?;
                best_segment_ms
            }
            0x70 => 0,
            _ => {
                // Since there is always a best segment when there is a best
                // time in Llanfair, I assume that there will never be another
                // Time object declaration before this data.
                cursor = cursor.get(0x5..).ok_or(Error::ReadSegmentTime)?;
                strip_u64(&mut cursor).ok_or(Error::ReadSegmentTime)?
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
        cursor = cursor.get(0x6..).ok_or(Error::ReadSegment)?;
    }

    Ok(run)
}
