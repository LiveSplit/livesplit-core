use crate::platform::prelude::*;
use alloc::collections::BinaryHeap;
use core::usize;

/// With a Fuzzy List, you can implement a fuzzy searching algorithm. The list
/// stores all the items that can be searched for. With the `search` method you
/// can then execute the actual fuzzy search which returns a list of all the
/// elements found. This can be used to implement searching in a list of games.
///
/// # Examples
///
/// ```
/// use livesplit_core::run::editor::FuzzyList;
/// let mut list = FuzzyList::new();
/// list.push("Hello");
/// list.push("World");
/// assert_eq!(list.search("ORL", 10), ["World"]);
/// ```
#[derive(Default)]
pub struct FuzzyList {
    list: Vec<(String, String)>,
}

impl FuzzyList {
    /// Creates a new Fuzzy List.
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds a new element to the list.
    pub fn push(&mut self, element: &str) {
        self.list
            .push((element.to_string(), element.to_lowercase()));
    }

    /// Searches for the pattern provided in the list. A list of all the
    /// matching elements is returned. The returned list has a maximum amount of
    /// elements provided to this method.
    pub fn search<'a>(&'a self, pattern: &str, max: usize) -> Vec<&'a str> {
        let pattern = pattern.to_lowercase();
        let mut heap = BinaryHeap::new();
        for (element, element_lower) in &self.list {
            if let Some(score) = match_against(&pattern, element_lower) {
                if heap.len() >= max {
                    heap.pop();
                }
                heap.push((usize::MAX - score, element));
            }
        }
        heap.into_sorted_vec()
            .iter()
            .map(|(_, s)| s.as_str())
            .collect()
    }
}

fn match_against(pattern: &str, text: &str) -> Option<usize> {
    let (mut current_score, mut total_score) = (0, 0);
    let mut pattern_chars = pattern.chars();
    let mut pattern_char = pattern_chars.next();

    for c in text.chars() {
        if pattern_char == Some(c) {
            pattern_char = pattern_chars.next();
            current_score = 1 + 2 * current_score;
        } else {
            current_score = 0;
        }
        total_score += current_score;
    }

    if pattern_char.is_none() {
        if pattern == text {
            Some(usize::MAX)
        } else {
            Some(total_score)
        }
    } else {
        None
    }
}
