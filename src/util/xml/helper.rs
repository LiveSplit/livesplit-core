use crate::platform::prelude::*;
use alloc::borrow::Cow;
use core::fmt;
use core::{mem::MaybeUninit, str};

use super::Writer;
use super::{Attributes, Event, Reader, TagName, Text};

/// The Error type for XML-based splits files that couldn't be parsed.
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// Failed to parse the XML.
    Xml,
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
}

pub fn text_parsed<F, T, E>(reader: &mut Reader<'_>, f: F) -> Result<(), E>
where
    F: FnOnce(T),
    T: str::FromStr,
    E: From<Error> + From<T::Err>,
{
    text_err(reader, |t| {
        f(t.parse()?);
        Ok(())
    })
}

pub fn text<'a, F, E>(reader: &mut Reader<'a>, f: F) -> Result<(), E>
where
    F: FnOnce(Cow<'a, str>),
    E: From<Error>,
{
    text_err(reader, |t| {
        f(t);
        Ok(())
    })
}

pub fn text_err<'a, F, E>(reader: &mut Reader<'a>, f: F) -> Result<(), E>
where
    F: FnOnce(Cow<'a, str>) -> Result<(), E>,
    E: From<Error>,
{
    text_as_str_err(reader, f)
}

pub fn text_as_escaped_string_err<F, T, E>(reader: &mut Reader<'_>, f: F) -> Result<T, E>
where
    F: FnOnce(&str) -> Result<T, E>,
    E: From<Error>,
{
    loop {
        match reader.read_event().ok_or(Error::Xml)? {
            Event::Start(_) => return Err(Error::UnexpectedElement.into()),
            Event::End(_) => {
                return f("");
            }
            Event::Text(text) | Event::CData(text) => {
                if text.is_empty() {
                    continue;
                }
                let val = f(text.escaped())?;
                end_tag_immediately(reader)?;
                return Ok(val);
            }
            Event::Ended => return Err(Error::UnexpectedEndOfFile.into()),
            _ => {}
        }
    }
}

pub fn text_as_str_err<'a, F, T, E>(reader: &mut Reader<'a>, f: F) -> Result<T, E>
where
    F: FnOnce(Cow<'a, str>) -> Result<T, E>,
    E: From<Error>,
{
    loop {
        match reader.read_event().ok_or(Error::Xml)? {
            Event::Start(_) => return Err(Error::UnexpectedElement.into()),
            Event::End(_) => {
                return f(Cow::Borrowed(""));
            }
            Event::Text(text) | Event::CData(text) => {
                if text.is_empty() {
                    continue;
                }
                let text = text.unescape_cow();
                let val = f(text)?;
                end_tag_immediately(reader)?;
                return Ok(val);
            }
            Event::Ended => return Err(Error::UnexpectedEndOfFile.into()),
            _ => {}
        }
    }
}

fn end_tag_immediately<E>(reader: &mut Reader<'_>) -> Result<(), E>
where
    E: From<Error>,
{
    loop {
        match reader.read_event().ok_or(Error::Xml)? {
            Event::Start(_) => return Err(Error::UnexpectedElement.into()),
            Event::End(_) => return Ok(()),
            Event::Ended => return Err(Error::UnexpectedEndOfFile.into()),
            _ => {}
        }
    }
}

pub fn reencode_children(reader: &mut Reader<'_>, target_buf: &mut String) -> Result<(), Error> {
    let mut writer = Writer::new_skip_header(target_buf);
    let mut depth = 0usize;
    loop {
        match reader.read_event().ok_or(Error::Xml)? {
            Event::Start(start) => {
                depth += 1;
                let (name, attributes) = start.name_and_attributes();
                writer
                    .just_start_tag(name.name(), |tag| {
                        for (k, v) in attributes.iter() {
                            tag.attribute(k, v)?;
                        }
                        Ok(())
                    })
                    .map_err(|fmt::Error| Error::Xml)?;
            }
            Event::End(end) => {
                if depth == 0 {
                    return Ok(());
                }
                depth -= 1;
                writer.just_end_tag(end.name()).map_err(|_| Error::Xml)?;
            }
            Event::Text(text) => {
                writer.text(text).map_err(|_| Error::Xml)?;
            }
            Event::Comment(text) => {
                writer.comment(text).map_err(|_| Error::Xml)?;
            }
            Event::CData(text) => {
                writer.cdata(text).map_err(|_| Error::Xml)?;
            }
            Event::ProcessingInstruction(text) => {
                writer
                    .processing_instruction(text)
                    .map_err(|_| Error::Xml)?;
            }
            Event::Decl => {
                // Shouldn't really be a child anyway.
            }
            Event::DocType => {
                // A DOCTYPE is not allowed in content.
            }
            Event::Ended => return Err(Error::UnexpectedEndOfFile),
        }
    }
}

pub fn end_tag<E>(reader: &mut Reader<'_>) -> Result<(), E>
where
    E: From<Error>,
{
    let mut depth = 0;
    loop {
        match reader.read_event().ok_or(Error::Xml)? {
            Event::Start(_) => {
                depth += 1;
            }
            Event::End(_) => {
                if depth == 0 {
                    return Ok(());
                }
                depth -= 1;
            }
            Event::Ended => return Err(Error::UnexpectedEndOfFile.into()),
            _ => {}
        }
    }
}

pub fn single_child<F, T, E>(reader: &mut Reader<'_>, tag: &str, mut f: F) -> Result<T, E>
where
    F: FnMut(&mut Reader<'_>, Attributes<'_>) -> Result<T, E>,
    E: From<Error>,
{
    let mut val = None;
    parse_children::<_, E>(reader, |reader, name, attributes| {
        if name.name() == tag && val.is_none() {
            val = Some(f(reader, attributes)?);
            Ok(())
        } else {
            end_tag(reader)
        }
    })?;
    val.ok_or(Error::ElementNotFound.into())
}

pub fn parse_children<F, E>(reader: &mut Reader<'_>, mut f: F) -> Result<(), E>
where
    F: FnMut(&mut Reader<'_>, TagName<'_>, Attributes<'_>) -> Result<(), E>,
    E: From<Error>,
{
    loop {
        match reader.read_event().ok_or(Error::Xml)? {
            Event::Start(start) => {
                let (name, attributes) = start.name_and_attributes();
                f(reader, name, attributes)?;
            }
            Event::End(_) => return Ok(()),
            Event::Ended => return Err(Error::UnexpectedEndOfFile.into()),
            _ => {}
        }
    }
}

pub fn parse_base<F, E>(reader: &mut Reader<'_>, tag: &str, mut f: F) -> Result<(), E>
where
    F: FnMut(&mut Reader<'_>, Attributes<'_>) -> Result<(), E>,
    E: From<Error>,
{
    loop {
        match reader.read_event().ok_or(Error::Xml)? {
            Event::Start(start) => {
                let (name, attributes) = start.name_and_attributes();
                return if name.name() == tag {
                    f(reader, attributes)
                } else {
                    Err(Error::ElementNotFound.into())
                };
            }
            Event::Ended => return Err(Error::UnexpectedEndOfFile.into()),
            _ => {}
        }
    }
}

pub fn parse_attributes<'a, F, E>(attributes: Attributes<'a>, mut f: F) -> Result<(), E>
where
    F: FnMut(&'a str, Text<'a>) -> Result<bool, E>,
    E: From<Error>,
{
    for (key, value) in attributes.iter() {
        if !f(key, value)? {
            return Ok(());
        }
    }
    Ok(())
}

pub fn optional_attribute_escaped_err<F, E>(
    attributes: Attributes<'_>,
    key: &str,
    mut f: F,
) -> Result<(), E>
where
    F: FnMut(&str) -> Result<(), E>,
    E: From<Error>,
{
    parse_attributes(attributes, |k, v| {
        if k == key {
            f(v.escaped())?;
            Ok(false)
        } else {
            Ok(true)
        }
    })
}

pub fn attribute_escaped_err<F, E>(attributes: Attributes<'_>, key: &str, mut f: F) -> Result<(), E>
where
    F: FnMut(&str) -> Result<(), E>,
    E: From<Error>,
{
    let mut called = false;
    parse_attributes::<_, E>(attributes, |k, v| {
        if k == key {
            f(v.escaped())?;
            called = true;
            Ok(false)
        } else {
            Ok(true)
        }
    })?;
    if called {
        Ok(())
    } else {
        Err(Error::AttributeNotFound.into())
    }
}

pub fn attribute_err<'a, F, E>(attributes: Attributes<'a>, key: &str, mut f: F) -> Result<(), E>
where
    F: FnMut(Cow<'a, str>) -> Result<(), E>,
    E: From<Error>,
{
    let mut called = false;
    parse_attributes::<_, E>(attributes, |k, v| {
        if k == key {
            f(v.unescape_cow())?;
            called = true;
            Ok(false)
        } else {
            Ok(true)
        }
    })?;
    if called {
        Ok(())
    } else {
        Err(Error::AttributeNotFound.into())
    }
}

pub fn attribute<'a, F, E>(attributes: Attributes<'a>, key: &str, mut f: F) -> Result<(), E>
where
    F: FnMut(Cow<'a, str>),
    E: From<Error>,
{
    attribute_err(attributes, key, |t| {
        f(t);
        Ok(())
    })
}

pub fn image<F, E>(
    reader: &mut Reader<'_>,
    image_buf: &mut Vec<MaybeUninit<u8>>,
    f: F,
) -> Result<(), E>
where
    F: FnOnce(&[u8]),
    E: From<Error>,
{
    text_as_escaped_string_err(reader, |text| {
        if text.len() >= 216 {
            let src = &text.as_bytes()[212..];

            image_buf.resize(
                base64_simd::STANDARD.estimated_decoded_length(src.len()),
                MaybeUninit::uninit(),
            );

            if let Ok(decoded) =
                base64_simd::STANDARD.decode(src, base64_simd::Out::from_uninit_slice(image_buf))
            {
                f(&decoded[2..decoded.len() - 1]);
                return Ok(());
            }
        }
        f(&[]);
        Ok(())
    })
}
