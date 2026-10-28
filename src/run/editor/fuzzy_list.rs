//! Cached case-normalized search entries and bounded, deterministic ranking.
//! This owns textual similarity, not website data, popularity or UI state.
//! Callers choose a result bound and keep catalogue construction off hot paths.

use crate::{
    platform::prelude::*,
    util::not_nan::{NotNaN, PositiveNotNaN, PositiveNotNaNNotZero},
};
use alloc::collections::BinaryHeap;
use core::cmp::Reverse;

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
        if max == 0 {
            return Vec::new();
        }

        let pattern = pattern.to_lowercase();
        let mut heap = BinaryHeap::new();

        for (element, element_lower) in &self.complex_list {
            if let Some(score) = match_against(&pattern, element_lower) {
                retain_best(&mut heap, max, (-score, element));
            }
        }

        if pattern.is_ascii() {
            for (element, element_lower) in &self.ascii_list {
                if let Some(score) = match_against_ascii(&pattern, element_lower) {
                    retain_best(&mut heap, max, (-score, element));
                }
            }
        } else {
            for (element, element_lower) in &self.ascii_list {
                if let Some(score) = match_against(&pattern, element_lower) {
                    retain_best(&mut heap, max, (-score, element));
                }
            }
        }

        heap.into_sorted_vec().iter().map(|(_, s)| &***s).collect()
    }

    /// Ranks up to `max` nearby titles, including ones that do not contain the
    /// complete query. Exact, prefix and substring matches lead, followed by
    /// query words found in order and word initialisms. Close typos precede weak
    /// scattered-letter abbreviations; short abbreviations still prefer their
    /// match quality over unrelated titles with a small edit distance.
    /// Levenshtein distance orders nearby titles and breaks match-quality ties;
    /// shorter titles and lexical order make the remaining order deterministic.
    ///
    /// Unlike [`Self::search`], this can suggest a different subtitle in the same
    /// series or recover from a mistyped character. No popularity is inferred.
    /// Search storage is bounded by the query workspace and result count;
    /// catalogue strings stay borrowed. Empty queries use internal catalogue
    /// order, and a zero bound always returns an empty result.
    pub fn search_similar<'a>(&'a self, pattern: &str, max: usize) -> Vec<&'a str> {
        if max == 0 {
            return Vec::new();
        }

        let entries = self.ascii_list.iter().chain(&self.complex_list);

        if pattern.is_empty() {
            return entries.take(max).map(|(name, _)| &**name).collect();
        }

        let pattern = pattern.to_lowercase();
        let mut distance = EditDistance::new(&pattern);
        let ascii_pattern = pattern.is_ascii();
        let single_token = !pattern.chars().any(char::is_whitespace);
        let pattern_length = pattern.chars().count();
        // Reuse the initialism workspace across candidates. Only proven UTF-8
        // boundaries become reachable; no title needs a temporary word list.
        let mut initialism_positions = vec![false; pattern.len() + 1];
        let mut heap = BinaryHeap::new();

        for (name, lower) in entries {
            let score = if ascii_pattern && lower.is_ascii() {
                match_against_ascii(&pattern, lower)
            } else {
                match_against(&pattern, lower)
            };
            let edits = distance.measure(lower);
            let kind = if lower.as_ref() == pattern {
                Match::Exact
            } else if lower.starts_with(&pattern) {
                Match::Prefix
            } else if lower.contains(&pattern) {
                Match::Substring
            } else if !single_token && words_in_order(&pattern, lower) {
                Match::Words
            } else if let Some(score) = score.filter(|_| {
                single_token && is_initialism(&pattern, lower, &mut initialism_positions)
            }) {
                Match::Initialism(Reverse(score))
            } else if edits <= MAX_TYPO_EDITS && edits < pattern_length.saturating_sub(edits) {
                // A strict majority of the query must survive. Otherwise tiny
                // abbreviations such as "mc" would favor "Me" over "Minecraft".
                Match::Typo
            } else if let Some(score) = score.filter(|_| single_token) {
                Match::Abbreviation(Reverse(score))
            } else {
                Match::Similar
            };

            retain_best(
                &mut heap,
                max,
                (kind, edits, Reverse(score), lower.chars().count(), name),
            );
        }

        heap.into_sorted_vec()
            .into_iter()
            .map(|(_, _, _, _, name)| &**name)
            .collect()
    }
}

/// Word fragments tolerate intervening punctuation or words in a title, without
/// treating a typo as an abbreviation assembled from distant trailing letters.
/// For example, "zelda ocarina" is strong evidence for "Zelda: Ocarina of Time",
/// while "the legend of zleda" should still use edit distance to recover "Zelda".
fn words_in_order(pattern: &str, mut text: &str) -> bool {
    for word in pattern.split_whitespace() {
        let Some(offset) = text.find(word) else {
            return false;
        };
        text = &text[offset + word.len()..];
    }

    true
}

/// Accept word initials or complete words, with unrelated words allowed between
/// them ("sm64" for "Super Mario 64"). Retain both choices when a word fits both:
/// "aba" needs the initial of "Ab" followed by all of "Ba", not all of "Ab".
fn is_initialism(pattern: &str, text: &str, reachable: &mut [bool]) -> bool {
    reachable.fill(false);
    reachable[0] = true;

    for word in text.split(|character: char| !character.is_alphanumeric()) {
        let Some(initial) = word.chars().next() else {
            continue;
        };

        // Walk backward so newly reached offsets cannot consume the same word
        // twice. Every marked offset ends a matched string or scalar value and
        // is therefore a valid byte boundary, including in non-ASCII queries.
        for offset in (0..pattern.len()).rev() {
            if !reachable[offset] {
                continue;
            }

            let remaining = &pattern[offset..];

            if remaining.starts_with(word) {
                reachable[offset + word.len()] = true;
            }

            if remaining.starts_with(initial) {
                reachable[offset + initial.len_utf8()] = true;
            }
        }

        if reachable[pattern.len()] {
            return true;
        }
    }

    false
}

fn retain_best<T: Ord>(heap: &mut BinaryHeap<T>, max: usize, candidate: T) {
    if heap.len() < max {
        heap.push(candidate);
    } else if let Some(mut worst) = heap.peek_mut()
        && candidate < *worst
    {
        // Replacing only an improvement is essential: discarding the current
        // worst before seeing the candidate evicts even an exact early match
        // when later, weaker API results arrive. PeekMut restores heap order.
        *worst = candidate;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Match {
    Exact,
    Prefix,
    Substring,
    Words,
    // Genuine word initialisms remain useful even when an unrelated short title
    // happens to look like a typo of the abbreviation.
    Initialism(Reverse<NotNaN>),
    Typo,
    // Compare match quality inside this group before the tuple's edit distance.
    // Otherwise scattered letters in a short name can beat a much clearer
    // abbreviation simply because the short name has lower edit distance.
    Abbreviation(Reverse<NotNaN>),
    Similar,
}

// Levenshtein insertions, deletions or substitutions. Two edits also cover an
// adjacent transposition; the additional majority check excludes short codes.
const MAX_TYPO_EDITS: usize = 2;

// Scalar values below this bound are ASCII. A fixed mask table avoids hashing
// for the common case; a query has at most 128 other masks in the fast path.
const ASCII_MASKS: usize = 128;

enum EditDistance {
    Bits {
        ascii: Box<[u128; ASCII_MASKS]>,
        unicode: Vec<(char, u128)>,
        highest: u128,
        length: usize,
    },
    Rows {
        pattern: Vec<char>,
        row: Vec<usize>,
    },
}

impl EditDistance {
    fn new(pattern: &str) -> Self {
        let pattern = pattern.chars().collect::<Vec<_>>();

        if pattern.is_empty() || pattern.len() > u128::BITS as usize {
            let row = (0..=pattern.len()).collect();
            return Self::Rows { pattern, row };
        }

        // Keep the large mask table indirect so the fallback workspace does
        // not inflate the query stack. This one allocation is reused for every
        // candidate in the search, never allocated per game.
        let mut ascii = Box::new([0; ASCII_MASKS]);
        let mut unicode = Vec::<(char, u128)>::new();

        for (index, &character) in pattern.iter().enumerate() {
            let bit = 1u128 << index;

            if character.is_ascii() {
                ascii[character as usize] |= bit;
            } else if let Some((_, mask)) =
                unicode.iter_mut().find(|(value, _)| *value == character)
            {
                *mask |= bit;
            } else {
                unicode.push((character, bit));
            }
        }

        Self::Bits {
            ascii,
            unicode,
            highest: 1u128 << (pattern.len() - 1),
            length: pattern.len(),
        }
    }

    fn measure(&mut self, text: &str) -> usize {
        match self {
            Self::Bits {
                ascii,
                unicode,
                highest,
                length,
            } => {
                // Myers' bit-vector recurrence processes an entire query column
                // at once. Ordinary game titles fit in one u128, making each
                // candidate linear in its character count, not query × title.
                let (mut positive, mut negative, mut distance) = (!0u128, 0u128, *length);

                for character in text.chars() {
                    let equal = if character.is_ascii() {
                        ascii[character as usize]
                    } else {
                        unicode
                            .iter()
                            .find(|(value, _)| *value == character)
                            .map_or(0, |(_, mask)| *mask)
                    };

                    let vertical = equal | negative;
                    let horizontal = ((equal & positive).wrapping_add(positive) ^ positive) | equal;
                    let mut plus = negative | !(horizontal | positive);
                    let mut minus = positive & horizontal;
                    distance += usize::from(plus & *highest != 0);
                    distance -= usize::from(minus & *highest != 0);
                    plus = (plus << 1) | 1;
                    minus <<= 1;
                    positive = minus | !(vertical | plus);
                    negative = plus & vertical;
                }

                distance
            }
            Self::Rows { pattern, row } => {
                // Long or empty queries use one reusable DP row. Its storage is
                // created once per search, never once per catalogue entry.
                for (index, value) in row.iter_mut().enumerate() {
                    *value = index;
                }

                for (index, character) in text.chars().enumerate() {
                    let mut diagonal = row[0];
                    row[0] = index + 1;

                    for (column, &expected) in pattern.iter().enumerate() {
                        let above = row[column + 1];
                        row[column + 1] = (above + 1)
                            .min(row[column] + 1)
                            .min(diagonal + usize::from(expected != character));
                        diagonal = above;
                    }
                }

                row[pattern.len()]
            }
        }
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

#[cfg(test)]
mod tests {
    //! Bounded ranking must retain strong early matches, independently of API order.
    use super::*;

    #[test]
    fn exact_matches_survive_later_weaker_candidates_and_zero_means_zero() {
        let mut list = FuzzyList::new();
        list.push("The Legend of Zelda");

        for title in [
            "The Legend of Zelda: Ocarina of Time",
            "The Legend of Zelda: A Link to the Past",
            "The Legend of Zelda: Fan Adventure",
        ] {
            list.push(title);
        }

        assert_eq!(
            list.search("The Legend of Zelda", 1),
            ["The Legend of Zelda"]
        );

        assert!(list.search("Zelda", 0).is_empty());
    }

    #[test]
    fn nearby_titles_include_changed_subtitles_and_keep_exact_matches_first() {
        let mut list = FuzzyList::new();

        for title in [
            "The Legend of Zelda: A Fan Adventure with a Very Long Name",
            "The Legend of Zelda: The Wind Waker",
            "The Legend of Zelda: Ocarina of Time 3D",
            "The Legend of Zelda: Majora's Mask",
            "The Legend of Zelda",
            "The Legend of Zelda: Ocarina of Time",
        ] {
            list.push(title);
        }

        assert_eq!(
            list.search_similar("the legend of zelda", 1),
            ["The Legend of Zelda"]
        );

        let related = list.search_similar("The Legend of Zelda: Ocarina of Time", 5);
        assert_eq!(related[0], "The Legend of Zelda: Ocarina of Time");

        assert!(
            related
                .iter()
                .any(|title| title.contains("Majora") || title.contains("Wind Waker"))
        );

        assert_eq!(
            list.search_similar("The Legend of Zleda", 1),
            ["The Legend of Zelda"]
        );

        let all = list.search_similar("Legend", usize::MAX);

        for count in 0..=all.len() {
            assert_eq!(list.search_similar("Legend", count), all[..count]);
        }
    }

    #[test]
    fn words_inside_a_title_survive_short_unrelated_catalogue_entries() {
        let mut list = FuzzyList::new();
        let title = "The Legend of Zelda: Ocarina of Time";
        list.push(title);

        // Each unrelated title is shorter than the query. Whole-title edit
        // distance alone therefore excluded the intended title from this list.
        const SHORT_TITLES: &[&str] = &[
            "Doom",
            "Quake",
            "Portal",
            "Celeste",
            "Hades",
            "Braid",
            "Fez",
            "VVVVVV",
            "Limbo",
            "Inside",
            "Spelunky",
            "Dead Cells",
            "Half-Life",
            "Terraria",
            "Control",
        ];

        for title in SHORT_TITLES {
            list.push(title);
        }

        for query in ["ocarina of time", "zelda ocarina", "legend of zelda"] {
            assert_eq!(list.search_similar(query, 1), [title], "{query}");
            assert!(
                list.search_similar(query, SHORT_TITLES.len())
                    .contains(&title)
            );
        }
    }

    #[test]
    fn whole_words_and_strong_abbreviations_precede_scattered_matches() {
        let mut list = FuzzyList::new();
        list.push("Super Mario 64");
        list.push("Marvel Rio");
        list.push("Doom 64");
        assert_eq!(list.search_similar("mario", 1), ["Super Mario 64"]);
        assert_eq!(list.search_similar("mario 64", 1), ["Super Mario 64"]);

        // Neither title contains this abbreviation literally. The stronger
        // consecutive-letter match must still win despite the longer title.
        assert_eq!(list.search("mri", 1), ["Super Mario 64"]);
        assert_eq!(list.search_similar("mri", 1), ["Super Mario 64"]);
    }

    #[test]
    fn single_word_typos_precede_weak_scattered_matches() {
        let mut list = FuzzyList::new();
        list.push("Zelda");
        list.push("Puzzle and Dragons");
        assert_eq!(list.search_similar("zleda", 1), ["Zelda"]);
    }

    #[test]
    fn initialisms_and_short_codes_keep_their_intended_titles() {
        for (query, expected, unrelated) in [
            ("mc", "Minecraft", "Me"),
            ("tloz", "The Legend of Zelda", "Talo"),
            ("sm64", "Super Mario 64", "sm4"),
            ("twwhd", "The Legend of Zelda: The Wind Waker HD", "Two"),
            ("aba", "Ab Ba", "Doom"),
            ("日本x", "日本 語 X", "日本z"),
        ] {
            let mut list = FuzzyList::new();
            list.push(unrelated);
            list.push(expected);
            assert_eq!(list.search_similar(query, 1), [expected], "{query}");
        }
    }

    #[track_caller]
    fn reference_distance(pattern: &str, text: &str) -> usize {
        let left = pattern.chars().collect::<Vec<_>>();
        let right = text.chars().collect::<Vec<_>>();
        let mut grid = vec![vec![0; right.len() + 1]; left.len() + 1];

        for (i, row) in grid.iter_mut().enumerate() {
            row[0] = i;
        }

        for (j, cell) in grid[0].iter_mut().enumerate() {
            *cell = j;
        }

        for (i, &a) in left.iter().enumerate() {
            for (j, &b) in right.iter().enumerate() {
                grid[i + 1][j + 1] = (grid[i][j + 1] + 1)
                    .min(grid[i + 1][j] + 1)
                    .min(grid[i][j] + usize::from(a != b));
            }
        }

        grid[left.len()][right.len()]
    }

    #[test]
    fn bit_distances_match_reference_for_unicode_empty_and_long_queries() {
        let mut strings = vec![String::new()];

        for _ in 0..3 {
            let previous = strings.clone();

            for prefix in previous {
                for c in ['a', 'b', 'é'] {
                    strings.push(format!("{prefix}{c}"));
                }
            }
        }

        for pattern in &strings {
            let mut distance = EditDistance::new(pattern);

            for text in &strings {
                assert_eq!(
                    distance.measure(text),
                    reference_distance(pattern, text),
                    "{pattern:?}, {text:?}"
                );
            }
        }

        for length in [u128::BITS as usize, u128::BITS as usize + 1] {
            let pattern = "字".repeat(length);
            let text = format!("{}文", "字".repeat(length - 1));
            let mut distance = EditDistance::new(&pattern);
            assert_eq!(distance.measure(&text), 1);
            assert_eq!(distance.measure(&pattern), 0);
        }
    }

    #[test]
    #[ignore = "manual windowless catalogue search profiling"]
    fn catalogue_search_profile() {
        let mut list = FuzzyList::new();

        for index in 0..50_000 {
            list.push(&format!(
                "Game title {index} with a moderately long subtitle"
            ));
        }

        list.push("The Legend of Zelda");
        let start = std::time::Instant::now();
        let results = list.search_similar("The Legend of Zelda", 15);
        eprintln!("50,001-title similarity search: {:?}", start.elapsed());
        assert_eq!(results.len(), 15);
        assert_eq!(results[0], "The Legend of Zelda");
    }
}
