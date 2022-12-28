//! Provides the parser for layout files of the original LiveSplit.

use super::{Component, Layout, LayoutDirection};
use crate::{
    component::{separator, timer::DeltaGradient},
    platform::{math::f32::powf, prelude::*},
    settings::{
        Alignment, Color, Font, FontStretch, FontStyle, FontWeight, Gradient, ListGradient,
    },
    timing::{
        formatter::{Accuracy, DigitsFormat},
        TimingMethod,
    },
    util::xml::{
        helper::{
            end_tag, parse_base, parse_children, text, text_as_escaped_string_err, text_parsed,
            Error as XmlError,
        },
        Reader,
    },
};
use core::{mem::MaybeUninit, num::ParseIntError, str};

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

#[cfg(all(windows, feature = "std"))]
mod font_resolving;

// One single row component is:
// 1.0 units high in component space.
// 24 pixels high in LiveSplit One's pixel coordinate space.
// ~30.5 pixels high in the original LiveSplit's pixel coordinate space.
const PIXEL_SPACE_RATIO: f32 = 24.0 / 30.5;

fn translate_size(v: u32) -> u32 {
    (v as f32 * PIXEL_SPACE_RATIO + 0.5) as u32
}

/// The Error type for parsing layout files of the original LiveSplit.
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// The underlying XML format couldn't be parsed.
    Xml {
        /// The underlying error.
        source: XmlError,
    },
    /// Failed to parse an integer.
    ParseInt {
        /// The underlying error.
        source: ParseIntError,
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
    /// Failed to parse a font.
    ParseFont,
    /// Parsed an empty layout, which is considered an invalid layout.
    Empty,
}

impl From<XmlError> for Error {
    fn from(source: XmlError) -> Self {
        Self::Xml { source }
    }
}

impl From<ParseIntError> for Error {
    fn from(source: ParseIntError) -> Self {
        Self::ParseInt { source }
    }
}

/// The Result type for parsing layout files of the original LiveSplit.
pub type Result<T> = core::result::Result<T, Error>;

enum GradientKind {
    Transparent,
    Plain,
    Vertical,
    Horizontal,
}

enum DeltaGradientKind {
    Transparent,
    Plain,
    Vertical,
    Horizontal,
    PlainWithDeltaColor,
    VerticalWithDeltaColor,
    HorizontalWithDeltaColor,
}

enum ListGradientKind {
    Same(GradientKind),
    Alternating,
}

trait GradientType: Sized {
    type Built;
    fn default() -> Self;
    fn parse(kind: &str) -> Result<Self>;
    fn build(self, first: Color, second: Color) -> Self::Built;
}

impl GradientType for DeltaGradientKind {
    type Built = DeltaGradient;

    fn default() -> Self {
        DeltaGradientKind::Transparent
    }

    fn parse(kind: &str) -> Result<Self> {
        match kind {
            "Plain" => Ok(DeltaGradientKind::Plain),
            "PlainWithDeltaColor" => Ok(DeltaGradientKind::PlainWithDeltaColor),
            "Vertical" => Ok(DeltaGradientKind::Vertical),
            "VerticalWithDeltaColor" => Ok(DeltaGradientKind::VerticalWithDeltaColor),
            "Horizontal" => Ok(DeltaGradientKind::Horizontal),
            "HorizontalWithDeltaColor" => Ok(DeltaGradientKind::HorizontalWithDeltaColor),
            _ => Err(Error::ParseGradientType),
        }
    }

    fn build(self, first: Color, second: Color) -> Self::Built {
        match self {
            DeltaGradientKind::Transparent => Gradient::Transparent.into(),
            DeltaGradientKind::Plain => if first.alpha == 0.0 {
                Gradient::Transparent
            } else {
                Gradient::Plain(first)
            }
            .into(),
            DeltaGradientKind::Vertical => Gradient::Vertical(first, second).into(),
            DeltaGradientKind::Horizontal => Gradient::Horizontal(first, second).into(),
            DeltaGradientKind::PlainWithDeltaColor => DeltaGradient::DeltaPlain,
            DeltaGradientKind::VerticalWithDeltaColor => DeltaGradient::DeltaVertical,
            DeltaGradientKind::HorizontalWithDeltaColor => DeltaGradient::DeltaHorizontal,
        }
    }
}

impl GradientType for GradientKind {
    type Built = Gradient;
    fn default() -> Self {
        GradientKind::Transparent
    }
    fn parse(kind: &str) -> Result<Self> {
        Ok(match kind {
            "Plain" => GradientKind::Plain,
            "Vertical" => GradientKind::Vertical,
            "Horizontal" => GradientKind::Horizontal,
            _ => return Err(Error::ParseGradientType),
        })
    }
    fn build(self, first: Color, second: Color) -> Self::Built {
        match self {
            GradientKind::Transparent => Gradient::Transparent,
            GradientKind::Plain => {
                if first.alpha == 0.0 {
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
    fn parse(kind: &str) -> Result<Self> {
        Ok(if kind == "Alternating" {
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
    tag_color1: &'static str,
    tag_color2: &'static str,
    tag_kind: &'static str,
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
        Self::with_tags("BackgroundColor", "BackgroundColor2", "BackgroundGradient")
    }

    fn with_tags(
        tag_color1: &'static str,
        tag_color2: &'static str,
        tag_kind: &'static str,
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

    fn parse_background(&mut self, reader: &mut Reader<'_>, tag_name: &str) -> Result<bool> {
        if tag_name == self.tag_color1 {
            color(reader, |c| self.first = c)?;
        } else if tag_name == self.tag_color2 {
            color(reader, |c| self.second = c)?;
        } else if tag_name == self.tag_kind {
            text_as_escaped_string_err::<_, _, Error>(reader, |text| {
                self.kind = T::parse(text)?;
                Ok(())
            })?;
        } else {
            return Ok(false);
        }
        Ok(true)
    }

    fn build(self) -> T::Built {
        self.kind.build(self.first, self.second)
    }
}

fn color<F>(reader: &mut Reader<'_>, func: F) -> Result<()>
where
    F: FnOnce(Color),
{
    text_as_escaped_string_err(reader, |text| {
        let number = u32::from_str_radix(text, 16)?;
        let [a, r, g, b] = number.to_be_bytes();
        let mut color = Color::rgba8(r, g, b, a);
        let [r, g, b, a] = color.to_array();

        // Adjust alpha based on the lightness of the color. The formula is
        // based on two sRGB curves measured for white on top of a black
        // background and for black on top of a white background. We interpolate
        // between the two curves based on the lightness of the color. The
        // problem is that we only have the foreground color, so based on the
        // actual background color, this may be wrong. Therefore this is only a
        // heuristic. We often have white on dark grey, instead of white on
        // black. Because of that, we use 1.75 as the exponent denominator for
        // the white on black case instead of the usual 2.2 for sRGB.
        let lightness = (r + g + b) * (1.0 / 3.0);
        color.alpha =
            (1.0 - lightness) * (1.0 - powf(1.0 - a, 1.0 / 2.2)) + lightness * powf(a, 1.0 / 1.75);

        func(color);
        Ok(())
    })
}

fn font<F>(reader: &mut Reader<'_>, font_buf: &mut Vec<MaybeUninit<u8>>, f: F) -> Result<()>
where
    F: FnOnce(Font),
{
    text_as_escaped_string_err(reader, |text| {
        // The format for this is documented here:
        // https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-nrbf/75b9fe09-be15-475f-85b8-ae7b7558cfe5
        //
        // The structure follows roughly:
        //
        // class System.Drawing.Font {
        //     String Name;
        //     float Size;
        //     System.Drawing.FontStyle Style;
        //     System.Drawing.GraphicsUnit Unit;
        // }
        //
        // The full definition can be found here:
        // https://referencesource.microsoft.com/#System.Drawing/commonui/System/Drawing/Advanced/Font.cs,130

        let rem = text.as_bytes().get(304..).ok_or(Error::ParseFont)?;

        font_buf.resize(
            base64_simd::STANDARD.estimated_decoded_length(rem.len()),
            MaybeUninit::uninit(),
        );

        let decoded = base64_simd::STANDARD
            .decode(rem, base64_simd::Out::from_uninit_slice(font_buf))
            .map_err(|_| Error::ParseFont)?;

        let mut cursor = decoded.get(1..).ok_or(Error::ParseFont)?.iter();

        // Strings are encoded as varint for the length + the UTF-8 string data.
        let mut len = 0;
        for _ in 0..5 {
            let byte = *cursor.next().ok_or(Error::ParseFont)?;
            len = len << 7 | (byte & 0b0111_1111) as usize;
            if byte <= 0b0111_1111 {
                break;
            }
        }
        let rem = cursor.as_slice();

        let font_name = rem.get(..len).ok_or(Error::ParseFont)?;
        let original_family_name = simdutf8::basic::from_utf8(font_name)
            .map_err(|_| Error::ParseFont)?
            .trim();
        let mut family = original_family_name;

        let mut style = FontStyle::Normal;
        let mut weight = FontWeight::Normal;
        let mut stretch = FontStretch::Normal;

        // The original LiveSplit is based on Windows Forms, which is just a
        // .NET wrapper around GDI+. It's a pretty old API from before
        // DirectWrite existed, and fonts used to be very different back then.
        // This is why GDI uses a very different identifier for fonts than
        // modern APIs. Since all the modern APIs take a font family, we somehow
        // need to convert the font identifier from the original LiveSplit into
        // a font family. The problem is that we may not necessarily even have
        // the font, nor be on a platform where we could even query for any
        // fonts or get enough metadata about them, such as in the browser. So
        // for those cases, we implement a very simple, though also really lossy
        // algorithm that simply splits away common tokens at the end that refer
        // to the subfamily / styling of the font. In most cases this should
        // yield the font family that we are looking for and the additional
        // styling information. Another problem with this approach is that GDI
        // limits its font identifiers to 32 characters, so the tokens that we
        // may want to split off, may themselves already be cut off, causing us
        // to not recognize them. An example of this is "Bahnschrift SemiLight
        // SemiConde" where the end should say "SemiCondensed" but doesn't due
        // to the character limit.

        for token in family.split_whitespace().rev() {
            // FontWeight and FontStretch both have the variant "normal"
            // which is the default and can thus be ignored.
            if token.eq_ignore_ascii_case("italic") {
                style = FontStyle::Italic;
            } else if token.eq_ignore_ascii_case("thin") || token.eq_ignore_ascii_case("hairline") {
                weight = FontWeight::Thin;
            } else if token.eq_ignore_ascii_case("extralight")
                || token.eq_ignore_ascii_case("ultralight")
            {
                weight = FontWeight::ExtraLight;
            } else if token.eq_ignore_ascii_case("light") {
                weight = FontWeight::Light;
            } else if token.eq_ignore_ascii_case("semilight")
                || token.eq_ignore_ascii_case("demilight")
            {
                weight = FontWeight::SemiLight;
            } else if token.eq_ignore_ascii_case("medium") {
                weight = FontWeight::Medium;
            } else if token.eq_ignore_ascii_case("semibold")
                || token.eq_ignore_ascii_case("demibold")
            {
                weight = FontWeight::SemiBold;
            } else if token.eq_ignore_ascii_case("bold") {
                weight = FontWeight::Bold;
            } else if token.eq_ignore_ascii_case("extrabold")
                || token.eq_ignore_ascii_case("ultrabold")
            {
                weight = FontWeight::ExtraBold;
            } else if token.eq_ignore_ascii_case("black") || token.eq_ignore_ascii_case("heavy") {
                weight = FontWeight::Black;
            } else if token.eq_ignore_ascii_case("extrablack")
                || token.eq_ignore_ascii_case("ultrablack")
            {
                weight = FontWeight::ExtraBlack;
            } else if token.eq_ignore_ascii_case("ultracondensed") {
                stretch = FontStretch::UltraCondensed;
            } else if token.eq_ignore_ascii_case("extracondensed") {
                stretch = FontStretch::ExtraCondensed;
            } else if token.eq_ignore_ascii_case("condensed") {
                stretch = FontStretch::Condensed;
            } else if token.eq_ignore_ascii_case("semicondensed") {
                stretch = FontStretch::SemiCondensed;
            } else if token.eq_ignore_ascii_case("semiexpanded") {
                stretch = FontStretch::SemiExpanded;
            } else if token.eq_ignore_ascii_case("expanded") {
                stretch = FontStretch::Expanded;
            } else if token.eq_ignore_ascii_case("extraexpanded") {
                stretch = FontStretch::ExtraExpanded;
            } else if token.eq_ignore_ascii_case("ultraexpanded") {
                stretch = FontStretch::UltraExpanded;
            } else if !token.eq_ignore_ascii_case("regular") {
                family =
                    &family[..token.as_ptr() as usize - family.as_ptr() as usize + token.len()];
                break;
            }
        }

        // Later on we find the style and weight as bitflags of System.Drawing.FontStyle.
        // 1 -> bold
        // 2 -> italic
        // 4 -> underline
        // 8 -> strikeout
        let flags = *rem.get(len + 52).ok_or(Error::ParseFont)?;
        let (bold_flag, italic_flag) = (flags & 1 != 0, flags & 2 != 0);

        // If we are on Windows, we can however directly use GDI to get the
        // proper family name out of the font. The problem is that GDI does not
        // give us access to either the path of the font or its data. However we can
        // receive the byte representation of individual tables we query for, so
        // we can get the family name from the `name` table.

        #[cfg(all(windows, feature = "std"))]
        let family = if let Some(info) =
            font_resolving::FontInfo::from_gdi(original_family_name, bold_flag, italic_flag)
        {
            weight = match info.weight {
                i32::MIN..=149 => FontWeight::Thin,
                150..=249 => FontWeight::ExtraLight,
                250..=324 => FontWeight::Light,
                325..=374 => FontWeight::SemiLight,
                375..=449 => FontWeight::Normal,
                450..=549 => FontWeight::Medium,
                550..=649 => FontWeight::SemiBold,
                650..=749 => FontWeight::Bold,
                750..=849 => FontWeight::ExtraBold,
                850..=924 => FontWeight::Black,
                925.. => FontWeight::ExtraBlack,
            };
            style = if info.italic {
                FontStyle::Italic
            } else {
                FontStyle::Normal
            };
            info.family
        } else {
            family.to_owned()
        };

        #[cfg(not(all(windows, feature = "std")))]
        let family = family.to_owned();

        // The font might not exist on the user's system, so we still prefer to
        // apply these flags.

        if bold_flag && weight < FontWeight::Bold {
            weight = FontWeight::Bold;
        }

        if italic_flag {
            style = FontStyle::Italic;
        }

        f(Font {
            family,
            style,
            weight,
            stretch,
        });
        Ok(())
    })
}

fn parse_bool<F>(reader: &mut Reader<'_>, f: F) -> Result<()>
where
    F: FnOnce(bool),
{
    text_as_escaped_string_err(reader, |t| match t {
        "True" => {
            f(true);
            Ok(())
        }
        "False" => {
            f(false);
            Ok(())
        }
        _ => Err(Error::ParseBool),
    })
}

fn comparison_override<F>(reader: &mut Reader<'_>, f: F) -> Result<()>
where
    F: FnOnce(Option<String>),
{
    text(reader, |t| {
        f(if t == "Current Comparison" {
            None
        } else {
            Some(t.into_owned())
        })
    })
}

fn timing_method_override<F>(reader: &mut Reader<'_>, f: F) -> Result<()>
where
    F: FnOnce(Option<TimingMethod>),
{
    text_as_escaped_string_err(reader, |t| {
        f(match t {
            "Current Timing Method" => None,
            "Real Time" => Some(TimingMethod::RealTime),
            "Game Time" => Some(TimingMethod::GameTime),
            _ => return Err(Error::ParseTimingMethod),
        });
        Ok(())
    })
}

fn accuracy<F>(reader: &mut Reader<'_>, f: F) -> Result<()>
where
    F: FnOnce(Accuracy),
{
    text_as_escaped_string_err(reader, |t| {
        f(match t {
            "Tenths" => Accuracy::Tenths,
            "Seconds" => Accuracy::Seconds,
            "Hundredths" => Accuracy::Hundredths,
            _ => return Err(Error::ParseAccuracy),
        });
        Ok(())
    })
}

fn timer_format<F>(reader: &mut Reader<'_>, f: F) -> Result<()>
where
    F: FnOnce(DigitsFormat, Accuracy),
{
    text_as_escaped_string_err(reader, |t| {
        let (digits_format, accuracy) = t.split_once('.').unwrap_or((t, ""));
        let digits_format = match digits_format {
            "1" => DigitsFormat::SingleDigitSeconds,
            "00:01" => DigitsFormat::DoubleDigitMinutes,
            "0:00:01" => DigitsFormat::SingleDigitHours,
            "00:00:01" => DigitsFormat::DoubleDigitHours,
            _ => return Err(Error::ParseDigitsFormat),
        };
        let accuracy = match accuracy {
            "23" => Accuracy::Hundredths,
            "2" => Accuracy::Tenths,
            "" => Accuracy::Seconds,
            _ => return Err(Error::ParseAccuracy),
        };
        f(digits_format, accuracy);
        Ok(())
    })
}

fn component<F>(reader: &mut Reader<'_>, f: F) -> Result<()>
where
    F: FnOnce(Component),
{
    let mut component = None;

    parse_children(reader, |reader, tag, _| {
        match tag.name() {
            "Path" => text_as_escaped_string_err(reader, |text| {
                component = Some(match text {
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
            }),
            "Settings" => {
                // Assumption: Settings always has to come after the Path.
                // Otherwise we need to cache the settings and load them later.
                if let Some(component) = &mut component {
                    match component {
                        Component::BlankSpace(c) => blank_space::settings(reader, c),
                        Component::CurrentComparison(c) => current_comparison::settings(reader, c),
                        Component::CurrentPace(c) => current_pace::settings(reader, c),
                        Component::Delta(c) => delta::settings(reader, c),
                        Component::DetailedTimer(c) => detailed_timer::settings(reader, c),
                        Component::Graph(c) => graph::settings(reader, c),
                        Component::PbChance(c) => pb_chance::settings(reader, c),
                        Component::PossibleTimeSave(c) => possible_time_save::settings(reader, c),
                        Component::PreviousSegment(c) => previous_segment::settings(reader, c),
                        Component::SegmentTime(_) => end_tag(reader),
                        Component::Separator(_) => end_tag(reader),
                        Component::Splits(c) => splits::settings(reader, c),
                        Component::SumOfBest(c) => sum_of_best::settings(reader, c),
                        Component::Text(c) => text::settings(reader, c),
                        Component::Timer(c) => timer::settings(reader, c),
                        Component::Title(c) => title::settings(reader, c),
                        Component::TotalPlaytime(c) => total_playtime::settings(reader, c),
                    }
                } else {
                    end_tag(reader)
                }
            }
            _ => end_tag(reader),
        }
    })?;

    if let Some(component) = component {
        f(component);
    }

    Ok(())
}

fn parse_general_settings(layout: &mut Layout, reader: &mut Reader<'_>) -> Result<()> {
    let settings = layout.general_settings_mut();
    let mut background_builder = GradientBuilder::new();

    let mut font_buf = Vec::new();

    parse_children(reader, |reader, tag, _| match tag.name() {
        "TextColor" => color(reader, |color| {
            settings.text_color = color;
        }),
        "BackgroundColor" => color(reader, |color| {
            background_builder.first = color;
        }),
        "BackgroundColor2" => color(reader, |color| {
            background_builder.second = color;
        }),
        "ThinSeparatorsColor" => color(reader, |color| {
            settings.thin_separators_color = color;
        }),
        "SeparatorsColor" => color(reader, |color| {
            settings.separators_color = color;
        }),
        "PersonalBestColor" => color(reader, |color| {
            settings.personal_best_color = color;
        }),
        "AheadGainingTimeColor" => color(reader, |color| {
            settings.ahead_gaining_time_color = color;
        }),
        "AheadLosingTimeColor" => color(reader, |color| {
            settings.ahead_losing_time_color = color;
        }),
        "BehindGainingTimeColor" => color(reader, |color| {
            settings.behind_gaining_time_color = color;
        }),
        "BehindLosingTimeColor" => color(reader, |color| {
            settings.behind_losing_time_color = color;
        }),
        "BestSegmentColor" => color(reader, |color| {
            settings.best_segment_color = color;
        }),
        "NotRunningColor" => color(reader, |color| {
            settings.not_running_color = color;
        }),
        "PausedColor" => color(reader, |color| {
            settings.paused_color = color;
        }),
        "TimerFont" => font(reader, &mut font_buf, |font| {
            if font.family != "Calibri" && font.family != "Century Gothic" {
                settings.timer_font = Some(font);
            }
        }),
        "TimesFont" => font(reader, &mut font_buf, |font| {
            if font.family != "Segoe UI" {
                settings.times_font = Some(font);
            }
        }),
        "TextFont" => font(reader, &mut font_buf, |font| {
            if font.family != "Segoe UI" {
                settings.text_font = Some(font);
            }
        }),
        "BackgroundType" => text_as_escaped_string_err(reader, |text| {
            background_builder.kind = match text {
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
        }),
        _ => end_tag(reader),
    })?;

    settings.background = background_builder.build();

    Ok(())
}

/// Attempts to parse a layout file of the original LiveSplit. They are only
/// parsed on a best effort basis, so if something isn't supported by
/// livesplit-core, then it will be parsed without that option.
pub fn parse(source: &str) -> Result<Layout> {
    let reader = &mut Reader::new(source);

    let mut layout = Layout::new();

    parse_base(reader, "Layout", |reader, _| {
        parse_children(reader, |reader, tag, _| match tag.name() {
            "Mode" => text_as_escaped_string_err(reader, |text| {
                layout.general_settings_mut().direction = match text {
                    "Vertical" => LayoutDirection::Vertical,
                    "Horizontal" => LayoutDirection::Horizontal,
                    _ => return Err(Error::ParseLayoutDirection),
                };
                Ok(())
            }),
            "Settings" => parse_general_settings(&mut layout, reader),
            "Components" => parse_children(reader, |reader, _, _| {
                component(reader, |c| {
                    layout.push(c);
                })
            }),
            _ => end_tag(reader),
        })
    })?;

    if layout.components.is_empty() {
        Err(Error::Empty)
    } else {
        Ok(layout)
    }
}
