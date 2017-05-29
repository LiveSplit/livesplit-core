use std::collections::BTreeMap;
use std::collections::btree_map::{Iter, Range};
use std::cmp::min;
use Time;

#[derive(Clone, Default, Debug)]
pub struct SegmentHistory(BTreeMap<i32, Time>);

impl SegmentHistory {
    pub fn try_get_min_index(&self) -> Option<i32> {
        // This assumes that the first element is the minimum,
        // which is only true for an ordered map
        self.0.keys().next().map(|&m| m)
    }

    /// Defaults to a maximum of 1
    pub fn min_index(&self) -> i32 {
        self.try_get_min_index().map_or(1, |m| min(m, 1))
    }

    pub fn try_get_max_index(&self) -> Option<i32> {
        // This assumes that the last element is the maximum,
        // which is only true for an ordered map
        self.0.keys().rev().next().map(|&m| m)
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
    pub fn get_mut(&mut self, index: i32) -> Option<&mut Time> {
        self.0.get_mut(&index)
    }

    #[inline]
    pub fn remove(&mut self, index: i32) {
        self.0.remove(&index);
    }

    #[inline]
    pub fn iter(&self) -> Iter<i32, Time> {
        IntoIterator::into_iter(self)
    }

    #[inline]
    pub fn iter_actual_runs(&self) -> Range<i32, Time> {
        self.0.range(1..)
    }
}

impl<'a> IntoIterator for &'a SegmentHistory {
    type Item = (&'a i32, &'a Time);
    type IntoIter = Iter<'a, i32, Time>;

    fn into_iter(self) -> Iter<'a, i32, Time> {
        self.0.iter()
    }
}
