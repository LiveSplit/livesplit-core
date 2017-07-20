use palette::{Rgba, Hsla};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use super::GeneralSettings;
use SemanticColor;

#[derive(Copy, Clone, PartialEq, From)]
pub struct Color {
    pub rgba: Rgba<f32>,
}

impl Color {
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

impl SemanticColor {
    pub fn visualize(&self, settings: &GeneralSettings) -> Color {
        match *self {
            SemanticColor::Default => settings.text_color,
            SemanticColor::AheadGainingTime => settings.ahead_gaining_time_color,
            SemanticColor::AheadLosingTime => settings.ahead_losing_time_color,
            SemanticColor::BehindLosingTime => settings.behind_losing_time_color,
            SemanticColor::BehindGainingTime => settings.behind_gaining_time_color,
            SemanticColor::BestSegment => settings.best_segment_color,
            SemanticColor::NotRunning => settings.not_running_color,
            SemanticColor::Paused => settings.paused_color,
            SemanticColor::PersonalBest => settings.personal_best_color,
        }
    }
}
