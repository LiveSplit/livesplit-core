use alloc::{borrow::Cow, string::String};
use core::{
    char::{self, REPLACEMENT_CHARACTER},
    fmt::{self, Debug, Display},
    iter, str,
};

pub mod helper;
mod reader;
mod writer;

pub use self::{
    reader::{Event, Reader},
    writer::{AttributeWriter, DisplayAlreadyEscaped, NO_ATTRIBUTES, Value, Writer},
};

use super::{ascii_char::AsciiChar, ascii_set::AsciiSet};

#[derive(Copy, Clone)]
pub struct Tag<'a>(&'a str);

impl Debug for Tag<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("<")?;
        Display::fmt(self.0, f)?;
        f.write_str(" />")
    }
}

impl<'a> Tag<'a> {
    pub fn name_and_attributes(&self) -> (TagName<'a>, Attributes<'a>) {
        match split_whitespace(self.0) {
            Some((before, after)) => (TagName(before), Attributes(after)),
            None => (TagName(self.0), Attributes("")),
        }
    }
}

#[derive(Copy, Clone)]
pub struct TagName<'a>(&'a str);

impl<'a> TagName<'a> {
    pub const fn name(self) -> &'a str {
        self.0
    }
}

impl Debug for TagName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("<")?;
        Display::fmt(self.0, f)?;
        f.write_str("/>")
    }
}

#[derive(Copy, Clone)]
pub struct Attributes<'a>(&'a str);

impl<'a> Attributes<'a> {
    pub fn iter(self) -> impl Iterator<Item = (&'a str, Text<'a>)> + use<'a> {
        let mut rem = self.0;
        iter::from_fn(move || {
            rem = trim_start(rem);
            let (key, space_maybe, after) =
                AsciiSet::EQUALITY_OR_WHITE_SPACE.split_three_way(rem)?;
            rem = after;
            if space_maybe.get() != b'=' {
                rem = trim_start(rem);
                match rem.strip_prefix('=') {
                    Some(rest) => rem = rest,
                    None => return None,
                }
            }
            rem = trim_start(rem);
            let attr = match rem.as_bytes() {
                [q @ b'"', rest @ ..] | [q @ b'\'', rest @ ..] => {
                    // SAFETY: q is either `"` or `'`, which are guaranteed to
                    // be valid ASCII characters. The rest of the string is
                    // still valid UTF-8 because we removed an ASCII character.
                    let (before, after) = unsafe {
                        AsciiChar::new_unchecked(*q).split_once(str::from_utf8_unchecked(rest))?
                    };
                    rem = after;
                    before
                }
                _ => return None,
            };

            Some((key, Text(attr)))
        })
    }
}

#[derive(Copy, Clone)]
pub struct Text<'a>(&'a str);

impl Display for Text<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.try_unescape_with_fn(|chunk| Display::fmt(chunk, f))
    }
}

impl Debug for Text<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self.0, f)
    }
}

impl<'a> Text<'a> {
    pub const fn new_escaped(s: &'a str) -> Self {
        Self(s)
    }

    pub const fn is_empty(self) -> bool {
        self.0.is_empty()
    }

    pub const fn escaped(self) -> &'a str {
        self.0
    }

    pub fn unescape_str_into(self, buf: &mut String) -> &mut String {
        buf.clear();
        buf.reserve(self.0.len());
        let _ = self.try_unescape_with_fn::<_, ()>(|chunk| {
            buf.push_str(chunk);
            Ok(())
        });
        buf
    }

    // FIXME: Remove in favor of unescape_str_into.
    pub fn unescape_cow(self) -> Cow<'a, str> {
        if AsciiChar::AMPERSAND.contains(self.0) {
            self.unescape_str().into()
        } else {
            self.0.into()
        }
    }

    pub fn unescape_str(self) -> String {
        let mut string = String::new();
        self.unescape_str_into(&mut string);
        string
    }

    pub fn try_unescape_with_fn<F, E>(self, mut f: F) -> Result<(), E>
    where
        F: FnMut(&str) -> Result<(), E>,
    {
        let mut rem = self.0;

        let mut buf = [0; 4];
        while let Some((before, after)) = AsciiChar::AMPERSAND.split_once(rem) {
            f(before)?;
            rem = after;

            match AsciiChar::SEMICOLON.split_once(rem) {
                Some((escaped, after)) => {
                    rem = after;
                    let char = match escaped {
                        "lt" => "<",
                        "gt" => ">",
                        "amp" => "&",
                        "apos" => "'",
                        "quot" => "\"",
                        _ => {
                            let char = if let Some(code) = escaped.strip_prefix("#x") {
                                parse_hexadecimal(code)
                            } else if let Some(code) = escaped.strip_prefix('#') {
                                parse_decimal(code)
                            } else {
                                REPLACEMENT_CHARACTER
                            };

                            char.encode_utf8(&mut buf)
                        }
                    };
                    f(char)?;
                }
                None => {
                    return f("�");
                }
            }
        }

        f(rem)
    }
}

fn split_whitespace(rem: &str) -> Option<(&str, &str)> {
    AsciiSet::WHITE_SPACE.split(rem)
}

fn trim(rem: &str) -> &str {
    AsciiSet::WHITE_SPACE.trim(rem)
}

fn trim_start(rem: &str) -> &str {
    AsciiSet::WHITE_SPACE.trim_start(rem)
}

fn parse_hexadecimal(bytes: &str) -> char {
    match u32::from_str_radix(bytes, 16) {
        Ok(c) => char::from_u32(c).unwrap_or(REPLACEMENT_CHARACTER),
        Err(_) => REPLACEMENT_CHARACTER,
    }
}

fn parse_decimal(bytes: &str) -> char {
    match bytes.parse() {
        Ok(c) => char::from_u32(c).unwrap_or(REPLACEMENT_CHARACTER),
        Err(_) => REPLACEMENT_CHARACTER,
    }
}
