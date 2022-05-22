use alloc::{borrow::Cow, string::String};
use core::{
    char::{self, REPLACEMENT_CHARACTER},
    fmt::{self, Debug, Display, Write},
    iter, str,
};

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
struct AsciiChar(u8);

impl AsciiChar {
    const LESS_THAN: Self = Self::new(b'<');
    const GREATER_THAN: Self = Self::new(b'>');
    const SPACE: Self = Self::new(b' ');
    const CARRIAGE_RETURN: Self = Self::new(b'\r');
    const NEW_LINE: Self = Self::new(b'\n');
    const TAB: Self = Self::new(b'\t');
    const AMPERSAND: Self = Self::new(b'&');
    const SEMICOLON: Self = Self::new(b';');
    const SINGLE_QUOTE: Self = Self::new(b'\'');
    const DOUBLE_QUOTE: Self = Self::new(b'"');
    const EQUALITY_SIGN: Self = Self::new(b'=');

    const fn new(c: u8) -> Self {
        if c > 127 {
            panic!("Is not an ASCII character.");
        }
        Self(c)
    }

    const unsafe fn new_unchecked(c: u8) -> Self {
        Self(c)
    }

    fn contains(self, text: &str) -> bool {
        memchr::memchr(self.0, text.as_bytes()).is_some()
    }

    fn split_once(self, text: &str) -> Option<(&str, &str)> {
        let pos = memchr::memchr(self.0, text.as_bytes())?;
        // SAFETY: memchr guarantees that the position is valid. Also since we
        // are looking for ASCII bytes, splitting at the position is guaranteed
        // to be valid UTF-8.
        unsafe { Some((text.get_unchecked(..pos), text.get_unchecked(pos + 1..))) }
    }
}

enum TagState<'a> {
    Closed,
    Opened,
    Empty(TagName<'a>),
}

#[derive(Debug)]
pub enum Event<'a> {
    Text(Text<'a>),
    Start(Tag<'a>),
    End(TagName<'a>),
    Comment(Text<'a>),
    CData(Text<'a>),
    DocType(Text<'a>),
    Decl(&'a str),
    ProcessingInstruction(Text<'a>),
    Ended,
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

#[derive(Copy, Clone)]
pub struct Tag<'a>(&'a str);

impl Debug for Tag<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<")?;
        Display::fmt(self.0, f)?;
        f.write_str("/>")
    }
}

#[derive(Copy, Clone)]
pub struct Attributes<'a>(&'a str);

fn split_whitespace(rem: &str) -> Option<(&str, &str)> {
    AsciiSet::WHITE_SPACE.split(rem)
}

fn trim(rem: &str) -> &str {
    AsciiSet::WHITE_SPACE.trim(rem)
}

fn trim_start(rem: &str) -> &str {
    AsciiSet::WHITE_SPACE.trim_start(rem)
}

fn trim_end(rem: &str) -> &str {
    AsciiSet::WHITE_SPACE.trim_end(rem)
}

impl<'a> Attributes<'a> {
    pub fn iter(self) -> impl Iterator<Item = (&'a str, Text<'a>)> + 'a {
        let mut rem = self.0;
        iter::from_fn(move || {
            rem = trim_start(rem);
            let (key, space_maybe, after) =
                AsciiSet::EQUALITY_OR_WHITE_SPACE.split_three_way(rem)?;
            rem = after;
            if space_maybe.0 != b'=' {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.try_unescape_with_fn(|chunk| Display::fmt(chunk, f))
    }
}

impl Debug for Text<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

fn strip_surrounding<'t>(before: &str, text: &'t str, after: &str) -> Option<&'t str> {
    text.strip_prefix(before)?.strip_suffix(after)
}

pub struct Reader<'a> {
    source: &'a str,
    state: TagState<'a>,
}

impl<'a> Reader<'a> {
    pub const fn new(source: &'a str) -> Self {
        Self {
            source,
            state: TagState::Closed,
        }
    }

    pub fn read_event(&mut self) -> Option<Event<'a>> {
        match self.state {
            TagState::Closed => Some(self.read_until_open()),
            TagState::Opened => self.read_until_close(),
            TagState::Empty(tag_name) => {
                self.state = TagState::Closed;
                Some(Event::End(tag_name))
            }
        }
    }

    fn read_until_close(&mut self) -> Option<Event<'a>> {
        self.state = TagState::Closed;

        Some(if let Some(rem) = self.source.strip_prefix('/') {
            self.source = rem;
            let tag_inner = self.read_until(AsciiChar::GREATER_THAN)?;
            Event::End(TagName(tag_inner))
        } else if let Some(rem) = self.source.strip_prefix('!') {
            self.source = rem;
            let tag_inner = self.read_until(AsciiChar::GREATER_THAN)?;
            if let Some(comment) = strip_surrounding("--", tag_inner, "--") {
                Event::Comment(Text(comment))
            } else if let Some(cdata) = strip_surrounding("[CDATA[", tag_inner, "]]") {
                Event::CData(Text(cdata))
            } else if let Some(doc_type) = tag_inner.strip_prefix("DOCTYPE") {
                Event::DocType(Text(doc_type))
            } else {
                return None;
            }
        } else if let Some(rem) = self.source.strip_prefix('?') {
            self.source = rem;
            let tag_inner = self.read_until(AsciiChar::GREATER_THAN)?;
            if let Some(pi) = tag_inner.strip_suffix('?') {
                if let Some(decl) = pi
                    .strip_prefix("xml")
                    .and_then(|decl| decl.strip_prefix(|b: char| b.is_ascii_whitespace()))
                {
                    Event::Decl(decl)
                } else {
                    Event::ProcessingInstruction(Text(pi))
                }
            } else {
                return None;
            }
        } else {
            let tag_inner = read_elem_until(&mut self.source, AsciiChar::GREATER_THAN)?;
            match tag_inner.strip_suffix('/') {
                Some(tag_inner) => {
                    let tag = Tag(tag_inner);
                    let (name, _) = tag.name_and_attributes();
                    self.state = TagState::Empty(name);
                    Event::Start(tag)
                }
                None => Event::Start(Tag(tag_inner)),
            }
        })
    }

    fn read_until_open(&mut self) -> Event<'a> {
        self.state = TagState::Opened;
        match self.read_until(AsciiChar::LESS_THAN) {
            Some(before) => Event::Text(Text(trim(before))),
            None => Event::Ended,
        }
    }

    fn read_until(&mut self, needle: AsciiChar) -> Option<&'a str> {
        match needle.split_once(self.source) {
            Some((before, after)) => {
                self.source = after;
                Some(before)
            }
            None => {
                self.source = "";
                None
            }
        }
    }
}

enum EscapeState {
    Elem,
    SingleQ,
    DoubleQ,
}

fn read_elem_until<'a>(source: &mut &'a str, end_byte: AsciiChar) -> Option<&'a str> {
    let mut state = EscapeState::Elem;

    // SAFETY: This relies on memchr giving us valid indices. Also since we only
    // look for ASCII characters, we can safely split the string at the
    // character while retaining the UTF-8 invariants.
    unsafe {
        for i in memchr::memchr3_iter(end_byte.0, b'\'', b'"', source.as_bytes()) {
            let c = *source.as_bytes().get_unchecked(i);
            state = match (state, c) {
                (EscapeState::Elem, b) if b == end_byte.0 => {
                    let before = source.get_unchecked(..i);
                    *source = source.get_unchecked(i + 1..);
                    return Some(before);
                }
                (EscapeState::Elem, b'\'') => EscapeState::SingleQ,
                (EscapeState::Elem, b'"') => EscapeState::DoubleQ,
                (EscapeState::SingleQ, b'\'') | (EscapeState::DoubleQ, b'"') => EscapeState::Elem,

                (state, _) => state,
            };
        }
    }

    None
}

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

impl<'a, T: fmt::Write> AttributeWriter<'a, T> {
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

pub struct DisplayValue<T>(pub T);

impl<D: fmt::Display> Value for DisplayValue<D> {
    fn write_escaped<T: fmt::Write>(self, sink: &mut T) -> fmt::Result {
        write!(EscapeSink(sink), "{}", self.0)
    }

    fn is_empty(&self) -> bool {
        let mut sink = IsEmptySink(true);
        let _ = write!(sink, "{}", self.0);
        sink.0
    }
}

impl Value for fmt::Arguments<'_> {
    fn write_escaped<T: fmt::Write>(self, sink: &mut T) -> fmt::Result {
        DisplayValue(self).write_escaped(sink)
    }

    fn is_empty(&self) -> bool {
        DisplayValue(self).is_empty()
    }
}

struct IsEmptySink(bool);

impl fmt::Write for IsEmptySink {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0 &= s.is_empty();
        Ok(())
    }
}

struct EscapeSink<'a, T>(&'a mut T);

impl<T: fmt::Write> fmt::Write for EscapeSink<'_, T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.write_escaped(self.0)
    }
}

#[repr(transparent)]
struct AsciiSet([bool; 256]);

impl AsciiSet {
    const WHITE_SPACE: &'static Self = &Self::new([
        AsciiChar::SPACE,
        AsciiChar::CARRIAGE_RETURN,
        AsciiChar::NEW_LINE,
        AsciiChar::TAB,
    ]);

    const CHARACTERS_TO_ESCAPE: &'static Self = &Self::new([
        AsciiChar::LESS_THAN,
        AsciiChar::GREATER_THAN,
        AsciiChar::AMPERSAND,
        AsciiChar::SINGLE_QUOTE,
        AsciiChar::DOUBLE_QUOTE,
    ]);

    const EQUALITY_OR_WHITE_SPACE: &'static Self = &Self::new([
        AsciiChar::EQUALITY_SIGN,
        AsciiChar::SPACE,
        AsciiChar::CARRIAGE_RETURN,
        AsciiChar::NEW_LINE,
        AsciiChar::TAB,
    ]);

    const fn new<const N: usize>(chars: [AsciiChar; N]) -> Self {
        let mut set = [false; 256];
        let mut i = 0;
        while i < N {
            let AsciiChar(c) = chars[i];
            set[c as usize] = true;
            i += 1;
        }
        Self(set)
    }

    fn split_three_way<'a>(&self, text: &'a str) -> Option<(&'a str, AsciiChar, &'a str)> {
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

    fn split<'a>(&self, text: &'a str) -> Option<(&'a str, &'a str)> {
        let pos = text.bytes().position(|c| self.0[c as usize])?;
        // SAFETY: The position is guaranteed to be valid because the position
        // is found by the `position` method. Since we only ever find positions
        // for characters that are valid ASCII characters, we can safely split
        // the string into the part before and the part after without breaking
        // any UTF-8 invariants.
        unsafe { Some((text.get_unchecked(..pos), text.get_unchecked(pos + 1..))) }
    }

    fn find_not(&self, text: &str) -> Option<usize> {
        text.bytes().position(|c| !self.0[c as usize])
    }

    fn rfind_not(&self, text: &str) -> Option<usize> {
        text.bytes().rposition(|c| !self.0[c as usize])
    }

    fn trim<'text>(&self, text: &'text str) -> &'text str {
        if let Some(pos) = self.find_not(text) {
            // SAFETY: The position is guaranteed to be valid because the
            // position is found by the find_not method. Also since we only
            // skipped ASCII characters, splitting the string at the position
            // will not result in a string that is not valid UTF-8.
            trim_end(unsafe { text.get_unchecked(pos..) })
        } else {
            ""
        }
    }

    fn trim_start<'text>(&self, text: &'text str) -> &'text str {
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

    fn trim_end<'text>(&self, text: &'text str) -> &'text str {
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

pub const NO_ATTRIBUTES: [(&str, Text<'static>); 0] = [];
