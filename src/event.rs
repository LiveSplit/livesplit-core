//! The `event` module provides functionality for forwarding events to the
//! timer. The events usually come from the hotkey system, an auto splitter, the
//! UI, or through a network connection. The UI usually provides the
//! implementation for this, forwarding all the events to the actual timer. It
//! is able to intercept the events and for example ask the user for
//! confirmation before applying them. Other handling is possible such as
//! automatically saving the splits or notifying a server about changes
//! happening in the run.

use alloc::sync::Arc;

use crate::{TimeSpan, TimerPhase};

/// An event sink accepts events that are meant to be passed to the timer. The
/// events usually come from the hotkey system, an auto splitter, the UI, or
/// through a network connection. The UI usually provides the implementation for
/// this, forwarding all the events to the actual timer. It is able to intercept
/// the events and for example ask the user for confirmation before applying
/// them. Other handling is possible such as automatically saving the splits or
/// notifying a server about changes happening in the run.
pub trait Sink {
    /// Starts the timer if there is no attempt in progress. If that's not the
    /// case, nothing happens.
    fn start(&self);
    /// If an attempt is in progress, stores the current time as the time of the
    /// current split. The attempt ends if the last split time is stored.
    fn split(&self);
    /// Starts a new attempt or stores the current time as the time of the
    /// current split. The attempt ends if the last split time is stored.
    fn split_or_start(&self);
    /// Resets the current attempt if there is one in progress. If the splits
    /// are to be updated, all the information of the current attempt is stored
    /// in the run's history. Otherwise the current attempt's information is
    /// discarded.
    fn reset(&self, save_attempt: Option<bool>);
    /// Removes the split time from the last split if an attempt is in progress
    /// and there is a previous split. The Timer Phase also switches to
    /// [`Running`](TimerPhase::Running) if it previously was
    /// [`Ended`](TimerPhase::Ended).
    fn undo_split(&self);
    /// Skips the current split if an attempt is in progress and the
    /// current split is not the last split.
    fn skip_split(&self);
    /// Toggles an active attempt between [`Paused`](TimerPhase::Paused) and
    /// [`Running`](TimerPhase::Paused) or starts an attempt if there's none in
    /// progress.
    fn toggle_pause_or_start(&self);
    /// Pauses an active attempt that is not paused.
    fn pause(&self);
    /// Resumes an attempt that is paused.
    fn resume(&self);
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
    fn undo_all_pauses(&self);
    /// Switches the current comparison to the previous comparison in the list.
    fn switch_to_previous_comparison(&self);
    /// Switches the current comparison to the next comparison in the list.
    fn switch_to_next_comparison(&self);
    /// Toggles between the `Real Time` and `Game Time` timing methods.
    fn toggle_timing_method(&self);
    /// Sets the game time to the time specified. This also works if the game
    /// time is paused, which can be used as a way of updating the game timer
    /// periodically without it automatically moving forward. This ensures that
    /// the game timer never shows any time that is not coming from the game.
    fn set_game_time(&self, time: TimeSpan);
    /// Pauses the game timer such that it doesn't automatically increment
    /// similar to real time.
    fn pause_game_time(&self);
    /// Resumes the game timer such that it automatically increments similar to
    /// real time, starting from the game time it was paused at.
    fn resume_game_time(&self);
    /// Sets the value of a custom variable with the name specified. If the
    /// variable does not exist, a temporary variable gets created that will not
    /// be stored in the splits file.
    fn set_custom_variable(&self, name: &str, value: &str);
}

/// This trait provides functionality for querying the current state of the
/// timer.
pub trait TimerQuery {
    /// Returns the current Timer Phase.
    fn current_phase(&self) -> TimerPhase;
}

#[cfg(feature = "std")]
impl Sink for crate::SharedTimer {
    fn start(&self) {
        self.write().unwrap().start();
    }

    fn split(&self) {
        self.write().unwrap().split();
    }

    fn split_or_start(&self) {
        self.write().unwrap().split_or_start();
    }

    fn reset(&self, save_attempt: Option<bool>) {
        self.write().unwrap().reset(save_attempt != Some(false));
    }

    fn undo_split(&self) {
        self.write().unwrap().undo_split();
    }

    fn skip_split(&self) {
        self.write().unwrap().skip_split();
    }

    fn toggle_pause_or_start(&self) {
        self.write().unwrap().toggle_pause_or_start();
    }

    fn pause(&self) {
        self.write().unwrap().pause();
    }

    fn resume(&self) {
        self.write().unwrap().resume();
    }

    fn undo_all_pauses(&self) {
        self.write().unwrap().undo_all_pauses();
    }

    fn switch_to_previous_comparison(&self) {
        self.write().unwrap().switch_to_previous_comparison();
    }

    fn switch_to_next_comparison(&self) {
        self.write().unwrap().switch_to_next_comparison();
    }

    fn toggle_timing_method(&self) {
        self.write().unwrap().toggle_timing_method();
    }

    fn set_game_time(&self, time: TimeSpan) {
        self.write().unwrap().set_game_time(time);
    }

    fn pause_game_time(&self) {
        self.write().unwrap().pause_game_time();
    }

    fn resume_game_time(&self) {
        self.write().unwrap().resume_game_time();
    }

    fn set_custom_variable(&self, name: &str, value: &str) {
        self.write().unwrap().set_custom_variable(name, value);
    }
}

#[cfg(feature = "std")]
impl TimerQuery for crate::SharedTimer {
    fn current_phase(&self) -> TimerPhase {
        self.read().unwrap().current_phase()
    }
}

impl<T: Sink + ?Sized> Sink for Arc<T> {
    fn start(&self) {
        Sink::start(&**self)
    }

    fn split(&self) {
        Sink::split(&**self)
    }

    fn split_or_start(&self) {
        Sink::split_or_start(&**self)
    }

    fn reset(&self, save_attempt: Option<bool>) {
        Sink::reset(&**self, save_attempt)
    }

    fn undo_split(&self) {
        Sink::undo_split(&**self)
    }

    fn skip_split(&self) {
        Sink::skip_split(&**self)
    }

    fn toggle_pause_or_start(&self) {
        Sink::toggle_pause_or_start(&**self)
    }

    fn pause(&self) {
        Sink::pause(&**self)
    }

    fn resume(&self) {
        Sink::resume(&**self)
    }

    fn undo_all_pauses(&self) {
        Sink::undo_all_pauses(&**self)
    }

    fn switch_to_previous_comparison(&self) {
        Sink::switch_to_previous_comparison(&**self)
    }

    fn switch_to_next_comparison(&self) {
        Sink::switch_to_next_comparison(&**self)
    }

    fn toggle_timing_method(&self) {
        Sink::toggle_timing_method(&**self)
    }

    fn set_game_time(&self, time: TimeSpan) {
        Sink::set_game_time(&**self, time)
    }

    fn pause_game_time(&self) {
        Sink::pause_game_time(&**self)
    }

    fn resume_game_time(&self) {
        Sink::resume_game_time(&**self)
    }

    fn set_custom_variable(&self, name: &str, value: &str) {
        Sink::set_custom_variable(&**self, name, value)
    }
}

impl<T: TimerQuery + ?Sized> TimerQuery for Arc<T> {
    fn current_phase(&self) -> TimerPhase {
        TimerQuery::current_phase(&**self)
    }
}
