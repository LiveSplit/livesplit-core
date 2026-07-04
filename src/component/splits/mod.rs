//! Provides the Splits Component and relevant types for using it. The Splits
//! Component is the main component for visualizing all the split times. Each
//! Each [`Segment`](crate::run::Segment) is shown in a tabular fashion showing
//! the segment icon, segment name, the delta compared to the chosen comparison,
//! and the split time. The list provides scrolling functionality, so not every
//! [`Segment`](crate::run::Segment) needs to be shown all the time.

use crate::{
    GeneralLayoutSettings,
    localization::{Lang, Text},
    platform::prelude::*,
    settings::{
        self, Color, Field, FieldHint, Gradient, ImageCache, ImageId, ListGradient,
        SettingsDescription, Value,
    },
    timing::{Snapshot, formatter::Accuracy},
    util::{Clear, ClearVec},
};
use alloc::borrow::Cow;
use core::{
    cmp::{max, min},
    hash::{Hash, Hasher},
};
use serde_derive::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

mod column;

pub use column::{
    ColumnKind, ColumnSettings, ColumnStartWith, ColumnState, ColumnUpdateTrigger,
    ColumnUpdateWith, TimeColumn, VariableColumn,
};

const SETTINGS_BEFORE_COLUMNS: usize = 16;
const SETTINGS_PER_TIME_COLUMN: usize = 6;
const SETTINGS_PER_VARIABLE_COLUMN: usize = 2;

/// The Splits Component is the main component for visualizing all the split
/// times. Each [`Segment`](crate::run::Segment) is shown in a tabular fashion
/// showing the segment icon, segment name, the delta compared to the chosen
/// comparison, and the split time.
/// The list provides scrolling functionality, so not every segment needs
/// to be shown all the time.
#[derive(Clone)]
pub struct Component {
    settings: Settings,
    current_split_index: Option<usize>,
    scroll_offset: isize,
}

// FIXME: This is needed for serde's default attribute. Ideally, we could pass
// the locale through to serde.
fn english_settings() -> Settings {
    Settings::new(Lang::English)
}

/// The Settings for this component.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default = "english_settings")]
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
    /// maintain the visual split count specified. Otherwise the visual
    /// split count is reduced to the actual amount of segments.
    pub fill_with_blank_space: bool,
    /// Specifies whether to display each split as two rows, with the segment
    /// name being in one row and the times being in the other.
    pub display_two_rows: bool,
    /// The gradient to show behind the current segment as an indicator of it
    /// being the current segment.
    pub current_split_gradient: Gradient,
    /// Specifies the display accuracy of split times.
    pub split_time_accuracy: Accuracy,
    /// Specifies the display accuracy of segment times.
    pub segment_time_accuracy: Accuracy,
    /// Specifies the display accuracy of delta times.
    pub delta_time_accuracy: Accuracy,
    /// Whether to drop the fractional part of a delta time once it goes past
    /// one minute.
    pub delta_drop_decimals: bool,
    /// Specifies whether to show the names of the columns above the splits.
    pub show_column_labels: bool,
    /// Specifies how native subsplits are displayed.
    pub subsplit_display_mode: SubsplitDisplayMode,
    /// The columns to show on the splits. These can be configured in various
    /// way to show split times, segment times, deltas and so on. The columns
    /// are defined from right to left.
    pub columns: Vec<ColumnSettings>,
}

/// Describes how native subsplits are displayed.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubsplitDisplayMode {
    /// Every segment is shown as part of the flat list without group hierarchy.
    #[default]
    Flat,
    /// Groups are shown with header rows and all group contents expanded.
    AllGroupsExpanded,
    /// Groups are shown with header rows, but only the current group has its
    /// contents expanded.
    CurrentGroupExpanded,
}

/// The state object that describes a single segment's information to visualize.
#[derive(Debug, Serialize, Deserialize)]
pub struct SplitState {
    /// The icon of the segment. The associated image can be looked up in the
    /// image cache. The image may be the empty image. This indicates that there
    /// is no icon.
    pub icon: ImageId,
    /// The name of the segment.
    pub name: String,
    /// The state of each column from right to left. The amount of columns is
    /// not guaranteed to be the same across different splits.
    pub columns: ClearVec<ColumnState>,
    /// Describes if this segment is the segment the active attempt is currently
    /// on.
    pub is_current_split: bool,
    /// Specifies whether this row should be indented.
    pub is_indented: bool,
    /// The visual section this row belongs to. This is used for alternating
    /// backgrounds when multiple flat segments collapse into a single section.
    pub section_index: usize,
    /// The index of the segment based on all the segments of the run. This may
    /// differ from the index of this `SplitState` in the `State` object, as
    /// there can be a scrolling window, showing only a subset of segments. Each
    /// index is guaranteed to be unique.
    pub index: usize,
}

impl Clear for SplitState {
    fn clear(&mut self) {
        self.icon = *ImageId::EMPTY;
        self.name.clear();
        self.columns.clear();
        self.is_current_split = false;
        self.is_indented = false;
        self.section_index = 0;
        self.index = 0;
    }
}

/// The state object describes the information to visualize for this component.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    /// The background shown behind the splits.
    pub background: ListGradient,
    /// The column labels to visualize about the list of splits. If this is
    /// `None`, no labels are supposed to be visualized. The list is specified
    /// from right to left.
    pub column_labels: Option<ClearVec<String>>,
    /// The list of all the segments to visualize.
    pub splits: ClearVec<SplitState>,
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

impl SplitState {
    pub(crate) fn content_fingerprint(&self, state: &mut impl Hasher) {
        self.icon.hash(state);
        self.name.hash(state);
        self.is_current_split.hash(state);
        self.is_indented.hash(state);
        self.section_index.hash(state);
        self.index.hash(state);
        self.columns.len().hash(state);
        for column in self.columns.iter() {
            column.content_fingerprint(state);
        }
    }

    pub(crate) fn updates_frequently(&self) -> bool {
        self.columns.iter().any(ColumnState::updates_frequently)
    }
}

impl State {
    pub(crate) fn content_fingerprint(&self, state: &mut impl Hasher) {
        self.has_icons.hash(state);
        self.column_labels.as_deref().hash(state);
        self.splits.len().hash(state);
        for split in self.splits.iter() {
            split.content_fingerprint(state);
        }
    }

    pub(crate) fn updates_frequently(&self) -> bool {
        self.splits.iter().any(SplitState::updates_frequently)
    }
}

impl Settings {
    /// Creates a new set of default settings.
    pub fn new(lang: Lang) -> Self {
        Settings {
            background: ListGradient::Alternating(
                Color::transparent(),
                Color::rgba(1.0, 1.0, 1.0, 0.04),
            ),
            visual_split_count: 16,
            split_preview_count: 1,
            show_thin_separators: true,
            separator_last_split: true,
            always_show_last_split: true,
            fill_with_blank_space: true,
            display_two_rows: false,
            current_split_gradient: Gradient::Vertical(
                Color::rgba(51.0 / 255.0, 115.0 / 255.0, 244.0 / 255.0, 1.0),
                Color::rgba(21.0 / 255.0, 53.0 / 255.0, 116.0 / 255.0, 1.0),
            ),
            split_time_accuracy: Accuracy::Seconds,
            segment_time_accuracy: Accuracy::Hundredths,
            delta_time_accuracy: Accuracy::Tenths,
            delta_drop_decimals: true,
            show_column_labels: false,
            subsplit_display_mode: SubsplitDisplayMode::Flat,
            columns: vec![
                ColumnSettings {
                    name: Text::SplitTime.resolve(lang).into(),
                    kind: ColumnKind::Time(TimeColumn {
                        start_with: ColumnStartWith::ComparisonTime,
                        update_with: ColumnUpdateWith::SplitTime,
                        update_trigger: ColumnUpdateTrigger::OnEndingSegment,
                        comparison_override: None,
                        timing_method: None,
                    }),
                },
                ColumnSettings {
                    name: String::from("+/−"),
                    kind: ColumnKind::Time(TimeColumn {
                        start_with: ColumnStartWith::Empty,
                        update_with: ColumnUpdateWith::Delta,
                        update_trigger: ColumnUpdateTrigger::Contextual,
                        comparison_override: None,
                        timing_method: None,
                    }),
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
    pub fn new(lang: Lang) -> Self {
        Self::with_settings(Settings::new(lang))
    }

    /// Creates a new Splits Component with the given settings.
    pub const fn with_settings(settings: Settings) -> Self {
        Self {
            settings,
            current_split_index: None,
            scroll_offset: 0,
        }
    }

    /// Accesses the settings of the component.
    pub const fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Grants mutable access to the settings of the component.
    pub const fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    /// Scrolls up the window of the segments that are shown. Doesn't move the
    /// scroll window if it reaches the top of the segments.
    pub const fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    /// Scrolls down the window of the segments that are shown. Doesn't move the
    /// scroll window if it reaches the bottom of the segments.
    pub const fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }

    /// Accesses the name of the component for the specified language.
    pub const fn name(&self, lang: Lang) -> &'static str {
        Text::ComponentSplits.resolve(lang)
    }

    /// Updates the component's state based on the timer and layout settings
    /// provided. The [`ImageCache`] is updated with all the images that are
    /// part of the state. The images are marked as visited in the
    /// [`ImageCache`]. You still need to manually run [`ImageCache::collect`]
    /// to ensure unused images are removed from the cache.
    pub fn update_state(
        &mut self,
        state: &mut State,
        image_cache: &mut ImageCache,
        timer: &Snapshot,
        layout_settings: &GeneralLayoutSettings,
        lang: Lang,
    ) {
        // Reset Scroll Offset when any movement of the split index is observed.
        if self.current_split_index != timer.current_split_index() {
            self.current_split_index = timer.current_split_index();
            self.scroll_offset = 0;
        }

        let run = timer.run();
        let current_split = timer.current_split_index();
        let displayed = displayed_splits(run, current_split, self.settings.subsplit_display_mode);

        let mut visual_split_count = self.settings.visual_split_count;
        if visual_split_count == 0 {
            visual_split_count = displayed.len();
        }

        let method = timer.current_timing_method();
        let current_display_index = current_split.and_then(|current_split| {
            displayed
                .iter()
                .position(|split| !split.is_group_header && split.segment_index == current_split)
                .or_else(|| (current_split >= run.len()).then(|| displayed.len().saturating_sub(1)))
        });

        let locked_last_split = isize::from(self.settings.always_show_last_split);
        let skip_count = min(
            current_display_index.map_or(0, |current_split| {
                max(
                    0,
                    current_split as isize
                        + self.settings.split_preview_count as isize
                        + locked_last_split
                        + 1
                        - visual_split_count as isize,
                )
            }),
            displayed.len() as isize - visual_split_count as isize,
        );
        self.scroll_offset = min(
            max(self.scroll_offset, -skip_count),
            displayed.len() as isize - skip_count - visual_split_count as isize,
        );
        let skip_count = max(0, skip_count + self.scroll_offset) as usize;
        let take_count = visual_split_count - locked_last_split as usize;
        let always_show_last_split = self.settings.always_show_last_split;

        let show_final_separator = self.settings.separator_last_split
            && always_show_last_split
            && skip_count + take_count + 1 < displayed.len();

        let Settings {
            show_thin_separators,
            fill_with_blank_space,
            display_two_rows,
            ref columns,
            ..
        } = self.settings;

        state.background = self.settings.background;

        if self.settings.show_column_labels {
            let column_labels = state.column_labels.get_or_insert_with(Default::default);
            column_labels.clear();
            for c in &self.settings.columns {
                column_labels.push().push_str(&c.name);
            }
        } else {
            state.column_labels = None;
        }

        state.splits.clear();
        for (_i, split) in displayed
            .iter()
            .enumerate()
            .skip(skip_count)
            .filter(|&(i, _)| {
                i - skip_count < take_count || (always_show_last_split && i + 1 == displayed.len())
            })
        {
            let state = state.splits.push_with(|| SplitState {
                icon: *ImageId::EMPTY,
                name: String::new(),
                columns: ClearVec::new(),
                is_current_split: false,
                is_indented: false,
                section_index: 0,
                index: 0,
            });

            let segment = split.segment;
            state.icon = *image_cache
                .cache(segment.icon().id(), || segment.icon().clone())
                .id();

            state.name.push_str(&split.name);

            if split.show_columns {
                for column in columns {
                    column::update_state(
                        state.columns.push_with(|| ColumnState {
                            value: String::new(),
                            semantic_color: Default::default(),
                            visual_color: Color::transparent(),
                            updates_frequently: false,
                        }),
                        column,
                        timer,
                        &self.settings,
                        layout_settings,
                        segment,
                        split.segment_index,
                        split.column_start_index,
                        current_split,
                        method,
                        lang,
                    );
                }
            }

            state.is_current_split =
                !split.is_group_header && Some(split.segment_index) == current_split;
            state.is_indented = split.is_indented;
            state.section_index = split.section_index;
            state.index = split.segment_index;
        }

        if fill_with_blank_space && state.splits.len() < visual_split_count {
            let blank_split_count = visual_split_count - state.splits.len();
            let first_blank_section_index = state
                .splits
                .last()
                .map_or(0, |split| split.section_index + 1);
            for i in 0..blank_split_count {
                let state = state.splits.push_with(|| SplitState {
                    icon: *ImageId::EMPTY,
                    name: String::new(),
                    columns: ClearVec::new(),
                    is_current_split: false,
                    is_indented: false,
                    section_index: 0,
                    index: 0,
                });
                state.is_current_split = false;
                state.section_index = first_blank_section_index + i;
                state.index = (usize::MAX ^ 1) - 2 * i;
            }
        }

        state.has_icons = run.segments().iter().any(|s| !s.icon().is_empty());
        state.show_thin_separators = show_thin_separators;
        state.show_final_separator = show_final_separator;
        state.display_two_rows = display_two_rows;
        state.current_split_gradient = self.settings.current_split_gradient;
    }

    /// Calculates the component's state based on the timer and layout settings
    /// provided. The [`ImageCache`] is updated with all the images that are
    /// part of the state. The images are marked as visited in the
    /// [`ImageCache`]. You still need to manually run [`ImageCache::collect`]
    /// to ensure unused images are removed from the cache.
    pub fn state(
        &mut self,
        image_cache: &mut ImageCache,
        timer: &Snapshot,
        layout_settings: &GeneralLayoutSettings,
        lang: Lang,
    ) -> State {
        let mut state = Default::default();
        self.update_state(&mut state, image_cache, timer, layout_settings, lang);
        state
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values for the specified language.
    pub fn settings_description(&self, lang: Lang) -> SettingsDescription {
        let mut settings = SettingsDescription::with_fields(vec![
            Field::new(
                Text::SplitsBackground.resolve(lang).into(),
                Text::SplitsBackgroundDescription.resolve(lang).into(),
                self.settings.background.into(),
            ),
            Field::new(
                Text::SplitsTotalRows.resolve(lang).into(),
                Text::SplitsTotalRowsDescription.resolve(lang).into(),
                Value::UInt(self.settings.visual_split_count as _),
            ),
            Field::new(
                Text::SplitsUpcomingSegments.resolve(lang).into(),
                Text::SplitsUpcomingSegmentsDescription.resolve(lang).into(),
                Value::UInt(self.settings.split_preview_count as _),
            ),
            Field::new(
                Text::SplitsShowThinSeparators.resolve(lang).into(),
                Text::SplitsShowThinSeparatorsDescription
                    .resolve(lang)
                    .into(),
                self.settings.show_thin_separators.into(),
            ),
            Field::new(
                Text::SplitsShowSeparatorBeforeLastSplit
                    .resolve(lang)
                    .into(),
                Text::SplitsShowSeparatorBeforeLastSplitDescription
                    .resolve(lang)
                    .into(),
                self.settings.separator_last_split.into(),
            ),
            Field::new(
                Text::SplitsAlwaysShowLastSplit.resolve(lang).into(),
                Text::SplitsAlwaysShowLastSplitDescription
                    .resolve(lang)
                    .into(),
                self.settings.always_show_last_split.into(),
            ),
            Field::new(
                Text::SplitsFillWithBlankSpace.resolve(lang).into(),
                Text::SplitsFillWithBlankSpaceDescription
                    .resolve(lang)
                    .into(),
                self.settings.fill_with_blank_space.into(),
            ),
            Field::new(
                Text::SplitsShowTimesBelowSegmentName.resolve(lang).into(),
                Text::SplitsShowTimesBelowSegmentNameDescription
                    .resolve(lang)
                    .into(),
                self.settings.display_two_rows.into(),
            ),
            Field::new(
                Text::SplitsCurrentSegmentGradient.resolve(lang).into(),
                Text::SplitsCurrentSegmentGradientDescription
                    .resolve(lang)
                    .into(),
                self.settings.current_split_gradient.into(),
            ),
            Field::new(
                Text::SplitsSplitTimeAccuracy.resolve(lang).into(),
                Text::SplitsSplitTimeAccuracyDescription
                    .resolve(lang)
                    .into(),
                self.settings.split_time_accuracy.into(),
            ),
            Field::new(
                Text::SplitsSegmentTimeAccuracy.resolve(lang).into(),
                Text::SplitsSegmentTimeAccuracyDescription
                    .resolve(lang)
                    .into(),
                self.settings.segment_time_accuracy.into(),
            ),
            Field::new(
                Text::SplitsDeltaTimeAccuracy.resolve(lang).into(),
                Text::SplitsDeltaTimeAccuracyDescription
                    .resolve(lang)
                    .into(),
                self.settings.delta_time_accuracy.into(),
            ),
            Field::new(
                Text::SplitsDropDeltaDecimals.resolve(lang).into(),
                Text::SplitsDropDeltaDecimalsDescription
                    .resolve(lang)
                    .into(),
                self.settings.delta_drop_decimals.into(),
            ),
            Field::new(
                Text::SplitsShowColumnLabels.resolve(lang).into(),
                Text::SplitsShowColumnLabelsDescription.resolve(lang).into(),
                self.settings.show_column_labels.into(),
            ),
            Field::new(
                "Subsplit Display Mode".into(),
                "0 = flat list, 1 = all groups expanded, 2 = current group expanded".into(),
                Value::UInt(self.settings.subsplit_display_mode as u64),
            ),
            Field::new(
                Text::SplitsColumns.resolve(lang).into(),
                Text::SplitsColumnsDescription.resolve(lang).into(),
                Value::UInt(self.settings.columns.len() as _),
            ),
        ]);

        settings.fields.reserve_exact(
            self.settings
                .columns
                .iter()
                .map(|column| match column.kind {
                    ColumnKind::Variable(_) => SETTINGS_PER_VARIABLE_COLUMN,
                    ColumnKind::Time(_) => SETTINGS_PER_TIME_COLUMN,
                })
                .sum(),
        );

        for column in &self.settings.columns {
            settings.fields.push(Field::new(
                Text::SplitsColumnName.resolve(lang).into(),
                Text::SplitsColumnNameDescription.resolve(lang).into(),
                column.name.clone().into(),
            ));

            match &column.kind {
                ColumnKind::Variable(column) => {
                    settings.fields.push(Field::new(
                        Text::SplitsColumnType.resolve(lang).into(),
                        Text::SplitsColumnTypeDescription.resolve(lang).into(),
                        settings::ColumnKind::Variable.into(),
                    ));
                    settings.fields.push(
                        Field::new(
                            Text::SplitsVariableName.resolve(lang).into(),
                            Text::SplitsVariableNameDescription.resolve(lang).into(),
                            column.variable_name.clone().into(),
                        )
                        .with_hint(FieldHint::CustomVariable),
                    );
                }
                ColumnKind::Time(column) => {
                    settings.fields.push(Field::new(
                        Text::SplitsColumnType.resolve(lang).into(),
                        Text::SplitsColumnTypeDescription.resolve(lang).into(),
                        settings::ColumnKind::Time.into(),
                    ));
                    settings.fields.push(Field::new(
                        Text::SplitsStartWith.resolve(lang).into(),
                        Text::SplitsStartWithDescription.resolve(lang).into(),
                        column.start_with.into(),
                    ));
                    settings.fields.push(Field::new(
                        Text::SplitsUpdateWith.resolve(lang).into(),
                        Text::SplitsUpdateWithDescription.resolve(lang).into(),
                        column.update_with.into(),
                    ));
                    settings.fields.push(Field::new(
                        Text::SplitsUpdateTrigger.resolve(lang).into(),
                        Text::SplitsUpdateTriggerDescription.resolve(lang).into(),
                        column.update_trigger.into(),
                    ));
                    settings.fields.push(
                        Field::new(
                            Text::SplitsColumnComparison.resolve(lang).into(),
                            Text::SplitsColumnComparisonDescription.resolve(lang).into(),
                            column.comparison_override.clone().into(),
                        )
                        .with_hint(FieldHint::Comparison),
                    );
                    settings.fields.push(Field::new(
                        Text::SplitsColumnTimingMethod.resolve(lang).into(),
                        Text::SplitsColumnTimingMethodDescription
                            .resolve(lang)
                            .into(),
                        column.timing_method.into(),
                    ));
                }
            }
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
            9 => self.settings.split_time_accuracy = value.into(),
            10 => self.settings.segment_time_accuracy = value.into(),
            11 => self.settings.delta_time_accuracy = value.into(),
            12 => self.settings.delta_drop_decimals = value.into(),
            13 => self.settings.show_column_labels = value.into(),
            14 => {
                self.settings.subsplit_display_mode = match value.into_uint().unwrap() {
                    0 => SubsplitDisplayMode::Flat,
                    1 => SubsplitDisplayMode::AllGroupsExpanded,
                    _ => SubsplitDisplayMode::CurrentGroupExpanded,
                }
            }
            15 => {
                let new_len = value.into_uint().unwrap() as usize;
                self.settings.columns.resize(new_len, Default::default());
            }
            index => {
                let mut index = index - SETTINGS_BEFORE_COLUMNS;
                for column in &mut self.settings.columns {
                    if index < 2 {
                        match index {
                            0 => column.name = value.into(),
                            _ => {
                                column.kind = match settings::ColumnKind::from(value) {
                                    settings::ColumnKind::Time => {
                                        ColumnKind::Time(Default::default())
                                    }
                                    settings::ColumnKind::Variable => {
                                        ColumnKind::Variable(Default::default())
                                    }
                                }
                            }
                        }
                        return;
                    }
                    index -= 2;
                    match &mut column.kind {
                        ColumnKind::Variable(column) => {
                            if index < 1 {
                                column.variable_name = value.into();
                                return;
                            }
                            index -= 1;
                        }
                        ColumnKind::Time(column) => {
                            if index < 5 {
                                match index {
                                    0 => column.start_with = value.into(),
                                    1 => column.update_with = value.into(),
                                    2 => column.update_trigger = value.into(),
                                    3 => column.comparison_override = value.into(),
                                    _ => column.timing_method = value.into(),
                                }
                                return;
                            }
                            index -= 5;
                        }
                    }
                }
                panic!("Unsupported Setting Index")
            }
        }
    }
}

struct DisplayedSplit<'a> {
    segment_index: usize,
    column_start_index: usize,
    segment: &'a crate::Segment,
    name: Cow<'a, str>,
    is_group_header: bool,
    is_indented: bool,
    show_columns: bool,
    section_index: usize,
}

fn displayed_splits<'a>(
    run: &'a crate::Run,
    current_split: Option<usize>,
    mode: SubsplitDisplayMode,
) -> Vec<DisplayedSplit<'a>> {
    let current_group =
        current_split.and_then(|index| run.segment_groups().group_index_for_segment(index));

    let mut displayed = Vec::with_capacity(run.len());
    for (section_index, view) in run.segment_groups_iter().enumerate() {
        let group_index = view.group_index();
        let is_group = group_index.is_some();
        let show_hierarchy = mode != SubsplitDisplayMode::Flat;
        let expand = match mode {
            SubsplitDisplayMode::Flat => true,
            SubsplitDisplayMode::AllGroupsExpanded => true,
            SubsplitDisplayMode::CurrentGroupExpanded => !is_group || group_index == current_group,
        };

        if is_group && show_hierarchy {
            displayed.push(DisplayedSplit {
                segment_index: view.major_index(),
                column_start_index: view.start_index(),
                segment: view.ending_segment(),
                name: Cow::Owned(view.name_or_default().to_owned()),
                is_group_header: true,
                is_indented: false,
                show_columns: !expand,
                section_index,
            });
        }

        if !expand {
            continue;
        }

        for (offset, segment) in view.segments().iter().enumerate() {
            let segment_index = view.start_index() + offset;
            displayed.push(DisplayedSplit {
                segment_index,
                column_start_index: segment_index,
                segment,
                name: Cow::Borrowed(segment.name()),
                is_group_header: false,
                is_indented: show_hierarchy && is_group,
                show_columns: true,
                section_index: if show_hierarchy {
                    section_index
                } else {
                    segment_index
                },
            });
        }
    }

    displayed
}
