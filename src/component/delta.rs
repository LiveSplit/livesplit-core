use {Timer, comparison, GeneralLayoutSettings};
use serde_json::{to_writer, Result};
use std::io::Write;
use analysis::{state_helper, delta};
use time::formatter::{Delta, TimeFormatter, Accuracy};
use std::borrow::Cow;
use settings::{SettingsDescription, Field, Value, SemanticColor, Color};

#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub comparison_override: Option<String>,
    pub drop_decimals: bool,
    pub accuracy: Accuracy,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            comparison_override: None,
            drop_decimals: true,
            accuracy: Accuracy::Tenths,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct State {
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
            self.settings
                .comparison_override
                .as_ref()
                .map(String::as_ref),
        )
    }

    fn text(&self, comparison: Option<&str>) -> Cow<str> {
        if let Some(comparison) = comparison {
            format!("Delta ({})", comparison::shorten(comparison)).into()
        } else {
            "Delta".into()
        }
    }

    pub fn state(&self, timer: &Timer, layout_settings: &GeneralLayoutSettings) -> State {
        let comparison = comparison::resolve(&self.settings.comparison_override, timer);
        let text = self.text(comparison);
        let comparison = comparison::or_current(comparison, timer);

        let (delta, use_live_delta) = delta::calculate(timer, comparison);

        let mut index = timer.current_split_index();
        if !use_live_delta {
            index -= 1;
        }

        let semantic_color = if index >= 0 {
            state_helper::split_color(
                timer,
                delta,
                index as usize,
                true,
                false,
                comparison,
                timer.current_timing_method(),
            )
        } else {
            SemanticColor::Default
        };

        let visual_color = semantic_color.visualize(layout_settings);

        State {
            text: text.into_owned(),
            time: Delta::custom(self.settings.drop_decimals, self.settings.accuracy)
                .format(delta)
                .to_string(),
            semantic_color,
            visual_color,
        }
    }

    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                "Comparison".into(),
                self.settings.comparison_override.clone().into(),
            ),
            Field::new("Drop Decimals".into(), self.settings.drop_decimals.into()),
            Field::new("Accuracy".into(), self.settings.accuracy.into()),
        ])
    }

    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.comparison_override = value.into(),
            1 => self.settings.drop_decimals = value.into(),
            2 => self.settings.accuracy = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
