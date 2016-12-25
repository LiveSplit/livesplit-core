use {AtomicDateTime, Run, Time, TimerPhase, TimeStamp, TimeSpan, Segment};

use TimerPhase::*;

#[derive(Clone)]
pub struct Timer {
    run: Run,
    phase: TimerPhase,
    current_split_index: isize,
    attempt_started: Option<AtomicDateTime>,
    attempt_ended: Option<AtomicDateTime>,
    start_time: Option<TimeStamp>,
    pause_time: Option<TimeSpan>,
}

impl Timer {
    pub fn current_time(&self) -> Time {
        let real_time = match self.phase {
            NotRunning => Some(TimeSpan::zero()),
            Running => Some(TimeStamp::now() - self.start_time.unwrap()),
            Paused => self.pause_time,
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

    pub fn start(&mut self) {
        if self.phase == NotRunning {
            self.phase = Running;
            self.current_split_index = 0;
            self.attempt_started = Some(AtomicDateTime::now());
            self.start_time = Some(TimeStamp::now() - self.run.offset());
            self.pause_time = Some(self.run.offset());
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
            self.start_time = Some(TimeStamp::now());

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
        let old_phase = self.phase;
        self.phase = TimerPhase::NotRunning;
        self.current_split_index = -1;

        // Reset Splits
        for segment in self.run.segments_mut() {
            segment.clear_split_time();
        }

        // TODO OnReset
    }

    fn pause(&mut self) {
        match self.phase {
            TimerPhase::Running => {
                self.pause_time = self.current_time().real_time;
                self.phase = TimerPhase::Paused;

                // TODO OnPause
            }
            TimerPhase::Paused => {
                self.start_time = Some(TimeStamp::now() - self.pause_time.unwrap());
                self.phase = TimerPhase::Running;

                // TODO OnResume
            }
            TimerPhase::NotRunning => self.start(), // Fuck abahbob
            _ => {}
        }
    }

    // TODO Remaining stuff
}
