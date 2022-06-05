use super::{ascii_char::AsciiChar, trim, Tag, TagName, Text};

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
        for i in memchr::memchr3_iter(end_byte.get(), b'\'', b'"', source.as_bytes()) {
            let c = *source.as_bytes().get_unchecked(i);
            state = match (state, c) {
                (EscapeState::Elem, b) if b == end_byte.get() => {
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

fn strip_surrounding<'t>(before: &str, text: &'t str, after: &str) -> Option<&'t str> {
    text.strip_prefix(before)?.strip_suffix(after)
}
