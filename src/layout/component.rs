use super::{ComponentSettings, ComponentState, GeneralSettings};
use crate::component::{
    blank_space, current_comparison, current_pace, delta, detailed_timer, graph, pb_chance,
    possible_time_save, previous_segment, segment_time, separator, splits, sum_of_best, text,
    timer, title, total_playtime,
};
use crate::platform::prelude::*;
use crate::settings::{SettingsDescription, Value};
use crate::Timer;
use alloc::borrow::Cow;

/// A Component provides information about a run in a way that is easy to
/// visualize. This type can store any of the components provided by this crate.
#[derive(derive_more::From, Clone)]
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
    DetailedTimer(Box<detailed_timer::Component>),
    /// The Graph Component.
    Graph(graph::Component),
    /// The PB Chance Component.
    PbChance(pb_chance::Component),
    /// The Possible Time Save Component.
    PossibleTimeSave(possible_time_save::Component),
    /// The Previous Segment Component.
    PreviousSegment(previous_segment::Component),
    /// The Segment Time Component.
    SegmentTime(segment_time::Component),
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
        match self {
            Component::BlankSpace(component) => ComponentState::BlankSpace(component.state(timer)),
            Component::CurrentComparison(component) => {
                ComponentState::KeyValue(component.state(timer))
            }
            Component::CurrentPace(component) => ComponentState::KeyValue(component.state(timer)),
            Component::Delta(component) => {
                ComponentState::KeyValue(component.state(timer, layout_settings))
            }
            Component::DetailedTimer(component) => {
                ComponentState::DetailedTimer(Box::new(component.state(timer, layout_settings)))
            }
            Component::Graph(component) => {
                ComponentState::Graph(component.state(timer, layout_settings))
            }
            Component::PbChance(component) => ComponentState::KeyValue(component.state(timer)),
            Component::PossibleTimeSave(component) => {
                ComponentState::KeyValue(component.state(timer))
            }
            Component::PreviousSegment(component) => {
                ComponentState::KeyValue(component.state(timer, layout_settings))
            }
            Component::SegmentTime(component) => ComponentState::KeyValue(component.state(timer)),
            Component::Separator(component) => ComponentState::Separator(component.state(timer)),
            Component::Splits(component) => {
                ComponentState::Splits(component.state(timer, layout_settings))
            }
            Component::SumOfBest(component) => ComponentState::KeyValue(component.state(timer)),
            Component::Text(component) => ComponentState::Text(component.state(timer)),
            Component::Timer(component) => {
                ComponentState::Timer(component.state(timer, layout_settings))
            }
            Component::Title(component) => ComponentState::Title(component.state(timer)),
            Component::TotalPlaytime(component) => ComponentState::KeyValue(component.state(timer)),
        }
    }

    /// Accesses the settings of the component. Each component has different
    /// settings, so you need to handle them on a case by case basis. If you
    /// want to access a more general description of the settings, access the
    /// Settings Description instead.
    pub fn settings(&self) -> ComponentSettings {
        match self {
            Component::BlankSpace(component) => {
                ComponentSettings::BlankSpace(component.settings().clone())
            }
            Component::CurrentComparison(component) => {
                ComponentSettings::CurrentComparison(component.settings().clone())
            }
            Component::CurrentPace(component) => {
                ComponentSettings::CurrentPace(component.settings().clone())
            }
            Component::Delta(component) => ComponentSettings::Delta(component.settings().clone()),
            Component::DetailedTimer(component) => {
                ComponentSettings::DetailedTimer(Box::new(component.settings().clone()))
            }
            Component::Graph(component) => ComponentSettings::Graph(component.settings().clone()),
            Component::PbChance(component) => {
                ComponentSettings::PbChance(component.settings().clone())
            }
            Component::PossibleTimeSave(component) => {
                ComponentSettings::PossibleTimeSave(component.settings().clone())
            }
            Component::PreviousSegment(component) => {
                ComponentSettings::PreviousSegment(component.settings().clone())
            }
            Component::SegmentTime(component) => {
                ComponentSettings::SegmentTime(component.settings().clone())
            }
            Component::Separator(_) => ComponentSettings::Separator,
            Component::Splits(component) => ComponentSettings::Splits(component.settings().clone()),
            Component::SumOfBest(component) => {
                ComponentSettings::SumOfBest(component.settings().clone())
            }
            Component::Text(component) => ComponentSettings::Text(component.settings().clone()),
            Component::Timer(component) => ComponentSettings::Timer(component.settings().clone()),
            Component::Title(component) => ComponentSettings::Title(component.settings().clone()),
            Component::TotalPlaytime(component) => {
                ComponentSettings::TotalPlaytime(component.settings().clone())
            }
        }
    }

    /// Accesses the name of the component.
    pub fn name(&self) -> Cow<'_, str> {
        match self {
            Component::BlankSpace(component) => component.name().into(),
            Component::CurrentComparison(component) => component.name().into(),
            Component::CurrentPace(component) => component.name(),
            Component::Delta(component) => component.name(),
            Component::DetailedTimer(component) => component.name().into(),
            Component::Graph(component) => component.name(),
            Component::PbChance(component) => component.name().into(),
            Component::PossibleTimeSave(component) => component.name(),
            Component::PreviousSegment(component) => component.name(),
            Component::SegmentTime(component) => component.name(),
            Component::Separator(component) => component.name().into(),
            Component::Splits(component) => component.name().into(),
            Component::SumOfBest(component) => component.name().into(),
            Component::Text(component) => component.name(),
            Component::Timer(component) => component.name().into(),
            Component::Title(component) => component.name().into(),
            Component::TotalPlaytime(component) => component.name().into(),
        }
    }

    /// Tells the component to scroll up. This may be interpreted differently
    /// based on the kind of component. Most components will ignore this.
    pub fn scroll_up(&mut self) {
        if let Component::Splits(component) = self {
            component.scroll_up();
        }
    }

    /// Tells the component to scroll down. This may be interpreted differently
    /// based on the kind of component. Most components will ignore this.
    pub fn scroll_down(&mut self) {
        if let Component::Splits(component) = self {
            component.scroll_down();
        }
    }

    /// Some component states provide relative information based on information
    /// they already provided. Remounting forces the components to provide
    /// absolute information again, as if they provide the state for the first
    /// time.
    pub fn remount(&mut self) {
        match self {
            Component::DetailedTimer(component) => component.remount(),
            Component::Splits(component) => component.remount(),
            Component::Title(component) => component.remount(),
            _ => {}
        }
    }

    /// Provides a general description of the settings. Such a Settings
    /// Description entirely describes all the settings that are available, what
    /// type they are and what value they currently have. This provides a user
    /// interface independent way of changing the settings.
    pub fn settings_description(&self) -> SettingsDescription {
        match self {
            Component::BlankSpace(component) => component.settings_description(),
            Component::CurrentComparison(component) => component.settings_description(),
            Component::CurrentPace(component) => component.settings_description(),
            Component::Delta(component) => component.settings_description(),
            Component::DetailedTimer(component) => component.settings_description(),
            Component::Graph(component) => component.settings_description(),
            Component::PbChance(component) => component.settings_description(),
            Component::PossibleTimeSave(component) => component.settings_description(),
            Component::PreviousSegment(component) => component.settings_description(),
            Component::SegmentTime(component) => component.settings_description(),
            Component::Separator(component) => component.settings_description(),
            Component::Splits(component) => component.settings_description(),
            Component::SumOfBest(component) => component.settings_description(),
            Component::Text(component) => component.settings_description(),
            Component::Timer(component) => component.settings_description(),
            Component::Title(component) => component.settings_description(),
            Component::TotalPlaytime(component) => component.settings_description(),
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
        match self {
            Component::BlankSpace(component) => component.set_value(index, value),
            Component::CurrentComparison(component) => component.set_value(index, value),
            Component::CurrentPace(component) => component.set_value(index, value),
            Component::Delta(component) => component.set_value(index, value),
            Component::DetailedTimer(component) => component.set_value(index, value),
            Component::Graph(component) => component.set_value(index, value),
            Component::PbChance(component) => component.set_value(index, value),
            Component::PossibleTimeSave(component) => component.set_value(index, value),
            Component::PreviousSegment(component) => component.set_value(index, value),
            Component::SegmentTime(component) => component.set_value(index, value),
            Component::Separator(component) => component.set_value(index, value),
            Component::Splits(component) => component.set_value(index, value),
            Component::SumOfBest(component) => component.set_value(index, value),
            Component::Text(component) => component.set_value(index, value),
            Component::Timer(component) => component.set_value(index, value),
            Component::Title(component) => component.set_value(index, value),
            Component::TotalPlaytime(component) => component.set_value(index, value),
        }
    }
}
