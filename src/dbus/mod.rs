//! The dbus module provides the functionality necessary to handle DBus
//! communication, specifically regarding reading and writing state to an
//! active timer.

use crate::{
    timing::{SharedTimer, TimerPhase},
    TimeSpan
};
use std::{
    future::pending,
    thread,
    // thread::JoinHandle
};
use tokio::runtime;
use zbus::{
    connection,
    interface,
};

// This type exists mainly to make registering the DBus interface easier.
#[derive(Clone)]
struct DBusTimer(SharedTimer);

#[interface(name = "org.livesplit.LiveSplit")]
impl DBusTimer {
    fn start(&self) {
        self.0.write().unwrap().start();
    }

    fn split(&self) {
        self.0.write().unwrap().split_or_start();
    }

    fn skip_split(&self) {
        self.0.write().unwrap().skip_split();
    }

    fn undo_split(&self) {
        self.0.write().unwrap().undo_split();
    }

    fn toggle_pause(&self) {
        self.0.write().unwrap().toggle_pause_or_start();
    }

    fn reset(&self) {
        self.0.write().unwrap().reset(true);
    }

    fn undo_all_pauses(&self) {
        self.0.write().unwrap().undo_all_pauses();
    }


    #[zbus(property)]
    fn comparison(&self) -> String {
        self.0.write().unwrap().current_comparison().to_string()
    }
    #[zbus(property)]
    fn set_comparison(&self, comparison: &str) {
        let _ = self.0.write().unwrap().set_current_comparison(comparison);
    }

    fn previous_comparison(&self) {
        self.0.write().unwrap().switch_to_previous_comparison();
    }

    fn next_comparison(&self) {
        self.0.write().unwrap().switch_to_next_comparison();
    }


    fn toggle_timing_method(&self) {
        self.0.write().unwrap().toggle_timing_method();
    }

    fn set_game_time(&self, time: f64) {
        self.0.write().unwrap().set_game_time(TimeSpan::from_seconds(time));
    }

    fn pause_game_time(&self) {
        self.0.write().unwrap().pause_game_time();
    }

    fn resume_game_time(&self) {
        self.0.write().unwrap().resume_game_time();
    }

    fn set_variable(&self, name: &str, value: &str) {
        self.0.write().unwrap().set_custom_variable(name, value);
    }

    #[zbus(property)]
    fn running(&self) -> bool {
        self.0.write().unwrap().current_phase() == TimerPhase::Running
    }
    #[zbus(property)]
    fn set_running(&self, running: bool) {
        if running {
            match self.0.write().unwrap().current_phase() {
                TimerPhase::Paused => self.0.write().unwrap().resume(),
                TimerPhase::NotRunning => self.0.write().unwrap().start(),
                _ => {}
            };
        } else {
            self.0.write().unwrap().pause();
        }
    }
}

/// A system reference that will spin up a background thread to handle DBus
/// communication.
pub struct DBusSystem {
    //dbus_worker: JoinHandle<zbus::Result<()>>,
}

impl DBusSystem {
    /// Starts the DBus system, will take ownership of the name and handle
    /// requests in the background.
    pub fn new(timer: SharedTimer) -> Self {
        let dbus_timer = DBusTimer(timer);
        // let dbus_worker = 
        thread::Builder::new()
            .name("DBus Worker".into())
            .spawn(move || {
                runtime::Builder::new_current_thread()
                    .enable_time()
                    .build()
                    .unwrap()
                    .block_on(run_dbus(dbus_timer))
            })
            .unwrap();

        Self {
        //    dbus_worker,
        }
    }
}

async fn run_dbus(
    timer: DBusTimer,
) -> zbus::Result<()> {
    let _conn = connection::Builder::session()?
        .name("org.livesplit.LiveSplit")?
        .serve_at("/org/livesplit/LiveSplit", timer)?
        .build()
        .await?;

    loop {
        pending::<()>().await;
    }
}
