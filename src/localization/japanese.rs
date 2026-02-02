use super::{Piece, PlaceholderText, Text};

pub const fn resolve(text: Text) -> &'static str {
    match text {
        Text::StartSplit => "開始 / スプリット",
        Text::StartSplitDescription => "スプリットと新しいアテンプト開始に使用するホットキー。",
        Text::Reset => "リセット",
        Text::ResetDescription => "現在のアテンプトをリセットするホットキー。",
        Text::UndoSplit => "スプリットを取り消す",
        Text::UndoSplitDescription => "最後のスプリットを取り消すホットキー。",
        Text::SkipSplit => "スプリットをスキップ",
        Text::SkipSplitDescription => "現在のスプリットをスキップするホットキー。",
        Text::Pause => "一時停止",
        Text::PauseDescription => {
            "現在のアテンプトを一時停止するホットキー。新しいアテンプトの開始にも使用できます。"
        }
        Text::UndoAllPauses => "すべてのポーズを取り消す",
        Text::UndoAllPausesDescription => {
            "現在の時間からすべてのポーズ時間を削除するホットキー。誤って一時停止した場合に便利です。"
        }
        Text::PreviousComparison => "前の比較",
        Text::PreviousComparisonDescription => "前の比較に切り替えるホットキー。",
        Text::NextComparison => "次の比較",
        Text::NextComparisonDescription => "次の比較に切り替えるホットキー。",
        Text::ToggleTimingMethod => "計測方法を切り替え",
        Text::ToggleTimingMethodDescription => {
            "「リアルタイム」と「ゲームタイム」を切り替えるホットキー。"
        }
        Text::TimerBackground => "背景",
        Text::TimerBackgroundDescription => {
            "コンポーネントの背景。先行/遅れの色を背景色として適用することもできます。"
        }
        Text::SegmentTimer => "セグメントタイマー",
        Text::SegmentTimerDescription => {
            "現在のセグメント開始からの経過時間を表示するか、試行開始からの経過時間を表示するかを指定します。"
        }
        Text::TimingMethod => "計測方法",
        Text::TimingMethodDescription => {
            "使用する計測方法を指定します。指定しない場合は現在の計測方法を使用します。"
        }
        Text::Height => "高さ",
        Text::HeightDescription => "タイマーの高さ。",
        Text::TimerTextColor => "テキスト色",
        Text::TimerTextColorDescription => {
            "表示する時間の色。指定しない場合は、現在のアテンプトの進行状況に基づいて自動的に選択されます。これらの色はレイアウトの一般設定で指定できます。"
        }
        Text::ShowGradient => "グラデーションを表示",
        Text::ShowGradientDescription => "タイマーの色をグラデーションで表示するかどうか。",
        Text::DigitsFormat => "桁数",
        Text::DigitsFormatDescription => {
            "表示する桁数を指定します。表示する桁数より短い場合は 0 が表示されます。"
        }
        Text::Accuracy => "精度",
        Text::AccuracyDescription => "表示する時間の精度。",
        Text::TitleBackground => "背景",
        Text::TitleBackgroundDescription => "コンポーネントの背景。",
        Text::TitleTextColor => "テキスト色",
        Text::TitleTextColorDescription => {
            "タイトル文字の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::ShowGameName => "ゲーム名を表示",
        Text::ShowGameNameDescription => "表示するタイトルにゲーム名を含めるかどうか。",
        Text::ShowCategoryName => "カテゴリ名を表示",
        Text::ShowCategoryNameDescription => "表示するタイトルにカテゴリ名を含めるかどうか。",
        Text::ShowFinishedRunsCount => "完走数を表示",
        Text::ShowFinishedRunsCountDescription => "成功裏に完走した回数を表示するかどうか。",
        Text::ShowAttemptCount => "試行回数を表示",
        Text::ShowAttemptCountDescription => "合計試行回数を表示するかどうか。",
        Text::TextAlignment => "テキスト配置",
        Text::TextAlignmentDescription => "タイトルの配置を指定します。",
        Text::DisplayTextAsSingleLine => "テキストを1行で表示",
        Text::DisplayTextAsSingleLineDescription => {
            "タイトルをゲーム名とカテゴリ名の2行に分けず、1行で表示するかどうか。"
        }
        Text::DisplayGameIcon => "ゲームアイコンを表示",
        Text::DisplayGameIconDescription => {
            "スプリットにゲームアイコンが保存されている場合、それを表示するかどうか。"
        }
        Text::ShowRegion => "地域を表示",
        Text::ShowRegionDescription => {
            "カテゴリ名に追加情報を付加します。スプリットエディタの変数タブで地域が指定されている場合に地域を付加します。"
        }
        Text::ShowPlatform => "プラットフォームを表示",
        Text::ShowPlatformDescription => {
            "カテゴリ名に追加情報を付加します。スプリットエディタの変数タブでプラットフォームが指定されている場合に付加します。"
        }
        Text::ShowVariables => "変数を表示",
        Text::ShowVariablesDescription => {
            "カテゴリ名に追加情報を付加します。スプリットエディタの変数タブで指定された追加変数を付加します。speedrun.com 変数を指し、カスタム変数ではありません。"
        }
        Text::TotalPlaytimeBackground => "背景",
        Text::TotalPlaytimeBackgroundDescription => "コンポーネントの背景。",
        Text::DisplayTwoRows => "2行で表示",
        Text::DisplayTwoRowsDescription => {
            "コンポーネント名と合計プレイ時間を2行で表示するかどうか。"
        }
        Text::ShowDays => "日数を表示（>24時間）",
        Text::ShowDaysDescription => {
            "合計プレイ時間が24時間以上になったときに日数を表示するかどうか。"
        }
        Text::LabelColor => "ラベル色",
        Text::LabelColorDescription => {
            "コンポーネント名の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::ValueColor => "値の色",
        Text::ValueColorDescription => {
            "合計プレイ時間の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::BlankSpaceBackground => "背景",
        Text::BlankSpaceBackgroundDescription => "コンポーネントの背景。",
        Text::BlankSpaceSize => "サイズ",
        Text::BlankSpaceSizeDescription => "コンポーネントのサイズ。",
        Text::CurrentComparisonBackground => "背景",
        Text::CurrentComparisonBackgroundDescription => "コンポーネントの背景。",
        Text::CurrentComparisonDisplayTwoRows => "2行で表示",
        Text::CurrentComparisonDisplayTwoRowsDescription => {
            "コンポーネント名と比較を2行で表示するかどうか。"
        }
        Text::CurrentComparisonLabelColor => "ラベル色",
        Text::CurrentComparisonLabelColorDescription => {
            "コンポーネント名の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::CurrentComparisonValueColor => "値の色",
        Text::CurrentComparisonValueColorDescription => {
            "比較名の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::CurrentPaceBackground => "背景",
        Text::CurrentPaceBackgroundDescription => "コンポーネントの背景。",
        Text::CurrentPaceComparison => "比較",
        Text::CurrentPaceComparisonDescription => {
            "最終タイムを予測するために使用する比較。指定しない場合は現在の比較を使用します。"
        }
        Text::CurrentPaceDisplayTwoRows => "2行で表示",
        Text::CurrentPaceDisplayTwoRowsDescription => {
            "コンポーネント名と予測タイムを2行で表示するかどうか。"
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
        Text::DeltaComparison => "比較",
        Text::DeltaComparisonDescription => {
            "先行/遅れを計算するための比較。指定しない場合は現在の比較を使用します。"
        }
        Text::DeltaDisplayTwoRows => "2行で表示",
        Text::DeltaDisplayTwoRowsDescription => "比較名と差分を2行で表示するかどうか。",
        Text::DeltaLabelColor => "ラベル色",
        Text::DeltaLabelColorDescription => {
            "比較名の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::DeltaDropDecimals => "小数点を省略",
        Text::DeltaDropDecimalsDescription => {
            "表示する差分が1分を超える場合に小数点を表示しないかどうか。"
        }
        Text::DeltaAccuracy => "精度",
        Text::DeltaAccuracyDescription => "表示する差分の精度。",
        Text::SumOfBestBackground => "背景",
        Text::SumOfBestBackgroundDescription => "コンポーネントの背景。",
        Text::SumOfBestDisplayTwoRows => "2行で表示",
        Text::SumOfBestDisplayTwoRowsDescription => {
            "コンポーネント名とベストセグメント合計を2行で表示するかどうか。"
        }
        Text::SumOfBestLabelColor => "ラベル色",
        Text::SumOfBestLabelColorDescription => {
            "コンポーネント名の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::SumOfBestValueColor => "値の色",
        Text::SumOfBestValueColorDescription => {
            "ベストセグメント合計の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::SumOfBestAccuracy => "精度",
        Text::SumOfBestAccuracyDescription => "表示するベスト合計の精度。",
        Text::PbChanceBackground => "背景",
        Text::PbChanceBackgroundDescription => "コンポーネントの背景。",
        Text::PbChanceDisplayTwoRows => "2行で表示",
        Text::PbChanceDisplayTwoRowsDescription => {
            "コンポーネント名とPB確率を2行で表示するかどうか。"
        }
        Text::PbChanceLabelColor => "ラベル色",
        Text::PbChanceLabelColorDescription => {
            "コンポーネント名の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::PbChanceValueColor => "値の色",
        Text::PbChanceValueColorDescription => {
            "PB確率の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::PossibleTimeSaveBackground => "背景",
        Text::PossibleTimeSaveBackgroundDescription => "コンポーネントの背景。",
        Text::PossibleTimeSaveComparison => "比較",
        Text::PossibleTimeSaveComparisonDescription => {
            "可能タイムセーブを計算する比較。指定しない場合は現在の比較を使用します。"
        }
        Text::PossibleTimeSaveDisplayTwoRows => "2行で表示",
        Text::PossibleTimeSaveDisplayTwoRowsDescription => {
            "コンポーネント名と可能タイムセーブを2行で表示するかどうか。"
        }
        Text::PossibleTimeSaveShowTotal => "合計可能タイムセーブを表示",
        Text::PossibleTimeSaveShowTotalDescription => {
            "現在のセグメントではなく、残り全体の合計可能タイムセーブを表示するかどうか。"
        }
        Text::PossibleTimeSaveLabelColor => "ラベル色",
        Text::PossibleTimeSaveLabelColorDescription => {
            "コンポーネント名の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::PossibleTimeSaveValueColor => "値の色",
        Text::PossibleTimeSaveValueColorDescription => {
            "可能タイムセーブの色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::PossibleTimeSaveAccuracy => "精度",
        Text::PossibleTimeSaveAccuracyDescription => "表示する可能タイムセーブの精度。",
        Text::PreviousSegmentBackground => "背景",
        Text::PreviousSegmentBackgroundDescription => "コンポーネントの背景。",
        Text::PreviousSegmentComparison => "比較",
        Text::PreviousSegmentComparisonDescription => {
            "どれだけ時間を得失したかを計算する比較。指定しない場合は現在の比較を使用します。"
        }
        Text::PreviousSegmentDisplayTwoRows => "2行で表示",
        Text::PreviousSegmentDisplayTwoRowsDescription => {
            "コンポーネント名と得失タイムを2行で表示するかどうか。"
        }
        Text::PreviousSegmentLabelColor => "ラベル色",
        Text::PreviousSegmentLabelColorDescription => {
            "コンポーネント名の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::PreviousSegmentDropDecimals => "小数点を省略",
        Text::PreviousSegmentDropDecimalsDescription => {
            "表示する時間が1分を超える場合に小数点を省略するかどうか。"
        }
        Text::PreviousSegmentAccuracy => "精度",
        Text::PreviousSegmentAccuracyDescription => "表示する時間の精度。",
        Text::PreviousSegmentShowPossibleTimeSave => "可能タイムセーブを表示",
        Text::PreviousSegmentShowPossibleTimeSaveDescription => {
            "前のセグメントで得失した時間に加えて、可能タイムセーブも表示するかどうか。"
        }
        Text::SegmentTimeBackground => "背景",
        Text::SegmentTimeBackgroundDescription => "コンポーネントの背景。",
        Text::SegmentTimeComparison => "比較",
        Text::SegmentTimeComparisonDescription => {
            "セグメントタイムの比較。指定しない場合は現在の比較を使用します。"
        }
        Text::SegmentTimeDisplayTwoRows => "2行で表示",
        Text::SegmentTimeDisplayTwoRowsDescription => {
            "コンポーネント名とセグメントタイムを2行で表示するかどうか。"
        }
        Text::SegmentTimeLabelColor => "ラベル色",
        Text::SegmentTimeLabelColorDescription => {
            "コンポーネント名の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::SegmentTimeValueColor => "値の色",
        Text::SegmentTimeValueColorDescription => {
            "セグメントタイムの色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::SegmentTimeAccuracy => "精度",
        Text::SegmentTimeAccuracyDescription => "表示するセグメントタイムの精度。",
        Text::GraphComparison => "比較",
        Text::GraphComparisonDescription => {
            "グラフに使用する比較。指定しない場合は現在の比較を使用します。"
        }
        Text::GraphHeight => "高さ",
        Text::GraphHeightDescription => "グラフの高さ。",
        Text::GraphShowBestSegments => "ベストセグメントを表示",
        Text::GraphShowBestSegmentsDescription => {
            "ベストセグメントをレイアウトのベストセグメント色で表示するかどうか。"
        }
        Text::GraphLiveGraph => "ライブグラフ",
        Text::GraphLiveGraphDescription => {
            "グラフを常時更新するかどうか。無効の場合、現在のセグメントが変わるときのみ更新されます。"
        }
        Text::GraphFlipGraph => "グラフを反転",
        Text::GraphFlipGraphDescription => {
            "グラフを縦に反転するかどうか。無効の場合、先行はX軸の下、遅れは上に表示されます。"
        }
        Text::GraphBehindBackgroundColor => "遅れ側の背景色",
        Text::GraphBehindBackgroundColorDescription => {
            "比較より遅れている時間の領域に使用する背景色。"
        }
        Text::GraphAheadBackgroundColor => "先行側の背景色",
        Text::GraphAheadBackgroundColorDescription => {
            "比較より先行している時間の領域に使用する背景色。"
        }
        Text::GraphGridLinesColor => "グリッド線の色",
        Text::GraphGridLinesColorDescription => "グラフのグリッド線の色。",
        Text::GraphLinesColor => "グラフ線の色",
        Text::GraphLinesColorDescription => "グラフの点を結ぶ線の色。",
        Text::GraphPartialFillColor => "部分塗りつぶし色",
        Text::GraphPartialFillColorDescription => {
            "X軸とグラフに囲まれた領域の色。部分塗りつぶし色はライブ変化にのみ使用され、最後のスプリットから現在の時間までに適用されます。"
        }
        Text::GraphCompleteFillColor => "完全塗りつぶし色",
        Text::GraphCompleteFillColorDescription => {
            "X軸とグラフに囲まれた領域の色（ライブ変化部分を除く）。"
        }
        Text::DetailedTimerBackground => "背景",
        Text::DetailedTimerBackgroundDescription => "コンポーネントの背景。",
        Text::DetailedTimerTimingMethod => "計測方法",
        Text::DetailedTimerTimingMethodDescription => {
            "使用する計測方法を指定します。指定しない場合は現在の計測方法を使用します。"
        }
        Text::DetailedTimerComparison1 => "比較 1",
        Text::DetailedTimerComparison1Description => {
            "セグメントタイムを表示する1つ目の比較。指定しない場合は現在の比較を使用します。"
        }
        Text::DetailedTimerComparison2 => "比較 2",
        Text::DetailedTimerComparison2Description => {
            "セグメントタイムを表示する2つ目の比較。指定しない場合は現在の比較を使用します（比較1もNoneの場合を除く）。2つ目の比較が非表示の場合は表示されません。"
        }
        Text::DetailedTimerHideSecondComparison => "第2比較を非表示",
        Text::DetailedTimerHideSecondComparisonDescription => "比較を1つだけ表示するかどうか。",
        Text::DetailedTimerTimerHeight => "タイマーの高さ",
        Text::DetailedTimerTimerHeightDescription => "ランタイマーの高さ。",
        Text::DetailedTimerSegmentTimerHeight => "セグメントタイマーの高さ",
        Text::DetailedTimerSegmentTimerHeightDescription => "セグメントタイマーの高さ。",
        Text::DetailedTimerTimerColor => "タイマー色",
        Text::DetailedTimerTimerColorDescription => {
            "現在の進行状況に基づく自動色ではなく、常に使用する色を指定できます。"
        }
        Text::DetailedTimerShowTimerGradient => "タイマーのグラデーションを表示",
        Text::DetailedTimerShowTimerGradientDescription => {
            "有効にするとタイマーの色が自動的に縦方向のグラデーションになります。無効の場合は単色で表示されます。"
        }
        Text::DetailedTimerTimerDigitsFormat => "タイマーの桁数",
        Text::DetailedTimerTimerDigitsFormatDescription => {
            "タイマーに表示する桁数。短い場合は0が表示されます。"
        }
        Text::DetailedTimerTimerAccuracy => "タイマーの精度",
        Text::DetailedTimerTimerAccuracyDescription => "タイマーに表示する時間の精度。",
        Text::DetailedTimerSegmentTimerColor => "セグメントタイマー色",
        Text::DetailedTimerSegmentTimerColorDescription => {
            "セグメントタイマーの色をデフォルトとは別の色に変更します。"
        }
        Text::DetailedTimerShowSegmentTimerGradient => "セグメントタイマーのグラデーションを表示",
        Text::DetailedTimerShowSegmentTimerGradientDescription => {
            "有効にするとセグメントタイマーの色が自動的に縦方向のグラデーションになります。無効の場合は単色で表示されます。"
        }
        Text::DetailedTimerSegmentTimerDigitsFormat => "セグメントタイマーの桁数",
        Text::DetailedTimerSegmentTimerDigitsFormatDescription => {
            "セグメントタイマーに表示する桁数。短い場合は0が表示されます。"
        }
        Text::DetailedTimerSegmentTimerAccuracy => "セグメントタイマーの精度",
        Text::DetailedTimerSegmentTimerAccuracyDescription => {
            "セグメントタイマーに表示する時間の精度。"
        }
        Text::DetailedTimerComparisonNamesColor => "比較名の色",
        Text::DetailedTimerComparisonNamesColorDescription => {
            "比較名を表示する場合の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::DetailedTimerComparisonTimesColor => "比較タイムの色",
        Text::DetailedTimerComparisonTimesColorDescription => {
            "比較タイムを表示する場合の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::DetailedTimerComparisonTimesAccuracy => "比較タイムの精度",
        Text::DetailedTimerComparisonTimesAccuracyDescription => "比較タイムの精度。",
        Text::DetailedTimerShowSegmentName => "セグメント名を表示",
        Text::DetailedTimerShowSegmentNameDescription => "セグメント名を表示するかどうか。",
        Text::DetailedTimerSegmentNameColor => "セグメント名の色",
        Text::DetailedTimerSegmentNameColorDescription => {
            "セグメント名を表示する場合の色。指定しない場合はレイアウトの色が使用されます。"
        }
        Text::DetailedTimerDisplayIcon => "アイコンを表示",
        Text::DetailedTimerDisplayIconDescription => "セグメントアイコンを表示するかどうか。",
        Text::SplitsBackground => "背景",
        Text::SplitsBackgroundDescription => {
            "コンポーネントの背景。交互色を選択すると、各行が2色で交互に表示されます。"
        }
        Text::SplitsTotalRows => "総行数",
        Text::SplitsTotalRowsDescription => {
            "表示するセグメント行の総数。0の場合は全セグメントを表示します。総数より小さい値の場合はウィンドウ表示となり、上下にスクロールします。"
        }
        Text::SplitsUpcomingSegments => "今後のセグメント",
        Text::SplitsUpcomingSegmentsDescription => {
            "表示行数よりセグメントが多い場合、現在のセグメントが変わるとウィンドウが自動的にスクロールします。この値は表示する将来セグメントの最小数を決めます。"
        }
        Text::SplitsShowThinSeparators => "細い区切り線を表示",
        Text::SplitsShowThinSeparatorsDescription => {
            "セグメント行の間に細い区切り線を表示するかどうか。"
        }
        Text::SplitsShowSeparatorBeforeLastSplit => "最後のスプリット前の区切り線を表示",
        Text::SplitsShowSeparatorBeforeLastSplitDescription => {
            "最後のセグメントを常に表示する場合、スクロール窓で直前のセグメントと隣接しないときに強調区切り線を表示するかどうかを指定します。"
        }
        Text::SplitsAlwaysShowLastSplit => "最後のスプリットを常に表示",
        Text::SplitsAlwaysShowLastSplitDescription => {
            "スクロール窓で全セグメントを表示しない場合、最後のセグメントを常に表示するかどうか。最後のセグメントには比較の合計時間が含まれるため有用です。"
        }
        Text::SplitsFillWithBlankSpace => "空白で埋める",
        Text::SplitsFillWithBlankSpaceDescription => {
            "表示行数に対してセグメントが少ない場合、空白で埋めて常に総行数を表示するかどうか。"
        }
        Text::SplitsShowTimesBelowSegmentName => "セグメント名の下にタイムを表示",
        Text::SplitsShowTimesBelowSegmentNameDescription => {
            "セグメント名の下にタイムを表示するかどうか。無効の場合は横に表示されます。"
        }
        Text::SplitsCurrentSegmentGradient => "現在セグメントのグラデーション",
        Text::SplitsCurrentSegmentGradientDescription => {
            "現在のセグメントを示すために背面に表示するグラデーション。"
        }
        Text::SplitsSplitTimeAccuracy => "スプリットタイムの精度",
        Text::SplitsSplitTimeAccuracyDescription => "スプリットタイム列に使用する精度。",
        Text::SplitsSegmentTimeAccuracy => "セグメントタイムの精度",
        Text::SplitsSegmentTimeAccuracyDescription => "セグメントタイム列に使用する精度。",
        Text::SplitsDeltaTimeAccuracy => "差分タイムの精度",
        Text::SplitsDeltaTimeAccuracyDescription => "先行/遅れ列に使用する精度。",
        Text::SplitsDropDeltaDecimals => "分表示時に小数点を省略",
        Text::SplitsDropDeltaDecimalsDescription => {
            "先行/遅れの列が1分を超える場合に小数点を表示しないかどうか。"
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
            "列に表示する情報の種類。時間またはカスタム変数を表示できます。"
        }
        Text::SplitsVariableName => "変数名",
        Text::SplitsVariableNameDescription => "この列に表示するカスタム変数名。",
        Text::SplitsStartWith => "開始値",
        Text::SplitsStartWithDescription => {
            "各セグメントで列が開始する値。更新トリガーが満たされるまでこの値が表示されます。"
        }
        Text::SplitsUpdateWith => "更新値",
        Text::SplitsUpdateWithDescription => {
            "条件が満たされると（通常は現在のセグメントか、完了済みであること）、ここで指定した値に更新されます。"
        }
        Text::SplitsUpdateTrigger => "更新トリガー",
        Text::SplitsUpdateTriggerDescription => {
            "更新値に置き換えるための条件。条件が満たされるまでは開始値が表示されます。"
        }
        Text::SplitsColumnComparison => "比較",
        Text::SplitsColumnComparisonDescription => {
            "この列が参照する比較。指定しない場合は現在の比較を使用します。"
        }
        Text::SplitsColumnTimingMethod => "計測方法",
        Text::SplitsColumnTimingMethodDescription => {
            "この列に使用する計測方法。指定しない場合は現在の計測方法を使用します。"
        }
        Text::TextComponentBackground => "背景",
        Text::TextComponentBackgroundDescription => "コンポーネントの背景。",
        Text::TextComponentUseVariable => "変数を使用",
        Text::TextComponentUseVariableDescription => {
            "動的な値を表示するためにカスタム変数を使用するかどうか。カスタム変数はスプリットエディタで設定でき、オートスプリッターから提供されることもあります。"
        }
        Text::TextComponentSplit => "分割",
        Text::TextComponentSplitDescription => {
            "テキストを左右に分割するかどうか。分割しない場合は中央に1つのテキストが表示されます。"
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
        Text::TextComponentDisplayTwoRows => "2行で表示",
        Text::TextComponentDisplayTwoRowsDescription => "左右のテキストを2行で表示するかどうか。",
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
        Text::BestSegment => "ベストセグメント",
        Text::BestSegmentDescription => "新しいベストセグメントを達成したときの色。",
        Text::AheadGainingTime => "先行（時間を伸ばしている）",
        Text::AheadGainingTimeDescription => {
            "比較より先行していて、さらに時間を伸ばしているときの色。"
        }
        Text::AheadLosingTime => "先行（時間を失っている）",
        Text::AheadLosingTimeDescription => "比較より先行しているが時間を失っているときの色。",
        Text::BehindGainingTime => "遅れ（時間を取り戻している）",
        Text::BehindGainingTimeDescription => "比較より遅れているが時間を取り戻しているときの色。",
        Text::BehindLosingTime => "遅れ（時間を失っている）",
        Text::BehindLosingTimeDescription => "比較より遅れていて、さらに時間を失っているときの色。",
        Text::NotRunning => "未走行",
        Text::NotRunningDescription => "アクティブなアテンプトがないときの色。",
        Text::PersonalBest => "自己ベスト",
        Text::PersonalBestDescription => "新しい自己ベストを達成したときの色。",
        Text::Paused => "一時停止",
        Text::PausedDescription => "タイマーが一時停止しているときの色。",
        Text::ThinSeparators => "細い区切り線",
        Text::ThinSeparatorsDescription => "細い区切り線の色。",
        Text::Separators => "区切り線",
        Text::SeparatorsDescription => "通常の区切り線の色。",
        Text::TextColor => "テキスト",
        Text::TextColorDescription => "独自の色が指定されていないテキストに使用する色。",
        Text::ComponentBlankSpace => "空白",
        Text::ComponentCurrentComparison => "現在の比較",
        Text::ComponentCurrentPace => "現在のペース",
        Text::ComponentDelta => "差",
        Text::ComponentDetailedTimer => "詳細タイマー",
        Text::ComponentGraph => "グラフ",
        Text::ComponentPbChance => "PB 可能性",
        Text::ComponentPossibleTimeSave => "節約可能時間",
        Text::ComponentPreviousSegment => "前セグメント",
        Text::ComponentSegmentTime => "セグメント時間",
        Text::ComponentSeparator => "区切り",
        Text::ComponentSplits => "スプリット",
        Text::ComponentSumOfBest => "ベスト合計",
        Text::ComponentText => "テキスト",
        Text::ComponentTimer => "タイマー",
        Text::ComponentSegmentTimer => "セグメントタイマー",
        Text::ComponentTitle => "タイトル",
        Text::ComponentTotalPlaytime => "総プレイ時間",
        Text::ComponentCurrentPaceBestPossibleTime => "最速可能タイム",
        Text::ComponentCurrentPaceWorstPossibleTime => "最悪可能タイム",
        Text::ComponentCurrentPacePredictedTime => "予測タイム",
        Text::ComponentSegmentTimeBest => "ベストセグメントタイム",
        Text::ComponentSegmentTimeWorst => "ワーストセグメントタイム",
        Text::ComponentSegmentTimeAverage => "平均セグメントタイム",
        Text::ComponentSegmentTimeMedian => "中央値セグメントタイム",
        Text::ComponentSegmentTimeLatest => "最新セグメントタイム",
        Text::ComponentPossibleTimeSaveTotal => "合計短縮可能時間",
        Text::LiveSegment => "ライブセグメント",
        Text::LiveSegmentShort => "ライブセグメント",
        Text::PreviousSegmentShort => "前のセグメント",
        Text::PreviousSegmentAbbreviation => "前セグ",
        Text::ComparingAgainst => "比較対象",
        Text::ComparisonShort => "比較",
        Text::CurrentPaceBestPossibleTimeShort => "最速可タイム",
        Text::CurrentPaceBestTimeShort => "最速タイム",
        Text::CurrentPaceBestPossibleTimeAbbreviation => "最速可",
        Text::CurrentPaceWorstPossibleTimeShort => "最悪可タイム",
        Text::CurrentPaceWorstTimeShort => "最悪タイム",
        Text::CurrentPacePredictedTimeShort => "予測タイム",
        Text::CurrentPaceShort => "現ペース",
        Text::CurrentPaceAbbreviation => "ペース",
        Text::Goal => "目標",
        Text::SumOfBestSegments => "ベストセグメント合計",
        Text::SumOfBestShort => "ベスト合計",
        Text::SumOfBestAbbreviation => "ベ合",
        Text::PlaytimeShort => "プレイ時間",
        Text::BestSegmentTimeShort => "最速セグ時間",
        Text::BestSegmentShort => "最速セグ",
        Text::WorstSegmentTimeShort => "最遅セグ時間",
        Text::WorstSegmentShort => "最遅セグ",
        Text::AverageSegmentTimeShort => "平均セグ時間",
        Text::AverageSegmentShort => "平均セグ",
        Text::MedianSegmentTimeShort => "中央値セグ時間",
        Text::MedianSegmentShort => "中央値セグ",
        Text::LatestSegmentTimeShort => "最新セグ時間",
        Text::LatestSegmentShort => "最新セグ",
        Text::SegmentTimeShort => "セグ時間",
        Text::PossibleTimeSaveShort => "短縮可能時間",
        Text::PossibleTimeSaveAbbreviation => "短縮可能",
        Text::TimeSaveShort => "短縮",
        Text::RealTime => "リアルタイム",
        Text::GameTime => "ゲームタイム",
        Text::SumOfBestCleanerStartOfRun => "ラン開始",
        Text::SumOfBestCleanerShouldRemove => {
            " この区間タイムは不正確だと思いますか？ もしそうなら、削除したほうがいいでしょうか？"
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
            Piece::Static("これは、ベスト区間の合計"),
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
