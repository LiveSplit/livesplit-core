use crate::{
    hotkey::display,
    localization::{Piece, PlaceholderText, Text},
};

pub const fn resolve(text: Text) -> &'static str {
    match text {
        Text::Hotkey(text) => match text {
            display::Text::Control => "Ctrl",
            display::Text::Alt => "Alt",
            display::Text::Shift => "Shift",
            display::Text::Meta => "Meta",
            display::Text::Super => "Super",
            display::Text::InternationalBackslash => "國際反斜線",
            display::Text::Backspace => "退格",
            display::Text::CapsLock => "大寫鎖定",
            display::Text::ContextMenu => "快顯功能表",
            display::Text::Enter => "輸入",
            display::Text::Space => "空白",
            display::Text::Tab => "定位",
            display::Text::Convert => "轉換",
            display::Text::KanaMode => "假名模式",
            display::Text::Lang1 => "韓文 / 假名",
            display::Text::Lang2 => "漢字 / 英數",
            display::Text::Lang3 => "片假名",
            display::Text::Lang4 => "平假名",
            display::Text::Lang5 => "半形 / 全形",
            display::Text::NonConvert => "不轉換",
            display::Text::Delete => "刪除",
            display::Text::End => "結尾",
            display::Text::Help => "說明",
            display::Text::Home => "開頭",
            display::Text::Insert => "插入",
            display::Text::PageDown => "向下翻頁",
            display::Text::PageUp => "向上翻頁",
            display::Text::NumLock => "數字鎖定",
            display::Text::Clear => "清除",
            display::Text::ClearEntry => "清除輸入",
            display::Text::Decimal => "小數點",
            display::Text::Escape => "Esc",
            display::Text::FnLock => "Fn 鎖定",
            display::Text::PrintScreen => "列印螢幕",
            display::Text::ScrollLock => "捲動鎖定",
            display::Text::Pause => "暫停 / 中斷",
            display::Text::BrowserBack => "瀏覽器上一頁",
            display::Text::BrowserFavorites => "瀏覽器我的最愛",
            display::Text::BrowserForward => "瀏覽器下一頁",
            display::Text::BrowserHome => "瀏覽器首頁",
            display::Text::BrowserRefresh => "重新整理瀏覽器",
            display::Text::BrowserSearch => "瀏覽器搜尋",
            display::Text::BrowserStop => "停止瀏覽器",
            display::Text::Eject => "退出媒體",
            display::Text::LaunchMail => "開啟郵件",
            display::Text::MediaPlayPause => "播放 / 暫停",
            display::Text::MediaSelect => "選擇媒體",
            display::Text::MediaStop => "停止媒體",
            display::Text::MediaTrackNext => "下一首",
            display::Text::MediaTrackPrevious => "上一首",
            display::Text::Power => "電源",
            display::Text::Sleep => "睡眠",
            display::Text::AudioVolumeDown => "降低音量",
            display::Text::AudioVolumeMute => "靜音",
            display::Text::AudioVolumeUp => "提高音量",
            display::Text::WakeUp => "喚醒",
            display::Text::Again => "重複",
            display::Text::Copy => "複製",
            display::Text::Cut => "剪下",
            display::Text::Find => "尋找",
            display::Text::Open => "開啟",
            display::Text::Paste => "貼上",
            display::Text::Properties => "內容",
            display::Text::Select => "選取",
            display::Text::Undo => "復原",
            display::Text::BrightnessDown => "降低亮度",
            display::Text::BrightnessUp => "提高亮度",
            display::Text::DisplayToggle => "切換內建 / 外接顯示器",
            display::Text::KeyboardLayout => "選擇鍵盤配置",
            display::Text::LaunchAssistant => "開啟助理",
            display::Text::LaunchControlPanel => "開啟控制台",
            display::Text::LaunchScreenSaver => "啟動螢幕保護程式",
            display::Text::MailForward => "轉寄郵件",
            display::Text::MailReply => "回覆郵件",
            display::Text::MailSend => "傳送郵件",
            display::Text::MediaFastForward => "快轉",
            display::Text::MediaPause => "暫停媒體",
            display::Text::MediaPlay => "播放媒體",
            display::Text::MediaRecord => "錄製媒體",
            display::Text::MediaRewind => "倒轉",
            display::Text::MicrophoneMute => "麥克風靜音",
            display::Text::PrivacyScreen => "切換隱私螢幕",
            display::Text::SelectTask => "選擇工作",
            display::Text::ShowAllWindows => "顯示所有視窗",
            display::Text::ZoomToggle => "切換縮放",
        },
        #[cfg(feature = "std")]
        Text::StartSplit => "開始 / 分段",
        #[cfg(feature = "std")]
        Text::StartSplitDescription => "用於分段並開始新嘗試的快捷鍵。",
        #[cfg(feature = "std")]
        Text::Reset => "重設",
        #[cfg(feature = "std")]
        Text::ResetDescription => "用於重設目前嘗試的快捷鍵。",
        #[cfg(feature = "std")]
        Text::UndoSplit => "撤銷分段",
        #[cfg(feature = "std")]
        Text::UndoSplitDescription => "用於撤銷上一個分段的快捷鍵。",
        #[cfg(feature = "std")]
        Text::SkipSplit => "跳過分段",
        #[cfg(feature = "std")]
        Text::SkipSplitDescription => "用於跳過目前分段的快捷鍵。",
        #[cfg(feature = "std")]
        Text::Pause => "暫停",
        #[cfg(feature = "std")]
        Text::PauseDescription => "用於暫停目前嘗試的快捷鍵。也可用於開始新嘗試。",
        #[cfg(feature = "std")]
        Text::UndoAllPauses => "撤銷全部暫停",
        #[cfg(feature = "std")]
        Text::UndoAllPausesDescription => {
            "用於從目前時間中移除所有暫停時間的快捷鍵。適用於誤暫停的情況。"
        }
        #[cfg(feature = "std")]
        Text::PreviousComparison => "上一個比較",
        #[cfg(feature = "std")]
        Text::PreviousComparisonDescription => "用於切換到上一個比較的快捷鍵。",
        #[cfg(feature = "std")]
        Text::NextComparison => "下一個比較",
        #[cfg(feature = "std")]
        Text::NextComparisonDescription => "用於切換到下一個比較的快捷鍵。",
        #[cfg(feature = "std")]
        Text::ToggleTimingMethod => "切換計時方式",
        #[cfg(feature = "std")]
        Text::ToggleTimingMethodDescription => "用於在「實際時間」和「遊戲時間」之間切換的快捷鍵。",
        Text::TimerBackground => "背景",
        Text::TimerBackgroundDescription => {
            "元件後方顯示的背景。也可以將領先或落後時間所對應的顏色套用為背景色。"
        }
        Text::SegmentTimer => "分段計時器",
        Text::SegmentTimerDescription => {
            "指定是否顯示自目前分段開始經過的時間，而不是自目前嘗試開始經過的時間。"
        }
        Text::TimingMethod => "計時方式",
        Text::TimingMethodDescription => "指定使用的計時方式。若未指定，則使用目前的計時方式。",
        Text::Height => "高度",
        Text::HeightDescription => "計時器的高度。",
        Text::TimerTextColor => "文字顏色",
        Text::TimerTextColorDescription => {
            "顯示時間的顏色。若未指定，則根據目前嘗試的表現自動選擇顏色。這些顏色可以在版面配置的一般設定中指定。"
        }
        Text::ShowGradient => "顯示漸層",
        Text::ShowGradientDescription => "指定是否以漸層顯示計時器的顏色。",
        Text::DigitsFormat => "位數格式",
        Text::DigitsFormatDescription => {
            "指定要顯示的位數。若時間長度不足以填滿指定的位數，則以 0 補齊。"
        }
        Text::Accuracy => "精確度",
        Text::AccuracyDescription => "顯示時間的精確度。",
        Text::TitleBackground => "背景",
        Text::TitleBackgroundDescription => "元件後方顯示的背景。",
        Text::TitleTextColor => "文字顏色",
        Text::TitleTextColorDescription => "標題文字的顏色。若未指定，則使用版面配置的顏色。",
        Text::ShowGameName => "顯示遊戲名稱",
        Text::ShowGameNameDescription => "指定是否在標題中顯示遊戲名稱。",
        Text::ShowCategoryName => "顯示分類名稱",
        Text::ShowCategoryNameDescription => "指定是否在標題中顯示分類名稱。",
        Text::ShowFinishedRunsCount => "顯示完成次數",
        Text::ShowFinishedRunsCountDescription => "指定是否顯示成功完成的嘗試次數。",
        Text::ShowAttemptCount => "顯示嘗試次數",
        Text::ShowAttemptCountDescription => "指定是否顯示嘗試的總次數。",
        Text::TextAlignment => "文字對齊",
        Text::TextAlignmentDescription => "指定標題的對齊方式。",
        Text::DisplayTextAsSingleLine => "單行顯示文字",
        Text::DisplayTextAsSingleLineDescription => {
            "指定是否將標題顯示為單行，而不是將遊戲名稱與分類名稱分成兩行顯示。"
        }
        Text::DisplayGameIcon => "顯示遊戲圖示",
        Text::DisplayGameIconDescription => "若分段列表中儲存了遊戲圖示，指定是否顯示該圖示。",
        Text::ShowRegion => "顯示地區",
        Text::ShowRegionDescription => {
            "分類名稱可以附加額外資訊。若分段編輯器的變數分頁中提供了遊戲地區，則將地區附加至分類名稱。"
        }
        Text::ShowPlatform => "顯示平台",
        Text::ShowPlatformDescription => {
            "分類名稱可以附加額外資訊。若分段編輯器的變數分頁中提供了遊玩平台，則將平台附加至分類名稱。"
        }
        Text::ShowVariables => "顯示變數",
        Text::ShowVariablesDescription => {
            "分類名稱可以附加額外資訊。此設定會附加分段編輯器的變數分頁中提供的額外變數。這裡指的是 Speedrun.com 變數，而不是自訂變數。"
        }
        Text::TotalPlaytimeBackground => "背景",
        Text::TotalPlaytimeBackgroundDescription => "元件後方顯示的背景。",
        Text::DisplayTwoRows => "顯示為兩列",
        Text::DisplayTwoRowsDescription => "指定是否將元件名稱與總遊玩時間分成兩列顯示。",
        Text::ShowDays => "顯示天數（>24小時）",
        Text::ShowDaysDescription => "當總遊玩時間達到 24 小時或以上時，指定是否顯示天數。",
        Text::LabelColor => "標籤顏色",
        Text::LabelColorDescription => "元件名稱的顏色。若未指定，則使用版面配置的顏色。",
        Text::ValueColor => "數值顏色",
        Text::ValueColorDescription => "總遊玩時間的顏色。若未指定，則使用版面配置的顏色。",
        Text::BlankSpaceBackground => "背景",
        Text::BlankSpaceBackgroundDescription => "元件後方顯示的背景。",
        Text::BlankSpaceSize => "大小",
        Text::BlankSpaceSizeDescription => "元件的大小。",
        Text::CurrentComparisonBackground => "背景",
        Text::CurrentComparisonBackgroundDescription => "元件後方顯示的背景。",
        Text::CurrentComparisonDisplayTwoRows => "顯示為兩列",
        Text::CurrentComparisonDisplayTwoRowsDescription => {
            "指定是否將元件名稱與比較分成兩列顯示。"
        }
        Text::CurrentComparisonLabelColor => "標籤顏色",
        Text::CurrentComparisonLabelColorDescription => {
            "元件名稱的顏色。若未指定，則使用版面配置的顏色。"
        }
        Text::CurrentComparisonValueColor => "數值顏色",
        Text::CurrentComparisonValueColorDescription => {
            "比較名稱的顏色。若未指定，則使用版面配置的顏色。"
        }
        Text::CurrentPaceBackground => "背景",
        Text::CurrentPaceBackgroundDescription => "元件後方顯示的背景。",
        Text::CurrentPaceComparison => "比較",
        Text::CurrentPaceComparisonDescription => {
            "用來預測最終時間的比較。若未指定，則使用目前的比較。"
        }
        Text::CurrentPaceDisplayTwoRows => "顯示為兩列",
        Text::CurrentPaceDisplayTwoRowsDescription => "指定是否將元件名稱與預測時間分成兩列顯示。",
        Text::CurrentPaceLabelColor => "標籤顏色",
        Text::CurrentPaceLabelColorDescription => {
            "元件名稱的顏色。若未指定，則使用版面配置的顏色。"
        }
        Text::CurrentPaceValueColor => "數值顏色",
        Text::CurrentPaceValueColorDescription => {
            "預測時間的顏色。若未指定，則使用版面配置的顏色。"
        }
        Text::CurrentPaceAccuracy => "精確度",
        Text::CurrentPaceAccuracyDescription => "顯示預測時間的精確度。",
        Text::DeltaBackground => "背景",
        Text::DeltaBackgroundDescription => "元件後方顯示的背景。",
        Text::DeltaComparison => "比較",
        Text::DeltaComparisonDescription => {
            "用來計算目前嘗試領先或落後多少時間的比較。若未指定，則使用目前的比較。"
        }
        Text::DeltaDisplayTwoRows => "顯示為兩列",
        Text::DeltaDisplayTwoRowsDescription => "指定是否將比較名稱與時間差分成兩列顯示。",
        Text::DeltaLabelColor => "標籤顏色",
        Text::DeltaLabelColorDescription => "比較名稱的顏色。若未指定，則使用版面配置的顏色。",
        Text::DeltaDropDecimals => "省略小數",
        Text::DeltaDropDecimalsDescription => {
            "指定是否在顯示的時間差超過一分鐘時，不再顯示小數部分。"
        }
        Text::DeltaAccuracy => "精確度",
        Text::DeltaAccuracyDescription => "顯示時間差的精確度。",
        Text::SumOfBestBackground => "背景",
        Text::SumOfBestBackgroundDescription => "元件後方顯示的背景。",
        Text::SumOfBestDisplayTwoRows => "顯示為兩列",
        Text::SumOfBestDisplayTwoRowsDescription => {
            "指定是否將元件名稱與最佳分段總和分成兩列顯示。"
        }
        Text::SumOfBestLabelColor => "標籤顏色",
        Text::SumOfBestLabelColorDescription => "元件名稱的顏色。若未指定，則使用版面配置的顏色。",
        Text::SumOfBestValueColor => "數值顏色",
        Text::SumOfBestValueColorDescription => {
            "最佳分段總和的顏色。若未指定，則使用版面配置的顏色。"
        }
        Text::SumOfBestAccuracy => "精確度",
        Text::SumOfBestAccuracyDescription => "顯示最佳分段總和的精確度。",
        Text::PbChanceBackground => "背景",
        Text::PbChanceBackgroundDescription => "元件後方顯示的背景。",
        Text::PbChanceDisplayTwoRows => "顯示為兩列",
        Text::PbChanceDisplayTwoRowsDescription => {
            "指定是否將元件名稱與刷新個人最佳的機率分成兩列顯示。"
        }
        Text::PbChanceLabelColor => "標籤顏色",
        Text::PbChanceLabelColorDescription => "元件名稱的顏色。若未指定，則使用版面配置的顏色。",
        Text::PbChanceValueColor => "數值顏色",
        Text::PbChanceValueColorDescription => {
            "刷新個人最佳的機率的顏色。若未指定，則使用版面配置的顏色。"
        }
        Text::PossibleTimeSaveBackground => "背景",
        Text::PossibleTimeSaveBackgroundDescription => "元件後方顯示的背景。",
        Text::PossibleTimeSaveComparison => "比較",
        Text::PossibleTimeSaveComparisonDescription => {
            "用來計算可節省時間的比較。若未指定，則使用目前的比較。"
        }
        Text::PossibleTimeSaveDisplayTwoRows => "顯示為兩列",
        Text::PossibleTimeSaveDisplayTwoRowsDescription => {
            "指定是否將元件名稱與可節省時間分成兩列顯示。"
        }
        Text::PossibleTimeSaveShowTotal => "顯示總可節省時間",
        Text::PossibleTimeSaveShowTotalDescription => {
            "指定是否顯示目前嘗試剩餘部分的總可節省時間，而不是目前分段的可節省時間。"
        }
        Text::PossibleTimeSaveLabelColor => "標籤顏色",
        Text::PossibleTimeSaveLabelColorDescription => {
            "元件名稱的顏色。若未指定，則使用版面配置的顏色。"
        }
        Text::PossibleTimeSaveValueColor => "數值顏色",
        Text::PossibleTimeSaveValueColorDescription => {
            "可節省時間的顏色。若未指定，則使用版面配置的顏色。"
        }
        Text::PossibleTimeSaveAccuracy => "精確度",
        Text::PossibleTimeSaveAccuracyDescription => "顯示可節省時間的精確度。",
        Text::PreviousSegmentBackground => "背景",
        Text::PreviousSegmentBackgroundDescription => "元件後方顯示的背景。",
        Text::PreviousSegmentComparison => "比較",
        Text::PreviousSegmentComparisonDescription => {
            "用來計算上一個分段節省或損失多少時間的比較。若未指定，則使用目前的比較。"
        }
        Text::PreviousSegmentDisplayTwoRows => "顯示為兩列",
        Text::PreviousSegmentDisplayTwoRowsDescription => {
            "指定是否將元件名稱與節省或損失的時間分成兩列顯示。"
        }
        Text::PreviousSegmentLabelColor => "標籤顏色",
        Text::PreviousSegmentLabelColorDescription => {
            "元件名稱的顏色。若未指定，則使用版面配置的顏色。"
        }
        Text::PreviousSegmentDropDecimals => "省略小數",
        Text::PreviousSegmentDropDecimalsDescription => {
            "指定是否在顯示的時間超過一分鐘時，省略小數部分。"
        }
        Text::PreviousSegmentAccuracy => "精確度",
        Text::PreviousSegmentAccuracyDescription => "顯示時間的精確度。",
        Text::PreviousSegmentShowPossibleTimeSave => "顯示可節省時間",
        Text::PreviousSegmentShowPossibleTimeSaveDescription => {
            "指定是否在顯示上一個分段節省或損失的時間時，也顯示該分段原本可節省的時間。"
        }
        Text::SegmentTimeBackground => "背景",
        Text::SegmentTimeBackgroundDescription => "元件後方顯示的背景。",
        Text::SegmentTimeComparison => "比較",
        Text::SegmentTimeComparisonDescription => {
            "用來顯示分段時間的比較。若未指定，則使用目前的比較。"
        }
        Text::SegmentTimeDisplayTwoRows => "顯示為兩列",
        Text::SegmentTimeDisplayTwoRowsDescription => "指定是否將元件名稱與分段時間分成兩列顯示。",
        Text::SegmentTimeLabelColor => "標籤顏色",
        Text::SegmentTimeLabelColorDescription => {
            "元件名稱的顏色。若未指定，則使用版面配置的顏色。"
        }
        Text::SegmentTimeValueColor => "數值顏色",
        Text::SegmentTimeValueColorDescription => {
            "分段時間的顏色。若未指定，則使用版面配置的顏色。"
        }
        Text::SegmentTimeAccuracy => "精確度",
        Text::SegmentTimeAccuracyDescription => "顯示分段時間的精確度。",
        Text::GraphComparison => "比較",
        Text::GraphComparisonDescription => "圖表使用的比較。若未指定，則使用目前的比較。",
        Text::GraphHeight => "高度",
        Text::GraphHeightDescription => "圖表的高度。",
        Text::GraphShowBestSegments => "顯示最佳分段",
        Text::GraphShowBestSegmentsDescription => {
            "指定是否使用版面配置的最佳分段顏色來標示最佳分段。"
        }
        Text::GraphLiveGraph => "即時圖表",
        Text::GraphLiveGraphDescription => {
            "指定是否持續自動更新圖表。若停用此設定，則只會在目前分段變更時更新圖表。"
        }
        Text::GraphFlipGraph => "翻轉圖表",
        Text::GraphFlipGraphDescription => {
            "指定是否垂直翻轉圖表。若未啟用，領先比較的累計時間會顯示在 X 軸下方，落後的時間則在上方。啟用此設定會反轉上下位置。"
        }
        Text::GraphBehindBackgroundColor => "落後背景顏色",
        Text::GraphBehindBackgroundColorDescription => "圖表中落後比較的時間區域的背景顏色。",
        Text::GraphAheadBackgroundColor => "領先背景顏色",
        Text::GraphAheadBackgroundColorDescription => "圖表中領先比較的時間區域的背景顏色。",
        Text::GraphGridLinesColor => "格線顏色",
        Text::GraphGridLinesColorDescription => "圖表格線的顏色。",
        Text::GraphLinesColor => "圖表線條顏色",
        Text::GraphLinesColorDescription => "連接圖表各點的線條顏色。",
        Text::GraphPartialFillColor => "部分填滿顏色",
        Text::GraphPartialFillColorDescription => {
            "用來填滿 X 軸與圖表之間區域的顏色。部分填滿顏色只用於即時變化的區域，也就是從上一個分段的累計時間到目前時間之間的範圍。"
        }
        Text::GraphCompleteFillColor => "完整填滿顏色",
        Text::GraphCompleteFillColorDescription => {
            "用來填滿 X 軸與圖表之間區域的顏色，不包含即時變化的圖表區段。"
        }
        Text::DetailedTimerBackground => "背景",
        Text::DetailedTimerBackgroundDescription => "元件後方顯示的背景。",
        Text::DetailedTimerTimingMethod => "計時方式",
        Text::DetailedTimerTimingMethodDescription => {
            "指定使用的計時方式。若未指定，則使用目前的計時方式。"
        }
        Text::DetailedTimerComparison1 => "比較 1",
        Text::DetailedTimerComparison1Description => {
            "第一個要顯示分段時間的比較。若未指定，則使用目前的比較。"
        }
        Text::DetailedTimerComparison2 => "比較 2",
        Text::DetailedTimerComparison2Description => {
            "第二個要顯示分段時間的比較。若未指定，則使用目前的比較，但第一個比較也未指定時除外。隱藏第二個比較時，不會顯示此比較。"
        }
        Text::DetailedTimerHideSecondComparison => "隱藏第二個比較",
        Text::DetailedTimerHideSecondComparisonDescription => "指定是否只顯示一個比較。",
        Text::DetailedTimerTimerHeight => "計時器高度",
        Text::DetailedTimerTimerHeightDescription => "整次嘗試的計時器高度。",
        Text::DetailedTimerSegmentTimerHeight => "分段計時器高度",
        Text::DetailedTimerSegmentTimerHeightDescription => "分段計時器的高度。",
        Text::DetailedTimerTimerColor => "計時器顏色",
        Text::DetailedTimerTimerColorDescription => {
            "允許指定主計時器固定使用的顏色，取代根據目前嘗試的表現自動選擇的顏色。"
        }
        Text::DetailedTimerShowTimerGradient => "顯示計時器漸層",
        Text::DetailedTimerShowTimerGradientDescription => {
            "啟用此設定時，主計時器會自動將其顏色轉換為垂直漸層。否則會直接使用原本的顏色，不顯示漸層。"
        }
        Text::DetailedTimerTimerDigitsFormat => "計時器位數格式",
        Text::DetailedTimerTimerDigitsFormatDescription => {
            "指定主計時器要顯示的位數。若時間長度不足以填滿指定的位數，則以 0 補齊。"
        }
        Text::DetailedTimerTimerAccuracy => "計時器精確度",
        Text::DetailedTimerTimerAccuracyDescription => "主計時器顯示時間的精確度。",
        Text::DetailedTimerSegmentTimerColor => "分段計時器顏色",
        Text::DetailedTimerSegmentTimerColorDescription => {
            "將分段計時器的顏色變更為預設顏色以外的顏色。"
        }
        Text::DetailedTimerShowSegmentTimerGradient => "顯示分段計時器漸層",
        Text::DetailedTimerShowSegmentTimerGradientDescription => {
            "啟用此設定時，分段計時器會自動將其顏色轉換為垂直漸層。否則會直接使用原本的顏色，不顯示漸層。"
        }
        Text::DetailedTimerSegmentTimerDigitsFormat => "分段計時器位數格式",
        Text::DetailedTimerSegmentTimerDigitsFormatDescription => {
            "指定分段計時器要顯示的位數。若時間長度不足以填滿指定的位數，則以 0 補齊。"
        }
        Text::DetailedTimerSegmentTimerAccuracy => "分段計時器精確度",
        Text::DetailedTimerSegmentTimerAccuracyDescription => "分段計時器顯示時間的精確度。",
        Text::DetailedTimerComparisonNamesColor => "比較名稱顏色",
        Text::DetailedTimerComparisonNamesColorDescription => {
            "顯示比較名稱時使用的顏色。若未指定，則使用版面配置的顏色。"
        }
        Text::DetailedTimerComparisonTimesColor => "比較時間顏色",
        Text::DetailedTimerComparisonTimesColorDescription => {
            "顯示比較時間時使用的顏色。若未指定，則使用版面配置的顏色。"
        }
        Text::DetailedTimerComparisonTimesAccuracy => "比較時間精確度",
        Text::DetailedTimerComparisonTimesAccuracyDescription => "比較時間的精確度。",
        Text::DetailedTimerShowSegmentName => "顯示分段名稱",
        Text::DetailedTimerShowSegmentNameDescription => "指定是否顯示分段名稱。",
        Text::DetailedTimerSegmentNameColor => "分段名稱顏色",
        Text::DetailedTimerSegmentNameColorDescription => {
            "顯示分段名稱時使用的顏色。若未指定，則使用版面配置的顏色。"
        }
        Text::DetailedTimerDisplayIcon => "顯示圖示",
        Text::DetailedTimerDisplayIconDescription => {
            "控制分段圖示是隱藏、跨越兩列，還是只佔第一列。"
        }
        Text::SplitsBackground => "背景",
        Text::SplitsBackgroundDescription => {
            "元件後方顯示的背景。可以選擇交替使用兩種顏色，讓每列的背景在所選的兩種顏色之間交替。"
        }
        Text::SplitsTotalRows => "總列數",
        Text::SplitsTotalRowsDescription => {
            "列表中要顯示的分段總列數。設為 0 時會顯示所有分段。若設為小於分段總數的數值，則只會顯示其中一部分，並可在此顯示範圍內上下捲動。"
        }
        Text::SplitsUpcomingSegments => "接下來的分段",
        Text::SplitsUpcomingSegmentsDescription => {
            "若分段數量超過顯示的列數，顯示範圍會在目前分段變更時自動上下捲動。此數值決定捲動範圍中至少要顯示多少個尚未到達的分段。"
        }
        Text::SplitsShowThinSeparators => "顯示細分隔線",
        Text::SplitsShowThinSeparatorsDescription => "指定是否在各個分段列之間顯示細分隔線。",
        Text::SplitsShowGapSeparators => "顯示省略處分隔線",
        Text::SplitsShowGapSeparatorsDescription => {
            "當某列前方緊鄰的一列或多列被省略在捲動範圍之外時，指定是否在該列前顯示醒目的分隔線。"
        }
        Text::SplitsAlwaysShowLastSplit => "一律顯示最後一個分段",
        Text::SplitsAlwaysShowLastSplitDescription => {
            "若捲動範圍未顯示所有分段，此選項決定是否一律顯示最後一個分段，因為它包含所選比較的總時間。這通常是玩家的個人最佳，因此是很有用的資訊。"
        }
        Text::SplitsFillWithBlankSpace => "以空白填滿",
        Text::SplitsFillWithBlankSpaceDescription => {
            "若分段數量不足以填滿列表，此選項可用空白填滿剩餘的列，讓顯示的總列數維持為設定值。否則，顯示的總列數會減少為實際的分段數量。"
        }
        Text::SplitsShowTimesBelowSegmentName => "在分段名稱下方顯示時間",
        Text::SplitsShowTimesBelowSegmentNameDescription => {
            "指定是否在分段名稱下方顯示時間。否則，時間會顯示在分段名稱旁邊。"
        }
        Text::SplitsCurrentSegmentGradient => "目前分段漸層",
        Text::SplitsCurrentSegmentGradientDescription => {
            "顯示在目前分段後方的漸層，用來標示這是目前的分段。"
        }
        Text::SplitsSplitTimeAccuracy => "累計時間精確度",
        Text::SplitsSplitTimeAccuracyDescription => "指定顯示累計時間的欄位所使用的精確度。",
        Text::SplitsSegmentTimeAccuracy => "分段時間精確度",
        Text::SplitsSegmentTimeAccuracyDescription => "指定顯示分段時間的欄位所使用的精確度。",
        Text::SplitsDeltaTimeAccuracy => "時間差精確度",
        Text::SplitsDeltaTimeAccuracyDescription => "指定顯示領先或落後時間的欄位所使用的精確度。",
        Text::SplitsDropDeltaDecimals => "顯示分鐘時省略時間差的小數",
        Text::SplitsDropDeltaDecimalsDescription => {
            "指定是否在顯示領先或落後時間的欄位超過一分鐘時，不再顯示小數部分。"
        }
        Text::SplitsShowColumnLabels => "顯示欄位標籤",
        Text::SplitsShowColumnLabelsDescription => "指定是否在列表頂端顯示欄位名稱。",
        Text::SplitsSubsplitDisplayMode => "子分段顯示模式",
        Text::SplitsSubsplitDisplayModeDescription => "控制子分段的顯示方式。",
        Text::SplitsColumns => "欄位",
        Text::SplitsColumnsDescription => {
            "每列要顯示的欄位數量。每個欄位可以設定為顯示不同的資訊。欄位的定義順序是由右至左。"
        }
        Text::SplitsColumnName => "欄位名稱",
        Text::SplitsColumnNameDescription => "欄位的名稱。啟用顯示欄位標籤時，會顯示在列表頂端。",
        Text::SplitsColumnType => "欄位類型",
        Text::SplitsColumnTypeDescription => {
            "此欄位顯示的資訊類型，可以是時間或儲存在分段列表中的自訂變數。"
        }
        Text::SplitsVariableName => "變數名稱",
        Text::SplitsVariableNameDescription => "此欄位顯示的自訂變數名稱。",
        Text::SplitsStartWith => "起始值",
        Text::SplitsStartWithDescription => {
            "此欄位在每個分段開始時顯示的值。「更新條件」決定何時替換此時間。"
        }
        Text::SplitsUpdateWith => "更新值",
        Text::SplitsUpdateWithDescription => {
            "達成特定條件時，時間會更新為此處指定的值。該條件通常是正在進行此分段，或已完成此分段。"
        }
        Text::SplitsUpdateTrigger => "更新條件",
        Text::SplitsUpdateTriggerDescription => {
            "將時間替換為「更新值」欄位所指定的值之前，必須達成的條件。在達成此條件之前，時間會維持為「起始值」欄位所指定的值。"
        }
        Text::SplitsColumnComparison => "比較",
        Text::SplitsColumnComparisonDescription => {
            "此欄位所使用的比較。若未指定，則使用目前的比較。"
        }
        Text::SplitsColumnTimingMethod => "計時方式",
        Text::SplitsColumnTimingMethodDescription => {
            "指定此欄位使用的計時方式。若未指定，則使用目前的計時方式。"
        }
        Text::TextComponentBackground => "背景",
        Text::TextComponentBackgroundDescription => "元件後方顯示的背景。",
        Text::TextComponentUseVariable => "使用變數",
        Text::TextComponentUseVariableDescription => {
            "指定是否使用自訂變數顯示動態數值。自訂變數可以在分段編輯器中設定，也可以由自動分段器自動提供。"
        }
        Text::TextComponentSplit => "分開文字",
        Text::TextComponentSplitDescription => {
            "指定是否將文字分成左右兩部分。若未啟用，則只會顯示一段置中的文字。"
        }
        Text::TextComponentText => "文字",
        Text::TextComponentTextDescription => "指定要置中顯示的文字。",
        Text::TextComponentLeft => "左側",
        Text::TextComponentLeftDescription => "指定要在左側顯示的文字。",
        Text::TextComponentRight => "右側",
        Text::TextComponentRightDescription => "指定要在右側顯示的文字。",
        Text::TextComponentVariable => "變數",
        Text::TextComponentVariableDescription => "指定要顯示的自訂變數名稱。",
        Text::TextComponentTextColor => "文字顏色",
        Text::TextComponentTextColorDescription => "文字的顏色。",
        Text::TextComponentLeftColor => "左側顏色",
        Text::TextComponentLeftColorDescription => "左側文字的顏色。",
        Text::TextComponentRightColor => "右側顏色",
        Text::TextComponentRightColorDescription => "右側文字的顏色。",
        Text::TextComponentNameColor => "名稱顏色",
        Text::TextComponentNameColorDescription => "變數名稱的顏色。",
        Text::TextComponentValueColor => "數值顏色",
        Text::TextComponentValueColorDescription => "變數值的顏色。",
        Text::TextComponentDisplayTwoRows => "顯示為兩列",
        Text::TextComponentDisplayTwoRowsDescription => "指定是否將左右兩部分的文字分成兩列顯示。",
        Text::LayoutDirection => "版面配置方向",
        Text::LayoutDirectionDescription => "元件排列的方向。",
        Text::GroupFixedWidth => "固定寬度",
        Text::GroupFixedWidthDescription => "此元件欄的選用固定寬度。若未指定，則自動決定寬度。",
        Text::GroupFixedHeight => "固定高度",
        Text::GroupFixedHeightDescription => "此元件列的選用固定高度。若未指定，則自動決定高度。",
        Text::CustomTimerFont => "自訂計時器字型",
        Text::CustomTimerFontDescription => "允許為計時器指定自訂字型。若未設定，則使用預設字型。",
        Text::CustomTimesFont => "自訂時間字型",
        Text::CustomTimesFontDescription => "允許為時間指定自訂字型。若未設定，則使用預設字型。",
        Text::CustomTextFont => "自訂文字字型",
        Text::CustomTextFontDescription => "允許為文字指定自訂字型。若未設定，則使用預設字型。",
        Text::TextShadow => "文字陰影",
        Text::TextShadowDescription => "允許選擇性地指定文字陰影的顏色。",
        Text::Background => "背景",
        Text::BackgroundDescription => "整個版面配置後方顯示的背景。",
        Text::BestSegment => "最佳分段",
        Text::BestSegmentDescription => "達成新的最佳分段時使用的顏色。",
        Text::AheadGainingTime => "領先（持續拉開差距）",
        Text::AheadGainingTimeDescription => "領先比較且持續擴大領先時間時使用的顏色。",
        Text::AheadLosingTime => "領先（差距縮小）",
        Text::AheadLosingTimeDescription => "領先比較但正在損失時間時使用的顏色。",
        Text::BehindGainingTime => "落後（追回時間）",
        Text::BehindGainingTimeDescription => "落後比較但正在追回時間時使用的顏色。",
        Text::BehindLosingTime => "落後（差距擴大）",
        Text::BehindLosingTimeDescription => "落後比較且持續損失時間時使用的顏色。",
        Text::NotRunning => "未執行",
        Text::NotRunningDescription => "沒有進行中的嘗試時使用的顏色。",
        Text::PersonalBest => "個人最佳",
        Text::PersonalBestDescription => "達成新的個人最佳時使用的顏色。",
        Text::Paused => "已暫停",
        Text::PausedDescription => "計時器暫停時使用的顏色。",
        Text::ThinSeparators => "細分隔線",
        Text::ThinSeparatorsDescription => "細分隔線的顏色。",
        Text::Separators => "分隔線",
        Text::SeparatorsDescription => "一般分隔線的顏色。",
        Text::TextColor => "文字",
        Text::TextColorDescription => "未指定自身顏色的文字所使用的顏色。",
        Text::ComponentBlankSpace => "空白",
        Text::ComponentCurrentComparison => "目前比較",
        Text::ComponentCurrentPace => "目前配速",
        Text::ComponentDelta => "時間差",
        Text::ComponentDetailedTimer => "詳細計時器",
        Text::ComponentGraph => "圖表",
        Text::ComponentPbChance => "刷新個人最佳的機率",
        Text::ComponentPossibleTimeSave => "可節省時間",
        Text::ComponentPreviousSegment => "上一個分段",
        Text::ComponentSegmentTime => "分段時間",
        Text::ComponentSeparator => "分隔線",
        Text::ComponentSplits => "分段列表",
        Text::ComponentSumOfBest => "最佳分段總和",
        Text::ComponentText => "文字",
        Text::ComponentTimer => "計時器",
        Text::ComponentSegmentTimer => "分段計時器",
        Text::ComponentTitle => "標題",
        Text::ComponentTotalPlaytime => "總遊玩時間",
        Text::Row => "列",
        Text::Column => "欄",
        Text::EmptyContainer => "空",
        Text::Carousel => "輪播",
        Text::CarouselFixedSize => "固定大小",
        Text::CarouselFixedSizeDescription => {
            "覆寫輪播元件的大小。若未設定，則根據最大的子元件自動決定大小。"
        }
        Text::CarouselInterval => "間隔（秒）",
        Text::CarouselIntervalDescription => {
            "切換到下一個子元件之前，目前子元件最多顯示的秒數。若另一個子元件的內容有明顯變化，輪播也可能提早切換。"
        }
        Text::CarouselMinDisplay => "最短顯示時間（秒）",
        Text::CarouselMinDisplayDescription => "切換到其他子元件之前，目前子元件至少要顯示的秒數。",
        Text::ComponentCurrentPaceBestPossibleTime => "最佳可能時間",
        Text::ComponentCurrentPaceWorstPossibleTime => "最差可能時間",
        Text::ComponentCurrentPacePredictedTime => "預測時間",
        Text::ComponentSegmentTimeBest => "最佳分段時間",
        Text::ComponentSegmentTimeWorst => "最差分段時間",
        Text::ComponentSegmentTimeAverage => "平均分段時間",
        Text::ComponentSegmentTimeMedian => "分段時間中位數",
        Text::ComponentSegmentTimeLatest => "最近分段時間",
        Text::ComponentPossibleTimeSaveTotal => "總可節省時間",
        Text::LiveSegment => "即時分段",
        Text::LiveSegmentShort => "即時分段",
        Text::PreviousSegmentShort => "上一分段",
        Text::PreviousSegmentAbbreviation => "上段",
        Text::ComparingAgainst => "比較對象",
        Text::ComparisonShort => "比較",
        Text::CurrentPaceBestPossibleTimeShort => "最佳可能時間",
        Text::CurrentPaceBestTimeShort => "最佳時間",
        Text::CurrentPaceBestPossibleTimeAbbreviation => "最佳可能",
        Text::CurrentPaceWorstPossibleTimeShort => "最差可能時間",
        Text::CurrentPaceWorstTimeShort => "最差時間",
        Text::CurrentPacePredictedTimeShort => "預測時間",
        Text::CurrentPaceShort => "目前配速",
        Text::CurrentPaceAbbreviation => "配速",
        Text::Goal => "目標",
        Text::SumOfBestSegments => "最佳分段總和",
        Text::SumOfBestShort => "最佳總和",
        Text::SumOfBestAbbreviation => "最佳總和",
        Text::PlaytimeShort => "遊玩時間",
        Text::BestSegmentTimeShort => "最佳分段時間",
        Text::BestSegmentShort => "最佳分段",
        Text::WorstSegmentTimeShort => "最差分段時間",
        Text::WorstSegmentShort => "最差分段",
        Text::AverageSegmentTimeShort => "平均分段時間",
        Text::AverageSegmentShort => "平均分段",
        Text::MedianSegmentTimeShort => "分段時間中位數",
        Text::MedianSegmentShort => "分段中位數",
        Text::LatestSegmentTimeShort => "最近分段時間",
        Text::LatestSegmentShort => "最近分段",
        Text::SegmentTimeShort => "分段時間",
        Text::SplitTime => "累計時間",
        Text::PossibleTimeSaveShort => "可節省時間",
        Text::PossibleTimeSaveAbbreviation => "可節省",
        Text::TimeSaveShort => "節省時間",
        Text::RealTime => "實際時間",
        Text::GameTime => "遊戲時間",
        Text::Untitled => "未命名",
        Text::SumOfBestCleanerStartOfRun => "本次挑戰開始",
        Text::SumOfBestCleanerShouldRemove => "。你認為這個分段時間不準確，應該移除嗎？",
    }
}

pub const fn resolve_placeholder(text: PlaceholderText) -> &'static [Piece] {
    match text {
        PlaceholderText::Hotkey(template) => match template {
            display::Template::NumpadKey => &[Piece::Static("數字鍵盤 "), Piece::Dynamic(0)],
            display::Template::GamepadButton => &[Piece::Static("遊戲控制器 "), Piece::Dynamic(0)],
            display::Template::LeftKey => &[Piece::Static("左 "), Piece::Dynamic(0)],
            display::Template::RightKey => &[Piece::Static("右 "), Piece::Dynamic(0)],
            display::Template::LaunchApp => &[Piece::Static("啟動應用程式 "), Piece::Dynamic(0)],
        },
        PlaceholderText::SumOfBestCleanerSegmentTimeBetween => &[
            Piece::Static("以"),
            Piece::Dynamic(0),
            Piece::Static("計時，從「"),
            Piece::Dynamic(2),
            Piece::Static("」到「"),
            Piece::Dynamic(3),
            Piece::Static("」的分段時間記錄為"),
            Piece::Dynamic(1),
        ],
        PlaceholderText::SumOfBestCleanerFasterThanCombined => &[
            Piece::Static("，比這些分段的最佳時間總和"),
            Piece::Dynamic(0),
            Piece::Static("還快"),
        ],
        PlaceholderText::SumOfBestCleanerRunOn => &[
            Piece::Static("。這筆記錄來自於"),
            Piece::Dynamic(0),
            Piece::Static("的"),
            Piece::Dynamic(1),
            Piece::Static("開始的一次挑戰"),
        ],
    }
}
