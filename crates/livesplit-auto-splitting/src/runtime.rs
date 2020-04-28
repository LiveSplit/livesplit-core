use crate::{
    environment::Environment, pointer::PointerValue, process::Process, std_stream::StdStream,
};
use std::{cell::RefCell, mem, rc::Rc, thread, time::Duration};
use wasmtime::{Export, Linker, Module, Trap};
use wasmtime_wasi::{Wasi, WasiCtxBuilder};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TimerState {
    NotRunning = 0,
    Running = 1,
    Finished = 2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TimerAction {
    Start,
    Split,
    Reset,
}

// TODO: Check if there's any memory leaks due to reference cycles. The
// exports keep the instance alive which keeps the imports alive, which all
// keep the environment alive, which keeps the memory alive, which may keep the
// instance alive -> reference cycle.
pub struct Runtime {
    env: Rc<RefCell<Environment>>,
    timer_state: TimerState,
    hooked: Option<Box<dyn Fn() -> Result<(), Trap>>>,
    unhooked: Option<Box<dyn Fn() -> Result<(), Trap>>>,
    should_start: Option<Box<dyn Fn() -> Result<i32, Trap>>>,
    should_split: Option<Box<dyn Fn() -> Result<i32, Trap>>>,
    should_reset: Option<Box<dyn Fn() -> Result<i32, Trap>>>,
    is_loading: Option<Box<dyn Fn() -> Result<i32, Trap>>>,
    game_time: Option<Box<dyn Fn() -> Result<f64, Trap>>>,
    update: Option<Box<dyn Fn() -> Result<(), Trap>>>,
    is_loading_val: Option<bool>,
    game_time_val: Option<f64>,
}

impl Runtime {
    pub fn new(binary: &[u8]) -> anyhow::Result<Self> {
        let module = Module::from_binary(&Default::default(), binary)?;
        let env = Rc::new(RefCell::new(Environment::default()));
        let mut linker = Linker::new(module.store());

        linker.func("env", "set_process_name", {
            let env = env.clone();
            move |ptr, len| env.borrow_mut().set_process_name(ptr, len)
        })?;

        linker.func("env", "push_pointer_path", {
            let env = env.clone();
            move |ptr, len, pointer_type| env.borrow_mut().push_pointer_path(ptr, len, pointer_type)
        })?;

        linker.func("env", "push_offset", {
            let env = env.clone();
            move |pointer_path_id, offset| env.borrow_mut().push_offset(pointer_path_id, offset)
        })?;

        linker.func("env", "get_u8", {
            let env = env.clone();
            move |pointer_path_id, current| {
                env.borrow()
                    .get_val(pointer_path_id, current, |v| match *v {
                        PointerValue::U8(v) => Some(v as i32),
                        _ => None,
                    })
            }
        })?;

        linker.func("env", "get_u16", {
            let env = env.clone();
            move |pointer_path_id, current| {
                env.borrow()
                    .get_val(pointer_path_id, current, |v| match *v {
                        PointerValue::U16(v) => Some(v as i32),
                        _ => None,
                    })
            }
        })?;

        linker.func("env", "get_u32", {
            let env = env.clone();
            move |pointer_path_id, current| {
                env.borrow()
                    .get_val(pointer_path_id, current, |v| match *v {
                        PointerValue::U32(v) => Some(v as i32),
                        _ => None,
                    })
            }
        })?;

        linker.func("env", "get_u64", {
            let env = env.clone();
            move |pointer_path_id, current| {
                env.borrow()
                    .get_val(pointer_path_id, current, |v| match *v {
                        PointerValue::U64(v) => Some(v as i64),
                        _ => None,
                    })
            }
        })?;

        linker.func("env", "get_i8", {
            let env = env.clone();
            move |pointer_path_id, current| {
                env.borrow()
                    .get_val(pointer_path_id, current, |v| match *v {
                        PointerValue::I8(v) => Some(v as i32),
                        _ => None,
                    })
            }
        })?;

        linker.func("env", "get_i16", {
            let env = env.clone();
            move |pointer_path_id, current| {
                env.borrow()
                    .get_val(pointer_path_id, current, |v| match *v {
                        PointerValue::I16(v) => Some(v as i32),
                        _ => None,
                    })
            }
        })?;

        linker.func("env", "get_i32", {
            let env = env.clone();
            move |pointer_path_id, current| {
                env.borrow()
                    .get_val(pointer_path_id, current, |v| match *v {
                        PointerValue::I32(v) => Some(v),
                        _ => None,
                    })
            }
        })?;

        linker.func("env", "get_i64", {
            let env = env.clone();
            move |pointer_path_id, current| {
                env.borrow()
                    .get_val(pointer_path_id, current, |v| match *v {
                        PointerValue::I64(v) => Some(v),
                        _ => None,
                    })
            }
        })?;

        linker.func("env", "get_f32", {
            let env = env.clone();
            move |pointer_path_id, current| {
                env.borrow()
                    .get_val(pointer_path_id, current, |v| match *v {
                        PointerValue::F32(v) => Some(v),
                        _ => None,
                    })
            }
        })?;

        linker.func("env", "get_f64", {
            let env = env.clone();
            move |pointer_path_id, current| {
                env.borrow()
                    .get_val(pointer_path_id, current, |v| match *v {
                        PointerValue::F64(v) => Some(v),
                        _ => None,
                    })
            }
        })?;

        linker.func("env", "scan_signature", {
            let env = env.clone();
            move |ptr, len| env.borrow_mut().scan_signature(ptr, len)
        })?;

        linker.func("env", "set_tick_rate", {
            let env = env.clone();
            move |ticks_per_sec| env.borrow_mut().set_tick_rate(ticks_per_sec)
        })?;

        linker.func("env", "print_message", {
            let env = env.clone();
            move |ptr, len| env.borrow_mut().print_message(ptr, len)
        })?;

        linker.func("env", "read_into_buf", {
            let env = env.clone();
            move |address, buf, buf_len| env.borrow_mut().read_into_buf(address, buf, buf_len)
        })?;

        linker.func("env", "set_variable", {
            let env = env.clone();
            move |key_ptr, key_len, value_ptr, value_len| {
                env.borrow_mut()
                    .set_variable(key_ptr, key_len, value_ptr, value_len)
            }
        })?;

        let wasi_ctx = WasiCtxBuilder::new()
            .stdout_virt(Box::new(StdStream::stdout()))
            .stderr_virt(Box::new(StdStream::stderr()))
            .build()
            .unwrap();

        Wasi::new(module.store(), wasi_ctx)
            .add_to_linker(&mut linker)
            .unwrap();

        let instance = linker.instantiate(&module)?;
        env.borrow_mut().memory = instance.exports().find_map(Export::into_memory);

        // TODO: _start is kind of correct, but not in the long term. They are
        // intending for us to use a different function for libraries.
        if let Some(func) = instance.get_func("_start") {
            func.get0()?()?;
        }

        // TODO: Do we error out if this doesn't exist?
        if let Some(func) = instance.get_func("configure") {
            func.get0()?()?;
        }

        let hooked = instance
            .get_func("hooked")
            .and_then(|f| f.get0().ok())
            .map(|f| Box::new(f) as _);

        let unhooked = instance
            .get_func("unhooked")
            .and_then(|f| f.get0().ok())
            .map(|f| Box::new(f) as _);

        let should_start = instance
            .get_func("should_start")
            .and_then(|f| f.get0().ok())
            .map(|f| Box::new(f) as _);

        let should_split = instance
            .get_func("should_split")
            .and_then(|f| f.get0().ok())
            .map(|f| Box::new(f) as _);

        let should_reset = instance
            .get_func("should_reset")
            .and_then(|f| f.get0().ok())
            .map(|f| Box::new(f) as _);

        let is_loading = instance
            .get_func("is_loading")
            .and_then(|f| f.get0().ok())
            .map(|f| Box::new(f) as _);

        let game_time = instance
            .get_func("game_time")
            .and_then(|f| f.get0().ok())
            .map(|f| Box::new(f) as _);

        let update = instance
            .get_func("update")
            .and_then(|f| f.get0().ok())
            .map(|f| Box::new(f) as _);

        Ok(Self {
            env,
            timer_state: TimerState::NotRunning,
            hooked,
            unhooked,
            should_start,
            should_split,
            should_reset,
            is_loading,
            game_time,
            update,
            is_loading_val: None,
            game_time_val: None,
        })
    }

    pub fn sleep(&self) {
        let env = self.env.borrow();
        let duration = if env.process.is_some() {
            env.tick_rate
        } else {
            Duration::from_secs(1)
        };
        thread::sleep(duration);
    }

    pub fn step(&mut self) -> anyhow::Result<Option<TimerAction>> {
        {
            let mut just_connected = false;

            let mut env = self.env.borrow_mut();
            if env.process.is_none() {
                env.process = match Process::with_name(&env.process_name) {
                    Ok(p) => Some(p),
                    Err(_) => return Ok(None),
                };
                log::info!(target: "Auto Splitter", "Hooked");
                just_connected = true;
            }
            if env.update_values(just_connected).is_err() {
                log::info!(target: "Auto Splitter", "Unhooked");
                env.process = None;
                if !just_connected {
                    if let Some(unhooked) = &self.unhooked {
                        unhooked()?;
                    }
                }
                return Ok(None);
            }
            if just_connected {
                if let Some(hooked) = &self.hooked {
                    hooked()?;
                }
            }
        }

        self.run_script()
    }

    pub fn set_state(&mut self, state: TimerState) {
        self.timer_state = state;
    }

    fn run_script(&mut self) -> anyhow::Result<Option<TimerAction>> {
        if let Some(update) = &self.update {
            update()?;
        }

        match self.timer_state {
            TimerState::NotRunning => {
                if let Some(should_start) = &self.should_start {
                    if should_start()? != 0 {
                        return Ok(Some(TimerAction::Start));
                    }
                }
            }
            TimerState::Running => {
                if let Some(is_loading) = &self.is_loading {
                    self.is_loading_val = Some(is_loading()? != 0);
                }
                if let Some(game_time) = &self.game_time {
                    self.game_time_val = Some(game_time()?).filter(|v| !v.is_nan());
                }

                if let Some(should_split) = &self.should_split {
                    if should_split()? != 0 {
                        return Ok(Some(TimerAction::Split));
                    }
                }
                if let Some(should_reset) = &self.should_reset {
                    if should_reset()? != 0 {
                        return Ok(Some(TimerAction::Reset));
                    }
                }
            }
            TimerState::Finished => {
                if let Some(should_reset) = &self.should_reset {
                    if should_reset()? != 0 {
                        return Ok(Some(TimerAction::Reset));
                    }
                }
            }
        }

        Ok(None)
    }

    pub fn is_loading(&self) -> Option<bool> {
        self.is_loading_val
    }

    pub fn game_time(&self) -> Option<f64> {
        self.game_time_val
    }

    pub fn drain_variable_changes(&mut self) -> impl Iterator<Item = (String, String)> {
        // TODO: This is kind of stupid. We lose all the capacity this way.
        mem::take(&mut self.env.borrow_mut().variable_changes).into_iter()
    }
}
