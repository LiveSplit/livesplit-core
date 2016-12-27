use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::cmp::min;
use Time;

#[derive(Clone, Default, Debug)]
pub struct SegmentHistory(HashMap<i32, Time>);

impl SegmentHistory {
    pub fn min_index(&self) -> i32 {
        self.0.keys().min().map_or(1, |&m| min(m, 1))
    }

    #[inline]
    pub fn insert(&mut self, index: i32, time: Time) {
        self.0.insert(index, time);
    }

    #[inline]
    pub fn get(&self, index: i32) -> Option<Time> {
        self.0.get(&index).cloned()
    }

    #[inline]
    pub fn remove(&mut self, index: i32) {
        self.0.remove(&index);
    }

    #[inline]
    pub fn iter(&self) -> Iter<i32, Time> {
        IntoIterator::into_iter(self)
    }
}

impl<'a> IntoIterator for &'a SegmentHistory {
    type Item = (&'a i32, &'a Time);
    type IntoIter = Iter<'a, i32, Time>;

    fn into_iter(self) -> Iter<'a, i32, Time> {
        self.0.iter()
    }
}
