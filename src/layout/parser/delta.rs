use super::{accuracy, color, comparison_override, parse_bool, GradientBuilder, Result};
use crate::{
    xml::Reader,
    xml_util::{end_tag, parse_children},
};

pub use crate::component::delta::Component;

pub fn settings(reader: &mut Reader<'_>, component: &mut Component) -> Result<()> {
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();
    let mut override_label = false;

    parse_children(reader, |reader, tag, _| {
        if !background_builder.parse_background(reader, tag.name())? {
            match tag.name() {
                "TextColor" => color(reader, |c| settings.label_color = Some(c)),
                "OverrideTextColor" => parse_bool(reader, |b| override_label = b),
                "Accuracy" => accuracy(reader, |a| settings.accuracy = a),
                "Comparison" => comparison_override(reader, |v| settings.comparison_override = v),
                "Display2Rows" => parse_bool(reader, |b| settings.display_two_rows = b),
                "DropDecimals" => parse_bool(reader, |b| settings.drop_decimals = b),
                _ => end_tag(reader),
            }
        } else {
            Ok(())
        }
    })?;

    if !override_label {
        settings.label_color = None;
    }
    settings.background = background_builder.build();

    Ok(())
}
