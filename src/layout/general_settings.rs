use settings::{Color, Gradient, SettingsDescription, Field, Value};

#[derive(Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub background: Gradient,
    pub best_segment_color: Color,
    pub ahead_gaining_time_color: Color,
    pub ahead_losing_time_color: Color,
    pub behind_gaining_time_color: Color,
    pub behind_losing_time_color: Color,
    pub not_running_color: Color,
    pub personal_best_color: Color,
    pub paused_color: Color,
    pub thin_separators_color: Color,
    pub separators_color: Color,
    pub text_color: Color,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            background: Gradient::Plain(Color::hsla(0.0, 0.0, 0.06, 1.0)),
            best_segment_color: Color::hsla(50.0, 1.0, 0.5, 1.0),
            ahead_gaining_time_color: Color::hsla(136.0, 1.0, 0.4, 1.0),
            ahead_losing_time_color: Color::hsla(136.0, 0.55, 0.6, 1.0),
            behind_gaining_time_color: Color::hsla(0.0, 0.55, 0.6, 1.0),
            behind_losing_time_color: Color::hsla(0.0, 1.0, 0.4, 1.0),
            not_running_color: Color::hsla(0.0, 0.0, 0.67, 1.0),
            personal_best_color: Color::hsla(203.0, 1.0, 0.54, 1.0),
            paused_color: Color::hsla(0.0, 0.0, 0.48, 1.0),
            thin_separators_color: Color::hsla(0.0, 0.0, 1.0, 0.06),
            separators_color: Color::hsla(0.0, 0.0, 1.0, 0.35),
            text_color: Color::hsla(0.0, 0.0, 1.0, 1.0),
        }
    }
}

impl GeneralSettings {
    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new("Background".into(), self.background.into()),
            Field::new("Best Segment".into(), self.best_segment_color.into()),
            Field::new(
                "Ahead (Gaining Time)".into(),
                self.ahead_gaining_time_color.into(),
            ),
            Field::new(
                "Ahead (Losing Time)".into(),
                self.ahead_losing_time_color.into(),
            ),
            Field::new(
                "Behind (Gaining Time)".into(),
                self.behind_gaining_time_color.into(),
            ),
            Field::new(
                "Behind (Losing Time)".into(),
                self.behind_losing_time_color.into(),
            ),
            Field::new("Not Running".into(), self.not_running_color.into()),
            Field::new("Personal Best".into(), self.personal_best_color.into()),
            Field::new("Paused".into(), self.paused_color.into()),
            Field::new("Thin Separators".into(), self.thin_separators_color.into()),
            Field::new("Separators".into(), self.separators_color.into()),
            Field::new("Text".into(), self.text_color.into()),
        ])
    }

    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.background = value.into(),
            1 => self.best_segment_color = value.into(),
            2 => self.ahead_gaining_time_color = value.into(),
            3 => self.ahead_losing_time_color = value.into(),
            4 => self.behind_gaining_time_color = value.into(),
            5 => self.behind_losing_time_color = value.into(),
            6 => self.not_running_color = value.into(),
            7 => self.personal_best_color = value.into(),
            8 => self.paused_color = value.into(),
            9 => self.thin_separators_color = value.into(),
            10 => self.separators_color = value.into(),
            11 => self.text_color = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
