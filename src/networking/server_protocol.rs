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
    let response = match serde_json::from_str::<Command>(command) {
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

#[derive(serde_derive::Deserialize)]
#[serde(tag = "command", rename_all = "camelCase")]
enum Command {
    SplitOrStart,
    Split,
    UndoSplit,
    SkipSplit,
    Pause,
    Resume,
    TogglePauseOrStart,
    Reset,
    Start,
    InitializeGameTime,
    SetGameTime {
        time: TimeSpan,
    },
    SetLoadingTimes {
        time: TimeSpan,
    },
    PauseGameTime,
    ResumeGameTime,
    SetCustomVariable {
        key: String,
        value: String,
    },
    SetCurrentComparison {
        comparison: String,
    },
    #[serde(rename_all = "camelCase")]
    SetCurrentTimingMethod {
        timing_method: TimingMethod,
    },
    #[serde(rename_all = "camelCase")]
    GetCurrentTime {
        timing_method: Option<TimingMethod>,
    },
    GetSegmentName {
        index: Option<isize>,
        #[serde(default)]
        relative: bool,
    },
    #[serde(rename_all = "camelCase")]
    GetComparisonTime {
        index: Option<isize>,
        #[serde(default)]
        relative: bool,
        comparison: Option<String>,
        timing_method: Option<TimingMethod>,
    },
    #[serde(rename_all = "camelCase")]
    GetCurrentRunSplitTime {
        index: Option<isize>,
        #[serde(default)]
        relative: bool,
        timing_method: Option<TimingMethod>,
    },
    GetCurrentState,
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

impl Command {
    async fn handle<E: event::CommandSink + event::TimerQuery>(
        &self,
        command_sink: &E,
    ) -> Result<Response, Error> {
        Ok(match self {
            Command::SplitOrStart => {
                command_sink.split_or_start().await.map_err(Error::timer)?;
                Response::None
            }
            Command::Split => {
                command_sink.split().await.map_err(Error::timer)?;
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
            Command::Pause => {
                command_sink.pause().await.map_err(Error::timer)?;
                Response::None
            }
            Command::Resume => {
                command_sink.resume().await.map_err(Error::timer)?;
                Response::None
            }
            Command::TogglePauseOrStart => {
                command_sink
                    .toggle_pause_or_start()
                    .await
                    .map_err(Error::timer)?;
                Response::None
            }
            Command::Reset => {
                command_sink.reset(None).await.map_err(Error::timer)?;
                Response::None
            }
            Command::Start => {
                command_sink.start().await.map_err(Error::timer)?;
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
                    .set_game_time(*time)
                    .await
                    .map_err(Error::timer)?;
                Response::None
            }
            Command::SetLoadingTimes { time } => {
                command_sink
                    .set_loading_times(*time)
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
            Command::SetCustomVariable { key, value } => {
                command_sink
                    .set_custom_variable(key, value)
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
            Command::SetCurrentTimingMethod { timing_method } => {
                command_sink
                    .set_current_timing_method(*timing_method)
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
                let index = resolve_index(timer, *index, *relative)?;
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
                let index = resolve_index(timer, *index, *relative)?;
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
                let index = resolve_index(timer, *index, *relative)?;
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
    formatter::none_wrapper::NoneWrapper::new(formatter::Complete::new(), ASCII_MINUS)
        .format(time)
        .to_string()
}
