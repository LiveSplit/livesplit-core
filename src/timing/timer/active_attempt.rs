use crate::{AtomicDateTime, Run, Time, TimeSpan, TimeStamp, TimingMethod};

#[derive(Debug, Clone)]
pub struct ActiveAttempt {
    pub state: State,
    pub attempt_started: AtomicDateTime,
    pub start_time: TimeStamp,
    pub start_time_with_offset: TimeStamp,
    // This gets adjusted after resuming
    pub adjusted_start_time: TimeStamp,
    pub game_time_paused_at: Option<TimeSpan>,
    pub loading_times: Option<TimeSpan>,
}

#[derive(Debug, Clone)]
pub enum State {
    NotEnded {
        current_split_index: usize,
        time_paused_at: Option<TimeSpan>,
    },
    Ended {
        attempt_ended: AtomicDateTime,
    },
}

pub struct TimerTime {
    pub real_time: TimeSpan,
    pub game_time: Option<TimeSpan>,
}

impl From<TimerTime> for Time {
    fn from(time: TimerTime) -> Self {
        Time {
            real_time: Some(time.real_time),
            game_time: time.game_time,
        }
    }
}

impl ActiveAttempt {
    pub fn current_time(&self, run: &Run) -> TimerTime {
        let real_time = match self.state {
            State::Ended { .. } => {
                let Time {
                    real_time,
                    game_time,
                } = run.segments().last().unwrap().split_time();

                return TimerTime {
                    real_time: real_time.unwrap_or_default(),
                    game_time,
                };
            }
            State::NotEnded { time_paused_at, .. } => {
                time_paused_at.unwrap_or_else(|| TimeStamp::now() - self.adjusted_start_time)
            }
        };

        let game_time = self
            .game_time_paused_at
            .or_else(|| Some(real_time - self.loading_times?));

        TimerTime {
            real_time,
            game_time,
        }
    }

    pub fn get_pause_time(&self) -> Option<TimeSpan> {
        if let State::NotEnded {
            time_paused_at: Some(pause_time),
            ..
        } = self.state
        {
            return Some(TimeStamp::now() - self.start_time_with_offset - pause_time);
        }

        if self.start_time_with_offset != self.adjusted_start_time {
            Some(self.start_time_with_offset - self.adjusted_start_time)
        } else {
            None
        }
    }

    pub fn set_loading_times(&mut self, time: TimeSpan, run: &Run) {
        self.loading_times = Some(time);
        if self.game_time_paused_at.is_some() {
            self.game_time_paused_at = Some(self.current_time(run).real_time - time);
        }
    }

    pub fn prepare_split(&mut self, run: &Run) -> Option<(usize, Time)> {
        let State::NotEnded {
            current_split_index,
            time_paused_at: None,
        } = &mut self.state
        else {
            return None;
        };

        let real_time = TimeStamp::now() - self.adjusted_start_time;

        if real_time < TimeSpan::zero() {
            return None;
        }

        let game_time = self
            .game_time_paused_at
            .or_else(|| Some(real_time - self.loading_times?));

        let previous_split_index = *current_split_index;
        *current_split_index += 1;

        if *current_split_index == run.len() {
            self.state = State::Ended {
                attempt_ended: AtomicDateTime::now(),
            };
        }

        Some((
            previous_split_index,
            Time {
                real_time: Some(real_time),
                game_time,
            },
        ))
    }

    pub const fn current_split_index(&self) -> Option<usize> {
        match self.state {
            State::NotEnded {
                current_split_index,
                ..
            } => Some(current_split_index),
            State::Ended { .. } => None,
        }
    }

    pub fn current_split_index_mut(&mut self) -> Option<&mut usize> {
        match &mut self.state {
            State::NotEnded {
                current_split_index,
                ..
            } => Some(current_split_index),
            State::Ended { .. } => None,
        }
    }

    pub fn current_split_index_overflowing(&self, run: &Run) -> usize {
        match self.state {
            State::NotEnded {
                current_split_index,
                ..
            } => current_split_index,
            State::Ended { .. } => run.len(),
        }
    }

    pub fn update_times(&self, run: &mut Run, timing_method: TimingMethod) {
        self.update_attempt_history(run);
        update_best_segments(run);
        update_pb_splits(run, timing_method);
        run.update_segment_history(self.current_split_index_overflowing(run));
    }

    pub fn update_attempt_history(&self, run: &mut Run) {
        let (attempt_ended, time) = match self.state {
            State::NotEnded { .. } => (AtomicDateTime::now(), Time::new()),
            State::Ended { attempt_ended } => {
                (attempt_ended, run.segments().last().unwrap().split_time())
            }
        };

        let pause_time = self.get_pause_time();

        run.add_attempt(
            time,
            Some(self.attempt_started),
            Some(attempt_ended),
            pause_time,
        );
    }
}

fn update_best_segments(run: &mut Run) {
    let mut previous_split_time_rta = Some(TimeSpan::zero());
    let mut previous_split_time_game_time = Some(TimeSpan::zero());

    for split in run.segments_mut() {
        let mut new_best_segment = split.best_segment_time();
        if let Some(split_time) = split.split_time().real_time {
            let current_segment = previous_split_time_rta.map(|previous| split_time - previous);
            previous_split_time_rta = Some(split_time);
            if split
                .best_segment_time()
                .real_time
                .map_or(true, |b| current_segment.is_some_and(|c| c < b))
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
                .map_or(true, |b| current_segment.is_some_and(|c| c < b))
            {
                new_best_segment.game_time = current_segment;
            }
        }
        split.set_best_segment_time(new_best_segment);
    }
}

fn update_pb_splits(run: &mut Run, method: TimingMethod) {
    let (split_time, pb_split_time) = {
        let last_segment = run.segments().last().unwrap();
        (
            last_segment.split_time()[method],
            last_segment.personal_best_split_time()[method],
        )
    };
    if split_time.is_some_and(|s| pb_split_time.map_or(true, |pb| s < pb)) {
        super::set_run_as_pb(run);
    }
}
