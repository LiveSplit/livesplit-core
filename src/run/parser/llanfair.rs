//! Provides the parser for Llanfair splits files.

use crate::{settings::Image, RealTime, Run, Segment, Time, TimeSpan};
use byteorder::{ReadBytesExt, BE};
use core::result::Result as StdResult;
use core::str::{from_utf8, Utf8Error};
use image::{png, ColorType, ImageBuffer, Rgba};
use snafu::{OptionExt, ResultExt};
use std::io::{self, Read, Seek, SeekFrom};

/// The Error type for splits files that couldn't be parsed by the Llanfair
/// Parser.
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// Failed to read the header.
    ReadHeader {
        /// The underlying error.
        source: io::Error,
    },
    /// The Header doesn't match the header of a Llanfair file.
    InvalidHeader,
    /// Failed to determine the length of the file.
    DetermineLength {
        /// The underlying error.
        source: io::Error,
    },
    /// Failed to skip ahead to the goal.
    SkipToGoal {
        /// The underlying error.
        source: io::Error,
    },
    /// Failed to read the goal.
    ReadGoal {
        /// The underlying error.
        source: StringError,
    },
    /// Failed to skip ahead to the title.
    SkipToTitle {
        /// The underlying error.
        source: io::Error,
    },
    /// Failed to read the title.
    ReadTitle {
        /// The underlying error.
        source: StringError,
    },
    /// Failed to read the amount of segments.
    ReadSegmentCount {
        /// The underlying error.
        source: io::Error,
    },
    /// Failed to read the next segment.
    ReadSegment {
        /// The underlying error.
        source: io::Error,
    },
    /// Failed to read the best segment time of a segment.
    ReadBestSegment {
        /// The underlying error.
        source: io::Error,
    },
    /// Failed to read the icon of a segment.
    ReadIcon {
        /// The underlying error.
        source: io::Error,
    },
    /// The icon of a segment has invalid dimensions.
    #[snafu(display(
        "The dimensions {}x{} of a segment's icon are not valid.",
        width,
        height
    ))]
    InvalidIconDimensions {
        /// The width of the icon.
        width: u32,
        /// The height of the icon.
        height: u32,
    },
    /// Failed to read the data of a segment's icon.
    ReadImageData {
        /// The underlying error.
        source: io::Error,
    },
    /// Failed to read the name of a segment.
    ReadSegmentName {
        /// The underlying error.
        source: StringError,
    },
    /// Failed to read the segment time of a segment.
    ReadSegmentTime {
        /// The underlying error.
        source: io::Error,
    },
}

/// An error type that indicates that a string failed to be parsed.
#[derive(Debug, snafu::Snafu)]
pub enum StringError {
    /// Failed to read the length of the string.
    ReadLength {
        /// The underlying error.
        source: io::Error,
    },
    /// The string was larger than the total remaining splits file.
    LengthOutOfBounds,
    /// Failed to read the string data.
    ReadData {
        /// The underlying error.
        source: io::Error,
    },
    /// The string is not encoded as valid UTF-8.
    Validate {
        /// The underlying error.
        source: Utf8Error,
    },
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

fn read_string<R: Read>(
    mut source: R,
    buf: &mut Vec<u8>,
    max_length: u64,
) -> StdResult<&str, StringError> {
    let str_length = source.read_u16::<BE>().context(ReadLength)? as usize;
    if str_length as u64 > max_length {
        return Err(StringError::LengthOutOfBounds);
    }
    buf.clear();
    buf.reserve(str_length);
    unsafe { buf.set_len(str_length) };
    source.read_exact(buf).context(ReadData)?;
    from_utf8(buf).context(Validate)
}

/// Attempts to parse a Llanfair splits file.
pub fn parse<R: Read + Seek>(mut source: R) -> Result<Run> {
    let mut buf = Vec::new();
    let mut buf2 = Vec::new();

    // The protocol is documented here:
    // https://docs.oracle.com/javase/7/docs/platform/serialization/spec/protocol.html

    const HEADER: [u8; 30] = [
        0xAC, 0xED, // Magic
        0x00, 0x05, // Version
        0x73, // New Object
        0x72, // New Class Declaration
        0x00, 0x16, // Length of Class Name
        // org.fenix.llanfair.Run
        0x6F, 0x72, 0x67, 0x2E, 0x66, 0x65, 0x6E, 0x69, 0x78, 0x2E, 0x6C, 0x6C, 0x61, 0x6E, 0x66,
        0x61, 0x69, 0x72, 0x2E, 0x52, 0x75, 0x6E,
    ];
    let mut header_buf = [0; 30];
    source.read_exact(&mut header_buf).context(ReadHeader)?;
    if HEADER != header_buf {
        return Err(Error::InvalidHeader);
    }

    // Determine total length of the source
    let total_len = source.seek(SeekFrom::End(0)).context(DetermineLength)?;

    let mut run = Run::new();

    // Skip to the goal string
    source.seek(SeekFrom::Start(0xc5)).context(SkipToGoal)?;
    run.set_category_name(read_string(&mut source, &mut buf, total_len).context(ReadGoal)?);

    // Skip to the title string
    source.read_u8().context(SkipToTitle)?;
    run.set_game_name(read_string(&mut source, &mut buf, total_len).context(ReadTitle)?);

    source
        .seek(SeekFrom::Current(0x6))
        .context(ReadSegmentCount)?;
    let segment_count = source.read_u32::<BE>().context(ReadSegmentCount)?;

    // The object header changes if there is no instance of one of the object
    // used by the Run class. The 2 objects that can be affected are the Time
    // object and the ImageIcon object. The next step of the import algorithm is
    // to check for their presence.
    let (mut time_encountered, mut icon_encountered) = (false, false);

    let mut aggregate_time_ms = 0;

    // Seek to the first byte of the first segment
    source.seek(SeekFrom::Current(0x8F)).context(ReadSegment)?;
    for _ in 0..segment_count {
        let mut icon = None;
        let mut best_segment_ms = 0;

        let id = source.read_u8().context(ReadSegment)?;
        if id != 0x70 {
            if !time_encountered {
                time_encountered = true;

                // Seek past the object declaration
                source.seek(SeekFrom::Current(0x36)).context(ReadSegment)?;
            } else {
                source.seek(SeekFrom::Current(0x5)).context(ReadSegment)?;
            }

            best_segment_ms = source.read_u64::<BE>().context(ReadBestSegment)?;
        }

        let id = source.read_u8().context(ReadIcon)?;
        if id != 0x70 {
            let seek_offset_base = if !icon_encountered {
                icon_encountered = true;
                source.seek(SeekFrom::Current(0xBC)).context(ReadIcon)?;
                0x25
            } else {
                source.seek(SeekFrom::Current(0x5)).context(ReadIcon)?;
                0x18
            };
            let height = source.read_u32::<BE>().context(ReadIcon)?;
            let width = source.read_u32::<BE>().context(ReadIcon)?;

            source
                .seek(SeekFrom::Current(seek_offset_base))
                .context(ReadIcon)?;

            let len = (width as usize)
                .checked_mul(height as usize)
                .and_then(|b| b.checked_mul(4))
                .context(InvalidIconDimensions { width, height })?;

            if len as u64 > total_len || width == 0 || height == 0 {
                return Err(Error::InvalidIconDimensions { width, height });
            }

            buf.clear();
            buf.reserve(len);
            unsafe { buf.set_len(len) };
            source.read_exact(&mut buf).context(ReadImageData)?;

            if let Some(image) = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, buf.as_slice())
            {
                buf2.clear();
                if png::PNGEncoder::new(&mut buf2)
                    .encode(image.as_ref(), width, height, ColorType::RGBA(8))
                    .is_ok()
                {
                    icon = Some(Image::new(&buf2));
                }
            }
        }

        // Skip to the segment name
        source.read_u8().context(ReadSegment)?;
        let mut segment =
            Segment::new(read_string(&mut source, &mut buf, total_len).context(ReadSegmentName)?);

        if let Some(icon) = icon {
            segment.set_icon(icon);
        }

        // Read the segment time
        let id = source.read_u8().context(ReadSegmentTime)?;
        let segment_time_ms = match id {
            0x71 => {
                source
                    .seek(SeekFrom::Current(0x4))
                    .context(ReadSegmentTime)?;
                best_segment_ms
            }
            0x70 => 0,
            _ => {
                // Since there is always a best segment when there is a best
                // time in Llanfair, I assume that there will never be another
                // Time object declaration before this data.
                source
                    .seek(SeekFrom::Current(0x5))
                    .context(ReadSegmentTime)?;
                source.read_u64::<BE>().context(ReadSegmentTime)?
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
        source.seek(SeekFrom::Current(0x6)).context(ReadSegment)?;
    }

    Ok(run)
}
