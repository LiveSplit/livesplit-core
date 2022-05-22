use super::{
    color, comparison_override, end_tag, parse_bool, parse_children, text_parsed, translate_size,
    Result,
};

pub use crate::component::graph::Component;
use crate::xml::Reader;

pub fn settings(reader: &mut Reader<'_>, component: &mut Component) -> Result<()> {
    let settings = component.settings_mut();

    parse_children(reader, |reader, tag, _| {
        match tag.name() {
            "Height" => text_parsed(reader, |v| settings.height = translate_size(v)),
            "BehindGraphColor" => color(reader, |c| settings.behind_background_color = c),
            "AheadGraphColor" => color(reader, |c| settings.ahead_background_color = c),
            "GridlinesColor" => color(reader, |c| settings.grid_lines_color = c),
            "PartialFillColorAhead" => {
                // Version >= 1.2
                color(reader, |c| settings.partial_fill_color = c)
            }
            "CompleteFillColorAhead" => {
                // Version >= 1.2
                color(reader, |c| settings.complete_fill_color = c)
            }
            "PartialFillColor" => {
                // Version < 1.2
                color(reader, |c| settings.partial_fill_color = c)
            }
            "CompleteFillColor" => {
                // Version < 1.2
                color(reader, |c| settings.complete_fill_color = c)
            }
            "GraphColor" => color(reader, |c| settings.graph_lines_color = c),
            "LiveGraph" => parse_bool(reader, |b| settings.live_graph = b),
            "FlipGraph" => parse_bool(reader, |b| settings.flip_graph = b),
            "Comparison" => comparison_override(reader, |v| settings.comparison_override = v),
            "ShowBestSegments" => parse_bool(reader, |b| settings.show_best_segments = b),
            _ => {
                // FIXME:
                // Width
                // PartialFillColorBehind // Version >= 1.2
                // CompleteFillColorBehind // Version >= 1.2
                // ShadowsColor
                // GraphLinesColor (separators, not our graph_lines_color)
                // GraphGoldColor
                end_tag(reader)
            }
        }
    })
}
