use super::Color;
use crate::layout;
use serde::{Deserialize, Serialize};

/// A Semantic Color describes a color by some meaningful event that is
/// happening. This information can be visualized as a color, but can also be
/// interpreted in other ways by the consumer of this API.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SemanticColor {
    /// There's no meaningful information for this color.
    Default,
    /// The runner is ahead of the comparison and is gaining even more
    /// time.
    AheadGainingTime,
    /// The runner is ahead of the comparison, but is losing time.
    AheadLosingTime,
    /// The runner is behind the comparison and is losing even more time.
    BehindLosingTime,
    /// The runner is behind the comparison, but is gaining back time.
    BehindGainingTime,
    /// The runner achieved a best segment.
    BestSegment,
    /// There's no active attempt.
    NotRunning,
    /// The timer is paused.
    Paused,
    /// The runner achieved a new Personal Best.
    PersonalBest,
}

impl Default for SemanticColor {
    fn default() -> SemanticColor {
        SemanticColor::Default
    }
}

impl SemanticColor {
    /// Replaces a Semantic Color by the Semantic Color provided if it is the
    /// default one.
    pub fn or(self, color: SemanticColor) -> SemanticColor {
        if self == SemanticColor::Default {
            color
        } else {
            self
        }
    }

    /// The General Settings store actual Color values for each of the different
    /// events. Using this method, you can use these to convert a Semantic Color
    /// to an actual Color.
    pub fn visualize(self, settings: &layout::GeneralSettings) -> Color {
        match self {
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
