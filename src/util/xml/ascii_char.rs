#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct AsciiChar(u8);

impl AsciiChar {
    pub const LESS_THAN: Self = Self::new(b'<');
    pub const GREATER_THAN: Self = Self::new(b'>');
    pub const SPACE: Self = Self::new(b' ');
    pub const CARRIAGE_RETURN: Self = Self::new(b'\r');
    pub const NEW_LINE: Self = Self::new(b'\n');
    pub const TAB: Self = Self::new(b'\t');
    pub const AMPERSAND: Self = Self::new(b'&');
    pub const SEMICOLON: Self = Self::new(b';');
    pub const SINGLE_QUOTE: Self = Self::new(b'\'');
    pub const DOUBLE_QUOTE: Self = Self::new(b'"');
    pub const EQUALITY_SIGN: Self = Self::new(b'=');

    pub const fn new(c: u8) -> Self {
        if c > 127 {
            panic!("Is not an ASCII character.");
        }
        Self(c)
    }

    pub const unsafe fn new_unchecked(c: u8) -> Self {
        Self(c)
    }

    pub const fn get(self) -> u8 {
        self.0
    }

    pub fn contains(self, text: &str) -> bool {
        memchr::memchr(self.0, text.as_bytes()).is_some()
    }

    pub fn split_once(self, text: &str) -> Option<(&str, &str)> {
        let pos = memchr::memchr(self.0, text.as_bytes())?;
        // SAFETY: memchr guarantees that the position is valid. Also since we
        // are looking for ASCII bytes, splitting at the position is guaranteed
        // to be valid UTF-8.
        unsafe { Some((text.get_unchecked(..pos), text.get_unchecked(pos + 1..))) }
    }
}
