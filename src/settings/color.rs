use crate::platform::math::f32::abs;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// [`Colors`](Color) can be used to describe what [`Color`] to use for
/// visualizing backgrounds, texts, lines and various other elements that are
/// being shown. They are stored as RGBA [`Colors`](Color) with 32-bit floating
/// point numbers ranging from 0.0 to 1.0 per channel.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Color {
    /// The red component (0 - 1) of the [`Color`].
    pub red: f32,
    /// The green component (0 - 1) of the [`Color`].
    pub green: f32,
    /// The blue component (0 - 1) of the [`Color`].
    pub blue: f32,
    /// The alpha component (0 - 1) of the [`Color`].
    pub alpha: f32,
}

impl Color {
    /// Creates a new [`Color`] from red (0 - 1), green (0 - 1), blue (0 - 1)
    /// and alpha (0 - 1) components.
    pub const fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Creates a new [`Color`] from red, green, blue and alpha byte components.
    pub fn rgba8(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        const RECIP_255: f32 = 1.0 / 255.0;

        Self {
            red: RECIP_255 * red as f32,
            green: RECIP_255 * green as f32,
            blue: RECIP_255 * blue as f32,
            alpha: RECIP_255 * alpha as f32,
        }
    }

    /// Converts the [`Color`] into red, green, blue and alpha byte components.
    pub fn to_rgba8(&self) -> [u8; 4] {
        [
            (255.0 * self.red + 0.5) as u8,
            (255.0 * self.green + 0.5) as u8,
            (255.0 * self.blue + 0.5) as u8,
            (255.0 * self.alpha + 0.5) as u8,
        ]
    }

    /// Creates a new transparent [`Color`].
    pub const fn transparent() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 0.0,
        }
    }

    /// Creates a new white [`Color`].
    pub const fn white() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }

    /// Creates a new black [`Color`].
    pub const fn black() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    /// Converts the [`Color`] into an array of red (0 - 1), green (0 - 1), blue
    /// (0 - 1) and alpha (0 - 1).
    pub const fn to_array(&self) -> [f32; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }

    /// Creates a new [`Color`] by providing the hue (0 - 360), saturation (0 -
    /// 1), lightness (0 - 1) and alpha (0 - 1) for it.
    pub fn hsla(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Self {
        const RECIP_60: f32 = 1.0 / 60.0;

        let c = (1.0 - abs(2.0 * lightness - 1.0)) * saturation;
        let x = c * (1.0 - abs((hue * RECIP_60) % 2.0 - 1.0));
        let m = lightness - 0.5 * c;

        let (red, green, blue) = if hue < 60.0 {
            (m + c, m + x, m)
        } else if hue < 120.0 {
            (m + x, m + c, m)
        } else if hue < 180.0 {
            (m, m + c, m + x)
        } else if hue < 240.0 {
            (m, m + x, m + c)
        } else if hue < 300.0 {
            (m + x, m, m + c)
        } else {
            (m + c, m, m + x)
        };

        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Creates a new [`Color`] by providing the hue (0 - 360), saturation (0 -
    /// 1), value (0 - 1) and alpha (0 - 1) for it.
    pub fn hsva(hue: f32, saturation: f32, value: f32, alpha: f32) -> Self {
        const RECIP_60: f32 = 1.0 / 60.0;

        let c = value * saturation;
        let x = c * (1.0 - abs((hue * RECIP_60) % 2.0 - 1.0));
        let m = value - c;

        let (red, green, blue) = if hue < 60.0 {
            (m + c, m + x, m)
        } else if hue < 120.0 {
            (m + x, m + c, m)
        } else if hue < 180.0 {
            (m, m + c, m + x)
        } else if hue < 240.0 {
            (m, m + x, m + c)
        } else if hue < 300.0 {
            (m + x, m, m + c)
        } else {
            (m + c, m, m + x)
        };

        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Converts the [`Color`] into hue (0 - 360), saturation (0 - 1), value (0
    /// - 1) and alpha (0 - 1).
    pub fn to_hsva(&self) -> [f32; 4] {
        let [r, g, b, a] = self.to_array();

        let (shift, max, diff, min) = if r > g {
            if r > b {
                if b > g {
                    (360.0, r, g - b, g)
                } else {
                    (0.0, r, g - b, b)
                }
            } else {
                (240.0, b, r - g, g)
            }
        } else if b > g {
            (240.0, b, r - g, r)
        } else {
            (120.0, g, b - r, if r > b { b } else { r })
        };

        let delta = max - min;

        let hue = if delta == 0.0 {
            0.0
        } else {
            60.0 * diff / delta + shift
        };
        let saturation = if max == 0.0 { 0.0 } else { delta / max };
        let value = max;

        [hue, saturation, value, a]
    }
}

impl From<[f32; 4]> for Color {
    fn from([red, green, blue, alpha]: [f32; 4]) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl From<Color> for [f32; 4] {
    fn from(c: Color) -> Self {
        c.to_array()
    }
}

impl From<[u8; 4]> for Color {
    fn from([red, green, blue, alpha]: [u8; 4]) -> Self {
        Self::rgba8(red, green, blue, alpha)
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_array().serialize(serializer)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::float_cmp)]
    #[test]
    fn to_hsva() {
        assert_eq!(
            Color::from([1.0, 0.0, 0.0, 1.0]).to_hsva(),
            [0.0, 1.0, 1.0, 1.0],
        );
        assert_eq!(
            Color::from([1.0, 1.0, 0.0, 1.0]).to_hsva(),
            [60.0, 1.0, 1.0, 1.0],
        );
        assert_eq!(
            Color::from([0.0, 1.0, 0.0, 1.0]).to_hsva(),
            [120.0, 1.0, 1.0, 1.0],
        );
        assert_eq!(
            Color::from([0.0, 1.0, 1.0, 1.0]).to_hsva(),
            [180.0, 1.0, 1.0, 1.0],
        );
        assert_eq!(
            Color::from([0.0, 0.0, 1.0, 1.0]).to_hsva(),
            [240.0, 1.0, 1.0, 1.0],
        );
        assert_eq!(
            Color::from([1.0, 0.0, 1.0, 1.0]).to_hsva(),
            [300.0, 1.0, 1.0, 1.0],
        );
    }

    #[test]
    fn from_hsva() {
        assert_eq!(Color::hsva(0.0, 1.0, 1.0, 1.0).to_rgba8(), [255, 0, 0, 255]);
        assert_eq!(
            Color::hsva(60.0, 1.0, 1.0, 1.0).to_rgba8(),
            [255, 255, 0, 255],
        );
        assert_eq!(
            Color::hsva(120.0, 1.0, 1.0, 1.0).to_rgba8(),
            [0, 255, 0, 255],
        );
        assert_eq!(
            Color::hsva(180.0, 1.0, 1.0, 1.0).to_rgba8(),
            [0, 255, 255, 255],
        );
        assert_eq!(
            Color::hsva(240.0, 1.0, 1.0, 1.0).to_rgba8(),
            [0, 0, 255, 255],
        );
        assert_eq!(
            Color::hsva(300.0, 1.0, 1.0, 1.0).to_rgba8(),
            [255, 0, 255, 255],
        );
    }
}
