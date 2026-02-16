use super::{Piece, PlaceholderText, Text};

pub const fn resolve(text: Text) -> &'static str {
    match text {
        Text::StartSplit => "開始 / スプリット",
        Text::StartSplitDescription => "スプリットと新しいタイマースタートに使用するホットキー。",
        Text::Reset => "リセット",
        Text::ResetDescription => "現在の試行をリセットするホットキー。",
        Text::UndoSplit => "スプリットを取り消す",
        Text::UndoSplitDescription => "最後のスプリットを取り消すホットキー。",
        Text::SkipSplit => "スプリットをスキップ",
        Text::SkipSplitDescription => "現在のスプリットをスキップするホットキー。",
        Text::Pause => "ポーズ",
        Text::PauseDescription => {
            "現在の試行をポーズするホットキー。新しい試行の開始にも使用できます。"
        }
        Text::UndoAllPauses => "すべてのポーズを取り消す",
        Text::UndoAllPausesDescription => {
            "現在のタイムからすべてのポーズ時間を削除するホットキー。誤ってポーズした場合に便利です。"
        }
        Text::PreviousComparison => "前の比較対象",
        Text::PreviousComparisonDescription => "前の比較対象に切り替えるホットキー。",
        Text::NextComparison => "次の比較対象",
        Text::NextComparisonDescription => "次の比較対象に切り替えるホットキー。",
        Text::ToggleTimingMethod => "計測方法を切り替え",
        Text::ToggleTimingMethodDescription => "「実時間」と「ゲーム時間」を切り替えるホットキー。",
        Text::TimerBackground => "背景",
        Text::TimerBackgroundDescription => {
            "コンポーネントの背景。リード/遅れの色を背景色として適用することもできます。"
        }
        Text::SegmentTimer => "区間タイマー",
        Text::SegmentTimerDescription => {
            "現在の区間開始からの経過時間を表示するか、タイマースタートからの経過時間を表示するかを指定します。"
        }
        Text::TimingMethod => "計測方法",
        Text::TimingMethodDescription => {
            "使用する計測方法を指定します。指定しない場合は現在の計測方法を使用します。"
        }
        Text::Height => "高さ",
        Text::HeightDescription => "タイマーの高さ。",
        Text::TimerTextColor => "テキスト色",
        Text::TimerTextColorDescription => {
            "表示するタイムの色。指定しない場合は、現在の試行のペースに基づいて自動的に選択されます。これらの色はレイアウトの一般設定で指定できます。"
        }
        Text::ShowGradient => "グラデーションを表示",
        Text::ShowGradientDescription => "タイマーの色をグラデーションで表示するかどうか。",
        Text::DigitsFormat => "桁数",
        Text::DigitsFormatDescription => {
            "表示する桁数を指定します。桁数より短いタイムの場合は 0 で埋めて表示されます。"
        }
        Text::Accuracy => "精度",
        Text::AccuracyDescription => "表示するタイムの精度。",
        Text::TitleBackground => "背景",
        Text::TitleBackgroundDescription => "コンポーネントの背景。",
        Text::TitleTextColor => "テキスト色",
        Text::TitleTextColorDescription => {
            "タイトル文字の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::ShowGameName => "ゲーム名を表示",
        Text::ShowGameNameDescription => "表示するタイトルにゲーム名を含めるかどうか。",
        Text::ShowCategoryName => "カテゴリー名を表示",
        Text::ShowCategoryNameDescription => "表示するタイトルにカテゴリー名を含めるかどうか。",
        Text::ShowFinishedRunsCount => "完走数を表示",
        Text::ShowFinishedRunsCountDescription => "完走した回数を表示するかどうか。",
        Text::ShowAttemptCount => "試行回数を表示",
        Text::ShowAttemptCountDescription => "試行回数を表示するかどうか。",
        Text::TextAlignment => "テキスト配置",
        Text::TextAlignmentDescription => "タイトルの配置を指定します。",
        Text::DisplayTextAsSingleLine => "テキストを 1 行で表示",
        Text::DisplayTextAsSingleLineDescription => {
            "タイトルをゲーム名とカテゴリー名の 2 行に分けず、1 行で表示するかどうか。"
        }
        Text::DisplayGameIcon => "ゲームアイコンを表示",
        Text::DisplayGameIconDescription => {
            "スプリットにゲームアイコンが保存されている場合、それを表示するかどうか。"
        }
        Text::ShowRegion => "地域を表示",
        Text::ShowRegionDescription => {
            "カテゴリー名に追加情報を付加します。スプリットエディタの変数タブで地域が指定されている場合に地域を付加します。"
        }
        Text::ShowPlatform => "プラットフォームを表示",
        Text::ShowPlatformDescription => {
            "カテゴリー名に追加情報を付加します。スプリットエディタの変数タブでプラットフォームが指定されている場合に付加します。"
        }
        Text::ShowVariables => "変数を表示",
        Text::ShowVariablesDescription => {
            "カテゴリー名に追加情報を付加します。スプリットエディタの変数タブで指定された追加変数を付加します。speedrun.com 変数を指し、カスタム変数ではありません。"
        }
        Text::TotalPlaytimeBackground => "背景",
        Text::TotalPlaytimeBackgroundDescription => "コンポーネントの背景。",
        Text::DisplayTwoRows => "2 行で表示",
        Text::DisplayTwoRowsDescription => {
            "コンポーネント名と総プレイタイムを 2 行で表示するかどうか。"
        }
        Text::ShowDays => "日数を表示（>24 時間）",
        Text::ShowDaysDescription => {
            "総プレイタイムが 24 時間以上になったときに日数を表示するかどうか。"
        }
        Text::LabelColor => "ラベル色",
        Text::LabelColorDescription => {
            "コンポーネント名の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::ValueColor => "値の色",
        Text::ValueColorDescription => {
            "総プレイタイムの色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::BlankSpaceBackground => "背景",
        Text::BlankSpaceBackgroundDescription => "コンポーネントの背景。",
        Text::BlankSpaceSize => "サイズ",
        Text::BlankSpaceSizeDescription => "コンポーネントのサイズ。",
        Text::CurrentComparisonBackground => "背景",
        Text::CurrentComparisonBackgroundDescription => "コンポーネントの背景。",
        Text::CurrentComparisonDisplayTwoRows => "2 行で表示",
        Text::CurrentComparisonDisplayTwoRowsDescription => {
            "コンポーネント名と比較対象を 2 行で表示するかどうか。"
        }
        Text::CurrentComparisonLabelColor => "ラベル色",
        Text::CurrentComparisonLabelColorDescription => {
            "コンポーネント名の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::CurrentComparisonValueColor => "値の色",
        Text::CurrentComparisonValueColorDescription => {
            "比較対象名の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::CurrentPaceBackground => "背景",
        Text::CurrentPaceBackgroundDescription => "コンポーネントの背景。",
        Text::CurrentPaceComparison => "比較対象",
        Text::CurrentPaceComparisonDescription => {
            "最終タイムを予測するために使用する比較対象。指定しない場合は現在の比較対象を使用します。"
        }
        Text::CurrentPaceDisplayTwoRows => "2 行で表示",
        Text::CurrentPaceDisplayTwoRowsDescription => {
            "コンポーネント名と予測タイムを 2 行で表示するかどうか。"
        }
        Text::CurrentPaceLabelColor => "ラベル色",
        Text::CurrentPaceLabelColorDescription => {
            "コンポーネント名の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::CurrentPaceValueColor => "値の色",
        Text::CurrentPaceValueColorDescription => {
            "予測タイムの色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::CurrentPaceAccuracy => "精度",
        Text::CurrentPaceAccuracyDescription => "表示する予測タイムの精度。",
        Text::DeltaBackground => "背景",
        Text::DeltaBackgroundDescription => "コンポーネントの背景。",
        Text::DeltaComparison => "比較対象",
        Text::DeltaComparisonDescription => {
            "リード/遅れを計算するための比較対象。指定しない場合は現在の比較対象を使用します。"
        }
        Text::DeltaDisplayTwoRows => "2 行で表示",
        Text::DeltaDisplayTwoRowsDescription => "比較対象名とタイム差を 2 行で表示するかどうか。",
        Text::DeltaLabelColor => "ラベル色",
        Text::DeltaLabelColorDescription => {
            "比較対象名の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::DeltaDropDecimals => "小数点以下を省略",
        Text::DeltaDropDecimalsDescription => {
            "表示するタイム差が 1 分を超える場合に小数点以下を表示しないかどうか。"
        }
        Text::DeltaAccuracy => "精度",
        Text::DeltaAccuracyDescription => "表示するタイム差の精度。",
        Text::SumOfBestBackground => "背景",
        Text::SumOfBestBackgroundDescription => "コンポーネントの背景。",
        Text::SumOfBestDisplayTwoRows => "2 行で表示",
        Text::SumOfBestDisplayTwoRowsDescription => {
            "コンポーネント名と区間ベストの合計を 2 行で表示するかどうか。"
        }
        Text::SumOfBestLabelColor => "ラベル色",
        Text::SumOfBestLabelColorDescription => {
            "コンポーネント名の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::SumOfBestValueColor => "値の色",
        Text::SumOfBestValueColorDescription => {
            "区間ベストの合計の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::SumOfBestAccuracy => "精度",
        Text::SumOfBestAccuracyDescription => "表示する区間ベストの合計の精度。",
        Text::PbChanceBackground => "背景",
        Text::PbChanceBackgroundDescription => "コンポーネントの背景。",
        Text::PbChanceDisplayTwoRows => "2 行で表示",
        Text::PbChanceDisplayTwoRowsDescription => {
            "コンポーネント名と更新確率を 2 行で表示するかどうか。"
        }
        Text::PbChanceLabelColor => "ラベル色",
        Text::PbChanceLabelColorDescription => {
            "コンポーネント名の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::PbChanceValueColor => "値の色",
        Text::PbChanceValueColorDescription => {
            "更新確率の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::PossibleTimeSaveBackground => "背景",
        Text::PossibleTimeSaveBackgroundDescription => "コンポーネントの背景。",
        Text::PossibleTimeSaveComparison => "比較対象",
        Text::PossibleTimeSaveComparisonDescription => {
            "更新余地を計算する比較対象。指定しない場合は現在の比較対象を使用します。"
        }
        Text::PossibleTimeSaveDisplayTwoRows => "2 行で表示",
        Text::PossibleTimeSaveDisplayTwoRowsDescription => {
            "コンポーネント名と更新余地を 2 行で表示するかどうか。"
        }
        Text::PossibleTimeSaveShowTotal => "ゴール時の更新余地を表示",
        Text::PossibleTimeSaveShowTotalDescription => {
            "現在の区間ではなく、ゴール時における更新余地を表示するかどうか。"
        }
        Text::PossibleTimeSaveLabelColor => "ラベル色",
        Text::PossibleTimeSaveLabelColorDescription => {
            "コンポーネント名の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::PossibleTimeSaveValueColor => "値の色",
        Text::PossibleTimeSaveValueColorDescription => {
            "更新余地の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::PossibleTimeSaveAccuracy => "精度",
        Text::PossibleTimeSaveAccuracyDescription => "表示する更新余地の精度。",
        Text::PreviousSegmentBackground => "背景",
        Text::PreviousSegmentBackgroundDescription => "コンポーネントの背景。",
        Text::PreviousSegmentComparison => "比較対象",
        Text::PreviousSegmentComparisonDescription => {
            "どれだけタイムを得失したかを計算する比較対象。指定しない場合は現在の比較対象を使用します。"
        }
        Text::PreviousSegmentDisplayTwoRows => "2 行で表示",
        Text::PreviousSegmentDisplayTwoRowsDescription => {
            "コンポーネント名と得失タイムを 2 行で表示するかどうか。"
        }
        Text::PreviousSegmentLabelColor => "ラベル色",
        Text::PreviousSegmentLabelColorDescription => {
            "コンポーネント名の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::PreviousSegmentDropDecimals => "小数点以下を省略",
        Text::PreviousSegmentDropDecimalsDescription => {
            "表示するタイムが 1 分を超える場合に小数点以下を省略するかどうか。"
        }
        Text::PreviousSegmentAccuracy => "精度",
        Text::PreviousSegmentAccuracyDescription => "表示するタイムの精度。",
        Text::PreviousSegmentShowPossibleTimeSave => "更新余地を表示",
        Text::PreviousSegmentShowPossibleTimeSaveDescription => {
            "前の区間で得失したタイムに加えて、更新余地も表示するかどうか。"
        }
        Text::SegmentTimeBackground => "背景",
        Text::SegmentTimeBackgroundDescription => "コンポーネントの背景。",
        Text::SegmentTimeComparison => "比較対象",
        Text::SegmentTimeComparisonDescription => {
            "区間タイムの比較対象。指定しない場合は現在の比較対象を使用します。"
        }
        Text::SegmentTimeDisplayTwoRows => "2 行で表示",
        Text::SegmentTimeDisplayTwoRowsDescription => {
            "コンポーネント名と区間タイムを 2 行で表示するかどうか。"
        }
        Text::SegmentTimeLabelColor => "ラベル色",
        Text::SegmentTimeLabelColorDescription => {
            "コンポーネント名の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::SegmentTimeValueColor => "値の色",
        Text::SegmentTimeValueColorDescription => {
            "区間タイムの色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::SegmentTimeAccuracy => "精度",
        Text::SegmentTimeAccuracyDescription => "表示する区間タイムの精度。",
        Text::GraphComparison => "比較対象",
        Text::GraphComparisonDescription => {
            "グラフに使用する比較対象。指定しない場合は現在の比較対象を使用します。"
        }
        Text::GraphHeight => "高さ",
        Text::GraphHeightDescription => "グラフの高さ。",
        Text::GraphShowBestSegments => "区間ベストを表示",
        Text::GraphShowBestSegmentsDescription => {
            "区間ベストをレイアウトの区間ベストの色で表示するかどうか。"
        }
        Text::GraphLiveGraph => "ライブグラフ",
        Text::GraphLiveGraphDescription => {
            "グラフを常時更新するかどうか。無効の場合、現在の区間が変わるときのみ更新されます。"
        }
        Text::GraphFlipGraph => "グラフを反転",
        Text::GraphFlipGraphDescription => {
            "グラフを縦に反転するかどうか。無効の場合、リードはX軸の下、遅れは上に表示されます。"
        }
        Text::GraphBehindBackgroundColor => "遅れ側の背景色",
        Text::GraphBehindBackgroundColorDescription => {
            "比較対象より遅れているタイムの領域に使用する背景色。"
        }
        Text::GraphAheadBackgroundColor => "リード側の背景色",
        Text::GraphAheadBackgroundColorDescription => {
            "比較対象よりリードしているタイムの領域に使用する背景色。"
        }
        Text::GraphGridLinesColor => "グリッド線の色",
        Text::GraphGridLinesColorDescription => "グラフのグリッド線の色。",
        Text::GraphLinesColor => "グラフ線の色",
        Text::GraphLinesColorDescription => "グラフの点を結ぶ線の色。",
        Text::GraphPartialFillColor => "ライブ領域塗りつぶし色",
        Text::GraphPartialFillColorDescription => {
            "X軸とグラフに囲まれた領域の色。ライブ変化にのみ使用され、直近のスプリットから現在のタイムまで適用されます。"
        }
        Text::GraphCompleteFillColor => "確定領域塗りつぶし色",
        Text::GraphCompleteFillColorDescription => {
            "X軸とグラフに囲まれた領域の色（ライブ変化部分を除く）。"
        }
        Text::DetailedTimerBackground => "背景",
        Text::DetailedTimerBackgroundDescription => "コンポーネントの背景。",
        Text::DetailedTimerTimingMethod => "計測方法",
        Text::DetailedTimerTimingMethodDescription => {
            "使用する計測方法を指定します。指定しない場合は現在の計測方法を使用します。"
        }
        Text::DetailedTimerComparison1 => "比較対象 1",
        Text::DetailedTimerComparison1Description => {
            "区間タイムを表示する 1 つ目の比較対象。指定しない場合は現在の比較対象を使用します。"
        }
        Text::DetailedTimerComparison2 => "比較対象 2",
        Text::DetailedTimerComparison2Description => {
            "区間タイムを表示する 2 つ目の比較対象。指定しない場合は現在の比較対象を使用します（比較対象 1 も None の場合を除く）。2 つ目の比較対象が非表示の場合は表示されません。"
        }
        Text::DetailedTimerHideSecondComparison => "第 2 比較対象を非表示",
        Text::DetailedTimerHideSecondComparisonDescription => {
            "比較対象を 1 つだけ表示するかどうか。"
        }
        Text::DetailedTimerTimerHeight => "タイマーの高さ",
        Text::DetailedTimerTimerHeightDescription => "タイマースタートからのタイマーの高さ。",
        Text::DetailedTimerSegmentTimerHeight => "区間タイマーの高さ",
        Text::DetailedTimerSegmentTimerHeightDescription => "区間タイマーの高さ。",
        Text::DetailedTimerTimerColor => "タイマー色",
        Text::DetailedTimerTimerColorDescription => {
            "現在のペースに基づく自動色ではなく、常に使用する色を指定できます。"
        }
        Text::DetailedTimerShowTimerGradient => "タイマーのグラデーションを表示",
        Text::DetailedTimerShowTimerGradientDescription => {
            "有効にするとタイマーの色が自動的に縦方向のグラデーションになります。無効の場合は単色で表示されます。"
        }
        Text::DetailedTimerTimerDigitsFormat => "タイマーの桁数",
        Text::DetailedTimerTimerDigitsFormatDescription => {
            "タイマーに表示する桁数。桁数より短いタイムの場合は 0 で埋めて表示されます。"
        }
        Text::DetailedTimerTimerAccuracy => "タイマーの精度",
        Text::DetailedTimerTimerAccuracyDescription => "タイマーに表示するタイムの精度。",
        Text::DetailedTimerSegmentTimerColor => "区間タイマー色",
        Text::DetailedTimerSegmentTimerColorDescription => {
            "区間タイマーの色をデフォルトとは別の色に変更します。"
        }
        Text::DetailedTimerShowSegmentTimerGradient => "区間タイマーのグラデーションを表示",
        Text::DetailedTimerShowSegmentTimerGradientDescription => {
            "有効にすると区間タイマーの色が自動的に縦方向のグラデーションになります。無効の場合は単色で表示されます。"
        }
        Text::DetailedTimerSegmentTimerDigitsFormat => "区間タイマーの桁数",
        Text::DetailedTimerSegmentTimerDigitsFormatDescription => {
            "区間タイマーに表示する桁数。桁数より短いタイムの場合は 0 で埋めて表示されます。"
        }
        Text::DetailedTimerSegmentTimerAccuracy => "区間タイマーの精度",
        Text::DetailedTimerSegmentTimerAccuracyDescription => {
            "区間タイマーに表示するタイムの精度。"
        }
        Text::DetailedTimerComparisonNamesColor => "比較対象名の色",
        Text::DetailedTimerComparisonNamesColorDescription => {
            "比較対象名を表示する場合の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::DetailedTimerComparisonTimesColor => "比較対象タイムの色",
        Text::DetailedTimerComparisonTimesColorDescription => {
            "比較対象タイムを表示する場合の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::DetailedTimerComparisonTimesAccuracy => "比較対象タイムの精度",
        Text::DetailedTimerComparisonTimesAccuracyDescription => "比較対象タイムの精度。",
        Text::DetailedTimerShowSegmentName => "区間名を表示",
        Text::DetailedTimerShowSegmentNameDescription => "区間名を表示するかどうか。",
        Text::DetailedTimerSegmentNameColor => "区間名の色",
        Text::DetailedTimerSegmentNameColorDescription => {
            "区間名を表示する場合の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::DetailedTimerDisplayIcon => "アイコンを表示",
        Text::DetailedTimerDisplayIconDescription => "区間アイコンを表示するかどうか。",
        Text::SplitsBackground => "背景",
        Text::SplitsBackgroundDescription => {
            "コンポーネントの背景。交互色を選択すると、各行が2色で交互に表示されます。"
        }
        Text::SplitsTotalRows => "表示行数",
        Text::SplitsTotalRowsDescription => {
            "表示する区間の総数。0 の場合は全区間を表示します。全区間数より小さい値の場合はウィンドウ表示となり、上下にスクロールします。"
        }
        Text::SplitsUpcomingSegments => "今後の区間",
        Text::SplitsUpcomingSegmentsDescription => {
            "表示行数より区間が多い場合、現在の区間が変わるとウィンドウが自動的にスクロールします。この値は表示する将来区間の最小数を決めます。"
        }
        Text::SplitsShowThinSeparators => "細い区切り線を表示",
        Text::SplitsShowThinSeparatorsDescription => "区間の間に細い区切り線を表示するかどうか。",
        Text::SplitsShowSeparatorBeforeLastSplit => "最後のスプリット前の区切り線を表示",
        Text::SplitsShowSeparatorBeforeLastSplitDescription => {
            "最後の区間を常に表示する場合、スクロール窓で直前の区間と隣接しないときに強調区切り線を表示するかどうかを指定します。"
        }
        Text::SplitsAlwaysShowLastSplit => "最後のスプリットを常に表示",
        Text::SplitsAlwaysShowLastSplitDescription => {
            "スクロール窓で全区間を表示しない場合、最後の区間を常に表示するかどうか。最後の区間には比較対象の合計タイムが含まれるため有用です。"
        }
        Text::SplitsFillWithBlankSpace => "余白で埋める",
        Text::SplitsFillWithBlankSpaceDescription => {
            "表示行数に対して区間が少ない場合、余白で埋めて常に表示行数分の空間を表示するかどうか。"
        }
        Text::SplitsShowTimesBelowSegmentName => "区間名の下にタイムを表示",
        Text::SplitsShowTimesBelowSegmentNameDescription => {
            "区間名の下にタイムを表示するかどうか。無効の場合は横に表示されます。"
        }
        Text::SplitsCurrentSegmentGradient => "現在区間のグラデーション",
        Text::SplitsCurrentSegmentGradientDescription => {
            "現在の区間を示すために背面に表示するグラデーション。"
        }
        Text::SplitsSplitTimeAccuracy => "スプリットタイムの精度",
        Text::SplitsSplitTimeAccuracyDescription => "スプリットタイム列に使用する精度。",
        Text::SplitsSegmentTimeAccuracy => "区間タイムの精度",
        Text::SplitsSegmentTimeAccuracyDescription => "区間タイム列に使用する精度。",
        Text::SplitsDeltaTimeAccuracy => "タイム差の精度",
        Text::SplitsDeltaTimeAccuracyDescription => "リード/遅れ列に使用する精度。",
        Text::SplitsDropDeltaDecimals => "タイム差 1 分以上は小数点以下を省略",
        Text::SplitsDropDeltaDecimalsDescription => {
            "リード/遅れの列が 1 分を超える場合に小数点以下を表示しないかどうか。"
        }
        Text::SplitsShowColumnLabels => "列ラベルを表示",
        Text::SplitsShowColumnLabelsDescription => "一覧の上部に列名を表示するかどうか。",
        Text::SplitsColumns => "列",
        Text::SplitsColumnsDescription => {
            "各行に表示する列数。各列は異なる情報を表示できます。列は右から左に定義されます。"
        }
        Text::SplitsColumnName => "列名",
        Text::SplitsColumnNameDescription => "列名。列ラベルを表示する場合に上部に表示されます。",
        Text::SplitsColumnType => "列タイプ",
        Text::SplitsColumnTypeDescription => {
            "列に表示する情報の種類。タイムまたはカスタム変数を表示できます。"
        }
        Text::SplitsVariableName => "変数名",
        Text::SplitsVariableNameDescription => "この列に表示するカスタム変数名。",
        Text::SplitsStartWith => "開始値",
        Text::SplitsStartWithDescription => {
            "各区間で列が開始する値。更新トリガーが満たされるまでこの値が表示されます。"
        }
        Text::SplitsUpdateWith => "更新値",
        Text::SplitsUpdateWithDescription => {
            "条件が満たされると（通常は現在の区間か、通過済みであること）、ここで指定した値に更新されます。"
        }
        Text::SplitsUpdateTrigger => "更新トリガー",
        Text::SplitsUpdateTriggerDescription => {
            "更新値に置き換えるための条件。条件が満たされるまでは開始値が表示されます。"
        }
        Text::SplitsColumnComparison => "比較対象",
        Text::SplitsColumnComparisonDescription => {
            "この列が参照する比較対象。指定しない場合は現在の比較対象を使用します。"
        }
        Text::SplitsColumnTimingMethod => "計測方法",
        Text::SplitsColumnTimingMethodDescription => {
            "この列に使用する計測方法。指定しない場合は現在の計測方法を使用します。"
        }
        Text::TextComponentBackground => "背景",
        Text::TextComponentBackgroundDescription => "コンポーネントの背景。",
        Text::TextComponentUseVariable => "変数を使用",
        Text::TextComponentUseVariableDescription => {
            "動的な値を表示するためにカスタム変数を使用するかどうか。カスタム変数はスプリット編集で設定でき、オートスプリッターから提供されることもあります。"
        }
        Text::TextComponentSplit => "分割",
        Text::TextComponentSplitDescription => {
            "テキストを左右に分割するかどうか。分割しない場合は中央に 1 つのテキストが表示されます。"
        }
        Text::TextComponentText => "テキスト",
        Text::TextComponentTextDescription => "中央に表示するテキスト。",
        Text::TextComponentLeft => "左",
        Text::TextComponentLeftDescription => "左に表示するテキスト。",
        Text::TextComponentRight => "右",
        Text::TextComponentRightDescription => "右に表示するテキスト。",
        Text::TextComponentVariable => "変数",
        Text::TextComponentVariableDescription => "表示するカスタム変数名。",
        Text::TextComponentTextColor => "テキスト色",
        Text::TextComponentTextColorDescription => "テキストの色。",
        Text::TextComponentLeftColor => "左の色",
        Text::TextComponentLeftColorDescription => "左側のテキスト色。",
        Text::TextComponentRightColor => "右の色",
        Text::TextComponentRightColorDescription => "右側のテキスト色。",
        Text::TextComponentNameColor => "名前の色",
        Text::TextComponentNameColorDescription => "変数名の色。",
        Text::TextComponentValueColor => "値の色",
        Text::TextComponentValueColorDescription => "変数値の色。",
        Text::TextComponentDisplayTwoRows => "2 行で表示",
        Text::TextComponentDisplayTwoRowsDescription => "左右のテキストを 2 行で表示するかどうか。",
        Text::LayoutDirection => "レイアウト方向",
        Text::LayoutDirectionDescription => "コンポーネントを配置する方向。",
        Text::CustomTimerFont => "カスタムタイマーフォント",
        Text::CustomTimerFontDescription => {
            "タイマーのフォントをカスタム指定できます。指定しない場合はデフォルトフォントを使用します。"
        }
        Text::CustomTimesFont => "カスタムタイムフォント",
        Text::CustomTimesFontDescription => {
            "タイムのフォントをカスタム指定できます。指定しない場合はデフォルトフォントを使用します。"
        }
        Text::CustomTextFont => "カスタムテキストフォント",
        Text::CustomTextFontDescription => {
            "テキストのフォントをカスタム指定できます。指定しない場合はデフォルトフォントを使用します。"
        }
        Text::TextShadow => "テキスト影",
        Text::TextShadowDescription => "テキスト影の色を任意で指定できます。",
        Text::Background => "背景",
        Text::BackgroundDescription => "レイアウト全体の背景。",
        Text::BestSegment => "区間ベスト",
        Text::BestSegmentDescription => "新しい区間ベストを達成したときの色。",
        Text::AheadGainingTime => "リード（タイムを短縮している）",
        Text::AheadGainingTimeDescription => {
            "比較対象よりリードしていて、さらにタイムを短縮しているときの色。"
        }
        Text::AheadLosingTime => "リード（タイムをロスしている）",
        Text::AheadLosingTimeDescription => {
            "比較対象よりリードしているがタイムをロスしているときの色。"
        }
        Text::BehindGainingTime => "遅れ（タイムを取り戻している）",
        Text::BehindGainingTimeDescription => {
            "比較対象より遅れているがタイムを取り戻しているときの色。"
        }
        Text::BehindLosingTime => "遅れ（タイムをロスしている）",
        Text::BehindLosingTimeDescription => {
            "比較対象より遅れていて、さらにタイムをロスしているときの色。"
        }
        Text::NotRunning => "未走行",
        Text::NotRunningDescription => "アクティブな思考がないときの色。",
        Text::PersonalBest => "自己ベスト",
        Text::PersonalBestDescription => "新しい自己ベストを達成したときの色。",
        Text::Paused => "ポーズ",
        Text::PausedDescription => "タイマーがポーズしているときの色。",
        Text::ThinSeparators => "細い区切り線",
        Text::ThinSeparatorsDescription => "細い区切り線の色。",
        Text::Separators => "区切り線",
        Text::SeparatorsDescription => "通常の区切り線の色。",
        Text::TextColor => "テキスト",
        Text::TextColorDescription => "独自の色が指定されていないテキストに使用する色。",
        Text::ComponentBlankSpace => "余白",
        Text::ComponentCurrentComparison => "現在の比較対象",
        Text::ComponentCurrentPace => "現在のペース",
        Text::ComponentDelta => "タイム差",
        Text::ComponentDetailedTimer => "詳細タイマー",
        Text::ComponentGraph => "グラフ",
        Text::ComponentPbChance => "更新確率",
        Text::ComponentPossibleTimeSave => "更新余地",
        Text::ComponentPreviousSegment => "前の区間",
        Text::ComponentSegmentTime => "区間タイム",
        Text::ComponentSeparator => "区切り",
        Text::ComponentSplits => "スプリット",
        Text::ComponentSumOfBest => "合計区間ベスト",
        Text::ComponentText => "テキスト",
        Text::ComponentTimer => "タイマー",
        Text::ComponentSegmentTimer => "区間タイマー",
        Text::ComponentTitle => "タイトル",
        Text::ComponentTotalPlaytime => "総プレイタイム",
        Text::ComponentCurrentPaceBestPossibleTime => "最速予想タイム",
        Text::ComponentCurrentPaceWorstPossibleTime => "最悪予想タイム",
        Text::ComponentCurrentPacePredictedTime => "予測タイム",
        Text::ComponentSegmentTimeBest => "ベスト区間タイム",
        Text::ComponentSegmentTimeWorst => "ワースト区間タイム",
        Text::ComponentSegmentTimeAverage => "平均区間タイム",
        Text::ComponentSegmentTimeMedian => "中央値区間タイム",
        Text::ComponentSegmentTimeLatest => "最新区間タイム",
        Text::ComponentPossibleTimeSaveTotal => "ゴール時の更新余地",
        Text::LiveSegment => "ライブ区間",
        Text::LiveSegmentShort => "ライブ",
        Text::PreviousSegmentShort => "前の区間",
        Text::PreviousSegmentAbbreviation => "前区",
        Text::ComparingAgainst => "比較対象",
        Text::ComparisonShort => "比較",
        Text::CurrentPaceBestPossibleTimeShort => "最速予想",
        Text::CurrentPaceBestTimeShort => "最速",
        Text::CurrentPaceBestPossibleTimeAbbreviation => "最速",
        Text::CurrentPaceWorstPossibleTimeShort => "最遅予想",
        Text::CurrentPaceWorstTimeShort => "最遅",
        Text::CurrentPacePredictedTimeShort => "予測",
        Text::CurrentPaceShort => "現ペース",
        Text::CurrentPaceAbbreviation => "ペース",
        Text::Goal => "目標",
        Text::SumOfBestSegments => "合計区間ベスト",
        Text::SumOfBestShort => "総最速区間",
        Text::SumOfBestAbbreviation => "総最速",
        Text::PlaytimeShort => "プレイ時間",
        Text::BestSegmentTimeShort => "最速区間",
        Text::BestSegmentShort => "最速区間",
        Text::WorstSegmentTimeShort => "最遅区間",
        Text::WorstSegmentShort => "最遅区間",
        Text::AverageSegmentTimeShort => "平均区間",
        Text::AverageSegmentShort => "平均区間",
        Text::MedianSegmentTimeShort => "中央区間",
        Text::MedianSegmentShort => "中央区間",
        Text::LatestSegmentTimeShort => "最新区間",
        Text::LatestSegmentShort => "最新区間",
        Text::SegmentTimeShort => "区間",
        Text::SplitTime => "タイム",
        Text::PossibleTimeSaveShort => "余地",
        Text::PossibleTimeSaveAbbreviation => "余地",
        Text::TimeSaveShort => "短縮",
        Text::RealTime => "実時間",
        Text::GameTime => "ゲーム時間",
        Text::Untitled => "無題",
        Text::SumOfBestCleanerStartOfRun => "ラン開始",
        Text::SumOfBestCleanerShouldRemove => {
            "この区間タイムは不正確だと思いますか？もしそうなら、削除したほうがいいでしょうか？"
        }
    }
}

pub const fn resolve_placeholder(text: PlaceholderText) -> &'static [Piece] {
    match text {
        PlaceholderText::SumOfBestCleanerSegmentTimeBetween => &[
            Piece::Dynamic(0),
            Piece::Static("基準で、「"),
            Piece::Dynamic(2),
            Piece::Static("」から「"),
            Piece::Dynamic(3),
            Piece::Static("」までの区間タイムが"),
            Piece::Dynamic(1),
            Piece::Static("として記録されています。"),
        ],
        PlaceholderText::SumOfBestCleanerFasterThanCombined => &[
            Piece::Static("これは、区間ベストの合計"),
            Piece::Dynamic(0),
            Piece::Static("よりも速い記録です。"),
        ],
        PlaceholderText::SumOfBestCleanerRunOn => &[
            Piece::Static("この記録は"),
            Piece::Dynamic(0),
            Piece::Dynamic(1),
            Piece::Static("に開始したランで出たものです。"),
        ],
    }
}
