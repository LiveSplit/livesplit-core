use std::path::PathBuf;
use std::cmp::max;
use {AtomicDateTime, TimeSpan, Time, TimingMethod, Attempt, RunMetadata, Segment};
use odds::vec::VecFindRemove;

pub const PERSONAL_BEST_COMPARISON_NAME: &'static str = "Personal Best";

#[derive(Clone, Debug)]
pub struct Run {
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
}

impl Run {
    #[inline]
    pub fn new(segments: Vec<Segment>) -> Self {
        Run {
            game_name: String::from(""),
            category_name: String::from(""),
            offset: TimeSpan::zero(),
            attempt_count: 0,
            attempt_history: Vec::new(),
            metadata: RunMetadata::new(),
            has_changed: false,
            path: None,
            segments: segments,
            custom_comparisons: vec![PERSONAL_BEST_COMPARISON_NAME.into()],
        }
    }

    #[inline]
    pub fn game_name(&self) -> &str {
        &self.game_name
    }

    #[inline]
    pub fn set_game_name<S>(&mut self, name: S)
        where S: AsRef<str>
    {
        self.game_name.clear();
        self.game_name.push_str(name.as_ref());
    }

    #[inline]
    pub fn category_name(&self) -> &str {
        &self.category_name
    }

    #[inline]
    pub fn set_category_name<S>(&mut self, name: S)
        where S: AsRef<str>
    {
        self.category_name.clear();
        self.category_name.push_str(name.as_ref());
    }

    #[inline]
    pub fn attempt_count(&self) -> u32 {
        self.attempt_count
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
    pub fn segments_mut(&mut self) -> &mut [Segment] {
        &mut self.segments
    }

    #[inline]
    pub fn segment(&self, index: usize) -> &Segment {
        &self.segments[index]
    }

    #[inline]
    pub fn attempt_history(&self) -> &[Attempt] {
        &self.attempt_history
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

    pub fn add_attempt(&mut self,
                       time: Time,
                       started: Option<AtomicDateTime>,
                       ended: Option<AtomicDateTime>) {
        let index = self.attempt_history.iter().map(Attempt::index).max().unwrap_or(0);
        let index = max(0, index + 1);
        let attempt = Attempt::new(index, time, started, ended);
        self.attempt_history.push(attempt);
    }

    #[inline]
    pub fn clear_run_id(&mut self) {
        self.metadata.set_run_id(String::new());
    }

    fn max_attempt_history_index(&self) -> Option<i32> {
        self.attempt_history().iter().map(|x| x.index()).max()
    }

    pub fn fix_splits(&mut self) {
        for &method in &TimingMethod::all() {
            self.fix_segment_history(method);
            self.fix_comparison_times(method);
            self.remove_duplicates(method);
        }
        self.remove_null_values();
    }

    fn fix_segment_history(&mut self, method: TimingMethod) {
        let max_index = self.max_attempt_history_index().unwrap_or(0) + 1;
        for segment in self.segments_mut() {
            for run_index in segment.segment_history().min_index()..max_index {
                if let Some(mut history_time) = segment.segment_history().get(run_index) {
                    // Make sure no times in the history are lower than the Best Segment
                    if TimeSpan::option_op(segment.best_segment_time()[method],
                                           history_time[method],
                                           |b, h| h < b)
                        .unwrap_or(false) {
                        history_time[method] = segment.best_segment_time()[method];
                    }

                    // If the Best Segment is gone, clear the history
                    if segment.best_segment_time()[method].is_none() &&
                       history_time[method].is_some() {
                        segment.segment_history_mut().remove(run_index);
                    } else {
                        segment.segment_history_mut().insert(run_index, history_time);
                    }
                }
            }
        }
    }

    fn fix_comparison_times(&mut self, method: TimingMethod) {
        for comparison in &self.custom_comparisons {
            let mut previous_time = TimeSpan::zero();
            for segment in &mut self.segments {
                if let Some(time) = segment.comparison_mut(comparison)[method] {
                    // Prevent comparison times from decreasing from one split to the next
                    if time < previous_time {
                        segment.comparison_mut(comparison)[method] = Some(previous_time);
                    }

                    // Fix Best Segment time if the PB segment is faster
                    let current_segment = time - previous_time;
                    if comparison == PERSONAL_BEST_COMPARISON_NAME {
                        let time = &mut segment.best_segment_time()[method];
                        if time.map_or(true, |t| t > current_segment) {
                            *time = Some(current_segment);
                        }
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
                        cache.push((run_index, element));
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
            cache.clear();
        }
    }

    fn remove_duplicates(&mut self, method: TimingMethod) {
        for index in 0..self.len() {
            let mut history = self.segments[index]
                .segment_history()
                .iter()
                .filter_map(|(_, t)| t[method])
                .collect::<Vec<_>>();

            for run_index in self.segments[index].segment_history().min_index()..1 {
                // Remove elements in the imported Segment History if they're duplicates of the real Segment History
                if let Some(element) = self.segments[index].segment_history().get(run_index) {
                    if let Some(element) = element[method] {
                        if history.iter().filter(|&&x| x == element).count() > 1 {
                            self.segments[index].segment_history_mut().remove(run_index);
                            history.find_remove(&element);
                        }
                    }
                }
            }
        }
    }

    fn remove_items_from_cache(&mut self, index: usize, cache: &mut Vec<(i32, Time)>) {
        let mut ind = index - cache.len();
        for &mut (index, _) in cache.iter_mut() {
            self.segments[ind].segment_history_mut().remove(index);
            ind += 1;
        }
        cache.clear();
    }

    fn min_segment_history_index(&self) -> i32 {
        self.segments.iter().map(|s| s.segment_history().min_index()).min().unwrap()
    }

    pub fn import_segment_history(&mut self) {
        let index = self.min_segment_history_index();
        for &timing_method in &TimingMethod::all() {
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

    pub fn update_segment_history(&mut self, current_split_index: isize) {
        let mut split_time = Time::zero();

        let segments = self.segments
            .iter_mut()
            .take(max(0, current_split_index) as usize);

        for split in segments {
            let time = Time::op(split.split_time(), split_time, |a, b| a - b);
            let index = self.attempt_history.last().unwrap().index();
            split.segment_history_mut().insert(index, time);
            if let Some(time) = time.real_time {
                split_time.real_time = Some(time);
            }
            if let Some(time) = time.game_time {
                split_time.game_time = Some(time);
            }
        }
    }
}
