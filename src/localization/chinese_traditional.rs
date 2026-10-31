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
        Text::ToggleTimingMethodDescription => "用於在「實時時間」和「遊戲時間」之間切換的快捷鍵。",
        Text::TimerBackground => "背景",
        Text::TimerBackgroundDescription => {
            "元件背後的背景。也可以將領先/落後對應的顏色套用為背景色。"
        }
        Text::SegmentTimer => "分段計時器",
        Text::SegmentTimerDescription => "指定顯示自目前分段開始的時間，還是自目前嘗試開始的時間。",
        Text::TimingMethod => "計時方式",
        Text::TimingMethodDescription => "指定計時方式為「實時時間」或「遊戲時間」。",
        Text::Height => "高度",
        Text::HeightDescription => "指定元件的高度。",
        Text::TimerTextColor => "文字顏色",
        Text::TimerTextColorDescription => "指定計時器文字顏色。",
        Text::ShowGradient => "顯示漸層",
        Text::ShowGradientDescription => "指定是否顯示漸層。",
        Text::DigitsFormat => "數字格式",
        Text::DigitsFormatDescription => "指定是否顯示毫秒或十分之一秒。",
        Text::Accuracy => "精確度",
        Text::AccuracyDescription => "顯示的時間精確度。",
        Text::TitleBackground => "標題背景",
        Text::TitleBackgroundDescription => "標題元件背景。",
        Text::TitleTextColor => "標題文字顏色",
        Text::TitleTextColorDescription => "標題文字顏色。",
        Text::ShowGameName => "顯示遊戲名稱",
        Text::ShowGameNameDescription => "指定是否顯示遊戲名稱。",
        Text::ShowCategoryName => "顯示分類名稱",
        Text::ShowCategoryNameDescription => "指定是否顯示分類名稱。",
        Text::ShowFinishedRunsCount => "顯示完成次數",
        Text::ShowFinishedRunsCountDescription => "指定是否顯示完成次數。",
        Text::ShowAttemptCount => "顯示嘗試次數",
        Text::ShowAttemptCountDescription => "指定是否顯示嘗試次數。",
        Text::TextAlignment => "文字對齊",
        Text::TextAlignmentDescription => "文字對齊方式。",
        Text::DisplayTextAsSingleLine => "單行顯示",
        Text::DisplayTextAsSingleLineDescription => "指定是否單行顯示文字。",
        Text::DisplayGameIcon => "顯示遊戲圖示",
        Text::DisplayGameIconDescription => "指定是否顯示遊戲圖示。",
        Text::ShowRegion => "顯示地區",
        Text::ShowRegionDescription => "指定是否顯示地區。",
        Text::ShowPlatform => "顯示平台",
        Text::ShowPlatformDescription => "指定是否顯示平台。",
        Text::ShowVariables => "顯示變數",
        Text::ShowVariablesDescription => "指定是否顯示變數。",
        Text::TotalPlaytimeBackground => "總遊玩時間背景",
        Text::TotalPlaytimeBackgroundDescription => "總遊玩時間元件背景。",
        Text::DisplayTwoRows => "顯示兩行",
        Text::DisplayTwoRowsDescription => "指定是否顯示兩行。",
        Text::ShowDays => "顯示天數",
        Text::ShowDaysDescription => "指定是否顯示天數。",
        Text::LabelColor => "標籤顏色",
        Text::LabelColorDescription => "標籤顏色。",
        Text::ValueColor => "數值顏色",
        Text::ValueColorDescription => "數值顏色。",
        Text::BlankSpaceBackground => "空白背景",
        Text::BlankSpaceBackgroundDescription => "空白元件背景。",
        Text::BlankSpaceSize => "空白大小",
        Text::BlankSpaceSizeDescription => "空白元件大小。",
        Text::GroupFixedWidth => "固定寬度",
        Text::GroupFixedWidthDescription => "此元件列的可選固定寬度。若未指定，寬度會自動確定。",
        Text::GroupFixedHeight => "固定高度",
        Text::GroupFixedHeightDescription => "此元件行的可選固定高度。若未指定，高度會自動確定。",
        Text::EmptyContainer => "空",
        Text::Carousel => "輪播",
        Text::CarouselFixedSize => "固定大小",
        Text::CarouselFixedSizeDescription => {
            "覆寫輪播元件的大小。若未指定，大小會根據最大的子元件自動決定。"
        }
        Text::CarouselInterval => "間隔（秒）",
        Text::CarouselIntervalDescription => {
            "在切換到下一個子元件之前，目前子元件最多顯示的秒數。若另一個子元件的內容有明顯變化，輪播也可能更早切換。"
        }
        Text::CarouselMinDisplay => "最短顯示時間（秒）",
        Text::CarouselMinDisplayDescription => "切換到其他子元件之前，目前子元件至少要顯示的秒數。",
        Text::SplitsSubsplitDisplayMode => "子分段顯示模式",
        Text::SplitsSubsplitDisplayModeDescription => "控制子分段的顯示方式。",
        Text::SplitTime => "分段時間",
        _ => super::chinese_simplified::resolve(text),
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
        PlaceholderText::SumOfBestCleanerSegmentTimeBetween
        | PlaceholderText::SumOfBestCleanerFasterThanCombined
        | PlaceholderText::SumOfBestCleanerRunOn => {
            super::chinese_simplified::resolve_placeholder(text)
        }
    }
}
