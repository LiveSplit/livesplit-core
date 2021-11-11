use crate::process::Process;
use crate::{timer::Timer, InterruptHandle};

use log::{info, warn};
use slotmap::{Key, KeyData, SlotMap};
use std::{
    path::Path,
    result, str, thread,
    time::{Duration, Instant},
};
use sysinfo::{RefreshKind, System, SystemExt};
use wasmtime::{
    Caller, Config, Engine, Extern, Instance, Linker, Memory, Module, Store, Trap,
    TypedFunc,
};

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// The autosplitter didn't export the required functions
    InvalidInterface,
    /// The autosplitter didn't expose any memory
    NoMemory,
    /// An error occured with the wasm autosplitter
    WasmTimeTrap { trap: Trap },
    /// An error occured in the wasmtime engine
    WasmTimeEngine { err: anyhow::Error },
}

impl From<Trap> for Error {
    fn from(trap: Trap) -> Self {
        Self::WasmTimeTrap { trap }
    }
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Self::WasmTimeEngine { err }
    }
}

pub type Result<T> = result::Result<T, Error>;

slotmap::new_key_type! {
    struct ProcessKey;
}

pub struct Context<T: Timer> {
    tick_rate: Duration,
    processes: SlotMap<ProcessKey, Process>,
    timer: T,
    memory: Option<Memory>,
    info: System,
}

pub struct Runtime<T: Timer> {
    instance: Instance,
    store: Store<Context<T>>,
    is_configured: bool,
    update: TypedFunc<(), ()>,
    prev_time: Instant,
}

impl<T: Timer> Runtime<T> {
    pub fn new<P: AsRef<Path>>(path: P, timer: T) -> Result<Self> {
        let engine = Engine::new(Config::new().interruptable(true))?;
        let mut store = Store::new(
            &engine,
            Context {
                processes: SlotMap::with_key(),
                tick_rate: Duration::from_secs(1) / 60,
                timer,
                memory: None,
                info: System::new_with_specifics(RefreshKind::new().with_processes()),
            },
        );
        let module = Module::from_file(&engine, path)?;
        let mut linker = Linker::new(&engine);
        bind_interface(&mut linker)?;
        let instance = linker.instantiate(&mut store, &module)?;

        let update = instance
            .get_typed_func(&mut store, "update")
            .map_err(|_| Error::InvalidInterface)?;
        if let Some(Extern::Memory(mem)) = instance.get_export(&mut store, "memory") {
            store.data_mut().memory = Some(mem);
        } else {
            return Err(Error::NoMemory);
        }

        Ok(Self {
            instance,
            store,
            is_configured: false,
            update,
            prev_time: Instant::now(),
        })
    }

    pub fn interrupt_handle(&self) -> InterruptHandle {
        self.store
            .interrupt_handle()
            .expect("We configured the runtime to produce an interrupt handle")
    }

    pub fn step(&mut self) -> Result<()> {
        if !self.is_configured {
            if let Ok(func) = self.instance.get_typed_func(&mut self.store, "configure") {
                func.call(&mut self.store, ())?;
            } else {
                return Err(Error::InvalidInterface);
            }
            self.is_configured = true;
        }
        Ok(self.update.call(&mut self.store, ())?)
    }

    pub fn sleep(&mut self) {
        let target = self.store.data().tick_rate;
        let delta = self.prev_time.elapsed();
        if delta < target {
            thread::sleep(target - delta);
        }
        self.prev_time = Instant::now();
    }
}

fn bind_interface<T: Timer>(linker: &mut Linker<Context<T>>) -> Result<()> {
    linker
        .func_wrap("env", "start", |mut caller: Caller<'_, Context<T>>| {
            caller.data_mut().timer.start();
        })?
        .func_wrap("env", "split", |mut caller: Caller<'_, Context<T>>| {
            caller.data_mut().timer.split();
        })?
        .func_wrap("env", "reset", |mut caller: Caller<'_, Context<T>>| {
            caller.data_mut().timer.reset();
        })?
        .func_wrap("env", "pause_game_time", {
            |mut caller: Caller<'_, Context<T>>| caller.data_mut().timer.pause_game_time()
        })?
        .func_wrap("env", "resume_game_time", {
            |mut caller: Caller<'_, Context<T>>| {
                caller.data_mut().timer.resume_game_time()
            }
        })?
        .func_wrap("env", "set_game_time", {
            |mut caller: Caller<'_, Context<T>>, secs: f64| {
                caller
                    .data_mut()
                    .timer
                    .set_game_time(time::Duration::seconds_f64(secs));
            }
        })?
        .func_wrap("env", "set_tick_rate", {
            |mut caller: Caller<'_, Context<T>>, ticks_per_sec: f64| {
                info!("New Tick Rate: {}", ticks_per_sec);
                caller.data_mut().tick_rate =
                    Duration::from_secs_f64(ticks_per_sec.recip());
            }
        })?
        .func_wrap("env", "get_timer_state", {
            |caller: Caller<'_, Context<T>>| caller.data().timer.state() as i32
        })?
        .func_wrap("env", "print_message", {
            |mut caller: Caller<'_, Context<T>>, ptr: u32, len: u32| {
                let message = read_str(&mut caller, ptr, len)?;
                info!(target: "Auto Splitter", "{}", message);
                Ok(())
            }
        })?
        .func_wrap("env", "set_variable", {
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
        })?
        .func_wrap("env", "attach", {
            |mut caller: Caller<'_, Context<T>>, ptr: u32, len: u32| {
                let process_name = read_str(&mut caller, ptr, len)?;
                Ok(
                    if let Ok(p) =
                        Process::with_name(&process_name, &mut caller.data_mut().info)
                    {
                        info!("Attached to a new process: {}", process_name);
                        caller.data_mut().processes.insert(p).data().as_ffi()
                    } else {
                        warn!("Couldn't find process: {}", &process_name);
                        0
                    },
                )
            }
        })?
        .func_wrap("env", "detach", {
            |mut caller: Caller<'_, Context<T>>, process: u64| {
                let key = ProcessKey::from(KeyData::from_ffi(process as u64));
                caller.data_mut().processes.remove(key).ok_or_else(|| {
                    Trap::new(format!("Invalid process handle {}", process))
                })?;
                Ok(())
            }
        })?
        .func_wrap("env", "get_module", {
            |mut caller: Caller<'_, Context<T>>, process: u64, ptr: u32, len: u32| {
                let module_name = read_str(&mut caller, ptr, len)?;
                Ok(caller
                    .data()
                    .processes
                    .get(ProcessKey::from(KeyData::from_ffi(process as u64)))
                    .ok_or_else(|| {
                        Trap::new(format!("Invalid process handle: {}", process))
                    })?
                    .module_address(&module_name)
                    .unwrap_or_default())
            }
        })?
        .func_wrap("env", "read_mem", {
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
                    .ok_or_else(|| {
                        Trap::new(format!("Invalid process handle: {}", process))
                    })?
                    .read_mem(
                        address,
                        slice
                            .get_mut(buf_ptr as usize..(buf_ptr + buf_len) as usize)
                            .ok_or_else(|| Trap::new("Out of bounds"))?,
                    )
                    .is_ok() as u32)
            }
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
