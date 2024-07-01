use crate::{
    analysis::check_best_segment,
    comparison::personal_best,
    event::{Error, Event},
    platform::prelude::*,
    util::PopulateString,
    AtomicDateTime, Run, Segment, Time, TimeSpan, TimeStamp,
    TimerPhase::{self, *},
    TimingMethod,
};
use core::{mem, ops::Deref};

#[cfg(test)]
mod tests;

mod active_attempt;
use active_attempt::{ActiveAttempt, State};

/// A `Timer` provides all the capabilities necessary for doing speedrun attempts.
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
/// timer.start().unwrap();
/// assert_eq!(timer.current_phase(), TimerPhase::Running);
///
/// // Create a split.
/// timer.split().unwrap();
///
/// // The run should be finished now.
/// assert_eq!(timer.current_phase(), TimerPhase::Ended);
///
/// // Reset the attempt and confirm that we want to store the attempt.
/// timer.reset(true).unwrap();
///
/// // The attempt is now over.
/// assert_eq!(timer.current_phase(), TimerPhase::NotRunning);
/// ```
#[derive(Debug, Clone)]
pub struct Timer {
    run: Run,
    current_comparison: String,
    current_timing_method: TimingMethod,
    active_attempt: Option<ActiveAttempt>,
}

/// A snapshot represents a specific point in time that the timer was observed
/// at. The snapshot dereferences to the timer. Everything you perceive through
/// the snapshot is entirely frozen in time.
pub struct Snapshot<'timer> {
    timer: &'timer Timer,
    time: Time,
}

impl Snapshot<'_> {
    /// Returns the time the timer was at when the snapshot was taken. The Game
    /// Time is None if the Game Time has not been initialized.
    pub const fn current_time(&self) -> Time {
        self.time
    }
}

impl Deref for Snapshot<'_> {
    type Target = Timer;
    fn deref(&self) -> &Self::Target {
        self.timer
    }
}

/// A `SharedTimer` is a wrapper around the [`Timer`](crate::timing::Timer) that can be shared across multiple threads with multiple owners.
#[cfg(feature = "std")]
pub type SharedTimer = alloc::sync::Arc<std::sync::RwLock<Timer>>;

/// The Error type for creating a new Timer from a Run.
#[derive(Debug, snafu::Snafu)]
pub enum CreationError {
    /// The Timer couldn't be created, because the Run has no segments.
    EmptyRun,
}

pub type Result<T = Event, E = Error> = core::result::Result<T, E>;

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

        Ok(Timer {
            run,
            current_comparison: personal_best::NAME.into(),
            current_timing_method: TimingMethod::RealTime,
            active_attempt: None,
        })
    }

    /// Consumes the Timer and creates a Shared Timer that can be shared across
    /// multiple threads with multiple owners.
    #[cfg(feature = "std")]
    pub fn into_shared(self) -> SharedTimer {
        alloc::sync::Arc::new(std::sync::RwLock::new(self))
    }

    /// Takes out the Run from the Timer and resets the current attempt if there
    /// is one in progress. If the splits are to be updated, all the information
    /// of the current attempt is stored in the Run's history. Otherwise the
    /// current attempt's information is discarded.
    pub fn into_run(mut self, update_splits: bool) -> Run {
        let _ = self.reset(update_splits);
        self.run
    }

    /// Replaces the Run object used by the Timer with the Run object provided.
    /// If the Run provided contains no segments, it can't be used for timing
    /// and is returned as the `Err` case of the `Result`. Otherwise the Run
    /// that was in use by the Timer is being returned. Before the Run is
    /// returned, the current attempt is reset and the splits are being updated
    /// depending on the `update_splits` parameter.
    #[allow(clippy::result_large_err)]
    pub fn replace_run(&mut self, mut run: Run, update_splits: bool) -> Result<Run, Run> {
        if run.is_empty() {
            return Err(run);
        }

        let _ = self.reset(update_splits);
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
    #[allow(clippy::result_large_err)]
    pub fn set_run(&mut self, run: Run) -> Result<(), Run> {
        self.replace_run(run, false).map(drop)
    }

    /// Accesses the Run in use by the Timer.
    #[inline]
    pub const fn run(&self) -> &Run {
        &self.run
    }

    /// Stores a settings map into the parsed auto splitter settings.
    #[cfg(feature = "auto-splitting")]
    pub fn run_auto_splitter_settings_map_store(
        &mut self,
        settings_map: livesplit_auto_splitting::settings::Map,
    ) {
        self.run.auto_splitter_settings_map_store(settings_map);
    }

    /// Marks the Run as unmodified, so that it is known that all the changes
    /// have been saved.
    #[inline]
    pub fn mark_as_unmodified(&mut self) {
        self.run.mark_as_unmodified();
    }

    /// Returns the current Timer Phase.
    #[inline]
    pub const fn current_phase(&self) -> TimerPhase {
        let Some(active_attempt) = &self.active_attempt else {
            return TimerPhase::NotRunning;
        };
        match active_attempt.state {
            State::NotEnded { time_paused_at, .. } => {
                if time_paused_at.is_some() {
                    Paused
                } else {
                    Running
                }
            }
            State::Ended { .. } => Ended,
        }
    }

    /// Creates a new snapshot of the timer at the point in time of this call.
    /// It represents a frozen state of the timer such that calculations can
    /// work with an entirely consistent view of the timer without the current
    /// time changing underneath.
    pub fn snapshot(&self) -> Snapshot<'_> {
        let time = match &self.active_attempt {
            Some(active_attempt) => active_attempt.current_time(&self.run).into(),
            None => {
                let offset = Some(self.run.offset());
                Time {
                    real_time: offset,
                    game_time: offset,
                }
            }
        };

        Snapshot { timer: self, time }
    }

    /// Returns the currently selected timing method.
    #[inline]
    pub const fn current_timing_method(&self) -> TimingMethod {
        self.current_timing_method
    }

    /// Sets the current timing method to the timing method provided.
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
    pub fn set_current_comparison<S: PopulateString>(&mut self, comparison: S) -> Result {
        let as_str = comparison.as_str();
        if self.run.comparisons().any(|c| c == as_str) {
            comparison.populate(&mut self.current_comparison);
            Ok(Event::ComparisonChanged)
        } else {
            Err(Error::ComparisonDoesntExist)
        }
    }

    /// Accesses the split the attempt is currently on. If there's no attempt in
    /// progress or the run finished, `None` is returned instead.
    pub fn current_split(&self) -> Option<&Segment> {
        self.active_attempt
            .as_ref()?
            .current_split_index()
            .map(|i| self.run.segment(i))
    }

    /// Accesses the index of the split the attempt is currently on. If there's
    /// no attempt in progress, `None` is returned instead. This returns an
    /// index that is equal to the amount of segments when the attempt is
    /// finished, but has not been reset. So you need to be careful when using
    /// this value for indexing.
    #[inline]
    pub fn current_split_index(&self) -> Option<usize> {
        Some(
            self.active_attempt
                .as_ref()?
                .current_split_index_overflowing(&self.run),
        )
    }

    /// Starts the Timer if there is no attempt in progress. If that's not the
    /// case, nothing happens.
    pub fn start(&mut self) -> Result {
        if self.active_attempt.is_none() {
            let attempt_started = AtomicDateTime::now();
            let start_time = TimeStamp::now();
            let offset = self.run.offset();

            self.active_attempt = Some(ActiveAttempt {
                state: State::NotEnded {
                    current_split_index: 0,
                    time_paused_at: None,
                },
                attempt_started,
                start_time,
                original_offset: offset,
                adjusted_offset: offset,
                game_time_paused_at: None,
                loading_times: None,
            });
            self.run.start_next_run();

            Ok(Event::Started)
        } else {
            Err(Error::RunAlreadyInProgress)
        }
    }

    /// If an attempt is in progress, stores the current time as the time of the
    /// current split. The attempt ends if the last split time is stored.
    pub fn split(&mut self) -> Result {
        let active_attempt = self.active_attempt.as_mut().ok_or(Error::NoRunInProgress)?;

        let (split_index, current_time, event) = active_attempt.prepare_split(&self.run)?;

        // FIXME: We shouldn't need to collect here.
        let variables = self
            .run
            .metadata()
            .custom_variables()
            .map(|(k, v)| (k.to_owned(), v.value.clone()))
            .collect();

        let segment = self.run.segment_mut(split_index);
        segment.set_split_time(current_time);
        *segment.variables_mut() = variables;

        self.run.mark_as_modified();

        Ok(event)
    }

    /// Starts a new attempt or stores the current time as the time of the
    /// current split. The attempt ends if the last split time is stored.
    pub fn split_or_start(&mut self) -> Result {
        if self.active_attempt.is_none() {
            self.start()
        } else {
            self.split()
        }
    }

    /// Skips the current split if an attempt is in progress and the
    /// current split is not the last split.
    pub fn skip_split(&mut self) -> Result {
        let active_attempt = self.active_attempt.as_mut().ok_or(Error::NoRunInProgress)?;

        let Some(current_split_index) = active_attempt.current_split_index_mut() else {
            return Err(Error::RunFinished);
        };

        if *current_split_index + 1 < self.run.len() {
            self.run
                .segment_mut(*current_split_index)
                .clear_split_info();

            *current_split_index += 1;

            self.run.mark_as_modified();

            Ok(Event::SplitSkipped)
        } else {
            Err(Error::CantSkipLastSplit)
        }
    }

    /// Removes the split time from the last split if an attempt is in progress
    /// and there is a previous split. The Timer Phase also switches to
    /// [`Running`] if it previously was [`Ended`].
    pub fn undo_split(&mut self) -> Result {
        let active_attempt = self.active_attempt.as_mut().ok_or(Error::NoRunInProgress)?;

        if let Some(previous_split_index) = active_attempt
            .current_split_index_overflowing(&self.run)
            .checked_sub(1)
        {
            let time_paused_at = match &active_attempt.state {
                State::NotEnded { time_paused_at, .. } => *time_paused_at,
                State::Ended { .. } => None,
            };

            active_attempt.state = State::NotEnded {
                current_split_index: previous_split_index,
                time_paused_at,
            };

            self.run
                .segment_mut(previous_split_index)
                .clear_split_info();

            self.run.mark_as_modified();

            Ok(Event::SplitUndone)
        } else {
            Err(Error::CantUndoFirstSplit)
        }
    }

    /// Checks whether the current attempt has a new Personal Best for the
    /// [`TimingMethod`] specified.
    pub fn current_attempt_has_new_personal_best(&self, timing_method: TimingMethod) -> bool {
        if self.current_phase() != Ended {
            return false;
        }

        let last_segment = self.run.segments().last().unwrap();

        if let Some(final_time) = last_segment.split_time()[timing_method] {
            if last_segment.personal_best_split_time()[timing_method]
                .map_or(true, |pb| final_time < pb)
            {
                return true;
            }
        }

        false
    }

    /// Checks whether the current attempt has new best segment times in any of
    /// the segments for the [`TimingMethod`] specified.
    pub fn current_attempt_has_new_best_segments(&self, timing_method: TimingMethod) -> bool {
        if self.active_attempt.is_none() {
            return false;
        }

        for segment_index in 0..self.run.len() {
            if check_best_segment(self, segment_index, timing_method) {
                return true;
            }
        }

        false
    }

    /// Checks whether the current attempt has new best segment times in any of
    /// the segments (for both [`TimingMethods`](TimingMethod)) or a new
    /// Personal Best (for the current [`TimingMethod`]). This can be used to
    /// ask the user whether to update the splits when resetting.
    pub fn current_attempt_has_new_best_times(&self) -> bool {
        self.current_attempt_has_new_best_segments(TimingMethod::RealTime)
            || self.current_attempt_has_new_best_segments(TimingMethod::GameTime)
            || self.current_attempt_has_new_personal_best(self.current_timing_method)
    }

    /// Resets the current attempt if there is one in progress. If the splits
    /// are to be updated, all the information of the current attempt is stored
    /// in the Run's history. Otherwise the current attempt's information is
    /// discarded.
    pub fn reset(&mut self, update_splits: bool) -> Result {
        if self.active_attempt.is_some() {
            self.reset_state(update_splits);
            self.reset_splits();
            Ok(Event::Reset)
        } else {
            Err(Error::NoRunInProgress)
        }
    }

    /// Resets the current attempt if there is one in progress. The splits are
    /// updated such that the current attempt's split times are being stored as
    /// the new Personal Best.
    pub fn reset_and_set_attempt_as_pb(&mut self) -> Result {
        if self.active_attempt.is_some() {
            self.reset_state(true);
            set_run_as_pb(&mut self.run);
            self.reset_splits();
            Ok(Event::Reset)
        } else {
            Err(Error::NoRunInProgress)
        }
    }

    fn reset_state(&mut self, update_times: bool) {
        let Some(active_attempt) = self.active_attempt.take() else {
            return;
        };

        if update_times {
            active_attempt.update_times(&mut self.run, self.current_timing_method);
        }
    }

    fn reset_splits(&mut self) {
        // Reset Splits
        for segment in self.run.segments_mut() {
            segment.clear_split_info();
        }

        self.run.fix_splits();
        self.run.regenerate_comparisons();
    }

    /// Pauses an active attempt that is not paused.
    pub fn pause(&mut self) -> Result {
        let active_attempt = self.active_attempt.as_mut().ok_or(Error::NoRunInProgress)?;

        let State::NotEnded { time_paused_at, .. } = &mut active_attempt.state else {
            return Err(Error::RunFinished);
        };

        if time_paused_at.is_none() {
            *time_paused_at =
                Some(TimeStamp::now() - active_attempt.start_time + active_attempt.adjusted_offset);
            Ok(Event::Paused)
        } else {
            Err(Error::AlreadyPaused)
        }
    }

    /// Resumes an attempt that is paused.
    pub fn resume(&mut self) -> Result {
        let active_attempt = self.active_attempt.as_mut().ok_or(Error::NoRunInProgress)?;

        let State::NotEnded { time_paused_at, .. } = &mut active_attempt.state else {
            return Err(Error::RunFinished);
        };

        if let Some(pause_time) = *time_paused_at {
            active_attempt.adjusted_offset =
                pause_time - (TimeStamp::now() - active_attempt.start_time);
            *time_paused_at = None;
            Ok(Event::Resumed)
        } else {
            Err(Error::NotPaused)
        }
    }

    /// Toggles an active attempt between `Paused` and `Running`.
    pub fn toggle_pause(&mut self) -> Result {
        match self.current_phase() {
            Running => self.pause(),
            Paused => self.resume(),
            NotRunning => Err(Error::NoRunInProgress),
            Ended => Err(Error::RunFinished),
        }
    }

    /// Toggles an active attempt between [`Paused`] and [`Running`] or starts
    /// an attempt if there's none in progress.
    pub fn toggle_pause_or_start(&mut self) -> Result {
        match self.current_phase() {
            Running => self.pause(),
            Paused => self.resume(),
            NotRunning => self.start(),
            Ended => Err(Error::RunFinished),
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
    pub fn undo_all_pauses(&mut self) -> Result {
        let event = match self.current_phase() {
            Paused => {
                self.resume()?;
                Event::PausesUndoneAndResumed
            }
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

                Event::PausesUndone
            }
            _ => Event::PausesUndone,
        };

        if let Some(active_attempt) = &mut self.active_attempt {
            active_attempt.adjusted_offset = active_attempt.original_offset;
            Ok(event)
        } else {
            Err(Error::NoRunInProgress)
        }
    }

    /// Switches the current comparison to the next comparison in the list.
    pub fn switch_to_next_comparison(&mut self) {
        let mut comparisons = self.run.comparisons();
        let len = comparisons.len();
        let index = comparisons
            .position(|c| c == self.current_comparison)
            .unwrap();
        let index = (index + 1) % len;
        self.run
            .comparisons()
            .nth(index)
            .unwrap()
            .populate(&mut self.current_comparison);
    }

    /// Switches the current comparison to the previous comparison in the list.
    pub fn switch_to_previous_comparison(&mut self) {
        let mut comparisons = self.run.comparisons();
        let len = comparisons.len();
        let index = comparisons
            .position(|c| c == self.current_comparison)
            .unwrap();
        let index = (index + len - 1) % len;
        self.run
            .comparisons()
            .nth(index)
            .unwrap()
            .populate(&mut self.current_comparison);
    }

    /// Returns the total duration of the current attempt. This is not affected
    /// by the start offset of the run. So if the start offset is -10s and the
    /// `start()` method was called 2s ago, the current time is -8s but the
    /// current attempt duration is 2s. If the timer is then however paused for
    /// 5s, the current attempt duration is still 2s. So the current attempt
    /// duration only counts the time the Timer Phase has actually been
    /// `Running`.
    pub fn current_attempt_duration(&self) -> TimeSpan {
        let Some(active_attempt) = &self.active_attempt else {
            return TimeSpan::zero();
        };

        if let State::Ended { attempt_ended } = active_attempt.state {
            attempt_ended - active_attempt.attempt_started
        } else {
            TimeStamp::now() - active_attempt.start_time
        }
    }

    /// Returns the total amount of time the current attempt has been paused
    /// for. None is returned if there have not been any pauses.
    pub fn get_pause_time(&self) -> Option<TimeSpan> {
        self.active_attempt.as_ref()?.get_pause_time()
    }

    /// Returns whether Game Time is currently initialized. Game Time
    /// automatically gets uninitialized for each new attempt.
    #[inline]
    pub const fn is_game_time_initialized(&self) -> bool {
        match &self.active_attempt {
            Some(active_attempt) => active_attempt.loading_times.is_some(),
            None => false,
        }
    }

    /// Initializes game time for the current attempt. Game time automatically
    /// gets uninitialized for each new attempt.
    #[inline]
    pub fn initialize_game_time(&mut self) -> Result {
        let active_attempt = self.active_attempt.as_mut().ok_or(Error::NoRunInProgress)?;

        if active_attempt.loading_times.is_none() {
            active_attempt.loading_times = Some(TimeSpan::zero());
            Ok(Event::GameTimeInitialized)
        } else {
            Err(Error::GameTimeAlreadyInitialized)
        }
    }

    /// Deinitializes Game Time for the current attempt.
    #[inline]
    pub fn deinitialize_game_time(&mut self) {
        if let Some(active_attempt) = &mut self.active_attempt {
            active_attempt.loading_times = None;
        }
    }

    /// Returns whether the Game Timer is currently paused. If the Game Timer is
    /// not paused, it automatically increments similar to Real Time.
    #[inline]
    pub const fn is_game_time_paused(&self) -> bool {
        match &self.active_attempt {
            Some(active_attempt) => active_attempt.game_time_paused_at.is_some(),
            None => false,
        }
    }

    /// Pauses the Game Timer such that it doesn't automatically increment
    /// similar to Real Time.
    pub fn pause_game_time(&mut self) -> Result {
        let active_attempt = self.active_attempt.as_mut().ok_or(Error::NoRunInProgress)?;

        if active_attempt.game_time_paused_at.is_none() {
            let current_time = active_attempt.current_time(&self.run);

            active_attempt.game_time_paused_at =
                current_time.game_time.or(Some(current_time.real_time));

            Ok(Event::GameTimePaused)
        } else {
            Err(Error::GameTimeAlreadyPaused)
        }
    }

    /// Resumes the Game Timer such that it automatically increments similar to
    /// Real Time, starting from the Game Time it was paused at.
    pub fn resume_game_time(&mut self) -> Result {
        let active_attempt = self.active_attempt.as_mut().ok_or(Error::NoRunInProgress)?;

        if active_attempt.game_time_paused_at.is_some() {
            let current_time = active_attempt.current_time(&self.run);

            let diff = catch! { current_time.real_time - current_time.game_time? };
            active_attempt.set_loading_times(diff.unwrap_or_default(), &self.run);
            active_attempt.game_time_paused_at = None;

            Ok(Event::GameTimeResumed)
        } else {
            Err(Error::GameTimeNotPaused)
        }
    }

    /// Sets the Game Time to the time specified. This also works if the Game
    /// Time is paused, which can be used as a way of updating the Game Timer
    /// periodically without it automatically moving forward. This ensures that
    /// the Game Timer never shows any time that is not coming from the game.
    #[inline]
    pub fn set_game_time(&mut self, game_time: TimeSpan) -> Result {
        let active_attempt = self.active_attempt.as_mut().ok_or(Error::NoRunInProgress)?;

        if active_attempt.game_time_paused_at.is_some() {
            active_attempt.game_time_paused_at = Some(game_time);
        }
        active_attempt.loading_times =
            Some(active_attempt.current_time(&self.run).real_time - game_time);

        Ok(Event::GameTimeSet)
    }

    /// Accesses the loading times. Loading times are defined as Game Time - Real Time.
    #[inline]
    pub fn loading_times(&self) -> TimeSpan {
        self.active_attempt
            .as_ref()
            .and_then(|a| a.loading_times)
            .unwrap_or_default()
    }

    /// Instead of setting the game time directly, this method can be used to
    /// just specify the amount of time the game has been loading. The game time
    /// is then automatically determined by Real Time - Loading Times.
    #[inline]
    pub fn set_loading_times(&mut self, time: TimeSpan) -> Result {
        if let Some(active_attempt) = &mut self.active_attempt {
            active_attempt.set_loading_times(time, &self.run);
            Ok(Event::LoadingTimesSet)
        } else {
            Err(Error::NoRunInProgress)
        }
    }

    /// Sets the value of a custom variable with the name specified. If the
    /// variable does not exist, a temporary variable gets created that will not
    /// be stored in the splits file.
    pub fn set_custom_variable<N, V>(&mut self, name: N, value: V)
    where
        N: PopulateString,
        V: PopulateString,
    {
        let var = self.run.metadata_mut().custom_variable_mut(name);
        var.set_value(value);
        if var.is_permanent {
            self.run.mark_as_modified();
        }
    }

    /// Notifies the `Timer` that the currently loaded [`Layout`](crate::Layout)
    /// has changed. If the [`Run`] has a
    /// [`LinkedLayout`](crate::run::LinkedLayout), it will be updated
    /// accordingly. Specify [`None`] if the default [`Layout`](crate::Layout)
    /// should be linked.
    #[inline]
    pub fn layout_path_changed<S>(&mut self, path: Option<S>)
    where
        S: PopulateString,
    {
        if self.run.layout_path_changed(path) {
            self.run.mark_as_modified();
        }
    }
}

fn set_run_as_pb(run: &mut Run) {
    run.import_pb_into_segment_history();
    run.fix_splits();
    for segment in run.segments_mut() {
        let split_time = segment.split_time();
        segment.set_personal_best_split_time(split_time);
    }
    run.clear_run_id();
}
