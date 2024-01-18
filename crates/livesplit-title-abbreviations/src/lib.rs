#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::missing_const_for_fn,
    clippy::undocumented_unsafe_blocks,
    // TODO: Write documentation
    // missing_docs,
    rust_2018_idioms
)]
#![no_std]

extern crate alloc;

use alloc::{boxed::Box, string::String, vec, vec::Vec};

// FIXME: Use generators once those work on stable Rust.

fn ends_with_roman_numeral(name: &str) -> bool {
    let roman_count = name
        .bytes()
        .rev()
        .take_while(|c| matches!(c, b'I' | b'V' | b'X'))
        .count();

    let rem = &name[..name.len() - roman_count];

    rem.is_empty() || rem.ends_with(|c: char| c.is_whitespace())
}

fn ends_with_numeric(name: &str) -> bool {
    name.chars().last().is_some_and(|c| c.is_numeric())
}

fn series_subtitle_handling(
    name: &str,
    split_token: &str,
    list: &mut Vec<Box<str>>,
    depth: usize,
) -> bool {
    if let Some((series, subtitle)) = name.split_once(split_token) {
        let series_abbreviations = abbreviate_recurse(series, depth);
        let subtitle_abbreviations = abbreviate_recurse(subtitle, depth);
        let series_trimmed = series.trim_end();

        let is_series_representative =
            ends_with_numeric(series_trimmed) || ends_with_roman_numeral(series_trimmed);

        let is_there_only_one_series_abbreviation = series_abbreviations.len() == 1;

        for subtitle_abbreviation in &subtitle_abbreviations {
            for series_abbreviation in &series_abbreviations {
                if is_series_representative
                    || &**series_abbreviation != series
                    || is_there_only_one_series_abbreviation
                {
                    list.push(
                        [&**series_abbreviation, &**subtitle_abbreviation]
                            .join(split_token)
                            .into(),
                    );
                }
            }
        }

        if is_series_representative {
            list.extend(series_abbreviations);
        }
        list.extend(subtitle_abbreviations);

        true
    } else {
        false
    }
}

fn left_right_handling(
    name: &str,
    split_token: &str,
    list: &mut Vec<Box<str>>,
    depth: usize,
) -> bool {
    if let Some((series, subtitle)) = name.split_once(split_token) {
        let series_abbreviations = abbreviate_recurse(series, depth);
        let subtitle_abbreviations = abbreviate_recurse(subtitle, depth);

        for subtitle_abbreviation in &subtitle_abbreviations {
            for series_abbreviation in &series_abbreviations {
                list.push(
                    [&**series_abbreviation, &**subtitle_abbreviation]
                        .join(split_token)
                        .into(),
                );
            }
        }

        true
    } else {
        false
    }
}

fn matches_ascii_key(value: &str, ascii_key_lower: &[u8]) -> bool {
    value
        .bytes()
        .map(|c| c.to_ascii_lowercase())
        .eq(ascii_key_lower.iter().copied())
}

fn and_handling(name: &str, list: &mut Vec<Box<str>>, depth: usize) -> bool {
    let mut buf = String::new();
    for word in name.split_whitespace() {
        if matches_ascii_key(word, b"and") {
            let index = word.as_ptr() as usize - name.as_ptr() as usize;
            let (left, rest) = name.split_at(index);
            let right = &rest[word.len()..];
            buf.clear();
            buf.push_str(left);
            buf.push('&');
            buf.push_str(right);
            list.extend(abbreviate_recurse(&buf, depth));
            return true;
        }
    }
    false
}

fn remove_prefix_word<'a>(text: &'a str, ascii_key_lower: &[u8]) -> Option<&'a str> {
    let first_word = text.split_whitespace().next()?;
    if matches_ascii_key(first_word, ascii_key_lower) {
        Some(text[first_word.len()..].trim_start())
    } else {
        None
    }
}

fn is_all_caps_or_digits(text: &str) -> bool {
    text.chars().all(|c| c.is_uppercase() || c.is_numeric())
}

fn abbreviate_recurse(name: &str, depth: usize) -> Vec<Box<str>> {
    let name = name.trim();
    let mut list = vec![];
    if name.is_empty() {
        return list;
    }

    if depth >= 10 {
        list.push(name.into());
        return list;
    }
    let depth = depth + 1;

    if let Some((before, after)) = name
        .rsplit_once('(')
        .and_then(|(before, rest)| Some((before, rest.split_once(')')?.1)))
    {
        let name = [before, after].join(" ");
        list.extend(abbreviate_recurse(&name, depth + 1));
    } else if series_subtitle_handling(name, ": ", &mut list, depth)
        || series_subtitle_handling(name, " - ", &mut list, depth)
        || left_right_handling(name, " | ", &mut list, depth)
        || and_handling(name, &mut list, depth)
    {
    } else {
        if let Some(rest) =
            remove_prefix_word(name, b"the").or_else(|| remove_prefix_word(name, b"a"))
        {
            list.push(rest.into());
        }

        if name.contains(char::is_whitespace) {
            let mut abbreviated = String::new();
            for word in name.split(|c: char| c.is_whitespace() || c == '-') {
                if let Some(first_char) = word.chars().next() {
                    if !first_char.is_numeric() {
                        if word.len() <= 4 && is_all_caps_or_digits(word) {
                            if !abbreviated.is_empty() {
                                abbreviated.push(' ');
                            }
                        } else {
                            abbreviated.push(if first_char == '&' { 'a' } else { first_char });
                            continue;
                        }
                    }

                    // SAFETY: We only replace ASCII characters, which means the
                    // UTF-8 encoding stays valid.
                    unsafe {
                        let from = abbreviated.len();
                        abbreviated.push_str(word);
                        abbreviated.as_bytes_mut()[from..].iter_mut().for_each(|c| {
                            if *c == b'&' {
                                *c = b'a';
                            }
                        });
                    }
                }
            }
            list.push(abbreviated.into());
        }
    }

    list.sort_unstable();
    list.dedup();

    if let Some(idx) = list.iter().position(|x| name == x.as_ref()) {
        let last = list.len() - 1;
        list.swap(idx, last);
    } else {
        list.push(name.into());
    }

    list
}

pub fn abbreviate(name: &str) -> Vec<Box<str>> {
    abbreviate_recurse(name, 0)
}

pub fn abbreviate_category(category: &str) -> Vec<Box<str>> {
    let mut abbrevs = Vec::new();

    if let Some((before, rest)) = category.split_once('(') {
        if let Some((inside, after)) = rest.split_once(')') {
            let before = before.trim();
            let after = after.trim_end();

            let mut buf = String::with_capacity(category.len());
            buf.push_str(before);
            buf.push_str(" (");

            let mut splits = inside.split(',');
            let mut variable = splits.next().unwrap();
            for next_variable in splits {
                buf.push_str(variable);
                let old_len = buf.len();

                buf.push(')');
                buf.push_str(after);
                abbrevs.push(buf.as_str().into());

                buf.drain(old_len..);
                buf.push(',');
                variable = next_variable;
            }

            if after.trim_start().is_empty() {
                buf.drain(before.len()..);
            } else {
                buf.drain(before.len() + 1..);
                buf.push_str(after);
            }

            abbrevs.push(buf.into());
        }
    }

    abbrevs.push(category.into());

    abbrevs
}

#[cfg(test)]
mod tests {
    use super::abbreviate;
    use alloc::{boxed::Box, string::String, vec};
    use core::iter::FromIterator;

    // The tests using actual game titles can be thrown out or edited if any
    // major changes need to be made to the abbreviation algorithm. Do not
    // hesitate to remove them if they are getting in the way of actual
    // improvements.
    //
    // They exist purely as another measure to prevent accidental breakage.
    #[test]
    fn game_test_1() {
        let abbreviations = abbreviate("Burnout 3: Takedown");

        let expected = vec![
            Box::from("B3"),
            Box::from("B3: Takedown"),
            Box::from("Burnout 3"),
            Box::from("Takedown"),
            Box::from("Burnout 3: Takedown"),
        ];

        assert_eq!(abbreviations, expected);
    }

    #[test]
    fn game_test_2() {
        let abbreviations = abbreviate("The Legend of Zelda: The Wind Waker");

        let expected = vec![
            Box::from("Legend of Zelda: TWW"),
            Box::from("Legend of Zelda: The Wind Waker"),
            Box::from("Legend of Zelda: Wind Waker"),
            Box::from("TLoZ: TWW"),
            Box::from("TLoZ: The Wind Waker"),
            Box::from("TLoZ: Wind Waker"),
            Box::from("TWW"),
            Box::from("The Wind Waker"),
            Box::from("Wind Waker"),
            Box::from("The Legend of Zelda: The Wind Waker"),
        ];

        assert_eq!(abbreviations, expected);
    }

    #[test]
    fn game_test_3() {
        let abbreviations = abbreviate("SpongeBob SquarePants: Battle for Bikini Bottom");

        let expected = vec![
            Box::from("Battle for Bikini Bottom"),
            Box::from("BfBB"),
            Box::from("SS: Battle for Bikini Bottom"),
            Box::from("SS: BfBB"),
            Box::from("SpongeBob SquarePants: Battle for Bikini Bottom"),
        ];

        assert_eq!(abbreviations, expected);
    }

    #[test]
    #[rustfmt::skip]
    fn game_test_4() {
        let abbreviations = abbreviate("Super Mario 64");

        let expected = vec![
            Box::from("SM64"),
            Box::from("Super Mario 64"),
        ];

        assert_eq!(abbreviations, expected);
    }

    #[test]
    #[rustfmt::skip]
    fn doesnt_overflow_stack() {
        let s = String::from_iter((0..4 << 20).map(|_| ": "));
        abbreviate(&s);
    }

    #[test]
    fn contains_original_title() {
        let abbreviations = abbreviate("test title: the game");
        assert!(abbreviations.contains(&Box::from("test title: the game")));
    }

    #[test]
    fn removes_parens() {
        let abbreviations = abbreviate("test title (the game)");
        assert!(abbreviations.contains(&Box::from("test title")));
    }

    #[test]
    fn original_title_is_last() {
        let abbreviations = abbreviate("test title: the game");
        let last = abbreviations.last().unwrap();

        assert_eq!("test title: the game", last.as_ref())
    }
}
