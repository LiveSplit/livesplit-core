/// This is a mirror of [`livesplit_core::TimerPhase`](https://docs.rs/livesplit-core/0.11.0/livesplit_core/enum.TimerPhase.html)
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TimerState {
    NotRunning = 0,
    Running = 1,
    Paused = 2,
    Finished = 3,
}

/// This interface allows the autosplitter to live outside of livesplit-core and
/// enables testing with a dummy timer implementation
pub trait Timer {
    fn state(&self) -> TimerState;
    fn start(&mut self);
    fn split(&mut self);
    fn reset(&mut self);
    fn set_game_time(&mut self, time: time::Duration);
    fn pause_game_time(&mut self);
    fn resume_game_time(&mut self);
    fn set_variable(&mut self, key: &str, value: &str);
}
