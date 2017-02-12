#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum TimerPhase {
    NotRunning = 0,
    Running = 1,
    Ended = 2,
    Paused = 3,
}
