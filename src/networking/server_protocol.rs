//! The server protocol is an experimental JSON based protocol that is used to
//! remotely control the timer. Every command that you send has a response in
//! the form of a JSON object indicating whether the command was successful or
//! not.
//!
//! A command looks like this:
//! ```json
//! { "command": "SplitOrStart" }
//! ```
//!
//! Additional parameters can be added to the command, depending on what command
//! it is. For example:
//! ```json
//! { "command": "SetGameTime", "time": "1:23:45" }
//! ```
//!
//! If the command was successful, the response looks like this:
//! ```json
//! { "success": null }
//! ```
//!
//! If the command was to retrieve information, the value is there instead of
//! `null`.
//!
//! An error is indicated by the following JSON object:
//! ```json
//! { "error": { "code": "NoRunInProgress" } }
//! ```
//!
//! An optional `message` field may be present to provide additional information
//! about the error.
//!
//! You are also sent events that indicate changes in the timer. The events look
//! like this:
//! ```json
//! { "event": "Splitted" }
//! ```
//!
//! The events are currently also sent as responses to commands, but this may
//! change in the future. So for example, if you split the timer, you will
//! receive a response like this:
//! ```json
//! { "event": "Splitted" }
//! { "success": null }
//! ```
//!
//! This does not mean that two splits happened.
//!
//! Keep in mind the experimental nature of the protocol. It will likely change
//! a lot in the future.

use alloc::borrow::Cow;
use serde::Serializer;

use crate::{
    event::{self, Event},
    timing::formatter::{self, TimeFormatter, ASCII_MINUS},
    TimeSpan, Timer, TimerPhase, TimingMethod,
};

/// Handles an incoming command and returns the response to be sent.
pub async fn handle_command<S: event::CommandSink + event::TimerQuery>(
    command: &str,
    command_sink: &S,
) -> String {
    let response = match serde_json::from_str::<Command<'_>>(command) {
        Ok(command) => command.handle(command_sink).await.into(),
        Err(e) => CommandResult::Error(Error::InvalidCommand {
            message: e.to_string(),
        }),
    };

    serde_json::to_string(&response).unwrap()
}

/// Encodes an event that happened to be sent.
pub fn encode_event(event: Event) -> String {
    serde_json::to_string(&IsEvent { event }).unwrap()
}

#[derive(serde_derive::Serialize)]
#[serde(rename_all = "camelCase")]
enum CommandResult<T, E> {
    Success(T),
    Error(E),
}

impl<T, E> From<Result<T, E>> for CommandResult<T, E> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(value) => CommandResult::Success(value),
            Err(error) => CommandResult::Error(error),
        }
    }
}

#[derive(serde_derive::Serialize)]
struct IsEvent {
    event: Event,
}

fn serialize_time_span<S: Serializer>(
    time_span: &TimeSpan,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let (secs, nanos) = time_span.to_seconds_and_subsec_nanoseconds();
    serializer.collect_str(&format_args!("{secs}.{:09}", nanos.abs()))
}

const fn is_false(v: &bool) -> bool {
    !*v
}

/// A command that can be sent to the timer.
#[derive(Clone, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(tag = "command", rename_all = "camelCase")]
pub enum Command<'a> {
    /// Starts the timer if there is no attempt in progress. If that's not the
    /// case, nothing happens.
    Start,
    /// If an attempt is in progress, stores the current time as the time of the
    /// current split. The attempt ends if the last split time is stored.
    Split,
    /// Starts a new attempt or stores the current time as the time of the
    /// current split. The attempt ends if the last split time is stored.
    SplitOrStart,
    /// Resets the current attempt if there is one in progress. If the splits
    /// are to be updated, all the information of the current attempt is stored
    /// in the run's history. Otherwise the current attempt's information is
    /// discarded.
    #[serde(rename_all = "camelCase")]
    Reset {
        /// Whether to save the current attempt in the run's history.
        #[serde(skip_serializing_if = "Option::is_none")]
        save_attempt: Option<bool>,
    },
    /// Removes the split time from the last split if an attempt is in progress
    /// and there is a previous split. The Timer Phase also switches to
    /// [`Running`](TimerPhase::Running) if it previously was
    /// [`Ended`](TimerPhase::Ended).
    UndoSplit,
    /// Skips the current split if an attempt is in progress and the current
    /// split is not the last split.
    SkipSplit,
    /// Toggles an active attempt between [`Paused`](TimerPhase::Paused) and
    /// [`Running`](TimerPhase::Paused) or starts an attempt if there's none in
    /// progress.
    TogglePauseOrStart,
    /// Pauses an active attempt that is not paused.
    Pause,
    /// Resumes an attempt that is paused.
    Resume,
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
    UndoAllPauses,
    /// Switches the current comparison to the previous comparison in the list.
    SwitchToPreviousComparison,
    /// Switches the current comparison to the next comparison in the list.
    SwitchToNextComparison,
    /// Tries to set the current comparison to the comparison specified. If the
    /// comparison doesn't exist an error is returned.
    SetCurrentComparison {
        /// The name of the comparison.
        #[serde(borrow)]
        comparison: Cow<'a, str>,
    },
    /// Toggles between the `Real Time` and `Game Time` timing methods.
    ToggleTimingMethod,
    /// Sets the current timing method to the timing method provided.
    #[serde(rename_all = "camelCase")]
    SetCurrentTimingMethod {
        /// The timing method to use.
        timing_method: TimingMethod,
    },
    /// Initializes game time for the current attempt. Game time automatically
    /// gets uninitialized for each new attempt.
    InitializeGameTime,
    /// Sets the game time to the time specified. This also works if the game
    /// time is paused, which can be used as a way of updating the game timer
    /// periodically without it automatically moving forward. This ensures that
    /// the game timer never shows any time that is not coming from the game.
    SetGameTime {
        /// The time to set the game time to.
        #[serde(serialize_with = "serialize_time_span")]
        time: TimeSpan,
    },
    /// Pauses the game timer such that it doesn't automatically increment
    /// similar to real time.
    PauseGameTime,
    /// Resumes the game timer such that it automatically increments similar to
    /// real time, starting from the game time it was paused at.
    ResumeGameTime,
    /// Instead of setting the game time directly, this method can be used to
    /// just specify the amount of time the game has been loading. The game time
    /// is then automatically determined by Real Time - Loading Times.
    SetLoadingTimes {
        /// The loading times to set the game time to.
        #[serde(serialize_with = "serialize_time_span")]
        time: TimeSpan,
    },
    /// Sets the value of a custom variable with the name specified. If the
    /// variable does not exist, a temporary variable gets created that will not
    /// be stored in the splits file.
    SetCustomVariable {
        /// The name of the custom variable.
        #[serde(borrow)]
        key: Cow<'a, str>,
        /// The value of the custom variable.
        #[serde(borrow)]
        value: Cow<'a, str>,
    },

    /// Returns the timer's current time. The Game Time is [`None`] if the Game
    /// Time has not been initialized.
    #[serde(rename_all = "camelCase")]
    GetCurrentTime {
        /// The timing method to retrieve the time for.
        #[serde(skip_serializing_if = "Option::is_none")]
        timing_method: Option<TimingMethod>,
    },
    /// Returns the name of the segment with the specified index. If no index is
    /// specified, the name of the current segment is returned. If the index is
    /// out of bounds, an error is returned. If the index is negative, it is
    /// treated as relative to the end of the segment list. If the `relative`
    /// field is set to `true`, the index is treated as relative to the current
    /// segment index.
    GetSegmentName {
        /// The index of the segment.
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<isize>,
        /// Specifies whether the index is relative to the current segment
        /// index.
        #[serde(default, skip_serializing_if = "is_false")]
        relative: bool,
    },
    /// Returns the time of the comparison with the specified name for the
    /// segment with the specified index. If the segment index is out of bounds,
    /// an error is returned. If the segment index is negative, it is treated as
    /// relative to the end of the segment list. If the `relative` field is set
    /// to `true`, the index is treated as relative to the current segment
    /// index. The current comparison is used if the comparison name is not
    /// specified. The current timing method is used if the timing method is not
    /// specified.
    #[serde(rename_all = "camelCase")]
    GetComparisonTime {
        /// The index of the segment.
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<isize>,
        /// Specifies whether the index is relative to the current segment
        /// index.
        #[serde(default, skip_serializing_if = "is_false")]
        relative: bool,
        /// The name of the comparison.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(borrow)]
        comparison: Option<Cow<'a, str>>,
        /// The timing method to retrieve the time for.
        #[serde(skip_serializing_if = "Option::is_none")]
        timing_method: Option<TimingMethod>,
    },
    /// Returns the current split time of the segment with the specified index
    /// and timing method. If the segment index is out of bounds, an error is
    /// returned. If the segment index is negative, it is treated as relative to
    /// the end of the segment list. If the `relative` field is set to `true`,
    /// the index is treated as relative to the current segment index. The
    /// current timing method is used if the timing method is not specified.
    #[serde(rename_all = "camelCase")]
    GetCurrentRunSplitTime {
        /// The index of the segment.
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<isize>,
        /// Specifies whether the index is relative to the current segment
        /// index.
        #[serde(default, skip_serializing_if = "is_false")]
        relative: bool,
        /// The timing method to retrieve the time for.
        #[serde(skip_serializing_if = "Option::is_none")]
        timing_method: Option<TimingMethod>,
    },
    /// Returns the current timer phase and split index.
    GetCurrentState,
    /// Pings the application to check whether it is still running.
    Ping,
}

#[derive(serde_derive::Serialize)]
#[serde(tag = "state", content = "index")]
enum State {
    NotRunning,
    Running(usize),
    Paused(usize),
    Ended,
}

#[derive(serde_derive::Serialize)]
#[serde(untagged)]
enum Response {
    None,
    String(String),
    State(State),
}

#[derive(serde_derive::Serialize)]
#[serde(tag = "code")]
enum Error {
    InvalidCommand {
        message: String,
    },
    InvalidIndex,
    #[serde(untagged)]
    Timer {
        code: event::Error,
    },
}

impl Error {
    const fn timer(code: event::Error) -> Self {
        Error::Timer { code }
    }
}

impl Command<'_> {
    async fn handle<E: event::CommandSink + event::TimerQuery>(
        self,
        command_sink: &E,
    ) -> Result<Response, Error> {
        Ok(match self {
            Command::Start => {
                command_sink.start().await.map_err(Error::timer)?;
                Response::None
            }
            Command::Split => {
                command_sink.split().await.map_err(Error::timer)?;
                Response::None
            }
            Command::SplitOrStart => {
                command_sink.split_or_start().await.map_err(Error::timer)?;
                Response::None
            }
            Command::Reset { save_attempt } => {
                command_sink
                    .reset(save_attempt)
                    .await
                    .map_err(Error::timer)?;
                Response::None
            }
            Command::UndoSplit => {
                command_sink.undo_split().await.map_err(Error::timer)?;
                Response::None
            }
            Command::SkipSplit => {
                command_sink.skip_split().await.map_err(Error::timer)?;
                Response::None
            }
            Command::TogglePauseOrStart => {
                command_sink
                    .toggle_pause_or_start()
                    .await
                    .map_err(Error::timer)?;
                Response::None
            }
            Command::Pause => {
                command_sink.pause().await.map_err(Error::timer)?;
                Response::None
            }
            Command::Resume => {
                command_sink.resume().await.map_err(Error::timer)?;
                Response::None
            }
            Command::UndoAllPauses => {
                command_sink.undo_all_pauses().await.map_err(Error::timer)?;
                Response::None
            }
            Command::SwitchToPreviousComparison => {
                command_sink
                    .switch_to_previous_comparison()
                    .await
                    .map_err(Error::timer)?;
                Response::None
            }
            Command::SwitchToNextComparison => {
                command_sink
                    .switch_to_next_comparison()
                    .await
                    .map_err(Error::timer)?;
                Response::None
            }
            Command::SetCurrentComparison { comparison } => {
                command_sink
                    .set_current_comparison(comparison)
                    .await
                    .map_err(Error::timer)?;
                Response::None
            }
            Command::ToggleTimingMethod => {
                command_sink
                    .toggle_timing_method()
                    .await
                    .map_err(Error::timer)?;
                Response::None
            }
            Command::SetCurrentTimingMethod { timing_method } => {
                command_sink
                    .set_current_timing_method(timing_method)
                    .await
                    .map_err(Error::timer)?;
                Response::None
            }
            Command::InitializeGameTime => {
                command_sink
                    .initialize_game_time()
                    .await
                    .map_err(Error::timer)?;
                Response::None
            }
            Command::SetGameTime { time } => {
                command_sink
                    .set_game_time(time)
                    .await
                    .map_err(Error::timer)?;
                Response::None
            }
            Command::PauseGameTime => {
                command_sink.pause_game_time().await.map_err(Error::timer)?;
                Response::None
            }
            Command::ResumeGameTime => {
                command_sink
                    .resume_game_time()
                    .await
                    .map_err(Error::timer)?;
                Response::None
            }
            Command::SetLoadingTimes { time } => {
                command_sink
                    .set_loading_times(time)
                    .await
                    .map_err(Error::timer)?;
                Response::None
            }
            Command::SetCustomVariable { key, value } => {
                command_sink
                    .set_custom_variable(key, value)
                    .await
                    .map_err(Error::timer)?;
                Response::None
            }

            Command::GetCurrentTime { timing_method } => {
                let guard = command_sink.get_timer();
                let timer = &*guard;

                let timing_method = timing_method.unwrap_or_else(|| timer.current_timing_method());
                let time = timer.snapshot().current_time()[timing_method];
                if let Some(time) = time {
                    Response::String(format_time(time))
                } else {
                    Response::None
                }
            }
            Command::GetSegmentName { index, relative } => {
                let guard = command_sink.get_timer();
                let timer = &*guard;
                let index = resolve_index(timer, index, relative)?;
                Response::String(timer.run().segment(index).name().into())
            }
            Command::GetComparisonTime {
                index,
                relative,
                comparison,
                timing_method,
            } => {
                let guard = command_sink.get_timer();
                let timer = &*guard;
                let index = resolve_index(timer, index, relative)?;
                let timing_method = timing_method.unwrap_or_else(|| timer.current_timing_method());

                let comparison = comparison.as_deref().unwrap_or(timer.current_comparison());

                let time = timer.run().segment(index).comparison(comparison)[timing_method];
                if let Some(time) = time {
                    Response::String(format_time(time))
                } else {
                    Response::None
                }
            }
            Command::GetCurrentRunSplitTime {
                index,
                relative,
                timing_method,
            } => {
                let guard = command_sink.get_timer();
                let timer = &*guard;
                let index = resolve_index(timer, index, relative)?;
                let timing_method = timing_method.unwrap_or_else(|| timer.current_timing_method());

                let time = timer.run().segment(index).split_time()[timing_method];
                if let Some(time) = time {
                    Response::String(format_time(time))
                } else {
                    Response::None
                }
            }
            Command::GetCurrentState => {
                let guard = command_sink.get_timer();
                let timer = &*guard;
                let phase = timer.current_phase();
                Response::State(match phase {
                    TimerPhase::NotRunning => State::NotRunning,
                    TimerPhase::Running => State::Running(timer.current_split_index().unwrap()),
                    TimerPhase::Paused => State::Paused(timer.current_split_index().unwrap()),
                    TimerPhase::Ended => State::Ended,
                })
            }
            Command::Ping => Response::None,
        })
    }
}

fn resolve_index(timer: &Timer, index: Option<isize>, relative: bool) -> Result<usize, Error> {
    let index = if let Some(index) = index {
        let base = if relative {
            timer.current_split_index().ok_or(Error::InvalidIndex)?
        } else if index < 0 {
            timer.run().len()
        } else {
            0
        };
        base.checked_add_signed(index)
    } else {
        timer.current_split_index()
    }
    .ok_or(Error::InvalidIndex)?;

    if index >= timer.run().len() {
        Err(Error::InvalidIndex)
    } else {
        Ok(index)
    }
}

fn format_time(time: TimeSpan) -> String {
    // FIXME: I don't think we can parse it again if days are included. Let's not
    // use this formatter.
    formatter::none_wrapper::NoneWrapper::new(formatter::Complete::new(), ASCII_MINUS)
        .format(time)
        .to_string()
}
