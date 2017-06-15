use Timer;
use super::{ComponentState, ComponentSettings};
use component::{current_comparison, current_pace, delta, graph, possible_time_save,
                previous_segment, splits, sum_of_best, text, timer, title, total_playtime};

#[derive(From)]
pub enum Component {
    CurrentComparison(current_comparison::Component),
    CurrentPace(current_pace::Component),
    Delta(delta::Component),
    Graph(graph::Component),
    PossibleTimeSave(possible_time_save::Component),
    PreviousSegment(previous_segment::Component),
    Splits(splits::Component),
    SumOfBest(sum_of_best::Component),
    Text(text::Component),
    Timer(timer::Component),
    Title(title::Component),
    TotalPlaytime(total_playtime::Component),
}

impl Component {
    pub fn state(&mut self, timer: &Timer) -> ComponentState {
        match *self {
            Component::CurrentComparison(ref mut component) => ComponentState::CurrentComparison(
                component.state(timer),
            ),
            Component::CurrentPace(ref mut component) => ComponentState::CurrentPace(
                component.state(timer),
            ),
            Component::Delta(ref mut component) => ComponentState::Delta(component.state(timer)),
            Component::Graph(ref mut component) => ComponentState::Graph(component.state(timer)),
            Component::PossibleTimeSave(ref mut component) => ComponentState::PossibleTimeSave(
                component.state(timer),
            ),
            Component::PreviousSegment(ref mut component) => ComponentState::PreviousSegment(
                component.state(timer),
            ),
            Component::Splits(ref mut component) => ComponentState::Splits(component.state(timer)),
            Component::SumOfBest(ref mut component) => ComponentState::SumOfBest(
                component.state(timer),
            ),
            Component::Text(ref mut component) => ComponentState::Text(component.state()),
            Component::Timer(ref mut component) => ComponentState::Timer(component.state(timer)),
            Component::Title(ref mut component) => ComponentState::Title(component.state(timer)),
            Component::TotalPlaytime(ref mut component) => ComponentState::TotalPlaytime(
                component.state(timer),
            ),
        }
    }

    pub fn settings(&self) -> ComponentSettings {
        match *self {
            Component::CurrentComparison(_) => ComponentSettings::CurrentComparison,
            Component::CurrentPace(ref component) => ComponentSettings::CurrentPace(
                component.settings().clone(),
            ),
            Component::Delta(ref component) => ComponentSettings::Delta(
                component.settings().clone(),
            ),
            Component::Graph(ref component) => ComponentSettings::Graph(
                component.settings().clone(),
            ),
            Component::PossibleTimeSave(_) => ComponentSettings::PossibleTimeSave,
            Component::PreviousSegment(_) => ComponentSettings::PreviousSegment,
            Component::Splits(ref component) => ComponentSettings::Splits(
                component.settings().clone(),
            ),
            Component::SumOfBest(_) => ComponentSettings::SumOfBest,
            Component::Text(ref component) => ComponentSettings::Text(component.settings().clone()),
            Component::Timer(_) => ComponentSettings::Timer,
            Component::Title(_) => ComponentSettings::Title,
            Component::TotalPlaytime(_) => ComponentSettings::TotalPlaytime,
        }
    }

    pub fn scroll_up(&mut self) {
        if let Component::Splits(ref mut component) = *self {
            component.scroll_up();
        }
    }

    pub fn scroll_down(&mut self) {
        if let Component::Splits(ref mut component) = *self {
            component.scroll_down();
        }
    }

    pub fn remount(&mut self) {
        match *self {
            Component::Splits(ref mut component) => component.remount(),
            Component::Title(ref mut component) => component.remount(),
            _ => {}
        }
    }
}
