use crate::palette::{Hsla, LinSrgba, Pixel};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Colors can be used to describe what color to use for visualizing
/// backgrounds, texts, lines and various other elements that are being shown.
/// They are stored as RGBA colors with 32-bit float point numbers ranging from
/// 0.0 to 1.0 per channel.
#[derive(Debug, Copy, Clone, PartialEq, derive_more::From)]
pub struct Color {
    /// The Red, Green, Blue, Alpha (RGBA) encoding of the color.
    pub rgba: LinSrgba,
}

impl Color {
    /// Creates a new transparent color.
    pub fn transparent() -> Self {
        (0.0, 0.0, 0.0, 0.0).into()
    }

    /// Creates a new white color.
    pub fn white() -> Self {
        (1.0, 1.0, 1.0, 1.0).into()
    }

    /// Creates a new black color.
    pub fn black() -> Self {
        (0.0, 0.0, 0.0, 1.0).into()
    }

    /// Creates a new color by providing the Hue, Saturation, Lightness and
    /// Alpha (HSLA) for it.
    pub fn hsla(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Self {
        Self {
            rgba: Hsla::new(hue, saturation, lightness, alpha).into(),
        }
    }
}

impl From<[f32; 4]> for Color {
    fn from(rgba: [f32; 4]) -> Self {
        Self {
            rgba: LinSrgba::from_components((rgba[0], rgba[1], rgba[2], rgba[3])),
        }
    }
}

impl From<[u8; 4]> for Color {
    fn from(rgba: [u8; 4]) -> Self {
        Self {
            rgba: LinSrgba::from_raw(&rgba).into_format(),
        }
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(rgba: (f32, f32, f32, f32)) -> Self {
        Self {
            rgba: LinSrgba::from_components(rgba),
        }
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (r, g, b, a) = self.rgba.into_components();
        let rgba: [f32; 4] = [r, g, b, a];
        rgba.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rgba = <[f32; 4]>::deserialize(deserializer)?;
        Ok(rgba.into())
    }
}
