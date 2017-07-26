#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[repr(u8)]
pub enum TimingMethod {
    RealTime = 0,
    GameTime = 1,
}

impl TimingMethod {
    pub fn all() -> [TimingMethod; 2] {
        [TimingMethod::RealTime, TimingMethod::GameTime]
    }
}
