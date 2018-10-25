//! Provides the Splits Component and relevant types for using it. The Splits
//! Component is the main component for visualizing all the split times. Each
//! segment is shown in a tabular fashion showing the segment icon, segment
//! name, the delta compared to the chosen comparison, and the split time. The
//! list provides scrolling functionality, so not every segment needs to be
//! shown all the time.

use analysis::split_color;
use serde_json::{to_writer, Result};
use settings::{Color, Field, Gradient, SemanticColor, SettingsDescription, Value};
use std::borrow::Cow;
use std::cmp::{max, min};
use std::io::Write;
use timing::formatter::none_wrapper::{DashWrapper, EmptyWrapper};
use timing::formatter::{Delta, Regular, TimeFormatter};
use {analysis, CachedImageId, GeneralLayoutSettings, Timer};

#[cfg(test)]
mod tests;

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
}

/// The state object that describes a single segment's information to visualize.
#[derive(Serialize, Deserialize)]
pub struct SplitState {
    /// The name of the segment.
    pub name: String,
    /// The delta to show for this segment.
    pub delta: String,
    /// The split time to show for this segment.
    pub time: String,
    /// The semantic coloring information the delta time carries.
    pub semantic_color: SemanticColor,
    /// The visual color of the delta time.
    pub visual_color: Color,
    /// Describes if this segment is the segment the active attempt is currently
    /// on.
    pub is_current_split: bool,
    /// The index of the segment based on all the segments of the run. This may
    /// differ from the index of this `SplitState` in the `State` object, as
    /// there can be a scrolling window, showing only a subset of segments.
    pub index: usize,
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
        let comparison = timer.current_comparison();

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
                let split = segment.split_time()[method];
                let comparison_time = segment.comparison(comparison)[method];

                let (time, delta, semantic_color) = if current_split > Some(i) {
                    let delta = catch! { split? - comparison_time? };
                    (
                        split,
                        delta,
                        split_color(timer, delta, i, true, true, comparison, method),
                    )
                } else if current_split == Some(i) {
                    (
                        comparison_time,
                        analysis::check_live_delta(timer, true, comparison, method),
                        SemanticColor::Default,
                    )
                } else {
                    (comparison_time, None, SemanticColor::Default)
                };

                let delta = if current_split > Some(i) {
                    DashWrapper::new(Delta::with_decimal_dropping())
                        .format(delta)
                        .to_string()
                } else {
                    EmptyWrapper::new(Delta::with_decimal_dropping())
                        .format(delta)
                        .to_string()
                };

                let visual_color = semantic_color.visualize(layout_settings);

                if let Some(icon_change) = icon_id.update_with(Some(segment.icon())) {
                    icon_changes.push(IconChange {
                        segment_index: i,
                        icon: icon_change.to_owned(),
                    });
                }

                SplitState {
                    name: segment.name().to_string(),
                    delta,
                    time: Regular::new().format(time).to_string(),
                    semantic_color,
                    visual_color,
                    is_current_split: Some(i) == current_split,
                    index: i,
                }
            }).collect();

        if fill_with_blank_space && splits.len() < visual_split_count {
            let blank_split_count = visual_split_count - splits.len();
            for _ in 0..blank_split_count {
                splits.push(SplitState {
                    name: String::from(""),
                    delta: String::from(""),
                    time: String::from(""),
                    semantic_color: SemanticColor::Default,
                    visual_color: SemanticColor::Default.visualize(layout_settings),
                    is_current_split: false,
                    index: ::std::usize::MAX ^ 1,
                });
            }
        }

        State {
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
        SettingsDescription::with_fields(vec![
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
        ])
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
            0 => self.settings.visual_split_count = value.into_uint().unwrap() as _,
            1 => self.settings.split_preview_count = value.into_uint().unwrap() as _,
            2 => self.settings.show_thin_separators = value.into(),
            3 => self.settings.separator_last_split = value.into(),
            4 => self.settings.always_show_last_split = value.into(),
            5 => self.settings.fill_with_blank_space = value.into(),
            6 => self.settings.display_two_rows = value.into(),
            7 => self.settings.current_split_gradient = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
