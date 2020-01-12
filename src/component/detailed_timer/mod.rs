//! Provides the Detailed Timer Component and relevant types for using it. The
//! Detailed Timer Component is a component that shows two timers, one for the
//! total time of the current attempt and one showing the time of just the
//! current segment. Other information, like segment times of up to two
//! comparisons, the segment icon, and the segment's name, can also be shown.

use super::timer;
use crate::analysis::comparison_single_segment_time;
use crate::comparison::{self, best_segments, none};
use crate::platform::prelude::*;
use crate::settings::{CachedImageId, Field, Gradient, ImageData, SettingsDescription, Value};
use crate::timing::formatter::{Accuracy, DigitsFormat, SegmentTime, TimeFormatter};
use crate::{GeneralLayoutSettings, Segment, Timer, TimerPhase};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// The Detailed Timer Component is a component that shows two timers, one for
/// the total time of the current attempt and one showing the time of just the
/// current segment. Other information, like segment times of up to two
/// comparisons, the segment icon, and the segment's name, can also be shown.
#[derive(Default, Clone)]
pub struct Component {
    icon_id: CachedImageId,
    timer: timer::Component,
    segment_timer: timer::Component,
    settings: Settings,
}

/// The Settings for this component.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// The background shown behind the component.
    pub background: Gradient,
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
}

/// The state object describes the information to visualize for this component.
#[derive(Serialize, Deserialize)]
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
    /// The name of the segment. This may be `None` if it's not supposed to be
    /// visualized.
    pub segment_name: Option<String>,
    /// The segment's icon encoded as the raw file bytes. This value is only
    /// specified whenever the icon changes. If you explicitly want to query
    /// this value, remount the component. The buffer itself may be empty. This
    /// indicates that there is no icon.
    pub icon_change: Option<ImageData>,
}

/// The state object describing a comparison to visualize.
#[derive(Serialize, Deserialize)]
pub struct ComparisonState {
    /// The name of the comparison.
    pub name: String,
    /// The time to show for the comparison.
    pub time: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            background: Gradient::Transparent,
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
                color_override: Some((170.0 / 255.0, 170.0 / 255.0, 170.0 / 255.0, 1.0).into()),
                ..Default::default()
            },
            display_icon: false,
            show_segment_name: false,
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
        Self::with_settings(Default::default())
    }

    /// Creates a new Detailed Timer Component with the given settings.
    pub fn with_settings(settings: Settings) -> Self {
        let timer = timer::Component::with_settings(settings.timer.clone());
        let segment_timer = timer::Component::with_settings(settings.segment_timer.clone());
        Self {
            timer,
            segment_timer,
            settings,
            ..Default::default()
        }
    }

    /// Accesses the settings of the component.
    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Sets the settings of the component.
    pub fn set_settings(&mut self, settings: Settings) {
        self.settings = settings;
        *self.timer.settings_mut() = self.settings.timer.clone();
        *self.segment_timer.settings_mut() = self.settings.segment_timer.clone();
    }

    /// Accesses the name of the component.
    pub fn name(&self) -> &'static str {
        "Detailed Timer"
    }

    /// Calculates the component's state based on the timer and layout settings
    /// provided.
    pub fn state(&mut self, timer: &Timer, layout_settings: &GeneralLayoutSettings) -> State {
        let current_phase = timer.current_phase();
        let timing_method = self
            .settings
            .timer
            .timing_method
            .unwrap_or_else(|| timer.current_timing_method());

        let run = timer.run();

        let (current_split, last_split_index) = if current_phase == TimerPhase::Ended {
            (run.segments().last(), run.len() - 1)
        } else {
            (
                timer.current_split(),
                timer.current_split_index().unwrap_or(0),
            )
        };

        let (comparison1, comparison2) = if current_phase != TimerPhase::NotRunning {
            let mut comparison1 = self
                .settings
                .comparison1
                .as_ref()
                .map(String::as_str)
                .unwrap_or_else(|| timer.current_comparison());

            let comparison2 = self
                .settings
                .comparison2
                .as_ref()
                .map(String::as_str)
                .unwrap_or_else(|| timer.current_comparison());

            let mut hide_comparison = self.settings.hide_second_comparison;

            if hide_comparison
                || !run.comparisons().any(|c| c == comparison2)
                || comparison2 == none::NAME
            {
                hide_comparison = true;
                if !run.comparisons().any(|c| c == comparison1) || comparison1 == none::NAME {
                    comparison1 = timer.current_comparison();
                }
            } else if !run.comparisons().any(|c| c == comparison1) || comparison1 == none::NAME {
                hide_comparison = true;
                comparison1 = comparison2;
            } else if comparison1 == comparison2 {
                hide_comparison = true;
            }

            let comparison1 = Some((
                comparison::shorten(comparison1),
                comparison_single_segment_time(run, last_split_index, comparison1, timing_method),
            ));

            let comparison2 = if !hide_comparison {
                Some((
                    comparison::shorten(comparison2),
                    comparison_single_segment_time(
                        run,
                        last_split_index,
                        comparison2,
                        timing_method,
                    ),
                ))
            } else {
                None
            };

            (comparison1, comparison2)
        } else {
            Default::default()
        };

        let timer_state = self.timer.state(timer, layout_settings);
        let segment_timer_state = self.segment_timer.state(timer, layout_settings);

        let formatter = SegmentTime::new();

        let comparison1 = comparison1.map(|(name, time)| ComparisonState {
            name: name.to_string(),
            time: formatter.format(time).to_string(),
        });

        let comparison2 = comparison2.map(|(name, time)| ComparisonState {
            name: name.to_string(),
            time: formatter.format(time).to_string(),
        });

        let display_icon = self.settings.display_icon;
        let icon_change = self
            .icon_id
            .update_with(current_split.filter(|_| display_icon).map(Segment::icon))
            .map(Into::into);

        let segment_name = if self.settings.show_segment_name {
            current_split.map(|s| s.name().to_owned())
        } else {
            None
        };

        State {
            background: self.settings.background,
            timer: timer_state,
            segment_timer: segment_timer_state,
            comparison1,
            comparison2,
            segment_name,
            icon_change,
        }
    }

    /// Remounts the component as if it was freshly initialized. The segment
    /// icons shown by this component are only provided in the state objects
    /// whenever the icon changes or whenever the component's state is first
    /// queried. Remounting returns the segment icon again, whenever its state
    /// is queried the next time.
    pub fn remount(&mut self) {
        self.icon_id.reset();
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new("Background".into(), self.settings.background.into()),
            Field::new(
                "Timing Method".into(),
                self.settings.timer.timing_method.into(),
            ),
            Field::new(
                "Comparison 1".into(),
                self.settings.comparison1.clone().into(),
            ),
            Field::new(
                "Comparison 2".into(),
                self.settings.comparison2.clone().into(),
            ),
            Field::new(
                "Hide Second Comparison".into(),
                self.settings.hide_second_comparison.into(),
            ),
            Field::new(
                "Timer Height".into(),
                u64::from(self.settings.timer.height).into(),
            ),
            Field::new(
                "Segment Timer Height".into(),
                u64::from(self.settings.segment_timer.height).into(),
            ),
            Field::new(
                "Timer Digits Format".into(),
                self.settings.timer.digits_format.into(),
            ),
            Field::new("Timer Accuracy".into(), self.settings.timer.accuracy.into()),
            Field::new(
                "Segment Timer Digits Format".into(),
                self.settings.segment_timer.digits_format.into(),
            ),
            Field::new(
                "Segment Timer Accuracy".into(),
                self.settings.segment_timer.accuracy.into(),
            ),
            Field::new(
                "Show Segment Name".into(),
                self.settings.show_segment_name.into(),
            ),
            Field::new("Display Icon".into(), self.settings.display_icon.into()),
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
                let value: DigitsFormat = value.into();
                self.settings.timer.digits_format = value;
                self.timer.settings_mut().digits_format = value;
            }
            8 => {
                let value: Accuracy = value.into();
                self.settings.timer.accuracy = value;
                self.timer.settings_mut().accuracy = value;
            }
            9 => {
                let value: DigitsFormat = value.into();
                self.settings.segment_timer.digits_format = value;
                self.segment_timer.settings_mut().digits_format = value;
            }
            10 => {
                let value: Accuracy = value.into();
                self.settings.segment_timer.accuracy = value;
                self.segment_timer.settings_mut().accuracy = value;
            }
            11 => self.settings.show_segment_name = value.into(),
            12 => self.settings.display_icon = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
