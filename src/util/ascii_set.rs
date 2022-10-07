use super::ascii_char::AsciiChar;

#[repr(transparent)]
pub struct AsciiSet([bool; 256]);

impl AsciiSet {
    pub const WHITE_SPACE: &'static Self = &Self::new([
        AsciiChar::SPACE,
        AsciiChar::CARRIAGE_RETURN,
        AsciiChar::NEW_LINE,
        AsciiChar::TAB,
    ]);

    pub const CHARACTERS_TO_ESCAPE: &'static Self = &Self::new([
        AsciiChar::LESS_THAN,
        AsciiChar::GREATER_THAN,
        AsciiChar::AMPERSAND,
        AsciiChar::SINGLE_QUOTE,
        AsciiChar::DOUBLE_QUOTE,
    ]);

    pub const EQUALITY_OR_WHITE_SPACE: &'static Self = &Self::new([
        AsciiChar::EQUALITY_SIGN,
        AsciiChar::SPACE,
        AsciiChar::CARRIAGE_RETURN,
        AsciiChar::NEW_LINE,
        AsciiChar::TAB,
    ]);

    pub const fn new<const N: usize>(chars: [AsciiChar; N]) -> Self {
        let mut set = [false; 256];
        let mut i = 0;
        while i < N {
            let c = chars[i].get();
            set[c as usize] = true;
            i += 1;
        }
        Self(set)
    }

    pub fn split_three_way<'a>(&self, text: &'a str) -> Option<(&'a str, AsciiChar, &'a str)> {
        let pos = text.bytes().position(|c| self.0[c as usize])?;
        // SAFETY: The position is guaranteed to be valid because the position
        // is found by the `position` method. Since we only ever find positions
        // for characters that are valid ASCII characters, we can safely split
        // the string into the part before, the ASCII character itself and the
        // part after without breaking any UTF-8 invariants.
        unsafe {
            Some((
                text.get_unchecked(..pos),
                AsciiChar::new_unchecked(*text.as_bytes().get_unchecked(pos)),
                text.get_unchecked(pos + 1..),
            ))
        }
    }

    pub fn split<'a>(&self, text: &'a str) -> Option<(&'a str, &'a str)> {
        let pos = text.bytes().position(|c| self.0[c as usize])?;
        // SAFETY: The position is guaranteed to be valid because the position
        // is found by the `position` method. Since we only ever find positions
        // for characters that are valid ASCII characters, we can safely split
        // the string into the part before and the part after without breaking
        // any UTF-8 invariants.
        unsafe { Some((text.get_unchecked(..pos), text.get_unchecked(pos + 1..))) }
    }

    pub fn find_not(&self, text: &str) -> Option<usize> {
        text.bytes().position(|c| !self.0[c as usize])
    }

    pub fn rfind_not(&self, text: &str) -> Option<usize> {
        text.bytes().rposition(|c| !self.0[c as usize])
    }

    pub fn trim<'text>(&self, text: &'text str) -> &'text str {
        if let Some(pos) = self.find_not(text) {
            // SAFETY: The position is guaranteed to be valid because the
            // position is found by the find_not method. Also since we only
            // skipped ASCII characters, splitting the string at the position
            // will not result in a string that is not valid UTF-8.
            self.trim_end(unsafe { text.get_unchecked(pos..) })
        } else {
            ""
        }
    }

    pub fn trim_start<'text>(&self, text: &'text str) -> &'text str {
        if let Some(pos) = self.find_not(text) {
            // SAFETY: The position is guaranteed to be valid because the
            // position is found by the find_not method. Also since we only
            // skipped ASCII characters, splitting the string at the position
            // will not result in a string that is not valid UTF-8.
            unsafe { text.get_unchecked(pos..) }
        } else {
            ""
        }
    }

    pub fn trim_end<'text>(&self, text: &'text str) -> &'text str {
        if let Some(pos) = self.rfind_not(text) {
            // SAFETY: The position is guaranteed to be valid because the
            // position is found by the find_not method. Also since we only
            // skipped ASCII characters, splitting the string at the position
            // will not result in a string that is not valid UTF-8.
            unsafe { text.get_unchecked(..pos + 1) }
        } else {
            ""
        }
    }
}
