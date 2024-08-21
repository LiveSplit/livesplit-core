use std::str;

use anyhow::{format_err, Context as _, Result};
use slotmap::{Key, KeyData};
use wasmtime::{Caller, Linker};

use crate::{
    runtime::{Context, ProcessKey},
    timer::LogLevel,
    CreationError, Process, Timer,
};

use super::{get_arr_mut, get_slice_mut, get_str, get_two_slice_mut, memory_and_context};

pub fn bind<T: Timer>(linker: &mut Linker<Context<T>>) -> Result<(), CreationError> {
    linker
        .func_wrap("env", "process_attach", {
            |mut caller: Caller<'_, Context<T>>, ptr: u32, len: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let process_name = get_str(memory, ptr, len)?;
                Ok(
                    if let Ok(p) = Process::with_name(process_name, &mut context.process_list) {
                        context.timer.log_runtime(
                            format_args!(
                                "Attached to a new process: {}",
                                p.name().unwrap_or("<Unnamed Process>")
                            ),
                            LogLevel::Debug,
                        );
                        context.processes.insert(p).data().as_ffi()
                    } else {
                        0
                    },
                )
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_attach",
        })?
        .func_wrap("env", "process_attach_by_pid", {
            |mut caller: Caller<'_, Context<T>>, pid: u64| {
                let (_, context) = memory_and_context(&mut caller);
                Ok(
                    if let Some(p) = pid
                        .try_into()
                        .ok()
                        .and_then(|pid| Process::with_pid(pid, &mut context.process_list).ok())
                    {
                        context.timer.log_runtime(
                            format_args!(
                                "Attached to a new process: {}",
                                p.name().unwrap_or("<Unnamed Process>")
                            ),
                            LogLevel::Debug,
                        );
                        context.processes.insert(p).data().as_ffi()
                    } else {
                        0
                    },
                )
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_attach_by_pid",
        })?
        .func_wrap("env", "process_detach", {
            |mut caller: Caller<'_, Context<T>>, process: u64| {
                caller
                    .data_mut()
                    .processes
                    .remove(ProcessKey::from(KeyData::from_ffi(process)))
                    .ok_or_else(|| format_err!("Invalid process handle {process}"))?;
                caller
                    .data_mut()
                    .timer
                    .log_runtime(format_args!("Detached from a process."), LogLevel::Debug);
                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_detach",
        })?
        .func_wrap("env", "process_list_by_name", {
            |mut caller: Caller<'_, Context<T>>,
             name_ptr: u32,
             name_len: u32,
             list_ptr: u32,
             list_len_ptr: u32| {
                let (memory, context) = memory_and_context(&mut caller);

                let list_len_bytes = get_arr_mut(memory, list_len_ptr)?;
                let list_len = u32::from_le_bytes(*list_len_bytes);

                let [name, list] = get_two_slice_mut(
                    memory,
                    name_ptr,
                    name_len,
                    list_ptr,
                    list_len
                        .checked_mul(8)
                        .context("The list length overflows the size of the address space.")?,
                )?;

                let mut count = 0u32;

                let mut iter =
                    Process::list_pids_by_name(str::from_utf8(name)?, &mut context.process_list)
                        .inspect(|_| {
                            count = count.saturating_add(1);
                        });

                for (pid, list_element) in iter.by_ref().zip(bytemuck::cast_slice_mut(list)) {
                    *list_element = (pid as u64).to_le_bytes();
                }
                // Consume the rest of the PIDs to ensure we fully count them.
                iter.for_each(drop);

                let list_len_bytes = get_arr_mut(memory, list_len_ptr)?;
                *list_len_bytes = count.to_le_bytes();

                // Currently this can't fail, but that's only because `sysinfo`
                // doesn't report any errors when listing the processes fails.
                Ok(1u32)
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_list_by_name",
        })?
        .func_wrap("env", "process_is_open", {
            |mut caller: Caller<'_, Context<T>>, process: u64| {
                let ctx = caller.data_mut();
                let proc = ctx
                    .processes
                    .get_mut(ProcessKey::from(KeyData::from_ffi(process)))
                    .ok_or_else(|| format_err!("Invalid process handle: {process}"))?;
                Ok(proc.is_open(&mut ctx.process_list) as u32)
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_is_open",
        })?
        .func_wrap("env", "process_read", {
            |mut caller: Caller<'_, Context<T>>,
             process: u64,
             address: u64,
             buf_ptr: u32,
             buf_len: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                Ok(context
                    .processes
                    .get(ProcessKey::from(KeyData::from_ffi(process)))
                    .ok_or_else(|| format_err!("Invalid process handle: {process}"))?
                    .read_mem(address, get_slice_mut(memory, buf_ptr, buf_len)?)
                    .is_ok() as u32)
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_read",
        })?
        .func_wrap("env", "process_get_module_address", {
            |mut caller: Caller<'_, Context<T>>, process: u64, ptr: u32, len: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let module_name = get_str(memory, ptr, len)?;
                Ok(context
                    .processes
                    .get_mut(ProcessKey::from(KeyData::from_ffi(process)))
                    .ok_or_else(|| format_err!("Invalid process handle: {process}"))?
                    .module_address(module_name)
                    .unwrap_or_default())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_get_module_address",
        })?
        .func_wrap("env", "process_get_module_size", {
            |mut caller: Caller<'_, Context<T>>, process: u64, ptr: u32, len: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let module_name = get_str(memory, ptr, len)?;
                Ok(context
                    .processes
                    .get_mut(ProcessKey::from(KeyData::from_ffi(process)))
                    .ok_or_else(|| format_err!("Invalid process handle: {process}"))?
                    .module_size(module_name)
                    .unwrap_or_default())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_get_module_size",
        })?
        .func_wrap("env", "process_get_module_path", {
            |mut caller: Caller<'_, Context<T>>,
             process: u64,
             name_ptr: u32,
             name_len: u32,
             path_ptr: u32,
             path_len_ptr: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let module_name = get_str(memory, name_ptr, name_len)?;
                let path = context
                    .processes
                    .get_mut(ProcessKey::from(KeyData::from_ffi(process)))
                    .ok_or_else(|| format_err!("Invalid process handle: {process}"))?
                    .module_path(module_name);

                let path_len_bytes = get_arr_mut(memory, path_len_ptr)?;
                if let Ok(path) = path {
                    let path_len = u32::from_le_bytes(*path_len_bytes) as usize;
                    *path_len_bytes = (path.len() as u32).to_le_bytes();
                    if path_len < path.len() {
                        return Ok(0u32);
                    }
                    let buf = get_slice_mut(memory, path_ptr, path.len() as _)?;
                    buf.copy_from_slice(path.as_bytes());
                    Ok(1u32)
                } else {
                    *path_len_bytes = 0u32.to_le_bytes();
                    Ok(0u32)
                }
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_get_module_path",
        })?
        .func_wrap("env", "process_get_path", {
            |mut caller: Caller<'_, Context<T>>, process: u64, ptr: u32, len_ptr: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let path = context
                    .processes
                    .get_mut(ProcessKey::from(KeyData::from_ffi(process)))
                    .ok_or_else(|| format_err!("Invalid process handle: {process}"))?
                    .path();

                let len_bytes = get_arr_mut(memory, len_ptr)?;
                if let Some(path) = path {
                    *len_bytes = (path.len() as u32).to_le_bytes();

                    let len = u32::from_le_bytes(*len_bytes) as usize;
                    if len < path.len() {
                        return Ok(0u32);
                    }
                    let buf = get_slice_mut(memory, ptr, path.len() as _)?;
                    buf.copy_from_slice(path.as_bytes());
                    Ok(1u32)
                } else {
                    *len_bytes = 0u32.to_le_bytes();
                    Ok(0u32)
                }
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_get_path",
        })?
        .func_wrap("env", "process_get_memory_range_count", {
            |mut caller: Caller<'_, Context<T>>, process: u64| {
                let ctx = caller.data_mut();
                Ok(ctx
                    .processes
                    .get_mut(ProcessKey::from(KeyData::from_ffi(process)))
                    .ok_or_else(|| format_err!("Invalid process handle: {process}"))?
                    .get_memory_range_count()
                    .unwrap_or_default() as u64)
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_get_memory_range_count",
        })?
        .func_wrap("env", "process_get_memory_range_address", {
            |mut caller: Caller<'_, Context<T>>, process: u64, idx: u64| {
                let ctx = caller.data_mut();
                Ok(ctx
                    .processes
                    .get_mut(ProcessKey::from(KeyData::from_ffi(process)))
                    .ok_or_else(|| format_err!("Invalid process handle: {process}"))?
                    .get_memory_range_address(idx as usize)
                    .unwrap_or_default())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_get_memory_range_address",
        })?
        .func_wrap("env", "process_get_memory_range_size", {
            |mut caller: Caller<'_, Context<T>>, process: u64, idx: u64| {
                let ctx = caller.data_mut();
                Ok(ctx
                    .processes
                    .get_mut(ProcessKey::from(KeyData::from_ffi(process)))
                    .ok_or_else(|| format_err!("Invalid process handle: {process}"))?
                    .get_memory_range_size(idx as usize)
                    .unwrap_or_default())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_get_memory_range_size",
        })?
        .func_wrap("env", "process_get_memory_range_flags", {
            |mut caller: Caller<'_, Context<T>>, process: u64, idx: u64| {
                let ctx = caller.data_mut();
                Ok(ctx
                    .processes
                    .get_mut(ProcessKey::from(KeyData::from_ffi(process)))
                    .ok_or_else(|| format_err!("Invalid process handle: {process}"))?
                    .get_memory_range_flags(idx as usize)
                    .unwrap_or_default())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_get_memory_range_flags",
        })?;
    Ok(())
}
