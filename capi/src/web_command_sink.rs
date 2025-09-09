//! Provides a command sink specifically for the web. This allows you to provide
//! a JavaScript object that implements the necessary functions to handle the
//! timer commands. All of them are optional except for `getTimer`.

use std::{borrow::Cow, cell::Cell, convert::TryFrom, future::Future, sync::Arc};

use livesplit_core::{
    TimeSpan, Timer, TimingMethod,
    event::{CommandSink, Error, Event, Result, TimerQuery},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{Function, Promise, Reflect};

use crate::command_sink;

/// A command sink specifically for the web. This allows you to provide a
/// JavaScript object that implements the necessary functions to handle the
/// timer commands. All of them are optional except for `getTimer`.
#[wasm_bindgen]
pub struct WebCommandSink {
    obj: JsValue,
    start: Option<Function>,
    split: Option<Function>,
    split_or_start: Option<Function>,
    reset: Option<Function>,
    undo_split: Option<Function>,
    skip_split: Option<Function>,
    toggle_pause_or_start: Option<Function>,
    pause: Option<Function>,
    resume: Option<Function>,
    undo_all_pauses: Option<Function>,
    switch_to_previous_comparison: Option<Function>,
    switch_to_next_comparison: Option<Function>,
    set_current_comparison: Option<Function>,
    toggle_timing_method: Option<Function>,
    set_current_timing_method: Option<Function>,
    initialize_game_time: Option<Function>,
    set_game_time: Option<Function>,
    pause_game_time: Option<Function>,
    resume_game_time: Option<Function>,
    set_loading_times: Option<Function>,
    set_custom_variable: Option<Function>,

    get_timer: Function,
    locked: Cell<bool>,
}

#[wasm_bindgen]
impl WebCommandSink {
    /// Creates a new web command sink with the provided JavaScript object.
    #[wasm_bindgen(constructor)]
    pub fn new(obj: JsValue) -> Self {
        Self {
            start: get_func(&obj, "start"),
            split: get_func(&obj, "split"),
            split_or_start: get_func(&obj, "splitOrStart"),
            reset: get_func(&obj, "reset"),
            undo_split: get_func(&obj, "undoSplit"),
            skip_split: get_func(&obj, "skipSplit"),
            toggle_pause_or_start: get_func(&obj, "togglePauseOrStart"),
            pause: get_func(&obj, "pause"),
            resume: get_func(&obj, "resume"),
            undo_all_pauses: get_func(&obj, "undoAllPauses"),
            switch_to_previous_comparison: get_func(&obj, "switchToPreviousComparison"),
            switch_to_next_comparison: get_func(&obj, "switchToNextComparison"),
            set_current_comparison: get_func(&obj, "setCurrentComparison"),
            toggle_timing_method: get_func(&obj, "toggleTimingMethod"),
            set_current_timing_method: get_func(&obj, "setCurrentTimingMethod"),
            initialize_game_time: get_func(&obj, "initializeGameTime"),
            set_game_time: get_func(&obj, "setGameTime"),
            pause_game_time: get_func(&obj, "pauseGameTime"),
            resume_game_time: get_func(&obj, "resumeGameTime"),
            set_loading_times: get_func(&obj, "setLoadingTimes"),
            set_custom_variable: get_func(&obj, "setCustomVariable"),

            get_timer: get_func(&obj, "getTimer").unwrap(),
            locked: Cell::new(false),
            obj,
        }
    }

    /// Converts the web command sink into a generic command sink that can be
    /// used by the hotkey system and others.
    pub fn intoGeneric(self) -> usize {
        let owned_command_sink: command_sink::OwnedCommandSink =
            Box::new(command_sink::CommandSink(Arc::new(self)));
        Box::into_raw(owned_command_sink) as usize
    }
}

fn get_func(obj: &JsValue, func_name: &str) -> Option<Function> {
    Reflect::get(obj, &JsValue::from_str(func_name))
        .ok()?
        .dyn_into()
        .ok()
}

unsafe impl Send for WebCommandSink {}
unsafe impl Sync for WebCommandSink {}

async fn handle_action_value(value: Option<JsValue>) -> Result {
    if let Some(mut value) = value {
        if let Ok(promise) = JsCast::dyn_into::<Promise>(value.clone()) {
            value = match JsFuture::from(promise).await {
                Ok(value) | Err(value) => value,
            };
        }
        if let Some(value) = value.as_f64() {
            let value = value as i32;
            if value >= 0 {
                Ok(Event::try_from(value as u32).map_err(|_| Error::Unknown)?)
            } else {
                Err(Error::from((-value - 1) as u32))
            }
        } else {
            Err(Error::Unknown)
        }
    } else {
        Err(Error::Unsupported)
    }
}

impl CommandSink for WebCommandSink {
    fn start(&self) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(self.start.as_ref().and_then(|f| f.call0(&self.obj).ok()))
    }

    fn split(&self) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(self.split.as_ref().and_then(|f| f.call0(&self.obj).ok()))
    }

    fn split_or_start(&self) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(
            self.split_or_start
                .as_ref()
                .and_then(|f| f.call0(&self.obj).ok()),
        )
    }

    fn reset(&self, save_attempt: Option<bool>) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(self.reset.as_ref().and_then(|f| {
            f.call1(
                &self.obj,
                &match save_attempt {
                    Some(true) => JsValue::TRUE,
                    Some(false) => JsValue::FALSE,
                    None => JsValue::UNDEFINED,
                },
            )
            .ok()
        }))
    }

    fn undo_split(&self) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(
            self.undo_split
                .as_ref()
                .and_then(|f| f.call0(&self.obj).ok()),
        )
    }

    fn skip_split(&self) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(
            self.skip_split
                .as_ref()
                .and_then(|f| f.call0(&self.obj).ok()),
        )
    }

    fn toggle_pause_or_start(&self) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(
            self.toggle_pause_or_start
                .as_ref()
                .and_then(|f| f.call0(&self.obj).ok()),
        )
    }

    fn pause(&self) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(self.pause.as_ref().and_then(|f| f.call0(&self.obj).ok()))
    }

    fn resume(&self) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(self.resume.as_ref().and_then(|f| f.call0(&self.obj).ok()))
    }

    fn undo_all_pauses(&self) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(
            self.undo_all_pauses
                .as_ref()
                .and_then(|f| f.call0(&self.obj).ok()),
        )
    }

    fn switch_to_previous_comparison(&self) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(
            self.switch_to_previous_comparison
                .as_ref()
                .and_then(|f| f.call0(&self.obj).ok()),
        )
    }

    fn switch_to_next_comparison(&self) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(
            self.switch_to_next_comparison
                .as_ref()
                .and_then(|f| f.call0(&self.obj).ok()),
        )
    }

    fn set_current_comparison(
        &self,
        comparison: Cow<str>,
    ) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(
            self.set_current_comparison
                .as_ref()
                .and_then(|f| f.call1(&self.obj, &JsValue::from_str(&comparison)).ok()),
        )
    }

    fn toggle_timing_method(&self) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(
            self.toggle_timing_method
                .as_ref()
                .and_then(|f| f.call0(&self.obj).ok()),
        )
    }

    fn set_current_timing_method(
        &self,
        timing_method: TimingMethod,
    ) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(self.set_current_timing_method.as_ref().and_then(|f| {
            f.call1(&self.obj, &JsValue::from_f64(timing_method as usize as f64))
                .ok()
        }))
    }

    fn initialize_game_time(&self) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(
            self.initialize_game_time
                .as_ref()
                .and_then(|f| f.call0(&self.obj).ok()),
        )
    }

    fn set_game_time(&self, time: TimeSpan) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(self.set_game_time.as_ref().and_then(|f| {
            f.call1(
                &self.obj,
                &JsValue::from_f64(&raw const time as usize as f64),
            )
            .ok()
        }))
    }

    fn pause_game_time(&self) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(
            self.pause_game_time
                .as_ref()
                .and_then(|f| f.call0(&self.obj).ok()),
        )
    }

    fn resume_game_time(&self) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(
            self.resume_game_time
                .as_ref()
                .and_then(|f| f.call0(&self.obj).ok()),
        )
    }

    fn set_loading_times(&self, time: TimeSpan) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(self.set_loading_times.as_ref().and_then(|f| {
            f.call1(
                &self.obj,
                &JsValue::from_f64(&raw const time as usize as f64),
            )
            .ok()
        }))
    }

    fn set_custom_variable(
        &self,
        name: Cow<str>,
        value: Cow<str>,
    ) -> impl Future<Output = Result> + 'static {
        debug_assert!(!self.locked.get());
        handle_action_value(self.set_custom_variable.as_ref().and_then(|f| {
            f.call2(
                &self.obj,
                &JsValue::from_str(&name),
                &JsValue::from_str(&value),
            )
            .ok()
        }))
    }
}

/// type
pub struct WebGuard<'a>(&'a Timer, &'a Cell<bool>);

impl Drop for WebGuard<'_> {
    fn drop(&mut self) {
        self.1.set(false);
    }
}

impl std::ops::Deref for WebGuard<'_> {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl TimerQuery for WebCommandSink {
    type Guard<'a> = WebGuard<'a>;

    fn get_timer(&self) -> Self::Guard<'_> {
        debug_assert!(!self.locked.replace(true));
        unsafe {
            WebGuard(
                &*(self.get_timer.call0(&self.obj).unwrap().as_f64().unwrap() as usize
                    as *const Timer),
                &self.locked,
            )
        }
    }
}
