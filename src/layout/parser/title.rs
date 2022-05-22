use super::{
    color, end_tag, parse_bool, parse_children, text_as_escaped_string_err, Alignment, Error,
    GradientBuilder, Result,
};

pub use crate::component::title::Component;
use crate::xml::Reader;

pub fn settings(reader: &mut Reader<'_>, component: &mut Component) -> Result<()> {
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();
    let mut override_title_color = false;

    parse_children(reader, |reader, tag, _| {
        if !background_builder.parse_background(reader, tag.name())? {
            match tag.name() {
                "ShowGameName" => parse_bool(reader, |b| settings.show_game_name = b),
                "ShowCategoryName" => parse_bool(reader, |b| settings.show_category_name = b),
                "ShowAttemptCount" => parse_bool(reader, |b| settings.show_attempt_count = b),
                "ShowFinishedRunsCount" => {
                    parse_bool(reader, |b| settings.show_finished_runs_count = b)
                }
                "OverrideTitleColor" => parse_bool(reader, |b| override_title_color = b),
                "TextAlignment" => {
                    // Version >= 1.7.3
                    text_as_escaped_string_err(reader, |v| {
                        settings.text_alignment = match v {
                            "0" => Alignment::Auto,
                            "1" => Alignment::Left,
                            "2" => Alignment::Center,
                            _ => return Err(Error::ParseAlignment),
                        };
                        Ok(())
                    })
                }
                "CenterTitle" => {
                    // Version >= 1.3 && Version < 1.7.3
                    parse_bool(reader, |b| {
                        settings.text_alignment = if b {
                            Alignment::Center
                        } else {
                            Alignment::Auto
                        }
                    })
                }
                "SingleLine" => parse_bool(reader, |b| settings.display_as_single_line = b),
                "TitleColor" => color(reader, |c| settings.text_color = Some(c)),
                "DisplayGameIcon" => parse_bool(reader, |b| settings.display_game_icon = b),
                "ShowRegion" => parse_bool(reader, |b| settings.show_region = b),
                "ShowPlatform" => parse_bool(reader, |b| settings.show_platform = b),
                "ShowVariables" => parse_bool(reader, |b| settings.show_variables = b),
                _ => {
                    // FIXME:
                    // OverrideTitleFont // Version >= 1.3
                    // TitleFont // Version >= 1.2
                    // UseLayoutSettingsFont // Version >= 1.2 && Version < 1.3
                    end_tag(reader)
                }
            }
        } else {
            Ok(())
        }
    })?;

    if !override_title_color {
        settings.text_color = None;
    }
    settings.background = background_builder.build();

    Ok(())
}
