use {
    crate::{timing::SharedTimer, TimeSpan, TimerPhase},
    crossbeam_channel::{unbounded, Receiver, Sender},
    livesplit_auto_splitting::{Runtime as ScriptRuntime, TimerAction, TimerState},
    std::{
        error::Error,
        thread::{self, Thread},
    },
};

pub struct Runtime {
    sender: Sender<Request>,
}

impl Runtime {
    pub fn new(timer: SharedTimer) -> Self {
        let (sender, receiver) = unbounded();
        // TODO: Store the join handle
        thread::spawn(move || {
            'back_to_not_having_a_runtime: loop {
                let mut runtime = loop {
                    let message = receiver.recv().unwrap(); // TODO: No unwrap
                    match message {
                        Request::LoadScript(script) => {
                            break ScriptRuntime::new(&script).unwrap();
                            // TODO: Send back result instead of unwrapping
                        }
                        Request::UnloadScript => {
                            // TODO: Nothing to do. Send back a result?
                        }
                    }
                };
                log::info!(target: "Auto Splitter", "Loaded script");
                loop {
                    // TODO: Handle the different kinds of errors here, such as
                    // needing to end
                    if let Ok(message) = receiver.try_recv() {
                        match message {
                            Request::LoadScript(script) => {
                                runtime = ScriptRuntime::new(&script).unwrap();
                                log::info!(target: "Auto Splitter", "Loaded script");
                                // TODO: Send back result instead of unwrapping
                            }
                            Request::UnloadScript => {
                                log::info!(target: "Auto Splitter", "Unloaded script");
                                continue 'back_to_not_having_a_runtime;
                            }
                        }
                    }
                    let phase = timer.read().current_phase();
                    runtime.set_state(match phase {
                        TimerPhase::NotRunning => TimerState::NotRunning,
                        TimerPhase::Running | TimerPhase::Paused => TimerState::Running,
                        TimerPhase::Ended => TimerState::Finished,
                    });
                    // TODO: No unwrap
                    let action = match runtime.step() {
                        Ok(action) => action,
                        Err(e) => {
                            log::error!(target: "Auto Splitter", "Unloaded the auto splitter because it failed executing: {}", e);
                            continue 'back_to_not_having_a_runtime;
                        }
                    };
                    for (key, value) in runtime.drain_variable_changes() {
                        timer.write().set_custom_variable(key, value);
                    }
                    if let Some(is_loading) = runtime.is_loading() {
                        if is_loading {
                            timer.write().pause_game_time();
                        } else {
                            timer.write().resume_game_time();
                        }
                    }
                    if let Some(game_time) = runtime.game_time() {
                        timer
                            .write()
                            .set_game_time(TimeSpan::from_seconds(game_time));
                    }
                    if let Some(action) = action {
                        let mut timer = timer.write();
                        match action {
                            TimerAction::Start => timer.start(),
                            TimerAction::Split => timer.split(),
                            TimerAction::Reset => timer.reset(true),
                        }
                    }
                    runtime.sleep();
                }
            }
        });

        Self { sender }
    }

    // TODO: Fix this error type
    // TODO: Futures based API
    pub fn load_script(&self, script: Vec<u8>) -> Result<(), Box<dyn Error>> {
        // TODO: unwrap
        self.sender.send(Request::LoadScript(script)).unwrap();
        // TODO: receive result
        Ok(())
    }

    pub fn unload_script(&self) -> Result<(), Box<dyn Error>> {
        // TODO: unwrap
        self.sender.send(Request::UnloadScript).unwrap();
        // TODO: receive result
        Ok(())
    }
}

enum Request {
    LoadScript(Vec<u8>),
    UnloadScript,
}
