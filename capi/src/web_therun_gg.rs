//! Provides a therun.gg client for the web that communicates with therun.gg to
//! provide live tracking and stats uploading for speedruns.

use livesplit_core::{Timer, event::Event, networking::therun_gg};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

/// A client for communicating with therun.gg. It handles live split tracking
/// and stats uploading. The upload key is a 36-character key obtained from
/// therun.gg.
#[wasm_bindgen]
pub struct TheRunClient {
    inner: therun_gg::Client,
}

#[wasm_bindgen]
impl TheRunClient {
    /// Creates a new therun.gg client with the given upload key and settings.
    #[wasm_bindgen(constructor)]
    pub fn new(
        upload_key: &str,
        is_live_tracking_enabled: bool,
        is_stats_uploading_enabled: bool,
    ) -> Self {
        Self {
            inner: therun_gg::Client::new(
                upload_key.to_owned(),
                is_live_tracking_enabled,
                is_stats_uploading_enabled,
            ),
        }
    }

    /// Sets whether live tracking is enabled. When enabled, live split data is
    /// sent to therun.gg after every split action.
    pub fn setLiveTrackingEnabled(&mut self, enabled: bool) {
        self.inner.set_live_tracking_enabled(enabled);
    }

    /// Sets whether stats uploading is enabled. When enabled, the .lss file is
    /// uploaded to therun.gg after every reset or finished run.
    pub fn setStatsUploadingEnabled(&mut self, enabled: bool) {
        self.inner.set_stats_uploading_enabled(enabled);
    }

    /// Handles a timer event. Pass the event code and a pointer to the timer.
    /// The event codes correspond to the `Event` enum variants (e.g. 0 =
    /// Started, 6 = Paused, 7 = Resumed, etc.).
    pub unsafe fn handleEvent(&mut self, event: u32, timer: *const Timer) {
        // SAFETY: The caller must ensure that the pointer is valid.
        let snapshot = unsafe { &*timer }.snapshot();
        if let Some(future) = self.inner.handle_event(Event::from(event), &snapshot) {
            spawn_local(future);
        }
    }
}
