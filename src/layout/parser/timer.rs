use super::{
    accuracy, color, end_tag, parse_bool, parse_children, text_parsed, timer_format,
    timing_method_override, translate_size, GradientBuilder, Result,
};
use crate::util::xml::Reader;

pub use crate::component::timer::Component;

pub fn settings(reader: &mut Reader<'_>, component: &mut Component) -> Result<()> {
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();
    let mut override_color = false;

    parse_children(reader, |reader, tag, _| {
        if !background_builder.parse_background(reader, tag.name())? {
            match tag.name() {
                "TimerHeight" => text_parsed(reader, |v| settings.height = translate_size(v)),
                "TimerFormat" => {
                    // Version >= 1.5
                    timer_format(reader, |d, a| {
                        settings.digits_format = d;
                        settings.accuracy = a;
                    })
                }
                "TimerAccuracy" => {
                    // Version >= 1.2 && Version < 1.5
                    accuracy(reader, |v| settings.accuracy = v)
                }
                "OverrideSplitColors" => {
                    // Version >= 1.3
                    parse_bool(reader, |b| override_color = b)
                }
                "UseSplitColors" => {
                    // Version < 1.3
                    parse_bool(reader, |b| override_color = !b)
                }
                "ShowGradient" => parse_bool(reader, |b| settings.show_gradient = b),
                "TimerColor" => color(reader, |c| settings.color_override = Some(c)),
                "TimingMethod" => timing_method_override(reader, |v| settings.timing_method = v),
                _ => {
                    // FIXME:
                    // TimerWidth
                    // CenterTimer
                    // DecimalsSize
                    end_tag(reader)
                }
            }
        } else {
            Ok(())
        }
    })?;

    if !override_color {
        settings.color_override = None;
    }
    settings.background = background_builder.build();

    Ok(())
}
