//! An event sink accepts events that are meant to be passed to the timer. The
//! events usually come from the hotkey system, an auto splitter, the UI, or
//! through a network connection. The UI usually provides the implementation for
//! this, forwarding all the events to the actual timer. It is able to intercept
//! the events and for example ask the user for confirmation before applying
//! them. Other handling is possible such as automatically saving the splits or
//! notifying a server about changes happening in the run.

use std::sync::Arc;

use crate::shared_timer::OwnedSharedTimer;

/// type
#[derive(Clone)]
pub struct EventSink(pub(crate) Arc<dyn EventSinkAndQuery>);

/// type
pub type OwnedEventSink = Box<EventSink>;

/// Creates a new Event Sink.
#[no_mangle]
pub extern "C" fn EventSink_from_timer(timer: OwnedSharedTimer) -> OwnedEventSink {
    Box::new(EventSink(Arc::new(*timer)))
}

/// drop
#[no_mangle]
pub extern "C" fn EventSink_drop(this: OwnedEventSink) {
    drop(this);
}

pub(crate) trait EventSinkAndQuery:
    livesplit_core::event::Sink + livesplit_core::event::TimerQuery + Send + Sync + 'static
{
}

impl<T> EventSinkAndQuery for T where
    T: livesplit_core::event::Sink + livesplit_core::event::TimerQuery + Send + Sync + 'static
{
}

impl livesplit_core::event::Sink for EventSink {
    fn start(&self) {
        self.0.start()
    }

    fn split(&self) {
        self.0.split()
    }

    fn split_or_start(&self) {
        self.0.split_or_start()
    }

    fn reset(&self, save_attempt: Option<bool>) {
        self.0.reset(save_attempt)
    }

    fn undo_split(&self) {
        self.0.undo_split()
    }

    fn skip_split(&self) {
        self.0.skip_split()
    }

    fn toggle_pause_or_start(&self) {
        self.0.toggle_pause_or_start()
    }

    fn pause(&self) {
        self.0.pause()
    }

    fn resume(&self) {
        self.0.resume()
    }

    fn undo_all_pauses(&self) {
        self.0.undo_all_pauses()
    }

    fn switch_to_previous_comparison(&self) {
        self.0.switch_to_previous_comparison()
    }

    fn switch_to_next_comparison(&self) {
        self.0.switch_to_next_comparison()
    }

    fn toggle_timing_method(&self) {
        self.0.toggle_timing_method()
    }

    fn set_game_time(&self, time: livesplit_core::TimeSpan) {
        self.0.set_game_time(time)
    }

    fn pause_game_time(&self) {
        self.0.pause_game_time()
    }

    fn resume_game_time(&self) {
        self.0.resume_game_time()
    }

    fn set_custom_variable(&self, name: &str, value: &str) {
        self.0.set_custom_variable(name, value)
    }
}
