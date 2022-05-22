use super::{
    accuracy, color, comparison_override, end_tag, parse_bool, parse_children, GradientBuilder,
    Result,
};

pub use crate::component::previous_segment::Component;
use crate::xml::Reader;

pub fn settings(reader: &mut Reader<'_>, component: &mut Component) -> Result<()> {
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();
    let mut override_label = false;

    parse_children(reader, |reader, tag, _| {
        if !background_builder.parse_background(reader, tag.name())? {
            match tag.name() {
                "TextColor" => color(reader, |c| settings.label_color = Some(c)),
                "OverrideTextColor" => parse_bool(reader, |b| override_label = b),
                "DeltaAccuracy" => accuracy(reader, |v| settings.accuracy = v),
                "DropDecimals" => parse_bool(reader, |b| settings.drop_decimals = b),
                "Comparison" => comparison_override(reader, |v| settings.comparison_override = v),
                "Display2Rows" => parse_bool(reader, |b| settings.display_two_rows = b),
                "ShowPossibleTimeSave" => {
                    parse_bool(reader, |b| settings.show_possible_time_save = b)
                }
                _ => {
                    // FIXME:
                    // TimeSaveAccuracy
                    end_tag(reader)
                }
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
