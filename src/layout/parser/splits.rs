use super::{
    accuracy, comparison_override, end_tag, parse_bool, parse_children, text, text_parsed,
    timing_method_override, Error, GradientBuilder, GradientKind, ListGradientKind, Result,
};
use crate::{
    component::splits::{
        self, ColumnKind, ColumnSettings, ColumnStartWith, ColumnUpdateTrigger, ColumnUpdateWith,
        TimeColumn,
    },
    platform::prelude::*,
    util::xml::{helper::text_as_escaped_string_err, Reader},
};

pub use crate::component::splits::Component;

pub fn settings(reader: &mut Reader<'_>, component: &mut Component) -> Result<()> {
    let settings = component.settings_mut();
    let mut split_gradient_builder = GradientBuilder::<GradientKind>::with_tags(
        "CurrentSplitTopColor",
        "CurrentSplitBottomColor",
        "CurrentSplitGradient",
    );
    let mut background_builder = GradientBuilder::<ListGradientKind>::new_gradient_type();

    parse_children(reader, |reader, tag, _| {
        if !background_builder.parse_background(reader, tag.name())? {
            if !split_gradient_builder.parse_background(reader, tag.name())? {
                match tag.name() {
                    "VisualSplitCount" => text_parsed(reader, |v| settings.visual_split_count = v),
                    "SplitPreviewCount" => {
                        text_parsed(reader, |v| settings.split_preview_count = v)
                    }
                    "ShowThinSeparators" => {
                        parse_bool(reader, |b| settings.show_thin_separators = b)
                    }
                    "AlwaysShowLastSplit" => {
                        parse_bool(reader, |b| settings.always_show_last_split = b)
                    }
                    "ShowBlankSplits" => parse_bool(reader, |b| settings.fill_with_blank_space = b),
                    "SeparatorLastSplit" => {
                        parse_bool(reader, |b| settings.separator_last_split = b)
                    }
                    "Display2Rows" => parse_bool(reader, |b| settings.display_two_rows = b),
                    "ShowColumnLabels" => parse_bool(reader, |b| settings.show_column_labels = b),
                    "Columns" => {
                        // Version >= 1.5
                        settings.columns.clear();

                        parse_children(reader, |reader, _, _| {
                            let mut column_name = String::new();
                            let mut column = TimeColumn::default();
                            parse_children(reader, |reader, tag, _| match tag.name() {
                                "Name" => text(reader, |v| column_name = v.into_owned()),
                                "Comparison" => {
                                    comparison_override(reader, |v| column.comparison_override = v)
                                }
                                "TimingMethod" => {
                                    timing_method_override(reader, |v| column.timing_method = v)
                                }
                                "Type" => text_as_escaped_string_err(reader, |v| {
                                    (
                                        column.start_with,
                                        column.update_with,
                                        column.update_trigger,
                                    ) = match v {
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

                                    Ok(())
                                }),
                                _ => end_tag(reader),
                            })?;
                            settings.columns.insert(
                                0,
                                splits::ColumnSettings {
                                    name: column_name,
                                    kind: ColumnKind::Time(column),
                                },
                            );
                            Ok(())
                        })
                    }
                    "Comparison" => {
                        // Version < 1.5
                        comparison_override(reader, |v| {
                            for column in &mut settings.columns {
                                if let ColumnKind::Time(column) = &mut column.kind {
                                    column.comparison_override = v.clone();
                                }
                            }
                        })
                    }
                    "ShowSplitTimes" => {
                        // Version < 1.5
                        parse_bool(reader, |b| {
                            if !b {
                                let comparison_override =
                                    settings.columns.pop().and_then(|c| match c.kind {
                                        ColumnKind::Variable(_) => None,
                                        ColumnKind::Time(c) => c.comparison_override,
                                    });

                                settings.columns.clear();
                                settings.columns.push(ColumnSettings {
                                    name: String::from("Time"),
                                    kind: ColumnKind::Time(TimeColumn {
                                        start_with: ColumnStartWith::ComparisonTime,
                                        update_with: ColumnUpdateWith::SplitTime,
                                        update_trigger: ColumnUpdateTrigger::OnEndingSegment,
                                        comparison_override: comparison_override.clone(),
                                        timing_method: None,
                                    }),
                                });
                                settings.columns.push(ColumnSettings {
                                    name: String::from("+/−"),
                                    kind: ColumnKind::Time(TimeColumn {
                                        start_with: ColumnStartWith::Empty,
                                        update_with: ColumnUpdateWith::Delta,
                                        update_trigger: ColumnUpdateTrigger::Contextual,
                                        comparison_override,
                                        timing_method: None,
                                    }),
                                });
                            }
                        })
                    }
                    "SplitTimesAccuracy" => accuracy(reader, |v| {
                        settings.split_time_accuracy = v;
                        settings.segment_time_accuracy = v;
                    }),
                    "DeltasAccuracy" => accuracy(reader, |v| settings.delta_time_accuracy = v),
                    "DropDecimals" => parse_bool(reader, |v| settings.delta_drop_decimals = v),
                    _ => {
                        // FIXME:
                        // DisplayIcons
                        // SplitWidth
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

                        end_tag(reader)
                    }
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
