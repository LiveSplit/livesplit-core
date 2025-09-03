use super::{ComponentSettings, ComponentState, GeneralSettings};
use crate::{
    component::{
        blank_space, current_comparison, current_pace, current_segment, delta, detailed_timer, graph, pb_chance,
        possible_time_save, previous_segment, segment_time, separator, splits, sum_of_best, text,
        timer, title, total_playtime,
    },
    platform::prelude::*,
    settings::{ImageCache, SettingsDescription, Value},
    timing::Snapshot,
};
use alloc::borrow::Cow;

/// A `Component` provides information about a run in a way that is easy to
/// visualize. This type can store any of the components provided by this crate.
#[derive(Clone)]
pub enum Component {
    /// The Blank Space Component.
    BlankSpace(blank_space::Component),
    /// The Current Comparison Component.
    CurrentComparison(current_comparison::Component),
    /// The Current Pace Component.
    CurrentPace(current_pace::Component),
    /// The Current Segment Componenet
    CurrentSegment(current_segment::Component),
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

impl From<blank_space::Component> for Component {
    fn from(component: blank_space::Component) -> Self {
        Self::BlankSpace(component)
    }
}

impl From<current_comparison::Component> for Component {
    fn from(component: current_comparison::Component) -> Self {
        Self::CurrentComparison(component)
    }
}

impl From<current_pace::Component> for Component {
    fn from(component: current_pace::Component) -> Self {
        Self::CurrentPace(component)
    }
}

impl From<current_segment::Component> for Component {
    fn from(component: current_segment::Component) -> Self {
        Self::CurrentSegment(component)
    }
}

impl From<delta::Component> for Component {
    fn from(component: delta::Component) -> Self {
        Self::Delta(component)
    }
}

impl From<Box<detailed_timer::Component>> for Component {
    fn from(component: Box<detailed_timer::Component>) -> Self {
        Self::DetailedTimer(component)
    }
}

impl From<graph::Component> for Component {
    fn from(component: graph::Component) -> Self {
        Self::Graph(component)
    }
}

impl From<pb_chance::Component> for Component {
    fn from(component: pb_chance::Component) -> Self {
        Self::PbChance(component)
    }
}

impl From<possible_time_save::Component> for Component {
    fn from(component: possible_time_save::Component) -> Self {
        Self::PossibleTimeSave(component)
    }
}

impl From<previous_segment::Component> for Component {
    fn from(component: previous_segment::Component) -> Self {
        Self::PreviousSegment(component)
    }
}

impl From<segment_time::Component> for Component {
    fn from(component: segment_time::Component) -> Self {
        Self::SegmentTime(component)
    }
}

impl From<separator::Component> for Component {
    fn from(component: separator::Component) -> Self {
        Self::Separator(component)
    }
}

impl From<splits::Component> for Component {
    fn from(component: splits::Component) -> Self {
        Self::Splits(component)
    }
}

impl From<sum_of_best::Component> for Component {
    fn from(component: sum_of_best::Component) -> Self {
        Self::SumOfBest(component)
    }
}

impl From<text::Component> for Component {
    fn from(component: text::Component) -> Self {
        Self::Text(component)
    }
}

impl From<timer::Component> for Component {
    fn from(component: timer::Component) -> Self {
        Self::Timer(component)
    }
}

impl From<title::Component> for Component {
    fn from(component: title::Component) -> Self {
        Self::Title(component)
    }
}

impl From<total_playtime::Component> for Component {
    fn from(component: total_playtime::Component) -> Self {
        Self::TotalPlaytime(component)
    }
}

impl Component {
    /// Updates the component's state based on the timer and settings provided.
    /// The timer provides the information to visualize and the layout settings
    /// provide general information about how to expose that information in the
    /// state. The [`ImageCache`] is updated with all the images that are part
    /// of the state. The images are marked as visited in the [`ImageCache`].
    /// You still need to manually run [`ImageCache::collect`] to ensure unused
    /// images are removed from the cache.
    pub fn update_state(
        &mut self,
        state: &mut ComponentState,
        image_cache: &mut ImageCache,
        timer: &Snapshot<'_>,
        layout_settings: &GeneralSettings,
    ) {
        match (state, self) {
            (ComponentState::BlankSpace(state), Component::BlankSpace(component)) => {
                component.update_state(state)
            }
            (ComponentState::KeyValue(state), Component::CurrentComparison(component)) => {
                component.update_state(state, timer)
            }
            (ComponentState::KeyValue(state), Component::CurrentPace(component)) => {
                component.update_state(state, timer)
            }
            (ComponentState::KeyValue(state), Component::CurrentSegment(component)) => {
                component.update_state(state, timer, layout_settings)
            }
            (ComponentState::KeyValue(state), Component::Delta(component)) => {
                component.update_state(state, timer, layout_settings)
            }
            (ComponentState::DetailedTimer(state), Component::DetailedTimer(component)) => {
                component.update_state(state, image_cache, timer, layout_settings)
            }
            (ComponentState::Graph(state), Component::Graph(component)) => {
                component.update_state(state, timer, layout_settings)
            }
            (ComponentState::KeyValue(state), Component::PbChance(component)) => {
                component.update_state(state, timer)
            }
            (ComponentState::KeyValue(state), Component::PossibleTimeSave(component)) => {
                component.update_state(state, timer)
            }
            (ComponentState::KeyValue(state), Component::PreviousSegment(component)) => {
                component.update_state(state, timer, layout_settings)
            }
            (ComponentState::KeyValue(state), Component::SegmentTime(component)) => {
                component.update_state(state, timer)
            }
            (ComponentState::Separator(state), Component::Separator(component)) => {
                component.update_state(state)
            }
            (ComponentState::Splits(state), Component::Splits(component)) => {
                component.update_state(state, image_cache, timer, layout_settings)
            }
            (ComponentState::KeyValue(state), Component::SumOfBest(component)) => {
                component.update_state(state, timer)
            }
            (ComponentState::Text(state), Component::Text(component)) => {
                component.update_state(state, timer)
            }
            (ComponentState::Timer(state), Component::Timer(component)) => {
                component.update_state(state, timer, layout_settings)
            }
            (ComponentState::Title(state), Component::Title(component)) => {
                component.update_state(state, image_cache, timer)
            }
            (ComponentState::KeyValue(state), Component::TotalPlaytime(component)) => {
                component.update_state(state, timer)
            }
            (state, component) => *state = component.state(image_cache, timer, layout_settings),
        }
    }

    /// Calculates the component's state based on the timer and settings
    /// provided. The timer provides the information to visualize and the layout
    /// settings provide general information about how to expose that
    /// information in the state. The [`ImageCache`] is updated with all the
    /// images that are part of the state. The images are marked as visited in
    /// the [`ImageCache`]. You still need to manually run
    /// [`ImageCache::collect`] to ensure unused images are removed from the
    /// cache.
    pub fn state(
        &mut self,
        image_cache: &mut ImageCache,
        timer: &Snapshot<'_>,
        layout_settings: &GeneralSettings,
    ) -> ComponentState {
        match self {
            Component::BlankSpace(component) => ComponentState::BlankSpace(component.state()),
            Component::CurrentComparison(component) => {
                ComponentState::KeyValue(component.state(timer))
            }
            Component::CurrentPace(component) => ComponentState::KeyValue(component.state(timer)),
            Component::Delta(component) => {
                ComponentState::KeyValue(component.state(timer, layout_settings))
            }
            Component::CurrentSegment(component) => {
                ComponentState::KeyValue(component.state(timer, layout_settings))
            }
            Component::DetailedTimer(component) => ComponentState::DetailedTimer(Box::new(
                component.state(image_cache, timer, layout_settings),
            )),
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
            Component::Separator(component) => ComponentState::Separator(component.state()),
            Component::Splits(component) => {
                ComponentState::Splits(component.state(image_cache, timer, layout_settings))
            }
            Component::SumOfBest(component) => ComponentState::KeyValue(component.state(timer)),
            Component::Text(component) => ComponentState::Text(component.state(timer)),
            Component::Timer(component) => {
                ComponentState::Timer(component.state(timer, layout_settings))
            }
            Component::Title(component) => {
                ComponentState::Title(component.state(image_cache, timer))
            }
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
            Component::CurrentSegment(component) => {
                ComponentSettings::CurrentSegment(component.settings().clone())
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
            Component::CurrentSegment(component) => component.name(),
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
    pub const fn scroll_up(&mut self) {
        if let Component::Splits(component) = self {
            component.scroll_up();
        }
    }

    /// Tells the component to scroll down. This may be interpreted differently
    /// based on the kind of component. Most components will ignore this.
    pub const fn scroll_down(&mut self) {
        if let Component::Splits(component) = self {
            component.scroll_down();
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
            Component::CurrentSegment(component) => component.settings_description(),
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
            Component::CurrentSegment(component) => component.set_value(index, value),
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
