#![no_std]

extern crate alloc;

use alloc::{boxed::Box, format, string::String, vec, vec::Vec};
use unicase::UniCase;

// FIXME: Use generators once those work on stable Rust.

fn ends_with_roman_numeral(name: &str) -> bool {
    name.split_whitespace().rev().next().map_or(false, |n| {
        n.chars().all(|c| c == 'I' || c == 'V' || c == 'X')
    })
}

fn ends_with_numeric(name: &str) -> bool {
    name.chars().last().map_or(false, |c| c.is_numeric())
}

fn series_subtitle_handling(name: &str, split_token: &str, list: &mut Vec<Box<str>>) -> bool {
    let mut iter = name.splitn(2, split_token);
    if let (Some(series), Some(subtitle)) = (iter.next(), iter.next()) {
        let series_abbreviations = abbreviate(series);
        let subtitle_abbreviations = abbreviate(subtitle);
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
                        format!("{series_abbreviation}{split_token}{subtitle_abbreviation}").into(),
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

fn left_right_handling(name: &str, split_token: &str, list: &mut Vec<Box<str>>) -> bool {
    let mut iter = name.splitn(2, split_token);
    if let (Some(series), Some(subtitle)) = (iter.next(), iter.next()) {
        let series_abbreviations = abbreviate(series);
        let subtitle_abbreviations = abbreviate(subtitle);

        for subtitle_abbreviation in &subtitle_abbreviations {
            for series_abbreviation in &series_abbreviations {
                list.push(
                    format!("{series_abbreviation}{split_token}{subtitle_abbreviation}").into(),
                );
            }
        }

        true
    } else {
        false
    }
}

fn and_handling(name: &str, list: &mut Vec<Box<str>>) -> bool {
    let and = UniCase::new("and");
    for word in name.split_whitespace() {
        if UniCase::new(word) == and {
            let index = word.as_ptr() as usize - name.as_ptr() as usize;
            let (left, rest) = name.split_at(index);
            let right = &rest[word.len()..];
            let name = format!("{left}&{right}");
            list.extend(abbreviate(&name));
            return true;
        }
    }
    false
}

fn remove_prefix_word<'a>(text: &'a str, word: &str) -> Option<&'a str> {
    let first_word = text.split_whitespace().next()?;
    if unicase::eq(first_word, word) {
        Some(text[first_word.len()..].trim_start())
    } else {
        None
    }
}

fn is_all_caps_or_digits(text: &str) -> bool {
    text.chars().all(|c| c.is_uppercase() || c.is_numeric())
}

pub fn abbreviate(name: &str) -> Vec<Box<str>> {
    let name = name.trim();
    let mut list = vec![];
    if name.is_empty() {
        return list;
    }

    let parenthesis = name
        .char_indices()
        .rev()
        .find(|&(_, c)| c == '(')
        .and_then(|(start, _)| {
            name[start + 1..]
                .char_indices()
                .find(|&(_, c)| c == ')')
                .map(|(end, _)| (start, end + 2))
        });

    if let Some((start, end)) = parenthesis {
        let (before_parenthesis, rest) = name.split_at(start);
        let after_parenthesis = &rest[end..];
        let name = format!(
            "{} {}",
            before_parenthesis.trim_end(),
            after_parenthesis.trim_start()
        );
        list.extend(abbreviate(&name));
    } else if series_subtitle_handling(name, ": ", &mut list)
        || series_subtitle_handling(name, " - ", &mut list)
        || left_right_handling(name, " | ", &mut list)
        || and_handling(name, &mut list)
    {
    } else {
        if let Some(rest) =
            remove_prefix_word(name, "the").or_else(|| remove_prefix_word(name, "a"))
        {
            list.push(rest.into());
        }

        if name.contains(char::is_whitespace) {
            let mut abbreviated = String::new();
            for word in name.split(|c: char| c.is_whitespace() || c == '-') {
                if let Some(first_char) = word.chars().next() {
                    let word_mapped = word.chars().map(|c| if c == '&' { 'a' } else { c });

                    if first_char.is_numeric() {
                        abbreviated.extend(word_mapped);
                    } else if word.len() <= 4 && is_all_caps_or_digits(word) {
                        if !abbreviated.is_empty() {
                            abbreviated.push(' ');
                        }
                        abbreviated.extend(word_mapped);
                    } else {
                        abbreviated.push(first_char);
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

pub fn abbreviate_category(category: &str) -> Vec<Box<str>> {
    let mut abbrevs = Vec::new();

    let mut splits = category.splitn(2, '(');
    let before = splits.next().unwrap().trim();

    if let Some(rest) = splits.next() {
        splits = rest.splitn(2, ')');
        let inside = splits.next().unwrap();
        if let Some(after) = splits.next() {
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

            if after.trim().is_empty() {
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
    use alloc::boxed::Box;
    use alloc::vec;

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
