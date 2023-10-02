#![allow(clippy::unnecessary_cast)]

use crate::{
    process::{build_path, Process},
    settings::UserSetting,
    timer::Timer,
    SettingValue, SettingsStore, UserSettingKind,
};

use anyhow::{ensure, format_err, Context as _, Result};
use slotmap::{Key, KeyData, SlotMap};
use snafu::Snafu;
use std::{
    env::consts::{ARCH, OS},
    path::{Path, PathBuf},
    str,
    time::{Duration, Instant},
};
use sysinfo::{ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};
use wasi_common::{
    dir::{OpenResult, ReaddirCursor, ReaddirEntity},
    file::{FdFlags, Filestat, OFlags},
    ErrorExt, WasiDir,
};
use wasmtime::{
    Caller, Engine, Extern, Linker, Memory, Module, OptLevel, Store, TypedFunc,
    WasmBacktraceDetails,
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
    next_check: Instant,
}

impl ProcessList {
    fn new() -> Self {
        Self {
            system: System::new_with_specifics(
                RefreshKind::new().with_processes(ProcessRefreshKind::new()),
            ),
            next_check: Instant::now() + Duration::from_secs(1),
        }
    }

    pub fn refresh(&mut self) {
        let now = Instant::now();
        if now >= self.next_check {
            self.system
                .refresh_processes_specifics(ProcessRefreshKind::new());
            self.next_check = now + Duration::from_secs(1);
        }
    }

    pub fn refresh_single_process(&mut self, pid: sysinfo::Pid) {
        if !self
            .system
            .refresh_process_specifics(pid, ProcessRefreshKind::new())
        {
            // FIXME: Unfortunately `refresh_process_specifics` doesn't remove
            // the process if it doesn't exist anymore. There also doesn't seem
            // to be a way to manually remove it. So we have to do a full
            // refresh of all processes.
            self.system
                .refresh_processes_specifics(ProcessRefreshKind::new());
            self.next_check = Instant::now() + Duration::from_secs(1);
        }
    }

    pub fn processes_by_name<'a: 'b, 'b>(
        &'a self,
        name: &'b str,
    ) -> impl Iterator<Item = &'a sysinfo::Process> + 'b {
        self.system
            .processes()
            .values()
            .filter(move |p| p.name() == name)
    }

    pub fn is_open(&self, pid: sysinfo::Pid) -> bool {
        self.get(pid).is_some()
    }

    pub fn get(&self, pid: sysinfo::Pid) -> Option<&sysinfo::Process> {
        self.system.process(pid)
    }
}

/// The configuration to use when creating a new [`Runtime`].
#[non_exhaustive]
pub struct Config<'a> {
    /// The settings store that is used to store the settings of the auto
    /// splitter. This contains all the settings that are currently modified by
    /// the user. It may not contain all the settings that are registered as
    /// user settings, because the user may not have modified them yet.
    pub settings_store: Option<SettingsStore>,
    /// The auto splitter itself may be a runtime that wants to load a script
    /// from a file to interpret. This is the path to that script. It is
    /// provided to the auto splitter as the `SCRIPT_PATH` environment variable.
    /// **This is currently experimental and may change in the future.**
    pub interpreter_script_path: Option<&'a Path>,
    /// This enables debug information for the WebAssembly module. This is
    /// useful for debugging purposes, but due to bugs in wasmtime might
    /// currently crash the runtime. This is disabled by default. Relevant
    /// issue: https://github.com/bytecodealliance/wasmtime/issues/3999
    pub debug_info: bool,
    /// This enables backtrace details for the WebAssembly module. If a trap
    /// occurs more details are printed in the backtrace. This is enabled by
    /// default.
    pub backtrace_details: bool,
}

impl Default for Config<'_> {
    fn default() -> Self {
        Self {
            settings_store: None,
            interpreter_script_path: None,
            debug_info: false,
            backtrace_details: true,
        }
    }
}

/// An auto splitter runtime that allows using an auto splitter provided as a
/// WebAssembly module to control a timer.
pub struct Runtime<T: Timer> {
    store: Store<Context<T>>,
    update: TypedFunc<(), ()>,
    engine: Engine,
    trapped: bool,
}

impl<T: Timer> Runtime<T> {
    /// Creates a new runtime with the given path to the WebAssembly module and
    /// the timer that the module then controls.
    pub fn new(module: &[u8], timer: T, config: Config<'_>) -> Result<Self, CreationError> {
        let engine = Engine::new(
            wasmtime::Config::new()
                .cranelift_opt_level(OptLevel::Speed)
                .debug_info(config.debug_info)
                .wasm_backtrace_details(if config.backtrace_details {
                    WasmBacktraceDetails::Enable
                } else {
                    WasmBacktraceDetails::Disable
                })
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
                settings_store: config.settings_store.unwrap_or_default(),
                tick_rate: Duration::new(0, 1_000_000_000 / 120),
                timer,
                memory: None,
                process_list: ProcessList::new(),
                wasi: build_wasi(config.interpreter_script_path),
            },
        );

        store.set_epoch_deadline(1);

        let mut linker = Linker::new(&engine);
        bind_interface(&mut linker)?;

        let uses_wasi = module
            .imports()
            .any(|import| import.module() == "wasi_snapshot_preview1");

        if uses_wasi {
            wasmtime_wasi::snapshots::preview_1::add_wasi_snapshot_preview1_to_linker(
                &mut linker,
                |ctx| &mut ctx.wasi,
            )
            .map_err(|source| CreationError::Wasi { source })?;
        }

        let instance = linker
            .instantiate(&mut store, &module)
            .map_err(|source| CreationError::ModuleInstantiation { source })?;

        let Some(Extern::Memory(mem)) = instance.get_export(&mut store, "memory") else {
            return Err(CreationError::MissingMemory);
        };
        store.data_mut().memory = Some(mem);

        if uses_wasi
            || module.get_export("_initialize").is_some()
            || module.get_export("_start").is_some()
        {
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

        Ok(Self {
            engine,
            store,
            update,
            trapped: false,
        })
    }

    /// Accesses an interrupt handle that allows you to interrupt the ongoing
    /// execution of the WebAssembly module. A WebAssembly module may
    /// accidentally or maliciously loop forever, which is why this is needed.
    pub fn interrupt_handle(&self) -> InterruptHandle {
        InterruptHandle(self.engine.clone())
    }

    /// Runs the exported `update` function of the WebAssembly module a single
    /// time.
    pub fn update(&mut self) -> Result<()> {
        if self.trapped {
            return Ok(());
        }
        match self.update.call(&mut self.store, ()) {
            Ok(()) => Ok(()),
            Err(e) => {
                self.trapped = true;
                Err(e)
            }
        }
    }

    /// Returns the duration to wait until the next execution. The auto splitter
    /// can change this tick rate on every update. You should therefore call
    /// this function after every update to sleep for the correct amount of
    /// time. It is 120Hz by default.
    pub fn tick_rate(&self) -> Duration {
        self.store.data().tick_rate
    }

    /// Accesses the currently stored settings.
    pub fn settings_store(&self) -> &SettingsStore {
        &self.store.data().settings_store
    }

    /// Accesses the currently stored settings as mutable.
    pub fn settings_store_mut(&mut self) -> &mut SettingsStore {
        &mut self.store.data_mut().settings_store
    }

    /// Accesses all the settings that are meant to be shown to and modified by
    /// the user.
    pub fn user_settings(&self) -> &[UserSetting] {
        &self.store.data().user_settings
    }

    /// Accesses the memory of the WebAssembly module. This may be useful for
    /// debugging purposes.
    pub fn memory(&self) -> &[u8] {
        self.store.data().memory.as_ref().unwrap().data(&self.store)
    }

    /// Iterates over all the processes that the auto splitter is currently
    /// attached to. This may be useful for debugging purposes.
    pub fn attached_processes(&self) -> impl Iterator<Item = &Process> {
        self.store.data().processes.values()
    }
}

fn build_wasi(script_path: Option<&Path>) -> WasiCtx {
    let mut wasi = WasiCtxBuilder::new().build();

    if let Some(script_path) = script_path {
        if let Some(path) = build_path(script_path) {
            let _ = wasi.push_env("SCRIPT_PATH", &path);
        }
    }

    #[cfg(windows)]
    {
        let mut drives = unsafe { windows_sys::Win32::Storage::FileSystem::GetLogicalDrives() };
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
                    Box::new(ReadOnlyDir(wasmtime_wasi::dir::Dir::from_cap_std(path))),
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
                Box::new(ReadOnlyDir(wasmtime_wasi::dir::Dir::from_cap_std(path))),
                PathBuf::from("/mnt"),
            )
            .unwrap();
        }
    }
    wasi
}

struct ReadOnlyDir(wasmtime_wasi::dir::Dir);

#[async_trait::async_trait]
impl WasiDir for ReadOnlyDir {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    async fn open_file(
        &self,
        symlink_follow: bool,
        path: &str,
        oflags: OFlags,
        read: bool,
        write: bool,
        fdflags: FdFlags,
    ) -> Result<OpenResult, wasi_common::Error> {
        // We whitelist the OFlags and FdFlags to not accidentally allow
        // ways to modify the file system.
        const WHITELISTED_O_FLAGS: OFlags = OFlags::DIRECTORY;
        const WHITELISTED_FD_FLAGS: FdFlags = FdFlags::NONBLOCK;

        if write || !WHITELISTED_O_FLAGS.contains(oflags) || !WHITELISTED_FD_FLAGS.contains(fdflags)
        {
            return Err(wasi_common::Error::not_supported());
        }

        Ok(
            match self
                .0
                .open_file_(symlink_follow, path, oflags, read, write, fdflags)?
            {
                wasmtime_wasi::dir::OpenResult::Dir(d) => OpenResult::Dir(Box::new(ReadOnlyDir(d))),
                // We assume that wrapping the file type itself is not
                // necessary, because we ensure that the open flags don't allow
                // for any modifications anyway.
                wasmtime_wasi::dir::OpenResult::File(f) => OpenResult::File(Box::new(f)),
            },
        )
    }

    async fn readdir(
        &self,
        cursor: ReaddirCursor,
    ) -> Result<
        Box<dyn Iterator<Item = Result<ReaddirEntity, wasi_common::Error>> + Send>,
        wasi_common::Error,
    > {
        self.0.readdir(cursor).await
    }

    async fn read_link(&self, path: &str) -> Result<PathBuf, wasi_common::Error> {
        self.0.read_link(path).await
    }

    async fn get_filestat(&self) -> Result<Filestat, wasi_common::Error> {
        // FIXME: Make sure this says it's readonly, if it ever contains the
        // permissions.
        self.0.get_filestat().await
    }

    async fn get_path_filestat(
        &self,
        path: &str,
        follow_symlinks: bool,
    ) -> Result<Filestat, wasi_common::Error> {
        // FIXME: Make sure this says it's readonly, if it ever contains the
        // permissions.
        self.0.get_path_filestat(path, follow_symlinks).await
    }
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
            |mut caller: Caller<'_, Context<T>>, ticks_per_sec: f64| -> Result<()> {
                caller
                    .data_mut()
                    .timer
                    .log(format_args!("New Tick Rate: {ticks_per_sec}"));

                ensure!(
                    ticks_per_sec > 0.0,
                    "The tick rate needs to be larger than 0."
                );
                let duration = ticks_per_sec.recip();

                const MAX_DURATION: f64 = u64::MAX as f64;
                ensure!(duration < MAX_DURATION, "The tick rate is too small.");

                caller.data_mut().tick_rate = Duration::from_secs_f64(duration);

                Ok(())
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
                        context.timer.log(format_args!(
                            "Attached to a new process: {}",
                            p.name().unwrap_or("<Unnamed Process>")
                        ));
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
                        context.timer.log(format_args!(
                            "Attached to a new process: {}",
                            p.name().unwrap_or("<Unnamed Process>")
                        ));
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
                    .log(format_args!("Detached from a process."));
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
        .func_wrap("env", "process_get_path", {
            |mut caller: Caller<'_, Context<T>>, process: u64, ptr: u32, len_ptr: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let path = context
                    .processes
                    .get_mut(ProcessKey::from(KeyData::from_ffi(process)))
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
                    None => default_value,
                };
                context.user_settings.push(UserSetting {
                    key,
                    description,
                    tooltip: None,
                    kind: UserSettingKind::Bool { default_value },
                });
                Ok(value_in_store as u32)
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "user_settings_add_bool",
        })?
        .func_wrap("env", "user_settings_add_title", {
            |mut caller: Caller<'_, Context<T>>,
             key_ptr: u32,
             key_len: u32,
             description_ptr: u32,
             description_len: u32,
             heading_level: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let key = get_str(memory, key_ptr, key_len)?.into();
                let description = get_str(memory, description_ptr, description_len)?.into();
                context.user_settings.push(UserSetting {
                    key,
                    description,
                    tooltip: None,
                    kind: UserSettingKind::Title { heading_level },
                });
                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "user_settings_add_title",
        })?
        .func_wrap("env", "user_settings_set_tooltip", {
            |mut caller: Caller<'_, Context<T>>,
             key_ptr: u32,
             key_len: u32,
             tooltip_ptr: u32,
             tooltip_len: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let key = get_str(memory, key_ptr, key_len)?.into();
                let tooltip = get_str(memory, tooltip_ptr, tooltip_len)?.into();
                context
                    .user_settings
                    .iter_mut()
                    .find(|s| s.key == key)
                    .context("There is no setting with the provided key.")?
                    .tooltip = Some(tooltip);
                Ok(())
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "user_settings_set_tooltip",
        })?
        .func_wrap("env", "get_auto_splitter_settings", {
            |mut caller: Caller<'_, Context<T>>, ptr: u32, len_ptr: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let auto_splitter_settings = &context.settings_store.get_auto_splitter_settings();
                let len_bytes = get_arr_mut(memory, len_ptr)?;
                let len = u32::from_le_bytes(*len_bytes) as usize;
                *len_bytes = (auto_splitter_settings.len() as u32).to_le_bytes();
                if len < auto_splitter_settings.len() {
                    return Ok(0u32);
                }
                let buf = get_slice_mut(memory, ptr, auto_splitter_settings.len() as _)?;
                buf.copy_from_slice(auto_splitter_settings.as_bytes());
                Ok(1u32)
            }
        })
        .map_err(|source| CreationError::LinkFunction {
            source,
            name: "get_auto_splitter_settings",
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
        .get(ptr as usize..)
        .context("Out of bounds pointer and length pair.")?
        .get(..len as usize)
        .context("Out of bounds pointer and length pair.")
}

fn get_slice_mut(memory: &mut [u8], ptr: u32, len: u32) -> Result<&mut [u8]> {
    memory
        .get_mut(ptr as usize..)
        .context("Out of bounds pointer and length pair.")?
        .get_mut(..len as usize)
        .context("Out of bounds pointer and length pair.")
}

fn get_str(memory: &[u8], ptr: u32, len: u32) -> Result<&str> {
    let slice = get_slice(memory, ptr, len)?;
    str::from_utf8(slice).map_err(Into::into)
}

fn get_two_slice_mut(
    memory: &mut [u8],
    ptr1: u32,
    len1: u32,
    ptr2: u32,
    len2: u32,
) -> Result<[&mut [u8]; 2]> {
    let (ptr1, ptr2) = (ptr1 as usize, ptr2 as usize);
    let (len1, len2) = (len1 as usize, len2 as usize);
    if ptr1 < ptr2 {
        if ptr2 >= memory.len() {
            return Err(format_err!("Out of bounds pointer and length pair."));
        }
        let (first, second) = memory.split_at_mut(ptr2);
        Ok([
            first
                .get_mut(ptr1..)
                .context("Out of bounds pointer and length pair.")?
                .get_mut(..len1)
                .context("Overlapping pair of pointer ranges.")?,
            second
                .get_mut(..len2)
                .context("Out of bounds pointer and length pair.")?,
        ])
    } else {
        if ptr1 >= memory.len() {
            return Err(format_err!("Out of bounds pointer and length pair."));
        }
        let (first, second) = memory.split_at_mut(ptr1);
        Ok([
            second
                .get_mut(..len1)
                .context("Out of bounds pointer and length pair.")?,
            first
                .get_mut(ptr2..)
                .context("Out of bounds pointer and length pair.")?
                .get_mut(..len2)
                .context("Overlapping pair of pointer ranges.")?,
        ])
    }
}
