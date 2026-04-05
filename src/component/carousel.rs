//! Provides the Carousel Component and relevant types for using it. A Carousel
//! Component contains multiple child components and periodically switches
//! between showing each one, displaying only one child at a time.
//!
//! The carousel uses an adaptive algorithm that detects meaningful content
//! changes in child components. When a child's content changes while it is not
//! being shown, the carousel prioritizes switching to it. Components that
//! update every frame (like timers) are excluded from change-based priority so
//! they don't dominate the carousel. A configurable interval acts as a fallback
//! for rotating through children when no priority changes are detected.

use core::hash::{Hash, Hasher};

use crate::{
    layout,
    localization::{Lang, Text},
    platform::{Duration, Instant, prelude::*},
    settings::{Field, ImageCache, SettingsDescription, Value},
    timing::Snapshot,
};
use serde_derive::{Deserialize, Serialize};

/// The default interval in seconds between switching to the next child when no
/// priority changes are detected.
const DEFAULT_INTERVAL_SECONDS: u64 = 8;

/// Default minimum number of seconds a child must be displayed before the
/// carousel considers switching to another child.
const DEFAULT_MIN_DISPLAY_SECONDS: u64 = 2;

/// A Carousel Component cycles through its child components one at a time.
/// It uses an adaptive algorithm that prioritizes showing children whose
/// content has meaningfully changed. A configurable interval serves as a
/// fallback for rotating when no changes are detected.
#[derive(Clone)]
pub struct Component {
    /// The components contained in this carousel.
    pub components: Vec<layout::Component>,
    /// An optional size override. In horizontal mode this sets the height, in
    /// vertical mode it sets the width. [`None`] means the size is determined
    /// automatically from the currently visible child.
    pub size: Option<u32>,
    /// The maximum interval in seconds between switches when no content changes
    /// are detected.
    pub interval_seconds: u64,
    /// The minimum interval in seconds a child needs to stay visible before the
    /// carousel considers switching away from it.
    pub min_display_seconds: u64,
    /// The index of the currently visible child.
    current_index: usize,
    /// The instant when the last switch occurred.
    last_switch: Instant,
    /// Content fingerprint of each child's state at the time it was last
    /// hidden. [`None`] means the child has not been seen yet and its baseline
    /// will be initialized on the next update.
    baseline_fingerprints: Vec<Option<u64>>,
    /// When each child was last the actively shown child.
    last_shown: Vec<Instant>,
}

impl Default for Component {
    fn default() -> Self {
        Self::new()
    }
}

/// The state object for a carousel component, containing the states of all
/// children and indicating which one is currently visible.
#[derive(Default, Serialize, Deserialize)]
pub struct State {
    /// An optional size override. In horizontal mode this sets the height, in
    /// vertical mode it sets the width. [`None`] means automatic sizing.
    pub size: Option<u32>,
    /// The state objects for all components in this carousel.
    pub components: Vec<layout::ComponentState>,
    /// The index of the currently visible component.
    pub current_index: usize,
}

impl State {
    pub(crate) fn has_same_content(&self, other: &Self) -> bool {
        self.current_index == other.current_index
            && self.components.len() == other.components.len()
            && self
                .components
                .iter()
                .zip(other.components.iter())
                .all(|(a, b)| a.has_same_content(b))
    }

    pub(crate) fn content_fingerprint(&self, state: &mut impl Hasher) {
        self.current_index.hash(state);
        self.components.len().hash(state);
        for component in &self.components {
            component.content_fingerprint().hash(state);
        }
    }
}

/// The serializable settings for a carousel component.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// An optional size override. In horizontal mode this sets the height, in
    /// vertical mode it sets the width. [`None`] means automatic sizing.
    pub size: Option<u32>,
    /// The interval in seconds between switching to the next child component.
    pub interval_seconds: u64,
    /// The minimum interval in seconds a child needs to stay visible before the
    /// carousel considers switching away from it.
    pub min_display_seconds: u64,
    /// The settings for each component in the carousel.
    pub components: Vec<layout::ComponentSettings>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            size: None,
            interval_seconds: DEFAULT_INTERVAL_SECONDS,
            min_display_seconds: DEFAULT_MIN_DISPLAY_SECONDS,
            components: Vec::new(),
        }
    }
}

impl Component {
    /// Creates a new empty Carousel Component.
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            size: None,
            interval_seconds: DEFAULT_INTERVAL_SECONDS,
            min_display_seconds: DEFAULT_MIN_DISPLAY_SECONDS,
            current_index: 0,
            last_switch: Instant::now(),
            baseline_fingerprints: Vec::new(),
            last_shown: Vec::new(),
        }
    }

    /// Creates a new Carousel Component with the given settings.
    pub fn with_settings(settings: Settings) -> Self {
        Self {
            size: settings.size,
            interval_seconds: settings.interval_seconds,
            min_display_seconds: settings.min_display_seconds,
            components: settings.components.into_iter().map(Into::into).collect(),
            current_index: 0,
            last_switch: Instant::now(),
            baseline_fingerprints: Vec::new(),
            last_shown: Vec::new(),
        }
    }

    /// Accesses the name of the component for the specified language.
    pub const fn name(&self, lang: Lang) -> &'static str {
        Text::Carousel.resolve(lang)
    }

    fn interval(&self) -> Duration {
        Duration::seconds(i64::try_from(self.interval_seconds).unwrap_or(i64::MAX))
    }

    fn min_display_time(&self) -> Duration {
        Duration::seconds(i64::try_from(self.min_display_seconds).unwrap_or(i64::MAX))
    }

    /// Ensures the per-child tracking vectors are correctly sized for the
    /// current number of children.
    fn ensure_tracking(&mut self, len: usize, now: Instant) {
        while self.baseline_fingerprints.len() < len {
            self.baseline_fingerprints.push(None);
        }
        while self.last_shown.len() < len {
            self.last_shown.push(now);
        }
        self.baseline_fingerprints.truncate(len);
        self.last_shown.truncate(len);
        if self.current_index >= len && len > 0 {
            self.current_index = 0;
            self.last_switch = now;
        }
    }

    /// Switches the carousel to show the child at `new_idx`. Stores the
    /// outgoing child's fingerprint as its baseline and updates timestamps.
    fn switch_to(&mut self, new_idx: usize, states: &[layout::ComponentState], now: Instant) {
        // Record the outgoing child's current state as its baseline.
        if self.current_index < states.len() {
            self.baseline_fingerprints[self.current_index] =
                Some(states[self.current_index].content_fingerprint());
        }
        self.current_index = new_idx;
        self.last_switch = now;
        self.last_shown[new_idx] = now;
        // Mark the incoming child's baseline as its current state so it
        // starts out as "unchanged".
        self.baseline_fingerprints[new_idx] = Some(states[new_idx].content_fingerprint());
    }

    /// Decides which child to show based on the just-computed component states.
    /// Prioritizes children whose meaningful content changed since they were
    /// last hidden, preferring the one that hasn't been shown the longest.
    /// Falls back to timed rotation when no priority changes are detected.
    fn choose_child(&mut self, states: &[layout::ComponentState]) -> usize {
        let len = states.len();
        if len == 0 {
            self.current_index = 0;
            return 0;
        }

        let now = Instant::now();
        self.ensure_tracking(len, now);

        // Initialize baselines for any children we haven't seen yet.
        for (i, state) in states.iter().enumerate() {
            if self.baseline_fingerprints[i].is_none() {
                self.baseline_fingerprints[i] = Some(state.content_fingerprint());
            }
        }

        let elapsed = now - self.last_switch;

        // Don't switch if we haven't shown the current child long enough.
        if elapsed < self.min_display_time() {
            return self.current_index.min(len - 1);
        }

        // Find candidate children that have changed since their baseline and
        // don't continuously update (which would always appear "changed").
        // Among candidates, pick the one that hasn't been shown the longest.
        let mut best_candidate: Option<(usize, Instant)> = None;
        for (i, state) in states.iter().enumerate() {
            if i == self.current_index {
                continue;
            }
            let fp = state.content_fingerprint();
            if let Some(baseline) = self.baseline_fingerprints[i]
                && fp != baseline
                && !state.updates_frequently()
            {
                let last = self.last_shown[i];
                if best_candidate.is_none_or(|(_, t)| last < t) {
                    best_candidate = Some((i, last));
                }
            }
        }

        if let Some((idx, _)) = best_candidate {
            self.switch_to(idx, states, now);
            return idx;
        }

        // No priority candidates. Fall back to timed rotation.
        if elapsed >= self.interval() {
            let next = (self.current_index + 1) % len;
            self.switch_to(next, states, now);
            return next;
        }

        self.current_index.min(len - 1)
    }

    /// Calculates the carousel's state.
    pub fn state(
        &mut self,
        image_cache: &mut ImageCache,
        timer: &Snapshot,
        layout_settings: &layout::GeneralSettings,
        lang: Lang,
    ) -> State {
        // Compute all children states first.
        let components: Vec<_> = self
            .components
            .iter_mut()
            .map(|c| c.state(image_cache, timer, layout_settings, lang))
            .collect();

        // Decide which child to show based on the computed states.
        let index = self.choose_child(&components);

        State {
            size: self.size,
            components,
            current_index: index,
        }
    }

    /// Updates the carousel's state in place.
    pub fn update_state(
        &mut self,
        state: &mut State,
        image_cache: &mut ImageCache,
        timer: &Snapshot,
        layout_settings: &layout::GeneralSettings,
        lang: Lang,
    ) {
        state.size = self.size;

        // Update all children states first.
        state.components.truncate(self.components.len());
        let mut components = self.components.iter_mut();
        for (state, component) in state.components.iter_mut().zip(components.by_ref()) {
            component.update_state(state, image_cache, timer, layout_settings, lang);
        }
        state
            .components
            .extend(components.map(|c| c.state(image_cache, timer, layout_settings, lang)));

        // Decide which child to show based on the updated states.
        state.current_index = self.choose_child(&state.components);
    }

    /// Returns the settings for this carousel.
    pub fn settings(&self) -> Settings {
        Settings {
            size: self.size,
            interval_seconds: self.interval_seconds,
            min_display_seconds: self.min_display_seconds,
            components: self
                .components
                .iter()
                .map(layout::Component::settings)
                .collect(),
        }
    }

    /// Returns a settings description for this carousel.
    pub fn settings_description(&self, lang: Lang) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                Text::CarouselFixedSize.resolve(lang).into(),
                Text::CarouselFixedSizeDescription.resolve(lang).into(),
                self.size.map(|s| s as u64).into(),
            ),
            Field::new(
                Text::CarouselInterval.resolve(lang).into(),
                Text::CarouselIntervalDescription.resolve(lang).into(),
                (self.interval_seconds).into(),
            ),
            Field::new(
                Text::CarouselMinDisplay.resolve(lang).into(),
                Text::CarouselMinDisplayDescription.resolve(lang).into(),
                (self.min_display_seconds).into(),
            ),
        ])
    }

    /// Sets a setting value by index.
    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => {
                self.size =
                    Option::<u64>::from(value).map(|v| u32::try_from(v).unwrap_or(u32::MAX));
            }
            1 => {
                let v = u64::from(value);
                self.interval_seconds = if v > 0 { v } else { DEFAULT_INTERVAL_SECONDS };
            }
            2 => {
                let v = u64::from(value);
                self.min_display_seconds = if v > 0 {
                    v
                } else {
                    DEFAULT_MIN_DISPLAY_SECONDS
                };
            }
            _ => panic!("Unsupported setting index"),
        }
    }

    /// Tells each component in the carousel to scroll up.
    pub fn scroll_up(&mut self) {
        for c in &mut self.components {
            c.scroll_up();
        }
    }

    /// Tells each component in the carousel to scroll down.
    pub fn scroll_down(&mut self) {
        for c in &mut self.components {
            c.scroll_down();
        }
    }
}
