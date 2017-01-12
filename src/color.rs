#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Color {
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

impl Color {
    pub fn or(self, color: Color) -> Color {
        if self == Color::Default { color } else { self }
    }
}
