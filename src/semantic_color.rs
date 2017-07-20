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
}
