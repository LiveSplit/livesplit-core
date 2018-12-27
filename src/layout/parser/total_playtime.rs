use super::{color, end_tag, parse_bool, parse_children, GradientBuilder, Result};
use quick_xml::Reader;
use std::io::BufRead;

pub use crate::component::total_playtime::Component;

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
    let (mut override_label, mut override_value) = (false, false);

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"TextColor" {
                color(reader, tag.into_buf(), |c| settings.label_color = Some(c))
            } else if tag.name() == b"OverrideTextColor" {
                parse_bool(reader, tag.into_buf(), |b| override_label = b)
            } else if tag.name() == b"TimeColor" {
                color(reader, tag.into_buf(), |c| settings.value_color = Some(c))
            } else if tag.name() == b"OverrideTimeColor" {
                parse_bool(reader, tag.into_buf(), |b| override_value = b)
            } else if tag.name() == b"Display2Rows" {
                parse_bool(reader, tag.into_buf(), |b| settings.display_two_rows = b)
            } else if tag.name() == b"ShowTotalHours" {
                parse_bool(reader, tag.into_buf(), |b| settings.show_days = !b)
            } else {
                end_tag(reader, tag.into_buf())
            }
        } else {
            Ok(())
        }
    })?;

    if !override_label {
        settings.label_color = None;
    }
    if !override_value {
        settings.value_color = None;
    }
    settings.background = background_builder.build();

    Ok(())
}
