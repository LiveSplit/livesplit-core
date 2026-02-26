//! Integration with [therun.gg](https://therun.gg/), a speedrunning statistics
//! and live tracking website.
//!
//! This module provides a [`Client`] that can be used to:
//! 1. Send live split data to therun.gg after every split action, enabling live
//!    run tracking.
//! 2. Upload the LiveSplit `.lss` file to therun.gg after every reset or
//!    finished run, automatically syncing runs with the site.
//!
//! This is a port of the original
//! [LiveSplit.TheRun](https://github.com/therungg/LiveSplit.TheRun) C#
//! component.

use core::{fmt, future::Future};

use reqwest::{
    Url,
    header::{ACCEPT, CONTENT_DISPOSITION, HeaderMap},
};
use serde_derive::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;

use crate::{
    Time, TimeSpan, TimerPhase, TimingMethod,
    event::Event,
    run::saver::{self, livesplit::IoWrite},
    timing::Snapshot,
    util::ordered_map::Map,
};

const SPLIT_WEBHOOK_URL: &str =
    "https://dspc6ekj2gjkfp44cjaffhjeue0fbswr.lambda-url.eu-west-1.on.aws/";
const FILE_UPLOAD_BASE_URL: &str =
    "https://2uxp372ks6nwrjnk6t7lqov4zu0solno.lambda-url.eu-west-1.on.aws/";

/// An error that can occur when communicating with therun.gg.
pub enum Error {
    /// An HTTP request failed.
    Reqwest(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Reqwest(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Reqwest(e) => fmt::Debug::fmt(e, f),
        }
    }
}

/// A client for communicating with therun.gg. It handles live split tracking
/// and stats uploading.
pub struct Client {
    client: reqwest::Client,
    upload_key: String,
    timer_paused: bool,
    current_paused_time: TimeSpan,
    time_paused_before_resume: TimeSpan,
    was_just_resumed: bool,
    is_live_tracking_enabled: bool,
    is_stats_uploading_enabled: bool,
}

impl Client {
    /// Creates a new therun.gg client with the given upload key and settings.
    /// The upload key is a 36-character key obtained from therun.gg.
    pub fn new(
        upload_key: String,
        is_live_tracking_enabled: bool,
        is_stats_uploading_enabled: bool,
    ) -> Self {
        let headers = HeaderMap::from_iter([
            (ACCEPT, "*/*".parse().unwrap()),
            (
                "Sec-Fetch-Site".parse().unwrap(),
                "cross-site".parse().unwrap(),
            ),
            (CONTENT_DISPOSITION, "attachment".parse().unwrap()),
        ]);

        Self {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
            upload_key,
            timer_paused: false,
            current_paused_time: TimeSpan::zero(),
            time_paused_before_resume: TimeSpan::zero(),
            was_just_resumed: false,
            is_live_tracking_enabled,
            is_stats_uploading_enabled,
        }
    }

    /// Sets whether live tracking is enabled. When enabled, live split data is
    /// sent to therun.gg after every split action.
    pub fn set_live_tracking_enabled(&mut self, enabled: bool) {
        self.is_live_tracking_enabled = enabled;
    }

    /// Sets whether stats uploading is enabled. When enabled, the `.lss` file
    /// is uploaded to therun.gg after every reset or finished run.
    pub fn set_stats_uploading_enabled(&mut self, enabled: bool) {
        self.is_stats_uploading_enabled = enabled;
    }

    /// Handles a timer event. Returns a future to execute if therun.gg needs to
    /// be notified, or [`None`] if no action is needed for this event.
    pub fn handle_event(
        &mut self,
        event: Event,
        timer: &Snapshot,
    ) -> Option<impl Future<Output = ()> + 'static> {
        let is_reset = match event {
            Event::Paused => {
                self.timer_paused = true;
                false
            }
            Event::Resumed | Event::PausesUndoneAndResumed => {
                let total_pause_time = timer.get_pause_time().unwrap_or_default();
                self.time_paused_before_resume = total_pause_time - self.current_paused_time;
                self.current_paused_time = total_pause_time;
                self.timer_paused = false;
                self.was_just_resumed = true;
                false
            }
            Event::Started
            | Event::Splitted
            | Event::Finished
            | Event::SplitSkipped
            | Event::SplitUndone
            | Event::PausesUndone => false,
            Event::Reset => true,
            Event::ComparisonChanged
            | Event::TimingMethodChanged
            | Event::GameTimeInitialized
            | Event::GameTimeSet
            | Event::GameTimePaused
            | Event::GameTimeResumed
            | Event::LoadingTimesSet
            | Event::CustomVariableSet
            | Event::Unknown => return None,
        };

        if !self.are_splits_valid(timer) {
            self.was_just_resumed = false;
            return None;
        }

        let is_live = self.is_live_tracking_enabled;

        if !is_reset && !is_live {
            self.was_just_resumed = false;
            return None;
        }

        let send_state = if is_live {
            Some(self.update_splits_state(timer))
        } else {
            None
        };

        let upload = if self.is_stats_uploading_enabled
            && (is_reset || timer.current_phase() == TimerPhase::Ended)
        {
            Some(self.upload_splits(timer))
        } else {
            None
        };

        self.was_just_resumed = false;

        Some(async move {
            if let Some(f) = send_state {
                let _ = f.await;
            }
            if let Some(f) = upload {
                let _ = f.await;
            }
        })
    }

    fn are_splits_valid(&self, timer: &Snapshot) -> bool {
        !timer.run().game_name().is_empty()
            && !timer.run().category_name().is_empty()
            && self.upload_key.len() == 36
    }

    fn update_splits_state(
        &self,
        timer: &Snapshot,
    ) -> impl Future<Output = Result<(), Error>> + use<> {
        let run = timer.run();

        let convert_time =
            |time: Time| Some(time[timer.current_timing_method()]?.total_milliseconds());

        let mut run_data = Vec::new();

        for segment in run.segments() {
            run_data.push(RunData {
                name: segment.name().to_owned(),
                split_time: convert_time(segment.split_time()),
                pb_split_time: convert_time(segment.personal_best_split_time()),
                best_possible: convert_time(segment.best_segment_time()),
                comparisons: run
                    .comparisons()
                    .map(|c| Comparison {
                        name: c.to_owned(),
                        time: convert_time(segment.comparison(c)),
                    })
                    .collect(),
            })
        }

        let format_date_time = |adt: crate::AtomicDateTime| -> String {
            adt.time.format(&Rfc3339).unwrap_or_default()
        };

        let live_data = LiveData {
            metadata: Metadata {
                game: run.game_name().to_owned(),
                category: run.category_name().to_owned(),
                platform: run.metadata().platform_name.clone(),
                region: run.metadata().region_name.clone(),
                emulator: run.metadata().uses_emulator,
                variables: run.metadata().speedrun_com_variables.clone(),
            },
            current_time: convert_time(timer.current_time()),
            current_split_name: timer
                .current_split()
                .map(|s| s.name().to_owned())
                .unwrap_or_default(),
            current_split_index: timer.current_split_index().map(|v| v as i64).unwrap_or(-1),
            timing_method: timer.current_timing_method(),
            current_duration: timer.current_attempt_duration().total_milliseconds(),
            start_time: timer.attempt_started().map(format_date_time),
            end_time: timer.attempt_ended().map(format_date_time),
            upload_key: self.upload_key.clone(),
            is_paused: self.timer_paused,
            is_game_time_paused: timer.is_game_time_paused(),
            game_time_pause_time: timer.game_time_paused_at().map(|v| v.total_milliseconds()),
            total_pause_time: timer.get_pause_time().map(|v| v.total_milliseconds()),
            current_pause_time: self.time_paused_before_resume.total_milliseconds(),
            time_paused_at: timer.time_paused_at().map(|v| v.total_milliseconds()),
            was_just_resumed: self.was_just_resumed,
            current_comparison: timer.current_comparison().to_owned(),
            run_data,
        };

        let request = self.client.post(SPLIT_WEBHOOK_URL).json(&live_data).send();

        async move {
            request
                .await
                .map_err(Error::Reqwest)?
                .error_for_status()
                .map_err(Error::Reqwest)?;

            Ok(())
        }
    }

    fn upload_splits(&self, timer: &Snapshot) -> impl Future<Output = Result<(), Error>> + use<> {
        let client = self.client.clone();

        let mut buf = Vec::new();
        saver::livesplit::save_timer(timer, IoWrite(&mut buf))
            .ok()
            .unwrap();

        let mut url = Url::parse(FILE_UPLOAD_BASE_URL).unwrap();

        let mut file_name = timer.run().extended_file_name(true);
        file_name.push_str(".lss");

        url.query_pairs_mut()
            .append_pair("filename", &file_name)
            .append_pair("uploadKey", &self.upload_key);

        let request = client.get(url).send();

        async move {
            let UrlResponse { url } = request
                .await
                .map_err(Error::Reqwest)?
                .error_for_status()
                .map_err(Error::Reqwest)?
                .json()
                .await
                .map_err(Error::Reqwest)?;

            client
                .put(url)
                .header(CONTENT_DISPOSITION, "attachment")
                .body(buf)
                .send()
                .await
                .map_err(Error::Reqwest)?
                .error_for_status()
                .map_err(Error::Reqwest)?;

            Ok(())
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    game: String,
    category: String,
    platform: String,
    region: String,
    emulator: bool,
    variables: Map<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Comparison {
    name: String,
    time: Option<f64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RunData {
    name: String,
    split_time: Option<f64>,
    pb_split_time: Option<f64>,
    best_possible: Option<f64>,
    comparisons: Vec<Comparison>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LiveData {
    metadata: Metadata,
    current_time: Option<f64>,
    current_split_name: String,
    current_split_index: i64,
    timing_method: TimingMethod,
    current_duration: f64,
    start_time: Option<String>,
    end_time: Option<String>,
    upload_key: String,
    is_paused: bool,
    is_game_time_paused: bool,
    game_time_pause_time: Option<f64>,
    total_pause_time: Option<f64>,
    current_pause_time: f64,
    time_paused_at: Option<f64>,
    was_just_resumed: bool,
    current_comparison: String,
    run_data: Vec<RunData>,
}

#[derive(Deserialize)]
struct UrlResponse {
    url: String,
}
