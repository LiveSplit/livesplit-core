use crate::platform::prelude::*;
use serde::{Deserialize, Serialize};

/// Describes a Font to visualize text with. Depending on the platform, a font
/// that matches the settings most closely is chosen. The settings may be
/// ignored entirely if the platform can't support different fonts, such as in a
/// terminal.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Font {
    /// The family name of the font to use. This corresponds with the
    /// `Typographic Family Name` (Name ID 16) in the name table of the font. If
    /// no such entry exists, the `Font Family Name` (Name ID 1) is to be used
    /// instead. If there are multiple entries for the name, the english entry
    /// is the one to choose. The subfamily is not specified at all, and instead
    /// a suitable subfamily is chosen based on the style, weight and stretch
    /// values.
    ///
    /// [`name — Naming Table` on Microsoft
    /// Docs](https://docs.microsoft.com/en-us/typography/opentype/spec/name)
    ///
    /// This is to ensure the highest portability across various platforms.
    /// Platforms often select fonts very differently, so if necessary it is
    /// also fine to store a different font identifier here at the cost of
    /// sacrificing portability.
    pub family: String,
    /// The style of the font to prefer selecting.
    pub style: Style,
    /// The weight of the font to prefer selecting.
    pub weight: Weight,
    /// The stretch of the font to prefer selecting.
    pub stretch: Stretch,
}

/// The style specifies whether to use a normal or italic version of a font. The
/// style may be emulated if no font dedicated to the style can be found.
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Style {
    /// Select a regular, non-italic version of the font.
    #[default]
    Normal,
    /// Select an italic version of the font.
    Italic,
}

impl Style {
    /// The value to assign to the `ital` variation axis.
    pub const fn value_for_italic(self) -> f32 {
        match self {
            Style::Normal => 0.0,
            Style::Italic => 1.0,
        }
    }
}

/// The weight specifies the weight / boldness of a font. If there is no font
/// with the exact weight value, a font with a similar weight is to be chosen
/// based on an algorithm similar to this:
///
/// [`Fallback weights` on
/// MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight#Fallback_weights)
#[derive(
    Debug, Copy, Clone, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "kebab-case")]
pub enum Weight {
    /// 100 (also known as Hairline)
    Thin,
    /// 200 (also known as Ultra Light)
    ExtraLight,
    /// 300
    Light,
    /// 350 (also known as Demi Light)
    SemiLight,
    /// 400 (also known as Regular)
    #[default]
    Normal,
    /// 500
    Medium,
    /// 600 (also known as Demi Bold)
    SemiBold,
    /// 700
    Bold,
    /// 800 (also known as Ultra Bold)
    ExtraBold,
    /// 900 (also known as Heavy)
    Black,
    /// 950 (also known as Ultra Black)
    ExtraBlack,
}

impl Weight {
    /// The numeric value of the weight.
    pub const fn to_u16(self) -> u16 {
        match self {
            Weight::Thin => 100,
            Weight::ExtraLight => 200,
            Weight::Light => 300,
            Weight::SemiLight => 350,
            Weight::Normal => 400,
            Weight::Medium => 500,
            Weight::SemiBold => 600,
            Weight::Bold => 700,
            Weight::ExtraBold => 800,
            Weight::Black => 900,
            Weight::ExtraBlack => 950,
        }
    }

    /// The numeric value of the weight.
    pub const fn to_f32(self) -> f32 {
        match self {
            Weight::Thin => 100.0,
            Weight::ExtraLight => 200.0,
            Weight::Light => 300.0,
            Weight::SemiLight => 350.0,
            Weight::Normal => 400.0,
            Weight::Medium => 500.0,
            Weight::SemiBold => 600.0,
            Weight::Bold => 700.0,
            Weight::ExtraBold => 800.0,
            Weight::Black => 900.0,
            Weight::ExtraBlack => 950.0,
        }
    }
}

/// The stretch specifies how wide a font should be. For example, it may make
/// sense to reduce the stretch of a font to ensure split names are not cut off.
/// A font with a stretch value that is close is to be selected.
///
/// [`Font face selection` on
/// MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/font-stretch#Font_face_selection)
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum Stretch {
    /// 50%
    UltraCondensed,
    /// 62.5%
    ExtraCondensed,
    /// 75%
    Condensed,
    /// 87.5%
    SemiCondensed,
    /// 100%
    #[default]
    Normal,
    /// 112.5%
    SemiExpanded,
    /// 125%
    Expanded,
    /// 150%
    ExtraExpanded,
    /// 200%
    UltraExpanded,
}

impl Stretch {
    /// The percentage the font is stretched by (50% to 200%).
    pub const fn percentage(self) -> f32 {
        match self {
            Stretch::UltraCondensed => 50.0,
            Stretch::ExtraCondensed => 62.5,
            Stretch::Condensed => 75.0,
            Stretch::SemiCondensed => 87.5,
            Stretch::Normal => 100.0,
            Stretch::SemiExpanded => 112.5,
            Stretch::Expanded => 125.0,
            Stretch::ExtraExpanded => 150.0,
            Stretch::UltraExpanded => 200.0,
        }
    }

    /// The factor the font is stretched by (0x to 2x).
    pub const fn factor(self) -> f32 {
        match self {
            Stretch::UltraCondensed => 0.5,
            Stretch::ExtraCondensed => 0.625,
            Stretch::Condensed => 0.75,
            Stretch::SemiCondensed => 0.875,
            Stretch::Normal => 1.0,
            Stretch::SemiExpanded => 1.125,
            Stretch::Expanded => 1.25,
            Stretch::ExtraExpanded => 1.5,
            Stretch::UltraExpanded => 2.0,
        }
    }
}
