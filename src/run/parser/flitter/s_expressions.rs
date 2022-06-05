//! Implements a serde deserializer for S-Expressions.
//! <http://people.csail.mit.edu/rivest/Sexp.txt>

use crate::platform::prelude::*;
use core::{fmt::Display, num::ParseIntError};
use serde::de::{self, DeserializeOwned, DeserializeSeed, MapAccess, SeqAccess, Visitor};

/// The Error types for splits files that couldn't be parsed by the Flitter
/// Parser.
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// Trailing Characters
    TrailingCharacters,
    /// Unexpected end of input
    Eof,
    /// Expected an opening parenthesis.
    ExpectedOpeningParenthesis,
    /// Expected an closing parenthesis.
    ExpectedClosingParenthesis,
    /// Expected a string.
    ExpectedString,
    /// Failed to parse an integer.
    ParseInt {
        /// The underlying error.
        source: ParseIntError,
    },
    /// Encountered invalid UTF-8 sequence.
    InvalidUtf8,
    /// Custom error.
    #[snafu(display("{error}"))]
    Custom {
        /// The underlying error.
        error: String,
    },
}

impl From<ParseIntError> for Error {
    fn from(source: ParseIntError) -> Self {
        Self::ParseInt { source }
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom {
            error: msg.to_string(),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

pub struct Deserializer<'source> {
    source: &'source str,
}

impl<'source> Deserializer<'source> {
    pub const fn from_str(source: &'source str) -> Self {
        Deserializer { source }
    }

    fn skip_whitespace(&mut self) {
        self.source = self.source.trim_start();
    }

    fn starts_with(&mut self, c: char) -> bool {
        self.source.starts_with(c)
    }

    fn strip_char(&mut self, c: char) -> bool {
        match self.source.strip_prefix(c) {
            Some(rem) => {
                self.source = rem;
                true
            }
            None => false,
        }
    }

    fn parse_ident(&mut self) -> &str {
        if let Some((pos, _)) = self
            .source
            .char_indices()
            .find(|&(_, c)| c.is_whitespace() || c == ')')
        {
            let (before, after) = &self.source.split_at(pos);
            self.source = after;
            before
        } else {
            self.source
        }
    }

    fn parse_string(&mut self) -> Result<&str> {
        if let Some(rem) = self.source.strip_prefix('"') {
            if let Some((in_str, after)) = rem.split_once('"') {
                self.source = after;
                Ok(in_str)
            } else {
                Err(Error::Eof)
            }
        } else {
            Ok(self.parse_ident())
        }
    }

    fn skip_to_matching_closing(&mut self) -> Result<()> {
        let mut count = 0;
        let mut in_string = false;
        if let Some((pos, _)) = self.source.char_indices().find(|&(_, c)| {
            if c == '(' {
                if !in_string {
                    count += 1;
                }
            } else if c == ')' {
                if !in_string {
                    if count == 0 {
                        return true;
                    } else {
                        count -= 1;
                    }
                }
            } else if c == '"' {
                in_string = !in_string;
            }
            false
        }) {
            self.source = &self.source[pos..];
            Ok(())
        } else {
            self.source = "";
            Err(Error::Eof)
        }
    }
}

pub fn from_str<T>(source: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let mut deserializer = Deserializer::from_str(source);
    let t = T::deserialize(&mut deserializer)?;
    deserializer.skip_whitespace();
    if deserializer.source.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}

impl<'de, 'source> de::Deserializer<'de> for &mut Deserializer<'source> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_ident().parse()?)
    }
    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.parse_string()?)
    }
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.parse_string()?)
    }
    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }
    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.skip_whitespace();
        if self.strip_char('(') {
            let value = visitor.visit_seq(&mut self)?;
            if self.strip_char(')') {
                Ok(value)
            } else {
                Err(Error::ExpectedClosingParenthesis)
            }
        } else {
            Err(Error::ExpectedOpeningParenthesis)
        }
    }
    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.skip_whitespace();
        if self.strip_char('(') {
            let value = visitor.visit_map(&mut self)?;
            if self.strip_char(')') {
                Ok(value)
            } else {
                Err(Error::ExpectedClosingParenthesis)
            }
        } else {
            Err(Error::ExpectedOpeningParenthesis)
        }
    }
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.parse_ident())
    }
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.skip_to_matching_closing()?;
        visitor.visit_unit()
    }
}

impl<'de, 'source> SeqAccess<'de> for &mut Deserializer<'source> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        self.skip_whitespace();
        if self.starts_with(')') {
            Ok(None)
        } else {
            seed.deserialize(&mut **self).map(Some)
        }
    }
}

impl<'de, 'source> MapAccess<'de> for &mut Deserializer<'source> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        self.skip_whitespace();
        if self.starts_with(')') {
            Ok(None)
        } else if self.strip_char('(') {
            self.skip_whitespace();
            seed.deserialize(&mut **self).map(Some)
        } else {
            Err(Error::ExpectedOpeningParenthesis)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        self.skip_whitespace();
        let result = seed.deserialize(&mut **self)?;
        self.skip_whitespace();
        if self.strip_char(')') {
            Ok(result)
        } else {
            Err(Error::ExpectedClosingParenthesis)
        }
    }
}
