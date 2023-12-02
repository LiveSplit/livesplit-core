#![allow(clippy::unnecessary_cast)]

use crate::{process::Process, settings, timer::Timer};

use anyhow::Result;
use slotmap::SlotMap;
use snafu::Snafu;
use std::{
    path::Path,
    sync::{Arc, Mutex, MutexGuard},
    time::{Duration, Instant},
};
use sysinfo::{ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};
use wasmtime::{
    Engine, Extern, Linker, Memory, Module, OptLevel, Store, TypedFunc, WasmBacktraceDetails,
};
use wasmtime_wasi::WasiCtx;

mod api;

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
    struct SettingsMapKey;
    struct SettingsListKey;
    struct SettingValueKey;
}

pub struct Context<T: Timer> {
    processes: SlotMap<ProcessKey, Process>,
    settings_maps: SlotMap<SettingsMapKey, settings::Map>,
    settings_lists: SlotMap<SettingsListKey, settings::List>,
    setting_values: SlotMap<SettingValueKey, settings::Value>,
    settings_widgets: Arc<Vec<settings::Widget>>,
    shared_data: Arc<SharedData>,
    timer: T,
    memory: Option<Memory>,
    process_list: ProcessList,
    wasi: WasiCtx,
}

/// A thread-safe handle used to interrupt the execution of the script.
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

    pub fn processes_by_name<'process: 'both, 'both>(
        &'process self,
        name: &'both str,
    ) -> impl Iterator<Item = &'process sysinfo::Process> + 'both {
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
    /// The settings map that is used to store the settings of the auto
    /// splitter. This contains all the settings that are currently modified by
    /// the user. It may not contain all the settings that are registered as
    /// settings widgets, because the user may not have modified them yet.
    pub settings_map: Option<settings::Map>,
    /// The auto splitter itself may be a runtime that wants to load a script
    /// from a file to interpret. This is the path to that script. It is
    /// provided to the auto splitter as the `SCRIPT_PATH` environment variable.
    /// **This is currently experimental and may change in the future.**
    pub interpreter_script_path: Option<&'a Path>,
    /// This enables debug information for the WebAssembly module. This is
    /// useful for debugging purposes. This is disabled by default.
    pub debug_info: bool,
    /// This enables optimizations for the WebAssembly module. This is enabled
    /// by default. You may want to disable this when debugging the auto
    /// splitter.
    pub optimize: bool,
    /// This enables backtrace details for the WebAssembly module. If a trap
    /// occurs more details are printed in the backtrace. This is enabled by
    /// default.
    pub backtrace_details: bool,
}

impl Default for Config<'_> {
    fn default() -> Self {
        Self {
            settings_map: None,
            interpreter_script_path: None,
            debug_info: false,
            optimize: true,
            backtrace_details: true,
        }
    }
}

struct SharedData {
    settings_map: Mutex<settings::Map>,
    tick_rate: Mutex<Duration>,
}

struct ExclusiveData<T: Timer> {
    trapped: bool,
    store: Store<Context<T>>,
    update: TypedFunc<(), ()>,
}

/// An auto splitter runtime that allows using an auto splitter provided as a
/// WebAssembly module to control a timer. You generally want to run the auto
/// splitter on a separate background thread as the auto splitter may block
/// indefinitely. The thread intending to run the auto splitter however needs to
/// [`lock`](Self::lock) the runtime. This can only be done by one thread at a
/// time. All other functions that are directly available on the runtime are
/// generally thread-safe and don't block. This allows other threads to access
/// and modify information such as settings without needing to worry that those
/// threads get blocked.
pub struct Runtime<T: Timer> {
    exclusive_data: Mutex<ExclusiveData<T>>,
    engine: Engine,
    settings_widgets: Mutex<Arc<Vec<settings::Widget>>>,
    shared_data: Arc<SharedData>,
}

/// This guard allows you to run the `update` function of the WebAssembly module
/// and access other information about the runtime that requires the auto
/// splitter to not run at the same time. It can only be accessed by one thread
/// at a time.
pub struct RuntimeGuard<'runtime, T: Timer> {
    settings_widgets: &'runtime Mutex<Arc<Vec<settings::Widget>>>,
    data: MutexGuard<'runtime, ExclusiveData<T>>,
}

impl<T: Timer> RuntimeGuard<'_, T> {
    /// Runs the exported `update` function of the WebAssembly module a single
    /// time.
    pub fn update(&mut self) -> Result<()> {
        let data = &mut *self.data;
        if data.trapped {
            return Ok(());
        }
        match data.update.call(&mut data.store, ()) {
            Ok(()) => {
                *self.settings_widgets.lock().unwrap() = data.store.data().settings_widgets.clone();
                Ok(())
            }
            Err(e) => {
                data.trapped = true;
                Err(e)
            }
        }
    }

    /// Accesses the memory of the WebAssembly module. This may be useful for
    /// debugging purposes.
    pub fn memory(&self) -> &[u8] {
        self.data
            .store
            .data()
            .memory
            .as_ref()
            .unwrap()
            .data(&self.data.store)
    }

    /// Iterates over all the processes that the auto splitter is currently
    /// attached to. This may be useful for debugging purposes.
    pub fn attached_processes(&self) -> impl Iterator<Item = &Process> {
        self.data.store.data().processes.values()
    }

    /// Returns the total amount of handles that are currently in use. This may
    /// be useful for debugging purposes to detect leaked handles.
    pub fn handles(&self) -> u64 {
        let data = self.data.store.data();
        data.processes.len() as u64
            + data.settings_maps.len() as u64
            + data.settings_lists.len() as u64
            + data.setting_values.len() as u64
    }
}

impl SharedData {
    fn set_settings_map(&self, settings_map: settings::Map) {
        *self.settings_map.lock().unwrap() = settings_map;
    }

    fn set_settings_map_if_unchanged(&self, old: &settings::Map, new: settings::Map) -> bool {
        let mut guard = self.settings_map.lock().unwrap();
        let success = guard.is_unchanged(old);
        if success {
            *guard = new;
        }
        success
    }
}

impl<T: Timer> Runtime<T> {
    /// Creates a new runtime with the given path to the WebAssembly module and
    /// the timer that the module then controls.
    pub fn new(module: &[u8], timer: T, config: Config<'_>) -> Result<Self, CreationError> {
        let mut engine_config = wasmtime::Config::new();

        engine_config
            .cranelift_opt_level(if config.optimize {
                OptLevel::Speed
            } else {
                OptLevel::None
            })
            .debug_info(config.debug_info)
            .wasm_backtrace_details(if config.backtrace_details {
                WasmBacktraceDetails::Enable
            } else {
                WasmBacktraceDetails::Disable
            })
            .epoch_interruption(true);

        let engine = Engine::new(&engine_config)
            .map_err(|source| CreationError::EngineCreation { source })?;

        let module = Module::from_binary(&engine, module)
            .map_err(|source| CreationError::ModuleLoading { source })?;

        let settings_widgets = Arc::new(Vec::new());

        let shared_data = Arc::new(SharedData {
            settings_map: Mutex::new(config.settings_map.unwrap_or_default()),
            tick_rate: Mutex::new(Duration::new(0, 1_000_000_000 / 120)),
        });

        let mut store = Store::new(
            &engine,
            Context {
                processes: SlotMap::with_key(),
                settings_maps: SlotMap::with_key(),
                settings_lists: SlotMap::with_key(),
                setting_values: SlotMap::with_key(),
                settings_widgets: settings_widgets.clone(),
                shared_data: shared_data.clone(),
                timer,
                memory: None,
                process_list: ProcessList::new(),
                wasi: api::wasi::build(config.interpreter_script_path),
            },
        );

        store.set_epoch_deadline(1);

        let mut linker = Linker::new(&engine);
        api::bind(&mut linker)?;

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
            exclusive_data: Mutex::new(ExclusiveData {
                trapped: false,
                store,
                update,
            }),
            engine,
            settings_widgets: Mutex::new(settings_widgets),
            shared_data,
        })
    }

    /// Accesses an interrupt handle that allows you to interrupt the ongoing
    /// execution of the WebAssembly module. A WebAssembly module may
    /// accidentally or maliciously loop forever, which is why this is needed.
    pub fn interrupt_handle(&self) -> InterruptHandle {
        InterruptHandle(self.engine.clone())
    }

    /// Calling this function allows you to run the `update` function of the
    /// WebAssembly module and access other information about the runtime that
    /// requires the auto splitter to not run at the same time. This blocks the
    /// thread when another thread still has access to a [`RuntimeGuard`]. All
    /// other functions that are directly available on the runtime are generally
    /// thread-safe and don't block. This allows other threads to access and
    /// modify information such as settings without needing to worry that those
    /// threads get blocked.
    pub fn lock(&self) -> RuntimeGuard<'_, T> {
        RuntimeGuard {
            settings_widgets: &self.settings_widgets,
            data: self.exclusive_data.lock().unwrap(),
        }
    }

    /// Returns the duration to wait until the next execution. The auto splitter
    /// can change this tick rate on every update. You should therefore call
    /// this function after every update to sleep for the correct amount of
    /// time. It is 120Hz by default.
    pub fn tick_rate(&self) -> Duration {
        *self.shared_data.tick_rate.lock().unwrap()
    }

    /// Accesses a copy of the currently stored settings. The auto splitter can
    /// change these at any time. If you intend to make modifications to the
    /// settings, you need to set them again via
    /// [`set_settings_map`](Self::set_settings_map) or
    /// [`set_settings_map_if_unchanged`](Self::set_settings_map_if_unchanged).
    pub fn settings_map(&self) -> settings::Map {
        self.shared_data.settings_map.lock().unwrap().clone()
    }

    /// Unconditionally sets the settings map.
    pub fn set_settings_map(&self, settings_map: settings::Map) {
        self.shared_data.set_settings_map(settings_map)
    }

    /// Sets the settings map if it didn't change in the meantime. Returns
    /// [`true`] if it got set and [`false`] if it didn't. The auto splitter may
    /// by itself change the settings map within each update. So changing the
    /// settings from outside may race the auto splitter. You may use this to
    /// reapply the changes if the auto splitter changed the settings in the
    /// meantime.
    pub fn set_settings_map_if_unchanged(&self, old: &settings::Map, new: settings::Map) -> bool {
        self.shared_data.set_settings_map_if_unchanged(old, new)
    }

    /// Accesses all the settings widgets that are meant to be shown to and
    /// modified by the user. The auto splitter may change these settings
    /// widgets within each update. You should change the settings widgets that
    /// are shown whenever this changes. This list can't tear. Any changes from
    /// within the auto splitter can only be perceived once the auto splitter
    /// tick is complete. Any changes the user does to these widgets should be
    /// applied to the settings map and stored back.
    pub fn settings_widgets(&self) -> Arc<Vec<settings::Widget>> {
        self.settings_widgets.lock().unwrap().clone()
    }
}
