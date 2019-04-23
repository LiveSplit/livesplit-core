use super::{Error, GradientBuilder, Result};
use crate::xml_util::{end_tag, parse_children, text_parsed};
use quick_xml::Reader;
use std::io::BufRead;

pub use crate::component::blank_space::Component;

pub fn settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut Component,
) -> Result<()>
where
    R: BufRead,
{
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();

    parse_children::<_, _, Error>(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"SpaceHeight" {
                text_parsed(reader, tag.into_buf(), |h| settings.size = h)
            } else {
                // FIXME:
                // SpaceWidth
                end_tag(reader, tag.into_buf())
            }
        } else {
            Ok(())
        }
    })?;

    settings.background = background_builder.build();

    Ok(())
}
