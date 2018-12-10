use super::super::ComparisonError;
use crate::timing;
use chrono::ParseError as ChronoError;
use quick_xml::events::{attributes, BytesStart, Event};
use quick_xml::{Error as XmlError, Reader, Writer};
use std::borrow::Cow;
use std::io::{self, BufRead};
use std::num::{ParseFloatError, ParseIntError};
use std::ops::Deref;
use std::result::Result as StdResult;
use std::{str, string};

quick_error! {
    /// The Error type for XML-based splits files that couldn't be parsed.
    #[derive(Debug)]
    pub enum Error {
        /// Failed to parse the XML.
        Xml(err: XmlError) {
            from()
        }
        /// Failed to read from the source.
        Io(err: io::Error) {
            from()
        }
        /// Failed to parse a boolean.
        Bool {}
        /// Didn't expect the end of the file.
        UnexpectedEndOfFile {}
        /// Didn't expect an inner element.
        UnexpectedElement {}
        /// A required attribute has not been found on an element.
        AttributeNotFound {}
        /// A required element has not been found.
        ElementNotFound {}
        /// The length of a buffer was too large.
        LengthOutOfBounds {}
        /// Parsed comparison has an invalid name.
        InvalidComparisonName(err: ComparisonError) {
            from()
        }
        /// Failed to decode a string slice as UTF-8.
        Utf8Str(err: str::Utf8Error) {
            from()
        }
        /// Failed to decode a string as UTF-8.
        Utf8String(err: string::FromUtf8Error) {
            from()
        }
        /// Failed to parse an integer.
        Int(err: ParseIntError) {
            from()
        }
        /// Failed to parse a floating point number.
        Float(err: ParseFloatError) {
            from()
        }
        /// Failed to parse a time.
        Time(err: timing::ParseError) {
            from()
        }
        /// Failed to parse a date.
        Date(err: ChronoError) {
            from()
        }
    }
}

/// The Result type for Parsers that parse XML-based splits files.
pub type Result<T> = StdResult<T, Error>;

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
    pub fn get(self) -> Result<Cow<'a, str>> {
        decode_cow_text(self.0.unescaped_value()?)
    }
}

fn decode_cow_text(cow: Cow<'_, [u8]>) -> Result<Cow<'_, str>> {
    Ok(match cow {
        Cow::Borrowed(b) => Cow::Borrowed(str::from_utf8(b)?),
        Cow::Owned(o) => Cow::Owned(String::from_utf8(o)?),
    })
}

pub fn text_parsed<R, F, T>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(T),
    T: str::FromStr,
    <T as str::FromStr>::Err: Into<Error>,
{
    text_err(reader, buf, |t| {
        f(t.parse().map_err(Into::into)?);
        Ok(())
    })
}

pub fn text<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(Cow<'_, str>),
{
    text_err(reader, buf, |t| {
        f(t);
        Ok(())
    })
}

pub fn text_err<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<()>
where
    R: BufRead,
    F: FnOnce(Cow<'_, str>) -> Result<()>,
{
    text_as_bytes_err(reader, buf, |b| f(decode_cow_text(b)?))
}

pub fn text_as_bytes_err<R, F, T>(reader: &mut Reader<R>, buf: &mut Vec<u8>, f: F) -> Result<T>
where
    R: BufRead,
    F: FnOnce(Cow<'_, [u8]>) -> Result<T>,
{
    let val;
    loop {
        buf.clear();
        match reader.read_event(buf)? {
            Event::Start(_) => return Err(Error::UnexpectedElement),
            Event::End(_) => {
                return f(Cow::Borrowed(&[]));
            }
            Event::Text(text) | Event::CData(text) => {
                let text = text.unescaped()?;
                val = f(text)?;
                break;
            }
            Event::Eof => return Err(Error::UnexpectedEndOfFile),
            _ => {}
        }
    }
    end_tag_immediately(reader, buf)?;
    Ok(val)
}

fn end_tag_immediately<R: BufRead>(reader: &mut Reader<R>, buf: &mut Vec<u8>) -> Result<()> {
    loop {
        buf.clear();
        match reader.read_event(buf)? {
            Event::Start(_) => return Err(Error::UnexpectedElement),
            Event::End(_) => return Ok(()),
            Event::Eof => return Err(Error::UnexpectedEndOfFile),
            _ => {}
        }
    }
}

pub fn reencode_children<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    target_buf: &mut Vec<u8>,
) -> Result<()>
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

pub fn end_tag<R: BufRead>(reader: &mut Reader<R>, buf: &mut Vec<u8>) -> Result<()> {
    let mut depth = 0;
    loop {
        buf.clear();
        match reader.read_event(buf)? {
            Event::Start(_) => {
                depth += 1;
            }
            Event::End(_) => {
                if depth == 0 {
                    return Ok(());
                }
                depth -= 1;
            }
            Event::Eof => return Err(Error::UnexpectedEndOfFile),
            _ => {}
        }
    }
}

pub fn single_child<R, F, T>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    tag: &[u8],
    mut f: F,
) -> Result<T>
where
    R: BufRead,
    F: FnMut(&mut Reader<R>, Tag<'_>) -> Result<T>,
{
    let mut val = None;
    parse_children(reader, buf, |reader, t| {
        if t.name() == tag && val.is_none() {
            val = Some(f(reader, t)?);
            Ok(())
        } else {
            end_tag(reader, t.into_buf())
        }
    })?;
    val.ok_or(Error::ElementNotFound)
}

pub fn parse_children<R, F>(reader: &mut Reader<R>, buf: &mut Vec<u8>, mut f: F) -> Result<()>
where
    R: BufRead,
    F: FnMut(&mut Reader<R>, Tag<'_>) -> Result<()>,
{
    unsafe {
        let ptr_buf: *mut Vec<u8> = buf;
        loop {
            buf.clear();
            match reader.read_event(buf)? {
                Event::Start(start) => {
                    let tag = Tag::new(start, ptr_buf);
                    f(reader, tag)?;
                }
                Event::End(_) => return Ok(()),
                Event::Eof => return Err(Error::UnexpectedEndOfFile),
                _ => {}
            }
        }
    }
}

pub fn parse_base<R, F>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    tag: &[u8],
    mut f: F,
) -> Result<()>
where
    R: BufRead,
    F: FnMut(&mut Reader<R>, Tag<'_>) -> Result<()>,
{
    unsafe {
        let ptr_buf: *mut Vec<u8> = buf;
        loop {
            buf.clear();
            match reader.read_event(buf)? {
                Event::Start(start) => {
                    if start.name() == tag {
                        let tag = Tag::new(start, ptr_buf);
                        return f(reader, tag);
                    } else {
                        return Err(Error::ElementNotFound);
                    }
                }
                Event::Eof => return Err(Error::UnexpectedEndOfFile),
                _ => {}
            }
        }
    }
}

pub fn parse_attributes<'a, F>(tag: &BytesStart<'a>, mut f: F) -> Result<()>
where
    F: FnMut(&[u8], AttributeValue<'_>) -> Result<bool>,
{
    for attribute in tag.attributes() {
        let attribute = attribute?;
        let key = attribute.key;
        if !f(key, AttributeValue(&attribute))? {
            return Ok(());
        }
    }
    Ok(())
}

pub fn optional_attribute_err<'a, F>(tag: &BytesStart<'a>, key: &[u8], mut f: F) -> Result<()>
where
    F: FnMut(Cow<'_, str>) -> Result<()>,
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

pub fn attribute_err<'a, F>(tag: &BytesStart<'a>, key: &[u8], mut f: F) -> Result<()>
where
    F: FnMut(Cow<'_, str>) -> Result<()>,
{
    let mut called = false;
    parse_attributes(tag, |k, v| {
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
        Err(Error::AttributeNotFound)
    }
}

pub fn attribute<'a, F>(tag: &BytesStart<'a>, key: &[u8], mut f: F) -> Result<()>
where
    F: FnMut(Cow<'_, str>),
{
    attribute_err(tag, key, |t| {
        f(t);
        Ok(())
    })
}
