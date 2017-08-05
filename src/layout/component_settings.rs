use super::Component;
use component::{blank_space, current_comparison, current_pace, delta, detailed_timer, graph,
                possible_time_save, previous_segment, separator, splits, sum_of_best, text, timer,
                title, total_playtime};

#[derive(Clone, Serialize, Deserialize)]
pub enum ComponentSettings {
    BlankSpace(blank_space::Settings),
    CurrentComparison(current_comparison::Settings),
    CurrentPace(current_pace::Settings),
    Delta(delta::Settings),
    DetailedTimer(detailed_timer::Settings),
    Graph(graph::Settings),
    PossibleTimeSave(possible_time_save::Settings),
    PreviousSegment(previous_segment::Settings),
    Separator,
    Splits(splits::Settings),
    SumOfBest(sum_of_best::Settings),
    Text(text::Settings),
    Timer(timer::Settings),
    Title(title::Settings),
    TotalPlaytime(total_playtime::Settings),
}

impl From<ComponentSettings> for Component {
    fn from(settings: ComponentSettings) -> Self {
        match settings {
            ComponentSettings::BlankSpace(settings) => {
                Component::BlankSpace(blank_space::Component::with_settings(settings))
            }
            ComponentSettings::CurrentComparison(settings) => {
                Component::CurrentComparison(current_comparison::Component::with_settings(settings))
            }
            ComponentSettings::CurrentPace(settings) => {
                Component::CurrentPace(current_pace::Component::with_settings(settings))
            }
            ComponentSettings::Delta(settings) => {
                Component::Delta(delta::Component::with_settings(settings))
            }
            ComponentSettings::DetailedTimer(settings) => {
                Component::DetailedTimer(detailed_timer::Component::with_settings(settings))
            }
            ComponentSettings::Graph(settings) => {
                Component::Graph(graph::Component::with_settings(settings))
            }
            ComponentSettings::PossibleTimeSave(settings) => {
                Component::PossibleTimeSave(possible_time_save::Component::with_settings(settings))
            }
            ComponentSettings::PreviousSegment(settings) => {
                Component::PreviousSegment(previous_segment::Component::with_settings(settings))
            }
            ComponentSettings::Separator => Component::Separator(separator::Component::new()),
            ComponentSettings::Splits(settings) => {
                Component::Splits(splits::Component::with_settings(settings))
            }
            ComponentSettings::SumOfBest(settings) => {
                Component::SumOfBest(sum_of_best::Component::with_settings(settings))
            }
            ComponentSettings::Text(settings) => {
                Component::Text(text::Component::with_settings(settings))
            }
            ComponentSettings::Timer(settings) => {
                Component::Timer(timer::Component::with_settings(settings))
            }
            ComponentSettings::Title(settings) => {
                Component::Title(title::Component::with_settings(settings))
            }
            ComponentSettings::TotalPlaytime(settings) => {
                Component::TotalPlaytime(total_playtime::Component::with_settings(settings))
            }
        }
    }
}
