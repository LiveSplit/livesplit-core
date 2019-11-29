use super::Component;
use crate::component::{
    blank_space, current_comparison, current_pace, delta, detailed_timer, graph, pb_chance,
    possible_time_save, previous_segment, segment_time, separator, splits, sum_of_best, text,
    timer, title, total_playtime,
};
use crate::platform::prelude::*;
use serde::{Deserialize, Serialize};

/// The settings for one of the components available.
#[derive(Clone, Serialize, Deserialize)]
pub enum ComponentSettings {
    /// The Settings for the Blank Space Component.
    BlankSpace(blank_space::Settings),
    /// The Settings for the Current Comparison Component.
    CurrentComparison(current_comparison::Settings),
    /// The Settings for the Current Pace Component.
    CurrentPace(current_pace::Settings),
    /// The Settings for the Delta Component.
    Delta(delta::Settings),
    /// The Settings for the Detailed Timer Component.
    DetailedTimer(Box<detailed_timer::Settings>),
    /// The Settings for the Graph Component.
    Graph(graph::Settings),
    /// The Settings for the PB Chance Component.
    PbChance(pb_chance::Settings),
    /// The Settings for the Possible Time Save Component.
    PossibleTimeSave(possible_time_save::Settings),
    /// The Settings for the Previous Segment Component.
    PreviousSegment(previous_segment::Settings),
    /// The Settings for the Segment Time Component.
    SegmentTime(segment_time::Settings),
    /// The Settings for the Separator Component.
    Separator,
    /// The Settings for the Splits Component.
    Splits(splits::Settings),
    /// The Settings for the Sum Of Best Component.
    SumOfBest(sum_of_best::Settings),
    /// The Settings for the Text Component.
    Text(text::Settings),
    /// The Settings for the Timer Component.
    Timer(timer::Settings),
    /// The Settings for the Title Component.
    Title(title::Settings),
    /// The Settings for the Total Playtime Component.
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
            ComponentSettings::DetailedTimer(settings) => Component::DetailedTimer(Box::new(
                detailed_timer::Component::with_settings(*settings),
            )),
            ComponentSettings::Graph(settings) => {
                Component::Graph(graph::Component::with_settings(settings))
            }
            ComponentSettings::PbChance(settings) => {
                Component::PbChance(pb_chance::Component::with_settings(settings))
            }
            ComponentSettings::PossibleTimeSave(settings) => {
                Component::PossibleTimeSave(possible_time_save::Component::with_settings(settings))
            }
            ComponentSettings::PreviousSegment(settings) => {
                Component::PreviousSegment(previous_segment::Component::with_settings(settings))
            }
            ComponentSettings::SegmentTime(settings) => {
                Component::SegmentTime(segment_time::Component::with_settings(settings))
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
