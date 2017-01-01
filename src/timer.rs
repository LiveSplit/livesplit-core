use {AtomicDateTime, Run, Time, TimerPhase, TimingMethod, TimeStamp, TimeSpan, Segment};
use TimerPhase::*;

#[derive(Debug, Clone)]
pub struct Timer {
    run: Run,
    phase: TimerPhase,
    current_split_index: isize,
    current_timing_method: TimingMethod,
    attempt_started: Option<AtomicDateTime>,
    attempt_ended: Option<AtomicDateTime>,
    start_time: TimeStamp,
    pause_time: TimeSpan,
}

impl Timer {
    #[inline]
    pub fn new(run: Run) -> Self {
        Timer {
            run: run,
            phase: TimerPhase::NotRunning,
            current_split_index: -1,
            current_timing_method: TimingMethod::RealTime,
            attempt_started: None,
            attempt_ended: None,
            start_time: TimeStamp::now(),
            pause_time: TimeSpan::zero(),
        }
    }

    #[inline]
    pub fn run(&self) -> &Run {
        &self.run
    }

    pub fn current_time(&self) -> Time {
        let real_time = match self.phase {
            NotRunning => Some(TimeSpan::zero()),
            Running => Some(TimeStamp::now() - self.start_time),
            Paused => Some(self.pause_time),
            Ended => self.run.segments().last().unwrap().split_time().real_time,
        };

        // TODO No Game Time Implementation for now.

        Time::new().with_real_time(real_time)
    }

    fn current_split_mut(&mut self) -> Option<&mut Segment> {
        let segments = self.run.segments_mut();
        if self.current_split_index >= 0 {
            segments.get_mut(self.current_split_index as usize)
        } else {
            None
        }
    }

    #[inline]
    pub fn current_split_index(&self) -> isize {
        self.current_split_index
    }

    pub fn start(&mut self) {
        if self.phase == NotRunning {
            self.phase = Running;
            self.current_split_index = 0;
            self.attempt_started = Some(AtomicDateTime::now());
            self.start_time = TimeStamp::now() - self.run.offset();
            self.pause_time = self.run.offset();
            self.run.start_next_run();

            // TODO OnStart
        }
    }

    pub fn split(&mut self) {
        if self.phase == TimerPhase::Running {
            let current_time = self.current_time();
            self.current_split_mut().unwrap().set_split_time(current_time);
            self.current_split_index += 1;
            if self.run.len() as isize == self.current_split_index {
                self.phase = TimerPhase::Ended;
                self.attempt_ended = Some(AtomicDateTime::now());
            }
            self.run.mark_as_changed();

            // TODO OnSplit
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
            // TODO Handle Game Time
            self.start_time = TimeStamp::now();

            if update_splits {
                self.update_attempt_history();
                self.update_best_segments();
                self.update_pb_splits();
                self.update_segment_history();
            }

            self.reset_splits();

            self.run.fix_splits();
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
                self.pause_time = self.current_time().real_time.unwrap();
                self.phase = TimerPhase::Paused;

                // TODO OnPause
            }
            TimerPhase::Paused => {
                self.start_time = TimeStamp::now() - self.pause_time;
                self.phase = TimerPhase::Running;

                // TODO OnResume
            }
            TimerPhase::NotRunning => self.start(), // Fuck abahbob
            _ => {}
        }
    }

    pub fn switch_to_next_comparison(&mut self) {
        unimplemented!()
    }

    pub fn switch_to_previous_comparison(&mut self) {
        unimplemented!()
    }

    fn update_attempt_history(&mut self) {
        let time = if self.phase == TimerPhase::Ended {
            self.current_time()
        } else {
            Default::default()
        };
        self.run.add_attempt(time, self.attempt_started, self.attempt_ended);
    }

    fn update_best_segments(&mut self) {
        let mut current_segment_rta;
        let mut previous_split_time_rta = None;
        let mut current_segment_game_time;
        let mut previous_split_time_game_time = None;
        for split in self.run.segments_mut() {
            let mut new_best_segment = split.best_segment_time();
            if let Some(split_time) = split.split_time().real_time {
                current_segment_rta = previous_split_time_rta.map(|previous| split_time - previous);
                previous_split_time_rta = Some(split_time);
                if split.best_segment_time()
                    .real_time
                    .map_or(true, |b| current_segment_rta.map_or(false, |c| c < b)) {
                    new_best_segment.real_time = current_segment_rta;
                }
            }
            if let Some(split_time) = split.split_time().game_time {
                current_segment_game_time =
                    previous_split_time_game_time.map(|previous| split_time - previous);
                previous_split_time_game_time = Some(split_time);
                if split.best_segment_time()
                    .game_time
                    .map_or(true, |b| current_segment_game_time.map_or(false, |c| c < b)) {
                    new_best_segment.game_time = current_segment_game_time;
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
