use super::Component;
use component::{current_comparison, current_pace, delta, graph, possible_time_save,
                previous_segment, splits, sum_of_best, text, timer, title, total_playtime};

#[derive(Serialize, Deserialize)]
pub enum ComponentSettings {
    CurrentComparison,
    CurrentPace(current_pace::Settings),
    Delta(delta::Settings),
    Graph(graph::Settings),
    PossibleTimeSave,
    PreviousSegment,
    Splits(splits::Settings),
    SumOfBest,
    Text(text::Settings),
    Timer,
    Title,
    TotalPlaytime,
}

impl From<ComponentSettings> for Component {
    fn from(settings: ComponentSettings) -> Self {
        match settings {
            ComponentSettings::CurrentComparison => Component::CurrentComparison(
                current_comparison::Component::new(),
            ),
            ComponentSettings::CurrentPace(settings) => Component::CurrentPace(
                current_pace::Component::with_settings(
                    settings,
                ),
            ),
            ComponentSettings::Delta(settings) => Component::Delta(
                delta::Component::with_settings(settings),
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
            ComponentSettings::Title => Component::Title(title::Component::new()),
            ComponentSettings::TotalPlaytime => Component::TotalPlaytime(
                total_playtime::Component::new(),
            ),
        }
    }
}
