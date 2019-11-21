//! The cleaning module provides the Sum of Best Cleaner which allows you to
//! interactively remove potential issues in the Segment History that lead to an
//! inaccurate Sum of Best. If you skip a split, whenever you get to the next
//! split, the combined segment time might be faster than the sum of the
//! individual best segments. The Sum of Best Cleaner will point out all
//! occurrences of this and allows you to delete them individually if any of
//! them seem wrong.

use crate::analysis::sum_of_segments::{best, track_branch, Prediction};
use crate::platform::prelude::*;
use crate::platform::Local;
use crate::timing::formatter::{Short, TimeFormatter};
use crate::{Attempt, Run, Segment, TimeSpan, TimingMethod};
use core::fmt;
use core::mem::replace;

/// A Sum of Best Cleaner allows you to interactively remove potential issues in
/// the Segment History that lead to an inaccurate Sum of Best. If you skip a
/// split, whenever you get to the next split, the combined segment time might
/// be faster than the sum of the individual best segments. The Sum of Best
/// Cleaner will point out all occurrences of this and allows you to delete them
/// individually if any of them seem wrong.
pub struct SumOfBestCleaner<'r> {
    run: &'r mut Run,
    predictions: Vec<Option<Prediction>>,
    state: State,
}

enum State {
    Poisoned,
    Done,
    WithTimingMethod(TimingMethod),
    IteratingRun(IteratingRunState),
    IteratingHistory(IteratingHistoryState),
}

struct IteratingRunState {
    method: TimingMethod,
    segment_index: usize,
}

struct IteratingHistoryState {
    parent: IteratingRunState,
    current_time: Option<TimeSpan>,
    skip_count: usize,
}

/// Describes a potential clean up that could be applied. You can use the
/// Display implementation to print out the details of this potential clean up.
/// A potential clean up can then be turned into an actual clean up in order to
/// apply it to the Run.
pub struct PotentialCleanUp<'r> {
    starting_segment: Option<&'r Segment>,
    ending_segment: &'r Segment,
    time_between: TimeSpan,
    combined_sum_of_best: Option<TimeSpan>,
    attempt: &'r Attempt,
    method: TimingMethod,
    clean_up: CleanUp,
}

/// Describes an actual clean up that is about to be applied.
pub struct CleanUp {
    ending_index: usize,
    run_index: i32,
}

impl fmt::Display for PotentialCleanUp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let short = Short::new();

        let method = match self.method {
            TimingMethod::RealTime => "Real Time",
            TimingMethod::GameTime => "Game Time",
        };

        write!(
            f,
            "You had a {} segment time of {} between ",
            method,
            short.format(self.time_between)
        )?;

        if let Some(starting_segment) = self.starting_segment {
            write!(f, "{}", starting_segment.name())?;
        } else {
            write!(f, "the start of the run")?;
        }

        write!(f, " and {}", self.ending_segment.name())?;

        if let Some(combined) = self.combined_sum_of_best {
            write!(
                f,
                ", which is faster than the Combined Best Segments of {}",
                short.format(combined)
            )?;
        }

        if let Some(ended) = self.attempt.ended() {
            write!(
                f,
                " in a run on {}",
                ended.time.with_timezone(&Local).format("%F")
            )?;
        }

        write!(
            f,
            ". Do you think that this segment time is inaccurate and should be removed?"
        )
    }
}

impl From<PotentialCleanUp<'_>> for CleanUp {
    fn from(potential: PotentialCleanUp<'_>) -> Self {
        potential.clean_up
    }
}

impl<'r> SumOfBestCleaner<'r> {
    /// Creates a new Sum of Best Cleaner for the provided Run object.
    pub fn new(run: &'r mut Run) -> Self {
        let predictions = Vec::with_capacity(run.len() + 1);
        Self {
            run,
            predictions,
            state: State::WithTimingMethod(TimingMethod::RealTime),
        }
    }

    /// Applies a clean up to the Run.
    #[allow(clippy::needless_pass_by_value)]
    pub fn apply(&mut self, clean_up: CleanUp) {
        self.run
            .segment_mut(clean_up.ending_index)
            .segment_history_mut()
            .remove(clean_up.run_index);

        self.run.mark_as_modified();
    }

    /// Returns the next potential clean up. If there are no more potential
    /// clean ups, `None` is returned.
    pub fn next_potential_clean_up(&mut self) -> Option<PotentialCleanUp<'_>> {
        loop {
            match replace(&mut self.state, State::Poisoned) {
                State::Poisoned => unreachable!(),
                State::Done => return None,
                State::WithTimingMethod(method) => {
                    next_timing_method(&self.run, &mut self.predictions, method);
                    self.state = State::IteratingRun(IteratingRunState {
                        method,
                        segment_index: 0,
                    });
                }
                State::IteratingRun(state) => {
                    self.state = if state.segment_index < self.run.len() {
                        let current_prediction = self.predictions[state.segment_index];
                        State::IteratingHistory(IteratingHistoryState {
                            parent: state,
                            current_time: current_prediction.map(|p| p.time),
                            skip_count: 0,
                        })
                    } else if state.method == TimingMethod::RealTime {
                        State::WithTimingMethod(TimingMethod::GameTime)
                    } else {
                        State::Done
                    };
                }
                State::IteratingHistory(state) => {
                    let iter = self
                        .run
                        .segment(state.parent.segment_index)
                        .segment_history()
                        .iter()
                        .enumerate()
                        .skip(state.skip_count);

                    for (skip_count, &(run_index, time)) in iter {
                        if time[state.parent.method].is_none() {
                            let (prediction_index, prediction_time) = track_branch(
                                self.run.segments(),
                                state.current_time,
                                state.parent.segment_index + 1,
                                run_index,
                                state.parent.method,
                            );
                            if prediction_index > 0 {
                                if let Some(question) = check_prediction(
                                    &self.run,
                                    &self.predictions,
                                    prediction_time[state.parent.method],
                                    state.parent.segment_index as isize - 1,
                                    prediction_index - 1,
                                    run_index,
                                    state.parent.method,
                                ) {
                                    self.state = State::IteratingHistory(IteratingHistoryState {
                                        skip_count: skip_count + 1,
                                        ..state
                                    });
                                    return Some(question);
                                }
                            }
                        }
                    }
                    self.state = State::IteratingRun(IteratingRunState {
                        method: state.parent.method,
                        segment_index: state.parent.segment_index + 1,
                    });
                }
            };
        }
    }
}

fn check_prediction<'a>(
    run: &'a Run,
    predictions: &[Option<Prediction>],
    predicted_time: Option<TimeSpan>,
    starting_index: isize,
    ending_index: usize,
    run_index: i32,
    method: TimingMethod,
) -> Option<PotentialCleanUp<'a>> {
    if let Some(predicted_time) = predicted_time {
        if predictions[ending_index + 1].map_or(true, |p| predicted_time < p.time) {
            if let Some(segment_history_element) =
                run.segment(ending_index).segment_history().get(run_index)
            {
                return Some(PotentialCleanUp {
                    starting_segment: if starting_index >= 0 {
                        Some(run.segment(starting_index as usize))
                    } else {
                        None
                    },
                    ending_segment: run.segment(ending_index),
                    time_between: segment_history_element[method]
                        .expect("Cleanup path is shorter but doesn't have a time"),
                    combined_sum_of_best: predictions[ending_index + 1].map(|p| {
                        p.time
                            - predictions[(starting_index + 1) as usize]
                                .expect("Start time must not be empty")
                                .time
                    }),
                    attempt: run
                        .attempt_history()
                        .iter()
                        .find(|attempt| attempt.index() == run_index)
                        .expect("The attempt has to exist"),
                    method,
                    clean_up: CleanUp {
                        ending_index,
                        run_index,
                    },
                });
            }
        }
    }
    None
}

fn next_timing_method(run: &Run, predictions: &mut Vec<Option<Prediction>>, method: TimingMethod) {
    let segments = run.segments();

    predictions.clear();
    predictions.resize(segments.len() + 1, None);
    best::calculate(segments, predictions, true, false, method);
}
