use super::{
    color, end_tag, parse_bool, parse_children, text_as_escaped_bytes_err, Alignment, Error,
    GradientBuilder, Result,
};
use quick_xml::Reader;
use std::io::BufRead;

pub use crate::component::title::Component;

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
    let mut override_title_color = false;

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if tag.name() == b"ShowGameName" {
                parse_bool(reader, tag.into_buf(), |b| settings.show_game_name = b)
            } else if tag.name() == b"ShowCategoryName" {
                parse_bool(reader, tag.into_buf(), |b| settings.show_category_name = b)
            } else if tag.name() == b"ShowAttemptCount" {
                parse_bool(reader, tag.into_buf(), |b| settings.show_attempt_count = b)
            } else if tag.name() == b"ShowFinishedRunsCount" {
                parse_bool(reader, tag.into_buf(), |b| {
                    settings.show_finished_runs_count = b
                })
            } else if tag.name() == b"OverrideTitleColor" {
                parse_bool(reader, tag.into_buf(), |b| override_title_color = b)
            } else if tag.name() == b"TextAlignment" {
                // Version >= 1.7.3
                text_as_escaped_bytes_err(reader, tag.into_buf(), |v| {
                    settings.text_alignment = match &*v {
                        b"0" => Alignment::Auto,
                        b"1" => Alignment::Left,
                        b"2" => Alignment::Center,
                        _ => return Err(Error::ParseAlignment),
                    };
                    Ok(())
                })
            } else if tag.name() == b"CenterTitle" {
                // Version >= 1.3 && Version < 1.7.3
                parse_bool(reader, tag.into_buf(), |b| {
                    settings.text_alignment = if b {
                        Alignment::Center
                    } else {
                        Alignment::Auto
                    }
                })
            } else if tag.name() == b"SingleLine" {
                parse_bool(reader, tag.into_buf(), |b| {
                    settings.display_as_single_line = b
                })
            } else if tag.name() == b"TitleColor" {
                color(reader, tag.into_buf(), |c| settings.text_color = Some(c))
            } else if tag.name() == b"DisplayGameIcon" {
                parse_bool(reader, tag.into_buf(), |b| settings.display_game_icon = b)
            } else if tag.name() == b"ShowRegion" {
                parse_bool(reader, tag.into_buf(), |b| settings.show_region = b)
            } else if tag.name() == b"ShowPlatform" {
                parse_bool(reader, tag.into_buf(), |b| settings.show_platform = b)
            } else if tag.name() == b"ShowVariables" {
                parse_bool(reader, tag.into_buf(), |b| settings.show_variables = b)
            } else {
                // FIXME:
                // OverrideTitleFont // Version >= 1.3
                // TitleFont // Version >= 1.2
                // UseLayoutSettingsFont // Version >= 1.2 && Version < 1.3
                end_tag(reader, tag.into_buf())
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
