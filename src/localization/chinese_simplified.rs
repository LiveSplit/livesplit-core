use super::{Piece, PlaceholderText, Text};

pub const fn resolve(text: Text) -> &'static str {
    match text {
        Text::StartSplit => "开始 / 分段",
        Text::StartSplitDescription => "用于分段并开始新尝试的快捷键。",
        Text::Reset => "重置",
        Text::ResetDescription => "用于重置当前尝试的快捷键。",
        Text::UndoSplit => "撤销分段",
        Text::UndoSplitDescription => "用于撤销上一个分段的快捷键。",
        Text::SkipSplit => "跳过分段",
        Text::SkipSplitDescription => "用于跳过当前分段的快捷键。",
        Text::Pause => "暂停",
        Text::PauseDescription => "用于暂停当前尝试的快捷键。也可用于开始新尝试。",
        Text::UndoAllPauses => "撤销全部暂停",
        Text::UndoAllPausesDescription => {
            "用于从当前时间中移除所有暂停时间的快捷键。适用于误暂停的情况。"
        }
        Text::PreviousComparison => "上一个比较",
        Text::PreviousComparisonDescription => "用于切换到上一个比较的快捷键。",
        Text::NextComparison => "下一个比较",
        Text::NextComparisonDescription => "用于切换到下一个比较的快捷键。",
        Text::ToggleTimingMethod => "切换计时方式",
        Text::ToggleTimingMethodDescription => "用于在“实时时间”和“游戏时间”之间切换的快捷键。",
        Text::TimerBackground => "背景",
        Text::TimerBackgroundDescription => {
            "组件背后的背景。也可以将领先/落后对应的颜色应用为背景色。"
        }
        Text::SegmentTimer => "分段计时器",
        Text::SegmentTimerDescription => "指定显示自当前分段开始的时间，还是自当前尝试开始的时间。",
        Text::TimingMethod => "计时方式",
        Text::TimingMethodDescription => "指定使用的计时方式。如未指定则使用当前计时方式。",
        Text::Height => "高度",
        Text::HeightDescription => "计时器的高度。",
        Text::TimerTextColor => "文本颜色",
        Text::TimerTextColorDescription => {
            "显示时间的颜色。如未指定，将根据当前尝试的进度自动选择。可在布局通用设置中指定这些颜色。"
        }
        Text::ShowGradient => "显示渐变",
        Text::ShowGradientDescription => "确定是否以渐变显示计时器颜色。",
        Text::DigitsFormat => "位数格式",
        Text::DigitsFormatDescription => "指定显示多少位。如果时长小于位数，则用 0 填充。",
        Text::Accuracy => "精度",
        Text::AccuracyDescription => "显示时间的精度。",
        Text::TitleBackground => "背景",
        Text::TitleBackgroundDescription => "组件背后的背景。",
        Text::TitleTextColor => "文本颜色",
        Text::TitleTextColorDescription => "标题文本颜色。如未指定则使用布局颜色。",
        Text::ShowGameName => "显示游戏名",
        Text::ShowGameNameDescription => "指定标题中是否包含游戏名。",
        Text::ShowCategoryName => "显示类别名",
        Text::ShowCategoryNameDescription => "指定标题中是否包含类别名。",
        Text::ShowFinishedRunsCount => "显示完成次数",
        Text::ShowFinishedRunsCountDescription => "指定是否显示成功完成的次数。",
        Text::ShowAttemptCount => "显示尝试次数",
        Text::ShowAttemptCountDescription => "指定是否显示总尝试次数。",
        Text::TextAlignment => "文本对齐",
        Text::TextAlignmentDescription => "指定标题的对齐方式。",
        Text::DisplayTextAsSingleLine => "单行显示文本",
        Text::DisplayTextAsSingleLineDescription => {
            "指定标题是否显示为单行，而不是分为游戏名一行和类别名一行。"
        }
        Text::DisplayGameIcon => "显示游戏图标",
        Text::DisplayGameIconDescription => "如果分段中存有游戏图标，则指定是否显示。",
        Text::ShowRegion => "显示地区",
        Text::ShowRegionDescription => {
            "类别名可扩展附加信息。若在分段编辑器的变量页提供了地区，则追加地区。"
        }
        Text::ShowPlatform => "显示平台",
        Text::ShowPlatformDescription => {
            "类别名可扩展附加信息。若在分段编辑器的变量页提供了平台，则追加平台。"
        }
        Text::ShowVariables => "显示变量",
        Text::ShowVariablesDescription => {
            "类别名可扩展附加信息。追加分段编辑器变量页提供的变量。此处指 speedrun.com 变量而非自定义变量。"
        }
        Text::TotalPlaytimeBackground => "背景",
        Text::TotalPlaytimeBackgroundDescription => "组件背后的背景。",
        Text::DisplayTwoRows => "显示为两行",
        Text::DisplayTwoRowsDescription => "指定是否将组件名称与总游玩时间显示为两行。",
        Text::ShowDays => "显示天数（>24小时）",
        Text::ShowDaysDescription => "当总游玩时间达到 24 小时或以上时，指定是否显示天数。",
        Text::LabelColor => "标签颜色",
        Text::LabelColorDescription => "组件名称颜色。如未指定则使用布局颜色。",
        Text::ValueColor => "数值颜色",
        Text::ValueColorDescription => "总游玩时间颜色。如未指定则使用布局颜色。",
        Text::BlankSpaceBackground => "背景",
        Text::BlankSpaceBackgroundDescription => "组件背后的背景。",
        Text::BlankSpaceSize => "大小",
        Text::BlankSpaceSizeDescription => "组件大小。",
        Text::CurrentComparisonBackground => "背景",
        Text::CurrentComparisonBackgroundDescription => "组件背后的背景。",
        Text::CurrentComparisonDisplayTwoRows => "显示为两行",
        Text::CurrentComparisonDisplayTwoRowsDescription => "指定是否将组件名称与比较显示为两行。",
        Text::CurrentComparisonLabelColor => "标签颜色",
        Text::CurrentComparisonLabelColorDescription => "组件名称颜色。如未指定则使用布局颜色。",
        Text::CurrentComparisonValueColor => "数值颜色",
        Text::CurrentComparisonValueColorDescription => "比较名称颜色。如未指定则使用布局颜色。",
        Text::CurrentPaceBackground => "背景",
        Text::CurrentPaceBackgroundDescription => "组件背后的背景。",
        Text::CurrentPaceComparison => "比较",
        Text::CurrentPaceComparisonDescription => {
            "用于预测最终时间的比较。如未指定则使用当前比较。"
        }
        Text::CurrentPaceDisplayTwoRows => "显示为两行",
        Text::CurrentPaceDisplayTwoRowsDescription => "指定是否将组件名称与预测时间显示为两行。",
        Text::CurrentPaceLabelColor => "标签颜色",
        Text::CurrentPaceLabelColorDescription => "组件名称颜色。如未指定则使用布局颜色。",
        Text::CurrentPaceValueColor => "数值颜色",
        Text::CurrentPaceValueColorDescription => "预测时间颜色。如未指定则使用布局颜色。",
        Text::CurrentPaceAccuracy => "精度",
        Text::CurrentPaceAccuracyDescription => "显示的预测时间精度。",
        Text::DeltaBackground => "背景",
        Text::DeltaBackgroundDescription => "组件背后的背景。",
        Text::DeltaComparison => "比较",
        Text::DeltaComparisonDescription => "用于计算领先或落后的比较。如未指定则使用当前比较。",
        Text::DeltaDisplayTwoRows => "显示为两行",
        Text::DeltaDisplayTwoRowsDescription => "指定是否将比较名称与差值显示为两行。",
        Text::DeltaLabelColor => "标签颜色",
        Text::DeltaLabelColorDescription => "比较名称颜色。如未指定则使用布局颜色。",
        Text::DeltaDropDecimals => "省略小数",
        Text::DeltaDropDecimalsDescription => "当显示的差值超过 1 分钟时，指定是否不再显示小数。",
        Text::DeltaAccuracy => "精度",
        Text::DeltaAccuracyDescription => "显示差值的精度。",
        Text::SumOfBestBackground => "背景",
        Text::SumOfBestBackgroundDescription => "组件背后的背景。",
        Text::SumOfBestDisplayTwoRows => "显示为两行",
        Text::SumOfBestDisplayTwoRowsDescription => "指定是否将组件名称与最佳分段总和显示为两行。",
        Text::SumOfBestLabelColor => "标签颜色",
        Text::SumOfBestLabelColorDescription => "组件名称颜色。如未指定则使用布局颜色。",
        Text::SumOfBestValueColor => "数值颜色",
        Text::SumOfBestValueColorDescription => "最佳分段总和颜色。如未指定则使用布局颜色。",
        Text::SumOfBestAccuracy => "精度",
        Text::SumOfBestAccuracyDescription => "显示最佳分段总和的精度。",
        Text::PbChanceBackground => "背景",
        Text::PbChanceBackgroundDescription => "组件背后的背景。",
        Text::PbChanceDisplayTwoRows => "显示为两行",
        Text::PbChanceDisplayTwoRowsDescription => "指定是否将组件名称与 PB 概率显示为两行。",
        Text::PbChanceLabelColor => "标签颜色",
        Text::PbChanceLabelColorDescription => "组件名称颜色。如未指定则使用布局颜色。",
        Text::PbChanceValueColor => "数值颜色",
        Text::PbChanceValueColorDescription => "PB 概率颜色。如未指定则使用布局颜色。",
        Text::PossibleTimeSaveBackground => "背景",
        Text::PossibleTimeSaveBackgroundDescription => "组件背后的背景。",
        Text::PossibleTimeSaveComparison => "比较",
        Text::PossibleTimeSaveComparisonDescription => {
            "用于计算可能节省时间的比较。如未指定则使用当前比较。"
        }
        Text::PossibleTimeSaveDisplayTwoRows => "显示为两行",
        Text::PossibleTimeSaveDisplayTwoRowsDescription => {
            "指定是否将组件名称与可能节省时间显示为两行。"
        }
        Text::PossibleTimeSaveShowTotal => "显示总可能节省时间",
        Text::PossibleTimeSaveShowTotalDescription => {
            "指定是否显示剩余全程的总可能节省时间，而不是当前分段的可能节省时间。"
        }
        Text::PossibleTimeSaveLabelColor => "标签颜色",
        Text::PossibleTimeSaveLabelColorDescription => "组件名称颜色。如未指定则使用布局颜色。",
        Text::PossibleTimeSaveValueColor => "数值颜色",
        Text::PossibleTimeSaveValueColorDescription => "可能节省时间颜色。如未指定则使用布局颜色。",
        Text::PossibleTimeSaveAccuracy => "精度",
        Text::PossibleTimeSaveAccuracyDescription => "显示可能节省时间的精度。",
        Text::PreviousSegmentBackground => "背景",
        Text::PreviousSegmentBackgroundDescription => "组件背后的背景。",
        Text::PreviousSegmentComparison => "比较",
        Text::PreviousSegmentComparisonDescription => {
            "用于计算上一段节省或损失时间的比较。如未指定则使用当前比较。"
        }
        Text::PreviousSegmentDisplayTwoRows => "显示为两行",
        Text::PreviousSegmentDisplayTwoRowsDescription => {
            "指定是否将组件名称与节省/损失时间显示为两行。"
        }
        Text::PreviousSegmentLabelColor => "标签颜色",
        Text::PreviousSegmentLabelColorDescription => "组件名称颜色。如未指定则使用布局颜色。",
        Text::PreviousSegmentDropDecimals => "省略小数",
        Text::PreviousSegmentDropDecimalsDescription => {
            "当显示时间超过 1 分钟时，指定是否省略小数。"
        }
        Text::PreviousSegmentAccuracy => "精度",
        Text::PreviousSegmentAccuracyDescription => "显示时间的精度。",
        Text::PreviousSegmentShowPossibleTimeSave => "显示可能节省时间",
        Text::PreviousSegmentShowPossibleTimeSaveDescription => {
            "指定是否在显示节省/损失时间的同时，显示上一分段可能节省的时间。"
        }
        Text::SegmentTimeBackground => "背景",
        Text::SegmentTimeBackgroundDescription => "组件背后的背景。",
        Text::SegmentTimeComparison => "比较",
        Text::SegmentTimeComparisonDescription => "分段时间使用的比较。如未指定则使用当前比较。",
        Text::SegmentTimeDisplayTwoRows => "显示为两行",
        Text::SegmentTimeDisplayTwoRowsDescription => "指定是否将组件名称与分段时间显示为两行。",
        Text::SegmentTimeLabelColor => "标签颜色",
        Text::SegmentTimeLabelColorDescription => "组件名称颜色。如未指定则使用布局颜色。",
        Text::SegmentTimeValueColor => "数值颜色",
        Text::SegmentTimeValueColorDescription => "分段时间颜色。如未指定则使用布局颜色。",
        Text::SegmentTimeAccuracy => "精度",
        Text::SegmentTimeAccuracyDescription => "显示分段时间的精度。",
        Text::GraphComparison => "比较",
        Text::GraphComparisonDescription => "图表使用的比较。如未指定则使用当前比较。",
        Text::GraphHeight => "高度",
        Text::GraphHeightDescription => "图表高度。",
        Text::GraphShowBestSegments => "显示最佳分段",
        Text::GraphShowBestSegmentsDescription => "指定是否使用布局的最佳分段颜色来标记最佳分段。",
        Text::GraphLiveGraph => "实时图表",
        Text::GraphLiveGraphDescription => {
            "指定图表是否持续自动刷新。若禁用，只有当前分段变化时才更新。"
        }
        Text::GraphFlipGraph => "翻转图表",
        Text::GraphFlipGraphDescription => {
            "指定是否垂直翻转图表。未启用时，领先在 x 轴下方，落后在上方；启用后反转。"
        }
        Text::GraphBehindBackgroundColor => "落后背景色",
        Text::GraphBehindBackgroundColorDescription => "图表中落后区域的背景色。",
        Text::GraphAheadBackgroundColor => "领先背景色",
        Text::GraphAheadBackgroundColorDescription => "图表中领先区域的背景色。",
        Text::GraphGridLinesColor => "网格线颜色",
        Text::GraphGridLinesColorDescription => "图表网格线的颜色。",
        Text::GraphLinesColor => "曲线颜色",
        Text::GraphLinesColorDescription => "连接图表点的线条颜色。",
        Text::GraphPartialFillColor => "部分填充颜色",
        Text::GraphPartialFillColorDescription => {
            "x 轴与图表之间的填充颜色。部分填充仅用于实时变化区间（从最后一次分段到当前时间）。"
        }
        Text::GraphCompleteFillColor => "完全填充颜色",
        Text::GraphCompleteFillColorDescription => {
            "x 轴与图表之间的填充颜色（不包含实时变化区间）。"
        }
        Text::DetailedTimerBackground => "背景",
        Text::DetailedTimerBackgroundDescription => "组件背后的背景。",
        Text::DetailedTimerTimingMethod => "计时方式",
        Text::DetailedTimerTimingMethodDescription => {
            "指定使用的计时方式。如未指定则使用当前计时方式。"
        }
        Text::DetailedTimerComparison1 => "比较 1",
        Text::DetailedTimerComparison1Description => {
            "显示分段时间的第一个比较。如未指定则使用当前比较。"
        }
        Text::DetailedTimerComparison2 => "比较 2",
        Text::DetailedTimerComparison2Description => {
            "显示分段时间的第二个比较。如未指定则使用当前比较，除非比较 1 也为 None。若隐藏第二比较则不显示。"
        }
        Text::DetailedTimerHideSecondComparison => "隐藏第二比较",
        Text::DetailedTimerHideSecondComparisonDescription => "指定是否只显示一个比较。",
        Text::DetailedTimerTimerHeight => "计时器高度",
        Text::DetailedTimerTimerHeightDescription => "主计时器高度。",
        Text::DetailedTimerSegmentTimerHeight => "分段计时器高度",
        Text::DetailedTimerSegmentTimerHeightDescription => "分段计时器高度。",
        Text::DetailedTimerTimerColor => "计时器颜色",
        Text::DetailedTimerTimerColorDescription => {
            "可指定主计时器的固定颜色，而非根据当前进度自动选择。"
        }
        Text::DetailedTimerShowTimerGradient => "显示计时器渐变",
        Text::DetailedTimerShowTimerGradientDescription => {
            "启用时主计时器颜色自动变为竖直渐变；否则使用纯色。"
        }
        Text::DetailedTimerTimerDigitsFormat => "计时器位数格式",
        Text::DetailedTimerTimerDigitsFormatDescription => {
            "指定主计时器显示的位数。若时长较短则用 0 填充。"
        }
        Text::DetailedTimerTimerAccuracy => "计时器精度",
        Text::DetailedTimerTimerAccuracyDescription => "主计时器显示时间的精度。",
        Text::DetailedTimerSegmentTimerColor => "分段计时器颜色",
        Text::DetailedTimerSegmentTimerColorDescription => "将分段计时器颜色改为与默认不同的颜色。",
        Text::DetailedTimerShowSegmentTimerGradient => "显示分段计时器渐变",
        Text::DetailedTimerShowSegmentTimerGradientDescription => {
            "启用时分段计时器颜色自动变为竖直渐变；否则使用纯色。"
        }
        Text::DetailedTimerSegmentTimerDigitsFormat => "分段计时器位数格式",
        Text::DetailedTimerSegmentTimerDigitsFormatDescription => {
            "指定分段计时器显示的位数。若时长较短则用 0 填充。"
        }
        Text::DetailedTimerSegmentTimerAccuracy => "分段计时器精度",
        Text::DetailedTimerSegmentTimerAccuracyDescription => "分段计时器显示时间的精度。",
        Text::DetailedTimerComparisonNamesColor => "比较名称颜色",
        Text::DetailedTimerComparisonNamesColorDescription => {
            "显示比较名称时的颜色。如未指定则使用布局颜色。"
        }
        Text::DetailedTimerComparisonTimesColor => "比较时间颜色",
        Text::DetailedTimerComparisonTimesColorDescription => {
            "显示比较时间时的颜色。如未指定则使用布局颜色。"
        }
        Text::DetailedTimerComparisonTimesAccuracy => "比较时间精度",
        Text::DetailedTimerComparisonTimesAccuracyDescription => "比较时间的精度。",
        Text::DetailedTimerShowSegmentName => "显示分段名称",
        Text::DetailedTimerShowSegmentNameDescription => "指定是否显示分段名称。",
        Text::DetailedTimerSegmentNameColor => "分段名称颜色",
        Text::DetailedTimerSegmentNameColorDescription => {
            "显示分段名称时的颜色。如未指定则使用布局颜色。"
        }
        Text::DetailedTimerDisplayIcon => "显示图标",
        Text::DetailedTimerDisplayIconDescription => "指定是否显示分段图标。",
        Text::SplitsBackground => "背景",
        Text::SplitsBackgroundDescription => {
            "组件背后的背景。可选择交替颜色；若启用，行会在两种颜色之间交替。"
        }
        Text::SplitsTotalRows => "总行数",
        Text::SplitsTotalRowsDescription => {
            "列表显示的总行数。为 0 时显示所有分段。若小于总分段数，则仅显示可滚动窗口。"
        }
        Text::SplitsUpcomingSegments => "即将到来的分段",
        Text::SplitsUpcomingSegmentsDescription => {
            "当分段多于显示行数时，窗口会随当前分段变化自动滚动。该值决定窗口中至少显示的未来分段数量。"
        }
        Text::SplitsShowThinSeparators => "显示细分隔线",
        Text::SplitsShowThinSeparatorsDescription => "指定是否在分段行之间显示细分隔线。",
        Text::SplitsShowSeparatorBeforeLastSplit => "在最后一段前显示分隔线",
        Text::SplitsShowSeparatorBeforeLastSplitDescription => {
            "若最后一段始终显示，则当其不与上一段相邻时，是否在其前显示更明显的分隔线。"
        }
        Text::SplitsAlwaysShowLastSplit => "始终显示最后一段",
        Text::SplitsAlwaysShowLastSplitDescription => {
            "若窗口不显示所有分段，指定是否始终显示最后一段，因为其中包含所选比较的总时长。"
        }
        Text::SplitsFillWithBlankSpace => "用空白填充",
        Text::SplitsFillWithBlankSpaceDescription => {
            "当分段不足以填满列表时，可用空白填充剩余行，以始终显示设置的总行数。"
        }
        Text::SplitsShowTimesBelowSegmentName => "在分段名下显示时间",
        Text::SplitsShowTimesBelowSegmentNameDescription => {
            "指定是否在分段名下方显示时间；否则在旁边显示。"
        }
        Text::SplitsCurrentSegmentGradient => "当前分段渐变",
        Text::SplitsCurrentSegmentGradientDescription => "用于突出当前分段的背景渐变。",
        Text::SplitsSplitTimeAccuracy => "分段总时间精度",
        Text::SplitsSplitTimeAccuracyDescription => "用于显示分段总时间列的精度。",
        Text::SplitsSegmentTimeAccuracy => "分段时间精度",
        Text::SplitsSegmentTimeAccuracyDescription => "用于显示分段时间列的精度。",
        Text::SplitsDeltaTimeAccuracy => "差值时间精度",
        Text::SplitsDeltaTimeAccuracyDescription => "用于显示领先/落后时间列的精度。",
        Text::SplitsDropDeltaDecimals => "显示分钟时省略差值小数",
        Text::SplitsDropDeltaDecimalsDescription => "当差值列超过 1 分钟时是否不再显示小数。",
        Text::SplitsShowColumnLabels => "显示列标题",
        Text::SplitsShowColumnLabelsDescription => "指定是否在列表顶部显示列名称。",
        Text::SplitsColumns => "列",
        Text::SplitsColumnsDescription => "每行显示的列数。每列可显示不同信息。列从右到左定义。",
        Text::SplitsColumnName => "列名",
        Text::SplitsColumnNameDescription => "列名。若启用列标题，则显示在列表顶部。",
        Text::SplitsColumnType => "列类型",
        Text::SplitsColumnTypeDescription => "该列显示的信息类型。可以是时间或自定义变量。",
        Text::SplitsVariableName => "变量名",
        Text::SplitsVariableNameDescription => "该列显示的自定义变量名称。",
        Text::SplitsStartWith => "起始值",
        Text::SplitsStartWithDescription => {
            "该列在每个分段开始时的值。更新触发条件满足后会替换为更新值。"
        }
        Text::SplitsUpdateWith => "更新值",
        Text::SplitsUpdateWithDescription => {
            "当满足某个条件（通常为正在该分段或已完成）时，时间会更新为此值。"
        }
        Text::SplitsUpdateTrigger => "更新触发条件",
        Text::SplitsUpdateTriggerDescription => "更新时间替换的条件。在满足之前，时间为“起始值”。",
        Text::SplitsColumnComparison => "比较",
        Text::SplitsColumnComparisonDescription => "该列使用的比较。如未指定则使用当前比较。",
        Text::SplitsColumnTimingMethod => "计时方式",
        Text::SplitsColumnTimingMethodDescription => {
            "该列使用的计时方式。如未指定则使用当前计时方式。"
        }
        Text::TextComponentBackground => "背景",
        Text::TextComponentBackgroundDescription => "组件背后的背景。",
        Text::TextComponentUseVariable => "使用变量",
        Text::TextComponentUseVariableDescription => {
            "指定是否使用自定义变量显示动态值。自定义变量可在分段编辑器中设置，并由自动分段器提供。"
        }
        Text::TextComponentSplit => "拆分",
        Text::TextComponentSplitDescription => "指定是否将文本拆分为左右部分。否则仅显示居中文本。",
        Text::TextComponentText => "文本",
        Text::TextComponentTextDescription => "指定要居中显示的文本。",
        Text::TextComponentLeft => "左侧",
        Text::TextComponentLeftDescription => "指定要在左侧显示的文本。",
        Text::TextComponentRight => "右侧",
        Text::TextComponentRightDescription => "指定要在右侧显示的文本。",
        Text::TextComponentVariable => "变量",
        Text::TextComponentVariableDescription => "指定要显示的自定义变量名称。",
        Text::TextComponentTextColor => "文本颜色",
        Text::TextComponentTextColorDescription => "文本的颜色。",
        Text::TextComponentLeftColor => "左侧颜色",
        Text::TextComponentLeftColorDescription => "左侧文本的颜色。",
        Text::TextComponentRightColor => "右侧颜色",
        Text::TextComponentRightColorDescription => "右侧文本的颜色。",
        Text::TextComponentNameColor => "名称颜色",
        Text::TextComponentNameColorDescription => "变量名称颜色。",
        Text::TextComponentValueColor => "数值颜色",
        Text::TextComponentValueColorDescription => "变量值颜色。",
        Text::TextComponentDisplayTwoRows => "显示为两行",
        Text::TextComponentDisplayTwoRowsDescription => "指定是否将左右文本显示为两行。",
        Text::LayoutDirection => "布局方向",
        Text::LayoutDirectionDescription => "组件排列的方向。",
        Text::CustomTimerFont => "自定义计时器字体",
        Text::CustomTimerFontDescription => "允许为计时器指定自定义字体。若未设置则使用默认字体。",
        Text::CustomTimesFont => "自定义时间字体",
        Text::CustomTimesFontDescription => "允许为时间指定自定义字体。若未设置则使用默认字体。",
        Text::CustomTextFont => "自定义文本字体",
        Text::CustomTextFontDescription => "允许为文本指定自定义字体。若未设置则使用默认字体。",
        Text::TextShadow => "文本阴影",
        Text::TextShadowDescription => "允许可选地指定文本阴影颜色。",
        Text::Background => "背景",
        Text::BackgroundDescription => "整个布局的背景。",
        Text::BestSegment => "最佳分段",
        Text::BestSegmentDescription => "达成新最佳分段时使用的颜色。",
        Text::AheadGainingTime => "领先（继续领先）",
        Text::AheadGainingTimeDescription => "领先并继续领先时使用的颜色。",
        Text::AheadLosingTime => "领先（失去时间）",
        Text::AheadLosingTimeDescription => "领先但在失去时间时使用的颜色。",
        Text::BehindGainingTime => "落后（追回时间）",
        Text::BehindGainingTimeDescription => "落后但在追回时间时使用的颜色。",
        Text::BehindLosingTime => "落后（继续落后）",
        Text::BehindLosingTimeDescription => "落后并继续失去时间时使用的颜色。",
        Text::NotRunning => "未运行",
        Text::NotRunningDescription => "无活动尝试时使用的颜色。",
        Text::PersonalBest => "个人最佳",
        Text::PersonalBestDescription => "达成新的个人最佳时使用的颜色。",
        Text::Paused => "已暂停",
        Text::PausedDescription => "计时器暂停时使用的颜色。",
        Text::ThinSeparators => "细分隔线",
        Text::ThinSeparatorsDescription => "细分隔线的颜色。",
        Text::Separators => "分隔线",
        Text::SeparatorsDescription => "普通分隔线的颜色。",
        Text::TextColor => "文本",
        Text::TextColorDescription => "用于未指定自身颜色的文本的颜色。",
        Text::ComponentBlankSpace => "空白",
        Text::ComponentCurrentComparison => "当前比较",
        Text::ComponentCurrentPace => "当前配速",
        Text::ComponentDelta => "差值",
        Text::ComponentDetailedTimer => "详细计时器",
        Text::ComponentGraph => "图表",
        Text::ComponentPbChance => "PB概率",
        Text::ComponentPossibleTimeSave => "可节省时间",
        Text::ComponentPreviousSegment => "上一个分段",
        Text::ComponentSegmentTime => "分段时间",
        Text::ComponentSeparator => "分隔线",
        Text::ComponentSplits => "分段列表",
        Text::ComponentSumOfBest => "最佳分段总和",
        Text::ComponentText => "文本",
        Text::ComponentTimer => "计时器",
        Text::ComponentSegmentTimer => "分段计时器",
        Text::ComponentTitle => "标题",
        Text::ComponentTotalPlaytime => "总游玩时间",
        Text::ComponentCurrentPaceBestPossibleTime => "最佳可能时间",
        Text::ComponentCurrentPaceWorstPossibleTime => "最差可能时间",
        Text::ComponentCurrentPacePredictedTime => "预测时间",
        Text::ComponentSegmentTimeBest => "最佳分段时间",
        Text::ComponentSegmentTimeWorst => "最差分段时间",
        Text::ComponentSegmentTimeAverage => "平均分段时间",
        Text::ComponentSegmentTimeMedian => "中位数分段时间",
        Text::ComponentSegmentTimeLatest => "最新分段时间",
        Text::ComponentPossibleTimeSaveTotal => "总可节省时间",
        Text::LiveSegment => "实时分段",
        Text::LiveSegmentShort => "实时分段",
        Text::PreviousSegmentShort => "上一分段",
        Text::PreviousSegmentAbbreviation => "上段",
        Text::ComparingAgainst => "比较对象",
        Text::ComparisonShort => "比较",
        Text::CurrentPaceBestPossibleTimeShort => "最佳可时间",
        Text::CurrentPaceBestTimeShort => "最佳时间",
        Text::CurrentPaceBestPossibleTimeAbbreviation => "最佳可",
        Text::CurrentPaceWorstPossibleTimeShort => "最差可时间",
        Text::CurrentPaceWorstTimeShort => "最差时间",
        Text::CurrentPacePredictedTimeShort => "预测时间",
        Text::CurrentPaceShort => "现节奏",
        Text::CurrentPaceAbbreviation => "节奏",
        Text::Goal => "目标",
        Text::SumOfBestSegments => "最佳分段总和",
        Text::SumOfBestShort => "最优总和",
        Text::SumOfBestAbbreviation => "最优总",
        Text::PlaytimeShort => "游玩时间",
        Text::BestSegmentTimeShort => "最佳分段时间",
        Text::BestSegmentShort => "最佳分段",
        Text::WorstSegmentTimeShort => "最差分段时间",
        Text::WorstSegmentShort => "最差分段",
        Text::AverageSegmentTimeShort => "平均分段时间",
        Text::AverageSegmentShort => "平均分段",
        Text::MedianSegmentTimeShort => "中位分段时间",
        Text::MedianSegmentShort => "中位分段",
        Text::LatestSegmentTimeShort => "最新分段时间",
        Text::LatestSegmentShort => "最新分段",
        Text::SegmentTimeShort => "分段时间",
        Text::PossibleTimeSaveShort => "可节省时间",
        Text::PossibleTimeSaveAbbreviation => "可节省",
        Text::TimeSaveShort => "节省时间",
        Text::RealTime => "实时时间",
        Text::GameTime => "游戏时间",
        Text::SumOfBestCleanerStartOfRun => "本次跑开始",
        Text::SumOfBestCleanerShouldRemove => "你觉得这个分段时间不准确、需要删除吗？",
    }
}

pub const fn resolve_placeholder(text: PlaceholderText) -> &'static [Piece] {
    match text {
        PlaceholderText::SumOfBestCleanerSegmentTimeBetween => &[
            Piece::Dynamic(0),
            Piece::Static("计时，“"),
            Piece::Dynamic(2),
            Piece::Static("”到“"),
            Piece::Dynamic(3),
            Piece::Static("”这一段的分段用时被记录为"),
            Piece::Dynamic(1),
            Piece::Static("。"),
        ],
        PlaceholderText::SumOfBestCleanerFasterThanCombined => &[
            Piece::Static("这比最佳分段总和"),
            Piece::Dynamic(0),
            Piece::Static("还要快。"),
        ],
        PlaceholderText::SumOfBestCleanerRunOn => &[
            Piece::Static("该记录来自一场于"),
            Piece::Dynamic(0),
            Piece::Dynamic(1),
            Piece::Static("开始的跑图。"),
        ],
    }
}
