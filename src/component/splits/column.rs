use crate::{
    analysis::{self, possible_time_save, split_color},
    comparison,
    component::splits::Settings as SplitsSettings,
    platform::prelude::*,
    settings::{Color, SemanticColor},
    timing::{
        formatter::{Delta, Regular, SegmentTime, TimeFormatter},
        Snapshot,
    },
    util::Clear,
    GeneralLayoutSettings, Segment, TimeSpan, TimingMethod,
};
use core::fmt::Write;
use serde::{Deserialize, Serialize};

/// The settings of an individual column showing timing information on each
/// split.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ColumnSettings {
    /// The name of the column.
    pub name: String,
    /// Specifies the value a segment starts out with before it gets replaced
    /// with the current attempt's information when splitting.
    pub start_with: ColumnStartWith,
    /// Once a certain condition is met, which is usually being on the split or
    /// already having completed the split, the time gets updated with the value
    /// specified here.
    pub update_with: ColumnUpdateWith,
    /// Specifies when a column's value gets updated.
    pub update_trigger: ColumnUpdateTrigger,
    /// The comparison chosen. Uses the Timer's current comparison if set to
    /// `None`.
    pub comparison_override: Option<String>,
    /// Specifies the Timing Method to use. If set to `None` the Timing Method
    /// of the Timer is used for showing the time. Otherwise the Timing Method
    /// provided is used.
    pub timing_method: Option<TimingMethod>,
}

/// Specifies the value a segment starts out with before it gets replaced
/// with the current attempt's information when splitting.
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ColumnStartWith {
    /// The column starts out with an empty value.
    Empty,
    /// The column starts out with the times stored in the comparison that is
    /// being compared against.
    ComparisonTime,
    /// The column starts out with the segment times stored in the comparison
    /// that is being compared against.
    ComparisonSegmentTime,
    /// The column starts out with the time that can be saved on each individual
    /// segment stored in the comparison that is being compared against.
    PossibleTimeSave,
}

/// Once a certain condition is met, which is usually being on the split or
/// already having completed the split, the time gets updated with the value
/// specified here.
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ColumnUpdateWith {
    /// The value doesn't get updated and stays on the value it started out
    /// with.
    DontUpdate,
    /// The value gets replaced by the current attempt's split time.
    SplitTime,
    /// The value gets replaced by the delta of the current attempt's and the
    /// comparison's split time.
    Delta,
    /// The value gets replaced by the delta of the current attempt's and the
    /// comparison's split time. If there is no delta, the value gets replaced
    /// by the current attempt's split time instead.
    DeltaWithFallback,
    /// The value gets replaced by the current attempt's segment time.
    SegmentTime,
    /// The value gets replaced by the current attempt's time saved or lost,
    /// which is how much faster or slower the current attempt's segment time is
    /// compared to the comparison's segment time. This matches the Previous
    /// Segment component.
    SegmentDelta,
    /// The value gets replaced by the current attempt's time saved or lost,
    /// which is how much faster or slower the current attempt's segment time is
    /// compared to the comparison's segment time. This matches the Previous
    /// Segment component. If there is no time saved or lost, then value gets
    /// replaced by the current attempt's segment time instead.
    SegmentDeltaWithFallback,
}

/// Specifies when a column's value gets updated.
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ColumnUpdateTrigger {
    /// The value gets updated as soon as the segment is started. The value
    /// constantly updates until the segment ends.
    OnStartingSegment,
    /// The value doesn't immediately get updated when the segment is started.
    /// Instead the value constantly gets updated once the segment time is
    /// longer than the best segment time. The final update to the value happens
    /// when the segment ends.
    Contextual,
    /// The value of a segment gets updated once the segment ends.
    OnEndingSegment,
}

impl Default for ColumnSettings {
    fn default() -> Self {
        ColumnSettings {
            name: String::from("Column"),
            start_with: ColumnStartWith::Empty,
            update_with: ColumnUpdateWith::DontUpdate,
            update_trigger: ColumnUpdateTrigger::Contextual,
            comparison_override: None,
            timing_method: None,
        }
    }
}

/// Describes the state of a single segment's column to visualize.
#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnState {
    /// The value shown in the column.
    pub value: String,
    /// The semantic coloring information the value carries.
    pub semantic_color: SemanticColor,
    /// The visual color of the value.
    pub visual_color: Color,
    /// This value indicates whether the column is currently frequently being
    /// updated. This can be used for rendering optimizations.
    pub updates_frequently: bool,
}

impl Clear for ColumnState {
    fn clear(&mut self) {
        self.value.clear();
    }
}

enum ColumnFormatter {
    Time,
    Delta,
    SegmentTime,
}

pub fn update_state(
    state: &mut ColumnState,
    column_settings: &ColumnSettings,
    timer: &Snapshot<'_>,
    splits_settings: &SplitsSettings,
    layout_settings: &GeneralLayoutSettings,
    segment: &Segment,
    segment_index: usize,
    current_split: Option<usize>,
    method: TimingMethod,
) {
    let method = column_settings.timing_method.unwrap_or(method);
    let resolved_comparison = comparison::resolve(&column_settings.comparison_override, timer);
    let comparison = comparison::or_current(resolved_comparison, timer);

    let update_value = column_update_value(
        column_settings,
        timer,
        segment,
        segment_index,
        current_split,
        method,
        comparison,
    );

    let updated = update_value.is_some();

    let ((column_value, semantic_color, formatter), is_live) = update_value.unwrap_or_else(|| {
        (
            match column_settings.start_with {
                ColumnStartWith::Empty => (None, SemanticColor::Default, ColumnFormatter::Time),
                ColumnStartWith::ComparisonTime => (
                    segment.comparison(comparison)[method],
                    SemanticColor::Default,
                    ColumnFormatter::Time,
                ),
                ColumnStartWith::ComparisonSegmentTime => (
                    analysis::comparison_combined_segment_time(
                        timer.run(),
                        segment_index,
                        comparison,
                        method,
                    ),
                    SemanticColor::Default,
                    ColumnFormatter::SegmentTime,
                ),
                ColumnStartWith::PossibleTimeSave => (
                    possible_time_save::calculate(timer, segment_index, comparison, false).0,
                    SemanticColor::Default,
                    ColumnFormatter::SegmentTime,
                ),
            },
            false,
        )
    });

    let is_empty = column_settings.start_with == ColumnStartWith::Empty && !updated;

    state.updates_frequently = is_live && column_value.is_some();

    state.value.clear();
    if !is_empty {
        let _ = match formatter {
            ColumnFormatter::Time => write!(
                state.value,
                "{}",
                Regular::with_accuracy(splits_settings.split_time_accuracy).format(column_value)
            ),
            ColumnFormatter::Delta => write!(
                state.value,
                "{}",
                Delta::custom(
                    splits_settings.delta_drop_decimals,
                    splits_settings.delta_time_accuracy,
                )
                .format(column_value)
            ),
            ColumnFormatter::SegmentTime => {
                write!(
                    state.value,
                    "{}",
                    SegmentTime::with_accuracy(splits_settings.segment_time_accuracy)
                        .format(column_value)
                )
            }
        };
    }

    state.semantic_color = semantic_color;
    state.visual_color = semantic_color.visualize(layout_settings);
}

fn column_update_value(
    column: &ColumnSettings,
    timer: &Snapshot<'_>,
    segment: &Segment,
    segment_index: usize,
    current_split: Option<usize>,
    method: TimingMethod,
    comparison: &str,
) -> Option<((Option<TimeSpan>, SemanticColor, ColumnFormatter), bool)> {
    use self::{ColumnUpdateTrigger::*, ColumnUpdateWith::*};

    if current_split < Some(segment_index) {
        // Didn't reach the segment yet.
        return None;
    }

    let is_current_split = current_split == Some(segment_index);

    if is_current_split {
        if column.update_trigger == OnEndingSegment {
            // The trigger wants the value to be updated when splitting, not before.
            return None;
        }

        if column.update_trigger == Contextual
            && analysis::check_live_delta(
                timer,
                !column.update_with.is_segment_based(),
                comparison,
                method,
            )
            .is_none()
        {
            // It's contextual and the live delta shouldn't be shown yet.
            return None;
        }
    }

    let is_live = is_current_split;

    let value = match (column.update_with, is_live) {
        (DontUpdate, _) => return None,

        (SplitTime, false) => (
            segment.split_time()[method],
            SemanticColor::Default,
            ColumnFormatter::Time,
        ),
        (SplitTime, true) => (
            timer.current_time()[method],
            SemanticColor::Default,
            ColumnFormatter::Time,
        ),

        (Delta | DeltaWithFallback, false) => {
            let split_time = segment.split_time()[method];
            let delta = catch! {
                split_time? -
                segment.comparison(comparison)[method]?
            };
            let (value, formatter) = if delta.is_none() && column.update_with.has_fallback() {
                (split_time, ColumnFormatter::Time)
            } else {
                (delta, ColumnFormatter::Delta)
            };
            (
                value,
                split_color(timer, delta, segment_index, true, true, comparison, method),
                formatter,
            )
        }
        (Delta | DeltaWithFallback, true) => (
            catch! {
                timer.current_time()[method]? -
                segment.comparison(comparison)[method]?
            },
            SemanticColor::Default,
            ColumnFormatter::Delta,
        ),

        (SegmentTime, false) => (
            analysis::previous_segment_time(timer, segment_index, method),
            SemanticColor::Default,
            ColumnFormatter::SegmentTime,
        ),
        (SegmentTime, true) => (
            analysis::live_segment_time(timer, segment_index, method),
            SemanticColor::Default,
            ColumnFormatter::SegmentTime,
        ),

        (SegmentDelta | SegmentDeltaWithFallback, false) => {
            let delta = analysis::previous_segment_delta(timer, segment_index, comparison, method);
            let (value, formatter) = if delta.is_none() && column.update_with.has_fallback() {
                (
                    analysis::previous_segment_time(timer, segment_index, method),
                    ColumnFormatter::SegmentTime,
                )
            } else {
                (delta, ColumnFormatter::Delta)
            };
            (
                value,
                split_color(timer, delta, segment_index, false, true, comparison, method),
                formatter,
            )
        }
        (SegmentDelta | SegmentDeltaWithFallback, true) => (
            analysis::live_segment_delta(timer, segment_index, comparison, method),
            SemanticColor::Default,
            ColumnFormatter::Delta,
        ),
    };

    Some((value, is_live))
}

impl ColumnUpdateWith {
    const fn is_segment_based(self) -> bool {
        use ColumnUpdateWith::*;
        matches!(self, SegmentDelta | SegmentTime | SegmentDeltaWithFallback)
    }

    const fn has_fallback(self) -> bool {
        use ColumnUpdateWith::*;
        matches!(self, DeltaWithFallback | SegmentDeltaWithFallback)
    }
}
