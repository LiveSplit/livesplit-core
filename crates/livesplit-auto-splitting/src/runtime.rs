#![allow(clippy::unnecessary_cast)]

use crate::{process::Process, settings::UserSetting, timer::Timer, SettingValue, SettingsStore};

use anyhow::{format_err, Context as _, Result};
use slotmap::{Key, KeyData, SlotMap};
use snafu::Snafu;
use std::{
    env::consts::{ARCH, OS},
    path::PathBuf,
    str,
    time::{Duration, Instant},
};
use sysinfo::{ProcessRefreshKind, RefreshKind, System, SystemExt};
use wasi_common::{dir::DirCaps, file::FileCaps};
use wasmtime::{
    Caller, Config, Engine, Extern, Linker, Memory, Module, OptLevel, Store, TypedFunc,
};
use wasmtime_wasi::{ambient_authority, WasiCtx, WasiCtxBuilder};

/// An error that is returned when the creation of a new runtime fails.
#[derive(Debug, Snafu)]
#[snafu(context(suffix(false)))]
pub enum CreationError {
    /// Failed creating the WebAssembly engine.
    EngineCreation {
        /// The underlying error.
        source: anyhow::Error,
    },
    /// Failed loading the WebAssembly module.
    ModuleLoading {
        /// The underlying error.
        source: anyhow::Error,
    },
    /// Failed linking the WebAssembly module.
    #[snafu(display("Failed linking the function `{name}` to the WebAssembly module."))]
    LinkFunction {
        /// The name of the function that failed to link.
        name: &'static str,
        /// The underlying error.
        source: anyhow::Error,
    },
    /// Failed instantiating the WebAssembly module.
    ModuleInstantiation {
        /// The underlying error.
        source: anyhow::Error,
    },
    /// The WebAssembly module has no exported function called `update`, which is
    /// a required function.
    MissingUpdateFunction {
        /// The underlying error.
        source: anyhow::Error,
    },
    /// The WebAssembly module has no exported memory called `memory`, which is
    /// a requirement.
    MissingMemory,

    /// Failed linking the WebAssembly System Interface (WASI).
    Wasi {
        /// The underlying error.
        source: anyhow::Error,
    },
    /// Failed running the WebAssembly System Interface (WASI) `_start` function.
    WasiStart {
        /// The underlying error.
        source: anyhow::Error,
    },
}

slotmap::new_key_type! {
    struct ProcessKey;
}

pub struct Context<T: Timer> {
    tick_rate: Duration,
    processes: SlotMap<ProcessKey, Process>,
    user_settings: Vec<UserSetting>,
    settings_store: SettingsStore,
    timer: T,
    memory: Option<Memory>,
    process_list: ProcessList,
    wasi: WasiCtx,
}

/// A threadsafe handle used to interrupt the execution of the script.
pub struct InterruptHandle(Engine);

impl InterruptHandle {
    /// Interrupts the execution.
    pub fn interrupt(&self) {
        self.0.increment_epoch();
    }
}

pub struct ProcessList {
    system: System,
    last_check: Instant,
}

impl ProcessList {
    fn new() -> Self {
        Self {
            system: System::new_with_specifics(
                RefreshKind::new().with_processes(ProcessRefreshKind::new()),
            ),
            last_check: Instant::now() - Duration::from_secs(1),
        }
    }

    pub fn refresh(&mut self) {
        let now = Instant::now();
        if now - self.last_check >= Duration::from_secs(1) {
            self.system
                .refresh_processes_specifics(ProcessRefreshKind::new());
            self.last_check = now;
        }
    }

    pub fn processes_by_name<'a>(
        &'a self,
        name: &'a str,
    ) -> Box<dyn Iterator<Item = &'a sysinfo::Process> + 'a> {
        self.system.processes_by_exact_name(name)
    }

    pub fn is_open(&self, pid: sysinfo::Pid) -> bool {
        self.get(pid).is_some()
    }

    pub fn get(&self, pid: sysinfo::Pid) -> Option<&sysinfo::Process> {
        self.system.process(pid)
    }
}

/// An auto splitter runtime that allows using an auto splitter provided as a
/// WebAssembly module to control a timer.
pub struct Runtime<T: Timer> {
    store: Store<Context<T>>,
    update: TypedFunc<(), ()>,
    engine: Engine,
}

impl<T: Timer> Runtime<T> {
    /// Creates a new runtime with the given path to the WebAssembly module and
    /// the timer that the module then controls.
    pub fn new(
        module: &[u8],
        timer: T,
        settings_store: SettingsStore,
    ) -> Result<Self, CreationError> {
        let engine = Engine::new(
            Config::new()
                .cranelift_opt_level(OptLevel::Speed)
                .epoch_interruption(true),
        )
        .map_err(|source| CreationError::EngineCreation { source })?;

        let module = Module::from_binary(&engine, module)
            .map_err(|source| CreationError::ModuleLoading { source })?;

        let mut store = Store::new(
            &engine,
            Context {
                processes: SlotMap::with_key(),
                user_settings: Vec::new(),
                settings_store,
                tick_rate: Duration::new(0, 1_000_000_000 / 120),
                timer,
                memory: None,
                process_list: ProcessList::new(),
                wasi: build_wasi(),
            },
        );

        store.set_epoch_deadline(1);

        let mut linker = Linker::new(&engine);
        bind_interface(&mut linker)?;

        let uses_wasi = module
            .imports()
            .any(|import| import.module() == "wasi_snapshot_preview1");

        if uses_wasi {
            wasmtime_wasi::add_to_linker(&mut linker, |ctx| &mut ctx.wasi)
                .map_err(|source| CreationError::Wasi { source })?;
        }

        let instance = linker
            .instantiate(&mut store, &module)
            .map_err(|source| CreationError::ModuleInstantiation { source })?;

        if uses_wasi {
            store.data_mut().timer.log(format_args!("This auto splitter uses WASI. The API is subject to change, because WASI is still in preview. Auto splitters using WASI may need to be recompiled in the future."));

            // These may be different in future WASI versions.
            if let Ok(func) = instance.get_typed_func(&mut store, "_initialize") {
                func.call(&mut store, ())
                    .map_err(|source| CreationError::WasiStart { source })?;
            } else if let Ok(func) = instance.get_typed_func(&mut store, "_start") {
                func.call(&mut store, ())
                    .map_err(|source| CreationError::WasiStart { source })?;
            }
        }

        let update = instance
            .get_typed_func(&mut store, "update")
            .map_err(|source| CreationError::MissingUpdateFunction { source })?;

        let Some(Extern::Memory(mem)) = instance.get_export(&mut store, "memory") else {
            return Err(CreationError::MissingMemory);
        };
        store.data_mut().memory = Some(mem);

        Ok(Self {
            engine,
            store,
            update,
        })
    }

    /// Accesses an interrupt handle that allows you to interrupt the ongoing
    /// execution of the WebAssembly module. A WebAssembly module may
    /// accidentally or maliciously loop forever, which is why this is needed.
    pub fn interrupt_handle(&self) -> InterruptHandle {
        InterruptHandle(self.engine.clone())
    }

    /// Runs the exported `update` function of the WebAssembly module a single
    /// time and returns the duration to wait until the next execution. The auto
    /// splitter can change this tick rate. It is 120Hz by default.
    pub fn update(&mut self) -> Result<Duration> {
        self.update.call(&mut self.store, ())?;
        Ok(self.store.data().tick_rate)
    }

    /// Accesses the currently stored settings.
    pub fn settings_store(&self) -> &SettingsStore {
        &self.store.data().settings_store
    }

    /// Accesses all the settings that are meant to be shown to and modified by
    /// the user.
    pub fn user_settings(&self) -> &[UserSetting] {
        &self.store.data().user_settings
    }
}

fn build_wasi() -> WasiCtx {
    let wasi = WasiCtxBuilder::new().build();

    #[cfg(windows)]
    {
        let mut drives = unsafe { winapi::um::fileapi::GetLogicalDrives() };
        loop {
            let drive_idx = drives.trailing_zeros();
            if drive_idx >= 26 {
                break;
            }
            drives &= !(1 << drive_idx);
            let drive = drive_idx as u8 + b'a';
            if let Ok(path) = wasmtime_wasi::Dir::open_ambient_dir(
                str::from_utf8(&[drive, b':', b'\\']).unwrap(),
                ambient_authority(),
            ) {
                wasi.push_dir(
                    Box::new(wasmtime_wasi::dir::Dir::from_cap_std(path)),
                    DirCaps::OPEN
                        | DirCaps::READDIR
                        | DirCaps::READLINK
                        | DirCaps::PATH_FILESTAT_GET
                        | DirCaps::FILESTAT_GET,
                    FileCaps::READ | FileCaps::SEEK | FileCaps::TELL | FileCaps::FILESTAT_GET,
                    PathBuf::from(str::from_utf8(&[b'/', b'm', b'n', b't', b'/', drive]).unwrap()),
                )
                .unwrap();
            }
        }
    }
    #[cfg(not(windows))]
    {
        if let Ok(path) = wasmtime_wasi::Dir::open_ambient_dir("/", ambient_authority()) {
            wasi.push_dir(
                Box::new(wasmtime_wasi::dir::Dir::from_cap_std(path)),
                DirCaps::OPEN
                    | DirCaps::READDIR
                    | DirCaps::READLINK
                    | DirCaps::PATH_FILESTAT_GET
                    | DirCaps::FILESTAT_GET,
                FileCaps::READ | FileCaps::SEEK | FileCaps::TELL | FileCaps::FILESTAT_GET,
                PathBuf::from("/mnt"),
            )
            .unwrap();
        }
    }
    wasi
}

fn bind_interface<T: Timer>(linker: &mut Linker<Context<T>>) -> Result<(), CreationError> {
    linker
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
            "timer_reset",
            |mut caller: Caller<'_, Context<T>>| {
                caller.data_mut().timer.reset();
            },
        )
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "timer_reset",
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
        .func_wrap("env", "timer_get_state", {
            |caller: Caller<'_, Context<T>>| caller.data().timer.state() as u32
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "timer_get_state",
        })?
        .func_wrap("env", "runtime_set_tick_rate", {
            |mut caller: Caller<'_, Context<T>>, ticks_per_sec: f64| {
                caller
                    .data_mut()
                    .timer
                    .log(format_args!("New Tick Rate: {ticks_per_sec}"));
                caller.data_mut().tick_rate = Duration::from_secs_f64(ticks_per_sec.recip());
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "runtime_set_tick_rate",
        })?
        .func_wrap("env", "runtime_print_message", {
            |mut caller: Caller<'_, Context<T>>, ptr: u32, len: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let message = get_str(memory, ptr, len)?;
                context.timer.log(format_args!("{message}"));
                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "runtime_print_message",
        })?
        .func_wrap("env", "runtime_get_os", {
            |mut caller: Caller<'_, Context<T>>, ptr: u32, len_ptr: u32| {
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
            |mut caller: Caller<'_, Context<T>>, ptr: u32, len_ptr: u32| {
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
        .func_wrap("env", "process_attach", {
            |mut caller: Caller<'_, Context<T>>, ptr: u32, len: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let process_name = get_str(memory, ptr, len)?;
                Ok(
                    if let Ok(p) = Process::with_name(process_name, &mut context.process_list) {
                        context
                            .timer
                            .log(format_args!("Attached to a new process: {process_name}"));
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
        .func_wrap("env", "process_detach", {
            |mut caller: Caller<'_, Context<T>>, process: u64| {
                caller
                    .data_mut()
                    .processes
                    .remove(ProcessKey::from(KeyData::from_ffi(process as u64)))
                    .ok_or_else(|| format_err!("Invalid process handle {process}"))?;
                caller
                    .data_mut()
                    .timer
                    .log(format_args!("Detached from a process."));
                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_detach",
        })?
        .func_wrap("env", "process_is_open", {
            |mut caller: Caller<'_, Context<T>>, process: u64| {
                let ctx = caller.data_mut();
                let proc = ctx
                    .processes
                    .get(ProcessKey::from(KeyData::from_ffi(process as u64)))
                    .ok_or_else(|| format_err!("Invalid process handle: {process}"))?;
                Ok(proc.is_open(&mut ctx.process_list) as u32)
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_is_open",
        })?
        .func_wrap("env", "process_get_module_address", {
            |mut caller: Caller<'_, Context<T>>, process: u64, ptr: u32, len: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let module_name = get_str(memory, ptr, len)?;
                Ok(context
                    .processes
                    .get_mut(ProcessKey::from(KeyData::from_ffi(process as u64)))
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
                    .get_mut(ProcessKey::from(KeyData::from_ffi(process as u64)))
                    .ok_or_else(|| format_err!("Invalid process handle: {process}"))?
                    .module_size(module_name)
                    .unwrap_or_default())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_get_module_size",
        })?
        .func_wrap("env", "process_get_path", {
            |mut caller: Caller<'_, Context<T>>, process: u64, ptr: u32, len_ptr: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let path = context
                    .processes
                    .get_mut(ProcessKey::from(KeyData::from_ffi(process as u64)))
                    .ok_or_else(|| format_err!("Invalid process handle: {process}"))?
                    .path()
                    .unwrap_or_default();

                let len_bytes = get_arr_mut(memory, len_ptr)?;
                let len = u32::from_le_bytes(*len_bytes) as usize;
                *len_bytes = (path.len() as u32).to_le_bytes();
                if len < path.len() {
                    return Ok(0u32);
                }
                let buf = get_slice_mut(memory, ptr, path.len() as _)?;
                buf.copy_from_slice(path.as_bytes());
                Ok(1u32)
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_get_path",
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
                    .get(ProcessKey::from(KeyData::from_ffi(process as u64)))
                    .ok_or_else(|| format_err!("Invalid process handle: {process}"))?
                    .read_mem(address, get_slice_mut(memory, buf_ptr, buf_len)?)
                    .is_ok() as u32)
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_read",
        })?
        .func_wrap("env", "process_get_memory_range_count", {
            |mut caller: Caller<'_, Context<T>>, process: u64| {
                let ctx = caller.data_mut();
                Ok(ctx
                    .processes
                    .get_mut(ProcessKey::from(KeyData::from_ffi(process as u64)))
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
                    .get_mut(ProcessKey::from(KeyData::from_ffi(process as u64)))
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
                    .get_mut(ProcessKey::from(KeyData::from_ffi(process as u64)))
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
                    .get_mut(ProcessKey::from(KeyData::from_ffi(process as u64)))
                    .ok_or_else(|| format_err!("Invalid process handle: {process}"))?
                    .get_memory_range_flags(idx as usize)
                    .unwrap_or_default())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "process_get_memory_range_flags",
        })?
        .func_wrap("env", "user_settings_add_bool", {
            |mut caller: Caller<'_, Context<T>>,
             key_ptr: u32,
             key_len: u32,
             description_ptr: u32,
             description_len: u32,
             default_value: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let key = Box::<str>::from(get_str(memory, key_ptr, key_len)?);
                let description = get_str(memory, description_ptr, description_len)?.into();
                let default_value = default_value != 0;
                let value_in_store = match context.settings_store.get(&key) {
                    Some(SettingValue::Bool(v)) => *v,
                    None => {
                        // TODO: Should this auto insert into the store?
                        context
                            .settings_store
                            .set(key.clone(), SettingValue::Bool(default_value));
                        default_value
                    }
                };
                context.user_settings.push(UserSetting {
                    key,
                    description,
                    default_value: SettingValue::Bool(default_value),
                });
                Ok(value_in_store as u32)
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "user_settings_add_bool",
        })?;
    Ok(())
}

fn memory_and_context<'a, T: Timer>(
    caller: &'a mut Caller<'_, Context<T>>,
) -> (&'a mut [u8], &'a mut Context<T>) {
    caller.data().memory.unwrap().data_and_store_mut(caller)
}

fn get_arr_mut<const N: usize>(memory: &mut [u8], ptr: u32) -> Result<&mut [u8; N]> {
    assert!(N <= u32::MAX as usize);
    Ok(get_slice_mut(memory, ptr, N as _)?.try_into().unwrap())
}

fn get_slice(memory: &[u8], ptr: u32, len: u32) -> Result<&[u8]> {
    memory
        .get(ptr as usize..(ptr + len) as usize)
        .context("Out of bounds pointer and length pair.")
}

fn get_slice_mut(memory: &mut [u8], ptr: u32, len: u32) -> Result<&mut [u8]> {
    memory
        .get_mut(ptr as usize..(ptr + len) as usize)
        .context("Out of bounds pointer and length pair.")
}

fn get_str(memory: &[u8], ptr: u32, len: u32) -> Result<&str> {
    let slice = get_slice(memory, ptr, len)?;
    str::from_utf8(slice).map_err(Into::into)
}
