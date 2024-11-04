use crate::{
    platform::prelude::*,
    util::not_nan::{NotNaN, PositiveNotNaN, PositiveNotNaNNotZero},
};
use alloc::collections::BinaryHeap;

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
    complex_list: Vec<(Box<str>, Box<str>)>,
    ascii_list: Vec<(Box<str>, Box<str>)>,
}

impl FuzzyList {
    /// Creates a new Fuzzy List.
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds a new element to the list.
    pub fn push(&mut self, element: &str) {
        let lower = element.to_lowercase().into_boxed_str();
        if lower.is_ascii() {
            &mut self.ascii_list
        } else {
            &mut self.complex_list
        }
        .push((element.into(), lower));
    }

    /// Searches for the pattern provided in the list. A list of all the
    /// matching elements is returned. The returned list has a maximum amount of
    /// elements provided to this method.
    pub fn search<'a>(&'a self, pattern: &str, max: usize) -> Vec<&'a str> {
        let pattern = pattern.to_lowercase();
        let mut heap = BinaryHeap::new();

        for (element, element_lower) in &self.complex_list {
            if let Some(score) = match_against(&pattern, element_lower) {
                if heap.len() >= max {
                    heap.pop();
                }
                heap.push((-score, element));
            }
        }

        if pattern.is_ascii() {
            for (element, element_lower) in &self.ascii_list {
                if let Some(score) = match_against_ascii(&pattern, element_lower) {
                    if heap.len() >= max {
                        heap.pop();
                    }
                    heap.push((-score, element));
                }
            }
        } else {
            for (element, element_lower) in &self.ascii_list {
                if let Some(score) = match_against(&pattern, element_lower) {
                    if heap.len() >= max {
                        heap.pop();
                    }
                    heap.push((-score, element));
                }
            }
        }

        heap.into_sorted_vec().iter().map(|(_, s)| &***s).collect()
    }
}

fn match_against(pattern: &str, text: &str) -> Option<NotNaN> {
    let [mut current_score, mut total_score] = [PositiveNotNaN::ZERO; 2];
    let mut pattern_chars = pattern.chars();
    let mut pattern_char = pattern_chars.next();

    for c in text.chars() {
        if pattern_char == Some(c) {
            pattern_char = pattern_chars.next();
            current_score = current_score * PositiveNotNaNNotZero::TWO + PositiveNotNaN::ONE;
        } else {
            current_score = PositiveNotNaN::ZERO;
        }
        total_score = total_score + current_score;
    }

    if pattern_char.is_none() {
        Some(if pattern == text {
            NotNaN::INFINITY
        } else {
            total_score.into()
        })
    } else {
        None
    }
}

fn match_against_ascii(pattern: &str, text: &str) -> Option<NotNaN> {
    let [mut current_score, mut total_score] = [PositiveNotNaN::ZERO; 2];
    let mut pattern_chars = pattern.bytes();
    let mut pattern_char = pattern_chars.next();

    for c in text.bytes() {
        if pattern_char == Some(c) {
            pattern_char = pattern_chars.next();
            current_score = current_score * PositiveNotNaNNotZero::TWO + PositiveNotNaN::ONE;
        } else {
            current_score = PositiveNotNaN::ZERO;
        }
        total_score = total_score + current_score;
    }

    if pattern_char.is_none() {
        Some(if pattern == text {
            NotNaN::INFINITY
        } else {
            total_score.into()
        })
    } else {
        None
    }
}
