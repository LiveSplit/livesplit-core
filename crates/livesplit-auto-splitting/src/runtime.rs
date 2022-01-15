use crate::{process::Process, timer::Timer, InterruptHandle};

use log::info;
use slotmap::{Key, KeyData, SlotMap};
use snafu::{ResultExt, Snafu};
use std::{
    path::Path,
    result, str, thread,
    time::{Duration, Instant},
};
use sysinfo::{ProcessRefreshKind, RefreshKind, System, SystemExt};
use wasmtime::{
    Caller, Config, Engine, Extern, Linker, Memory, Module, OptLevel, Store, Trap, TypedFunc,
};

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
    /// The WebAssembly module has no exported function named `update`, which is
    /// a required function.
    MissingUpdateFunction {
        /// The underlying error.
        source: anyhow::Error,
    },
    /// The WebAssembly module has no exported function memory called `memory`,
    /// which is a requirement.
    MissingMemory,
}

/// An error that is returned when executing the WebAssembly module fails.
#[derive(Debug, Snafu)]
#[snafu(context(suffix(false)))]
pub enum RunError {
    /// Failed running the `update` function.
    RunUpdate {
        /// The underlying error.
        source: Trap,
    },
}

slotmap::new_key_type! {
    struct ProcessKey;
}

pub struct Context<T: Timer> {
    tick_rate: Duration,
    processes: SlotMap<ProcessKey, Process>,
    timer: T,
    memory: Option<Memory>,
    process_list: ProcessList,
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

    pub fn process_by_name(&self, name: &str) -> Vec<&sysinfo::Process> {
        self.system.process_by_name(name)
    }

    pub fn is_open(&self, pid: sysinfo::Pid) -> bool {
        self.system.process(pid).is_some()
    }
}

/// An auto splitter runtime that allows using an auto splitter provided as a
/// WebAssembly module to control a timer.
pub struct Runtime<T: Timer> {
    store: Store<Context<T>>,
    update: TypedFunc<(), ()>,
    prev_time: Instant,
}

impl<T: Timer> Runtime<T> {
    /// Creates a new runtime with the given path to the WebAssembly module and
    /// the timer that the module then controls.
    pub fn new<P: AsRef<Path>>(path: P, timer: T) -> Result<Self, CreationError> {
        let engine = Engine::new(
            Config::new()
                .cranelift_opt_level(OptLevel::Speed)
                .interruptable(true),
        )
        .context(EngineCreation)?;

        let mut store = Store::new(
            &engine,
            Context {
                processes: SlotMap::with_key(),
                tick_rate: Duration::from_secs(1) / 120,
                timer,
                memory: None,
                process_list: ProcessList::new(),
            },
        );

        let module = Module::from_file(&engine, path).context(ModuleLoading)?;
        let mut linker = Linker::new(&engine);
        bind_interface(&mut linker)?;
        let instance = linker
            .instantiate(&mut store, &module)
            .context(ModuleInstantiation)?;

        let update = instance
            .get_typed_func(&mut store, "update")
            .context(MissingUpdateFunction)?;

        if let Some(Extern::Memory(mem)) = instance.get_export(&mut store, "memory") {
            store.data_mut().memory = Some(mem);
        } else {
            return Err(CreationError::MissingMemory);
        }

        Ok(Self {
            store,
            update,
            prev_time: Instant::now(),
        })
    }

    /// Accesses an interrupt handle that allows you to interrupt the ongoing
    /// execution of the WebAssembly module. A WebAssembly module may
    /// accidentally or maliciously loop forever, which is why this is needed.
    pub fn interrupt_handle(&self) -> InterruptHandle {
        self.store
            .interrupt_handle()
            .expect("We configured the runtime to produce an interrupt handle.")
    }

    /// Runs the exported `update` function of the WebAssembly module a single
    /// time. If the module has not been configured yet, this will also call the
    /// optional `configure` function beforehand.
    pub fn step(&mut self) -> Result<(), RunError> {
        self.update.call(&mut self.store, ()).context(RunUpdate)
    }

    /// Sleeps until the next tick based on the current tick rate. The auto
    /// splitter can change this tick rate. It is 120Hz by default.
    pub fn sleep(&mut self) {
        let target = self.store.data().tick_rate;
        let delta = self.prev_time.elapsed();
        if delta < target {
            thread::sleep(target - delta);
        }
        self.prev_time = Instant::now();
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
        .context(LinkFunction {
            name: "timer_start",
        })?
        .func_wrap(
            "env",
            "timer_split",
            |mut caller: Caller<'_, Context<T>>| {
                caller.data_mut().timer.split();
            },
        )
        .context(LinkFunction {
            name: "timer_split",
        })?
        .func_wrap(
            "env",
            "timer_reset",
            |mut caller: Caller<'_, Context<T>>| {
                caller.data_mut().timer.reset();
            },
        )
        .context(LinkFunction {
            name: "timer_reset",
        })?
        .func_wrap("env", "timer_pause_game_time", {
            |mut caller: Caller<'_, Context<T>>| caller.data_mut().timer.pause_game_time()
        })
        .context(LinkFunction {
            name: "timer_pause_game_time",
        })?
        .func_wrap("env", "timer_resume_game_time", {
            |mut caller: Caller<'_, Context<T>>| caller.data_mut().timer.resume_game_time()
        })
        .context(LinkFunction {
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
        .context(LinkFunction {
            name: "timer_set_game_time",
        })?
        .func_wrap("env", "runtime_set_tick_rate", {
            |mut caller: Caller<'_, Context<T>>, ticks_per_sec: f64| {
                info!(target: "Auto Splitter", "New Tick Rate: {ticks_per_sec}");
                caller.data_mut().tick_rate = Duration::from_secs_f64(ticks_per_sec.recip());
            }
        })
        .context(LinkFunction {
            name: "runtime_set_tick_rate",
        })?
        .func_wrap("env", "timer_get_state", {
            |caller: Caller<'_, Context<T>>| caller.data().timer.state() as u32
        })
        .context(LinkFunction {
            name: "timer_get_state",
        })?
        .func_wrap("env", "runtime_print_message", {
            |mut caller: Caller<'_, Context<T>>, ptr: u32, len: u32| {
                let message = read_str(&mut caller, ptr, len)?;
                info!(target: "Auto Splitter", "{message}");
                Ok(())
            }
        })
        .context(LinkFunction {
            name: "runtime_print_message",
        })?
        .func_wrap("env", "timer_set_variable", {
            |mut caller: Caller<'_, Context<T>>,
             name_ptr: u32,
             name_len: u32,
             value_ptr: u32,
             value_len: u32|
             -> result::Result<(), Trap> {
                let name = read_str(&mut caller, name_ptr, name_len)?;
                let value = read_str(&mut caller, value_ptr, value_len)?;
                caller.data_mut().timer.set_variable(&name, &value);
                Ok(())
            }
        })
        .context(LinkFunction {
            name: "timer_set_variable",
        })?
        .func_wrap("env", "process_attach", {
            |mut caller: Caller<'_, Context<T>>, ptr: u32, len: u32| {
                let process_name = read_str(&mut caller, ptr, len)?;
                Ok(
                    if let Ok(p) =
                        Process::with_name(&process_name, &mut caller.data_mut().process_list)
                    {
                        info!(target: "Auto Splitter", "Attached to a new process: {process_name}");
                        caller.data_mut().processes.insert(p).data().as_ffi()
                    } else {
                        0
                    },
                )
            }
        })
        .context(LinkFunction {
            name: "process_attach",
        })?
        .func_wrap("env", "process_detach", {
            |mut caller: Caller<'_, Context<T>>, process: u64| {
                caller
                    .data_mut()
                    .processes
                    .remove(ProcessKey::from(KeyData::from_ffi(process as u64)))
                    .ok_or_else(|| Trap::new(format!("Invalid process handle {process}")))?;
                info!(target: "Auto Splitter", "Detached from a process.");
                Ok(())
            }
        })
        .context(LinkFunction {
            name: "process_detach",
        })?
        .func_wrap("env", "process_is_open", {
            |mut caller: Caller<'_, Context<T>>, process: u64| {
                let ctx = caller.data_mut();
                let proc = ctx
                    .processes
                    .get(ProcessKey::from(KeyData::from_ffi(process as u64)))
                    .ok_or_else(|| Trap::new(format!("Invalid process handle: {process}")))?;
                Ok(proc.is_open(&mut ctx.process_list) as u32)
            }
        })
        .context(LinkFunction {
            name: "process_is_open",
        })?
        .func_wrap("env", "process_get_module_address", {
            |mut caller: Caller<'_, Context<T>>, process: u64, ptr: u32, len: u32| {
                let module_name = read_str(&mut caller, ptr, len)?;
                Ok(caller
                    .data_mut()
                    .processes
                    .get_mut(ProcessKey::from(KeyData::from_ffi(process as u64)))
                    .ok_or_else(|| Trap::new(format!("Invalid process handle: {process}")))?
                    .module_address(&module_name)
                    .unwrap_or_default())
            }
        })
        .context(LinkFunction {
            name: "process_get_module_address",
        })?
        .func_wrap("env", "process_read", {
            |mut caller: Caller<'_, Context<T>>,
             process: u64,
             address: u64,
             buf_ptr: u32,
             buf_len: u32| {
                let (slice, context) = caller
                    .data()
                    .memory
                    .unwrap()
                    .data_and_store_mut(&mut caller);
                Ok(context
                    .processes
                    .get(ProcessKey::from(KeyData::from_ffi(process as u64)))
                    .ok_or_else(|| Trap::new(format!("Invalid process handle: {process}")))?
                    .read_mem(
                        address,
                        slice
                            .get_mut(buf_ptr as usize..(buf_ptr + buf_len) as usize)
                            .ok_or_else(|| Trap::new("Out of bounds"))?,
                    )
                    .is_ok() as u32)
            }
        })
        .context(LinkFunction {
            name: "process_read",
        })?;
    Ok(())
}

fn read_str<T: Timer>(
    caller: &mut Caller<'_, Context<T>>,
    ptr: u32,
    len: u32,
) -> result::Result<String, Trap> {
    let data = caller
        .data()
        .memory
        .unwrap()
        .data(&caller)
        .get(ptr as usize..(ptr + len) as usize)
        .ok_or_else(|| Trap::new("Pointer out of bounds"))?;
    let s = str::from_utf8(data).map_err(|_| Trap::new("Invalid utf-8"))?;
    Ok(s.to_string())
}
