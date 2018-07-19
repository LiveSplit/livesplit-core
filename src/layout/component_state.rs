use component::{
    blank_space, current_comparison, current_pace, delta, detailed_timer, graph,
    possible_time_save, previous_segment, separator, splits, sum_of_best, text, timer, title,
    total_playtime,
};

/// The state object for one of the components available.
#[derive(Serialize, Deserialize)]
pub enum ComponentState {
    /// The state object for the Blank Space Component.
    BlankSpace(blank_space::State),
    /// The state object for the Current Comparison Component.
    CurrentComparison(current_comparison::State),
    /// The state object for the Current Pace Component.
    CurrentPace(current_pace::State),
    /// The state object for the Delta Component.
    Delta(delta::State),
    /// The state object for the Detailed Timer Component.
    DetailedTimer(Box<detailed_timer::State>),
    /// The state object for the Graph Component.
    Graph(graph::State),
    /// The state object for the Possible Time Save Component.
    PossibleTimeSave(possible_time_save::State),
    /// The state object for the Previous Segment Component.
    PreviousSegment(previous_segment::State),
    /// The state object for the Separator Component.
    Separator(separator::State),
    /// The state object for the Splits Component.
    Splits(splits::State),
    /// The state object for the Sum Of Best Component.
    SumOfBest(sum_of_best::State),
    /// The state object for the Text Component.
    Text(text::State),
    /// The state object for the Timer Component.
    Timer(timer::State),
    /// The state object for the Title Component.
    Title(title::State),
    /// The state object for the Total Playtime Component.
    TotalPlaytime(total_playtime::State),
}
