use palette::{Hsla, Rgba};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Copy, Clone, PartialEq, From)]
pub struct Color {
    pub rgba: Rgba<f32>,
}

impl Color {
    pub fn transparent() -> Self {
        (0.0, 0.0, 0.0, 0.0).into()
    }

    pub fn hsla(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Self {
        Self {
            rgba: Hsla::new(hue.into(), saturation, lightness, alpha).into(),
        }
    }
}

impl From<[f32; 4]> for Color {
    fn from(rgba: [f32; 4]) -> Self {
        Self {
            rgba: Rgba::from_pixel(&rgba),
        }
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(rgba: (f32, f32, f32, f32)) -> Self {
        Self {
            rgba: Rgba::from_pixel(&rgba),
        }
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let rgba: [f32; 4] = self.rgba.to_pixel();
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
