extern crate unicase;

use unicase::UniCase;

fn ends_with_roman_numeral(name: &str) -> bool {
    name.split_whitespace().rev().next().map_or(false, |n| {
        n.chars().all(|c| c == 'I' || c == 'V' || c == 'X')
    })
}

fn ends_with_numeric(name: &str) -> bool {
    name.chars().last().map_or(false, |c| c.is_numeric())
}

fn series_subtitle_handling(name: &str, split_token: &str, list: &mut Vec<String>) -> bool {
    let mut iter = name.splitn(2, split_token);
    if let (Some(series), Some(subtitle)) = (iter.next(), iter.next()) {
        let series_abbreviations = abbreviate(series);
        let subtitle_abbreviations = abbreviate(subtitle);
        let series_trimmed = series.trim_right();

        let is_series_representative =
            ends_with_numeric(series_trimmed) || ends_with_roman_numeral(series_trimmed);

        let is_there_only_one_series_abbreviation = series_abbreviations.len() == 1;

        for subtitle_abbreviation in &subtitle_abbreviations {
            for series_abbreviation in &series_abbreviations {
                if is_series_representative
                    || series_abbreviation != series
                    || is_there_only_one_series_abbreviation
                {
                    list.push(format!(
                        "{}{}{}",
                        series_abbreviation, split_token, subtitle_abbreviation
                    ));
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

fn left_right_handling(name: &str, split_token: &str, list: &mut Vec<String>) -> bool {
    let mut iter = name.splitn(2, split_token);
    if let (Some(series), Some(subtitle)) = (iter.next(), iter.next()) {
        let series_abbreviations = abbreviate(series);
        let subtitle_abbreviations = abbreviate(subtitle);

        for subtitle_abbreviation in &subtitle_abbreviations {
            for series_abbreviation in &series_abbreviations {
                list.push(format!(
                    "{}{}{}",
                    series_abbreviation, split_token, subtitle_abbreviation
                ));
            }
        }

        true
    } else {
        false
    }
}

fn and_handling(name: &str, list: &mut Vec<String>) -> bool {
    let and = UniCase::new("and");
    for word in name.split_whitespace() {
        if UniCase::new(word) == and {
            let index = word.as_ptr() as usize - name.as_ptr() as usize;
            let (left, rest) = name.split_at(index);
            let right = &rest[word.len()..];
            let name = format!("{}&{}", left, right);
            list.extend(abbreviate(&name));
            return true;
        }
    }
    false
}

fn remove_prefix_word<'a>(text: &'a str, word: &str) -> Option<&'a str> {
    let first_word = text.split_whitespace().next()?;
    if unicase::eq(first_word, word) {
        Some(text[first_word.len()..].trim_left())
    } else {
        None
    }
}

fn is_all_caps_or_digits(text: &str) -> bool {
    text.chars().all(|c| c.is_uppercase() || c.is_numeric())
}

pub fn abbreviate(name: &str) -> Vec<String> {
    let name = name.trim();
    let mut list = vec![name.into()];
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
            before_parenthesis.trim_right(),
            after_parenthesis.trim_left()
        );
        list.extend(abbreviate(&name));
    } else if series_subtitle_handling(&name, ": ", &mut list)
        || series_subtitle_handling(&name, " - ", &mut list)
        || left_right_handling(&name, " | ", &mut list)
        || and_handling(&name, &mut list)
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
            list.push(abbreviated);
        }
    }

    list.sort_unstable();
    list.dedup();

    list
}
