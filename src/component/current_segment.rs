//! Provides the Current Segment Component and relevant types for using it. The
//! Current Segment Component is a component that shows how much time will be saved
//! or lost during the current [`Segment`](crate::run::Segment) based on the
//! chosen comparison. It displays the difference between the current segment time
//! and the chosen comparison segment time. Additionally, the potential time save for the current
//! [`Segment`](crate::run::Segment) can be displayed.

use super::key_value;
use crate::{
    GeneralLayoutSettings, TimerPhase, analysis, comparison,
    platform::prelude::*,
    settings::{Color, Field, Gradient, SemanticColor, SettingsDescription, Value},
    timing::{
        Snapshot,
        formatter::{Accuracy, Delta, SegmentTime, TimeFormatter},
    },
};
use alloc::borrow::Cow;
use core::fmt::Write as FmtWrite;
use serde_derive::{Deserialize, Serialize};

/// Provides the Current Segment Component and relevant types for using it. The
/// Current Segment Component is a component that shows how much time will be saved
/// or lost during the current [`Segment`](crate::run::Segment) based on the
/// chosen comparison. It displays the difference between the current segment time
/// and the chosen comparison segment time. Additionally, the potential time save for the current
/// [`Segment`](crate::run::Segment) can be displayed.
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
    /// Creates a new Current Segment Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Current Segment Component with the given settings.
    pub const fn with_settings(settings: Settings) -> Self {
        Self { settings }
    }

    /// Accesses the settings of the component.
    pub const fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Grants mutable access to the settings of the component.
    pub const fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    /// Accesses the name of the component.
    pub fn name(&self) -> Cow<'static, str> {
        self.text(
            self.settings
                .comparison_override
                .as_ref()
                .map(String::as_ref),
        )
    }

    fn text(&self, comparison: Option<&str>) -> Cow<'static, str> {
        let text = "Current Segment";
        let mut text = Cow::from(text);
        if let Some(comparison) = comparison {
            write!(text.to_mut(), " ({})", comparison::shorten(comparison)).unwrap();
        }
        text
    }

    /// Updates the component's state based on the timer and layout settings
    /// provided.
    pub fn update_state(
        &self,
        state: &mut key_value::State,
        timer: &Snapshot<'_>,
        layout_settings: &GeneralLayoutSettings,
    ) {
        let mut possible_save = None;
        let resolved_comparison = comparison::resolve(&self.settings.comparison_override, timer);
        let comparison = comparison::or_current(resolved_comparison, timer);
        let method = timer.current_timing_method();
        let split_index = timer.current_split_index().unwrap();
        let phase = timer.current_phase();
        let time_change = analysis::live_segment_delta(timer, split_index, comparison, method);
        let semantic_color = if phase == TimerPhase::NotRunning {
            SemanticColor::Default
        } else {
            if self.settings.show_possible_time_save {
                possible_save = analysis::possible_time_save::calculate(
                    timer,
                    split_index,
                    comparison,
                    false,
                )
                .0;
            };
            analysis::split_color(
                timer,
                time_change,
                split_index,
                false,
                false,
                comparison,
                method,
            )
        };

        let value_color = Some(semantic_color.visualize(layout_settings));

        let text = self.text(resolved_comparison);

        state.background = self.settings.background;
        state.key_color = self.settings.label_color;
        state.value_color = value_color;
        state.semantic_color = semantic_color;

        state.key.clear();
        state.key.push_str(&text); // FIXME: Uncow

        state.value.clear();
        let _ = write!(
            state.value,
            "{}",
            Delta::custom(self.settings.drop_decimals, self.settings.accuracy).format(time_change),
        );

        if self.settings.show_possible_time_save {
            let _ = write!(
                state.value,
                " / {}",
                SegmentTime::with_accuracy(self.settings.accuracy).format(possible_save),
            );
        }

        state.key_abbreviations.clear();
        state.key_abbreviations.push("Current Segment".into());
        state.key_abbreviations.push("Curr. Segment".into());
        state.key_abbreviations.push("Curr. Seg.".into());

        state.display_two_rows = self.settings.display_two_rows;
        state.updates_frequently = phase.updates_frequently(method);
    }

    /// Calculates the component's state based on the timer and the layout
    /// settings provided.
    pub fn state(
        &self,
        timer: &Snapshot<'_>,
        layout_settings: &GeneralLayoutSettings,
    ) -> key_value::State {
        let mut state = Default::default();
        self.update_state(&mut state, timer, layout_settings);
        state
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                "Background".into(),
                "The background shown behind the component.".into(),
                self.settings.background.into(),
            ),
            Field::new(
                "Comparison".into(),
                "The comparison used for calculating how much time was saved or lost. If not specified, the current comparison is used.".into(),
                self.settings.comparison_override.clone().into(),
            ),
            Field::new(
                "Display 2 Rows".into(),
                "Specifies whether to display the name of the component and how much time was saved or lost in two separate rows.".into(),
                self.settings.display_two_rows.into(),
            ),
            Field::new(
                "Label Color".into(),
                "The color of the component's name. If not specified, the color is taken from the layout.".into(),
                self.settings.label_color.into(),
            ),
            Field::new(
                "Drop Decimals".into(),
                "Specifies whether to drop the decimals from the time when the time shown is over a minute.".into(),
                self.settings.drop_decimals.into(),
            ),
            Field::new(
                "Accuracy".into(),
                "The accuracy of the time shown.".into(),
                self.settings.accuracy.into(),
            ),
            Field::new(
                "Show Possible Time Save".into(),
                "Specifies whether to show how much time could be saved for the currrent segment in addition to the current delta.".into(),
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
