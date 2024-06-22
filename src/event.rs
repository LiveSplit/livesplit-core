//! The event module provides functionality for forwarding commands to the
//! timer. The commands usually come from the hotkey system, an auto splitter,
//! the UI, or through a network connection. The UI usually provides the
//! implementation for this, forwarding all the commands to the actual timer. It
//! is able to intercept the commands and for example ask the user for
//! confirmation before applying them. Other handling is possible such as
//! automatically saving the splits or notifying a server about changes
//! happening in the run. After processing a command, changes to the timer are
//! reported as [`Event`]s. Various [`Error`] conditions can occur if the
//! command couldn't be processed.

use core::{future::Future, ops::Deref};

use alloc::sync::Arc;

use crate::{TimeSpan, Timer, TimingMethod};

/// An event informs you about a change in the timer.
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Hash, serde_derive::Serialize, serde_derive::Deserialize,
)]
#[non_exhaustive]
pub enum Event {
    /// The timer has been started.
    Started = 0,
    /// A split happened. Note that the final split is signaled by
    /// [`Finished`].
    Splitted = 1,
    /// The final split happened, the run is now finished, but has not been
    /// reset yet.
    Finished = 2,
    /// The timer has been reset.
    Reset = 3,
    /// The previous split has been undone.
    SplitUndone = 4,
    /// The current split has been skipped.
    SplitSkipped = 5,
    /// The timer has been paused.
    Paused = 6,
    /// The timer has been resumed.
    Resumed = 7,
    /// All the pauses have been undone.
    PausesUndone = 8,
    /// All the pauses have been undone and the timer has been resumed.
    PausesUndoneAndResumed = 9,
    /// The comparison has been changed.
    ComparisonChanged = 10,
    /// The timing method has been changed.
    TimingMethodChanged = 11,
    /// The game time has been initialized.
    GameTimeInitialized = 12,
    /// The game time has been set.
    GameTimeSet = 13,
    /// The game time has been paused.
    GameTimePaused = 14,
    /// The game time has been resumed.
    GameTimeResumed = 15,
    /// The loading times have been set.
    LoadingTimesSet = 16,
    /// A custom variable has been set.
    CustomVariableSet = 17,
}

impl TryFrom<u32> for Event {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Event::Started,
            1 => Event::Splitted,
            2 => Event::Finished,
            3 => Event::Reset,
            4 => Event::SplitUndone,
            5 => Event::SplitSkipped,
            6 => Event::Paused,
            7 => Event::Resumed,
            8 => Event::PausesUndone,
            9 => Event::PausesUndoneAndResumed,
            10 => Event::ComparisonChanged,
            11 => Event::TimingMethodChanged,
            12 => Event::GameTimeInitialized,
            13 => Event::GameTimeSet,
            14 => Event::GameTimePaused,
            15 => Event::GameTimeResumed,
            16 => Event::LoadingTimesSet,
            17 => Event::CustomVariableSet,
            _ => return Err(()),
        })
    }
}

/// An error that occurred when a command was being processed.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    snafu::Snafu,
    serde_derive::Serialize,
    serde_derive::Deserialize,
)]
#[snafu(context(suffix(false)))]
#[non_exhaustive]
pub enum Error {
    /// The operation is not supported.
    Unsupported = 0,
    /// The timer can't be interacted with at the moment.
    Busy = 1,
    /// There is already a run in progress.
    RunAlreadyInProgress = 2,
    /// There is no run in progress.
    NoRunInProgress = 3,
    /// The run is already finished.
    RunFinished = 4,
    /// The time is negative, you can't split yet.
    NegativeTime = 5,
    /// The last split can't be skipped.
    CantSkipLastSplit = 6,
    /// There is no split to undo.
    CantUndoFirstSplit = 7,
    /// The timer is already paused.
    AlreadyPaused = 8,
    /// The timer is not paused.
    NotPaused = 9,
    /// The requested comparison doesn't exist.
    ComparisonDoesntExist = 10,
    /// The game time is already initialized.
    GameTimeAlreadyInitialized = 11,
    /// The game time is already paused.
    GameTimeAlreadyPaused = 12,
    /// The game time is not paused.
    GameTimeNotPaused = 13,
    /// The time could not be parsed.
    CouldNotParseTime = 14,
    /// The timer is currently paused.
    TimerPaused = 15,
    /// The runner decided to not reset the run.
    RunnerDecidedAgainstReset = 16,
    /// An unknown error occurred.
    #[serde(other)]
    Unknown,
}

impl From<u32> for Error {
    fn from(value: u32) -> Self {
        match value {
            0 => Error::Unsupported,
            1 => Error::Busy,
            2 => Error::RunAlreadyInProgress,
            3 => Error::NoRunInProgress,
            4 => Error::RunFinished,
            5 => Error::NegativeTime,
            6 => Error::CantSkipLastSplit,
            7 => Error::CantUndoFirstSplit,
            8 => Error::AlreadyPaused,
            9 => Error::NotPaused,
            10 => Error::ComparisonDoesntExist,
            11 => Error::GameTimeAlreadyInitialized,
            12 => Error::GameTimeAlreadyPaused,
            13 => Error::GameTimeNotPaused,
            14 => Error::CouldNotParseTime,
            15 => Error::TimerPaused,
            16 => Error::RunnerDecidedAgainstReset,
            _ => Error::Unknown,
        }
    }
}

/// The result of a command that was processed.
pub type Result<T = Event, E = Error> = core::result::Result<T, E>;

/// A command sink accepts commands that are meant to be passed to the timer.
/// The commands usually come from the hotkey system, an auto splitter, the UI,
/// or through a network connection. The UI usually provides the implementation
/// for this, forwarding all the commands to the actual timer. It is able to
/// intercept the commands and for example ask the user for confirmation before
/// applying them. Other handling is possible such as automatically saving the
/// splits or notifying a server about changes happening in the run. After
/// processing a command, changes to the timer are reported as [`Event`]s.
/// Various [`Error`] conditions can occur if the command couldn't be processed.
///
/// # Asynchronous Events
///
/// The events or the errors are returned asynchronously. This allows for
/// handling commands that may take some time to complete. However, there are
/// various sources of these commands such as the hotkey system and the auto
/// splitters that do not care about the result of each command. They
/// immediately drop the returned future. This means that the command sink needs
/// to be implemented in a way such that the commands reach their destination,
/// even if the future is never being polled.
pub trait CommandSink {
    /// Starts the timer if there is no attempt in progress. If that's not the
    /// case, nothing happens.
    fn start(&self) -> impl Future<Output = Result> + 'static;
    /// If an attempt is in progress, stores the current time as the time of the
    /// current split. The attempt ends if the last split time is stored.
    fn split(&self) -> impl Future<Output = Result> + 'static;
    /// Starts a new attempt or stores the current time as the time of the
    /// current split. The attempt ends if the last split time is stored.
    fn split_or_start(&self) -> impl Future<Output = Result> + 'static;
    /// Resets the current attempt if there is one in progress. If the splits
    /// are to be updated, all the information of the current attempt is stored
    /// in the run's history. Otherwise the current attempt's information is
    /// discarded.
    fn reset(&self, save_attempt: Option<bool>) -> impl Future<Output = Result> + 'static;
    /// Removes the split time from the last split if an attempt is in progress
    /// and there is a previous split. The Timer Phase also switches to
    /// [`Running`](TimerPhase::Running) if it previously was
    /// [`Ended`](TimerPhase::Ended).
    fn undo_split(&self) -> impl Future<Output = Result> + 'static;
    /// Skips the current split if an attempt is in progress and the
    /// current split is not the last split.
    fn skip_split(&self) -> impl Future<Output = Result> + 'static;
    /// Toggles an active attempt between [`Paused`](TimerPhase::Paused) and
    /// [`Running`](TimerPhase::Paused) or starts an attempt if there's none in
    /// progress.
    fn toggle_pause_or_start(&self) -> impl Future<Output = Result> + 'static;
    /// Pauses an active attempt that is not paused.
    fn pause(&self) -> impl Future<Output = Result> + 'static;
    /// Resumes an attempt that is paused.
    fn resume(&self) -> impl Future<Output = Result> + 'static;
    /// Removes all the pause times from the current time. If the current
    /// attempt is paused, it also resumes that attempt. Additionally, if the
    /// attempt is finished, the final split time is adjusted to not include the
    /// pause times as well.
    ///
    /// # Warning
    ///
    /// This behavior is not entirely optimal, as generally only the final split
    /// time is modified, while all other split times are left unmodified, which
    /// may not be what actually happened during the run.
    fn undo_all_pauses(&self) -> impl Future<Output = Result> + 'static;
    /// Switches the current comparison to the previous comparison in the list.
    fn switch_to_previous_comparison(&self) -> impl Future<Output = Result> + 'static;
    /// Switches the current comparison to the next comparison in the list.
    fn switch_to_next_comparison(&self) -> impl Future<Output = Result> + 'static;
    /// Tries to set the current comparison to the comparison specified. If the
    /// comparison doesn't exist an error is returned.
    fn set_current_comparison(&self, comparison: &str) -> impl Future<Output = Result> + 'static;
    /// Toggles between the `Real Time` and `Game Time` timing methods.
    fn toggle_timing_method(&self) -> impl Future<Output = Result> + 'static;
    /// Sets the current timing method to the timing method provided.
    fn set_current_timing_method(
        &self,
        method: TimingMethod,
    ) -> impl Future<Output = Result> + 'static;
    /// Initializes game time for the current attempt. Game time automatically
    /// gets uninitialized for each new attempt.
    fn initialize_game_time(&self) -> impl Future<Output = Result> + 'static;
    /// Sets the game time to the time specified. This also works if the game
    /// time is paused, which can be used as a way of updating the game timer
    /// periodically without it automatically moving forward. This ensures that
    /// the game timer never shows any time that is not coming from the game.
    fn set_game_time(&self, time: TimeSpan) -> impl Future<Output = Result> + 'static;
    /// Pauses the game timer such that it doesn't automatically increment
    /// similar to real time.
    fn pause_game_time(&self) -> impl Future<Output = Result> + 'static;
    /// Resumes the game timer such that it automatically increments similar to
    /// real time, starting from the game time it was paused at.
    fn resume_game_time(&self) -> impl Future<Output = Result> + 'static;
    /// Instead of setting the game time directly, this method can be used to
    /// just specify the amount of time the game has been loading. The game time
    /// is then automatically determined by Real Time - Loading Times.
    fn set_loading_times(&self, time: TimeSpan) -> impl Future<Output = Result> + 'static;
    /// Sets the value of a custom variable with the name specified. If the
    /// variable does not exist, a temporary variable gets created that will not
    /// be stored in the splits file.
    fn set_custom_variable(
        &self,
        name: &str,
        value: &str,
    ) -> impl Future<Output = Result> + 'static;
}

/// This trait provides functionality for querying information from the timer.
pub trait TimerQuery {
    /// The timer can be protected by a guard. This could be a lock guard for
    /// example.
    type Guard<'a>: 'a + Deref<Target = Timer>
    where
        Self: 'a;
    /// Accesses the timer to query information from it.
    fn get_timer(&self) -> Self::Guard<'_>;
}

#[cfg(feature = "std")]
impl CommandSink for crate::SharedTimer {
    fn start(&self) -> impl Future<Output = Result> + 'static {
        let result = self.write().unwrap().start();
        async move { result }
    }

    fn split(&self) -> impl Future<Output = Result> + 'static {
        let result = self.write().unwrap().split();
        async move { result }
    }

    fn split_or_start(&self) -> impl Future<Output = Result> + 'static {
        let result = self.write().unwrap().split_or_start();
        async move { result }
    }

    fn reset(&self, save_attempt: Option<bool>) -> impl Future<Output = Result> + 'static {
        let result = self.write().unwrap().reset(save_attempt != Some(false));
        async move { result }
    }

    fn undo_split(&self) -> impl Future<Output = Result> + 'static {
        let result = self.write().unwrap().undo_split();
        async move { result }
    }

    fn skip_split(&self) -> impl Future<Output = Result> + 'static {
        let result = self.write().unwrap().skip_split();
        async move { result }
    }

    fn toggle_pause_or_start(&self) -> impl Future<Output = Result> + 'static {
        let result = self.write().unwrap().toggle_pause_or_start();
        async move { result }
    }

    fn pause(&self) -> impl Future<Output = Result> + 'static {
        let result = self.write().unwrap().pause();
        async move { result }
    }

    fn resume(&self) -> impl Future<Output = Result> + 'static {
        let result = self.write().unwrap().resume();
        async move { result }
    }

    fn undo_all_pauses(&self) -> impl Future<Output = Result> + 'static {
        let result = self.write().unwrap().undo_all_pauses();
        async move { result }
    }

    fn switch_to_previous_comparison(&self) -> impl Future<Output = Result> + 'static {
        self.write().unwrap().switch_to_previous_comparison();
        async { Ok(Event::ComparisonChanged) }
    }

    fn switch_to_next_comparison(&self) -> impl Future<Output = Result> + 'static {
        self.write().unwrap().switch_to_next_comparison();
        async { Ok(Event::ComparisonChanged) }
    }

    fn set_current_comparison(&self, comparison: &str) -> impl Future<Output = Result> + 'static {
        let result = self.write().unwrap().set_current_comparison(comparison);
        async move { result }
    }

    fn toggle_timing_method(&self) -> impl Future<Output = Result> + 'static {
        self.write().unwrap().toggle_timing_method();
        async { Ok(Event::TimingMethodChanged) }
    }

    fn set_current_timing_method(
        &self,
        method: TimingMethod,
    ) -> impl Future<Output = Result> + 'static {
        self.write().unwrap().set_current_timing_method(method);
        async { Ok(Event::TimingMethodChanged) }
    }

    fn initialize_game_time(&self) -> impl Future<Output = Result> + 'static {
        let result = self.write().unwrap().initialize_game_time();
        async move { result }
    }

    fn set_game_time(&self, time: TimeSpan) -> impl Future<Output = Result> + 'static {
        let result = self.write().unwrap().set_game_time(time);
        async move { result }
    }

    fn pause_game_time(&self) -> impl Future<Output = Result> + 'static {
        let result = self.write().unwrap().pause_game_time();
        async move { result }
    }

    fn resume_game_time(&self) -> impl Future<Output = Result> + 'static {
        let result = self.write().unwrap().resume_game_time();
        async move { result }
    }

    fn set_loading_times(&self, time: TimeSpan) -> impl Future<Output = Result> + 'static {
        let result = self.write().unwrap().set_loading_times(time);
        async move { result }
    }

    fn set_custom_variable(
        &self,
        name: &str,
        value: &str,
    ) -> impl Future<Output = Result> + 'static {
        self.write().unwrap().set_custom_variable(name, value);
        async { Ok(Event::CustomVariableSet) }
    }
}

#[cfg(feature = "std")]
impl TimerQuery for crate::SharedTimer {
    type Guard<'a> = std::sync::RwLockReadGuard<'a, Timer>;
    fn get_timer(&self) -> Self::Guard<'_> {
        self.read().unwrap()
    }
}

impl<T: CommandSink + ?Sized> CommandSink for Arc<T> {
    fn start(&self) -> impl Future<Output = Result> + 'static {
        CommandSink::start(&**self)
    }

    fn split(&self) -> impl Future<Output = Result> + 'static {
        CommandSink::split(&**self)
    }

    fn split_or_start(&self) -> impl Future<Output = Result> + 'static {
        CommandSink::split_or_start(&**self)
    }

    fn reset(&self, save_attempt: Option<bool>) -> impl Future<Output = Result> + 'static {
        CommandSink::reset(&**self, save_attempt)
    }

    fn undo_split(&self) -> impl Future<Output = Result> + 'static {
        CommandSink::undo_split(&**self)
    }

    fn skip_split(&self) -> impl Future<Output = Result> + 'static {
        CommandSink::skip_split(&**self)
    }

    fn toggle_pause_or_start(&self) -> impl Future<Output = Result> + 'static {
        CommandSink::toggle_pause_or_start(&**self)
    }

    fn pause(&self) -> impl Future<Output = Result> + 'static {
        CommandSink::pause(&**self)
    }

    fn resume(&self) -> impl Future<Output = Result> + 'static {
        CommandSink::resume(&**self)
    }

    fn undo_all_pauses(&self) -> impl Future<Output = Result> + 'static {
        CommandSink::undo_all_pauses(&**self)
    }

    fn switch_to_previous_comparison(&self) -> impl Future<Output = Result> + 'static {
        CommandSink::switch_to_previous_comparison(&**self)
    }

    fn switch_to_next_comparison(&self) -> impl Future<Output = Result> + 'static {
        CommandSink::switch_to_next_comparison(&**self)
    }

    fn set_current_comparison(&self, comparison: &str) -> impl Future<Output = Result> + 'static {
        CommandSink::set_current_comparison(&**self, comparison)
    }

    fn toggle_timing_method(&self) -> impl Future<Output = Result> + 'static {
        CommandSink::toggle_timing_method(&**self)
    }

    fn set_current_timing_method(
        &self,
        method: TimingMethod,
    ) -> impl Future<Output = Result> + 'static {
        CommandSink::set_current_timing_method(&**self, method)
    }

    fn initialize_game_time(&self) -> impl Future<Output = Result> + 'static {
        CommandSink::initialize_game_time(&**self)
    }

    fn set_game_time(&self, time: TimeSpan) -> impl Future<Output = Result> + 'static {
        CommandSink::set_game_time(&**self, time)
    }

    fn pause_game_time(&self) -> impl Future<Output = Result> + 'static {
        CommandSink::pause_game_time(&**self)
    }

    fn resume_game_time(&self) -> impl Future<Output = Result> + 'static {
        CommandSink::resume_game_time(&**self)
    }

    fn set_loading_times(&self, time: TimeSpan) -> impl Future<Output = Result> + 'static {
        CommandSink::set_loading_times(&**self, time)
    }

    fn set_custom_variable(
        &self,
        name: &str,
        value: &str,
    ) -> impl Future<Output = Result> + 'static {
        CommandSink::set_custom_variable(&**self, name, value)
    }
}

impl<T: TimerQuery + ?Sized> TimerQuery for Arc<T> {
    type Guard<'a> = T::Guard<'a> where T: 'a;
    fn get_timer(&self) -> Self::Guard<'_> {
        TimerQuery::get_timer(&**self)
    }
}
