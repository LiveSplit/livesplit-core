use crate::TimingMethod;

/// Describes which phase the timer is currently in. This tells you if there's
/// an active speedrun attempt and whether it is paused or it ended.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[repr(u8)]
pub enum TimerPhase {
    /// There's currently no active attempt.
    NotRunning = 0,
    /// There's an active attempt that didn't end yet and isn't paused.
    Running = 1,
    /// There's an attempt that already ended, but didn't get reset yet.
    Ended = 2,
    /// There's an active attempt that is currently paused.
    Paused = 3,
}

impl TimerPhase {
    /// Returns [`true`] if the value is [`TimerPhase::NotRunning`].
    pub const fn is_not_running(&self) -> bool {
        matches!(self, Self::NotRunning)
    }

    /// Returns [`true`] if the value is [`TimerPhase::Running`].
    pub const fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }

    /// Returns [`true`] if the value is [`TimerPhase::Ended`].
    pub const fn is_ended(&self) -> bool {
        matches!(self, Self::Ended)
    }

    /// Returns [`true`] if the value is [`TimerPhase::Paused`].
    pub const fn is_paused(&self) -> bool {
        matches!(self, Self::Paused)
    }

    /// Returns [`true`] if the timer is currently in a phase where it updates
    /// frequently. This means that the timer is [`TimerPhase::Running`] or
    /// [`TimerPhase::Paused`] and the timing method is not
    /// [`TimingMethod::RealTime`].
    pub(crate) const fn updates_frequently(&self, method: TimingMethod) -> bool {
        match self {
            Self::Running => true,
            Self::Paused => matches!(method, TimingMethod::GameTime),
            _ => false,
        }
    }
}
