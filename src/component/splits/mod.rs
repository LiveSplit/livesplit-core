//! Provides the Splits Component and relevant types for using it. The Splits
//! Component is the main component for visualizing all the split times. Each
//! segment is shown in a tabular fashion showing the segment icon, segment
//! name, the delta compared to the chosen comparison, and the split time. The
//! list provides scrolling functionality, so not every segment needs to be
//! shown all the time.

use crate::platform::prelude::*;
use crate::{
    settings::{
        CachedImageId, Color, Field, Gradient, ImageData, ListGradient, SettingsDescription, Value,
    },
    GeneralLayoutSettings, Timer,
};
use core::cmp::{max, min};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

mod column;

pub use column::{
    ColumnSettings, ColumnStartWith, ColumnState, ColumnUpdateTrigger, ColumnUpdateWith,
};

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

/// The state object that describes a single segment's information to visualize.
#[derive(Debug, Serialize, Deserialize)]
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
    /// there can be a scrolling window, showing only a subset of segments. Each
    /// index is guaranteed to be unique.
    pub index: usize,
}

/// Describes the icon to be shown for a certain segment. This is provided
/// whenever a segment is first shown or whenever its icon changes. If
/// necessary, you may remount this component to reset the component into a
/// state where these icons are provided again.
#[derive(Debug, Serialize, Deserialize)]
pub struct IconChange {
    /// The index of the segment of which the icon changed. This is based on the
    /// index in the run, not on the index of the `SplitState` in the `State`
    /// object. The corresponding index is the `index` field of the `SplitState`
    /// object.
    pub segment_index: usize,
    /// The segment's icon encoded as the raw file bytes. The buffer itself may
    /// be empty. This indicates that there is no icon.
    pub icon: ImageData,
}

/// The state object describes the information to visualize for this component.
#[derive(Debug, Serialize, Deserialize)]
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
    /// Specifies whether the current run has any icons, even those that are not
    /// currently visible by the splits component. This allows for properly
    /// indenting the icon column, even when the icons are scrolled outside the
    /// splits component.
    pub has_icons: bool,
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

#[cfg(feature = "std")]
impl State {
    /// Encodes the state object's information as JSON.
    pub fn write_json<W>(&self, writer: W) -> serde_json::Result<()>
    where
        W: std::io::Write,
    {
        serde_json::to_writer(writer, self)
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
    pub fn name(&self) -> &'static str {
        "Splits"
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
            })
            .map(|((i, segment), icon_id)| {
                let columns = columns
                    .iter()
                    .map(|column| {
                        column::state(
                            column,
                            timer,
                            layout_settings,
                            segment,
                            i,
                            current_split,
                            method,
                        )
                    })
                    .collect();

                if let Some(icon_change) = icon_id.update_with(Some(segment.icon())) {
                    icon_changes.push(IconChange {
                        segment_index: i,
                        icon: icon_change.into(),
                    });
                }

                SplitState {
                    name: segment.name().to_string(),
                    columns,
                    is_current_split: Some(i) == current_split,
                    index: i,
                }
            })
            .collect();

        if fill_with_blank_space && splits.len() < visual_split_count {
            let blank_split_count = visual_split_count - splits.len();
            for i in 0..blank_split_count {
                splits.push(SplitState {
                    name: String::new(),
                    columns: Vec::new(),
                    is_current_split: false,
                    index: (usize::max_value() ^ 1) - 2 * i,
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
            has_icons: run.segments().iter().any(|s| !s.icon().is_empty()),
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
