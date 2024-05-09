pub fn eq(a: &str, b: &str) -> bool {
    Iterator::eq(
        a.chars().flat_map(|c| c.to_lowercase()),
        b.chars().flat_map(|c| c.to_lowercase()),
    )
}

/// The key needs to already be lowercase.
pub fn matches_ascii_key(key: &str, input: &str) -> bool {
    key.len() == input.len()
        && key
            .bytes()
            .eq(input.bytes().map(|b| b.to_ascii_lowercase()))
}
