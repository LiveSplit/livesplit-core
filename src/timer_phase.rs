#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TimerPhase {
    NotRunning,
    Running,
    Ended,
    Paused,
}
