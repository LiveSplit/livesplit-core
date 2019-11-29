//! Provides the parser for layout files of the original LiveSplit.

use super::{Component, Layout, LayoutDirection};
use crate::{
    component::separator,
    settings::{Alignment, Color, Gradient, ListGradient},
    timing::{
        formatter::{Accuracy, DigitsFormat},
        TimingMethod,
    },
    xml_util::{
        end_tag, parse_base, parse_children, text, text_as_escaped_bytes_err, text_err,
        text_parsed, Error as XmlError, Tag,
    },
};
use quick_xml::Reader;
use std::io::BufRead;

mod blank_space;
mod current_comparison;
mod current_pace;
mod delta;
mod detailed_timer;
mod graph;
mod pb_chance;
mod possible_time_save;
mod previous_segment;
mod splits;
mod sum_of_best;
mod text;
mod timer;
mod title;
mod total_playtime;

/// The Error type for parsing layout files of the original LiveSplit.
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
    ParseInt {
        /// The underlying error.
        source: core::num::ParseIntError,
    },
    /// Failed to parse a boolean.
    ParseBool,
    /// Failed to parse the layout direction.
    ParseLayoutDirection,
    /// Failed to parse a gradient type.
    ParseGradientType,
    /// Failed to parse an accuracy.
    ParseAccuracy,
    /// Failed to parse a digits format.
    ParseDigitsFormat,
    /// Failed to parse a timing method.
    ParseTimingMethod,
    /// Failed to parse an alignment.
    ParseAlignment,
    /// Failed to parse a column type.
    ParseColumnType,
    /// Parsed an empty layout, which is considered an invalid layout.
    Empty,
}

/// The Result type for parsing layout files of the original LiveSplit.
pub type Result<T> = core::result::Result<T, Error>;

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
            _ => return Err(Error::ParseGradientType),
        })
    }
    fn build(self, first: Color, second: Color) -> Self::Built {
        match self {
            GradientKind::Transparent => Gradient::Transparent,
            GradientKind::Plain => {
                if first.rgba.alpha == 0.0 {
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
            text_as_escaped_bytes_err::<_, _, _, Error>(reader, tag.into_buf(), |text| {
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

fn color<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, func: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(Color),
{
    text_err(reader, buf, |text| {
        let number = u32::from_str_radix(&text, 16)?;
        let [a, r, g, b] = number.to_be_bytes();
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

        func(color);
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
        _ => Err(Error::ParseBool),
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
            _ => return Err(Error::ParseTimingMethod),
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
            _ => return Err(Error::ParseAccuracy),
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
            _ => return Err(Error::ParseDigitsFormat),
        };
        let accuracy = match splits.next().unwrap_or(b"") {
            b"23" => Accuracy::Hundredths,
            b"2" => Accuracy::Tenths,
            b"" => Accuracy::Seconds,
            _ => return Err(Error::ParseAccuracy),
        };
        f(digits_format, accuracy);
        Ok(())
    })
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
                    "PBChance.dll" => pb_chance::Component::new().into(),
                    "LiveSplit.PossibleTimeSave.dll" => possible_time_save::Component::new().into(),
                    "LiveSplit.PreviousSegment.dll" => previous_segment::Component::new().into(),
                    "" => separator::Component::new().into(),
                    "LiveSplit.Splits.dll" | "LiveSplit.Subsplits.dll" => {
                        splits::Component::new().into()
                    }
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
                    Component::BlankSpace(c) => blank_space::settings(reader, tag.into_buf(), c),
                    Component::CurrentComparison(c) => {
                        current_comparison::settings(reader, tag.into_buf(), c)
                    }
                    Component::CurrentPace(c) => current_pace::settings(reader, tag.into_buf(), c),
                    Component::Delta(c) => delta::settings(reader, tag.into_buf(), c),
                    Component::DetailedTimer(c) => {
                        detailed_timer::settings(reader, tag.into_buf(), c)
                    }
                    Component::Graph(c) => graph::settings(reader, tag.into_buf(), c),
                    Component::PbChance(c) => pb_chance::settings(reader, tag.into_buf(), c),
                    Component::PossibleTimeSave(c) => {
                        possible_time_save::settings(reader, tag.into_buf(), c)
                    }
                    Component::PreviousSegment(c) => {
                        previous_segment::settings(reader, tag.into_buf(), c)
                    }
                    Component::SegmentTime(_) => end_tag(reader, tag.into_buf()),
                    Component::Separator(_) => end_tag(reader, tag.into_buf()),
                    Component::Splits(c) => splits::settings(reader, tag.into_buf(), c),
                    Component::SumOfBest(c) => sum_of_best::settings(reader, tag.into_buf(), c),
                    Component::Text(c) => text::settings(reader, tag.into_buf(), c),
                    Component::Timer(c) => timer::settings(reader, tag.into_buf(), c),
                    Component::Title(c) => title::settings(reader, tag.into_buf(), c),
                    Component::TotalPlaytime(c) => {
                        total_playtime::settings(reader, tag.into_buf(), c)
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
                    _ => return Err(Error::ParseGradientType),
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

/// Attempts to parse a layout file of the original LiveSplit. They are only
/// parsed on a best effort basis, so if something isn't supported by
/// livesplit-core, then it will be parsed without that option.
pub fn parse<R: BufRead>(source: R) -> Result<Layout> {
    let reader = &mut Reader::from_reader(source);
    reader.expand_empty_elements(true);
    reader.trim_text(true);

    let mut buf = Vec::with_capacity(4096);

    let mut layout = Layout::new();

    parse_base(reader, &mut buf, b"Layout", |reader, tag| {
        parse_children(reader, tag.into_buf(), |reader, tag| {
            if tag.name() == b"Mode" {
                text_err(reader, tag.into_buf(), |text| {
                    layout.general_settings_mut().direction = match &*text {
                        "Vertical" => LayoutDirection::Vertical,
                        "Horizontal" => LayoutDirection::Horizontal,
                        _ => return Err(Error::ParseLayoutDirection),
                    };
                    Ok(())
                })
            } else if tag.name() == b"Settings" {
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
