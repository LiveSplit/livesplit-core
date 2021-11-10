use std::time::Duration;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TimerState {
    NotRunning = 0,
    Running = 1,
    Paused = 2,
    Finished = 3,
}

pub trait Timer: 'static {
    fn state(&self) -> TimerState;
    fn start(&mut self);
    fn split(&mut self);
    fn reset(&mut self);
    fn set_game_time(&mut self, time: Duration);
    fn pause_game_time(&mut self);
    fn resume_game_time(&mut self);
    fn set_variable(&mut self, key: &str, value: &str);
}
