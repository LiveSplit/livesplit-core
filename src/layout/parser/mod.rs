mod xml_util;
pub use self::xml_util::{Error, Result};

use self::xml_util::{
    end_tag, parse_base, parse_children, text, text_as_escaped_bytes_err, text_err, text_parsed,
    Tag,
};
use super::{Component, Layout};
use crate::component::{
    blank_space, current_comparison, current_pace, delta, detailed_timer, graph,
    possible_time_save, previous_segment, separator, splits, sum_of_best, text, timer, title,
    total_playtime,
};
use crate::settings::{Alignment, Color, Gradient, ListGradient};
use crate::timing::{
    formatter::{Accuracy, DigitsFormat},
    TimingMethod,
};
use quick_xml::Reader;
use std::io::BufRead;

enum GradientKind {
    Transparent,
    Plain,
    Vertical,
    Horizontal,
}

enum ListGradientKind {
    Same(GradientKind),
    Alternating,
}

trait GradientType: Sized {
    type Built;
    fn default() -> Self;
    fn parse(kind: &[u8]) -> Result<Self>;
    fn build(self, first: Color, second: Color) -> Self::Built;
}

impl GradientType for GradientKind {
    type Built = Gradient;
    fn default() -> Self {
        GradientKind::Transparent
    }
    fn parse(kind: &[u8]) -> Result<Self> {
        Ok(match kind {
            b"Plain" => GradientKind::Plain,
            b"Vertical" => GradientKind::Vertical,
            b"Horizontal" => GradientKind::Horizontal,
            _ => return Err(Error::UnexpectedGradientType),
        })
    }
    fn build(self, first: Color, second: Color) -> Self::Built {
        match self {
            GradientKind::Transparent => Gradient::Transparent,
            GradientKind::Plain => {
                if first == Color::transparent() {
                    Gradient::Transparent
                } else {
                    Gradient::Plain(first)
                }
            }
            GradientKind::Horizontal => Gradient::Horizontal(first, second),
            GradientKind::Vertical => Gradient::Vertical(first, second),
        }
    }
}

impl GradientType for ListGradientKind {
    type Built = ListGradient;
    fn default() -> Self {
        ListGradientKind::Same(GradientKind::default())
    }
    fn parse(kind: &[u8]) -> Result<Self> {
        Ok(if kind == b"Alternating" {
            ListGradientKind::Alternating
        } else {
            ListGradientKind::Same(GradientKind::parse(kind)?)
        })
    }
    fn build(self, first: Color, second: Color) -> Self::Built {
        match self {
            ListGradientKind::Alternating => ListGradient::Alternating(first, second),
            ListGradientKind::Same(same) => ListGradient::Same(same.build(first, second)),
        }
    }
}

struct GradientBuilder<T: GradientType = GradientKind> {
    tag_color1: &'static [u8],
    tag_color2: &'static [u8],
    tag_kind: &'static [u8],
    kind: T,
    first: Color,
    second: Color,
}

impl GradientBuilder<GradientKind> {
    fn new() -> Self {
        Self::new_gradient_type()
    }
}

impl<T: GradientType> GradientBuilder<T> {
    fn new_gradient_type() -> Self {
        Self::with_tags(
            b"BackgroundColor",
            b"BackgroundColor2",
            b"BackgroundGradient",
        )
    }

    fn with_tags(
        tag_color1: &'static [u8],
        tag_color2: &'static [u8],
        tag_kind: &'static [u8],
    ) -> Self {
        Self {
            tag_color1,
            tag_color2,
            tag_kind,
            kind: T::default(),
            first: Color::transparent(),
            second: Color::transparent(),
        }
    }

    fn parse_background<'a, R>(
        &mut self,
        reader: &mut Reader<R>,
        tag: Tag<'a>,
    ) -> Result<Option<Tag<'a>>>
    where
        R: BufRead,
    {
        if tag.name() == self.tag_color1 {
            color(reader, tag.into_buf(), |c| self.first = c)?;
        } else if tag.name() == self.tag_color2 {
            color(reader, tag.into_buf(), |c| self.second = c)?;
        } else if tag.name() == self.tag_kind {
            text_as_escaped_bytes_err(reader, tag.into_buf(), |text| {
                self.kind = T::parse(&text)?;
                Ok(())
            })?;
        } else {
            return Ok(Some(tag));
        }
        Ok(None)
    }

    fn build(self) -> T::Built {
        self.kind.build(self.first, self.second)
    }
}

fn color<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(Color),
{
    text_err(reader, buf, |text| {
        let n = u32::from_str_radix(&text, 16)?;
        let b = (n & 0xFF) as u8;
        let g = ((n >> 8) & 0xFF) as u8;
        let r = ((n >> 16) & 0xFF) as u8;
        let a = ((n >> 24) & 0xFF) as u8;
        let mut color = Color::from([r, g, b, a]);
        let (r, g, b, a) = color.rgba.into_components();

        // Adjust alpha based on the lightness of the color. The formula is
        // based on two sRGB curves measured for white on top of a black
        // background and for black on top of a white background. We interpolate
        // between the two curves based on the lightness of the color. The
        // problem is that we only have the foreground color, so based on the
        // actual background color, this may be wrong. Therefore this is only a
        // heuristic. We often have white on dark grey, instead of white on
        // black. Because of that, we use 1.75 as the exponent denominator for
        // the white on black case instead of the usual 2.2 for sRGB.
        let lightness = (r + g + b) / 3.0;
        color.rgba.alpha =
            (1.0 - lightness) * (1.0 - (1.0 - a).powf(1.0 / 2.2)) + lightness * a.powf(1.0 / 1.75);

        f(color);
        Ok(())
    })
}

fn parse_bool<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(bool),
{
    text_as_escaped_bytes_err(reader, buf, |t| match &*t {
        b"True" => {
            f(true);
            Ok(())
        }
        b"False" => {
            f(false);
            Ok(())
        }
        _ => Err(Error::Bool),
    })
}

fn comparison_override<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(Option<String>),
{
    text(reader, buf, |t| {
        f(if t == "Current Comparison" {
            None
        } else {
            Some(t.into_owned())
        })
    })
}

fn timing_method_override<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(Option<TimingMethod>),
{
    text_as_escaped_bytes_err(reader, buf, |t| {
        f(match &*t {
            b"Current Timing Method" => None,
            b"Real Time" => Some(TimingMethod::RealTime),
            b"Game Time" => Some(TimingMethod::GameTime),
            _ => return Err(Error::TimingMethod),
        });
        Ok(())
    })
}

fn accuracy<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(Accuracy),
{
    text_as_escaped_bytes_err(reader, buf, |t| {
        f(match &*t {
            b"Tenths" => Accuracy::Tenths,
            b"Seconds" => Accuracy::Seconds,
            b"Hundredths" => Accuracy::Hundredths,
            _ => return Err(Error::Accuracy),
        });
        Ok(())
    })
}

fn timer_format<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(DigitsFormat, Accuracy),
{
    text_as_escaped_bytes_err(reader, buf, |t| {
        let mut splits = t.splitn(2, |&b| b == b'.');
        let digits_format = match splits.next().unwrap_or(b"") {
            b"1" => DigitsFormat::SingleDigitSeconds,
            b"00:01" => DigitsFormat::DoubleDigitMinutes,
            b"0:00:01" => DigitsFormat::SingleDigitHours,
            b"00:00:01" => DigitsFormat::DoubleDigitHours,
            _ => return Err(Error::DigitsFormat),
        };
        let accuracy = match splits.next().unwrap_or(b"") {
            b"23" => Accuracy::Hundredths,
            b"2" => Accuracy::Tenths,
            b"" => Accuracy::Seconds,
            _ => return Err(Error::Accuracy),
        };
        f(digits_format, accuracy);
        Ok(())
    })
}

fn blank_space_settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut blank_space::Component,
) -> Result<()>
where
    R: BufRead,
{
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"SpaceHeight" {
                text_parsed(reader, tag.into_buf(), |h| settings.height = h)
            } else {
                // FIXME:
                // SpaceWidth
                end_tag(reader, tag.into_buf())
            }
        } else {
            Ok(())
        }
    })?;

    settings.background = background_builder.build();

    Ok(())
}

fn current_comparison_settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut current_comparison::Component,
) -> Result<()>
where
    R: BufRead,
{
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();
    let (mut override_label, mut override_value) = (false, false);

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"TextColor" {
                color(reader, tag.into_buf(), |c| settings.label_color = Some(c))
            } else if tag.name() == b"OverrideTextColor" {
                parse_bool(reader, tag.into_buf(), |b| override_label = b)
            } else if tag.name() == b"TimeColor" {
                color(reader, tag.into_buf(), |c| settings.value_color = Some(c))
            } else if tag.name() == b"OverrideTimeColor" {
                parse_bool(reader, tag.into_buf(), |b| override_value = b)
            } else if tag.name() == b"Display2Rows" {
                parse_bool(reader, tag.into_buf(), |b| settings.display_two_rows = b)
            } else {
                // FIXME:
                // Font1
                // Font2
                // OverrideFont1
                // OverrideFont2
                end_tag(reader, tag.into_buf())
            }
        } else {
            Ok(())
        }
    })?;

    if !override_label {
        settings.label_color = None;
    }
    if !override_value {
        settings.value_color = None;
    }
    settings.background = background_builder.build();

    Ok(())
}

fn current_pace_settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut current_pace::Component,
) -> Result<()>
where
    R: BufRead,
{
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();
    let (mut override_label, mut override_value) = (false, false);

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"TextColor" {
                color(reader, tag.into_buf(), |c| settings.label_color = Some(c))
            } else if tag.name() == b"OverrideTextColor" {
                parse_bool(reader, tag.into_buf(), |b| override_label = b)
            } else if tag.name() == b"TimeColor" {
                color(reader, tag.into_buf(), |c| settings.value_color = Some(c))
            } else if tag.name() == b"OverrideTimeColor" {
                parse_bool(reader, tag.into_buf(), |b| override_value = b)
            } else if tag.name() == b"Accuracy" {
                accuracy(reader, tag.into_buf(), |a| settings.accuracy = a)
            } else if tag.name() == b"Comparison" {
                comparison_override(reader, tag.into_buf(), |v| settings.comparison_override = v)
            } else if tag.name() == b"Display2Rows" {
                parse_bool(reader, tag.into_buf(), |b| settings.display_two_rows = b)
            } else {
                end_tag(reader, tag.into_buf())
            }
        } else {
            Ok(())
        }
    })?;

    if !override_label {
        settings.label_color = None;
    }
    if !override_value {
        settings.value_color = None;
    }
    settings.background = background_builder.build();

    Ok(())
}

fn delta_settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut delta::Component,
) -> Result<()>
where
    R: BufRead,
{
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();
    let mut override_label = false;

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"TextColor" {
                color(reader, tag.into_buf(), |c| settings.label_color = Some(c))
            } else if tag.name() == b"OverrideTextColor" {
                parse_bool(reader, tag.into_buf(), |b| override_label = b)
            } else if tag.name() == b"Accuracy" {
                accuracy(reader, tag.into_buf(), |a| settings.accuracy = a)
            } else if tag.name() == b"Comparison" {
                comparison_override(reader, tag.into_buf(), |v| settings.comparison_override = v)
            } else if tag.name() == b"Display2Rows" {
                parse_bool(reader, tag.into_buf(), |b| settings.display_two_rows = b)
            } else if tag.name() == b"DropDecimals" {
                parse_bool(reader, tag.into_buf(), |b| settings.drop_decimals = b)
            } else {
                end_tag(reader, tag.into_buf())
            }
        } else {
            Ok(())
        }
    })?;

    if !override_label {
        settings.label_color = None;
    }
    settings.background = background_builder.build();

    Ok(())
}

fn detailed_timer_settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut detailed_timer::Component,
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

fn graph_settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut graph::Component,
) -> Result<()>
where
    R: BufRead,
{
    let settings = component.settings_mut();

    parse_children(reader, buf, |reader, tag| {
        if tag.name() == b"Height" {
            text_parsed(reader, tag.into_buf(), |v| settings.height = v)
        } else if tag.name() == b"BehindGraphColor" {
            color(reader, tag.into_buf(), |c| {
                settings.behind_background_color = c
            })
        } else if tag.name() == b"AheadGraphColor" {
            color(reader, tag.into_buf(), |c| {
                settings.ahead_background_color = c
            })
        } else if tag.name() == b"GridlinesColor" {
            color(reader, tag.into_buf(), |c| {
                println!("{:?} <- {:?}", settings.grid_lines_color, c);
                settings.grid_lines_color = c
            })
        } else if tag.name() == b"PartialFillColorAhead" {
            // Version >= 1.2
            color(reader, tag.into_buf(), |c| settings.partial_fill_color = c)
        } else if tag.name() == b"CompleteFillColorAhead" {
            // Version >= 1.2
            color(reader, tag.into_buf(), |c| settings.complete_fill_color = c)
        } else if tag.name() == b"PartialFillColor" {
            // Version < 1.2
            color(reader, tag.into_buf(), |c| settings.partial_fill_color = c)
        } else if tag.name() == b"CompleteFillColor" {
            // Version < 1.2
            color(reader, tag.into_buf(), |c| settings.complete_fill_color = c)
        } else if tag.name() == b"GraphColor" {
            color(reader, tag.into_buf(), |c| settings.graph_lines_color = c)
        } else if tag.name() == b"LiveGraph" {
            parse_bool(reader, tag.into_buf(), |b| settings.live_graph = b)
        } else if tag.name() == b"FlipGraph" {
            parse_bool(reader, tag.into_buf(), |b| settings.flip_graph = b)
        } else if tag.name() == b"Comparison" {
            comparison_override(reader, tag.into_buf(), |v| settings.comparison_override = v)
        } else if tag.name() == b"ShowBestSegments" {
            parse_bool(reader, tag.into_buf(), |b| settings.show_best_segments = b)
        } else {
            // FIXME:
            // Width
            // PartialFillColorBehind // Version >= 1.2
            // CompleteFillColorBehind // Version >= 1.2
            // ShadowsColor
            // GraphLinesColor (separators, not our graph_lines_color)
            // GraphGoldColor
            end_tag(reader, tag.into_buf())
        }
    })
}

fn possible_time_save_settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut possible_time_save::Component,
) -> Result<()>
where
    R: BufRead,
{
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();
    let (mut override_label, mut override_value) = (false, false);

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"TextColor" {
                color(reader, tag.into_buf(), |c| settings.label_color = Some(c))
            } else if tag.name() == b"OverrideTextColor" {
                parse_bool(reader, tag.into_buf(), |b| override_label = b)
            } else if tag.name() == b"TimeColor" {
                color(reader, tag.into_buf(), |c| settings.value_color = Some(c))
            } else if tag.name() == b"OverrideTimeColor" {
                parse_bool(reader, tag.into_buf(), |b| override_value = b)
            } else if tag.name() == b"Accuracy" {
                accuracy(reader, tag.into_buf(), |v| settings.accuracy = v)
            } else if tag.name() == b"Comparison" {
                comparison_override(reader, tag.into_buf(), |v| settings.comparison_override = v)
            } else if tag.name() == b"Display2Rows" {
                parse_bool(reader, tag.into_buf(), |b| settings.display_two_rows = b)
            } else if tag.name() == b"TotalTimeSave" {
                parse_bool(reader, tag.into_buf(), |b| {
                    settings.total_possible_time_save = b
                })
            } else {
                end_tag(reader, tag.into_buf())
            }
        } else {
            Ok(())
        }
    })?;

    if !override_label {
        settings.label_color = None;
    }
    if !override_value {
        settings.value_color = None;
    }
    settings.background = background_builder.build();

    Ok(())
}

fn previous_segment_settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut previous_segment::Component,
) -> Result<()>
where
    R: BufRead,
{
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();
    let mut override_label = false;

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"TextColor" {
                color(reader, tag.into_buf(), |c| settings.label_color = Some(c))
            } else if tag.name() == b"OverrideTextColor" {
                parse_bool(reader, tag.into_buf(), |b| override_label = b)
            } else if tag.name() == b"DeltaAccuracy" {
                accuracy(reader, tag.into_buf(), |v| settings.accuracy = v)
            } else if tag.name() == b"DropDecimals" {
                parse_bool(reader, tag.into_buf(), |b| settings.drop_decimals = b)
            } else if tag.name() == b"Comparison" {
                comparison_override(reader, tag.into_buf(), |v| settings.comparison_override = v)
            } else if tag.name() == b"Display2Rows" {
                parse_bool(reader, tag.into_buf(), |b| settings.display_two_rows = b)
            } else if tag.name() == b"ShowPossibleTimeSave" {
                parse_bool(reader, tag.into_buf(), |b| {
                    settings.show_possible_time_save = b
                })
            } else {
                // FIXME:
                // TimeSaveAccuracy
                end_tag(reader, tag.into_buf())
            }
        } else {
            Ok(())
        }
    })?;

    if !override_label {
        settings.label_color = None;
    }
    settings.background = background_builder.build();

    Ok(())
}

fn splits_settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut splits::Component,
) -> Result<()>
where
    R: BufRead,
{
    let settings = component.settings_mut();
    let mut split_gradient_builder = GradientBuilder::<GradientKind>::with_tags(
        b"CurrentSplitTopColor",
        b"CurrentSplitBottomColor",
        b"CurrentSplitGradient",
    );
    let mut background_builder = GradientBuilder::<ListGradientKind>::new_gradient_type();

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if let Some(tag) = split_gradient_builder.parse_background(reader, tag)? {
                if tag.name() == b"VisualSplitCount" {
                    text_parsed(reader, tag.into_buf(), |v| settings.visual_split_count = v)
                } else if tag.name() == b"SplitPreviewCount" {
                    text_parsed(reader, tag.into_buf(), |v| settings.split_preview_count = v)
                } else if tag.name() == b"ShowThinSeparators" {
                    parse_bool(reader, tag.into_buf(), |b| {
                        settings.show_thin_separators = b
                    })
                } else if tag.name() == b"AlwaysShowLastSplit" {
                    parse_bool(reader, tag.into_buf(), |b| {
                        settings.always_show_last_split = b
                    })
                } else if tag.name() == b"SplitPreviewCount" {
                    text_parsed(reader, tag.into_buf(), |v| settings.split_preview_count = v)
                } else if tag.name() == b"ShowBlankSplits" {
                    parse_bool(reader, tag.into_buf(), |b| {
                        settings.fill_with_blank_space = b
                    })
                } else if tag.name() == b"SeparatorLastSplit" {
                    parse_bool(reader, tag.into_buf(), |b| {
                        settings.separator_last_split = b
                    })
                } else if tag.name() == b"Display2Rows" {
                    parse_bool(reader, tag.into_buf(), |b| settings.display_two_rows = b)
                } else if tag.name() == b"ShowColumnLabels" {
                    parse_bool(reader, tag.into_buf(), |b| settings.show_column_labels = b)
                } else if tag.name() == b"Columns" {
                    // Version >= 1.5
                    settings.columns.clear();

                    parse_children(reader, tag.into_buf(), |reader, tag| {
                        let mut column = splits::ColumnSettings::default();
                        parse_children(reader, tag.into_buf(), |reader, tag| {
                            if tag.name() == b"Name" {
                                text(reader, tag.into_buf(), |v| column.name = v.into_owned())
                            } else if tag.name() == b"Comparison" {
                                comparison_override(reader, tag.into_buf(), |v| {
                                    column.comparison_override = v
                                })
                            } else if tag.name() == b"TimingMethod" {
                                timing_method_override(reader, tag.into_buf(), |v| {
                                    column.timing_method = v
                                })
                            } else if tag.name() == b"Type" {
                                text_err(reader, tag.into_buf(), |v| {
                                    use self::splits::{
                                        ColumnStartWith, ColumnUpdateTrigger, ColumnUpdateWith,
                                    };
                                    let (start_with, update_with, update_trigger) = match &*v {
                                        "Delta" => (
                                            ColumnStartWith::Empty,
                                            ColumnUpdateWith::Delta,
                                            ColumnUpdateTrigger::Contextual,
                                        ),
                                        "SplitTime" => (
                                            ColumnStartWith::ComparisonTime,
                                            ColumnUpdateWith::SplitTime,
                                            ColumnUpdateTrigger::OnEndingSegment,
                                        ),
                                        "DeltaorSplitTime" => (
                                            ColumnStartWith::ComparisonTime,
                                            ColumnUpdateWith::Delta, // TODO: With Fallback
                                            ColumnUpdateTrigger::Contextual,
                                        ),
                                        "SegmentDelta" => (
                                            ColumnStartWith::Empty,
                                            ColumnUpdateWith::SegmentDelta,
                                            ColumnUpdateTrigger::Contextual,
                                        ),
                                        "SegmentTime" => (
                                            ColumnStartWith::ComparisonSegmentTime,
                                            ColumnUpdateWith::SegmentTime,
                                            ColumnUpdateTrigger::OnEndingSegment,
                                        ),
                                        "SegmentDeltaorSegmentTime" => (
                                            ColumnStartWith::ComparisonSegmentTime,
                                            ColumnUpdateWith::SegmentDelta, // TODO: With Fallback
                                            ColumnUpdateTrigger::Contextual,
                                        ),
                                        _ => return Err(Error::UnexpectedColumnType),
                                    };
                                    column.start_with = start_with;
                                    column.update_with = update_with;
                                    column.update_trigger = update_trigger;
                                    Ok(())
                                })
                            } else {
                                end_tag(reader, tag.into_buf())
                            }
                        })?;
                        settings.columns.insert(0, column);
                        Ok(())
                    })
                } else if tag.name() == b"Comparison" {
                    // Version < 1.5
                    comparison_override(reader, tag.into_buf(), |v| {
                        for column in &mut settings.columns {
                            column.comparison_override = v.clone();
                        }
                    })
                } else if tag.name() == b"ShowSplitTimes" {
                    // Version < 1.5
                    use self::splits::{
                        ColumnSettings, ColumnStartWith, ColumnUpdateTrigger, ColumnUpdateWith,
                    };
                    parse_bool(reader, tag.into_buf(), |b| {
                        if !b {
                            let comparison_override =
                                settings.columns.pop().and_then(|c| c.comparison_override);
                            settings.columns.clear();
                            // TODO: Write a test that verifies the order of the
                            // two columns (The assumption here is that it's
                            // right to left).
                            settings.columns.push(ColumnSettings {
                                name: String::from("Time"),
                                start_with: ColumnStartWith::ComparisonTime,
                                update_with: ColumnUpdateWith::SplitTime,
                                update_trigger: ColumnUpdateTrigger::OnEndingSegment,
                                comparison_override: comparison_override.clone(),
                                timing_method: None,
                            });
                            settings.columns.push(ColumnSettings {
                                name: String::from("+/−"),
                                start_with: ColumnStartWith::Empty,
                                update_with: ColumnUpdateWith::Delta,
                                update_trigger: ColumnUpdateTrigger::Contextual,
                                comparison_override: comparison_override,
                                timing_method: None,
                            });
                        }
                    })
                } else {
                    // FIXME:
                    // DisplayIcons
                    // SplitWidth
                    // SplitTimesAccuracy
                    // AutomaticAbbreviations
                    // BeforeNamesColor // Version >= 1.3
                    // CurrentNamesColor // Version >= 1.3
                    // AfterNamesColor // Version >= 1.3
                    // OverrideTextColor // Version >= 1.3
                    // SplitNamesColor // Version >= 1.2 && Version < 1.3
                    // UseTextColor // Version < 1.3
                    // BeforeTimesColor
                    // CurrentTimesColor
                    // AfterTimesColor
                    // OverrideTimesColor
                    // LockLastSplit
                    // IconSize
                    // IconShadows
                    // SplitHeight
                    // DeltasAccuracy
                    // DropDecimals
                    // OverrideDeltasColor
                    // DeltasColor
                    // LabelsColor
                    end_tag(reader, tag.into_buf())
                }
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    })?;

    settings.current_split_gradient = split_gradient_builder.build();
    settings.background = background_builder.build();

    Ok(())
}

fn sum_of_best_settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut sum_of_best::Component,
) -> Result<()>
where
    R: BufRead,
{
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();
    let (mut override_label, mut override_value) = (false, false);

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"TextColor" {
                color(reader, tag.into_buf(), |c| settings.label_color = Some(c))
            } else if tag.name() == b"OverrideTextColor" {
                parse_bool(reader, tag.into_buf(), |b| override_label = b)
            } else if tag.name() == b"TimeColor" {
                color(reader, tag.into_buf(), |c| settings.value_color = Some(c))
            } else if tag.name() == b"OverrideTimeColor" {
                parse_bool(reader, tag.into_buf(), |b| override_value = b)
            } else if tag.name() == b"Accuracy" {
                accuracy(reader, tag.into_buf(), |v| settings.accuracy = v)
            } else if tag.name() == b"Display2Rows" {
                parse_bool(reader, tag.into_buf(), |b| settings.display_two_rows = b)
            } else {
                end_tag(reader, tag.into_buf())
            }
        } else {
            Ok(())
        }
    })?;

    if !override_label {
        settings.label_color = None;
    }
    if !override_value {
        settings.value_color = None;
    }
    settings.background = background_builder.build();

    Ok(())
}

fn text_settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut text::Component,
) -> Result<()>
where
    R: BufRead,
{
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();
    let (mut override_label, mut override_value) = (false, false);
    let (mut left_center, mut right) = (String::new(), String::new());

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"TextColor" {
                color(reader, tag.into_buf(), |c| {
                    settings.left_center_color = Some(c)
                })
            } else if tag.name() == b"OverrideTextColor" {
                parse_bool(reader, tag.into_buf(), |b| override_label = b)
            } else if tag.name() == b"TimeColor" {
                color(reader, tag.into_buf(), |c| settings.right_color = Some(c))
            } else if tag.name() == b"OverrideTimeColor" {
                parse_bool(reader, tag.into_buf(), |b| override_value = b)
            } else if tag.name() == b"Text1" {
                text(reader, tag.into_buf(), |v| left_center = v.into_owned())
            } else if tag.name() == b"Text2" {
                text(reader, tag.into_buf(), |v| right = v.into_owned())
            } else if tag.name() == b"Display2Rows" {
                parse_bool(reader, tag.into_buf(), |b| settings.display_two_rows = b)
            } else {
                // FIXME:
                // Font1
                // Font2
                // OverrideFont1
                // OverrideFont2
                end_tag(reader, tag.into_buf())
            }
        } else {
            Ok(())
        }
    })?;

    if !override_label {
        settings.left_center_color = None;
    }
    if !override_value {
        settings.right_color = None;
    }
    settings.text = match (left_center.is_empty(), right.is_empty()) {
        (false, false) => text::Text::Split(left_center, right),
        (false, true) => text::Text::Center(left_center),
        _ => text::Text::Center(right),
    };
    settings.background = background_builder.build();

    Ok(())
}

fn timer_settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut timer::Component,
) -> Result<()>
where
    R: BufRead,
{
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();
    let mut override_color = false;

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"TimerHeight" {
                text_parsed(reader, tag.into_buf(), |v| settings.height = v)
            } else if tag.name() == b"TimerFormat" {
                // Version >= 1.5
                timer_format(reader, tag.into_buf(), |d, a| {
                    settings.digits_format = d;
                    settings.accuracy = a;
                })
            } else if tag.name() == b"TimerAccuracy" {
                // Version >= 1.2 && Version < 1.5
                accuracy(reader, tag.into_buf(), |v| settings.accuracy = v)
            } else if tag.name() == b"OverrideSplitColors" {
                // Version >= 1.3
                parse_bool(reader, tag.into_buf(), |b| override_color = b)
            } else if tag.name() == b"UseSplitColors" {
                // Version < 1.3
                parse_bool(reader, tag.into_buf(), |b| override_color = !b)
            } else if tag.name() == b"ShowGradient" {
                parse_bool(reader, tag.into_buf(), |b| settings.show_gradient = b)
            } else if tag.name() == b"TimerColor" {
                color(reader, tag.into_buf(), |c| {
                    settings.color_override = Some(c)
                })
            } else if tag.name() == b"TimingMethod" {
                timing_method_override(reader, tag.into_buf(), |v| settings.timing_method = v)
            } else {
                // FIXME:
                // TimerWidth
                // CenterTimer
                // DecimalsSize
                end_tag(reader, tag.into_buf())
            }
        } else {
            Ok(())
        }
    })?;

    if !override_color {
        settings.color_override = None;
    }
    settings.background = background_builder.build();

    Ok(())
}

fn title_settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut title::Component,
) -> Result<()>
where
    R: BufRead,
{
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();
    let mut override_title_color = false;

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"ShowGameName" {
                parse_bool(reader, tag.into_buf(), |b| settings.show_game_name = b)
            } else if tag.name() == b"ShowCategoryName" {
                parse_bool(reader, tag.into_buf(), |b| settings.show_category_name = b)
            } else if tag.name() == b"ShowAttemptCount" {
                parse_bool(reader, tag.into_buf(), |b| settings.show_attempt_count = b)
            } else if tag.name() == b"ShowFinishedRunsCount" {
                parse_bool(reader, tag.into_buf(), |b| {
                    settings.show_finished_runs_count = b
                })
            } else if tag.name() == b"OverrideTitleColor" {
                parse_bool(reader, tag.into_buf(), |b| override_title_color = b)
            } else if tag.name() == b"TextAlignment" {
                // Version >= 1.7.3
                text_as_escaped_bytes_err(reader, tag.into_buf(), |v| {
                    settings.text_alignment = match &*v {
                        b"0" => Alignment::Auto,
                        b"1" => Alignment::Left,
                        b"2" => Alignment::Center,
                        _ => return Err(Error::Alignment),
                    };
                    Ok(())
                })
            } else if tag.name() == b"CenterTitle" {
                // Version >= 1.3 && Version < 1.7.3
                parse_bool(reader, tag.into_buf(), |b| {
                    settings.text_alignment = if b {
                        Alignment::Center
                    } else {
                        Alignment::Auto
                    }
                })
            } else if tag.name() == b"SingleLine" {
                parse_bool(reader, tag.into_buf(), |b| {
                    settings.display_as_single_line = b
                })
            } else if tag.name() == b"TitleColor" {
                color(reader, tag.into_buf(), |c| settings.text_color = Some(c))
            } else if tag.name() == b"DisplayGameIcon" {
                parse_bool(reader, tag.into_buf(), |b| settings.display_game_icon = b)
            } else if tag.name() == b"ShowRegion" {
                parse_bool(reader, tag.into_buf(), |b| settings.show_region = b)
            } else if tag.name() == b"ShowPlatform" {
                parse_bool(reader, tag.into_buf(), |b| settings.show_platform = b)
            } else if tag.name() == b"ShowVariables" {
                parse_bool(reader, tag.into_buf(), |b| settings.show_variables = b)
            } else {
                // FIXME:
                // OverrideTitleFont // Version >= 1.3
                // TitleFont // Version >= 1.2
                // UseLayoutSettingsFont // Version >= 1.2 && Version < 1.3
                end_tag(reader, tag.into_buf())
            }
        } else {
            Ok(())
        }
    })?;

    if !override_title_color {
        settings.text_color = None;
    }
    settings.background = background_builder.build();

    Ok(())
}

fn total_playtime_settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut total_playtime::Component,
) -> Result<()>
where
    R: BufRead,
{
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();
    let (mut override_label, mut override_value) = (false, false);

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"TextColor" {
                color(reader, tag.into_buf(), |c| settings.label_color = Some(c))
            } else if tag.name() == b"OverrideTextColor" {
                parse_bool(reader, tag.into_buf(), |b| override_label = b)
            } else if tag.name() == b"TimeColor" {
                color(reader, tag.into_buf(), |c| settings.value_color = Some(c))
            } else if tag.name() == b"OverrideTimeColor" {
                parse_bool(reader, tag.into_buf(), |b| override_value = b)
            } else if tag.name() == b"Display2Rows" {
                parse_bool(reader, tag.into_buf(), |b| settings.display_two_rows = b)
            } else if tag.name() == b"ShowTotalHours" {
                parse_bool(reader, tag.into_buf(), |b| settings.show_days = !b)
            } else {
                end_tag(reader, tag.into_buf())
            }
        } else {
            Ok(())
        }
    })?;

    if !override_label {
        settings.label_color = None;
    }
    if !override_value {
        settings.value_color = None;
    }
    settings.background = background_builder.build();

    Ok(())
}

fn component<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(Component),
{
    let mut component = None;

    parse_children(reader, buf, |reader, tag| {
        if tag.name() == b"Path" {
            text_err(reader, tag.into_buf(), |text| {
                component = Some(match &*text {
                    "LiveSplit.BlankSpace.dll" => blank_space::Component::new().into(),
                    "LiveSplit.CurrentComparison.dll" => {
                        current_comparison::Component::new().into()
                    }
                    "LiveSplit.RunPrediction.dll" => current_pace::Component::new().into(),
                    "LiveSplit.Delta.dll" => delta::Component::new().into(),
                    "LiveSplit.DetailedTimer.dll" => {
                        Box::new(detailed_timer::Component::new()).into()
                    }
                    "LiveSplit.Graph.dll" => graph::Component::new().into(),
                    "LiveSplit.PossibleTimeSave.dll" => possible_time_save::Component::new().into(),
                    "LiveSplit.PreviousSegment.dll" => previous_segment::Component::new().into(),
                    "" => separator::Component::new().into(),
                    "LiveSplit.Splits.dll" => splits::Component::new().into(),
                    "LiveSplit.SumOfBest.dll" => sum_of_best::Component::new().into(),
                    "LiveSplit.Text.dll" => text::Component::new().into(),
                    "LiveSplit.Timer.dll" => timer::Component::new().into(),
                    "LiveSplit.Title.dll" => title::Component::new().into(),
                    "LiveSplit.TotalPlaytime.dll" => total_playtime::Component::new().into(),
                    _ => return Ok(()),
                });
                Ok(())
            })
        } else if tag.name() == b"Settings" {
            // Assumption: Settings always has to come after the Path.
            // Otherwise we need to cache the settings and load them later.
            if let Some(component) = &mut component {
                match component {
                    Component::BlankSpace(c) => blank_space_settings(reader, tag.into_buf(), c),
                    Component::CurrentComparison(c) => {
                        current_comparison_settings(reader, tag.into_buf(), c)
                    }
                    Component::CurrentPace(c) => current_pace_settings(reader, tag.into_buf(), c),
                    Component::Delta(c) => delta_settings(reader, tag.into_buf(), c),
                    Component::DetailedTimer(c) => {
                        detailed_timer_settings(reader, tag.into_buf(), c)
                    }
                    Component::Graph(c) => graph_settings(reader, tag.into_buf(), c),
                    Component::PossibleTimeSave(c) => {
                        possible_time_save_settings(reader, tag.into_buf(), c)
                    }
                    Component::PreviousSegment(c) => {
                        previous_segment_settings(reader, tag.into_buf(), c)
                    }
                    Component::Separator(_) => end_tag(reader, tag.into_buf()),
                    Component::Splits(c) => splits_settings(reader, tag.into_buf(), c),
                    Component::SumOfBest(c) => sum_of_best_settings(reader, tag.into_buf(), c),
                    Component::Text(c) => text_settings(reader, tag.into_buf(), c),
                    Component::Timer(c) => timer_settings(reader, tag.into_buf(), c),
                    Component::Title(c) => title_settings(reader, tag.into_buf(), c),
                    Component::TotalPlaytime(c) => {
                        total_playtime_settings(reader, tag.into_buf(), c)
                    }
                }
            } else {
                end_tag(reader, tag.into_buf())
            }
        } else {
            end_tag(reader, tag.into_buf())
        }
    })?;

    if let Some(component) = component {
        f(component);
    }

    Ok(())
}

fn parse_general_settings<R: BufRead>(
    layout: &mut Layout,
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
) -> Result<()> {
    let settings = layout.general_settings_mut();
    let mut background_builder = GradientBuilder::new();

    parse_children(reader, buf, |reader, tag| {
        if tag.name() == b"TextColor" {
            color(reader, tag.into_buf(), |color| {
                settings.text_color = color;
            })
        } else if tag.name() == b"BackgroundColor" {
            color(reader, tag.into_buf(), |color| {
                background_builder.first = color;
            })
        } else if tag.name() == b"BackgroundColor2" {
            color(reader, tag.into_buf(), |color| {
                background_builder.second = color;
            })
        } else if tag.name() == b"ThinSeparatorsColor" {
            color(reader, tag.into_buf(), |color| {
                settings.thin_separators_color = color;
            })
        } else if tag.name() == b"SeparatorsColor" {
            color(reader, tag.into_buf(), |color| {
                settings.separators_color = color;
            })
        } else if tag.name() == b"PersonalBestColor" {
            color(reader, tag.into_buf(), |color| {
                settings.personal_best_color = color;
            })
        } else if tag.name() == b"AheadGainingTimeColor" {
            color(reader, tag.into_buf(), |color| {
                settings.ahead_gaining_time_color = color;
            })
        } else if tag.name() == b"AheadLosingTimeColor" {
            color(reader, tag.into_buf(), |color| {
                settings.ahead_losing_time_color = color;
            })
        } else if tag.name() == b"BehindGainingTimeColor" {
            color(reader, tag.into_buf(), |color| {
                settings.behind_gaining_time_color = color;
            })
        } else if tag.name() == b"BehindLosingTimeColor" {
            color(reader, tag.into_buf(), |color| {
                settings.behind_losing_time_color = color;
            })
        } else if tag.name() == b"BestSegmentColor" {
            color(reader, tag.into_buf(), |color| {
                settings.best_segment_color = color;
            })
        } else if tag.name() == b"NotRunningColor" {
            color(reader, tag.into_buf(), |color| {
                settings.not_running_color = color;
            })
        } else if tag.name() == b"PausedColor" {
            color(reader, tag.into_buf(), |color| {
                settings.paused_color = color;
            })
        } else if tag.name() == b"BackgroundType" {
            text_err(reader, tag.into_buf(), |text| {
                background_builder.kind = match &*text {
                    "SolidColor" => GradientKind::Plain,
                    "VerticalGradient" => GradientKind::Vertical,
                    "HorizontalGradient" => GradientKind::Horizontal,
                    "Image" => {
                        background_builder.first = Color::black();
                        background_builder.second = Color::black();
                        GradientKind::Plain
                    }
                    _ => return Err(Error::UnexpectedGradientType),
                };
                Ok(())
            })
        } else {
            end_tag(reader, tag.into_buf())
        }
    })?;

    settings.background = background_builder.build();

    Ok(())
}

pub fn parse<R: BufRead>(source: R) -> Result<Layout> {
    let reader = &mut Reader::from_reader(source);
    reader.expand_empty_elements(true);
    reader.trim_text(true);

    let mut buf = Vec::with_capacity(4096);

    let mut layout = Layout::new();

    parse_base(reader, &mut buf, b"Layout", |reader, tag| {
        parse_children(reader, tag.into_buf(), |reader, tag| {
            if tag.name() == b"Settings" {
                parse_general_settings(&mut layout, reader, tag.into_buf())
            } else if tag.name() == b"Components" {
                parse_children(reader, tag.into_buf(), |reader, tag| {
                    component(reader, tag.into_buf(), |c| {
                        layout.push(c);
                    })
                })
            } else {
                end_tag(reader, tag.into_buf())
            }
        })
    })?;

    if layout.components.is_empty() {
        Err(Error::Empty)
    } else {
        Ok(layout)
    }
}
