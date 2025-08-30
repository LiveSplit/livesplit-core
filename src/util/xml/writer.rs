use crate::run::parser::livesplit::Version;
use core::fmt::{self, Write};

use crate::util::{ascii_char::AsciiChar, ascii_set::AsciiSet};

use super::Text;

pub struct Writer<T> {
    sink: T,
}

impl<T: fmt::Write> Writer<T> {
    pub const fn new_skip_header(sink: T) -> Self {
        Self { sink }
    }

    pub fn new_with_default_header(mut sink: T) -> Result<Self, fmt::Error> {
        sink.write_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
        Ok(Self { sink })
    }

    pub fn text(&mut self, text: impl Value) -> fmt::Result {
        text.write_escaped(&mut self.sink)
    }

    pub fn tag<O, E: From<fmt::Error>, F: FnOnce(AttributeWriter<'_, T>) -> Result<O, E>>(
        &mut self,
        tag: &str,
        f: F,
    ) -> Result<O, E> {
        self.sink.write_str("<")?;
        self.sink.write_str(tag)?;
        let mut has_content = false;
        let res = f(AttributeWriter {
            writer: self,
            has_content: &mut has_content,
        })?;
        if has_content {
            self.sink.write_str("</")?;
            self.sink.write_str(tag)?;
            self.sink.write_str(">")?;
        } else {
            self.sink.write_str("/>")?;
        }
        Ok(res)
    }

    pub fn empty_tag<'a, I: IntoIterator<Item = (&'a str, impl Value)>>(
        &mut self,
        tag: &str,
        attributes: I,
    ) -> fmt::Result {
        self.tag(tag, |mut tag| {
            for (k, v) in attributes {
                tag.attribute(k, v)?;
            }
            Ok(())
        })
    }

    pub fn tag_with_content<
        'a,
        O,
        E: From<fmt::Error>,
        F: FnOnce(&mut Writer<T>) -> Result<O, E>,
        I: IntoIterator<Item = (&'a str, impl Value)>,
    >(
        &mut self,
        tag: &str,
        attributes: I,
        f: F,
    ) -> Result<O, E> {
        self.tag(tag, |mut tag| {
            for (k, v) in attributes {
                tag.attribute(k, v)?;
            }
            tag.content(f)
        })
    }

    pub fn tag_with_text_content<'a, I: IntoIterator<Item = (&'a str, impl Value)>>(
        &mut self,
        tag: &str,
        attributes: I,
        value: impl Value,
    ) -> fmt::Result {
        self.tag(tag, |mut tag| {
            for (k, v) in attributes {
                tag.attribute(k, v)?;
            }
            tag.text_content(value)
        })
    }

    pub fn comment(&mut self, text: impl Value) -> fmt::Result {
        self.sink.write_str("<!--")?;
        text.write_escaped(&mut self.sink)?;
        self.sink.write_str("-->")
    }

    pub fn cdata(&mut self, text: impl Value) -> fmt::Result {
        self.sink.write_str("<![CDATA[")?;
        text.write_escaped(&mut self.sink)?;
        self.sink.write_str("]]>")
    }

    pub fn processing_instruction(&mut self, text: impl Value) -> fmt::Result {
        self.sink.write_str("<?")?;
        text.write_escaped(&mut self.sink)?;
        self.sink.write_str("?>")
    }

    pub fn just_start_tag<
        O,
        E: From<fmt::Error>,
        F: FnOnce(&mut AttributeWriter<'_, T>) -> Result<O, E>,
    >(
        &mut self,
        tag: &str,
        f: F,
    ) -> Result<O, E> {
        self.sink.write_str("<")?;
        self.sink.write_str(tag)?;
        let mut has_content = false;
        let res = f(&mut AttributeWriter {
            writer: self,
            has_content: &mut has_content,
        })?;
        self.sink.write_str(">")?;
        Ok(res)
    }

    pub fn just_end_tag(&mut self, tag: &str) -> fmt::Result {
        self.sink.write_str("</")?;
        self.sink.write_str(tag)?;
        self.sink.write_str(">")
    }
}

pub struct AttributeWriter<'a, T> {
    writer: &'a mut Writer<T>,
    has_content: &'a mut bool,
}

impl<T: fmt::Write> AttributeWriter<'_, T> {
    pub fn attribute(&mut self, key: &str, value: impl Value) -> fmt::Result {
        self.writer.sink.write_str(" ")?;
        self.writer.sink.write_str(key)?;
        self.writer.sink.write_str("=\"")?;
        value.write_escaped(&mut self.writer.sink)?;
        self.writer.sink.write_str("\"")
    }

    pub fn content<O, E: From<fmt::Error>, F: FnOnce(&mut Writer<T>) -> Result<O, E>>(
        self,
        f: F,
    ) -> Result<O, E> {
        *self.has_content = true;
        self.writer.sink.write_str(">")?;
        f(self.writer)
    }

    pub fn text_content(self, value: impl Value) -> fmt::Result {
        if !value.is_empty() {
            self.content(|writer| writer.text(value))
        } else {
            Ok(())
        }
    }
}

pub trait Value {
    fn write_escaped<T: fmt::Write>(self, sink: &mut T) -> fmt::Result;
    fn is_empty(&self) -> bool;
}

impl Value for &str {
    fn write_escaped<T: fmt::Write>(mut self, sink: &mut T) -> fmt::Result {
        while let Some((before, char, rem)) = AsciiSet::CHARACTERS_TO_ESCAPE.split_three_way(self) {
            self = rem;
            sink.write_str(before)?;
            sink.write_str(match char {
                AsciiChar::LESS_THAN => "&lt;",
                AsciiChar::GREATER_THAN => "&gt;",
                AsciiChar::AMPERSAND => "&amp;",
                AsciiChar::SINGLE_QUOTE => "&apos;",
                _ => "&quot;",
            })?;
        }
        sink.write_str(self)
    }

    fn is_empty(&self) -> bool {
        str::is_empty(self)
    }
}

impl Value for Text<'_> {
    fn write_escaped<T: fmt::Write>(self, sink: &mut T) -> fmt::Result {
        sink.write_str(self.0)
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Value for Version {
    fn write_escaped<T: fmt::Write>(self, sink: &mut T) -> fmt::Result {
        write!(sink, "{}.{}.{}.{}", self.0, self.1, self.2, self.3)
    }

    fn is_empty(&self) -> bool {
        self.0 == 0 && self.1 == 0 && self.2 == 0 && self.3 == 0
    }
}

pub struct DisplayAlreadyEscaped<T>(pub T);

impl<D: fmt::Display> Value for DisplayAlreadyEscaped<D> {
    fn write_escaped<T: fmt::Write>(self, sink: &mut T) -> fmt::Result {
        write!(sink, "{}", self.0)
    }

    fn is_empty(&self) -> bool {
        let mut sink = IsEmptySink(true);
        let _ = write!(sink, "{}", self.0);
        sink.0
    }
}

struct IsEmptySink(bool);

impl fmt::Write for IsEmptySink {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0 &= s.is_empty();
        Ok(())
    }
}

pub const NO_ATTRIBUTES: [(&str, Text<'static>); 0] = [];
