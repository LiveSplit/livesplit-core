use super::Component;
use component::{current_comparison, current_pace, delta, detailed_timer, graph,
                possible_time_save, previous_segment, splits, sum_of_best, text, timer, title,
                total_playtime};

#[derive(Clone, Serialize, Deserialize)]
pub enum ComponentSettings {
    CurrentComparison,
    CurrentPace(current_pace::Settings),
    Delta(delta::Settings),
    DetailedTimer(detailed_timer::Settings),
    Graph(graph::Settings),
    PossibleTimeSave,
    PreviousSegment,
    Splits(splits::Settings),
    SumOfBest,
    Text(text::Settings),
    Timer,
    Title(title::Settings),
    TotalPlaytime,
}

impl From<ComponentSettings> for Component {
    fn from(settings: ComponentSettings) -> Self {
        match settings {
            ComponentSettings::CurrentComparison => Component::CurrentComparison(
                current_comparison::Component::new(),
            ),
            ComponentSettings::CurrentPace(settings) => Component::CurrentPace(
                current_pace::Component::with_settings(settings),
            ),
            ComponentSettings::Delta(settings) => Component::Delta(
                delta::Component::with_settings(settings),
            ),
            ComponentSettings::DetailedTimer(settings) => Component::DetailedTimer(
                detailed_timer::Component::with_settings(settings),
            ),
            ComponentSettings::Graph(settings) => Component::Graph(
                graph::Component::with_settings(settings),
            ),
            ComponentSettings::PossibleTimeSave => Component::PossibleTimeSave(
                possible_time_save::Component::new(),
            ),
            ComponentSettings::PreviousSegment => Component::PreviousSegment(
                previous_segment::Component::new(),
            ),
            ComponentSettings::Splits(settings) => Component::Splits(
                splits::Component::with_settings(settings),
            ),
            ComponentSettings::SumOfBest => Component::SumOfBest(sum_of_best::Component::new()),
            ComponentSettings::Text(settings) => Component::Text(
                text::Component::with_settings(settings),
            ),
            ComponentSettings::Timer => Component::Timer(timer::Component::new()),
            ComponentSettings::Title(settings) => Component::Title(
                title::Component::with_settings(settings),
            ),
            ComponentSettings::TotalPlaytime => Component::TotalPlaytime(
                total_playtime::Component::new(),
            ),
        }
    }
}
