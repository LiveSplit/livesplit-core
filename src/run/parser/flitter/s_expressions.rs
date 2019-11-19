//! Implements a serde deserializer for S-Expressions.
//! http://people.csail.mit.edu/rivest/Sexp.txt

use core::fmt::Display;
use core::num::ParseIntError;
use serde::de::{self, DeserializeOwned, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use std::io::{self, BufRead};
use utf8::{BufReadDecoder, BufReadDecoderError};

/// The Error types for splits files that couldn't be parsed by the Flitter
/// Parser.
#[derive(Debug, snafu::Snafu, derive_more::From)]
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
    InvalidUTF8,
    /// Failed to read from the source.
    Io {
        /// The underlying error.
        source: io::Error,
    },
    /// Custom error.
    #[snafu(display("{}", error))]
    Custom {
        /// The underlying error.
        error: String,
    },
}

impl From<BufReadDecoderError<'_>> for Error {
    fn from(error: BufReadDecoderError<'_>) -> Error {
        match error {
            BufReadDecoderError::InvalidByteSequence(_) => Error::InvalidUTF8,
            BufReadDecoderError::Io(source) => Error::Io { source },
        }
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

pub struct Deserializer<B: BufRead> {
    decoder: BufReadDecoder<B>,
    buf: String,
    index: usize,
}

impl<B: BufRead> Deserializer<B> {
    pub fn from_reader(reader: B) -> Self {
        Deserializer {
            decoder: BufReadDecoder::new(reader),
            buf: String::new(),
            index: 0,
        }
    }

    fn maybe_fill_buf(&mut self, clear: bool) -> Result<()> {
        if self.index == self.buf.len() {
            self.actually_fill_buf(clear)?;
        }
        Ok(())
    }

    fn actually_fill_buf(&mut self, clear: bool) -> Result<()> {
        if clear {
            self.buf.clear();
        }
        self.index = self.buf.len();
        if let Some(text) = self.decoder.next_strict() {
            let text = text?;
            self.buf.push_str(text);
        }
        Ok(())
    }

    fn skip_whitespace(&mut self) -> Result<()> {
        self.maybe_fill_buf(true)?;
        while self.index != self.buf.len() {
            if let Some((index, _)) = self.buf[self.index..]
                .char_indices()
                .find(|(_, c)| !c.is_whitespace())
            {
                self.index += index;
                return Ok(());
            } else {
                self.actually_fill_buf(true)?;
            }
        }
        Ok(())
    }

    fn peek_char(&mut self) -> Result<char> {
        self.maybe_fill_buf(true)?;
        self.buf[self.index..].chars().next().ok_or(Error::Eof)
    }

    fn next_char(&mut self) -> Result<char> {
        let ch = self.peek_char()?;
        self.index += ch.len_utf8();
        Ok(ch)
    }

    fn parse_ident(&mut self) -> Result<&str> {
        self.maybe_fill_buf(true)?;
        let begin = self.index;
        while self.index != self.buf.len() {
            if let Some(len) = self.buf[self.index..].find(|c: char| c.is_whitespace() || c == ')')
            {
                self.index += len;
                return Ok(&self.buf[begin..self.index]);
            } else {
                self.actually_fill_buf(false)?;
            }
        }
        Ok(&self.buf[begin..])
    }

    fn parse_string(&mut self) -> Result<&str> {
        if self.peek_char()? == '"' {
            self.next_char()?;
            self.maybe_fill_buf(true)?;
            let begin = self.index;
            while self.index != self.buf.len() {
                if let Some(len) = self.buf[self.index..].find('"') {
                    self.index += len + 1;
                    return Ok(&self.buf[begin..self.index - 1]);
                } else {
                    self.actually_fill_buf(false)?;
                }
            }
            Err(Error::Eof)
        } else {
            self.parse_ident()
        }
    }

    fn skip_to_matching_closing(&mut self) -> Result<()> {
        let mut count = 0;
        let mut in_string = false;
        self.maybe_fill_buf(true)?;
        while self.index != self.buf.len() {
            if let Some(len) = self.buf[self.index..].find(|c: char| {
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
                self.index += len;
                return Ok(());
            } else {
                self.actually_fill_buf(false)?;
            }
        }
        Err(Error::Eof)
    }
}

pub fn from_reader<T, B>(reader: B) -> Result<T>
where
    T: DeserializeOwned,
    B: BufRead,
{
    let mut deserializer = Deserializer::from_reader(reader);
    let t = T::deserialize(&mut deserializer)?;
    deserializer.skip_whitespace()?;
    if deserializer.index == deserializer.buf.len() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}

impl<'de, B: BufRead> de::Deserializer<'de> for &mut Deserializer<B> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_ident()?.parse()?)
    }
    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
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
        unimplemented!()
    }
    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
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
        unimplemented!()
    }
    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.skip_whitespace()?;
        if self.next_char()? == '(' {
            let value = visitor.visit_seq(&mut self)?;
            if self.next_char()? == ')' {
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
        unimplemented!()
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
        unimplemented!()
    }
    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.skip_whitespace()?;
        if self.next_char()? == '(' {
            let value = visitor.visit_map(&mut self)?;
            if self.next_char()? == ')' {
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
        unimplemented!()
    }
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.parse_ident()?)
    }
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.skip_to_matching_closing()?;
        visitor.visit_unit()
    }
}

impl<'de, B: BufRead> SeqAccess<'de> for &mut Deserializer<B> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        self.skip_whitespace()?;
        let peeked = self.peek_char()?;
        if peeked == ')' {
            Ok(None)
        } else {
            seed.deserialize(&mut **self).map(Some)
        }
    }
}

impl<'de, B: BufRead> MapAccess<'de> for &mut Deserializer<B> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        self.skip_whitespace()?;
        let peeked = self.peek_char()?;
        if peeked == ')' {
            Ok(None)
        } else if peeked == '(' {
            self.next_char()?;
            self.skip_whitespace()?;
            seed.deserialize(&mut **self).map(Some)
        } else {
            Err(Error::ExpectedOpeningParenthesis)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        self.skip_whitespace()?;
        let result = seed.deserialize(&mut **self)?;
        self.skip_whitespace()?;
        if self.next_char()? == ')' {
            Ok(result)
        } else {
            Err(Error::ExpectedClosingParenthesis)
        }
    }
}
