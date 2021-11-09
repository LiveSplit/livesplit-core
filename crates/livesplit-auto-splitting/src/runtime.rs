use crate::{
    environment::Environment,
    timer::Timer,
    InterruptHandle,
};
use std::{cell::RefCell, rc::Rc, thread, time::Instant};
use wasmtime::{Config, Engine, Export, Instance, Linker, Module, Store, TypedFunc};

// TODO: Check if there's any memory leaks due to reference cycles. The
// exports keep the instance alive which keeps the imports alive, which all
// keep the environment alive, which keeps the memory alive, which may keep the
// instance alive -> reference cycle.
pub struct Runtime<T> {
    instance: Instance,
    is_configured: bool,
    env: Rc<RefCell<Environment<T>>>,
    update: Option<TypedFunc<(), ()>>,
    prev_time: Instant,
}

impl<T: Timer> Runtime<T> {
    pub fn new(binary: &[u8], timer: T) -> anyhow::Result<Self> {
        let engine = Engine::new(Config::new().interruptable(true))?;
        let store = Store::new(&engine);
        let module = Module::from_binary(&engine, binary)?;
        let env = Rc::new(RefCell::new(Environment::new(timer)));

        let mut linker = Linker::new(&store);

        linker.func("env", "start", {
            let env = env.clone();
            move || env.borrow_mut().start()
        })?;

        linker.func("env", "split", {
            let env = env.clone();
            move || env.borrow_mut().split()
        })?;

        linker.func("env", "reset", {
            let env = env.clone();
            move || env.borrow_mut().reset()
        })?;

        linker.func("env", "attach", {
            let env = env.clone();
            move |ptr, len| env.borrow_mut().attach(ptr, len)
        })?;

        linker.func("env", "detach", {
            let env = env.clone();
            move |process| env.borrow_mut().detach(process)
        })?;

        linker.func("env", "read_into_buf", {
            let env = env.clone();
            move |process, address, buf_ptr, buf_len| {
                env.borrow_mut()
                    .read_into_buf(process, address, buf_ptr, buf_len)
            }
        })?;

        linker.func("env", "set_tick_rate", {
            let env = env.clone();
            move |ticks_per_sec| env.borrow_mut().set_tick_rate(ticks_per_sec)
        })?;

        linker.func("env", "print_message", {
            let env = env.clone();
            move |ptr, len| env.borrow_mut().print_message(ptr, len)
        })?;

        linker.func("env", "set_variable", {
            let env = env.clone();
            move |key_ptr, key_len, value_ptr, value_len| {
                env.borrow_mut()
                    .set_variable(key_ptr, key_len, value_ptr, value_len)
            }
        })?;

        linker.func("env", "set_game_time", {
            let env = env.clone();
            move |secs, nanos| env.borrow_mut().set_game_time(secs, nanos)
        })?;

        linker.func("env", "pause_game_time", {
            let env = env.clone();
            move || env.borrow_mut().pause_game_time()
        })?;

        linker.func("env", "resume_game_time", {
            let env = env.clone();
            move || env.borrow_mut().resume_game_time()
        })?;

        linker.func("env", "get_timer_state", {
            let env = env.clone();
            move || env.borrow().timer_state()
        })?;

        let instance = linker.instantiate(&module)?;
        env.borrow_mut().memory = instance.exports().find_map(Export::into_memory);

        let update = instance.get_typed_func("update").ok();

        Ok(Self {
            instance,
            is_configured: false,
            env,
            update,
            prev_time: Instant::now(),
        })
    }

    pub fn interrupt_handle(&self) -> InterruptHandle {
        self.instance
            .store()
            .interrupt_handle()
            .expect("We configured the runtime to produce an interrupt handle")
    }

    pub fn step(&mut self) -> anyhow::Result<()> {
        if !self.is_configured {
            // TODO: _start is kind of correct, but not in the long term
            // See: https://github.com/WebAssembly/WASI/issues/24
            if let Ok(func) = self.instance.get_typed_func("_start") {
                func.call(())?;
            }
            // TODO: Do we error out if this doesn't exist?
            if let Ok(func) = self.instance.get_typed_func("configure") {
                func.call(())?;
            }
            self.is_configured = true;
        }
        self.run_script()
    }

    fn run_script(&mut self) -> anyhow::Result<()> {
        if let Some(update) = &self.update {
            update.call(())?;
        }
        Ok(())
    }

    pub fn sleep(&mut self) {
        let target = self.env.borrow().tick_rate;
        let delta = self.prev_time.elapsed();
        if delta < target {
            thread::sleep(target - delta);
        }
        self.prev_time = Instant::now();
    }
}
