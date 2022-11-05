use crate::{process::Process, timer::Timer};

use log::info;
use slotmap::{Key, KeyData, SlotMap};
use snafu::{ResultExt, Snafu};
use std::{
    path::Path,
    result, str,
    time::{Duration, Instant},
};
use sysinfo::{ProcessRefreshKind, RefreshKind, System, SystemExt};
use wasmtime::{
    Caller, Config, Engine, Extern, Linker, Memory, Module, OptLevel, Store, Trap, TypedFunc,
};
#[cfg(feature = "unstable")]
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

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

    #[cfg(feature = "unstable")]
    /// Failed linking the WebAssembly System Interface (WASI).
    Wasi {
        /// The underlying error.
        source: anyhow::Error,
    },
    #[cfg(feature = "unstable")]
    /// Failed running the WebAssembly System Interface (WASI) `_start` function.
    WasiStart {
        /// The underlying error.
        source: wasmtime::Trap,
    },
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
    #[cfg(feature = "unstable")]
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
        self.system.processes_by_name(name)
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
    engine: Engine,
}

impl<T: Timer> Runtime<T> {
    /// Creates a new runtime with the given path to the WebAssembly module and
    /// the timer that the module then controls.
    pub fn new(path: &Path, timer: T) -> Result<Self, CreationError> {
        let engine = Engine::new(
            Config::new()
                .cranelift_opt_level(OptLevel::Speed)
                .epoch_interruption(true),
        )
        .context(EngineCreation)?;

        let module = Module::from_file(&engine, path).context(ModuleLoading)?;

        let mut store = Store::new(
            &engine,
            Context {
                processes: SlotMap::with_key(),
                tick_rate: Duration::from_secs(1) / 120,
                timer,
                memory: None,
                process_list: ProcessList::new(),
                #[cfg(feature = "unstable")]
                wasi: WasiCtxBuilder::new().build(),
            },
        );

        store.set_epoch_deadline(1);

        let mut linker = Linker::new(&engine);
        bind_interface(&mut linker)?;

        #[cfg(feature = "unstable")]
        wasmtime_wasi::add_to_linker(&mut linker, |ctx| &mut ctx.wasi).context(Wasi)?;

        let instance = linker
            .instantiate(&mut store, &module)
            .context(ModuleInstantiation)?;

        #[cfg(feature = "unstable")]
        // TODO: _start is kind of correct, but not in the long term. They are
        // intending for us to use a different function for libraries. Look into
        // reactors.
        if let Ok(func) = instance.get_typed_func(&mut store, "_start") {
            func.call(&mut store, ()).context(WasiStart)?;
        }

        let update = instance
            .get_typed_func(&mut store, "update")
            .context(MissingUpdateFunction)?;

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
    pub fn step(&mut self) -> Result<Duration, RunError> {
        self.update.call(&mut self.store, ()).context(RunUpdate)?;
        Ok(self.store.data().tick_rate)
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
                let (memory, _) = memory_and_context(&mut caller);
                let message = read_str(memory, ptr, len)?;
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
                let (memory, context) = memory_and_context(&mut caller);
                let name = read_str(memory, name_ptr, name_len)?;
                let value = read_str(memory, value_ptr, value_len)?;
                context.timer.set_variable(name, value);
                Ok(())
            }
        })
        .context(LinkFunction {
            name: "timer_set_variable",
        })?
        .func_wrap("env", "process_attach", {
            |mut caller: Caller<'_, Context<T>>, ptr: u32, len: u32| {
                let (memory, context) = memory_and_context(&mut caller);
                let process_name = read_str(memory, ptr, len)?;
                Ok(
                    if let Ok(p) = Process::with_name(process_name, &mut context.process_list) {
                        info!(target: "Auto Splitter", "Attached to a new process: {process_name}");
                        context.processes.insert(p).data().as_ffi()
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
                let (memory, context) = memory_and_context(&mut caller);
                let module_name = read_str(memory, ptr, len)?;
                Ok(context
                    .processes
                    .get_mut(ProcessKey::from(KeyData::from_ffi(process as u64)))
                    .ok_or_else(|| Trap::new(format!("Invalid process handle: {process}")))?
                    .module_address(module_name)
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
                let (memory, context) = memory_and_context(&mut caller);
                Ok(context
                    .processes
                    .get(ProcessKey::from(KeyData::from_ffi(process as u64)))
                    .ok_or_else(|| Trap::new(format!("Invalid process handle: {process}")))?
                    .read_mem(address, read_slice_mut(memory, buf_ptr, buf_len)?)
                    .is_ok() as u32)
            }
        })
        .context(LinkFunction {
            name: "process_read",
        })?;
    Ok(())
}

fn memory_and_context<'a, T: Timer>(
    caller: &'a mut Caller<'_, Context<T>>,
) -> (&'a mut [u8], &'a mut Context<T>) {
    caller.data().memory.unwrap().data_and_store_mut(caller)
}

fn read_slice(memory: &[u8], ptr: u32, len: u32) -> Result<&[u8], Trap> {
    memory
        .get(ptr as usize..(ptr + len) as usize)
        .ok_or_else(|| Trap::new("Out of bounds pointer and length pair."))
}

fn read_slice_mut(memory: &mut [u8], ptr: u32, len: u32) -> Result<&mut [u8], Trap> {
    memory
        .get_mut(ptr as usize..(ptr + len) as usize)
        .ok_or_else(|| Trap::new("Out of bounds pointer and length pair."))
}

fn read_str(memory: &[u8], ptr: u32, len: u32) -> Result<&str, Trap> {
    let slice = read_slice(memory, ptr, len)?;
    str::from_utf8(slice).map_err(|_| Trap::new("Invalid utf-8"))
}
