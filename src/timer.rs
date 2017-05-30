use {AtomicDateTime, Run, Time, TimerPhase, TimingMethod, TimeStamp, TimeSpan, Segment};
use TimerPhase::*;
use run::PERSONAL_BEST_COMPARISON_NAME;
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Timer {
    run: Run,
    phase: TimerPhase,
    current_split_index: isize,
    current_timing_method: TimingMethod,
    current_comparison: String,
    attempt_started: Option<AtomicDateTime>,
    attempt_ended: Option<AtomicDateTime>,
    adjusted_start_time: TimeStamp,
    start_time: TimeStamp,
    time_paused_at: TimeSpan,
    is_game_time_paused: bool,
    game_time_pause_time: Option<TimeSpan>,
    loading_times: Option<TimeSpan>,
}

pub type SharedTimer = Arc<RwLock<Timer>>;

impl Timer {
    #[inline]
    pub fn new(mut run: Run) -> Self {
        assert!(run.len() > 0);

        run.regenerate_comparisons();
        let now = TimeStamp::now();

        Timer {
            run: run,
            phase: TimerPhase::NotRunning,
            current_split_index: -1,
            current_timing_method: TimingMethod::RealTime,
            current_comparison: PERSONAL_BEST_COMPARISON_NAME.into(),
            attempt_started: None,
            attempt_ended: None,
            adjusted_start_time: now,
            start_time: now,
            time_paused_at: TimeSpan::zero(),
            is_game_time_paused: false,
            game_time_pause_time: None,
            loading_times: None,
        }
    }

    pub fn into_shared(self) -> SharedTimer {
        Arc::new(RwLock::new(self))
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
            Ended => {
                self.run
                    .segments()
                    .last()
                    .unwrap()
                    .split_time()
                    .real_time
            }
        };

        let game_time = match self.phase {
            NotRunning => Some(self.run.offset()),
            Ended => {
                self.run
                    .segments()
                    .last()
                    .unwrap()
                    .split_time()
                    .game_time
            }
            _ => {
                if self.is_game_time_paused() {
                    self.game_time_pause_time
                } else {
                    TimeSpan::option_op(real_time,
                                        if self.is_game_time_initialized() {
                                            Some(self.loading_times())
                                        } else {
                                            None
                                        },
                                        |a, b| a - b)
                }
            }
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
        if self.current_split_index >= 0 {
            self.run
                .segments()
                .get(self.current_split_index as usize)
        } else {
            None
        }
    }

    fn current_split_mut(&mut self) -> Option<&mut Segment> {
        if self.current_split_index >= 0 {
            self.run
                .segments_mut()
                .get_mut(self.current_split_index as usize)
        } else {
            None
        }
    }

    #[inline]
    pub fn current_split_index(&self) -> isize {
        self.current_split_index
    }

    pub fn split(&mut self) {
        if self.phase == NotRunning {
            self.phase = Running;
            self.current_split_index = 0;
            self.attempt_started = Some(AtomicDateTime::now());
            self.adjusted_start_time = TimeStamp::now() - self.run.offset();
            self.start_time = self.adjusted_start_time;
            self.time_paused_at = self.run.offset();
            self.uninitialize_game_time();
            self.run.start_next_run();

            // TODO OnStart
        } else {
            let current_time = self.current_time();
            if self.phase == TimerPhase::Running &&
               current_time
                   .real_time
                   .map_or(false, |t| t >= TimeSpan::zero()) {
                self.current_split_mut()
                    .unwrap()
                    .set_split_time(current_time);
                self.current_split_index += 1;
                if self.run.len() as isize == self.current_split_index {
                    self.phase = TimerPhase::Ended;
                    self.attempt_ended = Some(AtomicDateTime::now());
                }
                self.run.mark_as_changed();

                // TODO OnSplit
            }
        }
    }

    pub fn skip_split(&mut self) {
        if (self.phase == TimerPhase::Running || self.phase == TimerPhase::Paused) &&
           self.current_split_index < self.run.len() as isize - 1 {
            self.current_split_mut().unwrap().clear_split_time();
            self.current_split_index += 1;
            self.run.mark_as_changed();

            // TODO OnSkipSplit
        }
    }

    pub fn undo_split(&mut self) {
        if self.phase != TimerPhase::NotRunning && self.current_split_index > 0 {
            if self.phase == TimerPhase::Ended {
                self.phase = TimerPhase::Running;
            }
            self.current_split_index -= 1;
            self.current_split_mut().unwrap().clear_split_time();
            self.run.mark_as_changed();

            // TODO OnUndoSplit
        }
    }

    pub fn reset(&mut self, update_splits: bool) {
        if self.phase != TimerPhase::NotRunning {
            if self.phase != TimerPhase::Ended {
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
        self.phase = TimerPhase::NotRunning;
        self.current_split_index = -1;

        // Reset Splits
        for segment in self.run.segments_mut() {
            segment.clear_split_time();
        }

        // TODO OnReset
    }

    pub fn pause(&mut self) {
        match self.phase {
            TimerPhase::Running => {
                self.time_paused_at = self.current_time().real_time.unwrap();
                self.phase = TimerPhase::Paused;

                // TODO OnPause
            }
            TimerPhase::Paused => {
                self.adjusted_start_time = TimeStamp::now() - self.time_paused_at;
                self.phase = TimerPhase::Running;

                // TODO OnResume
            }
            TimerPhase::NotRunning => self.split(), // Fuck abahbob
            _ => {}
        }
    }

    pub fn undo_all_pauses(&mut self) {
        match self.current_phase() {
            TimerPhase::Paused => self.pause(),
            TimerPhase::Ended => {
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

        self.adjusted_start_time = self.start_time;

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

    pub fn get_pause_time(&self) -> Option<TimeSpan> {
        if self.phase != TimerPhase::NotRunning && self.start_time != self.adjusted_start_time {
            Some(self.adjusted_start_time - self.start_time)
        } else {
            None
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
            self.set_loading_times(TimeSpan::option_op(current_time.real_time,
                                                       current_time.game_time,
                                                       |r, g| r - g)
                                           .unwrap_or_default());
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
        let (time, pause_time) = if self.phase == TimerPhase::Ended {
            (self.current_time(), self.get_pause_time())
        } else {
            Default::default()
        };
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
                       .map_or(true, |b| current_segment.map_or(false, |c| c < b)) {
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
                       .map_or(true, |b| current_segment.map_or(false, |c| c < b)) {
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
            (last_segment.split_time()[method], last_segment.personal_best_split_time()[method])
        };
        if split_time.map_or(false, |s| pb_split_time.map_or(true, |pb| s < pb)) {
            self.set_run_as_pb();
        }
    }

    fn update_segment_history(&mut self) {
        self.run.update_segment_history(self.current_split_index);
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
