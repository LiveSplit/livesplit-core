//! The LiveSplit Saver saves Runs as LiveSplit splits files (*.lss).
//!
//! # Examples
//!
//! Using the LiveSplit Saver to save a Run as a LiveSplit splits file.
//!
//! ```no_run
//! use livesplit_core::run::saver::livesplit::{self, IoWrite};
//! use livesplit_core::{Run, Segment};
//! use std::fs::File;
//! use std::io::BufWriter;
//!
//! // Create a run object that we can use.
//! let mut run = Run::new();
//! run.set_game_name("Super Mario Odyssey");
//! run.set_category_name("Any%");
//! run.push_segment(Segment::new("Cap Kingdom"));
//!
//! // Create the splits file.
//! let file = File::create("path/to/splits_file.lss");
//! let writer = BufWriter::new(file.expect("Failed creating the file"));
//!
//! // Save the splits file as a LiveSplit splits file.
//! livesplit::save_run(&run, IoWrite(writer)).expect("Couldn't save the splits file");
//! ```

use crate::{
    platform::prelude::*,
    settings::Image,
    timing::formatter::{Complete, TimeFormatter},
    util::xml::{AttributeWriter, DisplayValue, Text, Writer, NO_ATTRIBUTES},
    DateTime, Run, Time, Timer, TimerPhase,
};
use alloc::borrow::Cow;
use core::{fmt, mem::MaybeUninit};
use time::UtcOffset;

const LSS_IMAGE_HEADER: &[u8; 156] = include_bytes!("lss_image_header.bin");

const fn bool(value: bool) -> Text<'static> {
    Text::new_escaped(if value { "True" } else { "False" })
}

fn scoped_iter<W, I, F>(writer: &mut Writer<W>, tag: &str, iter: I, mut scope: F) -> fmt::Result
where
    W: fmt::Write,
    I: IntoIterator,
    F: FnMut(&mut Writer<W>, I::Item) -> fmt::Result,
{
    writer.tag(tag, |tag| {
        let mut iter = iter.into_iter().peekable();
        if iter.peek().is_some() {
            tag.content(|writer| {
                for item in iter {
                    scope(writer, item)?;
                }
                Ok(())
            })?;
        }
        Ok(())
    })
}

fn image<W: fmt::Write>(
    writer: &mut Writer<W>,
    tag: &str,
    image: &Image,
    base64_buf: &mut Vec<MaybeUninit<u8>>,
    image_buf: &mut Cow<'_, [u8]>,
) -> fmt::Result {
    writer.tag(tag, |tag| {
        let image_data = image.data();
        if image_data.is_empty() {
            return Ok(());
        }

        let len = image_data.len();
        let image_buf = image_buf.to_mut();
        image_buf.truncate(LSS_IMAGE_HEADER.len());
        image_buf.reserve(len + 6);
        image_buf.extend((len as u32).to_le_bytes());
        image_buf.push(0x2);
        image_buf.extend(image_data);
        image_buf.push(0xB);

        base64_buf.resize(
            base64_simd::STANDARD.encoded_length(image_buf.len()),
            MaybeUninit::uninit(),
        );

        let encoded = base64_simd::STANDARD
            .encode_as_str(image_buf, base64_simd::Out::from_uninit_slice(base64_buf));

        tag.content(|writer| writer.cdata(Text::new_escaped(encoded)))
    })
}

fn date<W: fmt::Write>(
    writer: &mut AttributeWriter<'_, W>,
    key: &str,
    date: DateTime,
) -> fmt::Result {
    let date = date.to_offset(UtcOffset::UTC);
    let (year, month, day) = date.to_calendar_date();
    let month = month as u8;
    let (hour, minute, second) = date.to_hms();

    writer.attribute(
        key,
        format_args!("{month:02}/{day:02}/{year:04} {hour:02}:{minute:02}:{second:02}"),
    )
}

fn time_inner<W: fmt::Write>(writer: &mut Writer<W>, time: Time) -> fmt::Result {
    if let Some(time) = time.real_time {
        writer.tag_with_text_content(
            "RealTime",
            NO_ATTRIBUTES,
            DisplayValue(Complete.format(time)),
        )?;
    }

    if let Some(time) = time.game_time {
        writer.tag_with_text_content(
            "GameTime",
            NO_ATTRIBUTES,
            DisplayValue(Complete.format(time)),
        )?;
    }

    Ok(())
}

fn time<W: fmt::Write>(writer: AttributeWriter<'_, W>, time: Time) -> fmt::Result {
    if time.real_time.is_some() || time.game_time.is_some() {
        writer.content(|writer| time_inner(writer, time))
    } else {
        Ok(())
    }
}

/// Wraps a type implementing `io::Write` to be used as a type implementing
/// `fmt::Write` in order to write to it.
#[cfg(feature = "std")]
pub struct IoWrite<W>(pub W);

#[cfg(feature = "std")]
impl<W: std::io::Write> fmt::Write for IoWrite<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        std::io::Write::write_all(&mut self.0, s.as_bytes()).map_err(|_| fmt::Error)
    }
}

/// Saves the Run in use by the Timer provided as a LiveSplit splits file
/// (*.lss).
pub fn save_timer<W: fmt::Write>(timer: &Timer, writer: W) -> fmt::Result {
    let run;
    let run = if timer.current_phase() == TimerPhase::NotRunning {
        timer.run()
    } else {
        run = timer.clone().into_run(true);
        &run
    };
    save_run(run, writer)
}

/// Saves a Run as a LiveSplit splits file (*.lss). Use the `save_timer`
/// function if the Run is in use by a timer in order to properly save the
/// current attempt as well.
pub fn save_run<W: fmt::Write>(run: &Run, writer: W) -> fmt::Result {
    let writer = &mut Writer::new_with_default_header(writer)?;

    let base64_buf = &mut Vec::new();
    let image_buf = &mut Cow::Borrowed(&LSS_IMAGE_HEADER[..]);

    writer.tag_with_content("Run", [("version", Text::new_escaped("1.8.0"))], |writer| {
        image(writer, "GameIcon", run.game_icon(), base64_buf, image_buf)?;
        writer.tag_with_text_content("GameName", NO_ATTRIBUTES, run.game_name())?;
        writer.tag_with_text_content("CategoryName", NO_ATTRIBUTES, run.category_name())?;

        writer.tag_with_content("Metadata", NO_ATTRIBUTES, |writer| {
            let metadata = run.metadata();
            writer.empty_tag("Run", [("id", metadata.run_id())])?;
            writer.tag_with_text_content(
                "Platform",
                [("usesEmulator", bool(metadata.uses_emulator()))],
                metadata.platform_name(),
            )?;
            writer.tag_with_text_content("Region", NO_ATTRIBUTES, metadata.region_name())?;
            scoped_iter(
                writer,
                "SpeedrunComVariables",
                metadata.speedrun_com_variables(),
                |writer, (name, value)| {
                    writer.tag_with_text_content("Variable", [("name", name)], value.as_str())
                },
            )?;
            scoped_iter(
                writer,
                "CustomVariables",
                metadata
                    .custom_variables()
                    .filter(|(_, var)| var.is_permanent),
                |writer, (name, var)| {
                    writer.tag_with_text_content("Variable", [("name", name)], var.value.as_str())
                },
            )
        })?;

        writer.tag_with_text_content(
            "Offset",
            NO_ATTRIBUTES,
            DisplayValue(Complete.format(run.offset())),
        )?;
        writer.tag_with_text_content(
            "AttemptCount",
            NO_ATTRIBUTES,
            DisplayValue(run.attempt_count()),
        )?;

        scoped_iter(
            writer,
            "AttemptHistory",
            run.attempt_history(),
            |writer, attempt| {
                writer.tag("Attempt", |mut tag| {
                    tag.attribute("id", DisplayValue(attempt.index()))?;

                    if let Some(started) = attempt.started() {
                        date(&mut tag, "started", started.time)?;
                        tag.attribute("isStartedSynced", bool(started.synced_with_atomic_clock))?;
                    }
                    if let Some(ended) = attempt.ended() {
                        date(&mut tag, "ended", ended.time)?;
                        tag.attribute("isEndedSynced", bool(ended.synced_with_atomic_clock))?;
                    }

                    let is_empty = attempt.time().real_time.is_none()
                        && attempt.time().game_time.is_none()
                        && attempt.pause_time().is_none();

                    if !is_empty {
                        tag.content(|writer| {
                            time_inner(writer, attempt.time())?;

                            if let Some(pause_time) = attempt.pause_time() {
                                writer.tag_with_text_content(
                                    "PauseTime",
                                    NO_ATTRIBUTES,
                                    DisplayValue(Complete.format(pause_time)),
                                )?;
                            }

                            Ok(())
                        })?;
                    }

                    Ok(())
                })
            },
        )?;

        scoped_iter(writer, "Segments", run.segments(), |writer, segment| {
            writer.tag_with_content("Segment", NO_ATTRIBUTES, |writer| {
                writer.tag_with_text_content("Name", NO_ATTRIBUTES, segment.name())?;
                image(writer, "Icon", segment.icon(), base64_buf, image_buf)?;

                scoped_iter(
                    writer,
                    "SplitTimes",
                    run.custom_comparisons(),
                    |writer, comparison| {
                        writer.tag("SplitTime", |mut tag| {
                            tag.attribute("name", comparison.as_str())?;
                            time(tag, segment.comparison(comparison))
                        })
                    },
                )?;

                writer.tag("BestSegmentTime", |tag| {
                    time(tag, segment.best_segment_time())
                })?;

                scoped_iter(
                    writer,
                    "SegmentHistory",
                    segment.segment_history(),
                    |writer, &(index, history_time)| {
                        writer.tag("Time", |mut tag| {
                            tag.attribute("id", DisplayValue(index))?;
                            time(tag, history_time)
                        })
                    },
                )
            })
        })?;

        writer.tag_with_text_content(
            "AutoSplitterSettings",
            NO_ATTRIBUTES,
            Text::new_escaped(run.auto_splitter_settings()),
        )
    })
}
