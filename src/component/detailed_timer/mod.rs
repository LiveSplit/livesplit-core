//! Provides the Detailed Timer Component and relevant types for using it. The
//! Detailed Timer Component is a component that shows two timers, one for the
//! total time of the current attempt and one showing the time of just the
//! current segment. Other information, like segment times of up to two
//! comparisons, the segment icon, and the segment's name, can also be shown.

use super::timer;
use crate::{
    GeneralLayoutSettings, TimeSpan, TimerPhase, TimingMethod,
    analysis::comparison_single_segment_time,
    comparison::{self, best_segments, none},
    localization::{Lang, Text},
    platform::prelude::*,
    settings::{
        Color, Field, FieldHint, Gradient, Image, ImageCache, ImageId, SettingsDescription, Value,
    },
    timing::{
        Snapshot,
        formatter::{Accuracy, DigitsFormat, SegmentTime, TimeFormatter},
    },
    util::ClearVec,
};
use alloc::borrow::Cow;
use core::{
    fmt::Write,
    hash::{Hash, Hasher},
};
use serde_derive::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// The Detailed Timer Component is a component that shows two timers, one for
/// the total time of the current attempt and one showing the time of just the
/// current segment. Other information, like segment times of up to two
/// comparisons, the segment icon, and the segment's name, can also be shown.
#[derive(Clone)]
pub struct Component {
    timer: timer::Component,
    segment_timer: timer::Component,
    settings: Settings,
}

impl Default for Component {
    fn default() -> Self {
        // Initialize both timers from the detailed timer's settings rather
        // than giving them the standalone timer's independent defaults.
        Self::new()
    }
}

/// The Settings for this component.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// The background shown behind the component.
    pub background: timer::DeltaGradient,
    /// The first comparison to show the segment time of. If it's not specified,
    /// the current comparison is used.
    pub comparison1: Option<String>,
    /// The first comparison to show the segment time of. If it's not specified,
    /// the current comparison is used, unless the first comparison is also
    /// `None`. This is not shown if the second comparison is hidden.
    pub comparison2: Option<String>,
    /// Specifies whether to only show a single comparison.
    pub hide_second_comparison: bool,
    /// The settings of the attempt timer.
    pub timer: timer::Settings,
    /// The settings of the segment timer.
    pub segment_timer: timer::Settings,
    /// Specifies whether the segment icon should be shown.
    pub display_icon: bool,
    /// Specifies whether the segment name should be shown.
    pub show_segment_name: bool,
    /// The color of the segment name if it's shown. If [`None`] is specified,
    /// the color is taken from the layout.
    pub segment_name_color: Option<Color>,
    /// The color of the comparison names if they are shown. If [`None`] is
    /// specified, the color is taken from the layout.
    pub comparison_names_color: Option<Color>,
    /// The color of the comparison times if they are shown. If [`None`] is
    /// specified, the color is taken from the layout.
    pub comparison_times_color: Option<Color>,
    /// The accuracy of the comparison times.
    pub comparison_times_accuracy: Accuracy,
}

/// The state object describes the information to visualize for this component.
#[derive(Default, Serialize, Deserialize)]
pub struct State {
    /// The background shown behind the component.
    pub background: Gradient,
    /// The state of the attempt timer.
    pub timer: timer::State,
    /// The state of the segment timer.
    pub segment_timer: timer::State,
    /// The first comparison to visualize.
    pub comparison1: Option<ComparisonState>,
    /// The second comparison to visualize.
    pub comparison2: Option<ComparisonState>,
    /// The configured comparison names and maximum time formats, corresponding
    /// to [`comparison1`](Self::comparison1) and [`comparison2`](Self::comparison2).
    /// An unconfigured comparison has no hint. Configured comparisons reserve
    /// space even while hidden before an attempt. Each time hint covers all of
    /// that comparison's segment times, keeping the lower row stable across splits.
    pub comparison_size_hints: [Option<ComparisonSizeHint>; 2],
    /// The name of the segment. This may be [`None`] if it's not supposed to be
    /// visualized.
    pub segment_name: Option<String>,
    /// The segment names used to reserve space for the widest name in the run.
    /// This keeps the attempt timer's size stable across segment changes. The
    /// renderer measures these names with the layout's text font. This is empty
    /// when segment names are hidden.
    pub segment_name_size_hints: ClearVec<String>,
    /// The shortest whole-time format the attempt timer can display, with
    /// normalized digits. Together with the fraction hint, this tells the
    /// renderer how much space a name could ever use at a stable timer size.
    /// Generated states borrow this fixed format without allocating a string.
    pub timer_minimum_size_hint: Cow<'static, str>,
    /// The icon of the segment. The associated image can be looked up in the
    /// image cache. The image may be the empty image. This indicates that there
    /// is no icon.
    pub icon: ImageId,
    /// The color of the segment name if it's shown. If [`None`] is specified,
    /// the color is taken from the layout.
    pub segment_name_color: Option<Color>,
    /// The color of the comparison names if they are shown. If [`None`] is
    /// specified, the color is taken from the layout.
    pub comparison_names_color: Option<Color>,
    /// The color of the comparison times if they are shown. If [`None`] is
    /// specified, the color is taken from the layout.
    pub comparison_times_color: Option<Color>,
}

/// The state object describing a comparison to visualize.
#[derive(Serialize, Deserialize)]
pub struct ComparisonState {
    /// The name of the comparison.
    pub name: String,
    /// The time to show for the comparison.
    pub time: String,
}

/// The space to reserve for a configured comparison, independently of whether
/// its current value is visible.
#[derive(Default, Clone, Hash, Serialize, Deserialize)]
pub struct ComparisonSizeHint {
    /// The comparison's displayed name, measured with the layout's text font.
    pub name: String,
    /// The largest known segment time, formatted with the comparison's
    /// accuracy and normalized digits. This is a dash if no times are known.
    /// The renderer also reserves the missing-time glyph alongside this hint.
    pub time: String,
}

impl State {
    pub(crate) fn content_fingerprint(&self, state: &mut impl Hasher) {
        self.timer.content_fingerprint(state);
        self.segment_timer.content_fingerprint(state);
        comparison_fingerprint(state, &self.comparison1);
        comparison_fingerprint(state, &self.comparison2);
        self.comparison_size_hints.hash(state);
        self.segment_name.hash(state);
        self.segment_name_size_hints.hash(state);
        self.timer_minimum_size_hint.hash(state);
        self.icon.hash(state);
    }

    pub(crate) const fn updates_frequently(&self) -> bool {
        self.timer.updates_frequently() || self.segment_timer.updates_frequently()
    }
}

impl ComparisonState {
    pub(crate) fn content_fingerprint(&self, state: &mut impl Hasher) {
        self.name.hash(state);
        self.time.hash(state);
    }
}

fn comparison_fingerprint(state: &mut impl Hasher, comparison: &Option<ComparisonState>) {
    comparison.is_some().hash(state);
    if let Some(comparison) = comparison {
        comparison.content_fingerprint(state);
    }
}

fn update_comparison(
    state: &mut Option<ComparisonState>,
    new_state: Option<(&str, Option<TimeSpan>)>,
    accuracy: Accuracy,
    lang: Lang,
) {
    if let Some((name, time)) = new_state {
        let state = state.get_or_insert_with(|| ComparisonState {
            name: String::new(),
            time: String::new(),
        });

        state.name.clear();
        state.name.push_str(name);

        state.time.clear();
        let _ = write!(
            state.time,
            "{}",
            SegmentTime::with_accuracy(accuracy).format(time, lang)
        );
    } else {
        *state = None;
    }
}

fn write_comparison_size_hint(
    output: &mut String,
    timer: &Snapshot,
    comparison: &str,
    method: TimingMethod,
    accuracy: Accuracy,
    lang: Lang,
) {
    // Comparison values are already known and do not grow as the timer runs.
    // Their exact maximum is sufficient; only live timers need a buffer.
    let mut has_negative = false;
    let largest = (0..timer.run().len())
        .filter_map(|index| comparison_single_segment_time(timer.run(), index, comparison, method))
        .map(|time| {
            has_negative |= time < TimeSpan::zero();
            TimeSpan::from(time.to_duration().abs())
        })
        .max();
    // Reserve a minus sign for the whole run if any segment needs one. Neither
    // the digits nor the sign in this hint depend on the current segment.
    let largest = largest.map(|time| if has_negative { -time } else { time });
    let _ = write!(
        timer::SizeHintWriter(output),
        "{}",
        SegmentTime::with_accuracy(accuracy).format(largest, lang)
    );
}

const SEGMENT_TIMER_DEFAULT_COLOR: Color =
    Color::rgba(170.0 / 255.0, 170.0 / 255.0, 170.0 / 255.0, 1.0);

impl Default for Settings {
    fn default() -> Self {
        Settings {
            background: timer::DeltaGradient::from(Gradient::Transparent),
            comparison1: None,
            comparison2: Some(String::from(best_segments::NAME)),
            hide_second_comparison: false,
            timer: timer::Settings {
                height: 40,
                ..Default::default()
            },
            segment_timer: timer::Settings {
                height: 25,
                is_segment_timer: true,
                color_override: Some(SEGMENT_TIMER_DEFAULT_COLOR),
                ..Default::default()
            },
            display_icon: false,
            show_segment_name: false,
            segment_name_color: None,
            comparison_names_color: None,
            comparison_times_color: None,
            comparison_times_accuracy: SegmentTime::DEFAULT_ACCURACY,
        }
    }
}

#[cfg(feature = "std")]
impl State {
    /// Encodes the state object's information as JSON.
    pub fn write_json<W>(&self, writer: W) -> serde_json::Result<()>
    where
        W: std::io::Write,
    {
        serde_json::to_writer(writer, self)
    }
}

impl Component {
    /// Creates a new Detailed Timer Component.
    pub fn new() -> Self {
        Self::with_settings(Settings::default())
    }

    /// Creates a new Detailed Timer Component with the given settings.
    pub fn with_settings(settings: Settings) -> Self {
        let timer = timer::Component::with_settings(settings.timer.clone());
        let segment_timer = timer::Component::with_settings(settings.segment_timer.clone());
        Self {
            timer,
            segment_timer,
            settings,
        }
    }

    /// Accesses the settings of the component.
    pub const fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Sets the settings of the component.
    pub fn set_settings(&mut self, settings: Settings) {
        self.settings = settings;
        *self.timer.settings_mut() = self.settings.timer.clone();
        *self.segment_timer.settings_mut() = self.settings.segment_timer.clone();
    }

    /// Resolve the displayed comparisons before considering their visibility.
    /// Keeping this selection identical before and during an attempt lets the
    /// renderer reserve their names, columns, and line count from the outset.
    fn resolve_comparisons<'a>(&'a self, timer: &'a Snapshot) -> [Option<&'a str>; 2] {
        let run = timer.run();
        let mut first = self
            .settings
            .comparison1
            .as_deref()
            .unwrap_or_else(|| timer.current_comparison());
        let second = self
            .settings
            .comparison2
            .as_deref()
            .unwrap_or_else(|| timer.current_comparison());
        let mut hide_second = self.settings.hide_second_comparison;

        if hide_second || !run.comparisons().any(|c| c == second) || second == none::NAME {
            hide_second = true;
            if !run.comparisons().any(|c| c == first) || first == none::NAME {
                first = timer.current_comparison();
            }
        } else if !run.comparisons().any(|c| c == first) || first == none::NAME {
            hide_second = true;
            first = second;
        } else if first == second {
            hide_second = true;
        }
        [Some(first), (!hide_second).then_some(second)]
    }

    fn update_comparisons(&self, state: &mut State, timer: &Snapshot, index: usize, lang: Lang) {
        let method = self
            .settings
            .timer
            .timing_method
            .unwrap_or_else(|| timer.current_timing_method());
        let accuracy = self.settings.comparison_times_accuracy;
        // Reuse each configured slot's strings independently of visibility.
        for ((comparison, visible), hint) in self
            .resolve_comparisons(timer)
            .into_iter()
            .zip([&mut state.comparison1, &mut state.comparison2])
            .zip(&mut state.comparison_size_hints)
        {
            if let Some(comparison) = comparison {
                let hint = hint.get_or_insert_with(ComparisonSizeHint::default);
                hint.name.clear();
                hint.name.push_str(comparison::shorten(comparison));
                hint.time.clear();
                write_comparison_size_hint(
                    &mut hint.time,
                    timer,
                    comparison,
                    method,
                    accuracy,
                    lang,
                );
            } else {
                *hint = None;
            }
            let visible_comparison = comparison
                .filter(|_| timer.current_phase() != TimerPhase::NotRunning)
                .map(|comparison| {
                    (
                        comparison::shorten(comparison),
                        comparison_single_segment_time(timer.run(), index, comparison, method),
                    )
                });
            update_comparison(visible, visible_comparison, accuracy, lang);
        }
    }

    /// Accesses the name of the component for the specified language.
    pub const fn name(&self, lang: Lang) -> &'static str {
        Text::ComponentDetailedTimer.resolve(lang)
    }

    /// Updates the component's state based on the timer and layout settings
    /// provided. The [`ImageCache`] is updated with all the images that are
    /// part of the state. The images are marked as visited in the
    /// [`ImageCache`]. You still need to manually run [`ImageCache::collect`]
    /// to ensure unused images are removed from the cache.
    pub fn update_state(
        &self,
        state: &mut State,
        image_cache: &mut ImageCache,
        timer: &Snapshot,
        layout_settings: &GeneralLayoutSettings,
        lang: Lang,
    ) {
        let current_phase = timer.current_phase();
        let run = timer.run();

        let (current_split, last_split_index) = if current_phase == TimerPhase::Ended {
            (run.segments().last(), run.len() - 1)
        } else {
            (
                timer.current_split(),
                timer.current_split_index().unwrap_or(0),
            )
        };

        let icon = current_split
            .filter(|_| self.settings.display_icon)
            .map(|s| s.icon())
            .unwrap_or(Image::EMPTY);
        state.icon = *image_cache.cache(icon.id(), || icon.clone()).id();

        self.timer
            .update_state(&mut state.timer, timer, layout_settings, lang);

        state.timer_minimum_size_hint = Cow::Borrowed(match self.settings.timer.digits_format {
            DigitsFormat::SingleDigitSeconds => "8",
            DigitsFormat::DoubleDigitSeconds => "88",
            DigitsFormat::SingleDigitMinutes => "8:88",
            DigitsFormat::DoubleDigitMinutes => "88:88",
            DigitsFormat::SingleDigitHours => "8:88:88",
            DigitsFormat::DoubleDigitHours => "88:88:88",
        });

        self.segment_timer
            .update_state(&mut state.segment_timer, timer, layout_settings, lang);

        state.background = self
            .settings
            .background
            .gradient(state.timer.semantic_color.visualize(layout_settings));

        self.update_comparisons(state, timer, last_split_index, lang);

        match current_split.filter(|_| self.settings.show_segment_name) {
            Some(segment) => {
                let segment_name = state.segment_name.get_or_insert_with(String::new);
                segment_name.clear();
                segment_name.push_str(segment.name());
            }
            None => state.segment_name = None,
        }

        state.segment_name_size_hints.clear();
        if self.settings.show_segment_name {
            for segment in run.segments() {
                state
                    .segment_name_size_hints
                    .push()
                    .push_str(segment.name());
            }
        }

        state.segment_name_color = self.settings.segment_name_color;
        state.comparison_names_color = self.settings.comparison_names_color;
        state.comparison_times_color = self.settings.comparison_times_color;
    }

    /// Calculates the component's state based on the timer and layout settings
    /// provided. The [`ImageCache`] is updated with all the images that are
    /// part of the state. The images are marked as visited in the
    /// [`ImageCache`]. You still need to manually run [`ImageCache::collect`]
    /// to ensure unused images are removed from the cache.
    pub fn state(
        &self,
        image_cache: &mut ImageCache,
        timer: &Snapshot,
        layout_settings: &GeneralLayoutSettings,
        lang: Lang,
    ) -> State {
        let mut state = Default::default();
        self.update_state(&mut state, image_cache, timer, layout_settings, lang);
        state
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values for the specified language.
    pub fn settings_description(&self, lang: Lang) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                Text::DetailedTimerBackground.resolve(lang).into(),
                Text::DetailedTimerBackgroundDescription
                    .resolve(lang)
                    .into(),
                self.settings.background.into(),
            ),
            Field::new(
                Text::DetailedTimerTimingMethod.resolve(lang).into(),
                Text::DetailedTimerTimingMethodDescription
                    .resolve(lang)
                    .into(),
                self.settings.timer.timing_method.into(),
            ),
            Field::new(
                Text::DetailedTimerComparison1.resolve(lang).into(),
                Text::DetailedTimerComparison1Description
                    .resolve(lang)
                    .into(),
                self.settings.comparison1.clone().into(),
            )
            .with_hint(FieldHint::Comparison),
            Field::new(
                Text::DetailedTimerComparison2.resolve(lang).into(),
                Text::DetailedTimerComparison2Description
                    .resolve(lang)
                    .into(),
                self.settings.comparison2.clone().into(),
            )
            .with_hint(FieldHint::Comparison),
            Field::new(
                Text::DetailedTimerHideSecondComparison.resolve(lang).into(),
                Text::DetailedTimerHideSecondComparisonDescription
                    .resolve(lang)
                    .into(),
                self.settings.hide_second_comparison.into(),
            ),
            Field::new(
                Text::DetailedTimerTimerHeight.resolve(lang).into(),
                Text::DetailedTimerTimerHeightDescription
                    .resolve(lang)
                    .into(),
                u64::from(self.settings.timer.height).into(),
            ),
            Field::new(
                Text::DetailedTimerSegmentTimerHeight.resolve(lang).into(),
                Text::DetailedTimerSegmentTimerHeightDescription
                    .resolve(lang)
                    .into(),
                u64::from(self.settings.segment_timer.height).into(),
            ),
            Field::new(
                Text::DetailedTimerTimerColor.resolve(lang).into(),
                Text::DetailedTimerTimerColorDescription
                    .resolve(lang)
                    .into(),
                self.settings.timer.color_override.into(),
            ),
            Field::new(
                Text::DetailedTimerShowTimerGradient.resolve(lang).into(),
                Text::DetailedTimerShowTimerGradientDescription
                    .resolve(lang)
                    .into(),
                self.settings.timer.show_gradient.into(),
            ),
            Field::new(
                Text::DetailedTimerTimerDigitsFormat.resolve(lang).into(),
                Text::DetailedTimerTimerDigitsFormatDescription
                    .resolve(lang)
                    .into(),
                self.settings.timer.digits_format.into(),
            ),
            Field::new(
                Text::DetailedTimerTimerAccuracy.resolve(lang).into(),
                Text::DetailedTimerTimerAccuracyDescription
                    .resolve(lang)
                    .into(),
                self.settings.timer.accuracy.into(),
            ),
            Field::new(
                Text::DetailedTimerSegmentTimerColor.resolve(lang).into(),
                Text::DetailedTimerSegmentTimerColorDescription
                    .resolve(lang)
                    .into(),
                self.settings
                    .segment_timer
                    .color_override
                    .unwrap_or(SEGMENT_TIMER_DEFAULT_COLOR)
                    .into(),
            ),
            Field::new(
                Text::DetailedTimerShowSegmentTimerGradient
                    .resolve(lang)
                    .into(),
                Text::DetailedTimerShowSegmentTimerGradientDescription
                    .resolve(lang)
                    .into(),
                self.settings.segment_timer.show_gradient.into(),
            ),
            Field::new(
                Text::DetailedTimerSegmentTimerDigitsFormat
                    .resolve(lang)
                    .into(),
                Text::DetailedTimerSegmentTimerDigitsFormatDescription
                    .resolve(lang)
                    .into(),
                self.settings.segment_timer.digits_format.into(),
            ),
            Field::new(
                Text::DetailedTimerSegmentTimerAccuracy.resolve(lang).into(),
                Text::DetailedTimerSegmentTimerAccuracyDescription
                    .resolve(lang)
                    .into(),
                self.settings.segment_timer.accuracy.into(),
            ),
            Field::new(
                Text::DetailedTimerComparisonNamesColor.resolve(lang).into(),
                Text::DetailedTimerComparisonNamesColorDescription
                    .resolve(lang)
                    .into(),
                self.settings.comparison_names_color.into(),
            ),
            Field::new(
                Text::DetailedTimerComparisonTimesColor.resolve(lang).into(),
                Text::DetailedTimerComparisonTimesColorDescription
                    .resolve(lang)
                    .into(),
                self.settings.comparison_times_color.into(),
            ),
            Field::new(
                Text::DetailedTimerComparisonTimesAccuracy
                    .resolve(lang)
                    .into(),
                Text::DetailedTimerComparisonTimesAccuracyDescription
                    .resolve(lang)
                    .into(),
                self.settings.comparison_times_accuracy.into(),
            ),
            Field::new(
                Text::DetailedTimerShowSegmentName.resolve(lang).into(),
                Text::DetailedTimerShowSegmentNameDescription
                    .resolve(lang)
                    .into(),
                self.settings.show_segment_name.into(),
            ),
            Field::new(
                Text::DetailedTimerSegmentNameColor.resolve(lang).into(),
                Text::DetailedTimerSegmentNameColorDescription
                    .resolve(lang)
                    .into(),
                self.settings.segment_name_color.into(),
            ),
            Field::new(
                Text::DetailedTimerDisplayIcon.resolve(lang).into(),
                Text::DetailedTimerDisplayIconDescription
                    .resolve(lang)
                    .into(),
                self.settings.display_icon.into(),
            ),
        ])
    }

    /// Sets a setting's value by its index to the given value.
    ///
    /// # Panics
    ///
    /// This panics if the type of the value to be set is not compatible with
    /// the type of the setting's value. A panic can also occur if the index of
    /// the setting provided is out of bounds.
    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.background = value.into(),
            1 => {
                let value = value.into();
                self.settings.timer.timing_method = value;
                self.settings.segment_timer.timing_method = value;
            }
            2 => self.settings.comparison1 = value.into(),
            3 => self.settings.comparison2 = value.into(),
            4 => self.settings.hide_second_comparison = value.into(),
            5 => {
                let value = value.into_uint().unwrap() as _;
                self.settings.timer.height = value;
                self.timer.settings_mut().height = value;
            }
            6 => {
                let value = value.into_uint().unwrap() as _;
                self.settings.segment_timer.height = value;
                self.segment_timer.settings_mut().height = value;
            }
            7 => {
                let value: Option<Color> = value.into();
                self.settings.timer.color_override = value;
                self.timer.settings_mut().color_override = value;
            }
            8 => {
                let value: bool = value.into();
                self.settings.timer.show_gradient = value;
                self.timer.settings_mut().show_gradient = value;
            }
            9 => {
                let value: DigitsFormat = value.into();
                self.settings.timer.digits_format = value;
                self.timer.settings_mut().digits_format = value;
            }
            10 => {
                let value: Accuracy = value.into();
                self.settings.timer.accuracy = value;
                self.timer.settings_mut().accuracy = value;
            }
            11 => {
                let value: Color = value.into();
                self.settings.segment_timer.color_override = Some(value);
                self.segment_timer.settings_mut().color_override = Some(value);
            }
            12 => {
                let value: bool = value.into();
                self.settings.segment_timer.show_gradient = value;
                self.segment_timer.settings_mut().show_gradient = value;
            }
            13 => {
                let value: DigitsFormat = value.into();
                self.settings.segment_timer.digits_format = value;
                self.segment_timer.settings_mut().digits_format = value;
            }
            14 => {
                let value: Accuracy = value.into();
                self.settings.segment_timer.accuracy = value;
                self.segment_timer.settings_mut().accuracy = value;
            }
            15 => self.settings.comparison_names_color = value.into(),
            16 => self.settings.comparison_times_color = value.into(),
            17 => self.settings.comparison_times_accuracy = value.into(),
            18 => self.settings.show_segment_name = value.into(),
            19 => self.settings.segment_name_color = value.into(),
            20 => self.settings.display_icon = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
