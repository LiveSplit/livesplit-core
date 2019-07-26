use serde::{Deserialize, Serialize};

/// A Timing Method describes which form of timing is used. This can either be
/// Real Time or Game Time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[repr(u8)]
pub enum TimingMethod {
    /// Real Time is the unmodified timing that is as close to an atomic clock
    /// as possible.
    RealTime = 0,
    /// Game Time describes the timing that is provided by the game that is
    /// being run. This is entirely optional and may either be Real Time with
    /// loading times removed or some time provided by the game.
    GameTime = 1,
}

impl TimingMethod {
    /// Returns an array of all the timing methods.
    pub fn all() -> [TimingMethod; 2] {
        [TimingMethod::RealTime, TimingMethod::GameTime]
    }
}
