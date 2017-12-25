/// Describes which phase the timer is currently in. This tells you if there's
/// an active speedrun attempt and whether it is paused or it ended.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
