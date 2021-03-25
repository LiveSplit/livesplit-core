use crate::{
    environment::Environment,
    pointer::PointerValue,
    process::Process,
    std_stream::{stderr, stdout},
    InterruptHandle,
};
use std::{cell::RefCell, mem, rc::Rc, thread, time::Duration};
use wasi_cap_std_sync::WasiCtxBuilder;
use wasmtime::{Config, Engine, Export, Instance, Linker, Module, Store, TypedFunc};
use wasmtime_wasi::Wasi;

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
    instance: Instance,
    is_configured: bool,
    env: Rc<RefCell<Environment>>,
    timer_state: TimerState,
    hooked: Option<TypedFunc<(), ()>>,
    unhooked: Option<TypedFunc<(), ()>>,
    should_start: Option<TypedFunc<(), i32>>,
    should_split: Option<TypedFunc<(), i32>>,
    should_reset: Option<TypedFunc<(), i32>>,
    is_loading: Option<TypedFunc<(), i32>>,
    game_time: Option<TypedFunc<(), f64>>,
    update: Option<TypedFunc<(), ()>>,
    is_loading_val: Option<bool>,
    game_time_val: Option<f64>,
}

impl Runtime {
    pub fn new(binary: &[u8]) -> anyhow::Result<Self> {
        let engine = Engine::new(Config::new().interruptable(true))?;
        let store = Store::new(&engine);
        let module = Module::from_binary( &engine, binary)?;
        let env = Rc::new(RefCell::new(Environment::default()));
        let mut linker = Linker::new(&store);

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
            .stdout(Box::new(stdout()))
            .stderr(Box::new(stderr()))
            .build()
            .unwrap();

        Wasi::new(&store, wasi_ctx)
            .add_to_linker(&mut linker)
            .unwrap();

        let instance = linker.instantiate(&module)?;
        env.borrow_mut().memory = instance.exports().find_map(Export::into_memory);

        let hooked = instance
            .get_typed_func("hooked").ok();

        let unhooked = instance
            .get_typed_func("unhooked").ok();

        let should_start = instance
            .get_typed_func("should_start").ok();

        let should_split = instance
            .get_typed_func("should_split").ok();

        let should_reset = instance
            .get_typed_func("should_reset").ok();

        let is_loading = instance
            .get_typed_func("is_loading").ok();

        let game_time = instance
            .get_typed_func("game_time").ok();

        let update = instance
            .get_typed_func("update").ok();

        Ok(Self {
            instance,
            is_configured: false,
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

    pub fn interrupt_handle(&self) -> InterruptHandle {
        self.instance
            .store()
            .interrupt_handle()
            .expect("We configured the runtime to produce an interrupt handle")
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
        if !self.is_configured {
            // TODO: _start is kind of correct, but not in the long term. They are
            // intending for us to use a different function for libraries. Look into
            // reactors.
            if let Ok(func) = self.instance.get_typed_func("_start") {
                func.call(())?;
            }

            // TODO: Do we error out if this doesn't exist?
            if let Ok(func) = self.instance.get_typed_func("configure") {
                func.call(())?;
            }
            self.is_configured = true;
        }

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
                        unhooked.call(())?;
                    }
                }
                return Ok(None);
            }
            if just_connected {
                if let Some(hooked) = &self.hooked {
                    hooked.call(())?;
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
            update.call(())?;
        }

        match self.timer_state {
            TimerState::NotRunning => {
                if let Some(should_start) = &self.should_start {
                    if should_start.call(())? != 0 {
                        return Ok(Some(TimerAction::Start));
                    }
                }
            }
            TimerState::Running => {
                if let Some(is_loading) = &self.is_loading {
                    self.is_loading_val = Some(is_loading.call(())? != 0);
                }
                if let Some(game_time) = &self.game_time {
                    self.game_time_val = Some(game_time.call(())?).filter(|v| !v.is_nan());
                }

                if let Some(should_split) = &self.should_split {
                    if should_split.call(())? != 0 {
                        return Ok(Some(TimerAction::Split));
                    }
                }
                if let Some(should_reset) = &self.should_reset {
                    if should_reset.call(())? != 0 {
                        return Ok(Some(TimerAction::Reset));
                    }
                }
            }
            TimerState::Finished => {
                if let Some(should_reset) = &self.should_reset {
                    if should_reset.call(())? != 0 {
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
