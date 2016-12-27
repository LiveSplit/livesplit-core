use std::path::PathBuf;
use std::cmp::max;
use {AtomicDateTime, TimeSpan, Time, TimingMethod, Attempt, RunMetadata, Segment};
use odds::vec::VecFindRemove;

#[derive(Default, Clone)]
pub struct Run {
    game_name: String,
    category_name: String,
    offset: TimeSpan,
    attempt_count: u64,
    attempt_history: Vec<Attempt>,
    metadata: RunMetadata,
    has_changed: bool,
    path: Option<PathBuf>,
    segments: Vec<Segment>,
}

impl Run {
    #[inline]
    pub fn new() -> Self {
        Default::default()
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

    fn fix_comparison_times(&mut self, _method: TimingMethod) {
        // TODO Implement Comparisons
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
