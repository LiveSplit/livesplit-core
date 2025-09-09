//! A command sink accepts commands that are meant to be passed to the timer.
//! The commands usually come from the hotkey system, an auto splitter, the UI,
//! or through a network connection. The UI usually provides the implementation
//! for this, forwarding all the commands to the actual timer. It is able to
//! intercept the commands and for example ask the user for confirmation before
//! applying them. Other handling is possible such as automatically saving the
//! splits or notifying a server about changes happening in the run. After
//! processing a command, changes to the timer are reported as events. Various
//! error conditions can occur if the command couldn't be processed.

use std::{borrow::Cow, future::Future, ops::Deref, pin::Pin, sync::Arc};

use livesplit_core::{
    TimeSpan, Timer, TimingMethod,
    event::{self, Result},
};

use crate::shared_timer::OwnedSharedTimer;

/// type
#[derive(Clone)]
pub struct CommandSink(pub(crate) Arc<dyn CommandSinkAndQuery>);

/// type
pub type OwnedCommandSink = Box<CommandSink>;

/// Creates a new Command Sink.
#[unsafe(no_mangle)]
pub extern "C" fn CommandSink_from_timer(timer: OwnedSharedTimer) -> OwnedCommandSink {
    Box::new(CommandSink(Arc::new(*timer)))
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn CommandSink_drop(this: OwnedCommandSink) {
    drop(this);
}

pub(crate) trait CommandSinkAndQuery: Send + Sync + 'static {
    fn dyn_query<'a>(&'a self) -> Box<dyn Deref<Target = Timer> + 'a>;
    fn dyn_start(&self) -> Fut;
    fn dyn_split(&self) -> Fut;
    fn dyn_split_or_start(&self) -> Fut;
    fn dyn_reset(&self, save_attempt: Option<bool>) -> Fut;
    fn dyn_undo_split(&self) -> Fut;
    fn dyn_skip_split(&self) -> Fut;
    fn dyn_toggle_pause_or_start(&self) -> Fut;
    fn dyn_pause(&self) -> Fut;
    fn dyn_resume(&self) -> Fut;
    fn dyn_undo_all_pauses(&self) -> Fut;
    fn dyn_switch_to_previous_comparison(&self) -> Fut;
    fn dyn_switch_to_next_comparison(&self) -> Fut;
    fn dyn_set_current_comparison(&self, comparison: Cow<str>) -> Fut;
    fn dyn_toggle_timing_method(&self) -> Fut;
    fn dyn_set_current_timing_method(&self, method: TimingMethod) -> Fut;
    fn dyn_initialize_game_time(&self) -> Fut;
    fn dyn_set_game_time(&self, time: TimeSpan) -> Fut;
    fn dyn_pause_game_time(&self) -> Fut;
    fn dyn_resume_game_time(&self) -> Fut;
    fn dyn_set_loading_times(&self, time: TimeSpan) -> Fut;
    fn dyn_set_custom_variable(&self, name: Cow<str>, value: Cow<str>) -> Fut;
}

type Fut = Pin<Box<dyn Future<Output = Result> + 'static>>;

impl<T> CommandSinkAndQuery for T
where
    T: event::CommandSink + event::TimerQuery + Send + Sync + 'static,
{
    fn dyn_query<'a>(&'a self) -> Box<dyn Deref<Target = Timer> + 'a> {
        Box::new(self.get_timer())
    }

    fn dyn_start(&self) -> Fut {
        Box::pin(self.start())
    }
    fn dyn_split(&self) -> Fut {
        Box::pin(self.split())
    }
    fn dyn_split_or_start(&self) -> Fut {
        Box::pin(self.split_or_start())
    }
    fn dyn_reset(&self, save_attempt: Option<bool>) -> Fut {
        Box::pin(self.reset(save_attempt))
    }
    fn dyn_undo_split(&self) -> Fut {
        Box::pin(self.undo_split())
    }
    fn dyn_skip_split(&self) -> Fut {
        Box::pin(self.skip_split())
    }
    fn dyn_toggle_pause_or_start(&self) -> Fut {
        Box::pin(self.toggle_pause_or_start())
    }
    fn dyn_pause(&self) -> Fut {
        Box::pin(self.pause())
    }
    fn dyn_resume(&self) -> Fut {
        Box::pin(self.resume())
    }
    fn dyn_undo_all_pauses(&self) -> Fut {
        Box::pin(self.undo_all_pauses())
    }
    fn dyn_switch_to_previous_comparison(&self) -> Fut {
        Box::pin(self.switch_to_previous_comparison())
    }
    fn dyn_switch_to_next_comparison(&self) -> Fut {
        Box::pin(self.switch_to_next_comparison())
    }
    fn dyn_set_current_comparison(&self, comparison: Cow<str>) -> Fut {
        Box::pin(self.set_current_comparison(comparison))
    }
    fn dyn_toggle_timing_method(&self) -> Fut {
        Box::pin(self.toggle_timing_method())
    }
    fn dyn_set_current_timing_method(&self, method: TimingMethod) -> Fut {
        Box::pin(self.set_current_timing_method(method))
    }
    fn dyn_initialize_game_time(&self) -> Fut {
        Box::pin(self.initialize_game_time())
    }
    fn dyn_set_game_time(&self, time: TimeSpan) -> Fut {
        Box::pin(self.set_game_time(time))
    }
    fn dyn_pause_game_time(&self) -> Fut {
        Box::pin(self.pause_game_time())
    }
    fn dyn_resume_game_time(&self) -> Fut {
        Box::pin(self.resume_game_time())
    }
    fn dyn_set_loading_times(&self, time: TimeSpan) -> Fut {
        Box::pin(self.set_loading_times(time))
    }
    fn dyn_set_custom_variable(&self, name: Cow<str>, value: Cow<str>) -> Fut {
        Box::pin(self.set_custom_variable(name, value))
    }
}

impl event::CommandSink for CommandSink {
    fn start(&self) -> impl Future<Output = Result> + 'static {
        self.0.dyn_start()
    }

    fn split(&self) -> impl Future<Output = Result> + 'static {
        self.0.dyn_split()
    }

    fn split_or_start(&self) -> impl Future<Output = Result> + 'static {
        self.0.dyn_split_or_start()
    }

    fn reset(&self, save_attempt: Option<bool>) -> impl Future<Output = Result> + 'static {
        self.0.dyn_reset(save_attempt)
    }

    fn undo_split(&self) -> impl Future<Output = Result> + 'static {
        self.0.dyn_undo_split()
    }

    fn skip_split(&self) -> impl Future<Output = Result> + 'static {
        self.0.dyn_skip_split()
    }

    fn toggle_pause_or_start(&self) -> impl Future<Output = Result> + 'static {
        self.0.dyn_toggle_pause_or_start()
    }

    fn pause(&self) -> impl Future<Output = Result> + 'static {
        self.0.dyn_pause()
    }

    fn resume(&self) -> impl Future<Output = Result> + 'static {
        self.0.dyn_resume()
    }

    fn undo_all_pauses(&self) -> impl Future<Output = Result> + 'static {
        self.0.dyn_undo_all_pauses()
    }

    fn switch_to_previous_comparison(&self) -> impl Future<Output = Result> + 'static {
        self.0.dyn_switch_to_previous_comparison()
    }

    fn switch_to_next_comparison(&self) -> impl Future<Output = Result> + 'static {
        self.0.dyn_switch_to_next_comparison()
    }

    fn set_current_comparison(
        &self,
        comparison: Cow<str>,
    ) -> impl Future<Output = Result> + 'static {
        self.0.dyn_set_current_comparison(comparison)
    }

    fn toggle_timing_method(&self) -> impl Future<Output = Result> + 'static {
        self.0.dyn_toggle_timing_method()
    }

    fn set_current_timing_method(
        &self,
        method: TimingMethod,
    ) -> impl Future<Output = Result> + 'static {
        self.0.dyn_set_current_timing_method(method)
    }

    fn initialize_game_time(&self) -> impl Future<Output = Result> + 'static {
        self.0.dyn_initialize_game_time()
    }

    fn set_game_time(&self, time: TimeSpan) -> impl Future<Output = Result> + 'static {
        self.0.dyn_set_game_time(time)
    }

    fn pause_game_time(&self) -> impl Future<Output = Result> + 'static {
        self.0.dyn_pause_game_time()
    }

    fn resume_game_time(&self) -> impl Future<Output = Result> + 'static {
        self.0.dyn_resume_game_time()
    }

    fn set_loading_times(&self, time: TimeSpan) -> impl Future<Output = Result> + 'static {
        self.0.dyn_set_loading_times(time)
    }

    fn set_custom_variable(
        &self,
        name: Cow<str>,
        value: Cow<str>,
    ) -> impl Future<Output = Result> + 'static {
        self.0.dyn_set_custom_variable(name, value)
    }
}

impl event::TimerQuery for CommandSink {
    type Guard<'a> = TimerGuard<'a>;

    fn get_timer(&self) -> Self::Guard<'_> {
        TimerGuard(self.0.dyn_query())
    }
}

/// type
#[repr(transparent)]
pub struct TimerGuard<'a>(Box<dyn Deref<Target = Timer> + 'a>);

impl std::ops::Deref for TimerGuard<'_> {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
