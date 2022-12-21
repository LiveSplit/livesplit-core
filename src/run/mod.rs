//! The run module provides everything necessary for working with a `Run`, like parsing and saving or editing them.
//!
//! # Examples
//!
//! ```
//! use livesplit_core::run::{Run, Segment};
//!
//! let mut run = Run::new();
//!
//! run.set_game_name("Super Mario Odyssey");
//! run.set_category_name("Darker Side");
//!
//! run.push_segment(Segment::new("Cap Kingdom"));
//! run.push_segment(Segment::new("Cascade Kingdom"));
//! ```

mod attempt;
mod comparisons;
pub mod editor;
pub mod parser;
mod run_metadata;
pub mod saver;
mod segment;
mod segment_history;

#[cfg(test)]
mod tests;

pub use attempt::Attempt;
pub use comparisons::Comparisons;
pub use editor::{Editor, RenameError};
pub use run_metadata::{CustomVariable, RunMetadata};
pub use segment::Segment;
pub use segment_history::SegmentHistory;

use crate::{
    comparison::{default_generators, personal_best, ComparisonGenerator, RACE_COMPARISON_PREFIX},
    platform::prelude::*,
    settings::Image,
    util::PopulateString,
    AtomicDateTime, Time, TimeSpan, TimingMethod,
};
use alloc::borrow::Cow;
use core::{cmp::max, fmt};
use hashbrown::HashSet;

/// A Run stores the split times for a specific game and category of a runner.
///
/// # Examples
///
/// ```
/// use livesplit_core::{Run, Segment};
///
/// let mut run = Run::new();
///
/// run.set_game_name("Super Mario Odyssey");
/// run.set_category_name("Darker Side");
///
/// run.push_segment(Segment::new("Cap Kingdom"));
/// run.push_segment(Segment::new("Cascade Kingdom"));
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Run {
    game_icon: Image,
    game_name: String,
    category_name: String,
    offset: TimeSpan,
    attempt_count: u32,
    attempt_history: Vec<Attempt>,
    metadata: RunMetadata,
    has_been_modified: bool,
    segments: Vec<Segment>,
    custom_comparisons: Vec<String>,
    comparison_generators: ComparisonGenerators,
    auto_splitter_settings: String,
}

#[derive(Clone, Debug)]
struct ComparisonGenerators(Vec<Box<dyn ComparisonGenerator>>);

impl PartialEq for ComparisonGenerators {
    fn eq(&self, other: &ComparisonGenerators) -> bool {
        self.0
            .iter()
            .map(|c| c.name())
            .eq(other.0.iter().map(|c| c.name()))
    }
}

/// Error type for an invalid comparison name
#[derive(PartialEq, Eq, Debug, snafu::Snafu)]
pub enum ComparisonError {
    /// Comparison name starts with "\[Race\]".
    NameStartsWithRace,
    /// Comparison name is a duplicate.
    DuplicateName,
}

/// Result type for an invalid comparison name
pub type ComparisonResult<T> = Result<T, ComparisonError>;

impl Run {
    /// Creates a new Run object with no segments.
    #[inline]
    pub fn new() -> Self {
        Self {
            game_icon: Image::default(),
            game_name: String::new(),
            category_name: String::new(),
            offset: TimeSpan::zero(),
            attempt_count: 0,
            attempt_history: Vec::new(),
            metadata: RunMetadata::new(),
            has_been_modified: false,
            segments: Vec::new(),
            custom_comparisons: vec![personal_best::NAME.to_string()],
            comparison_generators: ComparisonGenerators(default_generators()),
            auto_splitter_settings: String::new(),
        }
    }

    /// Accesses the name of the game this Run is for.
    #[inline]
    pub fn game_name(&self) -> &str {
        &self.game_name
    }

    /// Sets the name of the game this Run is for.
    #[inline]
    pub fn set_game_name<S>(&mut self, name: S)
    where
        S: PopulateString,
    {
        name.populate(&mut self.game_name);
    }

    /// Accesses the game's icon.
    #[inline]
    pub const fn game_icon(&self) -> &Image {
        &self.game_icon
    }

    /// Sets the game's icon.
    #[inline]
    pub fn set_game_icon<D: Into<Image>>(&mut self, image: D) {
        self.game_icon = image.into();
    }

    /// Accesses the name of the category this Run is for.
    #[inline]
    pub fn category_name(&self) -> &str {
        &self.category_name
    }

    /// Sets the name of the category this Run is for.
    #[inline]
    pub fn set_category_name<S>(&mut self, name: S)
    where
        S: PopulateString,
    {
        name.populate(&mut self.category_name);
    }

    /// Returns the amount of runs that have been attempted with these splits.
    #[inline]
    pub const fn attempt_count(&self) -> u32 {
        self.attempt_count
    }

    /// Sets the amount of runs that have been attempted with these splits.
    #[inline]
    pub fn set_attempt_count(&mut self, attempts: u32) {
        self.attempt_count = attempts;
    }

    /// Accesses additional metadata of this Run, like the platform and region
    /// of the game.
    #[inline]
    pub const fn metadata(&self) -> &RunMetadata {
        &self.metadata
    }

    /// Grants mutable access to the additional metadata of this Run, like the
    /// platform and region of the game.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut RunMetadata {
        &mut self.metadata
    }

    /// Sets the time an attempt of this Run should start at.
    #[inline]
    pub fn set_offset(&mut self, offset: TimeSpan) {
        self.offset = offset;
    }

    /// Accesses the time an attempt of this Run should start at.
    #[inline]
    pub const fn offset(&self) -> TimeSpan {
        self.offset
    }

    /// Marks a Run that a new Attempt has started. If you use it with a Timer,
    /// this is done automatically.
    pub fn start_next_run(&mut self) {
        self.attempt_count += 1;
        self.has_been_modified = true;
    }

    /// Accesses the Segments of this Run object.
    #[inline]
    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    /// Grants mutable access to the Segments of this Run object.
    #[inline]
    pub fn segments_mut(&mut self) -> &mut Vec<Segment> {
        &mut self.segments
    }

    /// Pushes the segment provided to the end of the list of segments of this Run.
    #[inline]
    pub fn push_segment(&mut self, segment: Segment) {
        self.segments.push(segment);
    }

    /// Accesses a certain segment of this Run.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    #[inline]
    pub fn segment(&self, index: usize) -> &Segment {
        &self.segments[index]
    }

    /// Mutably accesses a certain segment of this Run.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    #[inline]
    pub fn segment_mut(&mut self, index: usize) -> &mut Segment {
        &mut self.segments[index]
    }

    /// Accesses the history of all the runs that have been attempted. This does
    /// not store the actual segment times, just the overall attempt
    /// information. Information about the individual segments is stored within
    /// each segment.
    #[inline]
    pub fn attempt_history(&self) -> &[Attempt] {
        &self.attempt_history
    }

    /// Accesses the custom comparisons that are stored in this Run. This
    /// includes `Personal Best` but excludes all the other Comparison
    /// Generators.
    #[inline]
    pub fn custom_comparisons(&self) -> &[String] {
        &self.custom_comparisons
    }

    /// Grants mutable access to the custom comparisons that are stored in this
    /// Run.  This includes `Personal Best` but excludes all the other
    /// Comparison Generators.
    ///
    /// # Warning
    ///
    /// You may not delete the `Personal Best` comparison.
    #[inline]
    pub fn custom_comparisons_mut(&mut self) -> &mut Vec<String> {
        &mut self.custom_comparisons
    }

    /// Accesses an iterator that iterates over all the comparisons. This
    /// includes both the custom comparisons defined by the user and the
    /// Comparison Generators.
    #[inline]
    pub fn comparisons(&self) -> ComparisonsIter<'_> {
        ComparisonsIter {
            custom: &self.custom_comparisons,
            generators: &self.comparison_generators.0,
        }
    }

    /// Accesses the Comparison Generators in use by this Run.
    #[inline]
    pub fn comparison_generators(&self) -> &[Box<dyn ComparisonGenerator>] {
        &self.comparison_generators.0
    }

    /// Grants mutable access to the Comparison Generators in use by this Run.
    #[inline]
    pub fn comparison_generators_mut(&mut self) -> &mut Vec<Box<dyn ComparisonGenerator>> {
        &mut self.comparison_generators.0
    }

    /// Accesses the Auto Splitter Settings that are encoded as XML.
    #[inline]
    pub fn auto_splitter_settings(&self) -> &str {
        &self.auto_splitter_settings
    }

    /// Grants mutable access to the XML encoded Auto Splitter Settings.
    ///
    /// # Warning
    ///
    /// You need to ensure that the Auto Splitter Settings are encoded as data
    /// that would be valid as an interior of an XML element.
    #[inline]
    pub fn auto_splitter_settings_mut(&mut self) -> &mut String {
        &mut self.auto_splitter_settings
    }

    /// Returns the amount of segments stored in this Run.
    #[inline]
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// Returns `true` if there's no segments stored in this Run.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Marks the Run as modified, so that it is known that there are changes
    /// that should be saved.
    #[inline]
    pub fn mark_as_modified(&mut self) {
        self.has_been_modified = true;
    }

    /// Marks the Run as unmodified, so that it is known that all the changes
    /// have been saved.
    #[inline]
    pub fn mark_as_unmodified(&mut self) {
        self.has_been_modified = false;
    }

    /// Returns whether the Run has been modified and should be saved so that
    /// the changes don't get lost.
    #[inline]
    pub const fn has_been_modified(&self) -> bool {
        self.has_been_modified
    }

    /// Adds a new Attempt to the Run's Attempt History. This is automatically
    /// done if the Run is used with a Timer.
    pub fn add_attempt(
        &mut self,
        time: Time,
        started: Option<AtomicDateTime>,
        ended: Option<AtomicDateTime>,
        pause_time: Option<TimeSpan>,
    ) {
        let index = self
            .attempt_history
            .iter()
            .map(Attempt::index)
            .max()
            .unwrap_or(0);
        let index = max(0, index + 1);
        self.add_attempt_with_index(time, index, started, ended, pause_time);
    }

    /// Adds a new Attempt to the Run's Attempt History with a predetermined
    /// History Index.
    ///
    /// # Warning
    ///
    /// This index may not overlap with an index that is already in the Attempt
    /// History.
    pub fn add_attempt_with_index(
        &mut self,
        time: Time,
        index: i32,
        started: Option<AtomicDateTime>,
        ended: Option<AtomicDateTime>,
        pause_time: Option<TimeSpan>,
    ) {
        let attempt = Attempt::new(index, time, started, ended, pause_time);
        self.attempt_history.push(attempt);
    }

    /// Clears the speedrun.com Run ID of this Run, as the current Run does not
    /// reflect the run on speedrun.com anymore. This may be the case if a new
    /// Personal Best is achieved for example.
    #[inline]
    pub fn clear_run_id(&mut self) {
        self.metadata.set_run_id("");
    }

    /// Adds a new custom comparison. If a custom comparison with that name
    /// already exists, it is not added.
    #[inline]
    pub fn add_custom_comparison<S>(&mut self, comparison: S) -> ComparisonResult<()>
    where
        S: PopulateString,
    {
        self.validate_comparison_name(comparison.as_str())?;
        self.custom_comparisons.push(comparison.into_string());
        Ok(())
    }

    /// Recalculates all the comparison times the Comparison Generators provide.
    #[inline]
    pub fn regenerate_comparisons(&mut self) {
        for generator in &mut self.comparison_generators.0 {
            generator.generate(&mut self.segments, &self.attempt_history);
        }
    }

    /// Returns a file name (without the extension) suitable for this Run that
    /// is built the following way:
    ///
    /// Game Name - Category Name
    ///
    /// If either is empty, the dash is omitted. Special characters that cause
    /// problems in file names are also omitted. If an extended category name is
    /// used, the variables of the category are appended in a parenthesis.
    pub fn extended_file_name(&self, use_extended_category_name: bool) -> String {
        let mut extended_name = self.extended_name(use_extended_category_name).into_owned();
        extended_name
            .retain(|c| !matches!(c, '\\' | '/' | ':' | '*' | '?' | '"' | '<' | '>' | '|'));
        extended_name
    }

    /// Returns a name suitable for this Run that is built the following way:
    ///
    /// Game Name - Category Name
    ///
    /// If either is empty, the dash is omitted. If an extended category name is
    /// used, the variables of the category are appended in a parenthesis.
    pub fn extended_name(&self, use_extended_category_name: bool) -> Cow<'_, str> {
        let mut name = Cow::Borrowed(self.game_name());

        let category_name = if use_extended_category_name {
            Cow::Owned(self.extended_category_name(false, false, true).to_string())
        } else {
            Cow::Borrowed(self.category_name())
        };

        if !category_name.is_empty() {
            if !name.is_empty() {
                let name = name.to_mut();
                name.push_str(" - ");
                name.push_str(&category_name);
            } else {
                name = category_name;
            }
        }

        name
    }

    /// Returns an extended category name that possibly includes the region,
    /// platform and variables, depending on the arguments provided. The
    /// returned object implements `Display` where it lazily formats the
    /// extended category name. An extended category name may look like this:
    ///
    /// Any% (No Tuner, JPN, Wii Emulator)
    pub const fn extended_category_name(
        &self,
        show_region: bool,
        show_platform: bool,
        show_variables: bool,
    ) -> ExtendedCategoryName<'_> {
        ExtendedCategoryName {
            run: self,
            show_region,
            show_platform,
            show_variables,
        }
    }

    /// Returns the maximum index currently in use by the Attempt History. This
    /// mostly serves as a helper function for the Timer.
    pub fn max_attempt_history_index(&self) -> Option<i32> {
        self.attempt_history().iter().map(Attempt::index).max()
    }

    /// Applies some fixing algorithms on the Run. This includes fixing the
    /// comparison times and history, removing duplicates in the segment
    /// histories and removing empty times.
    pub fn fix_splits(&mut self) {
        for method in TimingMethod::all() {
            self.fix_comparison_times_and_history(method);
        }
        self.remove_duplicates();
        self.remove_none_values();
        self.reattach_unattached_segment_history_elements();
    }

    /// Clears out the Attempt History and the Segment Histories of all the segments.
    pub fn clear_history(&mut self) {
        self.attempt_history.clear();
        for segment in &mut self.segments {
            segment.segment_history_mut().clear();
        }
    }

    /// Clears out the Attempt History, the Segment Histories, all the times,
    /// sets the Attempt Count to 0 and clears the speedrun.com run id
    /// association. All Custom Comparisons other than `Personal Best` are
    /// deleted as well.
    pub fn clear_times(&mut self) {
        self.clear_history();
        self.custom_comparisons.retain(|c| c == personal_best::NAME);
        for segment in &mut self.segments {
            segment.comparisons_mut().clear();
            segment.set_best_segment_time(Time::default());
        }
        self.attempt_count = 0;
        self.clear_run_id();
    }

    fn fix_comparison_times_and_history(&mut self, method: TimingMethod) {
        // Remove negative Best Segment Times
        for segment in &mut self.segments {
            if segment.best_segment_time_mut()[method].map_or(false, |t| t < TimeSpan::zero()) {
                segment.best_segment_time_mut()[method] = None;
            }
        }

        for segment in &mut self.segments {
            fix_history_from_none_best_segments(segment, method);
        }

        for comparison in &self.custom_comparisons {
            let mut previous_time = TimeSpan::zero();
            for segment in &mut self.segments {
                if let Some(mut time) = segment.comparison_mut(comparison)[method] {
                    // Prevent comparison times from decreasing from one split to the next
                    if time < previous_time {
                        time = previous_time;
                        segment.comparison_mut(comparison)[method] = Some(time);
                    }

                    // Fix Best Segment time if the PB segment is faster
                    if comparison == personal_best::NAME {
                        let current_segment = time - previous_time;
                        if segment.best_segment_time()[method].map_or(true, |t| t > current_segment)
                        {
                            segment.best_segment_time_mut()[method] = Some(current_segment);
                        }
                    }

                    previous_time = time;
                }
            }
        }

        for segment in &mut self.segments {
            fix_history_from_best_segment_times(segment, method);
        }
    }

    fn remove_none_values(&mut self) {
        let mut cache = Vec::new();
        if let Some(min_index) = self.min_segment_history_index() {
            let max_index = self.max_attempt_history_index().unwrap_or(0) + 1;
            for run_index in min_index..max_index {
                for index in 0..self.len() {
                    if let Some(element) = self.segments[index].segment_history().get(run_index) {
                        if element.real_time.is_none() && element.game_time.is_none() {
                            cache.push(run_index);
                        } else {
                            cache.clear();
                        }
                    } else {
                        // Remove None times in history that aren't followed by a non-None time
                        self.remove_items_from_cache(index, &mut cache);
                    }
                }
                let len = self.len();
                self.remove_items_from_cache(len, &mut cache);
            }
        }
    }

    fn remove_duplicates(&mut self) {
        let mut rta_set = HashSet::new();
        let mut igt_set = HashSet::new();

        for segment in self.segments_mut() {
            let history = segment.segment_history_mut();

            rta_set.clear();
            igt_set.clear();

            for &(_, time) in history.iter_actual_runs() {
                if let Some(time) = time.real_time {
                    rta_set.insert(time);
                }
                if let Some(time) = time.game_time {
                    igt_set.insert(time);
                }
            }

            history.retain(|&(index, time)| {
                if index >= 1 {
                    return true;
                }

                let (mut is_none, mut is_unique) = (true, false);
                if let Some(time) = time.real_time {
                    is_unique |= rta_set.insert(time);
                    is_none = false;
                }

                if let Some(time) = time.game_time {
                    is_unique |= igt_set.insert(time);
                    is_none = false;
                }

                is_none || is_unique
            });
        }
    }

    fn remove_items_from_cache(&mut self, index: usize, cache: &mut Vec<i32>) {
        let ind = index - cache.len();
        for (index, segment) in cache.drain(..).zip(self.segments_mut()[ind..].iter_mut()) {
            segment.segment_history_mut().remove(index);
        }
    }

    /// Returns the minimum index in use by all the Segment Histories. `None` is
    /// returned if the Run has no segments.
    pub fn min_segment_history_index(&self) -> Option<i32> {
        self.segments
            .iter()
            .map(|s| s.segment_history().min_index())
            .min()
    }

    /// Fixes the Segment History by calculating the segment times from the
    /// Personal Best times and adding those to the Segment History.
    pub fn import_pb_into_segment_history(&mut self) {
        if let Some(mut index) = self.min_segment_history_index() {
            for timing_method in TimingMethod::all() {
                index -= 1;
                let mut prev_time = TimeSpan::zero();

                for segment in self.segments_mut() {
                    // Import the PB splits into the history
                    let pb_time = segment.personal_best_split_time()[timing_method];
                    let time = Time::new()
                        .with_timing_method(timing_method, pb_time.map(|p| p - prev_time));
                    segment.segment_history_mut().insert(index, time);

                    if let Some(time) = pb_time {
                        prev_time = time;
                    }
                }
            }
        }
    }

    /// Fixes a segment's Segment History by adding its Best Segment Time to its
    /// Segment History.
    ///
    /// # Panics
    ///
    /// This panics if the segment index provided is out of bounds.
    pub fn import_best_segment(&mut self, segment_index: usize) {
        let best_segment_time = self.segments[segment_index].best_segment_time();
        if best_segment_time.real_time.is_some() || best_segment_time.game_time.is_some() {
            // We can unwrap here because due to the fact that we can access the
            // best_segment_time of some segment, at least one exists.
            let index = self.min_segment_history_index().unwrap() - 1;
            self.segments[segment_index]
                .segment_history_mut()
                .insert(index, best_segment_time);
        }
    }

    /// Updates the Segment History by adding the split times of the most recent
    /// attempt up to the provided current split index to the Segment History.
    ///
    /// # Panics
    ///
    /// This panics if there is no attempt in the Attempt History.
    pub fn update_segment_history(&mut self, current_split_index: usize) {
        let mut last_split_time = Time::zero();

        let segments = self.segments.iter_mut().take(current_split_index);
        let index = self
            .attempt_history
            .last()
            .expect("There is no attempt in the Attempt History.")
            .index();

        for segment in segments {
            let split_time = segment.split_time();
            let segment_time = Time::op(split_time, last_split_time, |a, b| a - b);
            segment.segment_history_mut().insert(index, segment_time);
            if let Some(time) = split_time.real_time {
                last_split_time.real_time = Some(time);
            }
            if let Some(time) = split_time.game_time {
                last_split_time.game_time = Some(time);
            }
        }
    }

    /// Checks a given name against the current comparisons in the Run to
    /// ensure that it is valid for use.
    pub fn validate_comparison_name(&self, new: &str) -> ComparisonResult<()> {
        if new.starts_with(RACE_COMPARISON_PREFIX) {
            Err(ComparisonError::NameStartsWithRace)
        } else if self.comparisons().any(|c| c == new) {
            Err(ComparisonError::DuplicateName)
        } else {
            Ok(())
        }
    }

    fn reattach_unattached_segment_history_elements(&mut self) {
        let max_id = self.max_attempt_history_index().unwrap_or_default();
        let mut min_id = self.min_segment_history_index().unwrap_or_default();

        while let Some(unattached_id) = self
            .segments
            .iter()
            .filter_map(|s| s.segment_history().try_get_max_index())
            .filter(|&i| i > max_id)
            .max()
        {
            let reassign_id = min_id - 1;

            for segment in self.segments_mut() {
                let history = segment.segment_history_mut();
                if let Some(time) = history.remove(unattached_id) {
                    history.insert(reassign_id, time);
                }
            }

            min_id = reassign_id;
        }
    }
}

impl Default for Run {
    fn default() -> Self {
        Run::new()
    }
}

fn fix_history_from_none_best_segments(segment: &mut Segment, method: TimingMethod) {
    // Only do anything if the Best Segment Time is gone for the Segment in question
    if segment.best_segment_time()[method].is_none() {
        // Keep only the skipped segments
        segment
            .segment_history_mut()
            .retain(|&(_, time)| time[method].is_none());
    }
}

fn fix_history_from_best_segment_times(segment: &mut Segment, method: TimingMethod) {
    if let Some(best_segment) = segment.best_segment_time()[method] {
        for (_, time) in segment.segment_history_mut().iter_mut() {
            // Make sure no times in the history are lower than the Best Segment
            if let Some(time) = &mut time[method] {
                if *time < best_segment {
                    *time = best_segment;
                }
            }
        }
    }
}

/// Iterator that iterates over all the comparisons. This includes both the
/// custom comparisons defined by the user and the Comparison Generators.
pub struct ComparisonsIter<'a> {
    custom: &'a [String],
    generators: &'a [Box<dyn ComparisonGenerator>],
}

impl<'a> Iterator for ComparisonsIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if let Some((a, b)) = self.custom.split_first() {
            self.custom = b;
            Some(a)
        } else if let Some((a, b)) = self.generators.split_first() {
            self.generators = b;
            Some(a.name())
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.custom.len() + self.generators.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for ComparisonsIter<'_> {}

/// Lazily formats an extended category name via the `Display` trait. It's a
/// category name that possibly includes the region, platform and variables,
/// depending on the arguments provided. An extended category name may look like
/// this:
///
/// Any% (No Tuner, JPN, Wii Emulator)
pub struct ExtendedCategoryName<'run> {
    run: &'run Run,
    show_region: bool,
    show_platform: bool,
    show_variables: bool,
}

impl fmt::Display for ExtendedCategoryName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let category_name = self.run.category_name.as_str();

        if category_name.is_empty() {
            return Ok(());
        }

        let mut is_empty = true;
        let mut has_pushed = false;

        let (before, after_parenthesis) = if let Some((i, u)) = self
            .run
            .category_name
            .find('(')
            .and_then(|i| category_name[i..].find(')').map(|u| (i, i + u)))
        {
            is_empty = u == i + 1;
            category_name.split_at(u)
        } else {
            (category_name, "")
        };
        f.write_str(before)?;

        let mut push = |values: &[&str]| {
            if is_empty {
                if !has_pushed {
                    f.write_str(" (")?;
                }
                is_empty = false;
            } else {
                f.write_str(", ")?;
            }
            for value in values {
                f.write_str(value)?;
            }
            has_pushed = true;
            Ok(())
        };

        if self.show_variables {
            for (name, value) in self.run.metadata.speedrun_com_variables() {
                let name = name.trim_end_matches('?');

                if unicase::eq(value.as_str(), "yes") {
                    push(&[name])?;
                } else if unicase::eq(value.as_str(), "no") {
                    push(&["No ", value])?;
                } else {
                    push(&[value])?;
                }
            }
        }

        if self.show_region {
            let region = self.run.metadata.region_name();
            if !region.is_empty() {
                push(&[region])?;
            }
        }

        if self.show_platform {
            let platform = self.run.metadata.platform_name();
            let uses_emulator = self.run.metadata.uses_emulator();

            match (!platform.is_empty(), uses_emulator) {
                (true, true) => push(&[platform, " Emulator"])?,
                (true, false) => push(&[platform])?,
                (false, true) => push(&["Emulator"])?,
                _ => (),
            }
        }

        if !after_parenthesis.is_empty() {
            f.write_str(after_parenthesis)?;
        } else if !is_empty {
            f.write_str(")")?;
        }

        Ok(())
    }
}
