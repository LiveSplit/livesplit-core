//! Provides an event sink specifically for the web. This allows you to provide
//! a JavaScript object that implements the necessary functions to handle the
//! timer events. All of them are optional except for `currentPhase`.

use core::ptr;
use std::sync::Arc;

use livesplit_core::{
    event::{Sink, TimerQuery},
    TimeSpan, TimerPhase,
};
use wasm_bindgen::prelude::*;
use web_sys::js_sys::{Function, Reflect};

use crate::event_sink;

/// An event sink specifically for the web. This allows you to provide a
/// JavaScript object that implements the necessary functions to handle the
/// timer events. All of them are optional except for `currentPhase`.
#[wasm_bindgen]
pub struct WebEventSink {
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
    toggle_timing_method: Option<Function>,
    set_game_time: Option<Function>,
    pause_game_time: Option<Function>,
    resume_game_time: Option<Function>,
    set_custom_variable: Option<Function>,
    current_phase: Function,
}

#[wasm_bindgen]
impl WebEventSink {
    /// Creates a new web event sink with the provided JavaScript object.
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
            toggle_timing_method: get_func(&obj, "toggleTimingMethod"),
            set_game_time: get_func(&obj, "setGameTime"),
            pause_game_time: get_func(&obj, "pauseGameTime"),
            resume_game_time: get_func(&obj, "resumeGameTime"),
            set_custom_variable: get_func(&obj, "setCustomVariable"),
            current_phase: get_func(&obj, "currentPhase").unwrap(),
            obj,
        }
    }

    /// Converts the web event sink into a generic event sink that can be used
    /// by the hotkey system and others.
    pub fn intoGeneric(self) -> usize {
        let owned_event_sink: event_sink::OwnedEventSink =
            Box::new(event_sink::EventSink(Arc::new(self)));
        Box::into_raw(owned_event_sink) as usize
    }
}

fn get_func(obj: &JsValue, func_name: &str) -> Option<Function> {
    Reflect::get(obj, &JsValue::from_str(func_name))
        .ok()?
        .dyn_into()
        .ok()
}

unsafe impl Send for WebEventSink {}
unsafe impl Sync for WebEventSink {}

impl Sink for WebEventSink {
    fn start(&self) {
        if let Some(func) = &self.start {
            let _ = func.call0(&self.obj);
        }
    }

    fn split(&self) {
        if let Some(func) = &self.split {
            let _ = func.call0(&self.obj);
        }
    }

    fn split_or_start(&self) {
        if let Some(func) = &self.split_or_start {
            let _ = func.call0(&self.obj);
        }
    }

    fn reset(&self, save_attempt: Option<bool>) {
        if let Some(func) = &self.reset {
            let _ = func.call1(
                &self.obj,
                &match save_attempt {
                    Some(true) => JsValue::TRUE,
                    Some(false) => JsValue::FALSE,
                    None => JsValue::UNDEFINED,
                },
            );
        }
    }

    fn undo_split(&self) {
        if let Some(func) = &self.undo_split {
            let _ = func.call0(&self.obj);
        }
    }

    fn skip_split(&self) {
        if let Some(func) = &self.skip_split {
            let _ = func.call0(&self.obj);
        }
    }

    fn toggle_pause_or_start(&self) {
        if let Some(func) = &self.toggle_pause_or_start {
            let _ = func.call0(&self.obj);
        }
    }

    fn pause(&self) {
        if let Some(func) = &self.pause {
            let _ = func.call0(&self.obj);
        }
    }

    fn resume(&self) {
        if let Some(func) = &self.resume {
            let _ = func.call0(&self.obj);
        }
    }

    fn undo_all_pauses(&self) {
        if let Some(func) = &self.undo_all_pauses {
            let _ = func.call0(&self.obj);
        }
    }

    fn switch_to_previous_comparison(&self) {
        if let Some(func) = &self.switch_to_previous_comparison {
            let _ = func.call0(&self.obj);
        }
    }

    fn switch_to_next_comparison(&self) {
        if let Some(func) = &self.switch_to_next_comparison {
            let _ = func.call0(&self.obj);
        }
    }

    fn toggle_timing_method(&self) {
        if let Some(func) = &self.toggle_timing_method {
            let _ = func.call0(&self.obj);
        }
    }

    fn set_game_time(&self, time: TimeSpan) {
        if let Some(func) = &self.set_game_time {
            let _ = func.call1(
                &self.obj,
                &JsValue::from_f64(ptr::addr_of!(time) as usize as f64),
            );
        }
    }

    fn pause_game_time(&self) {
        if let Some(func) = &self.pause_game_time {
            let _ = func.call0(&self.obj);
        }
    }

    fn resume_game_time(&self) {
        if let Some(func) = &self.resume_game_time {
            let _ = func.call0(&self.obj);
        }
    }

    fn set_custom_variable(&self, name: &str, value: &str) {
        if let Some(func) = &self.set_custom_variable {
            let _ = func.call2(
                &self.obj,
                &JsValue::from_str(name),
                &JsValue::from_str(value),
            );
        }
    }
}

impl TimerQuery for WebEventSink {
    fn current_phase(&self) -> TimerPhase {
        let phase = self.current_phase.call0(&self.obj).unwrap();
        match phase.as_f64().unwrap() as usize {
            0 => TimerPhase::NotRunning,
            1 => TimerPhase::Running,
            2 => TimerPhase::Paused,
            3 => TimerPhase::Ended,
            _ => panic!("Unknown TimerPhase"),
        }
    }
}
