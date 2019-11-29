//! Provides the Previous Segment Component and relevant types for using it. The
//! Previous Segment Component is a component that shows how much time was saved
//! or lost during the previous segment based on the chosen comparison.
//! Additionally, the potential time save for the previous segment can be
//! displayed. This component switches to a `Live Segment` view that shows
//! active time loss whenever the runner is losing time on the current segment.

use super::key_value;
use crate::platform::prelude::*;
use crate::settings::{Color, Field, Gradient, SemanticColor, SettingsDescription, Value};
use crate::timing::formatter::{Accuracy, Delta, SegmentTime, TimeFormatter};
use crate::{analysis, comparison, GeneralLayoutSettings, Timer, TimerPhase};
use alloc::borrow::Cow;
use core::fmt::Write as FmtWrite;
use serde::{Deserialize, Serialize};

/// The Previous Segment Component is a component that shows how much time was
/// saved or lost during the previous segment based on the chosen comparison.
/// Additionally, the potential time save for the previous segment can be
/// displayed. This component switches to a `Live Segment` view that shows
/// active time loss whenever the runner is losing time on the current segment.
#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
}

/// The Settings for this component.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// The background shown behind the component.
    pub background: Gradient,
    /// The comparison chosen. Uses the Timer's current comparison if set to
    /// `None`.
    pub comparison_override: Option<String>,
    /// Specifies whether to display the name of the component and its value in
    /// two separate rows.
    pub display_two_rows: bool,
    /// The color of the label. If `None` is specified, the color is taken from
    /// the layout.
    pub label_color: Option<Color>,
    /// Specifies if the decimals should not be shown anymore when the
    /// visualized delta is above one minute.
    pub drop_decimals: bool,
    /// The accuracy of the time shown.
    pub accuracy: Accuracy,
    /// Determines if the time save that could've been saved is shown in
    /// addition to the previous segment.
    pub show_possible_time_save: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: key_value::DEFAULT_GRADIENT,
            comparison_override: None,
            display_two_rows: false,
            label_color: None,
            drop_decimals: true,
            accuracy: Accuracy::Tenths,
            show_possible_time_save: false,
        }
    }
}

impl Component {
    /// Creates a new Previous Segment Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Previous Segment Component with the given settings.
    pub fn with_settings(settings: Settings) -> Self {
        Self { settings }
    }

    /// Accesses the settings of the component.
    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Grants mutable access to the settings of the component.
    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    /// Accesses the name of the component.
    pub fn name(&self) -> Cow<'static, str> {
        self.text(
            false,
            self.settings
                .comparison_override
                .as_ref()
                .map(String::as_ref),
        )
    }

    fn text(&self, live: bool, comparison: Option<&str>) -> Cow<'static, str> {
        let text = if live {
            "Live Segment"
        } else {
            "Previous Segment"
        };
        let mut text = Cow::from(text);
        if let Some(comparison) = comparison {
            write!(text.to_mut(), " ({})", comparison::shorten(comparison)).unwrap();
        }
        text
    }

    /// Calculates the component's state based on the timer and the layout
    /// settings provided.
    pub fn state(
        &self,
        timer: &Timer,
        layout_settings: &GeneralLayoutSettings,
    ) -> key_value::State {
        let mut time_change = None;
        let mut previous_possible = None;
        let resolved_comparison = comparison::resolve(&self.settings.comparison_override, timer);
        let comparison = comparison::or_current(resolved_comparison, timer);
        let live_segment =
            analysis::check_live_delta(timer, false, comparison, timer.current_timing_method());

        let phase = timer.current_phase();
        let method = timer.current_timing_method();
        let semantic_color = if phase != TimerPhase::NotRunning {
            let split_index = timer.current_split_index().unwrap();
            if live_segment.is_some() {
                time_change = analysis::live_segment_delta(timer, split_index, comparison, method);
                if self.settings.show_possible_time_save {
                    previous_possible = analysis::possible_time_save::calculate(
                        timer,
                        split_index,
                        comparison,
                        false,
                    );
                }
            } else if let Some(prev_split_index) = split_index.checked_sub(1) {
                time_change =
                    analysis::previous_segment_delta(timer, prev_split_index, comparison, method);
                if self.settings.show_possible_time_save {
                    previous_possible = analysis::possible_time_save::calculate(
                        timer,
                        prev_split_index,
                        comparison,
                        false,
                    );
                }
            };

            if let Some(time_change) = time_change {
                if live_segment.is_some() {
                    analysis::split_color(
                        timer,
                        time_change.into(),
                        split_index,
                        false,
                        false,
                        comparison,
                        method,
                    )
                } else if let Some(prev_split_index) = split_index.checked_sub(1) {
                    analysis::split_color(
                        timer,
                        time_change.into(),
                        prev_split_index,
                        false,
                        true,
                        comparison,
                        method,
                    )
                } else {
                    SemanticColor::Default
                }
            } else if let Some(prev_split_index) = split_index.checked_sub(1) {
                analysis::split_color(
                    timer,
                    None,
                    prev_split_index,
                    true,
                    true,
                    comparison,
                    method,
                )
            } else {
                SemanticColor::Default
            }
        } else {
            SemanticColor::Default
        };

        let value_color = Some(semantic_color.visualize(layout_settings));

        let text = self.text(live_segment.is_some(), resolved_comparison);
        let mut time = Delta::custom(self.settings.drop_decimals, self.settings.accuracy)
            .format(time_change)
            .to_string();

        if self.settings.show_possible_time_save {
            write!(
                time,
                " / {}",
                SegmentTime::with_accuracy(self.settings.accuracy).format(previous_possible)
            )
            .unwrap();
        }

        let key_abbreviations = if live_segment.is_some() {
            Box::new(["Live Segment".into(), "Live Seg.".into()]) as _
        } else {
            Box::new([
                "Previous Segment".into(),
                "Prev. Segment".into(),
                "Prev. Seg.".into(),
            ]) as _
        };

        key_value::State {
            background: self.settings.background,
            key_color: self.settings.label_color,
            value_color,
            semantic_color,
            key: text.into_owned().into(),
            value: time.into(),
            key_abbreviations,
            display_two_rows: self.settings.display_two_rows,
        }
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new("Background".into(), self.settings.background.into()),
            Field::new(
                "Comparison".into(),
                self.settings.comparison_override.clone().into(),
            ),
            Field::new(
                "Display 2 Rows".into(),
                self.settings.display_two_rows.into(),
            ),
            Field::new("Label Color".into(), self.settings.label_color.into()),
            Field::new("Drop Decimals".into(), self.settings.drop_decimals.into()),
            Field::new("Accuracy".into(), self.settings.accuracy.into()),
            Field::new(
                "Show Possible Time Save".into(),
                self.settings.show_possible_time_save.into(),
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
            1 => self.settings.comparison_override = value.into(),
            2 => self.settings.display_two_rows = value.into(),
            3 => self.settings.label_color = value.into(),
            4 => self.settings.drop_decimals = value.into(),
            5 => self.settings.accuracy = value.into(),
            6 => self.settings.show_possible_time_save = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
