use layout;
use super::Color;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SemanticColor {
    Default,
    AheadGainingTime,
    AheadLosingTime,
    BehindLosingTime,
    BehindGainingTime,
    BestSegment,
    NotRunning,
    Paused,
    PersonalBest,
}

impl SemanticColor {
    pub fn or(self, color: SemanticColor) -> SemanticColor {
        if self == SemanticColor::Default {
            color
        } else {
            self
        }
    }

    pub fn visualize(&self, settings: &layout::GeneralSettings) -> Color {
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
