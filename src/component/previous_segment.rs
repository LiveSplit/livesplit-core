use {analysis, comparison, GeneralLayoutSettings, Timer, TimerPhase};
use time::formatter::{Accuracy, Delta, PossibleTimeSave, TimeFormatter};
use serde_json::{to_writer, Result};
use std::io::Write;
use std::fmt::Write as FmtWrite;
use std::borrow::Cow;
use settings::{Color, Field, Gradient, SemanticColor, SettingsDescription, Value};
use super::DEFAULT_INFO_TEXT_GRADIENT;

#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub background: Gradient,
    pub comparison_override: Option<String>,
    pub label_color: Option<Color>,
    pub drop_decimals: bool,
    pub accuracy: Accuracy,
    pub show_possible_time_save: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: DEFAULT_INFO_TEXT_GRADIENT,
            comparison_override: None,
            label_color: None,
            drop_decimals: true,
            accuracy: Accuracy::Tenths,
            show_possible_time_save: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub background: Gradient,
    pub label_color: Option<Color>,
    pub text: String,
    pub time: String,
    pub semantic_color: SemanticColor,
    pub visual_color: Color,
}

impl State {
    pub fn write_json<W>(&self, writer: W) -> Result<()>
    where
        W: Write,
    {
        to_writer(writer, self)
    }
}

impl Component {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_settings(settings: Settings) -> Self {
        Self {
            settings,
            ..Default::default()
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    pub fn name(&self) -> Cow<str> {
        self.text(
            false,
            self.settings
                .comparison_override
                .as_ref()
                .map(String::as_ref),
        )
    }

    fn text(&self, live: bool, comparison: Option<&str>) -> Cow<str> {
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

    pub fn state(&self, timer: &Timer, layout_settings: &GeneralLayoutSettings) -> State {
        let mut time_change = None;
        let mut previous_possible = None;
        let mut live_segment = false;
        let resolved_comparison = comparison::resolve(&self.settings.comparison_override, timer);
        let comparison = comparison::or_current(resolved_comparison, timer);

        let phase = timer.current_phase();
        let method = timer.current_timing_method();
        let semantic_color = if phase != TimerPhase::NotRunning {
            let split_index = timer.current_split_index().unwrap();
            if (phase == TimerPhase::Running || phase == TimerPhase::Paused)
                && analysis::check_live_delta(timer, false, comparison, method).is_some()
            {
                live_segment = true;
            }

            if live_segment {
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
                if live_segment {
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

        let visual_color = semantic_color.visualize(layout_settings);

        let text = self.text(live_segment, resolved_comparison);
        let mut time = Delta::custom(self.settings.drop_decimals, self.settings.accuracy)
            .format(time_change)
            .to_string();

        if self.settings.show_possible_time_save {
            write!(
                time,
                " / {}",
                PossibleTimeSave::with_accuracy(self.settings.accuracy).format(previous_possible)
            ).unwrap();
        }

        State {
            background: self.settings.background,
            label_color: self.settings.label_color,
            text: text.into_owned(),
            time,
            semantic_color,
            visual_color,
        }
    }

    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new("Background".into(), self.settings.background.into()),
            Field::new(
                "Comparison".into(),
                self.settings.comparison_override.clone().into(),
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

    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.background = value.into(),
            1 => self.settings.comparison_override = value.into(),
            2 => self.settings.label_color = value.into(),
            3 => self.settings.drop_decimals = value.into(),
            4 => self.settings.accuracy = value.into(),
            5 => self.settings.show_possible_time_save = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
