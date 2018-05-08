use super::{ComponentSettings, ComponentState, GeneralSettings};
use component::{
    blank_space, current_comparison, current_pace, delta, detailed_timer, graph,
    possible_time_save, previous_segment, separator, splits, sum_of_best, text, timer, title,
    total_playtime,
};
use settings::{SettingsDescription, Value};
use std::borrow::Cow;
use Timer;

/// A Component provides information about a run in a way that is easy to
/// visualize. This type can store any of the components provided by this crate.
#[derive(From, Clone)]
pub enum Component {
    /// The Blank Space Component.
    BlankSpace(blank_space::Component),
    /// The Current Comparison Component.
    CurrentComparison(current_comparison::Component),
    /// The Current Pace Component.
    CurrentPace(current_pace::Component),
    /// The Delta Component.
    Delta(delta::Component),
    /// The Detailed Timer Component.
    DetailedTimer(detailed_timer::Component),
    /// The Graph Component.
    Graph(graph::Component),
    /// The Possible Time Save Component.
    PossibleTimeSave(possible_time_save::Component),
    /// The Previous Segment Component.
    PreviousSegment(previous_segment::Component),
    /// The Separator Component.
    Separator(separator::Component),
    /// The Splits Component.
    Splits(splits::Component),
    /// The Sum of Best Component.
    SumOfBest(sum_of_best::Component),
    /// The Text Component.
    Text(text::Component),
    /// The Timer Component.
    Timer(timer::Component),
    /// The Title Component.
    Title(title::Component),
    /// The Total Playtime Component.
    TotalPlaytime(total_playtime::Component),
}

impl Component {
    /// Calculates the component's state based on the timer and settings
    /// provided. The timer provides the information to visualize and the layout
    /// settings provide general information about how to expose that
    /// information in the state.
    pub fn state(&mut self, timer: &Timer, layout_settings: &GeneralSettings) -> ComponentState {
        match *self {
            Component::BlankSpace(ref mut component) => {
                ComponentState::BlankSpace(component.state(timer))
            }
            Component::CurrentComparison(ref mut component) => {
                ComponentState::CurrentComparison(component.state(timer))
            }
            Component::CurrentPace(ref mut component) => {
                ComponentState::CurrentPace(component.state(timer))
            }
            Component::Delta(ref mut component) => {
                ComponentState::Delta(component.state(timer, layout_settings))
            }
            Component::DetailedTimer(ref mut component) => {
                ComponentState::DetailedTimer(component.state(timer, layout_settings))
            }
            Component::Graph(ref mut component) => {
                ComponentState::Graph(component.state(timer, layout_settings))
            }
            Component::PossibleTimeSave(ref mut component) => {
                ComponentState::PossibleTimeSave(component.state(timer))
            }
            Component::PreviousSegment(ref mut component) => {
                ComponentState::PreviousSegment(component.state(timer, layout_settings))
            }
            Component::Separator(ref mut component) => {
                ComponentState::Separator(component.state(timer))
            }
            Component::Splits(ref mut component) => {
                ComponentState::Splits(component.state(timer, layout_settings))
            }
            Component::SumOfBest(ref mut component) => {
                ComponentState::SumOfBest(component.state(timer))
            }
            Component::Text(ref mut component) => ComponentState::Text(component.state()),
            Component::Timer(ref mut component) => {
                ComponentState::Timer(component.state(timer, layout_settings))
            }
            Component::Title(ref mut component) => ComponentState::Title(component.state(timer)),
            Component::TotalPlaytime(ref mut component) => {
                ComponentState::TotalPlaytime(component.state(timer))
            }
        }
    }

    /// Accesses the settings of the component. Each component has different
    /// settings, so you need to handle them on a case by case basis. If you
    /// want to access a more general description of the settings, access the
    /// Settings Description instead.
    pub fn settings(&self) -> ComponentSettings {
        match *self {
            Component::BlankSpace(ref component) => {
                ComponentSettings::BlankSpace(component.settings().clone())
            }
            Component::CurrentComparison(ref component) => {
                ComponentSettings::CurrentComparison(component.settings().clone())
            }
            Component::CurrentPace(ref component) => {
                ComponentSettings::CurrentPace(component.settings().clone())
            }
            Component::Delta(ref component) => {
                ComponentSettings::Delta(component.settings().clone())
            }
            Component::DetailedTimer(ref component) => {
                ComponentSettings::DetailedTimer(component.settings().clone())
            }
            Component::Graph(ref component) => {
                ComponentSettings::Graph(component.settings().clone())
            }
            Component::PossibleTimeSave(ref component) => {
                ComponentSettings::PossibleTimeSave(component.settings().clone())
            }
            Component::PreviousSegment(ref component) => {
                ComponentSettings::PreviousSegment(component.settings().clone())
            }
            Component::Separator(_) => ComponentSettings::Separator,
            Component::Splits(ref component) => {
                ComponentSettings::Splits(component.settings().clone())
            }
            Component::SumOfBest(ref component) => {
                ComponentSettings::SumOfBest(component.settings().clone())
            }
            Component::Text(ref component) => ComponentSettings::Text(component.settings().clone()),
            Component::Timer(ref component) => {
                ComponentSettings::Timer(component.settings().clone())
            }
            Component::Title(ref component) => {
                ComponentSettings::Title(component.settings().clone())
            }
            Component::TotalPlaytime(ref component) => {
                ComponentSettings::TotalPlaytime(component.settings().clone())
            }
        }
    }

    /// Accesses the name of the component.
    pub fn name(&self) -> Cow<str> {
        match *self {
            Component::BlankSpace(ref component) => component.name(),
            Component::CurrentComparison(ref component) => component.name(),
            Component::CurrentPace(ref component) => component.name(),
            Component::Delta(ref component) => component.name(),
            Component::DetailedTimer(ref component) => component.name(),
            Component::Graph(ref component) => component.name(),
            Component::PossibleTimeSave(ref component) => component.name(),
            Component::PreviousSegment(ref component) => component.name(),
            Component::Separator(ref component) => component.name(),
            Component::Splits(ref component) => component.name(),
            Component::SumOfBest(ref component) => component.name(),
            Component::Text(ref component) => component.name(),
            Component::Timer(ref component) => component.name(),
            Component::Title(ref component) => component.name(),
            Component::TotalPlaytime(ref component) => component.name(),
        }
    }

    /// Tells the component to scroll up. This may be interpreted differently
    /// based on the kind of component. Most components will ignore this.
    pub fn scroll_up(&mut self) {
        if let Component::Splits(ref mut component) = *self {
            component.scroll_up();
        }
    }

    /// Tells the component to scroll down. This may be interpreted differently
    /// based on the kind of component. Most components will ignore this.
    pub fn scroll_down(&mut self) {
        if let Component::Splits(ref mut component) = *self {
            component.scroll_down();
        }
    }

    /// Some component states provide relative information based on information
    /// they already provided. Remounting forces the components to provide
    /// absolute information again, as if they provide the state for the first
    /// time.
    pub fn remount(&mut self) {
        match *self {
            Component::DetailedTimer(ref mut component) => component.remount(),
            Component::Splits(ref mut component) => component.remount(),
            Component::Title(ref mut component) => component.remount(),
            _ => {}
        }
    }

    /// Provides a general description of the settings. Such a Settings
    /// Description entirely describes all the settings that are available, what
    /// type they are and what value they currently have. This provides a user
    /// interface independent way of changing the settings.
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

    /// Changes a setting of the component based on its Settings Description
    /// index.
    ///
    /// # Panics
    ///
    /// This may panic if the index doesn't match any setting provided by the
    /// Settings Description of this component. Additionally, the value needs to
    /// have a compatible type.
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
