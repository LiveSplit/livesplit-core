//! Provides the Splits Component and relevant types for using it. The Splits
//! Component is the main component for visualizing all the split times. Each
//! segment is shown in a tabular fashion showing the segment icon, segment
//! name, the delta compared to the chosen comparison, and the split time. The
//! list provides scrolling functionality, so not every segment needs to be
//! shown all the time.

use analysis::split_color;
use serde_json::{to_writer, Result};
use settings::{Color, Field, Gradient, ListGradient, SemanticColor, SettingsDescription, Value};
use std::borrow::Cow;
use std::cmp::{max, min};
use std::io::Write;
use timing::formatter::{Delta, Regular, TimeFormatter};
use {
    analysis, comparison, CachedImageId, GeneralLayoutSettings, Segment, TimeSpan, Timer,
    TimingMethod,
};

#[cfg(test)]
mod tests;

const SETTINGS_BEFORE_COLUMNS: usize = 11;
const SETTINGS_PER_COLUMN: usize = 6;

/// The Splits Component is the main component for visualizing all the split
/// times. Each segment is shown in a tabular fashion showing the segment icon,
/// segment name, the delta compared to the chosen comparison, and the split
/// time. The list provides scrolling functionality, so not every segment needs
/// to be shown all the time.
#[derive(Default, Clone)]
pub struct Component {
    icon_ids: Vec<CachedImageId>,
    settings: Settings,
    current_split_index: Option<usize>,
    scroll_offset: isize,
}

/// The Settings for this component.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// The background shown behind the splits.
    pub background: ListGradient,
    /// The amount of segments to show in the list at any given time. If this is
    /// set to 0, all the segments are shown. If this is set to a number lower
    /// than the total amount of segments, only a certain window of all the
    /// segments is shown. This window can scroll up or down.
    pub visual_split_count: usize,
    /// If there's more segments than segments that are shown, the window
    /// showing the segments automatically scrolls up and down when the current
    /// segment changes. This count determines the minimum number of future
    /// segments to be shown in this scrolling window when it automatically
    /// scrolls.
    pub split_preview_count: usize,
    /// Specifies whether thin separators should be shown between the individual
    /// segments shown by the component.
    pub show_thin_separators: bool,
    /// If the last segment is to always be shown, this determines whether to
    /// show a more pronounced separator in front of the last segment, if it is
    /// not directly adjacent to the segment shown right before it in the
    /// scrolling window.
    pub separator_last_split: bool,
    /// If not every segment is shown in the scrolling window of segments, then
    /// this determines whether the final segment is always to be shown, as it
    /// contains valuable information about the total duration of the chosen
    /// comparison, which is often the runner's Personal Best.
    pub always_show_last_split: bool,
    /// If there's not enough segments to fill the list of splits, this option
    /// allows filling the remaining splits with blank space in order to
    /// maintain the visual split count specified. Otherwise the visual split
    /// count is reduced to the actual amount of segments.
    pub fill_with_blank_space: bool,
    /// Specifies whether to display each split as two rows, with the segment
    /// name being in one row and the times being in the other.
    pub display_two_rows: bool,
    /// The gradient to show behind the current segment as an indicator of it
    /// being the current segment.
    pub current_split_gradient: Gradient,
    /// Specifies whether to show the names of the columns above the splits.
    pub show_column_labels: bool,
    /// The columns to show on the splits. These can be configured in various
    /// way to show split times, segment times, deltas and so on. The columns
    /// are defined from right to left.
    pub columns: Vec<ColumnSettings>,
}

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
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum ColumnStartWith {
    /// The column starts out with an empty value.
    Empty,
    /// The column starts out with the times stored in the comparison that is
    /// being compared against.
    ComparisonTime,
    /// The column starts out with the segment times stored in the comparison
    /// that is being compared against.
    ComparisonSegmentTime,
}

/// Once a certain condition is met, which is usually being on the split or
/// already having completed the split, the time gets updated with the value
/// specified here.
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum ColumnUpdateWith {
    /// The value doesn't get updated and stays on the value it started out
    /// with.
    DontUpdate,
    /// The value gets replaced by the current attempt's split time.
    SplitTime,
    /// The value gets replaced by the delta of the current attempt's and the
    /// comparison's split time.
    Delta,
    /// The value gets replaced by the current attempt's segment time.
    SegmentTime,
    /// The value gets replaced by the current attempt's segment delta, which is
    /// how much faster or slower the current attempt's segment time is compared
    /// to the comparison's segment time. This matches the Previous Segment
    /// component.
    SegmentDelta,
}

/// Specifies when a column's value gets updated.
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
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

/// The state object that describes a single segment's information to visualize.
#[derive(Serialize, Deserialize)]
pub struct SplitState {
    /// The name of the segment.
    pub name: String,
    /// The state of each column from right to left. The amount of columns is
    /// not guaranteed to be the same across different splits.
    pub columns: Vec<ColumnState>,
    /// Describes if this segment is the segment the active attempt is currently
    /// on.
    pub is_current_split: bool,
    /// The index of the segment based on all the segments of the run. This may
    /// differ from the index of this `SplitState` in the `State` object, as
    /// there can be a scrolling window, showing only a subset of segments.
    pub index: usize,
}

/// Describes the state of a single segment's column to visualize.
#[derive(Serialize, Deserialize)]
pub struct ColumnState {
    /// The value shown in the column.
    pub value: String,
    /// The semantic coloring information the value carries.
    pub semantic_color: SemanticColor,
    /// The visual color of the value.
    pub visual_color: Color,
}

/// Describes the icon to be shown for a certain segment. This is provided
/// whenever a segment is first shown or whenever its icon changes. If
/// necessary, you may remount this component to reset the component into a
/// state where these icons are provided again.
#[derive(Serialize, Deserialize)]
pub struct IconChange {
    /// The index of the segment of which the icon changed. This is based on the
    /// index in the run, not on the index of the `SplitState` in the `State`
    /// object. The corresponding index is the `index` field of the `SplitState`
    /// object.
    pub segment_index: usize,
    /// The segment's icon encoded as a Data URL. The String itself may be
    /// empty. This indicates that there is no icon.
    pub icon: String,
}

/// The state object describes the information to visualize for this component.
#[derive(Serialize, Deserialize)]
pub struct State {
    /// The background shown behind the splits.
    pub background: ListGradient,
    /// The column labels to visualize about the list of splits. If this is
    /// `None`, no labels are supposed to be visualized. The list is specified
    /// from right to left.
    pub column_labels: Option<Vec<String>>,
    /// The list of all the segments to visualize.
    pub splits: Vec<SplitState>,
    /// This list describes all the icon changes that happened. Each time a
    /// segment is first shown or its icon changes, the new icon is provided in
    /// this list. If necessary, you may remount this component to reset the
    /// component into a state where these icons are provided again.
    pub icon_changes: Vec<IconChange>,
    /// Specifies whether thin separators should be shown between the individual
    /// segments shown by the component.
    pub show_thin_separators: bool,
    /// Describes whether a more pronounced separator should be shown in front
    /// of the last segment provided.
    pub show_final_separator: bool,
    /// Specifies whether to display each split as two rows, with the segment
    /// name being in one row and the times being in the other.
    pub display_two_rows: bool,
    /// The gradient to show behind the current segment as an indicator of it
    /// being the current segment.
    pub current_split_gradient: Gradient,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            background: ListGradient::Alternating(
                Color::transparent(),
                Color::from((1.0, 1.0, 1.0, 0.04)),
            ),
            visual_split_count: 16,
            split_preview_count: 1,
            show_thin_separators: true,
            separator_last_split: true,
            always_show_last_split: true,
            fill_with_blank_space: true,
            display_two_rows: false,
            current_split_gradient: Gradient::Vertical(
                Color::from((51.0 / 255.0, 115.0 / 255.0, 244.0 / 255.0, 1.0)),
                Color::from((21.0 / 255.0, 53.0 / 255.0, 116.0 / 255.0, 1.0)),
            ),
            show_column_labels: false,
            columns: vec![
                ColumnSettings {
                    name: String::from("Time"),
                    start_with: ColumnStartWith::ComparisonTime,
                    update_with: ColumnUpdateWith::SplitTime,
                    update_trigger: ColumnUpdateTrigger::OnEndingSegment,
                    comparison_override: None,
                    timing_method: None,
                },
                ColumnSettings {
                    name: String::from("+/−"),
                    start_with: ColumnStartWith::Empty,
                    update_with: ColumnUpdateWith::Delta,
                    update_trigger: ColumnUpdateTrigger::Contextual,
                    comparison_override: None,
                    timing_method: None,
                },
            ],
        }
    }
}

impl State {
    /// Encodes the state object's information as JSON.
    pub fn write_json<W>(&self, writer: W) -> Result<()>
    where
        W: Write,
    {
        to_writer(writer, self)
    }
}

impl Component {
    /// Creates a new Splits Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Splits Component with the given settings.
    pub fn with_settings(settings: Settings) -> Self {
        Self {
            settings,
            ..Default::default()
        }
    }

    /// Accesses the settings of the component.
    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Grants mutable access to the settings of the component.
    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    /// Scrolls up the window of the segments that are shown. Doesn't move the
    /// scroll window if it reaches the top of the segments.
    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    /// Scrolls down the window of the segments that are shown. Doesn't move the
    /// scroll window if it reaches the bottom of the segments.
    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }

    /// Remounts the component as if it was freshly initialized. The segment
    /// icons shown by this component are only provided in the state objects
    /// whenever the icon changes or whenever the component's state is first
    /// queried. Remounting returns the segment icons again, whenever its state
    /// is queried the next time.
    pub fn remount(&mut self) {
        self.icon_ids.clear();
    }

    /// Accesses the name of the component.
    pub fn name(&self) -> Cow<str> {
        "Splits".into()
    }

    /// Calculates the component's state based on the timer and layout settings
    /// provided.
    pub fn state(&mut self, timer: &Timer, layout_settings: &GeneralLayoutSettings) -> State {
        // Reset Scroll Offset when any movement of the split index is observed.
        if self.current_split_index != timer.current_split_index() {
            self.current_split_index = timer.current_split_index();
            self.scroll_offset = 0;
        }

        let run = timer.run();
        self.icon_ids.resize(run.len(), CachedImageId::default());

        let mut visual_split_count = self.settings.visual_split_count;
        if visual_split_count == 0 {
            visual_split_count = run.len();
        }

        let current_split = timer.current_split_index();
        let method = timer.current_timing_method();

        let always_show_last_split = if self.settings.always_show_last_split {
            0
        } else {
            1
        };
        let skip_count = min(
            current_split.map_or(0, |c_s| {
                c_s.saturating_sub(
                    visual_split_count
                        .saturating_sub(2)
                        .saturating_sub(self.settings.split_preview_count)
                        .saturating_add(always_show_last_split),
                ) as isize
            }),
            run.len() as isize - visual_split_count as isize,
        );
        self.scroll_offset = min(
            max(self.scroll_offset, -skip_count),
            run.len() as isize - skip_count - visual_split_count as isize,
        );
        let skip_count = max(0, skip_count + self.scroll_offset) as usize;
        let take_count = visual_split_count + always_show_last_split as usize - 1;
        let always_show_last_split = self.settings.always_show_last_split;

        let show_final_separator = self.settings.separator_last_split
            && always_show_last_split
            && skip_count + take_count + 1 < run.len();

        let Settings {
            show_thin_separators,
            fill_with_blank_space,
            display_two_rows,
            ref columns,
            ..
        } = self.settings;

        let mut icon_changes = Vec::new();

        let mut splits: Vec<_> = run
            .segments()
            .iter()
            .enumerate()
            .zip(self.icon_ids.iter_mut())
            .skip(skip_count)
            .filter(|&((i, _), _)| {
                i - skip_count < take_count || (always_show_last_split && i + 1 == run.len())
            }).map(|((i, segment), icon_id)| {
                let columns = columns
                    .iter()
                    .map(|column| {
                        column_state(
                            column,
                            timer,
                            layout_settings,
                            segment,
                            i,
                            current_split,
                            method,
                        )
                    }).collect();

                if let Some(icon_change) = icon_id.update_with(Some(segment.icon())) {
                    icon_changes.push(IconChange {
                        segment_index: i,
                        icon: icon_change.to_owned(),
                    });
                }

                SplitState {
                    name: segment.name().to_string(),
                    columns,
                    is_current_split: Some(i) == current_split,
                    index: i,
                }
            }).collect();

        if fill_with_blank_space && splits.len() < visual_split_count {
            let blank_split_count = visual_split_count - splits.len();
            for _ in 0..blank_split_count {
                splits.push(SplitState {
                    name: String::new(),
                    columns: Vec::new(),
                    is_current_split: false,
                    index: ::std::usize::MAX ^ 1,
                });
            }
        }

        let column_labels = if self.settings.show_column_labels {
            Some(
                self.settings
                    .columns
                    .iter()
                    .map(|c| c.name.clone())
                    .collect(),
            )
        } else {
            None
        };

        State {
            background: self.settings.background,
            column_labels,
            splits,
            icon_changes,
            show_thin_separators,
            show_final_separator,
            display_two_rows,
            current_split_gradient: self.settings.current_split_gradient,
        }
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        let mut settings = SettingsDescription::with_fields(vec![
            Field::new("Background".into(), self.settings.background.into()),
            Field::new(
                "Total Splits".into(),
                Value::UInt(self.settings.visual_split_count as _),
            ),
            Field::new(
                "Upcoming Splits".into(),
                Value::UInt(self.settings.split_preview_count as _),
            ),
            Field::new(
                "Show Thin Separators".into(),
                self.settings.show_thin_separators.into(),
            ),
            Field::new(
                "Show Separator Before Last Split".into(),
                self.settings.separator_last_split.into(),
            ),
            Field::new(
                "Always Show Last Split".into(),
                self.settings.always_show_last_split.into(),
            ),
            Field::new(
                "Fill with Blank Space if Not Enough Splits".into(),
                self.settings.fill_with_blank_space.into(),
            ),
            Field::new(
                "Display 2 Rows".into(),
                self.settings.display_two_rows.into(),
            ),
            Field::new(
                "Current Split Gradient".into(),
                self.settings.current_split_gradient.into(),
            ),
            Field::new(
                "Show Column Labels".into(),
                self.settings.show_column_labels.into(),
            ),
            Field::new(
                "Columns".into(),
                Value::UInt(self.settings.columns.len() as _),
            ),
        ]);

        settings
            .fields
            .reserve_exact(SETTINGS_PER_COLUMN * self.settings.columns.len());

        for column in &self.settings.columns {
            settings
                .fields
                .push(Field::new("Column Name".into(), column.name.clone().into()));
            settings
                .fields
                .push(Field::new("Start With".into(), column.start_with.into()));
            settings
                .fields
                .push(Field::new("Update With".into(), column.update_with.into()));
            settings.fields.push(Field::new(
                "Update Trigger".into(),
                column.update_trigger.into(),
            ));
            settings.fields.push(Field::new(
                "Comparison".into(),
                column.comparison_override.clone().into(),
            ));
            settings.fields.push(Field::new(
                "Timing Method".into(),
                column.timing_method.into(),
            ));
        }

        settings
    }

    /// Sets a setting's value by its index to the given value.
    ///
    /// # Panics
    ///
    /// This panics if the type of the value to be set is not compatible with
    /// the type of the setting's value. A panic can also occur if the index of
    /// the setting provided is out of bounds.
    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.background = value.into(),
            1 => self.settings.visual_split_count = value.into_uint().unwrap() as _,
            2 => self.settings.split_preview_count = value.into_uint().unwrap() as _,
            3 => self.settings.show_thin_separators = value.into(),
            4 => self.settings.separator_last_split = value.into(),
            5 => self.settings.always_show_last_split = value.into(),
            6 => self.settings.fill_with_blank_space = value.into(),
            7 => self.settings.display_two_rows = value.into(),
            8 => self.settings.current_split_gradient = value.into(),
            9 => self.settings.show_column_labels = value.into(),
            10 => {
                let new_len = value.into_uint().unwrap() as usize;
                self.settings.columns.resize(new_len, Default::default());
            }
            index => {
                let index = index - SETTINGS_BEFORE_COLUMNS;
                let column_index = index / SETTINGS_PER_COLUMN;
                let setting_index = index % SETTINGS_PER_COLUMN;
                if let Some(column) = self.settings.columns.get_mut(column_index) {
                    match setting_index {
                        0 => column.name = value.into(),
                        1 => column.start_with = value.into(),
                        2 => column.update_with = value.into(),
                        3 => column.update_trigger = value.into(),
                        4 => column.comparison_override = value.into(),
                        5 => column.timing_method = value.into(),
                        _ => unreachable!(),
                    }
                } else {
                    panic!("Unsupported Setting Index")
                }
            }
        }
    }
}

fn column_state(
    column: &ColumnSettings,
    timer: &Timer,
    layout_settings: &GeneralLayoutSettings,
    segment: &Segment,
    segment_index: usize,
    current_split: Option<usize>,
    method: TimingMethod,
) -> ColumnState {
    let method = column.timing_method.unwrap_or_else(|| method);
    let resolved_comparison = comparison::resolve(&column.comparison_override, timer);
    let comparison = comparison::or_current(resolved_comparison, timer);

    let update_value = column_update_value(
        column,
        timer,
        segment,
        segment_index,
        current_split,
        method,
        comparison,
    );

    let updated = update_value.is_some();

    let (column_value, semantic_color, is_delta) =
        update_value.unwrap_or_else(|| match column.start_with {
            ColumnStartWith::Empty => Default::default(),
            ColumnStartWith::ComparisonTime => (
                segment.comparison(comparison)[method],
                SemanticColor::Default,
                false,
            ),
            ColumnStartWith::ComparisonSegmentTime => (
                analysis::comparison_segment_time(timer.run(), segment_index, comparison, method),
                SemanticColor::Default,
                false,
            ),
        });

    let is_empty = column.start_with == ColumnStartWith::Empty && !updated;

    let value = if is_empty {
        String::new()
    } else if is_delta {
        Delta::with_decimal_dropping()
            .format(column_value)
            .to_string()
    } else {
        Regular::new().format(column_value).to_string()
    };

    ColumnState {
        value,
        semantic_color,
        visual_color: semantic_color.visualize(layout_settings),
    }
}

fn column_update_value(
    column: &ColumnSettings,
    timer: &Timer,
    segment: &Segment,
    segment_index: usize,
    current_split: Option<usize>,
    method: TimingMethod,
    comparison: &str,
) -> Option<(Option<TimeSpan>, SemanticColor, bool)> {
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

        if column.update_trigger == Contextual && analysis::check_live_delta(
            timer,
            !column.update_with.is_segment_based(),
            comparison,
            method,
        ).is_none()
        {
            // It's contextual and the live delta shouldn't be shown yet.
            return None;
        }
    }

    let is_live = is_current_split;

    match (column.update_with, is_live) {
        (DontUpdate, _) => None,

        (SplitTime, false) => Some((segment.split_time()[method], SemanticColor::Default, false)),
        (SplitTime, true) => Some((timer.current_time()[method], SemanticColor::Default, false)),

        (Delta, false) => {
            let value = catch! {
                segment.split_time()[method]? -
                segment.comparison(comparison)[method]?
            };
            Some((
                value,
                split_color(timer, value, segment_index, true, true, comparison, method),
                true,
            ))
        }
        (Delta, true) => Some((
            catch! {
                timer.current_time()[method]? -
                segment.comparison(comparison)[method]?
            },
            SemanticColor::Default,
            true,
        )),

        (SegmentTime, false) => Some((
            analysis::previous_segment_time(timer, segment_index, method),
            SemanticColor::Default,
            false,
        )),
        (SegmentTime, true) => Some((
            analysis::live_segment_time(timer, segment_index, method),
            SemanticColor::Default,
            false,
        )),

        (SegmentDelta, false) => {
            let value = analysis::previous_segment_delta(timer, segment_index, comparison, method);
            Some((
                value,
                split_color(timer, value, segment_index, false, true, comparison, method),
                true,
            ))
        }
        (SegmentDelta, true) => Some((
            analysis::live_segment_delta(timer, segment_index, comparison, method),
            SemanticColor::Default,
            true,
        )),
    }
}

impl ColumnUpdateWith {
    fn is_segment_based(self) -> bool {
        use self::ColumnUpdateWith::*;
        match self {
            SegmentDelta | SegmentTime => true,
            _ => false,
        }
    }
}
