use super::{Component, Field, Value};

#[derive(Default, Serialize, Deserialize)]
pub struct SettingsDescription {
    pub fields: Vec<Field>,
}

impl SettingsDescription {
    pub fn with_fields(fields: Vec<Field>) -> Self {
        Self { fields }
    }
}

impl Component {
    pub fn settings_description(&self) -> SettingsDescription {
        match *self {
            Component::BlankSpace(ref component) => component.settings_description(),
            Component::CurrentComparison(ref component) => component.settings_description(),
            Component::CurrentPace(ref component) => component.settings_description(),
            Component::Delta(ref component) => component.settings_description(),
            Component::DetailedTimer(ref component) => component.settings_description(),
            Component::Graph(ref component) => component.settings_description(),
            Component::PossibleTimeSave(ref component) => component.settings_description(),
            Component::PreviousSegment(ref component) => component.settings_description(),
            Component::Separator(ref component) => component.settings_description(),
            Component::Splits(ref component) => component.settings_description(),
            Component::SumOfBest(ref component) => component.settings_description(),
            Component::Text(ref component) => component.settings_description(),
            Component::Timer(ref component) => component.settings_description(),
            Component::Title(ref component) => component.settings_description(),
            Component::TotalPlaytime(ref component) => component.settings_description(),
        }
    }

    pub fn set_value(&mut self, index: usize, value: Value) {
        match *self {
            Component::BlankSpace(ref mut component) => component.set_value(index, value),
            Component::CurrentComparison(ref mut component) => component.set_value(index, value),
            Component::CurrentPace(ref mut component) => component.set_value(index, value),
            Component::Delta(ref mut component) => component.set_value(index, value),
            Component::DetailedTimer(ref mut component) => component.set_value(index, value),
            Component::Graph(ref mut component) => component.set_value(index, value),
            Component::PossibleTimeSave(ref mut component) => component.set_value(index, value),
            Component::PreviousSegment(ref mut component) => component.set_value(index, value),
            Component::Separator(ref mut component) => component.set_value(index, value),
            Component::Splits(ref mut component) => component.set_value(index, value),
            Component::SumOfBest(ref mut component) => component.set_value(index, value),
            Component::Text(ref mut component) => component.set_value(index, value),
            Component::Timer(ref mut component) => component.set_value(index, value),
            Component::Title(ref mut component) => component.set_value(index, value),
            Component::TotalPlaytime(ref mut component) => component.set_value(index, value),
        }
    }
}
