use super::{
    accuracy, color, comparison_override, end_tag, parse_bool, parse_children, text_parsed,
    timer_format, timing_method_override, translate_size, GradientBuilder, Result,
};
use crate::{timing::formatter::DigitsFormat, xml::Reader};

pub use crate::component::detailed_timer::Component;

pub fn settings(reader: &mut Reader<'_>, component: &mut Component) -> Result<()> {
    let mut settings = component.settings().clone();
    let mut background_builder = GradientBuilder::new();
    let mut timer_override_color = false;
    let (mut total_height, mut segment_timer_ratio) = (65u32, 0.4);

    parse_children(reader, |reader, tag, _| {
        if !background_builder.parse_background(reader, tag.name())? {
            match tag.name() {
                "Height" => text_parsed(reader, |v| total_height = translate_size(v)),
                "SegmentTimerSizeRatio" => {
                    text_parsed(reader, |v: u32| segment_timer_ratio = 0.01 * v as f32)
                }
                "TimerShowGradient" => parse_bool(reader, |b| settings.timer.show_gradient = b),
                "OverrideTimerColors" => {
                    // Version >= 1.3
                    parse_bool(reader, |b| timer_override_color = b)
                }
                "TimerUseSplitColors" => {
                    // Version < 1.3
                    parse_bool(reader, |b| timer_override_color = !b)
                }
                "SegmentTimerShowGradient" => {
                    parse_bool(reader, |b| settings.segment_timer.show_gradient = b)
                }
                "TimerFormat" => {
                    // Version >= 1.5
                    timer_format(reader, |d, a| {
                        settings.timer.digits_format = d;
                        settings.timer.accuracy = a;
                    })
                }
                "SegmentTimerFormat" => {
                    // Version >= 1.5
                    timer_format(reader, |d, a| {
                        settings.segment_timer.digits_format = d;
                        settings.segment_timer.accuracy = a;
                    })
                }
                "TimerAccuracy" => {
                    // Version < 1.5
                    settings.timer.digits_format = DigitsFormat::SingleDigitSeconds;
                    accuracy(reader, |v| settings.timer.accuracy = v)
                }
                "SegmentTimerAccuracy" => {
                    // Version < 1.5
                    settings.segment_timer.digits_format = DigitsFormat::SingleDigitSeconds;
                    accuracy(reader, |v| settings.segment_timer.accuracy = v)
                }
                "TimerColor" => color(reader, |v| settings.timer.color_override = Some(v)),
                "DisplayIcon" => parse_bool(reader, |b| settings.display_icon = b),
                "ShowSplitName" => parse_bool(reader, |b| settings.show_segment_name = b),
                "Comparison" => comparison_override(reader, |v| settings.comparison1 = v),
                "Comparison2" => comparison_override(reader, |v| settings.comparison2 = v),
                "HideComparison" => parse_bool(reader, |b| settings.hide_second_comparison = b),
                "TimingMethod" => {
                    timing_method_override(reader, |v| settings.timer.timing_method = v)
                }
                _ => {
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
                    end_tag(reader)
                }
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
