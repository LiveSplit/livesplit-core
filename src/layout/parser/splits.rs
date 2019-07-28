use super::{
    comparison_override, end_tag, parse_bool, parse_children, text, text_err, text_parsed,
    timing_method_override, Error, GradientBuilder, GradientKind, ListGradientKind, Result,
};
use quick_xml::Reader;
use std::io::BufRead;

use crate::component::splits;
pub use crate::component::splits::Component;

pub fn settings<R>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    component: &mut Component,
) -> Result<()>
where
    R: BufRead,
{
    let settings = component.settings_mut();
    let mut split_gradient_builder = GradientBuilder::<GradientKind>::with_tags(
        b"CurrentSplitTopColor",
        b"CurrentSplitBottomColor",
        b"CurrentSplitGradient",
    );
    let mut background_builder = GradientBuilder::<ListGradientKind>::new_gradient_type();

    parse_children(reader, buf, |reader, tag| {
        if let Some(tag) = background_builder.parse_background(reader, tag)? {
            if let Some(tag) = split_gradient_builder.parse_background(reader, tag)? {
                if tag.name() == b"VisualSplitCount" {
                    text_parsed(reader, tag.into_buf(), |v| settings.visual_split_count = v)
                } else if tag.name() == b"SplitPreviewCount" {
                    text_parsed(reader, tag.into_buf(), |v| settings.split_preview_count = v)
                } else if tag.name() == b"ShowThinSeparators" {
                    parse_bool(reader, tag.into_buf(), |b| {
                        settings.show_thin_separators = b
                    })
                } else if tag.name() == b"AlwaysShowLastSplit" {
                    parse_bool(reader, tag.into_buf(), |b| {
                        settings.always_show_last_split = b
                    })
                } else if tag.name() == b"SplitPreviewCount" {
                    text_parsed(reader, tag.into_buf(), |v| settings.split_preview_count = v)
                } else if tag.name() == b"ShowBlankSplits" {
                    parse_bool(reader, tag.into_buf(), |b| {
                        settings.fill_with_blank_space = b
                    })
                } else if tag.name() == b"SeparatorLastSplit" {
                    parse_bool(reader, tag.into_buf(), |b| {
                        settings.separator_last_split = b
                    })
                } else if tag.name() == b"Display2Rows" {
                    parse_bool(reader, tag.into_buf(), |b| settings.display_two_rows = b)
                } else if tag.name() == b"ShowColumnLabels" {
                    parse_bool(reader, tag.into_buf(), |b| settings.show_column_labels = b)
                } else if tag.name() == b"Columns" {
                    // Version >= 1.5
                    settings.columns.clear();

                    parse_children(reader, tag.into_buf(), |reader, tag| {
                        let mut column = splits::ColumnSettings::default();
                        parse_children(reader, tag.into_buf(), |reader, tag| {
                            if tag.name() == b"Name" {
                                text(reader, tag.into_buf(), |v| column.name = v.into_owned())
                            } else if tag.name() == b"Comparison" {
                                comparison_override(reader, tag.into_buf(), |v| {
                                    column.comparison_override = v
                                })
                            } else if tag.name() == b"TimingMethod" {
                                timing_method_override(reader, tag.into_buf(), |v| {
                                    column.timing_method = v
                                })
                            } else if tag.name() == b"Type" {
                                text_err(reader, tag.into_buf(), |v| {
                                    use self::splits::{
                                        ColumnStartWith, ColumnUpdateTrigger, ColumnUpdateWith,
                                    };
                                    let (start_with, update_with, update_trigger) = match &*v {
                                        "Delta" => (
                                            ColumnStartWith::Empty,
                                            ColumnUpdateWith::Delta,
                                            ColumnUpdateTrigger::Contextual,
                                        ),
                                        "SplitTime" => (
                                            ColumnStartWith::ComparisonTime,
                                            ColumnUpdateWith::SplitTime,
                                            ColumnUpdateTrigger::OnEndingSegment,
                                        ),
                                        "DeltaorSplitTime" => (
                                            ColumnStartWith::ComparisonTime,
                                            ColumnUpdateWith::DeltaWithFallback,
                                            ColumnUpdateTrigger::Contextual,
                                        ),
                                        "SegmentDelta" => (
                                            ColumnStartWith::Empty,
                                            ColumnUpdateWith::SegmentDelta,
                                            ColumnUpdateTrigger::Contextual,
                                        ),
                                        "SegmentTime" => (
                                            ColumnStartWith::ComparisonSegmentTime,
                                            ColumnUpdateWith::SegmentTime,
                                            ColumnUpdateTrigger::OnEndingSegment,
                                        ),
                                        "SegmentDeltaorSegmentTime" => (
                                            ColumnStartWith::ComparisonSegmentTime,
                                            ColumnUpdateWith::SegmentDeltaWithFallback,
                                            ColumnUpdateTrigger::Contextual,
                                        ),
                                        _ => return Err(Error::ParseColumnType),
                                    };
                                    column.start_with = start_with;
                                    column.update_with = update_with;
                                    column.update_trigger = update_trigger;
                                    Ok(())
                                })
                            } else {
                                end_tag(reader, tag.into_buf())
                            }
                        })?;
                        settings.columns.insert(0, column);
                        Ok(())
                    })
                } else if tag.name() == b"Comparison" {
                    // Version < 1.5
                    comparison_override(reader, tag.into_buf(), |v| {
                        for column in &mut settings.columns {
                            column.comparison_override = v.clone();
                        }
                    })
                } else if tag.name() == b"ShowSplitTimes" {
                    // Version < 1.5
                    use self::splits::{
                        ColumnSettings, ColumnStartWith, ColumnUpdateTrigger, ColumnUpdateWith,
                    };
                    parse_bool(reader, tag.into_buf(), |b| {
                        if !b {
                            let comparison_override =
                                settings.columns.pop().and_then(|c| c.comparison_override);
                            settings.columns.clear();
                            settings.columns.push(ColumnSettings {
                                name: String::from("Time"),
                                start_with: ColumnStartWith::ComparisonTime,
                                update_with: ColumnUpdateWith::SplitTime,
                                update_trigger: ColumnUpdateTrigger::OnEndingSegment,
                                comparison_override: comparison_override.clone(),
                                timing_method: None,
                            });
                            settings.columns.push(ColumnSettings {
                                name: String::from("+/−"),
                                start_with: ColumnStartWith::Empty,
                                update_with: ColumnUpdateWith::Delta,
                                update_trigger: ColumnUpdateTrigger::Contextual,
                                comparison_override,
                                timing_method: None,
                            });
                        }
                    })
                } else {
                    // FIXME:
                    // DisplayIcons
                    // SplitWidth
                    // SplitTimesAccuracy
                    // AutomaticAbbreviations
                    // BeforeNamesColor // Version >= 1.3
                    // CurrentNamesColor // Version >= 1.3
                    // AfterNamesColor // Version >= 1.3
                    // OverrideTextColor // Version >= 1.3
                    // SplitNamesColor // Version >= 1.2 && Version < 1.3
                    // UseTextColor // Version < 1.3
                    // BeforeTimesColor
                    // CurrentTimesColor
                    // AfterTimesColor
                    // OverrideTimesColor
                    // LockLastSplit
                    // IconSize
                    // IconShadows
                    // SplitHeight
                    // DeltasAccuracy
                    // DropDecimals
                    // OverrideDeltasColor
                    // DeltasColor
                    // LabelsColor

                    // FIXME: Subsplits
                    // MinimumMajorSplits
                    // IndentBlankIcons
                    // IndentSubsplits
                    // HideSubsplits
                    // ShowSubsplits
                    // CurrentSectionOnly
                    // OverrideSubsplitColor
                    // SubsplitTopColor
                    // SubsplitBottomColor
                    // SubsplitGradient
                    // ShowHeader
                    // IndentSectionSplit
                    // ShowIconSectionSplit
                    // ShowSectionIcon
                    // HeaderTopColor
                    // HeaderBottomColor
                    // HeaderGradient
                    // OverrideHeaderColor
                    // HeaderTextColor
                    // HeaderText
                    // HeaderTimesColor
                    // HeaderTimes
                    // HeaderAccuracy
                    // SectionTimer
                    // SectionTimerColor
                    // SectionTimerGradient
                    // SectionTimerAccuracy

                    end_tag(reader, tag.into_buf())
                }
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    })?;

    settings.current_split_gradient = split_gradient_builder.build();
    settings.background = background_builder.build();

    Ok(())
}
