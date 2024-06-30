use std::fmt;

/// Represents the state that a timer is in.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub enum TimerState {
    /// The timer is not running.
    #[default]
    NotRunning = 0,
    /// The timer is running.
    Running = 1,
    /// The timer started but got paused. This is separate from the game time
    /// being paused. Game time may even always be paused.
    Paused = 2,
    /// The timer has ended, but didn't get reset yet.
    Ended = 3,
}

/// A timer that can be controlled by an auto splitter.
pub trait Timer {
    /// Returns the current state of the timer.
    fn state(&self) -> TimerState;
    /// Starts the timer.
    fn start(&mut self);
    /// Splits the current segment.
    fn split(&mut self);
    /// Skips the current split.
    fn skip_split(&mut self);
    /// Undoes the previous split.
    fn undo_split(&mut self);
    /// Resets the timer.
    fn reset(&mut self);
    /// Accesses the index of the split the attempt is currently on. If there's
    /// no attempt in progress, `None` is returned instead. This returns an
    /// index that is equal to the amount of segments when the attempt is
    /// finished, but has not been reset. So you need to be careful when using
    /// this value for indexing.
    fn current_split_index(&self) -> Option<usize>;
    /// Sets the game time.
    fn set_game_time(&mut self, time: time::Duration);
    /// Pauses the game time. This does not pause the timer, only the automatic
    /// flow of time for the game time.
    fn pause_game_time(&mut self);
    /// Resumes the game time. This does not resume the timer, only the
    /// automatic flow of time for the game time.
    fn resume_game_time(&mut self);
    /// Sets a custom key value pair. This may be arbitrary information that the
    /// auto splitter wants to provide for visualization.
    fn set_variable(&mut self, key: &str, value: &str);
    /// Logs a message either from the auto splitter directly or from the
    /// runtime.
    fn log(&mut self, message: fmt::Arguments<'_>);
}
