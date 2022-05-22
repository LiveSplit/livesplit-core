use super::{color, end_tag, parse_bool, parse_children, text, GradientBuilder, Result};

pub use crate::component::text::Component;
use crate::{component::text::Text, platform::prelude::*, xml::Reader};

pub fn settings(reader: &mut Reader<'_>, component: &mut Component) -> Result<()> {
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();
    let (mut override_label, mut override_value) = (false, false);
    let (mut left_center, mut right) = (String::new(), String::new());

    parse_children(reader, |reader, tag, _| {
        if !background_builder.parse_background(reader, tag.name())? {
            match tag.name() {
                "TextColor" => color(reader, |c| settings.left_center_color = Some(c)),
                "OverrideTextColor" => parse_bool(reader, |b| override_label = b),
                "TimeColor" => color(reader, |c| settings.right_color = Some(c)),
                "OverrideTimeColor" => parse_bool(reader, |b| override_value = b),
                "Text1" => text(reader, |v| left_center = v.into_owned()),
                "Text2" => text(reader, |v| right = v.into_owned()),
                "Display2Rows" => parse_bool(reader, |b| settings.display_two_rows = b),
                _ => {
                    // FIXME:
                    // Font1
                    // Font2
                    // OverrideFont1
                    // OverrideFont2
                    end_tag(reader)
                }
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
