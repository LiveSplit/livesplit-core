use std::borrow::Cow;
use std::path::PathBuf;
use std::cmp::max;
use {AtomicDateTime, Attempt, Image, RunMetadata, Segment, Time, TimeSpan, TimingMethod};
use comparison::{default_generators, personal_best, ComparisonGenerator};
use odds::vec::VecFindRemove;
use unicase;

#[derive(Clone, Debug)]
pub struct Run {
    game_icon: Image,
    game_name: String,
    category_name: String,
    offset: TimeSpan,
    attempt_count: u32,
    attempt_history: Vec<Attempt>,
    metadata: RunMetadata,
    has_changed: bool,
    path: Option<PathBuf>,
    segments: Vec<Segment>,
    custom_comparisons: Vec<String>,
    comparison_generators: Vec<Box<ComparisonGenerator>>,
}

impl Run {
    #[inline]
    pub fn new() -> Self {
        let mut run = Run {
            game_icon: Image::default(),
            game_name: String::new(),
            category_name: String::new(),
            offset: TimeSpan::zero(),
            attempt_count: 0,
            attempt_history: Vec::new(),
            metadata: RunMetadata::new(),
            has_changed: false,
            path: None,
            segments: Vec::new(),
            custom_comparisons: vec![personal_best::NAME.to_string()],
            comparison_generators: default_generators(),
        };
        run.regenerate_comparisons();
        run
    }

    #[inline]
    pub fn game_name(&self) -> &str {
        &self.game_name
    }

    #[inline]
    pub fn set_game_name<S>(&mut self, name: S)
    where
        S: AsRef<str>,
    {
        self.game_name.clear();
        self.game_name.push_str(name.as_ref());
    }

    #[inline]
    pub fn game_icon(&self) -> &Image {
        &self.game_icon
    }

    #[inline]
    pub fn set_game_icon<D: Into<Image>>(&mut self, image: D) {
        self.game_icon = image.into();
    }

    #[inline]
    pub fn category_name(&self) -> &str {
        &self.category_name
    }

    #[inline]
    pub fn set_category_name<S>(&mut self, name: S)
    where
        S: AsRef<str>,
    {
        self.category_name.clear();
        self.category_name.push_str(name.as_ref());
    }

    #[inline]
    pub fn set_path(&mut self, path: Option<PathBuf>) {
        self.path = path;
    }

    #[inline]
    pub fn set_offset(&mut self, offset: TimeSpan) {
        self.offset = offset;
    }

    #[inline]
    pub fn attempt_count(&self) -> u32 {
        self.attempt_count
    }

    #[inline]
    pub fn set_attempt_count(&mut self, attempts: u32) {
        self.attempt_count = attempts;
    }

    #[inline]
    pub fn metadata(&self) -> &RunMetadata {
        &self.metadata
    }

    #[inline]
    pub fn metadata_mut(&mut self) -> &mut RunMetadata {
        &mut self.metadata
    }

    #[inline]
    pub fn offset(&self) -> TimeSpan {
        self.offset
    }

    pub fn start_next_run(&mut self) {
        self.attempt_count += 1;
        self.has_changed = true;
    }

    #[inline]
    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    #[inline]
    pub fn segments_mut(&mut self) -> &mut Vec<Segment> {
        &mut self.segments
    }

    #[inline]
    pub fn push_segment(&mut self, segment: Segment) {
        self.segments.push(segment);
    }

    #[inline]
    pub fn segment(&self, index: usize) -> &Segment {
        &self.segments[index]
    }

    #[inline]
    pub fn segment_mut(&mut self, index: usize) -> &mut Segment {
        &mut self.segments[index]
    }

    #[inline]
    pub fn attempt_history(&self) -> &[Attempt] {
        &self.attempt_history
    }

    #[inline]
    pub fn custom_comparisons(&self) -> &[String] {
        &self.custom_comparisons
    }

    #[inline]
    pub fn custom_comparisons_mut(&mut self) -> &mut Vec<String> {
        &mut self.custom_comparisons
    }

    #[inline]
    pub fn comparisons(&self) -> ComparisonsIter {
        ComparisonsIter {
            custom: &self.custom_comparisons,
            generators: &self.comparison_generators,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    #[inline]
    pub fn mark_as_changed(&mut self) {
        self.has_changed = true;
    }

    pub fn add_attempt(
        &mut self,
        time: Time,
        started: Option<AtomicDateTime>,
        ended: Option<AtomicDateTime>,
        pause_time: Option<TimeSpan>,
    ) {
        let index = self.attempt_history
            .iter()
            .map(Attempt::index)
            .max()
            .unwrap_or(0);
        let index = max(0, index + 1);
        self.add_attempt_with_index(time, index, started, ended, pause_time);
    }

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

    #[inline]
    pub fn clear_run_id(&mut self) {
        self.metadata.set_run_id(String::new());
    }

    #[inline]
    pub fn add_custom_comparison<S: Into<String>>(&mut self, comparison: S) {
        let comparison = comparison.into();
        if !self.custom_comparisons.contains(&comparison) {
            self.custom_comparisons.push(comparison);
        }
    }

    #[inline]
    pub fn regenerate_comparisons(&mut self) {
        for generator in &mut self.comparison_generators {
            generator.generate(&mut self.segments, &self.attempt_history);
        }
    }

    pub fn extended_file_name(&self, use_extended_category_name: bool) -> String {
        let extended_name = self.extended_name(use_extended_category_name);

        extended_name
            .chars()
            .filter(|&c| {
                c != '\\' && c != '/' && c != ':' && c != '*' && c != '?' && c != '"' && c != '<' &&
                    c != '>' && c != '|'
            })
            .collect()
    }

    pub fn extended_name(&self, use_extended_category_name: bool) -> String {
        let mut name = self.game_name().to_owned();

        let category_name = if use_extended_category_name {
            self.extended_category_name(false, false, true)
        } else {
            self.category_name().into()
        };

        if !category_name.is_empty() {
            if !name.is_empty() {
                name.push_str(" - ");
            }

            name.push_str(&category_name);
        }

        name
    }

    pub fn extended_category_name(
        &self,
        show_region: bool,
        show_platform: bool,
        show_variables: bool,
    ) -> Cow<str> {
        let mut category_name: Cow<str> = Cow::Borrowed(&self.category_name);
        let mut after_parenthesis = "";

        if category_name.is_empty() {
            return category_name;
        }

        let mut is_empty = true;
        let mut has_pushed = false;

        if let Some((i, u)) = self.category_name
            .find('(')
            .and_then(|i| self.category_name[i..].find(')').map(|u| (i, i + u)))
        {
            category_name = Cow::Borrowed(&self.category_name[..u]);
            is_empty = u == i + 1;
            after_parenthesis = &self.category_name[u..];
        }

        {
            let mut push = |buf: &mut String, values: &[&str]| {
                if is_empty {
                    if !has_pushed {
                        buf.push_str(" (");
                    }
                    is_empty = false;
                } else {
                    buf.push_str(", ");
                }
                for value in values {
                    buf.push_str(value);
                }
                has_pushed = true;
            };

            if show_variables {
                for (name, value) in self.metadata.variables() {
                    let name = name.trim_right_matches('?');

                    if unicase::eq(value.as_str(), "yes") {
                        push(category_name.to_mut(), &[name]);
                    } else if unicase::eq(value.as_str(), "no") {
                        push(category_name.to_mut(), &["No ", value]);
                    } else {
                        push(category_name.to_mut(), &[value]);
                    }
                }
            }

            if show_region {
                let region = self.metadata.region_name();
                if !region.is_empty() {
                    push(category_name.to_mut(), &[region]);
                }
            }

            if show_platform {
                let platform = self.metadata.platform_name();
                let uses_emulator = self.metadata.uses_emulator();

                match (!platform.is_empty(), uses_emulator) {
                    (true, true) => push(category_name.to_mut(), &[platform, " Emulator"]),
                    (true, false) => push(category_name.to_mut(), &[platform]),
                    (false, true) => push(category_name.to_mut(), &["Emulator"]),
                    _ => (),
                }
            }
        }

        if !after_parenthesis.is_empty() {
            if !has_pushed {
                return Cow::Borrowed(&self.category_name);
            }
            category_name.to_mut().push_str(after_parenthesis);
        } else if !is_empty {
            category_name.to_mut().push_str(")");
        }

        category_name
    }

    pub fn max_attempt_history_index(&self) -> Option<i32> {
        self.attempt_history().iter().map(|x| x.index()).max()
    }

    pub fn fix_splits(&mut self) {
        for &method in &TimingMethod::all() {
            self.fix_comparison_times_and_history(method);
            self.remove_duplicates(method);
        }
        self.remove_null_values();
    }

    pub fn clear_history(&mut self) {
        self.attempt_history.clear();
        for segment in &mut self.segments {
            segment.segment_history_mut().clear();
        }
    }

    pub fn clear_times(&mut self) {
        self.clear_history();
        self.custom_comparisons.retain(|c| c == personal_best::NAME);
        for segment in &mut self.segments {
            segment.comparisons_mut().clear();
            segment.set_best_segment_time(Time::default());
        }
        self.attempt_count = 0;
        self.metadata.set_run_id("");
    }

    fn fix_comparison_times_and_history(&mut self, method: TimingMethod) {
        // Remove negative Best Segment Times
        for segment in &mut self.segments {
            if segment.best_segment_time_mut()[method].map_or(false, |t| t < TimeSpan::zero()) {
                segment.best_segment_time_mut()[method] = None;
            }
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
                    let current_segment = time - previous_time;
                    if comparison == personal_best::NAME {
                        fix_history_from_null_best_segments(segment, method);

                        if segment.best_segment_time()[method].map_or(true, |t| t > current_segment)
                        {
                            segment.best_segment_time_mut()[method] = Some(current_segment);
                        }

                        fix_history_from_best_segment_times(segment, method);
                    }
                    previous_time = segment.comparison_mut(comparison)[method].unwrap();
                }
            }
        }
    }

    fn remove_null_values(&mut self) {
        let mut cache = Vec::new();
        let min_index = self.min_segment_history_index();
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
                    // Remove null times in history that aren't followed by a non-null time
                    self.remove_items_from_cache(index, &mut cache);
                }
            }
            let len = self.len();
            self.remove_items_from_cache(len, &mut cache);
        }
    }

    fn remove_duplicates(&mut self, method: TimingMethod) {
        let mut history = Vec::new();

        for segment in self.segments_mut() {
            let segment_history = segment.segment_history_mut();
            history.clear();
            history.extend(segment_history.iter().filter_map(|&(_, t)| t[method]));

            segment_history.retain(|&(index, time)| {
                if index >= 1 {
                    return true;
                }

                if let Some(time) = time[method] {
                    // Remove elements in the imported Segment History if
                    // they're duplicates of the real Segment History
                    if history.iter().filter(|&&x| x == time).take(2).count() > 1 {
                        history.find_remove(&time);
                        return false;
                    }
                }

                true
            });
        }
    }

    fn remove_items_from_cache(&mut self, index: usize, cache: &mut Vec<i32>) {
        let ind = index - cache.len();
        for (index, segment) in cache.drain(..).zip(self.segments_mut()[ind..].iter_mut()) {
            segment.segment_history_mut().remove(index);
        }
    }

    pub fn min_segment_history_index(&self) -> i32 {
        self.segments
            .iter()
            .map(|s| s.segment_history().min_index())
            .min()
            .unwrap()
    }

    pub fn import_segment_history(&mut self) {
        let mut index = self.min_segment_history_index();
        for &timing_method in &[TimingMethod::RealTime, TimingMethod::GameTime] {
            index -= 1;
            let mut prev_time = TimeSpan::zero();

            for segment in self.segments_mut() {
                // Import the PB splits into the history
                let pb_time = segment.personal_best_split_time()[timing_method];
                let time =
                    Time::new().with_timing_method(timing_method, pb_time.map(|p| p - prev_time));
                segment.segment_history_mut().insert(index, time);

                if let Some(time) = pb_time {
                    prev_time = time;
                }
            }
        }
    }

    pub fn import_best_segment(&mut self, segment_index: usize) {
        let best_segment_time = self.segments[segment_index].best_segment_time();
        if best_segment_time.real_time.is_some() || best_segment_time.game_time.is_some() {
            let index = self.min_segment_history_index() - 1;
            self.segments[segment_index]
                .segment_history_mut()
                .insert(index, best_segment_time);
        }
    }

    pub fn update_segment_history(&mut self, current_split_index: usize) {
        let mut last_split_time = Time::zero();

        let segments = self.segments
            .iter_mut()
            .take(current_split_index);
        let index = self.attempt_history.last().unwrap().index();

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
}

fn fix_history_from_null_best_segments(segment: &mut Segment, method: TimingMethod) {
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
        for &mut (_, ref mut time) in segment.segment_history_mut() {
            // Make sure no times in the history are lower than the Best Segment
            if let Some(ref mut time) = time[method] {
                if *time < best_segment {
                    *time = best_segment;
                }
            }
        }
    }
}

pub struct ComparisonsIter<'a> {
    custom: &'a [String],
    generators: &'a [Box<ComparisonGenerator>],
}

impl<'a> Iterator for ComparisonsIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if !self.custom.is_empty() {
            let (a, b) = self.custom.split_at(1);
            self.custom = b;
            Some(&a[0])
        } else if !self.generators.is_empty() {
            let (a, b) = self.generators.split_at(1);
            self.generators = b;
            Some(a[0].name())
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.custom.len() + self.generators.len();
        (len, Some(len))
    }
}

impl<'a> ExactSizeIterator for ComparisonsIter<'a> {}
