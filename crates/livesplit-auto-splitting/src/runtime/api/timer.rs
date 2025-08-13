use anyhow::Result;
use wasmtime::{Caller, Linker};

use crate::{runtime::Context, CreationError, Timer};

use super::{get_str, memory_and_context};

pub fn bind<T: Timer>(linker: &mut Linker<Context<T>>) -> Result<(), CreationError> {
    linker
        .func_wrap("env", "timer_get_state", {
            |caller: Caller<'_, Context<T>>| caller.data().timer.state() as u32
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "timer_get_state",
        })?
        .func_wrap(
            "env",
            "timer_start",
            |mut caller: Caller<'_, Context<T>>| {
                caller.data_mut().timer.start();
            },
        )
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "timer_start",
        })?
        .func_wrap(
            "env",
            "timer_split",
            |mut caller: Caller<'_, Context<T>>| {
                caller.data_mut().timer.split();
            },
        )
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "timer_split",
        })?
        .func_wrap(
            "env",
            "timer_skip_split",
            |mut caller: Caller<'_, Context<T>>| {
                caller.data_mut().timer.skip_split();
            },
        )
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "timer_skip_split",
        })?
        .func_wrap(
            "env",
            "timer_undo_split",
            |mut caller: Caller<'_, Context<T>>| {
                caller.data_mut().timer.undo_split();
            },
        )
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "timer_undo_split",
        })?
        .func_wrap(
            "env",
            "timer_reset",
            |mut caller: Caller<'_, Context<T>>| {
                caller.data_mut().timer.reset();
            },
        )
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "timer_reset",
        })?
        .func_wrap("env", "timer_current_split_index", {
            |caller: Caller<'_, Context<T>>| {
                caller
                    .data()
                    .timer
                    .current_split_index()
                    .map_or(-1, |i| i as i32)
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "timer_current_split_index",
        })?
        .func_wrap("env", "timer_set_variable", {
            |mut caller: Caller<'_, Context<T>>,
             name_ptr: u32,
             name_len: u32,
             value_ptr: u32,
             value_len: u32|
             -> Result<()> {
                let (memory, context) = memory_and_context(&mut caller);
                let name = get_str(memory, name_ptr, name_len)?;
                let value = get_str(memory, value_ptr, value_len)?;
                context.timer.set_variable(name, value);
                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "timer_set_variable",
        })?
        .func_wrap("env", "timer_set_game_time", {
            |mut caller: Caller<'_, Context<T>>, secs: i64, nanos: i32| {
                caller
                    .data_mut()
                    .timer
                    .set_game_time(time::Duration::new(secs, nanos));
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "timer_set_game_time",
        })?
        .func_wrap("env", "timer_pause_game_time", {
            |mut caller: Caller<'_, Context<T>>| caller.data_mut().timer.pause_game_time()
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "timer_pause_game_time",
        })?
        .func_wrap("env", "timer_resume_game_time", {
            |mut caller: Caller<'_, Context<T>>| caller.data_mut().timer.resume_game_time()
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "timer_resume_game_time",
        })?;
    Ok(())
}
