use std::io::{self, Read, Seek, SeekFrom};
use std::result::Result as StdResult;
use std::str::{from_utf8, Utf8Error};
use byteorder::{ReadBytesExt, BigEndian as BE};
use imagelib::{png, Rgba, ImageBuffer, ColorType};
use {Run, Image, RealTime, TimeSpan, Time, Segment};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        LengthOutOfBounds {}
        Utf8(err: Utf8Error) {
            from()
        }
        Io(err: io::Error) {
            from()
        }
    }
}

pub type Result<T> = StdResult<T, Error>;

fn to_time(milliseconds: u64) -> Time {
    if milliseconds == 0 {
        Time::default()
    } else {
        RealTime(Some(TimeSpan::from_milliseconds(milliseconds as f64))).into()
    }
}

fn read_string<R: Read>(mut source: R, buf: &mut Vec<u8>, max_length: u64) -> Result<&str> {
    let str_length = source.read_u16::<BE>()? as usize;
    if str_length as u64 > max_length {
        return Err(Error::LengthOutOfBounds);
    }
    buf.clear();
    buf.reserve(str_length);
    unsafe { buf.set_len(str_length) };
    source.read_exact(buf)?;
    from_utf8(buf).map_err(Into::into)
}

pub fn parse<R: Read + Seek>(mut source: R) -> Result<Run> {
    let mut run = Run::new();
    let mut buf = Vec::new();
    let mut buf2 = Vec::new();

    // Determine total length of the source
    let total_len = source.seek(SeekFrom::End(0))?;

    // Skip to the goal string
    source.seek(SeekFrom::Start(0xc5))?;
    run.set_category_name(read_string(&mut source, &mut buf, total_len)?);

    // Skip to the title string
    source.read_u8()?;
    run.set_game_name(read_string(&mut source, &mut buf, total_len)?);

    source.seek(SeekFrom::Current(0x6))?;
    let segment_count = source.read_u32::<BE>()?;

    // The object header changes if there is no instance of one of the object used by the Run class.
    // The 2 objects that can be affected are the Time object and the ImageIcon object.
    // The next step of the import algorithm is to check for their presence.
    let (mut time_encountered, mut icon_encountered) = (false, false);

    let mut aggregate_time_ms = 0;

    // Seek to the first byte of the first segment
    source.seek(SeekFrom::Current(0x8F))?;
    for _ in 0..segment_count {
        let mut icon = None;
        let mut best_segment_ms = 0;

        let id = source.read_u8()?;
        if id != 0x70 {
            if !time_encountered {
                time_encountered = true;

                // Seek past the object declaration
                source.seek(SeekFrom::Current(0x36))?;
            } else {
                source.seek(SeekFrom::Current(0x5))?;
            }

            best_segment_ms = source.read_u64::<BE>()?;
        }

        let id = source.read_u8()?;
        if id != 0x70 {
            let seek_offset_base = if !icon_encountered {
                icon_encountered = true;
                source.seek(SeekFrom::Current(0xBC))?;
                0x25
            } else {
                source.seek(SeekFrom::Current(0x5))?;
                0x18
            };
            let height = source.read_u32::<BE>()?;
            let width = source.read_u32::<BE>()?;

            source.seek(SeekFrom::Current(seek_offset_base))?;

            let len = (width as usize)
                .checked_mul(height as usize)
                .and_then(|b| b.checked_mul(4))
                .ok_or(Error::LengthOutOfBounds)?;

            if len as u64 > total_len || width == 0 || height == 0 {
                return Err(Error::LengthOutOfBounds);
            }

            buf.clear();
            buf.reserve(len);
            unsafe { buf.set_len(len) };
            source.read_exact(&mut buf)?;

            if let Some(image) =
                ImageBuffer::<Rgba<_>, _>::from_raw(width, height, buf.as_slice())
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
        source.read_u8()?;
        let mut segment = Segment::new(read_string(&mut source, &mut buf, total_len)?);

        if let Some(icon) = icon {
            segment.set_icon(icon);
        }

        // Read the segment time
        let id = source.read_u8()?;
        let segment_time_ms = match id {
            0x71 => {
                source.seek(SeekFrom::Current(0x4))?;
                best_segment_ms
            }
            0x70 => 0,
            _ => {
                // Since there is always a best segment when there is a best time in Llanfair,
                // I assume that there will never be another Time object declaration before this data.
                source.seek(SeekFrom::Current(0x5))?;
                source.read_u64::<BE>()?
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
        source.seek(SeekFrom::Current(0x6))?;
    }

    Ok(run)
}
