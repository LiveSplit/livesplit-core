use super::{color, end_tag, parse_bool, parse_children, text, GradientBuilder, Result};
use quick_xml::Reader;
use std::io::BufRead;

pub use crate::component::text::Component;
use crate::component::text::Text;

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
    let (mut left_center, mut right) = (String::new(), String::new());

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"TextColor" {
                color(reader, tag.into_buf(), |c| {
                    settings.left_center_color = Some(c)
                })
            } else if tag.name() == b"OverrideTextColor" {
                parse_bool(reader, tag.into_buf(), |b| override_label = b)
            } else if tag.name() == b"TimeColor" {
                color(reader, tag.into_buf(), |c| settings.right_color = Some(c))
            } else if tag.name() == b"OverrideTimeColor" {
                parse_bool(reader, tag.into_buf(), |b| override_value = b)
            } else if tag.name() == b"Text1" {
                text(reader, tag.into_buf(), |v| left_center = v.into_owned())
            } else if tag.name() == b"Text2" {
                text(reader, tag.into_buf(), |v| right = v.into_owned())
            } else if tag.name() == b"Display2Rows" {
                parse_bool(reader, tag.into_buf(), |b| settings.display_two_rows = b)
            } else {
                // FIXME:
                // Font1
                // Font2
                // OverrideFont1
                // OverrideFont2
                end_tag(reader, tag.into_buf())
            }
        } else {
            Ok(())
        }
    })?;

    if !override_label {
        settings.left_center_color = None;
    }
    if !override_value {
        settings.right_color = None;
    }
    settings.text = match (left_center.is_empty(), right.is_empty()) {
        (false, false) => Text::Split(left_center, right),
        (false, true) => Text::Center(left_center),
        _ => Text::Center(right),
    };
    settings.background = background_builder.build();

    Ok(())
}
