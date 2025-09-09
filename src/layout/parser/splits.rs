use super::{
    Error, GradientBuilder, GradientKind, ListGradientKind, Result, accuracy, comparison_override,
    end_tag, parse_bool, parse_children, text, text_parsed, timing_method_override,
};
use crate::{
    component::splits::{
        self, ColumnKind, ColumnSettings, ColumnStartWith, ColumnUpdateTrigger, ColumnUpdateWith,
        TimeColumn, VariableColumn,
    },
    platform::prelude::*,
    util::xml::{Reader, helper::text_as_escaped_string_err},
};

pub use crate::component::splits::Component;

pub fn settings(reader: &mut Reader, component: &mut Component) -> Result<()> {
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
                            let mut custom_variable = false;
                            parse_children(reader, |reader, tag, _| match tag.name() {
                                "Name" => text(reader, |v| column_name = v.into_owned()),
                                "Comparison" => {
                                    comparison_override(reader, |v| column.comparison_override = v)
                                }
                                "TimingMethod" => {
                                    timing_method_override(reader, |v| column.timing_method = v)
                                }
                                "Type" => text_as_escaped_string_err(reader, |v| {
                                    match v {
                                        "Delta" => {
                                            column.start_with = ColumnStartWith::Empty;
                                            column.update_with = ColumnUpdateWith::Delta;
                                            column.update_trigger = ColumnUpdateTrigger::Contextual;
                                        }
                                        "SplitTime" => {
                                            column.start_with = ColumnStartWith::ComparisonTime;
                                            column.update_with = ColumnUpdateWith::SplitTime;
                                            column.update_trigger =
                                                ColumnUpdateTrigger::OnEndingSegment;
                                        }
                                        "DeltaorSplitTime" => {
                                            column.start_with = ColumnStartWith::ComparisonTime;
                                            column.update_with =
                                                ColumnUpdateWith::DeltaWithFallback;
                                            column.update_trigger = ColumnUpdateTrigger::Contextual;
                                        }
                                        "SegmentDelta" => {
                                            column.start_with = ColumnStartWith::Empty;
                                            column.update_with = ColumnUpdateWith::SegmentDelta;
                                            column.update_trigger = ColumnUpdateTrigger::Contextual;
                                        }
                                        "SegmentTime" => {
                                            column.start_with =
                                                ColumnStartWith::ComparisonSegmentTime;
                                            column.update_with = ColumnUpdateWith::SegmentTime;
                                            column.update_trigger =
                                                ColumnUpdateTrigger::OnEndingSegment;
                                        }
                                        "SegmentDeltaorSegmentTime" => {
                                            column.start_with =
                                                ColumnStartWith::ComparisonSegmentTime;
                                            column.update_with =
                                                ColumnUpdateWith::SegmentDeltaWithFallback;
                                            column.update_trigger = ColumnUpdateTrigger::Contextual;
                                        }
                                        "CustomVariable" => custom_variable = true,
                                        _ => return Err(Error::ParseColumnType),
                                    };

                                    Ok(())
                                }),
                                _ => end_tag(reader),
                            })?;
                            settings.columns.insert(
                                0,
                                splits::ColumnSettings {
                                    kind: if custom_variable {
                                        ColumnKind::Variable(VariableColumn {
                                            variable_name: column_name.clone(),
                                        })
                                    } else {
                                        ColumnKind::Time(column)
                                    },
                                    name: column_name,
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
                                    column.comparison_override.clone_from(&v);
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
