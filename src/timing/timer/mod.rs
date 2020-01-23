use crate::comparison::personal_best;
use crate::platform::prelude::*;
use crate::TimerPhase::*;
use crate::{AtomicDateTime, Run, Segment, Time, TimeSpan, TimeStamp, TimerPhase, TimingMethod};
use core::mem;

#[cfg(test)]
mod tests;

/// A Timer provides all the capabilities necessary for doing speedrun attempts.
///
/// # Examples
///
/// ```
/// use livesplit_core::{Run, Segment, Timer, TimerPhase};
///
/// // Create a run object that we can use with at least one segment.
/// let mut run = Run::new();
/// run.set_game_name("Super Mario Odyssey");
/// run.set_category_name("Any%");
/// run.push_segment(Segment::new("Cap Kingdom"));
///
/// // Create the timer from the run.
/// let mut timer = Timer::new(run).expect("Run with at least one segment provided");
///
/// // Start a new attempt.
/// timer.start();
/// assert_eq!(timer.current_phase(), TimerPhase::Running);
///
/// // Create a split.
/// timer.split();
///
/// // The run should be finished now.
/// assert_eq!(timer.current_phase(), TimerPhase::Ended);
///
/// // Reset the attempt and confirm that we want to store the attempt.
/// timer.reset(true);
///
/// // The attempt is now over.
/// assert_eq!(timer.current_phase(), TimerPhase::NotRunning);
/// ```
#[derive(Debug, Clone)]
pub struct Timer {
    run: Run,
    phase: TimerPhase,
    current_split_index: Option<usize>,
    current_timing_method: TimingMethod,
    current_comparison: String,
    attempt_started: Option<AtomicDateTime>,
    attempt_ended: Option<AtomicDateTime>,
    start_time: TimeStamp,
    start_time_with_offset: TimeStamp,
    // This gets adjusted after resuming
    adjusted_start_time: TimeStamp,
    time_paused_at: TimeSpan,
    is_game_time_paused: bool,
    game_time_pause_time: Option<TimeSpan>,
    loading_times: Option<TimeSpan>,
}

/// A Shared Timer is a wrapper around the Timer that can be shared across
/// multiple threads with multiple owners.
#[cfg(feature = "std")]
pub type SharedTimer = alloc::sync::Arc<parking_lot::RwLock<Timer>>;

/// The Error type for creating a new Timer from a Run.
#[derive(Debug, snafu::Snafu)]
pub enum CreationError {
    /// The Timer couldn't be created, because the Run has no segments.
    EmptyRun,
}

impl Timer {
    /// Creates a new Timer based on a Run object storing all the information
    /// about the splits. The Run object needs to have at least one segment, so
    /// that the Timer can store the final time. If a Run object with no
    /// segments is provided, the Timer creation fails.
    #[inline]
    pub fn new(mut run: Run) -> Result<Self, CreationError> {
        if run.is_empty() {
            return Err(CreationError::EmptyRun);
        }

        run.fix_splits();
        run.regenerate_comparisons();
        let now = TimeStamp::now();

        Ok(Timer {
            run,
            phase: NotRunning,
            current_split_index: None,
            current_timing_method: TimingMethod::RealTime,
            current_comparison: personal_best::NAME.into(),
            attempt_started: None,
            attempt_ended: None,
            start_time: now,
            start_time_with_offset: now,
            adjusted_start_time: now,
            time_paused_at: TimeSpan::zero(),
            is_game_time_paused: false,
            game_time_pause_time: None,
            loading_times: None,
        })
    }

    /// Consumes the Timer and creates a Shared Timer that can be shared across
    /// multiple threads with multiple owners.
    #[cfg(feature = "std")]
    pub fn into_shared(self) -> SharedTimer {
        alloc::sync::Arc::new(parking_lot::RwLock::new(self))
    }

    /// Takes out the Run from the Timer and resets the current attempt if there
    /// is one in progress. If the splits are to be updated, all the information
    /// of the current attempt is stored in the Run's history. Otherwise the
    /// current attempt's information is discarded.
    pub fn into_run(mut self, update_splits: bool) -> Run {
        self.reset(update_splits);
        self.run
    }

    /// Replaces the Run object used by the Timer with the Run object provided.
    /// If the Run provided contains no segments, it can't be used for timing
    /// and is returned as the `Err` case of the `Result`. Otherwise the Run
    /// that was in use by the Timer is being returned. Before the Run is
    /// returned, the current attempt is reset and the splits are being updated
    /// depending on the `update_splits` parameter.
    pub fn replace_run(&mut self, mut run: Run, update_splits: bool) -> Result<Run, Run> {
        if run.is_empty() {
            return Err(run);
        }

        self.reset(update_splits);
        if !run.comparisons().any(|c| c == self.current_comparison) {
            self.current_comparison = personal_best::NAME.to_string();
        }

        run.fix_splits();
        run.regenerate_comparisons();

        Ok(mem::replace(&mut self.run, run))
    }

    /// Sets the Run object used by the Timer with the Run object provided. If
    /// the Run provided contains no segments, it can't be used for timing and
    /// is returned as the Err case of the Result. The Run object in use by the
    /// Timer is dropped by this method.
    pub fn set_run(&mut self, run: Run) -> Result<(), Run> {
        self.replace_run(run, false).map(drop)
    }

    /// Accesses the Run in use by the Timer.
    #[inline]
    pub fn run(&self) -> &Run {
        &self.run
    }

    /// Marks the Run as unmodified, so that it is known that all the changes
    /// have been saved.
    #[inline]
    pub fn mark_as_unmodified(&mut self) {
        self.run.mark_as_unmodified();
    }

    /// Returns the current Timer Phase.
    #[inline]
    pub fn current_phase(&self) -> TimerPhase {
        self.phase
    }

    /// Returns the current time of the Timer. The Game Time is None if the
    /// Game Time has not been initialized.
    pub fn current_time(&self) -> Time {
        let real_time = match self.phase {
            NotRunning => Some(self.run.offset()),
            Running => Some(TimeStamp::now() - self.adjusted_start_time),
            Paused => Some(self.time_paused_at),
            Ended => self.run.segments().last().unwrap().split_time().real_time,
        };

        let game_time = match self.phase {
            NotRunning => Some(self.run.offset()),
            Ended => self.run.segments().last().unwrap().split_time().game_time,
            _ => {
                if self.is_game_time_paused() {
                    self.game_time_pause_time
                } else if self.is_game_time_initialized() {
                    catch! { real_time? - self.loading_times() }
                } else {
                    None
                }
            }
        };

        Time::new()
            .with_real_time(real_time)
            .with_game_time(game_time)
    }

    /// Returns the currently selected Timing Method.
    #[inline]
    pub fn current_timing_method(&self) -> TimingMethod {
        self.current_timing_method
    }

    /// Sets the current Timing Method to the Timing Method provided.
    #[inline]
    pub fn set_current_timing_method(&mut self, method: TimingMethod) {
        self.current_timing_method = method;
    }

    /// Toggles between the `Real Time` and `Game Time` timing methods.
    #[inline]
    pub fn toggle_timing_method(&mut self) {
        self.current_timing_method = match self.current_timing_method {
            TimingMethod::RealTime => TimingMethod::GameTime,
            TimingMethod::GameTime => TimingMethod::RealTime,
        };
    }

    /// Returns the current comparison that is being compared against. This may
    /// be a custom comparison or one of the Comparison Generators.
    #[inline]
    pub fn current_comparison(&self) -> &str {
        &self.current_comparison
    }

    /// Tries to set the current comparison to the comparison specified. If the
    /// comparison doesn't exist `Err` is returned.
    #[inline]
    pub fn set_current_comparison<S: AsRef<str>>(&mut self, comparison: S) -> Result<(), ()> {
        let comparison = comparison.as_ref();
        if self.run.comparisons().any(|c| c == comparison) {
            self.current_comparison.clear();
            self.current_comparison.push_str(comparison);
            Ok(())
        } else {
            Err(())
        }
    }

    /// Accesses the split the attempt is currently on. If there's no attempt in
    /// progress or the run finished, `None` is returned instead.
    pub fn current_split(&self) -> Option<&Segment> {
        self.current_split_index
            .and_then(|i| self.run.segments().get(i))
    }

    fn current_split_mut(&mut self) -> Option<&mut Segment> {
        self.current_split_index
            .and_then(move |i| self.run.segments_mut().get_mut(i))
    }

    /// Accesses the index of the split the attempt is currently on. If there's
    /// no attempt in progress, `None` is returned instead. This returns an
    /// index that is equal to the amount of segments when the attempt is
    /// finished, but has not been reset. So you need to be careful when using
    /// this value for indexing.
    #[inline]
    pub fn current_split_index(&self) -> Option<usize> {
        self.current_split_index
    }

    /// Starts the Timer if there is no attempt in progress. If that's not the
    /// case, nothing happens.
    pub fn start(&mut self) {
        if self.phase == NotRunning {
            self.phase = Running;
            self.current_split_index = Some(0);
            self.attempt_started = Some(AtomicDateTime::now());
            self.start_time = TimeStamp::now();
            self.start_time_with_offset = self.start_time - self.run.offset();
            self.adjusted_start_time = self.start_time_with_offset;
            self.time_paused_at = self.run.offset();
            self.deinitialize_game_time();
            self.run.start_next_run();

            // FIXME: OnStart
        }
    }

    /// If an attempt is in progress, stores the current time as the time of the
    /// current split. The attempt ends if the last split time is stored.
    pub fn split(&mut self) {
        let current_time = self.current_time();
        if self.phase == Running
            && current_time
                .real_time
                .map_or(false, |t| t >= TimeSpan::zero())
        {
            self.current_split_mut()
                .unwrap()
                .set_split_time(current_time);
            *self.current_split_index.as_mut().unwrap() += 1;
            if Some(self.run.len()) == self.current_split_index {
                self.phase = Ended;
                self.attempt_ended = Some(AtomicDateTime::now());
            }
            self.run.mark_as_modified();

            // FIXME: OnSplit
        }
    }

    /// Starts a new attempt or stores the current time as the time of the
    /// current split. The attempt ends if the last split time is stored.
    pub fn split_or_start(&mut self) {
        if self.phase == NotRunning {
            self.start();
        } else {
            self.split();
        }
    }

    /// Skips the current split if an attempt is in progress and the
    /// current split is not the last split.
    pub fn skip_split(&mut self) {
        if (self.phase == Running || self.phase == Paused)
            && self.current_split_index < self.run.len().checked_sub(1)
        {
            self.current_split_mut().unwrap().clear_split_time();
            self.current_split_index = self.current_split_index.map(|i| i + 1);
            self.run.mark_as_modified();

            // FIXME: OnSkipSplit
        }
    }

    /// Removes the split time from the last split if an attempt is in progress
    /// and there is a previous split. The Timer Phase also switches to
    /// `Running` if it previously was `Ended`.
    pub fn undo_split(&mut self) {
        if self.phase != NotRunning && self.current_split_index > Some(0) {
            if self.phase == Ended {
                self.phase = Running;
            }
            self.current_split_index = self.current_split_index.map(|i| i - 1);
            self.current_split_mut().unwrap().clear_split_time();
            self.run.mark_as_modified();

            // FIXME: OnUndoSplit
        }
    }

    /// Resets the current attempt if there is one in progress. If the splits
    /// are to be updated, all the information of the current attempt is stored
    /// in the Run's history. Otherwise the current attempt's information is
    /// discarded.
    pub fn reset(&mut self, update_splits: bool) {
        if self.phase != NotRunning {
            self.reset_state(update_splits);
            self.reset_splits();
        }
    }

    /// Resets the current attempt if there is one in progress. The splits are
    /// updated such that the current attempt's split times are being stored as
    /// the new Personal Best.
    pub fn reset_and_set_attempt_as_pb(&mut self) {
        if self.phase != NotRunning {
            self.reset_state(true);
            self.set_run_as_pb();
            self.reset_splits();
        }
    }

    fn reset_state(&mut self, update_times: bool) {
        if self.phase != Ended {
            self.attempt_ended = Some(AtomicDateTime::now());
        }
        self.resume_game_time();
        self.set_loading_times(TimeSpan::zero());

        if update_times {
            self.update_attempt_history();
            self.update_best_segments();
            self.update_pb_splits();
            self.update_segment_history();
        }
    }

    fn reset_splits(&mut self) {
        self.phase = NotRunning;
        self.current_split_index = None;

        // Reset Splits
        for segment in self.run.segments_mut() {
            segment.clear_split_time();
        }

        // FIXME: OnReset

        self.run.fix_splits();
        self.run.regenerate_comparisons();
    }

    /// Pauses an active attempt that is not paused.
    pub fn pause(&mut self) {
        if self.phase == Running {
            self.time_paused_at = self.current_time().real_time.unwrap();
            self.phase = Paused;

            // FIXME: OnPause
        }
    }

    /// Resumes an attempt that is paused.
    pub fn resume(&mut self) {
        if self.phase == Paused {
            self.adjusted_start_time = TimeStamp::now() - self.time_paused_at;
            self.phase = Running;

            // FIXME: OnResume
        }
    }

    /// Toggles an active attempt between `Paused` and `Running`.
    pub fn toggle_pause(&mut self) {
        match self.phase {
            Running => self.pause(),
            Paused => self.resume(),
            _ => {}
        }
    }

    /// Toggles an active attempt between `Paused` and `Running` or starts an
    /// attempt if there's none in progress.
    pub fn toggle_pause_or_start(&mut self) {
        match self.phase {
            Running => self.pause(),
            Paused => self.resume(),
            NotRunning => self.start(),
            _ => {}
        }
    }

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
    pub fn undo_all_pauses(&mut self) {
        match self.current_phase() {
            Paused => self.resume(),
            Ended => {
                let pause_time = Some(self.get_pause_time().unwrap_or_default());

                let split_time = self
                    .run
                    .segments_mut()
                    .iter_mut()
                    .last()
                    .unwrap()
                    .split_time_mut();

                *split_time += Time::new()
                    .with_real_time(pause_time)
                    .with_game_time(pause_time);
            }
            _ => {}
        }

        self.adjusted_start_time = self.start_time_with_offset;

        // FIXME: OnUndoAllPauses
    }

    /// Switches the current comparison to the next comparison in the list.
    pub fn switch_to_next_comparison(&mut self) {
        let mut comparisons = self.run.comparisons();
        let len = comparisons.len();
        let index = comparisons
            .position(|c| c == self.current_comparison)
            .unwrap();
        let index = (index + 1) % len;
        self.current_comparison = self.run.comparisons().nth(index).unwrap().to_owned();

        // FIXME: OnNextComparison
    }

    /// Switches the current comparison to the previous comparison in the list.
    pub fn switch_to_previous_comparison(&mut self) {
        let mut comparisons = self.run.comparisons();
        let len = comparisons.len();
        let index = comparisons
            .position(|c| c == self.current_comparison)
            .unwrap();
        let index = (index + len - 1) % len;
        self.current_comparison = self.run.comparisons().nth(index).unwrap().to_owned();

        // FIXME: OnPreviousComparison
    }

    /// Returns the total duration of the current attempt. This is not affected
    /// by the start offset of the run. So if the start offset is -10s and the
    /// `start()` method was called 2s ago, the current time is -8s but the
    /// current attempt duration is 2s. If the timer is then however paused for
    /// 5s, the current attempt duration is still 2s. So the current attempt
    /// duration only counts the time the Timer Phase has actually been
    /// `Running`.
    pub fn current_attempt_duration(&self) -> TimeSpan {
        match self.current_phase() {
            NotRunning => TimeSpan::zero(),
            Paused | Running => TimeStamp::now() - self.start_time,
            Ended => self.attempt_ended.unwrap() - self.attempt_started.unwrap(),
        }
    }

    /// Returns the total amount of time the current attempt has been paused
    /// for. None is returned if there have not been any pauses.
    pub fn get_pause_time(&self) -> Option<TimeSpan> {
        match self.current_phase() {
            Paused => Some(TimeStamp::now() - self.start_time_with_offset - self.time_paused_at),
            Running | Ended if self.start_time_with_offset != self.adjusted_start_time => {
                Some(self.adjusted_start_time - self.start_time_with_offset)
            }
            _ => None,
        }
    }

    /// Returns whether Game Time is currently initialized. Game Time
    /// automatically gets uninitialized for each new attempt.
    #[inline]
    pub fn is_game_time_initialized(&self) -> bool {
        self.loading_times.is_some()
    }

    /// Initializes Game Time for the current attempt. Game Time automatically
    /// gets uninitialized for each new attempt.
    #[inline]
    pub fn initialize_game_time(&mut self) {
        self.loading_times = Some(self.loading_times());
    }

    /// Deinitializes Game Time for the current attempt.
    #[inline]
    pub fn deinitialize_game_time(&mut self) {
        self.loading_times = None;
    }

    /// Returns whether the Game Timer is currently paused. If the Game Timer is
    /// not paused, it automatically increments similar to Real Time.
    #[inline]
    pub fn is_game_time_paused(&self) -> bool {
        self.is_game_time_paused
    }

    /// Pauses the Game Timer such that it doesn't automatically increment
    /// similar to Real Time.
    pub fn pause_game_time(&mut self) {
        if !self.is_game_time_paused() {
            let current_time = self.current_time();
            self.game_time_pause_time = current_time.game_time.or(current_time.real_time);
            self.is_game_time_paused = true;
        }
    }

    /// Resumes the Game Timer such that it automatically increments similar to
    /// Real Time, starting from the Game Time it was paused at.
    pub fn resume_game_time(&mut self) {
        if self.is_game_time_paused() {
            let current_time = self.current_time();
            let diff = catch! { current_time.real_time? - current_time.game_time? };
            self.set_loading_times(diff.unwrap_or_default());
            self.is_game_time_paused = false;
        }
    }

    /// Sets the Game Time to the time specified. This also works if the Game
    /// Time is paused, which can be used as a way of updating the Game Timer
    /// periodically without it automatically moving forward. This ensures that
    /// the Game Timer never shows any time that is not coming from the game.
    #[inline]
    pub fn set_game_time(&mut self, game_time: TimeSpan) {
        if self.is_game_time_paused() {
            self.game_time_pause_time = Some(game_time);
        }
        let loading_times = self.current_time().real_time.unwrap() - game_time;
        self.loading_times = Some(loading_times);
    }

    /// Accesses the loading times. Loading times are defined as Game Time - Real Time.
    #[inline]
    pub fn loading_times(&self) -> TimeSpan {
        self.loading_times.unwrap_or_default()
    }

    /// Instead of setting the Game Time directly, this method can be used to
    /// just specify the amount of time the game has been loading. The Game Time
    /// is then automatically determined by Real Time - Loading Times.
    #[inline]
    pub fn set_loading_times(&mut self, time: TimeSpan) {
        self.loading_times = Some(time);
        if self.is_game_time_paused() {
            self.game_time_pause_time = Some(self.current_time().real_time.unwrap() - time);
        }
    }

    /// Sets the value of a custom variable with the name specified. If the
    /// variable does not exist, a temporary variable gets created that will not
    /// be stored in the splits file.
    pub fn set_custom_variable<N, V>(&mut self, name: N, value: V)
    where
        N: Into<String>,
        V: AsRef<str>,
    {
        let var = self.run.metadata_mut().custom_variable_mut(name);
        var.set_value(value);
        if var.is_permanent {
            self.run.mark_as_modified();
        }
    }

    fn update_attempt_history(&mut self) {
        let time = if self.phase == Ended {
            self.current_time()
        } else {
            Default::default()
        };

        let pause_time = self.get_pause_time();

        self.run
            .add_attempt(time, self.attempt_started, self.attempt_ended, pause_time);
    }

    fn update_best_segments(&mut self) {
        let mut previous_split_time_rta = Some(TimeSpan::zero());
        let mut previous_split_time_game_time = Some(TimeSpan::zero());

        for split in self.run.segments_mut() {
            let mut new_best_segment = split.best_segment_time();
            if let Some(split_time) = split.split_time().real_time {
                let current_segment = previous_split_time_rta.map(|previous| split_time - previous);
                previous_split_time_rta = Some(split_time);
                if split
                    .best_segment_time()
                    .real_time
                    .map_or(true, |b| current_segment.map_or(false, |c| c < b))
                {
                    new_best_segment.real_time = current_segment;
                }
            }
            if let Some(split_time) = split.split_time().game_time {
                let current_segment =
                    previous_split_time_game_time.map(|previous| split_time - previous);
                previous_split_time_game_time = Some(split_time);
                if split
                    .best_segment_time()
                    .game_time
                    .map_or(true, |b| current_segment.map_or(false, |c| c < b))
                {
                    new_best_segment.game_time = current_segment;
                }
            }
            split.set_best_segment_time(new_best_segment);
        }
    }

    fn update_pb_splits(&mut self) {
        let method = self.current_timing_method;
        let (split_time, pb_split_time) = {
            let last_segment = self.run.segments().last().unwrap();
            (
                last_segment.split_time()[method],
                last_segment.personal_best_split_time()[method],
            )
        };
        if split_time.map_or(false, |s| pb_split_time.map_or(true, |pb| s < pb)) {
            self.set_run_as_pb();
        }
    }

    fn update_segment_history(&mut self) {
        if let Some(index) = self.current_split_index {
            self.run.update_segment_history(index);
        }
    }

    fn set_run_as_pb(&mut self) {
        self.run.import_pb_into_segment_history();
        self.run.fix_splits();
        for segment in self.run.segments_mut() {
            let split_time = segment.split_time();
            segment.set_personal_best_split_time(split_time);
        }
        self.run.clear_run_id();
    }
}
