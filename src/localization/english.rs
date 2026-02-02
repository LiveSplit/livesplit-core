use super::{Piece, PlaceholderText, Text};

pub const fn resolve(text: Text) -> &'static str {
    match text {
        Text::StartSplit => "Start / Split",
        Text::StartSplitDescription => {
            "The hotkey to use for splitting and starting a new attempt."
        }
        Text::Reset => "Reset",
        Text::ResetDescription => "The hotkey to use for resetting the current attempt.",
        Text::UndoSplit => "Undo Split",
        Text::UndoSplitDescription => "The hotkey to use for undoing the last split.",
        Text::SkipSplit => "Skip Split",
        Text::SkipSplitDescription => "The hotkey to use for skipping the current split.",
        Text::Pause => "Pause",
        Text::PauseDescription => {
            "The hotkey to use for pausing the current attempt. It can also be used for starting a new attempt."
        }
        Text::UndoAllPauses => "Undo All Pauses",
        Text::UndoAllPausesDescription => {
            "The hotkey to use for removing all the pause times from the current time. This is useful in case you accidentally paused and want to undo it."
        }
        Text::PreviousComparison => "Previous Comparison",
        Text::PreviousComparisonDescription => {
            "The hotkey to use for switching to the previous comparison."
        }
        Text::NextComparison => "Next Comparison",
        Text::NextComparisonDescription => {
            "The hotkey to use for switching to the next comparison."
        }
        Text::ToggleTimingMethod => "Toggle Timing Method",
        Text::ToggleTimingMethodDescription => {
            "The hotkey to use for toggling between the \"Real Time\" and \"Game Time\" timing methods."
        }
        Text::TimerBackground => "Background",
        Text::TimerBackgroundDescription => {
            "The background shown behind the component. It is also possible to apply the color associated with the time ahead or behind as the background color."
        }
        Text::SegmentTimer => "Segment Timer",
        Text::SegmentTimerDescription => {
            "Specifies whether to show how much time has passed since the start of the current segment, rather than how much time has passed since the start of the current attempt."
        }
        Text::TimingMethod => "Timing Method",
        Text::TimingMethodDescription => {
            "Specifies the timing method to use. If not specified, the current timing method is used."
        }
        Text::Height => "Height",
        Text::HeightDescription => "The height of the timer.",
        Text::TimerTextColor => "Text Color",
        Text::TimerTextColorDescription => {
            "The color of the time shown. If not specified, the color is automatically chosen based on how well the current attempt is going. Those colors can be specified in the general settings for the layout."
        }
        Text::ShowGradient => "Show Gradient",
        Text::ShowGradientDescription => {
            "Determines whether to display the timer's color as a gradient."
        }
        Text::DigitsFormat => "Digits Format",
        Text::DigitsFormatDescription => {
            "Specifies how many digits to show. If the duration is lower than the digits to be shown, zeros are shown instead."
        }
        Text::Accuracy => "Accuracy",
        Text::AccuracyDescription => "The accuracy of the time shown.",
        Text::TitleBackground => "Background",
        Text::TitleBackgroundDescription => "The background shown behind the component.",
        Text::TitleTextColor => "Text Color",
        Text::TitleTextColorDescription => {
            "The color of the title text. If no color is specified, the color is taken from the layout."
        }
        Text::ShowGameName => "Show Game Name",
        Text::ShowGameNameDescription => {
            "Specifies whether the game name should be part of the title that is being shown."
        }
        Text::ShowCategoryName => "Show Category Name",
        Text::ShowCategoryNameDescription => {
            "Specifies whether the category name should be part of the title that is being shown."
        }
        Text::ShowFinishedRunsCount => "Show Finished Runs Count",
        Text::ShowFinishedRunsCountDescription => {
            "Specifies whether the number of successfully finished attempts should be shown."
        }
        Text::ShowAttemptCount => "Show Attempt Count",
        Text::ShowAttemptCountDescription => {
            "Specifies whether the total number of attempts should be shown."
        }
        Text::TextAlignment => "Text Alignment",
        Text::TextAlignmentDescription => "Specifies the alignment of the title.",
        Text::DisplayTextAsSingleLine => "Display Text as Single Line",
        Text::DisplayTextAsSingleLineDescription => {
            "Specifies if the title should be shown as a single line, instead of being separated into one line for the game name and one for the category name."
        }
        Text::DisplayGameIcon => "Display Game Icon",
        Text::DisplayGameIconDescription => {
            "Specifies whether the game's icon should be shown, if there is a game icon stored in the splits."
        }
        Text::ShowRegion => "Show Region",
        Text::ShowRegionDescription => {
            "The category name can be extended with additional information. This extends it with the game's region, if it is provided in the variables tab of the splits editor."
        }
        Text::ShowPlatform => "Show Platform",
        Text::ShowPlatformDescription => {
            "The category name can be extended with additional information. This extends it with the platform the game is being played on, if it is provided in the variables tab of the splits editor."
        }
        Text::ShowVariables => "Show Variables",
        Text::ShowVariablesDescription => {
            "The category name can be extended with additional information. This extends it with additional variables provided in the variables tab of the splits editor. This refers to speedrun.com variables, not custom variables."
        }
        Text::TotalPlaytimeBackground => "Background",
        Text::TotalPlaytimeBackgroundDescription => "The background shown behind the component.",
        Text::DisplayTwoRows => "Display 2 Rows",
        Text::DisplayTwoRowsDescription => {
            "Specifies whether to display the name of the component and the total playtime in two separate rows."
        }
        Text::ShowDays => "Show Days (>24h)",
        Text::ShowDaysDescription => {
            "Specifies whether to show the number of days, when the total playtime reaches 24 hours or more."
        }
        Text::LabelColor => "Label Color",
        Text::LabelColorDescription => {
            "The color of the component's name. If not specified, the color is taken from the layout."
        }
        Text::ValueColor => "Value Color",
        Text::ValueColorDescription => {
            "The color of the total playtime. If not specified, the color is taken from the layout."
        }
        Text::BlankSpaceBackground => "Background",
        Text::BlankSpaceBackgroundDescription => "The background shown behind the component.",
        Text::BlankSpaceSize => "Size",
        Text::BlankSpaceSizeDescription => "The size of the component.",
        Text::CurrentComparisonBackground => "Background",
        Text::CurrentComparisonBackgroundDescription => {
            "The background shown behind the component."
        }
        Text::CurrentComparisonDisplayTwoRows => "Display 2 Rows",
        Text::CurrentComparisonDisplayTwoRowsDescription => {
            "Specifies whether to display the name of the component and the comparison in two separate rows."
        }
        Text::CurrentComparisonLabelColor => "Label Color",
        Text::CurrentComparisonLabelColorDescription => {
            "The color of the component's name. If not specified, the color is taken from the layout."
        }
        Text::CurrentComparisonValueColor => "Value Color",
        Text::CurrentComparisonValueColorDescription => {
            "The color of the comparison's name. If not specified, the color is taken from the layout."
        }
        Text::CurrentPaceBackground => "Background",
        Text::CurrentPaceBackgroundDescription => "The background shown behind the component.",
        Text::CurrentPaceComparison => "Comparison",
        Text::CurrentPaceComparisonDescription => {
            "The comparison to predict the final time from. If not specified, the current comparison is used."
        }
        Text::CurrentPaceDisplayTwoRows => "Display 2 Rows",
        Text::CurrentPaceDisplayTwoRowsDescription => {
            "Specifies whether to display the name of the component and the predicted time in two separate rows."
        }
        Text::CurrentPaceLabelColor => "Label Color",
        Text::CurrentPaceLabelColorDescription => {
            "The color of the component's name. If not specified, the color is taken from the layout."
        }
        Text::CurrentPaceValueColor => "Value Color",
        Text::CurrentPaceValueColorDescription => {
            "The color of the predicted time. If not specified, the color is taken from the layout."
        }
        Text::CurrentPaceAccuracy => "Accuracy",
        Text::CurrentPaceAccuracyDescription => "The accuracy of the predicted time shown.",
        Text::DeltaBackground => "Background",
        Text::DeltaBackgroundDescription => "The background shown behind the component.",
        Text::DeltaComparison => "Comparison",
        Text::DeltaComparisonDescription => {
            "The comparison to use for calculating how far ahead or behind the current attempt is. If not specified, the current comparison is used."
        }
        Text::DeltaDisplayTwoRows => "Display 2 Rows",
        Text::DeltaDisplayTwoRowsDescription => {
            "Specifies whether to display the name of the comparison and the delta in two separate rows."
        }
        Text::DeltaLabelColor => "Label Color",
        Text::DeltaLabelColorDescription => {
            "The color of the comparison name. If not specified, the color is taken from the layout."
        }
        Text::DeltaDropDecimals => "Drop Decimals",
        Text::DeltaDropDecimalsDescription => {
            "Specifies if the decimals should not be shown anymore when the visualized delta is over a minute."
        }
        Text::DeltaAccuracy => "Accuracy",
        Text::DeltaAccuracyDescription => "The accuracy of the delta shown.",
        Text::SumOfBestBackground => "Background",
        Text::SumOfBestBackgroundDescription => "The background shown behind the component.",
        Text::SumOfBestDisplayTwoRows => "Display 2 Rows",
        Text::SumOfBestDisplayTwoRowsDescription => {
            "Specifies whether to display the name of the component and the sum of best segments in two separate rows."
        }
        Text::SumOfBestLabelColor => "Label Color",
        Text::SumOfBestLabelColorDescription => {
            "The color of the component's name. If not specified, the color is taken from the layout."
        }
        Text::SumOfBestValueColor => "Value Color",
        Text::SumOfBestValueColorDescription => {
            "The color of the sum of best segments. If not specified, the color is taken from the layout."
        }
        Text::SumOfBestAccuracy => "Accuracy",
        Text::SumOfBestAccuracyDescription => "The accuracy of the sum of best segments shown.",
        Text::PbChanceBackground => "Background",
        Text::PbChanceBackgroundDescription => "The background shown behind the component.",
        Text::PbChanceDisplayTwoRows => "Display 2 Rows",
        Text::PbChanceDisplayTwoRowsDescription => {
            "Specifies whether to display the name of the component and the PB chance in two separate rows."
        }
        Text::PbChanceLabelColor => "Label Color",
        Text::PbChanceLabelColorDescription => {
            "The color of the component's name. If not specified, the color is taken from the layout."
        }
        Text::PbChanceValueColor => "Value Color",
        Text::PbChanceValueColorDescription => {
            "The color of the PB chance. If not specified, the color is taken from the layout."
        }
        Text::PossibleTimeSaveBackground => "Background",
        Text::PossibleTimeSaveBackgroundDescription => "The background shown behind the component.",
        Text::PossibleTimeSaveComparison => "Comparison",
        Text::PossibleTimeSaveComparisonDescription => {
            "The comparison to calculate the possible time save for. If not specified, the current comparison is used."
        }
        Text::PossibleTimeSaveDisplayTwoRows => "Display 2 Rows",
        Text::PossibleTimeSaveDisplayTwoRowsDescription => {
            "Specifies whether to display the name of the component and the possible time save in two separate rows."
        }
        Text::PossibleTimeSaveShowTotal => "Show Total Possible Time Save",
        Text::PossibleTimeSaveShowTotalDescription => {
            "Specifies whether to show the total possible time save for the remainder of the current attempt, instead of the possible time save for the current segment."
        }
        Text::PossibleTimeSaveLabelColor => "Label Color",
        Text::PossibleTimeSaveLabelColorDescription => {
            "The color of the component's name. If not specified, the color is taken from the layout."
        }
        Text::PossibleTimeSaveValueColor => "Value Color",
        Text::PossibleTimeSaveValueColorDescription => {
            "The color of the possible time save. If not specified, the color is taken from the layout."
        }
        Text::PossibleTimeSaveAccuracy => "Accuracy",
        Text::PossibleTimeSaveAccuracyDescription => {
            "The accuracy of the possible time save shown."
        }
        Text::PreviousSegmentBackground => "Background",
        Text::PreviousSegmentBackgroundDescription => "The background shown behind the component.",
        Text::PreviousSegmentComparison => "Comparison",
        Text::PreviousSegmentComparisonDescription => {
            "The comparison used for calculating how much time was saved or lost. If not specified, the current comparison is used."
        }
        Text::PreviousSegmentDisplayTwoRows => "Display 2 Rows",
        Text::PreviousSegmentDisplayTwoRowsDescription => {
            "Specifies whether to display the name of the component and how much time was saved or lost in two separate rows."
        }
        Text::PreviousSegmentLabelColor => "Label Color",
        Text::PreviousSegmentLabelColorDescription => {
            "The color of the component's name. If not specified, the color is taken from the layout."
        }
        Text::PreviousSegmentDropDecimals => "Drop Decimals",
        Text::PreviousSegmentDropDecimalsDescription => {
            "Specifies whether to drop the decimals from the time when the time shown is over a minute."
        }
        Text::PreviousSegmentAccuracy => "Accuracy",
        Text::PreviousSegmentAccuracyDescription => "The accuracy of the time shown.",
        Text::PreviousSegmentShowPossibleTimeSave => "Show Possible Time Save",
        Text::PreviousSegmentShowPossibleTimeSaveDescription => {
            "Specifies whether to show how much time could've been saved for the previous segment in addition to the time saved or lost."
        }
        Text::SegmentTimeBackground => "Background",
        Text::SegmentTimeBackgroundDescription => "The background shown behind the component.",
        Text::SegmentTimeComparison => "Comparison",
        Text::SegmentTimeComparisonDescription => {
            "The comparison for the segment time. If not specified, the current comparison is used."
        }
        Text::SegmentTimeDisplayTwoRows => "Display 2 Rows",
        Text::SegmentTimeDisplayTwoRowsDescription => {
            "Specifies whether to display the name of the component and the segment time in two separate rows."
        }
        Text::SegmentTimeLabelColor => "Label Color",
        Text::SegmentTimeLabelColorDescription => {
            "The color of the component's name. If not specified, the color is taken from the layout."
        }
        Text::SegmentTimeValueColor => "Value Color",
        Text::SegmentTimeValueColorDescription => {
            "The color of the segment time. If not specified, the color is taken from the layout."
        }
        Text::SegmentTimeAccuracy => "Accuracy",
        Text::SegmentTimeAccuracyDescription => "The accuracy of the segment time shown.",
        Text::GraphComparison => "Comparison",
        Text::GraphComparisonDescription => {
            "The comparison to use for the graph. If not specified, the current comparison is used."
        }
        Text::GraphHeight => "Height",
        Text::GraphHeightDescription => "The height of the chart.",
        Text::GraphShowBestSegments => "Show Best Segments",
        Text::GraphShowBestSegmentsDescription => {
            "Specifies whether to color the best segments with the layout's best segment color."
        }
        Text::GraphLiveGraph => "Live Graph",
        Text::GraphLiveGraphDescription => {
            "Specifies whether the graph should automatically refresh all the time. If this is deactivated, changes to the graph only happen whenever the current segment changes."
        }
        Text::GraphFlipGraph => "Flip Graph",
        Text::GraphFlipGraphDescription => {
            "Specifies whether the chart should be flipped vertically. If not enabled, split times which are ahead of the comparison are displayed below the x-axis and times which are behind are above it. Enabling this settings flips it."
        }
        Text::GraphBehindBackgroundColor => "Behind Background Color",
        Text::GraphBehindBackgroundColorDescription => {
            "The background color for the chart region containing the times that are behind the comparison."
        }
        Text::GraphAheadBackgroundColor => "Ahead Background Color",
        Text::GraphAheadBackgroundColorDescription => {
            "The background color for the chart region containing the times that are ahead of the comparison."
        }
        Text::GraphGridLinesColor => "Grid Lines Color",
        Text::GraphGridLinesColorDescription => "The color of the chart's grid lines.",
        Text::GraphLinesColor => "Graph Lines Color",
        Text::GraphLinesColorDescription => "The color of the lines connecting the graph's points.",
        Text::GraphPartialFillColor => "Partial Fill Color",
        Text::GraphPartialFillColorDescription => {
            "The color of the region enclosed by the x-axis and the graph. The partial fill color is only used for live changes. More specifically, this color is used in the interval from the last split time to the current time."
        }
        Text::GraphCompleteFillColor => "Complete Fill Color",
        Text::GraphCompleteFillColorDescription => {
            "The color of the region enclosed by the x-axis and the graph, excluding the graph segment with live changes."
        }
        Text::DetailedTimerBackground => "Background",
        Text::DetailedTimerBackgroundDescription => "The background shown behind the component.",
        Text::DetailedTimerTimingMethod => "Timing Method",
        Text::DetailedTimerTimingMethodDescription => {
            "Specifies the timing method to use. If not specified, the current timing method is used."
        }
        Text::DetailedTimerComparison1 => "Comparison 1",
        Text::DetailedTimerComparison1Description => {
            "The first comparison to show the segment time of. If not specified, the current comparison is used."
        }
        Text::DetailedTimerComparison2 => "Comparison 2",
        Text::DetailedTimerComparison2Description => {
            "The second comparison to show the segment time of. If not specified, the current comparison is used, unless the first comparison is also None. This is not shown if the second comparison is hidden."
        }
        Text::DetailedTimerHideSecondComparison => "Hide Second Comparison",
        Text::DetailedTimerHideSecondComparisonDescription => {
            "Specifies whether to only show a single comparison."
        }
        Text::DetailedTimerTimerHeight => "Timer Height",
        Text::DetailedTimerTimerHeightDescription => "The height of the run timer.",
        Text::DetailedTimerSegmentTimerHeight => "Segment Timer Height",
        Text::DetailedTimerSegmentTimerHeightDescription => "The height of the segment timer.",
        Text::DetailedTimerTimerColor => "Timer Color",
        Text::DetailedTimerTimerColorDescription => {
            "Instead of automatically determining the color for the main timer based on a how well the current attempt is doing, a specific color to always be used can be provided instead."
        }
        Text::DetailedTimerShowTimerGradient => "Show Timer Gradient",
        Text::DetailedTimerShowTimerGradientDescription => {
            "The main timer automatically turns its color into a vertical gradient if this setting is activated. Otherwise, the actual color is used instead of a gradient."
        }
        Text::DetailedTimerTimerDigitsFormat => "Timer Digits Format",
        Text::DetailedTimerTimerDigitsFormatDescription => {
            "Specifies how many digits to show for the main timer. If the duration is lower than the digits to be shown, zeros are shown instead."
        }
        Text::DetailedTimerTimerAccuracy => "Timer Accuracy",
        Text::DetailedTimerTimerAccuracyDescription => {
            "The accuracy of the time shown for the main timer."
        }
        Text::DetailedTimerSegmentTimerColor => "Segment Timer Color",
        Text::DetailedTimerSegmentTimerColorDescription => {
            "Changes the color of the segment timer to a color different from the default color."
        }
        Text::DetailedTimerShowSegmentTimerGradient => "Show Segment Timer Gradient",
        Text::DetailedTimerShowSegmentTimerGradientDescription => {
            "The segment timer automatically turns its color into a vertical gradient if this setting is activated. Otherwise, the actual color is used instead of a gradient."
        }
        Text::DetailedTimerSegmentTimerDigitsFormat => "Segment Timer Digits Format",
        Text::DetailedTimerSegmentTimerDigitsFormatDescription => {
            "Specifies how many digits to show for the segment timer. If the duration is lower than the digits to be shown, zeros are shown instead."
        }
        Text::DetailedTimerSegmentTimerAccuracy => "Segment Timer Accuracy",
        Text::DetailedTimerSegmentTimerAccuracyDescription => {
            "The accuracy of the time shown for the segment timer."
        }
        Text::DetailedTimerComparisonNamesColor => "Comparison Names Color",
        Text::DetailedTimerComparisonNamesColorDescription => {
            "The color of the comparison names if they are shown. If no color is specified, the color is taken from the layout."
        }
        Text::DetailedTimerComparisonTimesColor => "Comparison Times Color",
        Text::DetailedTimerComparisonTimesColorDescription => {
            "The color of the comparison times if they are shown. If no color is specified, the color is taken from the layout."
        }
        Text::DetailedTimerComparisonTimesAccuracy => "Comparison Times Accuracy",
        Text::DetailedTimerComparisonTimesAccuracyDescription => {
            "The accuracy of the comparison times."
        }
        Text::DetailedTimerShowSegmentName => "Show Segment Name",
        Text::DetailedTimerShowSegmentNameDescription => {
            "Specifies whether the segment name should be shown."
        }
        Text::DetailedTimerSegmentNameColor => "Segment Name Color",
        Text::DetailedTimerSegmentNameColorDescription => {
            "The color of the segment name if it's shown. If no color is specified, the color is taken from the layout."
        }
        Text::DetailedTimerDisplayIcon => "Display Icon",
        Text::DetailedTimerDisplayIconDescription => {
            "Specifies whether the segment icon should be shown."
        }
        Text::SplitsBackground => "Background",
        Text::SplitsBackgroundDescription => {
            "The background shown behind the component. You can choose for the colors to be alternating. In that case each row alternates between the two colors chosen."
        }
        Text::SplitsTotalRows => "Total Rows",
        Text::SplitsTotalRowsDescription => {
            "The total number of rows of segments to show in the list. If set to 0, all the segments are shown. If set to a number lower than the total number of segments, only a certain window of all the segments is shown. This window can scroll up or down."
        }
        Text::SplitsUpcomingSegments => "Upcoming Segments",
        Text::SplitsUpcomingSegmentsDescription => {
            "If there's more segments than rows that are shown, the window showing the segments automatically scrolls up and down when the current segment changes. This number determines the minimum number of future segments to be shown in this scrolling window."
        }
        Text::SplitsShowThinSeparators => "Show Thin Separators",
        Text::SplitsShowThinSeparatorsDescription => {
            "Specifies whether thin separators should be shown between the individual segment rows."
        }
        Text::SplitsShowSeparatorBeforeLastSplit => "Show Separator Before Last Split",
        Text::SplitsShowSeparatorBeforeLastSplitDescription => {
            "If the last segment is to always be shown, this determines whether to show a more pronounced separator in front of the last segment, if it is not directly adjacent to the segment shown right before it in the scrolling window."
        }
        Text::SplitsAlwaysShowLastSplit => "Always Show Last Split",
        Text::SplitsAlwaysShowLastSplitDescription => {
            "If not every segment is shown in the scrolling window of segments, then this option determines whether the final segment should always be shown, as it contains the total duration of the chosen comparison. This can be valuable information, as it is often the runner's Personal Best."
        }
        Text::SplitsFillWithBlankSpace => "Fill with Blank Space",
        Text::SplitsFillWithBlankSpaceDescription => {
            "If there's not enough segments to fill the list, this option allows filling the remaining rows with blank space in order to always show the number of total rows specified in the settings. Otherwise, the number of total rows shown is reduced to the actual number of segments."
        }
        Text::SplitsShowTimesBelowSegmentName => "Show Times Below Segment Name",
        Text::SplitsShowTimesBelowSegmentNameDescription => {
            "Specifies whether to show the times below the segment name. Otherwise the times are shown next to the segment name."
        }
        Text::SplitsCurrentSegmentGradient => "Current Segment Gradient",
        Text::SplitsCurrentSegmentGradientDescription => {
            "The gradient to show behind the current segment as an indicator of it being the current segment."
        }
        Text::SplitsSplitTimeAccuracy => "Split Time Accuracy",
        Text::SplitsSplitTimeAccuracyDescription => {
            "Specifies the accuracy to use for visualizing columns that contain split times."
        }
        Text::SplitsSegmentTimeAccuracy => "Segment Time Accuracy",
        Text::SplitsSegmentTimeAccuracyDescription => {
            "Specifies the accuracy to use for visualizing columns that contain segment times."
        }
        Text::SplitsDeltaTimeAccuracy => "Delta Time Accuracy",
        Text::SplitsDeltaTimeAccuracyDescription => {
            "Specifies the accuracy to use for visualizing columns that contain the amount of time you are ahead or behind."
        }
        Text::SplitsDropDeltaDecimals => "Drop Delta Decimals When Showing Minutes",
        Text::SplitsDropDeltaDecimalsDescription => {
            "Specifies if the decimals should not be shown anymore when a column that contains the amount of time you are ahead or behind is over a minute."
        }
        Text::SplitsShowColumnLabels => "Show Column Labels",
        Text::SplitsShowColumnLabelsDescription => {
            "Specifies whether to show the names of the columns at the top of the list."
        }
        Text::SplitsColumns => "Columns",
        Text::SplitsColumnsDescription => {
            "The number of columns to show in each row. Each column can be configured to show different information. The columns are defined from right to left."
        }
        Text::SplitsColumnName => "Column Name",
        Text::SplitsColumnNameDescription => {
            "The name of the column. This is shown at the top of the list if the option to show column labels is enabled."
        }
        Text::SplitsColumnType => "Column Type",
        Text::SplitsColumnTypeDescription => {
            "The type of information this column displays. This can be a time or a custom variable that you have stored in your splits."
        }
        Text::SplitsVariableName => "Variable Name",
        Text::SplitsVariableNameDescription => {
            "The name of the custom variable that this column displays."
        }
        Text::SplitsStartWith => "Start With",
        Text::SplitsStartWithDescription => {
            "The value that this column starts with for each segment. The Update Trigger determines when this time is replaced."
        }
        Text::SplitsUpdateWith => "Update With",
        Text::SplitsUpdateWithDescription => {
            "Once a certain condition is met, which is usually being on the segment or having already completed the segment, the time gets updated with the value specified here."
        }
        Text::SplitsUpdateTrigger => "Update Trigger",
        Text::SplitsUpdateTriggerDescription => {
            "The condition that needs to be met for the time to get updated with the value specified in the Update With field. Before this condition is met, the time is the value specified in the Start With field."
        }
        Text::SplitsColumnComparison => "Comparison",
        Text::SplitsColumnComparisonDescription => {
            "The comparison that is being compared against for this column. If not specified, the current comparison is used."
        }
        Text::SplitsColumnTimingMethod => "Timing Method",
        Text::SplitsColumnTimingMethodDescription => {
            "Specifies the timing method to use for this column. If not specified, the current timing method is used."
        }
        Text::TextComponentBackground => "Background",
        Text::TextComponentBackgroundDescription => "The background shown behind the component.",
        Text::TextComponentUseVariable => "Use Variable",
        Text::TextComponentUseVariableDescription => {
            "Specifies whether to use a custom variable to display a dynamic value. Custom variables can be specified in the splits editor and provided automatically by auto splitters."
        }
        Text::TextComponentSplit => "Split",
        Text::TextComponentSplitDescription => {
            "Specifies whether to split the text into a left and right part. If this is not the case then only a single centered text is displayed."
        }
        Text::TextComponentText => "Text",
        Text::TextComponentTextDescription => "Specifies the text to display in the center.",
        Text::TextComponentLeft => "Left",
        Text::TextComponentLeftDescription => "Specifies the text to display on the left.",
        Text::TextComponentRight => "Right",
        Text::TextComponentRightDescription => "Specifies the text to display on the right.",
        Text::TextComponentVariable => "Variable",
        Text::TextComponentVariableDescription => {
            "Specifies the name of the custom variable to display."
        }
        Text::TextComponentTextColor => "Text Color",
        Text::TextComponentTextColorDescription => "The color of the text.",
        Text::TextComponentLeftColor => "Left Color",
        Text::TextComponentLeftColorDescription => "The color of the text on the left.",
        Text::TextComponentRightColor => "Right Color",
        Text::TextComponentRightColorDescription => "The color of the text on the right.",
        Text::TextComponentNameColor => "Name Color",
        Text::TextComponentNameColorDescription => "The color of the variable name.",
        Text::TextComponentValueColor => "Value Color",
        Text::TextComponentValueColorDescription => "The color of the variable value.",
        Text::TextComponentDisplayTwoRows => "Display 2 Rows",
        Text::TextComponentDisplayTwoRowsDescription => {
            "Specifies whether to display the left and right text in two separate rows."
        }
        Text::LayoutDirection => "Layout Direction",
        Text::LayoutDirectionDescription => "The direction in which the components are laid out.",
        Text::CustomTimerFont => "Custom Timer Font",
        Text::CustomTimerFontDescription => {
            "Allows you to specify a custom font for the timer. If this is not set, the default font is used."
        }
        Text::CustomTimesFont => "Custom Times Font",
        Text::CustomTimesFontDescription => {
            "Allows you to specify a custom font for the times. If this is not set, the default font is used."
        }
        Text::CustomTextFont => "Custom Text Font",
        Text::CustomTextFontDescription => {
            "Allows you to specify a custom font for the text. If this is not set, the default font is used."
        }
        Text::TextShadow => "Text Shadow",
        Text::TextShadowDescription => "Allows you to optionally specify a color for text shadows.",
        Text::Background => "Background",
        Text::BackgroundDescription => "The background shown behind the entire layout.",
        Text::BestSegment => "Best Segment",
        Text::BestSegmentDescription => "The color to use for when you achieve a new best segment.",
        Text::AheadGainingTime => "Ahead (Gaining Time)",
        Text::AheadGainingTimeDescription => {
            "The color to use for when you are ahead of the comparison and are gaining even more time."
        }
        Text::AheadLosingTime => "Ahead (Losing Time)",
        Text::AheadLosingTimeDescription => {
            "The color to use for when you are ahead of the comparison, but are losing time."
        }
        Text::BehindGainingTime => "Behind (Gaining Time)",
        Text::BehindGainingTimeDescription => {
            "The color to use for when you are behind the comparison, but are gaining back time."
        }
        Text::BehindLosingTime => "Behind (Losing Time)",
        Text::BehindLosingTimeDescription => {
            "The color to use for when you are behind the comparison and are losing even more time."
        }
        Text::NotRunning => "Not Running",
        Text::NotRunningDescription => "The color to use for when there is no active attempt.",
        Text::PersonalBest => "Personal Best",
        Text::PersonalBestDescription => {
            "The color to use for when you achieve a new Personal Best."
        }
        Text::Paused => "Paused",
        Text::PausedDescription => "The color to use for when the timer is paused.",
        Text::ThinSeparators => "Thin Separators",
        Text::ThinSeparatorsDescription => "The color of thin separators.",
        Text::Separators => "Separators",
        Text::SeparatorsDescription => "The color of normal separators.",
        Text::TextColor => "Text",
        Text::TextColorDescription => {
            "The color to use for text that doesn't specify its own color."
        }
        Text::ComponentBlankSpace => "Blank Space",
        Text::ComponentCurrentComparison => "Current Comparison",
        Text::ComponentCurrentPace => "Current Pace",
        Text::ComponentDelta => "Delta",
        Text::ComponentDetailedTimer => "Detailed Timer",
        Text::ComponentGraph => "Graph",
        Text::ComponentPbChance => "PB Chance",
        Text::ComponentPossibleTimeSave => "Possible Time Save",
        Text::ComponentPreviousSegment => "Previous Segment",
        Text::ComponentSegmentTime => "Segment Time",
        Text::ComponentSeparator => "Separator",
        Text::ComponentSplits => "Splits",
        Text::ComponentSumOfBest => "Sum of Best",
        Text::ComponentText => "Text",
        Text::ComponentTimer => "Timer",
        Text::ComponentSegmentTimer => "Segment Timer",
        Text::ComponentTitle => "Title",
        Text::ComponentTotalPlaytime => "Total Playtime",
        Text::ComponentCurrentPaceBestPossibleTime => "Best Possible Time",
        Text::ComponentCurrentPaceWorstPossibleTime => "Worst Possible Time",
        Text::ComponentCurrentPacePredictedTime => "Predicted Time",
        Text::ComponentSegmentTimeBest => "Best Segment Time",
        Text::ComponentSegmentTimeWorst => "Worst Segment Time",
        Text::ComponentSegmentTimeAverage => "Average Segment Time",
        Text::ComponentSegmentTimeMedian => "Median Segment Time",
        Text::ComponentSegmentTimeLatest => "Latest Segment Time",
        Text::ComponentPossibleTimeSaveTotal => "Total Possible Time Save",
        Text::LiveSegment => "Live Segment",
        Text::LiveSegmentShort => "Live Seg.",
        Text::PreviousSegmentShort => "Prev. Segment",
        Text::PreviousSegmentAbbreviation => "Prev. Seg.",
        Text::ComparingAgainst => "Comparing Against",
        Text::ComparisonShort => "Comparison",
        Text::CurrentPaceBestPossibleTimeShort => "Best Poss. Time",
        Text::CurrentPaceBestTimeShort => "Best Time",
        Text::CurrentPaceBestPossibleTimeAbbreviation => "BPT",
        Text::CurrentPaceWorstPossibleTimeShort => "Worst Poss. Time",
        Text::CurrentPaceWorstTimeShort => "Worst Time",
        Text::CurrentPacePredictedTimeShort => "Pred. Time",
        Text::CurrentPaceShort => "Cur. Pace",
        Text::CurrentPaceAbbreviation => "Pace",
        Text::Goal => "Goal",
        Text::SumOfBestSegments => "Sum of Best Segments",
        Text::SumOfBestShort => "Sum of Best",
        Text::SumOfBestAbbreviation => "SoB",
        Text::PlaytimeShort => "Playtime",
        Text::BestSegmentTimeShort => "Best Seg. Time",
        Text::BestSegmentShort => "Best Segment",
        Text::WorstSegmentTimeShort => "Worst Seg. Time",
        Text::WorstSegmentShort => "Worst Segment",
        Text::AverageSegmentTimeShort => "Average Seg. Time",
        Text::AverageSegmentShort => "Average Segment",
        Text::MedianSegmentTimeShort => "Median Seg. Time",
        Text::MedianSegmentShort => "Median Segment",
        Text::LatestSegmentTimeShort => "Latest Seg. Time",
        Text::LatestSegmentShort => "Latest Segment",
        Text::SegmentTimeShort => "Seg. Time",
        Text::PossibleTimeSaveShort => "Possible Time Save",
        Text::PossibleTimeSaveAbbreviation => "Poss. Time Save",
        Text::TimeSaveShort => "Time Save",
        Text::RealTime => "Real Time",
        Text::GameTime => "Game Time",
        Text::SumOfBestCleanerStartOfRun => "the start of the run",
        Text::SumOfBestCleanerShouldRemove => {
            ". Do you think that this segment time is inaccurate and should be removed?"
        }
    }
}

pub const fn resolve_placeholder(text: PlaceholderText) -> &'static [Piece] {
    match text {
        PlaceholderText::SumOfBestCleanerSegmentTimeBetween => &[
            Piece::Static("You had a "),
            Piece::Dynamic(0),
            Piece::Static(" segment time of "),
            Piece::Dynamic(1),
            Piece::Static(" between “"),
            Piece::Dynamic(2),
            Piece::Static("” and “"),
            Piece::Dynamic(3),
            Piece::Static("”"),
        ],
        PlaceholderText::SumOfBestCleanerFasterThanCombined => &[
            Piece::Static(", which is faster than the combined best segments of "),
            Piece::Dynamic(0),
        ],
        PlaceholderText::SumOfBestCleanerRunOn => &[
            Piece::Static(" in a run on "),
            Piece::Dynamic(0),
            Piece::Static(" that started at "),
            Piece::Dynamic(1),
        ],
    }
}
