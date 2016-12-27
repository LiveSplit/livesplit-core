#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TimingMethod {
    RealTime,
    GameTime,
}

impl TimingMethod {
    pub fn all() -> [TimingMethod; 2] {
        [TimingMethod::RealTime, TimingMethod::GameTime]
    }
}
