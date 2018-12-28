use super::{
    accuracy, color, comparison_override, end_tag, parse_bool, parse_children, text_parsed,
    timer_format, timing_method_override, GradientBuilder, Result,
};
use crate::timing::formatter::DigitsFormat;
use quick_xml::Reader;
use std::io::BufRead;

pub use crate::component::detailed_timer::Component;

pub fn settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut Component,
) -> Result<()>
where
    R: BufRead,
{
    let mut settings = component.settings().clone();
    let mut background_builder = GradientBuilder::new();
    let mut timer_override_color = false;
    let (mut total_height, mut segment_timer_ratio) = (65u32, 0.4);

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"Height" {
                text_parsed(reader, tag.into_buf(), |v| total_height = v)
            } else if tag.name() == b"SegmentTimerSizeRatio" {
                text_parsed(reader, tag.into_buf(), |v: u32| {
                    segment_timer_ratio = v as f32 / 100.0
                })
            } else if tag.name() == b"TimerShowGradient" {
                parse_bool(reader, tag.into_buf(), |b| settings.timer.show_gradient = b)
            } else if tag.name() == b"OverrideTimerColors" {
                // Version >= 1.3
                parse_bool(reader, tag.into_buf(), |b| timer_override_color = b)
            } else if tag.name() == b"TimerUseSplitColors" {
                // Version < 1.3
                parse_bool(reader, tag.into_buf(), |b| timer_override_color = !b)
            } else if tag.name() == b"SegmentTimerShowGradient" {
                parse_bool(reader, tag.into_buf(), |b| {
                    settings.segment_timer.show_gradient = b
                })
            } else if tag.name() == b"TimerFormat" {
                // Version >= 1.5
                timer_format(reader, tag.into_buf(), |d, a| {
                    settings.timer.digits_format = d;
                    settings.timer.accuracy = a;
                })
            } else if tag.name() == b"SegmentTimerFormat" {
                // Version >= 1.5
                timer_format(reader, tag.into_buf(), |d, a| {
                    settings.segment_timer.digits_format = d;
                    settings.segment_timer.accuracy = a;
                })
            } else if tag.name() == b"TimerAccuracy" {
                // Version < 1.5
                settings.timer.digits_format = DigitsFormat::SingleDigitSeconds;
                accuracy(reader, tag.into_buf(), |v| settings.timer.accuracy = v)
            } else if tag.name() == b"SegmentTimerAccuracy" {
                // Version < 1.5
                settings.segment_timer.digits_format = DigitsFormat::SingleDigitSeconds;
                accuracy(reader, tag.into_buf(), |v| {
                    settings.segment_timer.accuracy = v
                })
            } else if tag.name() == b"TimerColor" {
                color(reader, tag.into_buf(), |v| {
                    settings.timer.color_override = Some(v)
                })
            } else if tag.name() == b"DisplayIcon" {
                parse_bool(reader, tag.into_buf(), |b| settings.display_icon = b)
            } else if tag.name() == b"ShowSplitName" {
                parse_bool(reader, tag.into_buf(), |b| settings.show_segment_name = b)
            } else if tag.name() == b"Comparison" {
                comparison_override(reader, tag.into_buf(), |v| settings.comparison1 = v)
            } else if tag.name() == b"Comparison2" {
                comparison_override(reader, tag.into_buf(), |v| settings.comparison2 = v)
            } else if tag.name() == b"HideComparison" {
                parse_bool(reader, tag.into_buf(), |b| {
                    settings.hide_second_comparison = b
                })
            } else if tag.name() == b"TimingMethod" {
                timing_method_override(reader, tag.into_buf(), |v| settings.timer.timing_method = v)
            } else {
                // FIXME:
                // Width
                // SegmentTimesAccuracy
                // SegmentTimerColor
                // SegmentLabelsColor
                // SegmentTimesColor
                // SegmentLabelsFont
                // SegmentTimesFont
                // SplitNameFont
                // IconSize
                // SplitNameColor
                // DecimalsSize
                // SegmentTimerDecimalsSize
                end_tag(reader, tag.into_buf())
            }
        } else {
            Ok(())
        }
    })?;

    if !timer_override_color {
        // FIXME: This isn't actually exposed in the Detailed Timer's settings.
        settings.timer.color_override = None;
    }
    settings.background = background_builder.build();

    settings.segment_timer.height = (total_height as f32 * segment_timer_ratio) as u32;
    settings.timer.height = total_height - settings.segment_timer.height;

    component.set_settings(settings);

    Ok(())
}
