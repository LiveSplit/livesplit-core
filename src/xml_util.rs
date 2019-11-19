use crate::timing;
use alloc::borrow::Cow;
use alloc::string;
use chrono::ParseError as ChronoError;
use core::num::{ParseFloatError, ParseIntError};
use core::ops::Deref;
use core::str;
use quick_xml::events::{attributes, BytesStart, Event};
use quick_xml::{Error as XmlError, Reader, Writer};
use std::io::{self, BufRead};

/// The Error type for XML-based splits files that couldn't be parsed.
#[derive(Debug, snafu::Snafu, derive_more::From)]
pub enum Error {
    /// Failed to parse the XML.
    #[snafu(display("{}", error))]
    Xml { error: XmlError },
    /// Failed to read from the source.
    Io { source: io::Error },
    /// Failed to parse a boolean.
    Bool,
    /// Didn't expect the end of the file.
    UnexpectedEndOfFile,
    /// Didn't expect an inner element.
    UnexpectedElement,
    /// A required attribute has not been found on an element.
    AttributeNotFound,
    /// A required element has not been found.
    ElementNotFound,
    /// The length of a buffer was too large.
    LengthOutOfBounds,
    /// Failed to decode a string slice as UTF-8.
    Utf8Str { source: str::Utf8Error },
    /// Failed to decode a string as UTF-8.
    Utf8String { source: string::FromUtf8Error },
    /// Failed to parse an integer.
    Int { source: ParseIntError },
    /// Failed to parse a floating point number.
    Float { source: ParseFloatError },
    /// Failed to parse a time.
    Time { source: timing::ParseError },
    /// Failed to parse a date.
    Date { source: ChronoError },
}

/// The Result type for Parsers that parse XML-based splits files.
// pub type Result<T> = StdResult<T, Error>;

pub struct Tag<'a>(BytesStart<'a>, *mut Vec<u8>);

impl<'a> Deref for Tag<'a> {
    type Target = BytesStart<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Tag<'a> {
    pub unsafe fn new(tag: BytesStart<'a>, buf: *mut Vec<u8>) -> Self {
        Tag(tag, buf)
    }

    pub fn into_buf(self) -> &'a mut Vec<u8> {
        unsafe { &mut *self.1 }
    }
}

pub struct AttributeValue<'a>(&'a attributes::Attribute<'a>);

impl<'a> AttributeValue<'a> {
    pub fn get<E>(self) -> Result<Cow<'a, str>, E>
    where
        E: From<Error> + From<string::FromUtf8Error> + From<str::Utf8Error>,
    {
        decode_cow_text(
            self.0
                .unescaped_value()
                .map_err(|error| Error::Xml { error })?,
        )
    }

    pub fn get_raw(&self) -> &'a [u8] {
        &self.0.value
    }
}

fn decode_cow_text<E>(cow: Cow<'_, [u8]>) -> Result<Cow<'_, str>, E>
where
    E: From<string::FromUtf8Error> + From<str::Utf8Error>,
{
    Ok(match cow {
        Cow::Borrowed(b) => Cow::Borrowed(str::from_utf8(b)?),
        Cow::Owned(o) => Cow::Owned(String::from_utf8(o)?),
    })
}

pub fn text_parsed<R, F, T, E>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<(), E>
where
    R: BufRead,
    F: FnOnce(T),
    T: str::FromStr,
    E: From<Error> + From<string::FromUtf8Error> + From<str::Utf8Error> + From<T::Err>,
{
    text_err(reader, buf, |t| {
        f(t.parse()?);
        Ok(())
    })
}

pub fn text<R, F, E>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<(), E>
where
    R: BufRead,
    F: FnOnce(Cow<'_, str>),
    E: From<Error> + From<string::FromUtf8Error> + From<str::Utf8Error>,
{
    text_err(reader, buf, |t| {
        f(t);
        Ok(())
    })
}

pub fn text_err<R, F, E>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<(), E>
where
    R: BufRead,
    F: FnOnce(Cow<'_, str>) -> Result<(), E>,
    E: From<Error> + From<string::FromUtf8Error> + From<str::Utf8Error>,
{
    text_as_bytes_err(reader, buf, |b| f(decode_cow_text::<E>(b)?))
}

pub fn text_as_escaped_bytes_err<R, F, T, E>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    f: F,
) -> Result<T, E>
where
    R: BufRead,
    F: FnOnce(&[u8]) -> Result<T, E>,
    E: From<Error>,
{
    loop {
        buf.clear();
        match reader
            .read_event(buf)
            .map_err(|error| Error::Xml { error })?
        {
            Event::Start(_) => return Err(Error::UnexpectedElement).map_err(Into::into),
            Event::End(_) => {
                return f(&[]);
            }
            Event::Text(text) | Event::CData(text) => {
                let val = f(&text)?;
                end_tag_immediately(reader, buf)?;
                return Ok(val);
            }
            Event::Eof => return Err(Error::UnexpectedEndOfFile).map_err(Into::into),
            _ => {}
        }
    }
}

pub fn text_as_bytes_err<R, F, T, E>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    f: F,
) -> Result<T, E>
where
    R: BufRead,
    F: FnOnce(Cow<'_, [u8]>) -> Result<T, E>,
    E: From<Error>,
{
    loop {
        buf.clear();
        match reader
            .read_event(buf)
            .map_err(|error| Error::Xml { error })?
        {
            Event::Start(_) => return Err(Error::UnexpectedElement).map_err(Into::into),
            Event::End(_) => {
                return f(Cow::Borrowed(&[]));
            }
            Event::Text(text) | Event::CData(text) => {
                let text = text.unescaped().map_err(Into::into)?;
                let val = f(text)?;
                end_tag_immediately(reader, buf)?;
                return Ok(val);
            }
            Event::Eof => return Err(Error::UnexpectedEndOfFile).map_err(Into::into),
            _ => {}
        }
    }
}

fn end_tag_immediately<R, E>(reader: &mut Reader<R>, buf: &mut Vec<u8>) -> Result<(), E>
where
    R: BufRead,
    E: From<Error>,
{
    loop {
        buf.clear();
        match reader
            .read_event(buf)
            .map_err(|error| Error::Xml { error })?
        {
            Event::Start(_) => return Err(Error::UnexpectedElement).map_err(Into::into),
            Event::End(_) => return Ok(()),
            Event::Eof => return Err(Error::UnexpectedEndOfFile).map_err(Into::into),
            _ => {}
        }
    }
}

pub fn reencode_children<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    target_buf: &mut Vec<u8>,
) -> Result<(), Error>
where
    R: BufRead,
{
    reader.expand_empty_elements(false);
    let mut writer = Writer::new(target_buf);
    let mut depth = 0;
    loop {
        buf.clear();
        match reader.read_event(buf)? {
            Event::Start(start) => {
                depth += 1;
                writer.write_event(Event::Start(start))?;
            }
            Event::End(end) => {
                if depth == 0 {
                    reader.expand_empty_elements(true);
                    return Ok(());
                }
                depth -= 1;
                writer.write_event(Event::End(end))?;
            }
            event @ Event::Text(_)
            | event @ Event::Comment(_)
            | event @ Event::CData(_)
            | event @ Event::PI(_)
            | event @ Event::Empty(_) => {
                writer.write_event(event)?;
            }
            Event::Decl(_) => {
                // Shouldn't really be a child anyway.
            }
            Event::DocType(_) => {
                // A DOCTYPE is not allowed in content.
            }
            Event::Eof => return Err(Error::UnexpectedEndOfFile),
        }
    }
}

pub fn end_tag<R, E>(reader: &mut Reader<R>, buf: &mut Vec<u8>) -> Result<(), E>
where
    R: BufRead,
    E: From<Error>,
{
    let mut depth = 0;
    loop {
        buf.clear();
        match reader
            .read_event(buf)
            .map_err(|error| Error::Xml { error })?
        {
            Event::Start(_) => {
                depth += 1;
            }
            Event::End(_) => {
                if depth == 0 {
                    return Ok(());
                }
                depth -= 1;
            }
            Event::Eof => return Err(Error::UnexpectedEndOfFile).map_err(Into::into),
            _ => {}
        }
    }
}

pub fn single_child<R, F, T, E>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    tag: &[u8],
    mut f: F,
) -> Result<T, E>
where
    R: BufRead,
    F: FnMut(&mut Reader<R>, Tag<'_>) -> Result<T, E>,
    E: From<Error>,
{
    let mut val = None;
    parse_children::<_, _, E>(reader, buf, |reader, t| {
        if t.name() == tag && val.is_none() {
            val = Some(f(reader, t)?);
            Ok(())
        } else {
            end_tag(reader, t.into_buf())
        }
    })?;
    val.ok_or(Error::ElementNotFound).map_err(Into::into)
}

pub fn parse_children<R, F, E>(reader: &mut Reader<R>, buf: &mut Vec<u8>, mut f: F) -> Result<(), E>
where
    R: BufRead,
    F: FnMut(&mut Reader<R>, Tag<'_>) -> Result<(), E>,
    E: From<Error>,
{
    unsafe {
        let ptr_buf: *mut Vec<u8> = buf;
        loop {
            buf.clear();
            match reader
                .read_event(buf)
                .map_err(|error| Error::Xml { error })?
            {
                Event::Start(start) => {
                    let tag = Tag::new(start, ptr_buf);
                    f(reader, tag)?;
                }
                Event::End(_) => return Ok(()),
                Event::Eof => return Err(Error::UnexpectedEndOfFile).map_err(Into::into),
                _ => {}
            }
        }
    }
}

pub fn parse_base<R, F, E>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    tag: &[u8],
    mut f: F,
) -> Result<(), E>
where
    R: BufRead,
    F: FnMut(&mut Reader<R>, Tag<'_>) -> Result<(), E>,
    E: From<Error>,
{
    unsafe {
        let ptr_buf: *mut Vec<u8> = buf;
        loop {
            buf.clear();
            match reader
                .read_event(buf)
                .map_err(|error| Error::Xml { error })?
            {
                Event::Start(start) => {
                    if start.name() == tag {
                        let tag = Tag::new(start, ptr_buf);
                        return f(reader, tag);
                    } else {
                        return Err(Error::ElementNotFound).map_err(Into::into);
                    }
                }
                Event::Eof => return Err(Error::UnexpectedEndOfFile).map_err(Into::into),
                _ => {}
            }
        }
    }
}

pub fn parse_attributes<F, E>(tag: &BytesStart<'_>, mut f: F) -> Result<(), E>
where
    F: FnMut(&[u8], AttributeValue<'_>) -> Result<bool, E>,
    E: From<Error>,
{
    for attribute in tag.attributes().with_checks(false) {
        let attribute = attribute.map_err(|error| Error::Xml { error })?;
        let key = attribute.key;
        if !f(key, AttributeValue(&attribute))? {
            return Ok(());
        }
    }
    Ok(())
}

pub fn optional_attribute_err<F, E>(tag: &BytesStart<'_>, key: &[u8], mut f: F) -> Result<(), E>
where
    F: FnMut(Cow<'_, str>) -> Result<(), E>,
    E: From<Error>,
{
    parse_attributes(tag, |k, v| {
        if k == key {
            f(v.get()?)?;
            Ok(false)
        } else {
            Ok(true)
        }
    })
}

pub fn attribute_err<F, E>(tag: &BytesStart<'_>, key: &[u8], mut f: F) -> Result<(), E>
where
    F: FnMut(Cow<'_, str>) -> Result<(), E>,
    E: From<Error>,
{
    let mut called = false;
    parse_attributes::<_, E>(tag, |k, v| {
        if k == key {
            f(v.get()?)?;
            called = true;
            Ok(false)
        } else {
            Ok(true)
        }
    })?;
    if called {
        Ok(())
    } else {
        Err(Error::AttributeNotFound).map_err(Into::into)
    }
}

pub fn attribute<F, E>(tag: &BytesStart<'_>, key: &[u8], mut f: F) -> Result<(), E>
where
    F: FnMut(Cow<'_, str>),
    E: From<Error>,
{
    attribute_err(tag, key, |t| {
        f(t);
        Ok(())
    })
}
