use crate::platform::prelude::*;
use crate::Time;
use core::cmp::min;
use core::slice::{Iter, IterMut};

/// Stores the segment times achieved for a certain segment. Each segment is
/// tagged with an index. Only segment times with an index larger than 0 are
/// considered times actually achieved by the runner, while the others are
/// artifacts of route changes and similar algorithmic changes.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct SegmentHistory(Vec<(i32, Time)>);

impl SegmentHistory {
    /// Returns the minimum index of all the segment times. Returns `None` if
    /// there's no segment times in this history.
    pub fn try_get_min_index(&self) -> Option<i32> {
        // This assumes that the first element is the minimum,
        // which is only true for an ordered map.
        Some(self.0.first()?.0)
    }

    /// Returns the minimum index of all the segment times. If there are no
    /// segment times or there are only indices above 1, then 1 is returned
    /// instead.
    pub fn min_index(&self) -> i32 {
        self.try_get_min_index().map_or(1, |m| min(m, 1))
    }

    /// Returns the maximum index of all the segment times. Returns `None` if
    /// there's no segment times in this history.
    pub fn try_get_max_index(&self) -> Option<i32> {
        // This assumes that the last element is the maximum,
        // which is only true for an ordered map.
        Some(self.0.last()?.0)
    }

    fn get_pos(&self, index: i32) -> Result<usize, usize> {
        self.0.binary_search_by_key(&index, |&(i, _)| i)
    }

    /// Inserts a new segment time into the Segment History, with the index
    /// provided. If there's already a segment time with that index, the time is
    /// not inserted.
    #[inline]
    pub fn insert(&mut self, index: i32, time: Time) {
        if let Err(pos) = self.get_pos(index) {
            self.0.insert(pos, (index, time));
        }
    }

    /// Accesses the segment time with the given index. If there's no segment
    /// time with that index, `None` is returned instead.
    #[inline]
    pub fn get(&self, index: i32) -> Option<Time> {
        let pos = self.get_pos(index).ok()?;
        Some(self.0.get(pos)?.1)
    }

    /// Grants mutable access to the segment time with the given index. If
    /// there's no segment time with that index, `None` is returned instead.
    #[inline]
    pub fn get_mut(&mut self, index: i32) -> Option<&mut Time> {
        let pos = self.get_pos(index).ok()?;
        Some(&mut self.0.get_mut(pos)?.1)
    }

    /// Removes the segment time with the given index. If it doesn't exist,
    /// nothing is done.
    #[inline]
    pub fn remove(&mut self, index: i32) -> Option<Time> {
        let pos = self.get_pos(index).ok()?;
        Some(self.0.remove(pos).1)
    }

    /// Removes all the segment times from the Segment History.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Removes all the segment times from the Segment History, where the given
    /// closure returns `false`.
    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&(i32, Time)) -> bool,
    {
        self.0.retain(f);
    }

    /// Iterates over all the segment times and their indices.
    #[inline]
    pub fn iter(&self) -> Iter<'_, (i32, Time)> {
        IntoIterator::into_iter(self)
    }

    /// Mutably iterates over all the segment times and their indices.
    ///
    /// # Warning
    ///
    /// While you are allowed to change the indices, you need to ensure they
    /// stay in rising order.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, (i32, Time)> {
        self.0.iter_mut()
    }

    /// Iterates over the actual segment times achieved by the runner. Segment
    /// times created by route changes or other algorithmic changes are filtered
    /// out.
    #[inline]
    pub fn iter_actual_runs(&self) -> Iter<'_, (i32, Time)> {
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
