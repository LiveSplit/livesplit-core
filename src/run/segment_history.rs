use std::slice::{Iter, IterMut};
use std::cmp::min;
use Time;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct SegmentHistory(Vec<(i32, Time)>);

impl SegmentHistory {
    pub fn try_get_min_index(&self) -> Option<i32> {
        // This assumes that the first element is the minimum,
        // which is only true for an ordered map
        self.0.first().map(|&(i, _)| i)
    }

    /// Defaults to a maximum of 1
    pub fn min_index(&self) -> i32 {
        self.try_get_min_index().map_or(1, |m| min(m, 1))
    }

    pub fn try_get_max_index(&self) -> Option<i32> {
        // This assumes that the last element is the maximum,
        // which is only true for an ordered map
        self.0.last().map(|&(i, _)| i)
    }

    fn get_pos(&self, index: i32) -> Result<usize, usize> {
        self.0.binary_search_by_key(&index, |&(i, _)| i)
    }

    #[inline]
    pub fn insert(&mut self, index: i32, time: Time) {
        if let Err(pos) = self.get_pos(index) {
            self.0.insert(pos, (index, time));
        }
    }

    #[inline]
    pub fn get(&self, index: i32) -> Option<Time> {
        self.get_pos(index)
            .ok()
            .and_then(|p| self.0.get(p))
            .map(|&(_, t)| t)
    }

    #[inline]
    pub fn get_mut(&mut self, index: i32) -> Option<&mut Time> {
        if let Ok(pos) = self.get_pos(index) {
            Some(&mut self.0[pos].1)
        } else {
            None
        }
    }

    #[inline]
    pub fn remove(&mut self, index: i32) {
        if let Ok(pos) = self.get_pos(index) {
            self.0.remove(pos);
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&(i32, Time)) -> bool,
    {
        self.0.retain(f);
    }

    #[inline]
    pub fn iter(&self) -> Iter<(i32, Time)> {
        IntoIterator::into_iter(self)
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<(i32, Time)> {
        IntoIterator::into_iter(self)
    }

    #[inline]
    pub fn iter_actual_runs(&self) -> Iter<(i32, Time)> {
        let start = match self.get_pos(1) {
            Ok(pos) | Err(pos) => pos,
        };
        self.0[start..].iter()
    }
}

impl<'a> IntoIterator for &'a SegmentHistory {
    type Item = &'a (i32, Time);
    type IntoIter = Iter<'a, (i32, Time)>;

    fn into_iter(self) -> Iter<'a, (i32, Time)> {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut SegmentHistory {
    type Item = &'a mut (i32, Time);
    type IntoIter = IterMut<'a, (i32, Time)>;

    fn into_iter(self) -> IterMut<'a, (i32, Time)> {
        self.0.iter_mut()
    }
}
