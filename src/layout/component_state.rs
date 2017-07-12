use component::{blank_space, current_comparison, current_pace, delta, detailed_timer, graph,
                possible_time_save, previous_segment, separator, splits, sum_of_best, text, timer,
                title, total_playtime};

#[derive(Serialize, Deserialize)]
pub enum ComponentState {
    BlankSpace(blank_space::State),
    CurrentComparison(current_comparison::State),
    CurrentPace(current_pace::State),
    Delta(delta::State),
    DetailedTimer(detailed_timer::State),
    Graph(graph::State),
    PossibleTimeSave(possible_time_save::State),
    PreviousSegment(previous_segment::State),
    Separator(separator::State),
    Splits(splits::State),
    SumOfBest(sum_of_best::State),
    Text(text::State),
    Timer(timer::State),
    Title(title::State),
    TotalPlaytime(total_playtime::State),
}
