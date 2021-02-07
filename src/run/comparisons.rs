use crate::{platform::prelude::*, Time};

// We use a Vec here because a HashMap would require hashing the comparison and
// then comparing the comparison with the string at the index calculated from
// the hash. This means at least two full iterations over the string are
// necessary, with one of them being somewhat expensive due to the hashing. Most
// of the time, it is faster to just iterate over the few comparisons we have and
// compare them directly. Most will be rejected right away since the first byte
// doesn't even match, so in the end, you'll end up with less than two full
// iterations over the string. In addition, Personal Best will be the first
// comparison in the list most of the time, and that's the one we want to look
// up most often anyway.
//
// One additional reason for doing this is that the ahash that was calculated
// for the HashMap uses 128-bit multiplications, which regressed a lot in Rust
// 1.44 for targets where the `compiler-builtins` helpers were used.
// https://github.com/rust-lang/rust/issues/73135
//
// We could potentially look into interning our comparisons in the future, which
// could yield even better performance.

/// A collection of a segment's comparison times.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Comparisons(Vec<(Box<str>, Time)>);

impl Comparisons {
    fn index_of(&self, comparison: &str) -> Option<usize> {
        Some(
            self.0
                .iter()
                .enumerate()
                .find(|(_, (c, _))| &**c == comparison)?
                .0,
        )
    }

    /// Accesses the time for the comparison specified.
    pub fn get(&self, comparison: &str) -> Option<Time> {
        Some(self.0[self.index_of(comparison)?].1)
    }

    /// Accesses the time for the comparison specified, or inserts a new empty
    /// one if there is none.
    pub fn get_or_insert_default(&mut self, comparison: &str) -> &mut Time {
        if let Some(index) = self.index_of(comparison) {
            &mut self.0[index].1
        } else {
            self.0.push((comparison.into(), Time::default()));
            &mut self.0.last_mut().unwrap().1
        }
    }

    /// Sets the time for the comparison specified.
    pub fn set(&mut self, comparison: &str, time: Time) {
        *self.get_or_insert_default(comparison) = time;
    }

    /// Removes the time for the comparison specified and returns it if there
    /// was one.
    pub fn remove(&mut self, comparison: &str) -> Option<Time> {
        let index = self.index_of(comparison)?;
        let (_, time) = self.0.remove(index);
        Some(time)
    }

    /// Clears all the comparisons and their times.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Iterates over all the comparisons and their times.
    pub fn iter(&self) -> impl Iterator<Item = &(Box<str>, Time)> + '_ {
        self.0.iter()
    }

    /// Mutably iterates over all the comparisons and their times. Be careful
    /// when modifying the comparison name. Having duplicates will likely cause
    /// problems.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (Box<str>, Time)> + '_ {
        self.0.iter_mut()
    }
}
