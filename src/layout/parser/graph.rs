use super::{color, comparison_override, end_tag, parse_bool, parse_children, text_parsed, Result};
use quick_xml::Reader;
use std::io::BufRead;

pub use crate::component::graph::Component;

pub fn settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut Component,
) -> Result<()>
where
    R: BufRead,
{
    let settings = component.settings_mut();

    parse_children(reader, buf, |reader, tag| {
        if tag.name() == b"Height" {
            text_parsed(reader, tag.into_buf(), |v| settings.height = v)
        } else if tag.name() == b"BehindGraphColor" {
            color(reader, tag.into_buf(), |c| {
                settings.behind_background_color = c
            })
        } else if tag.name() == b"AheadGraphColor" {
            color(reader, tag.into_buf(), |c| {
                settings.ahead_background_color = c
            })
        } else if tag.name() == b"GridlinesColor" {
            color(reader, tag.into_buf(), |c| settings.grid_lines_color = c)
        } else if tag.name() == b"PartialFillColorAhead" {
            // Version >= 1.2
            color(reader, tag.into_buf(), |c| settings.partial_fill_color = c)
        } else if tag.name() == b"CompleteFillColorAhead" {
            // Version >= 1.2
            color(reader, tag.into_buf(), |c| settings.complete_fill_color = c)
        } else if tag.name() == b"PartialFillColor" {
            // Version < 1.2
            color(reader, tag.into_buf(), |c| settings.partial_fill_color = c)
        } else if tag.name() == b"CompleteFillColor" {
            // Version < 1.2
            color(reader, tag.into_buf(), |c| settings.complete_fill_color = c)
        } else if tag.name() == b"GraphColor" {
            color(reader, tag.into_buf(), |c| settings.graph_lines_color = c)
        } else if tag.name() == b"LiveGraph" {
            parse_bool(reader, tag.into_buf(), |b| settings.live_graph = b)
        } else if tag.name() == b"FlipGraph" {
            parse_bool(reader, tag.into_buf(), |b| settings.flip_graph = b)
        } else if tag.name() == b"Comparison" {
            comparison_override(reader, tag.into_buf(), |v| settings.comparison_override = v)
        } else if tag.name() == b"ShowBestSegments" {
            parse_bool(reader, tag.into_buf(), |b| settings.show_best_segments = b)
        } else {
            // FIXME:
            // Width
            // PartialFillColorBehind // Version >= 1.2
            // CompleteFillColorBehind // Version >= 1.2
            // ShadowsColor
            // GraphLinesColor (separators, not our graph_lines_color)
            // GraphGoldColor
            end_tag(reader, tag.into_buf())
        }
    })
}
