use super::{Error, GradientBuilder, Result, translate_size};
use crate::util::xml::{
    Reader,
    helper::{end_tag, parse_children, text_parsed},
};

pub use crate::component::blank_space::Component;

pub fn settings(reader: &mut Reader, component: &mut Component) -> Result<()> {
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();

    parse_children::<_, Error>(reader, |reader, tag, _| {
        if !background_builder.parse_background(reader, tag.name())? {
            if tag.name() == "SpaceHeight" {
                text_parsed(reader, |h| settings.size = translate_size(h))
            } else {
                // FIXME:
                // SpaceWidth
                end_tag(reader)
            }
        } else {
            Ok(())
        }
    })?;

    settings.background = background_builder.build();

    Ok(())
}
