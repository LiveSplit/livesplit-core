use super::{
    accuracy, color, end_tag, parse_bool, parse_children, text_parsed, timer_format,
    timing_method_override, GradientBuilder, Result,
};
use quick_xml::Reader;
use std::io::BufRead;

pub use crate::component::timer::Component;

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
    let mut override_color = false;

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"TimerHeight" {
                text_parsed(reader, tag.into_buf(), |v| settings.height = v)
            } else if tag.name() == b"TimerFormat" {
                // Version >= 1.5
                timer_format(reader, tag.into_buf(), |d, a| {
                    settings.digits_format = d;
                    settings.accuracy = a;
                })
            } else if tag.name() == b"TimerAccuracy" {
                // Version >= 1.2 && Version < 1.5
                accuracy(reader, tag.into_buf(), |v| settings.accuracy = v)
            } else if tag.name() == b"OverrideSplitColors" {
                // Version >= 1.3
                parse_bool(reader, tag.into_buf(), |b| override_color = b)
            } else if tag.name() == b"UseSplitColors" {
                // Version < 1.3
                parse_bool(reader, tag.into_buf(), |b| override_color = !b)
            } else if tag.name() == b"ShowGradient" {
                parse_bool(reader, tag.into_buf(), |b| settings.show_gradient = b)
            } else if tag.name() == b"TimerColor" {
                color(reader, tag.into_buf(), |c| {
                    settings.color_override = Some(c)
                })
            } else if tag.name() == b"TimingMethod" {
                timing_method_override(reader, tag.into_buf(), |v| settings.timing_method = v)
            } else {
                // FIXME:
                // TimerWidth
                // CenterTimer
                // DecimalsSize
                end_tag(reader, tag.into_buf())
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
