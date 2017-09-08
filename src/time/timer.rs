use {AtomicDateTime, Run, Segment, Time, TimeSpan, TimeStamp, TimerPhase, TimingMethod};
use TimerPhase::*;
use comparison::personal_best;
use parking_lot::RwLock;
use std::sync::Arc;
use std::mem;

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
    // This gets adjusted after unpausing
    adjusted_start_time: TimeStamp,
    time_paused_at: TimeSpan,
    is_game_time_paused: bool,
    game_time_pause_time: Option<TimeSpan>,
    loading_times: Option<TimeSpan>,
}

pub type SharedTimer = Arc<RwLock<Timer>>;

quick_error! {
    #[derive(Debug)]
    pub enum CreationError {
        EmptyRun
    }
}

impl Timer {
    #[inline]
    pub fn new(mut run: Run) -> Result<Self, CreationError> {
        if run.is_empty() {
            return Err(CreationError::EmptyRun);
        }

        run.regenerate_comparisons();
        let now = TimeStamp::now();

        Ok(Timer {
            run: run,
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

    pub fn into_shared(self) -> SharedTimer {
        Arc::new(RwLock::new(self))
    }

    pub fn replace_run(&mut self, run: Run, update_splits: bool) -> Result<Run, Run> {
        if run.is_empty() {
            return Err(run);
        }

        self.reset(update_splits);
        if !run.comparisons().any(|c| c == self.current_comparison) {
            self.current_comparison = personal_best::NAME.to_string();
        }

        Ok(mem::replace(&mut self.run, run))
    }

    pub fn set_run(&mut self, run: Run) -> Result<(), Run> {
        self.replace_run(run, false).map(|_| ())
    }

    #[inline]
    pub fn run(&self) -> &Run {
        &self.run
    }

    #[inline]
    pub fn current_phase(&self) -> TimerPhase {
        self.phase
    }

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
            _ => if self.is_game_time_paused() {
                self.game_time_pause_time
            } else {
                TimeSpan::option_sub(
                    real_time,
                    if self.is_game_time_initialized() {
                        Some(self.loading_times())
                    } else {
                        None
                    },
                )
            },
        };

        Time::new()
            .with_real_time(real_time)
            .with_game_time(game_time)
    }

    #[inline]
    pub fn current_timing_method(&self) -> TimingMethod {
        self.current_timing_method
    }

    #[inline]
    pub fn set_current_timing_method(&mut self, method: TimingMethod) {
        self.current_timing_method = method;
    }

    #[inline]
    pub fn current_comparison(&self) -> &str {
        &self.current_comparison
    }

    pub fn current_split(&self) -> Option<&Segment> {
        self.current_split_index.and_then(|i| {
            self.run.segments().get(i)
        })
    }

    fn current_split_mut(&mut self) -> Option<&mut Segment> {
        self.current_split_index.and_then(move |i| {
            self.run.segments_mut().get_mut(i)
        })
    }

    #[inline]
    pub fn current_split_index(&self) -> Option<usize> {
        self.current_split_index
    }

    pub fn start(&mut self) {
        if self.phase == NotRunning {
            self.phase = Running;
            self.current_split_index = Some(0);
            self.attempt_started = Some(AtomicDateTime::now());
            self.start_time = TimeStamp::now();
            self.start_time_with_offset = self.start_time - self.run.offset();
            self.adjusted_start_time = self.start_time_with_offset;
            self.time_paused_at = self.run.offset();
            self.uninitialize_game_time();
            self.run.start_next_run();

            // TODO OnStart
        }
    }

    pub fn split(&mut self) {
        let current_time = self.current_time();
        if self.phase == Running &&
            current_time
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
            self.run.mark_as_changed();

            // TODO OnSplit
        }
    }

    pub fn split_or_start(&mut self) {
        if self.phase == NotRunning {
            self.start();
        } else {
            self.split();
        }
    }

    pub fn skip_split(&mut self) {
        if (self.phase == Running || self.phase == Paused) &&
            self.current_split_index < self.run.len().checked_sub(1)
        {
            self.current_split_mut().unwrap().clear_split_time();
            self.current_split_index = self.current_split_index.map(|i| i+1);
            self.run.mark_as_changed();

            // TODO OnSkipSplit
        }
    }

    pub fn undo_split(&mut self) {
        if self.phase != NotRunning && self.current_split_index > Some(0) {
            if self.phase == Ended {
                self.phase = Running;
            }
            self.current_split_index = self.current_split_index.map(|i| i-1);
            self.current_split_mut().unwrap().clear_split_time();
            self.run.mark_as_changed();

            // TODO OnUndoSplit
        }
    }

    pub fn reset(&mut self, update_splits: bool) {
        if self.phase != NotRunning {
            if self.phase != Ended {
                self.attempt_ended = Some(AtomicDateTime::now());
            }
            self.unpause_game_time();
            self.set_loading_times(TimeSpan::zero());

            if update_splits {
                self.update_attempt_history();
                self.update_best_segments();
                self.update_pb_splits();
                self.update_segment_history();
            }

            self.reset_splits();
            self.run.fix_splits();
            self.run.regenerate_comparisons();
        }
    }

    fn reset_splits(&mut self) {
        self.phase = NotRunning;
        self.current_split_index = None;

        // Reset Splits
        for segment in self.run.segments_mut() {
            segment.clear_split_time();
        }

        // TODO OnReset
    }

    pub fn pause(&mut self) {
        if self.phase == Running {
            self.time_paused_at = self.current_time().real_time.unwrap();
            self.phase = Paused;

            // TODO OnPause
        }
    }

    pub fn resume(&mut self) {
        if self.phase == Paused {
            self.adjusted_start_time = TimeStamp::now() - self.time_paused_at;
            self.phase = Running;

            // TODO OnResume
        }
    }

    pub fn toggle_pause(&mut self) {
        match self.phase {
            Running => self.pause(),
            Paused => self.resume(),
            _ => {}
        }
    }

    pub fn toggle_pause_or_start(&mut self) {
        match self.phase {
            Running => self.pause(),
            Paused => self.resume(),
            NotRunning => self.start(),
            _ => {}
        }
    }

    pub fn undo_all_pauses(&mut self) {
        match self.current_phase() {
            Paused => self.resume(),
            Ended => {
                let pause_time = Some(self.get_pause_time().unwrap_or_default());

                let split_time = self.run
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

        // TODO OnUndoAllPauses
    }

    pub fn switch_to_next_comparison(&mut self) {
        let mut comparisons = self.run.comparisons();
        let len = comparisons.len();
        let index = comparisons
            .position(|c| c == self.current_comparison)
            .unwrap();
        let index = (index + 1) % len;
        self.current_comparison = self.run.comparisons().nth(index).unwrap().to_owned();

        // TODO OnNextComparison
    }

    pub fn switch_to_previous_comparison(&mut self) {
        let mut comparisons = self.run.comparisons();
        let len = comparisons.len();
        let index = comparisons
            .position(|c| c == self.current_comparison)
            .unwrap();
        let index = (index + len - 1) % len;
        self.current_comparison = self.run.comparisons().nth(index).unwrap().to_owned();

        // TODO OnPreviousComparison
    }

    pub fn current_attempt_duration(&self) -> TimeSpan {
        match self.current_phase() {
            NotRunning => TimeSpan::zero(),
            Paused | Running => TimeStamp::now() - self.start_time,
            Ended => self.attempt_ended.unwrap() - self.attempt_started.unwrap(),
        }
    }

    pub fn get_pause_time(&self) -> Option<TimeSpan> {
        match self.current_phase() {
            Paused => Some(
                TimeStamp::now() - self.start_time_with_offset - self.time_paused_at,
            ),
            Running | Ended if self.start_time_with_offset != self.adjusted_start_time => {
                Some(self.adjusted_start_time - self.start_time_with_offset)
            }
            _ => None,
        }
    }

    #[inline]
    pub fn is_game_time_initialized(&self) -> bool {
        self.loading_times.is_some()
    }

    #[inline]
    pub fn initialize_game_time(&mut self) {
        self.loading_times = Some(self.loading_times());
    }

    #[inline]
    pub fn uninitialize_game_time(&mut self) {
        self.loading_times = None;
    }

    #[inline]
    pub fn is_game_time_paused(&self) -> bool {
        self.is_game_time_paused
    }

    pub fn pause_game_time(&mut self) {
        if !self.is_game_time_paused() {
            let current_time = self.current_time();
            self.game_time_pause_time = current_time.game_time.or(current_time.real_time);
            self.is_game_time_paused = true;
        }
    }

    pub fn unpause_game_time(&mut self) {
        if self.is_game_time_paused() {
            let current_time = self.current_time();
            self.set_loading_times(
                TimeSpan::option_sub(current_time.real_time, current_time.game_time)
                    .unwrap_or_default(),
            );
            self.is_game_time_paused = false;
        }
    }

    #[inline]
    pub fn set_game_time(&mut self, game_time: TimeSpan) {
        if self.is_game_time_paused() {
            self.game_time_pause_time = Some(game_time);
        }
        let loading_times = self.current_time().real_time.unwrap() - game_time;
        self.loading_times = Some(loading_times);
    }

    #[inline]
    pub fn loading_times(&self) -> TimeSpan {
        self.loading_times.unwrap_or_default()
    }

    #[inline]
    pub fn set_loading_times(&mut self, time: TimeSpan) {
        self.loading_times = Some(time);
        if self.is_game_time_paused() {
            self.game_time_pause_time = Some(self.current_time().real_time.unwrap() - time);
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
        match self.current_split_index {
            Some(i) => self.run.update_segment_history(i),
            None => {},
        }
    }

    fn set_run_as_pb(&mut self) {
        self.run.import_segment_history();
        self.run.fix_splits();
        for segment in self.run.segments_mut() {
            let split_time = segment.split_time();
            segment.set_personal_best_split_time(split_time);
        }
    }
}
