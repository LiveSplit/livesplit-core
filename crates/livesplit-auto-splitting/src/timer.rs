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

/// The level of criticalness of a log message.
pub enum LogLevel {
    /// A trace message. This is the least critical and most verbose message.
    Trace,
    /// A debug message. This is a message that is useful for debugging.
    Debug,
    /// An info message. This is a message that provides information.
    Info,
    /// A warning message. This is a message that warns about something that
    /// may be problematic.
    Warning,
    /// An error message. This is a message that indicates an error.
    Error,
}

/// A timer that can be controlled by an auto splitter.
pub trait Timer: Send {
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
    /// Logs a message from the auto splitter.
    fn log_auto_splitter(&mut self, message: fmt::Arguments<'_>);
    /// Logs a message from the runtime.
    fn log_runtime(&mut self, message: fmt::Arguments<'_>, log_level: LogLevel);
}
