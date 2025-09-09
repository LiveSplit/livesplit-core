use std::{
    env::consts::{ARCH, OS},
    sync::atomic,
};

use anyhow::{Result, ensure};
use wasmtime::{Caller, Linker};

use crate::{CreationError, Timer, runtime::Context, timer::LogLevel};

use super::{get_arr_mut, get_slice_mut, get_str, memory_and_context};

pub fn bind<T: Timer>(linker: &mut Linker<Context<T>>) -> Result<(), CreationError> {
    linker
        .func_wrap("env", "runtime_set_tick_rate", {
            |mut caller: Caller<Context<T>>, ticks_per_sec: f64| -> Result<()> {
                caller.data_mut().timer.log_runtime(
                    format_args!("New Tick Rate: {ticks_per_sec}"),
                    LogLevel::Debug,
                );

                ensure!(
                    ticks_per_sec > 0.0,
                    "The tick rate needs to be larger than 0."
                );
                let duration = ticks_per_sec.recip();

                const MAX_DURATION: f64 = u64::MAX as f64;
                ensure!(duration < MAX_DURATION, "The tick rate is too small.");

                caller
                    .data_mut()
                    .shared_data
                    .tick_rate
                    .store(duration.to_bits(), atomic::Ordering::Relaxed);

                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "runtime_set_tick_rate",
        })?
        .func_wrap("env", "runtime_print_message", {
            |mut caller: Caller<Context<T>>, ptr: u32, len: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let message = get_str(memory, ptr, len)?;
                context.timer.log_auto_splitter(format_args!("{message}"));
                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "runtime_print_message",
        })?
        .func_wrap("env", "runtime_get_os", {
            |mut caller: Caller<Context<T>>, ptr: u32, len_ptr: u32| {
                let (memory, _) = memory_and_context(&mut caller);

                let len_bytes = get_arr_mut(memory, len_ptr)?;
                let len = u32::from_le_bytes(*len_bytes) as usize;
                *len_bytes = (OS.len() as u32).to_le_bytes();

                if len < OS.len() {
                    return Ok(0u32);
                }
                let buf = get_slice_mut(memory, ptr, OS.len() as _)?;
                buf.copy_from_slice(OS.as_bytes());
                Ok(1u32)
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "runtime_get_os",
        })?
        .func_wrap("env", "runtime_get_arch", {
            |mut caller: Caller<Context<T>>, ptr: u32, len_ptr: u32| {
                let (memory, _) = memory_and_context(&mut caller);

                let len_bytes = get_arr_mut(memory, len_ptr)?;
                let len = u32::from_le_bytes(*len_bytes) as usize;
                *len_bytes = (ARCH.len() as u32).to_le_bytes();

                if len < ARCH.len() {
                    return Ok(0u32);
                }
                let buf = get_slice_mut(memory, ptr, ARCH.len() as _)?;
                buf.copy_from_slice(ARCH.as_bytes());
                Ok(1u32)
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "runtime_get_arch",
        })?;
    Ok(())
}
