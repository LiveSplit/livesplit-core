use std::collections::HashMap;
use std::cmp::min;
use Time;

#[derive(Clone, Default)]
pub struct SegmentHistory(HashMap<usize, Time>);

impl SegmentHistory {
    pub fn min_index(&self) -> usize {
        self.0.keys().min().map_or(1, |&m| min(m, 1))
    }
}
