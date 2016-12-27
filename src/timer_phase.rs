#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TimerPhase {
    NotRunning,
    Running,
    Ended,
    Paused,
}
