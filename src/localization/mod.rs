use core::fmt;

use crate::util::ascii_char::AsciiChar;

#[cfg(feature = "localization")]
mod brazilian_portuguese;
#[cfg(feature = "localization")]
mod chinese_simplified;
#[cfg(feature = "localization")]
mod chinese_traditional;
#[cfg(feature = "localization")]
mod dutch;
mod english;
#[cfg(feature = "localization")]
mod french;
#[cfg(feature = "localization")]
mod german;
#[cfg(feature = "localization")]
mod italian;
#[cfg(feature = "localization")]
mod japanese;
#[cfg(feature = "localization")]
mod korean;
#[cfg(feature = "localization")]
mod polish;
#[cfg(feature = "localization")]
mod portuguese;
#[cfg(feature = "localization")]
mod russian;
#[cfg(feature = "localization")]
mod spanish;

/// The supported languages for localization.
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Lang {
    /// English / English / en
    English,
    /// Dutch / Nederlands / nl
    #[cfg(feature = "localization")]
    Dutch,
    /// French / Français / fr
    #[cfg(feature = "localization")]
    French,
    /// German / Deutsch / de
    #[cfg(feature = "localization")]
    German,
    /// Italian / Italiano / it
    #[cfg(feature = "localization")]
    Italian,
    /// Portuguese / Português / pt
    #[cfg(feature = "localization")]
    Portuguese,
    /// Polish / Polski / pl
    #[cfg(feature = "localization")]
    Polish,
    /// Russian / Русский / ru
    #[cfg(feature = "localization")]
    Russian,
    /// Spanish / Español / es
    #[cfg(feature = "localization")]
    Spanish,
    /// Brazilian Portuguese / Português (Brasil) / pt-BR
    #[cfg(feature = "localization")]
    BrazilianPortuguese,
    /// Chinese (Simplified) / 简体中文 / zh-Hans
    #[cfg(feature = "localization")]
    ChineseSimplified,
    /// Chinese (Traditional) / 繁體中文 / zh-Hant
    #[cfg(feature = "localization")]
    ChineseTraditional,
    /// Japanese / 日本語 / ja
    #[cfg(feature = "localization")]
    Japanese,
    /// Korean / 한국어 / ko
    #[cfg(feature = "localization")]
    Korean,
}

impl Lang {
    /// Parses the locale string and returns the corresponding language.
    pub fn parse_locale(_locale: &str) -> Self {
        #[cfg(feature = "localization")]
        {
            let mut parts = _locale.split('-');
            let language = parts.next().unwrap_or(_locale).to_ascii_lowercase();
            let mut is_br = false;
            let mut is_traditional = false;
            for part in parts {
                if part.eq_ignore_ascii_case("br") {
                    is_br = true;
                }
                if part.eq_ignore_ascii_case("hant")
                    || part.eq_ignore_ascii_case("tw")
                    || part.eq_ignore_ascii_case("hk")
                    || part.eq_ignore_ascii_case("mo")
                {
                    is_traditional = true;
                }
            }

            match language.as_str() {
                "en" => Lang::English,
                "pt" => {
                    if is_br {
                        Lang::BrazilianPortuguese
                    } else {
                        Lang::Portuguese
                    }
                }
                "pl" => Lang::Polish,
                "zh" => {
                    if is_traditional {
                        Lang::ChineseTraditional
                    } else {
                        Lang::ChineseSimplified
                    }
                }
                "nl" => Lang::Dutch,
                "fr" => Lang::French,
                "de" => Lang::German,
                "it" => Lang::Italian,
                "ja" => Lang::Japanese,
                "ko" => Lang::Korean,
                "ru" => Lang::Russian,
                "es" => Lang::Spanish,
                _ => Lang::English,
            }
        }
        #[cfg(not(feature = "localization"))]
        {
            Lang::English
        }
    }

    /// Parses the language name and returns the corresponding language.
    pub fn from_name(name: &str) -> Self {
        match name {
            "English" => Lang::English,
            #[cfg(feature = "localization")]
            "Português (Brasil)" => Lang::BrazilianPortuguese,
            #[cfg(feature = "localization")]
            "简体中文" => Lang::ChineseSimplified,
            #[cfg(feature = "localization")]
            "繁體中文" => Lang::ChineseTraditional,
            #[cfg(feature = "localization")]
            "Nederlands" => Lang::Dutch,
            #[cfg(feature = "localization")]
            "Français" => Lang::French,
            #[cfg(feature = "localization")]
            "Deutsch" => Lang::German,
            #[cfg(feature = "localization")]
            "Italiano" => Lang::Italian,
            #[cfg(feature = "localization")]
            "日本語" => Lang::Japanese,
            #[cfg(feature = "localization")]
            "한국어" => Lang::Korean,
            #[cfg(feature = "localization")]
            "Português" => Lang::Portuguese,
            #[cfg(feature = "localization")]
            "Polski" => Lang::Polish,
            #[cfg(feature = "localization")]
            "Русский" => Lang::Russian,
            #[cfg(feature = "localization")]
            "Español" => Lang::Spanish,
            _ => Lang::English,
        }
    }

    /// Returns the localized display name for this language.
    pub const fn name(&self) -> &'static str {
        match self {
            Lang::English => "English",
            #[cfg(feature = "localization")]
            Lang::BrazilianPortuguese => "Português (Brasil)",
            #[cfg(feature = "localization")]
            Lang::ChineseSimplified => "简体中文",
            #[cfg(feature = "localization")]
            Lang::ChineseTraditional => "繁體中文",
            #[cfg(feature = "localization")]
            Lang::Dutch => "Nederlands",
            #[cfg(feature = "localization")]
            Lang::French => "Français",
            #[cfg(feature = "localization")]
            Lang::German => "Deutsch",
            #[cfg(feature = "localization")]
            Lang::Italian => "Italiano",
            #[cfg(feature = "localization")]
            Lang::Japanese => "日本語",
            #[cfg(feature = "localization")]
            Lang::Korean => "한국어",
            #[cfg(feature = "localization")]
            Lang::Portuguese => "Português",
            #[cfg(feature = "localization")]
            Lang::Polish => "Polski",
            #[cfg(feature = "localization")]
            Lang::Russian => "Русский",
            #[cfg(feature = "localization")]
            Lang::Spanish => "Español",
        }
    }

    pub(crate) const fn decimal_separator(self) -> AsciiChar {
        match self {
            Lang::English => AsciiChar::DOT,
            #[cfg(feature = "localization")]
            Lang::German => AsciiChar::COMMA,
            #[cfg(feature = "localization")]
            Lang::French => AsciiChar::COMMA,
            #[cfg(feature = "localization")]
            Lang::Spanish => AsciiChar::COMMA,
            #[cfg(feature = "localization")]
            Lang::Italian => AsciiChar::COMMA,
            #[cfg(feature = "localization")]
            Lang::Portuguese => AsciiChar::COMMA,
            #[cfg(feature = "localization")]
            Lang::Polish => AsciiChar::COMMA,
            #[cfg(feature = "localization")]
            Lang::BrazilianPortuguese => AsciiChar::COMMA,
            #[cfg(feature = "localization")]
            Lang::Dutch => AsciiChar::COMMA,
            #[cfg(feature = "localization")]
            Lang::Russian => AsciiChar::COMMA,
            #[cfg(feature = "localization")]
            Lang::ChineseSimplified => AsciiChar::DOT,
            #[cfg(feature = "localization")]
            Lang::ChineseTraditional => AsciiChar::DOT,
            #[cfg(feature = "localization")]
            Lang::Japanese => AsciiChar::DOT,
            #[cfg(feature = "localization")]
            Lang::Korean => AsciiChar::DOT,
        }
    }
}

static MONTHS_EN: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

#[cfg(feature = "localization")]
static MONTHS_DE: [&str; 12] = [
    "Januar",
    "Februar",
    "März",
    "April",
    "Mai",
    "Juni",
    "Juli",
    "August",
    "September",
    "Oktober",
    "November",
    "Dezember",
];

#[cfg(feature = "localization")]
static MONTHS_FR: [&str; 12] = [
    "janvier",
    "février",
    "mars",
    "avril",
    "mai",
    "juin",
    "juillet",
    "août",
    "septembre",
    "octobre",
    "novembre",
    "décembre",
];

#[cfg(feature = "localization")]
static MONTHS_ES: [&str; 12] = [
    "enero",
    "febrero",
    "marzo",
    "abril",
    "mayo",
    "junio",
    "julio",
    "agosto",
    "septiembre",
    "octubre",
    "noviembre",
    "diciembre",
];

#[cfg(feature = "localization")]
static MONTHS_IT: [&str; 12] = [
    "gennaio",
    "febbraio",
    "marzo",
    "aprile",
    "maggio",
    "giugno",
    "luglio",
    "agosto",
    "settembre",
    "ottobre",
    "novembre",
    "dicembre",
];

#[cfg(feature = "localization")]
static MONTHS_PT: [&str; 12] = [
    "janeiro",
    "fevereiro",
    "março",
    "abril",
    "maio",
    "junho",
    "julho",
    "agosto",
    "setembro",
    "outubro",
    "novembro",
    "dezembro",
];

#[cfg(feature = "localization")]
static MONTHS_NL: [&str; 12] = [
    "januari",
    "februari",
    "maart",
    "april",
    "mei",
    "juni",
    "juli",
    "augustus",
    "september",
    "oktober",
    "november",
    "december",
];

#[cfg(feature = "localization")]
static MONTHS_RU: [&str; 12] = [
    "января",
    "февраля",
    "марта",
    "апреля",
    "мая",
    "июня",
    "июля",
    "августа",
    "сентября",
    "октября",
    "ноября",
    "декабря",
];

#[cfg(feature = "localization")]
static MONTHS_PL: [&str; 12] = [
    "styczeń",
    "luty",
    "marzec",
    "kwiecień",
    "maj",
    "czerwiec",
    "lipiec",
    "sierpień",
    "wrzesień",
    "październik",
    "listopad",
    "grudzień",
];

const fn month_name(lang: Lang, month: u8) -> &'static str {
    let index = month.saturating_sub(1) as usize;
    if index >= 12 {
        return "";
    }

    match lang {
        Lang::English => MONTHS_EN[index],
        #[cfg(feature = "localization")]
        Lang::German => MONTHS_DE[index],
        #[cfg(feature = "localization")]
        Lang::French => MONTHS_FR[index],
        #[cfg(feature = "localization")]
        Lang::Spanish => MONTHS_ES[index],
        #[cfg(feature = "localization")]
        Lang::Italian => MONTHS_IT[index],
        #[cfg(feature = "localization")]
        Lang::Portuguese => MONTHS_PT[index],
        #[cfg(feature = "localization")]
        Lang::BrazilianPortuguese => MONTHS_PT[index],
        #[cfg(feature = "localization")]
        Lang::Dutch => MONTHS_NL[index],
        #[cfg(feature = "localization")]
        Lang::Russian => MONTHS_RU[index],
        #[cfg(feature = "localization")]
        Lang::Polish => MONTHS_PL[index],
        #[cfg(feature = "localization")]
        Lang::ChineseSimplified | Lang::ChineseTraditional | Lang::Japanese | Lang::Korean => "",
    }
}

/// Formats a localized date string matching JavaScript's `toLocaleDateString`
/// with `{ year: "numeric", month: "long", day: "numeric" }`.
pub(crate) const fn localize_date(lang: Lang, year: i32, month: u8, day: u8) -> Date {
    Date {
        lang,
        year,
        month,
        day,
    }
}

/// A localized date formatter that avoids allocations.
pub(crate) struct Date {
    lang: Lang,
    year: i32,
    month: u8,
    day: u8,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.lang {
            Lang::English => {
                write!(
                    f,
                    "{} {}, {}",
                    month_name(self.lang, self.month),
                    self.day,
                    self.year
                )
            }
            #[cfg(feature = "localization")]
            Lang::German => write!(
                f,
                "{}. {} {}",
                self.day,
                month_name(self.lang, self.month),
                self.year
            ),
            #[cfg(feature = "localization")]
            Lang::French => write!(
                f,
                "{} {} {}",
                self.day,
                month_name(self.lang, self.month),
                self.year
            ),
            #[cfg(feature = "localization")]
            Lang::Spanish => write!(
                f,
                "{} de {} de {}",
                self.day,
                month_name(self.lang, self.month),
                self.year
            ),
            #[cfg(feature = "localization")]
            Lang::Italian => write!(
                f,
                "{} {} {}",
                self.day,
                month_name(self.lang, self.month),
                self.year
            ),
            #[cfg(feature = "localization")]
            Lang::Portuguese | Lang::BrazilianPortuguese => write!(
                f,
                "{} de {} de {}",
                self.day,
                month_name(self.lang, self.month),
                self.year
            ),
            #[cfg(feature = "localization")]
            Lang::Dutch => write!(
                f,
                "{} {} {}",
                self.day,
                month_name(self.lang, self.month),
                self.year
            ),
            #[cfg(feature = "localization")]
            Lang::Russian => write!(
                f,
                "{} {} {} г.",
                self.day,
                month_name(self.lang, self.month),
                self.year
            ),
            #[cfg(feature = "localization")]
            Lang::Polish => write!(
                f,
                "{} {} {}",
                self.day,
                month_name(self.lang, self.month),
                self.year
            ),
            #[cfg(feature = "localization")]
            Lang::ChineseSimplified | Lang::ChineseTraditional | Lang::Japanese => {
                write!(f, "{}年{}月{}日", self.year, self.month, self.day)
            }
            #[cfg(feature = "localization")]
            Lang::Korean => write!(f, "{}년 {}월 {}일", self.year, self.month, self.day),
        }
    }
}

/// Formats a localized time string matching JavaScript's `toLocaleTimeString`
/// defaults for the given language.
pub(crate) const fn localize_time_of_day(
    lang: Lang,
    hour: u8,
    minute: u8,
    time_zone: &'static str,
) -> TimeOfDay {
    TimeOfDay {
        lang,
        hour,
        minute,
        time_zone,
    }
}

/// A localized time formatter that avoids allocations.
pub(crate) struct TimeOfDay {
    lang: Lang,
    hour: u8,
    minute: u8,
    time_zone: &'static str,
}

impl fmt::Display for TimeOfDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.lang {
            Lang::English => {
                let (hour, am_pm) = if self.hour >= 12 {
                    (self.hour - 12, "PM")
                } else {
                    (self.hour, "AM")
                };
                let hour = if hour == 0 { 12 } else { hour };
                write!(f, "{hour}:{:02} {am_pm}{}", self.minute, self.time_zone)
            }
            #[cfg(feature = "localization")]
            Lang::Korean => {
                let (hour, am_pm) = if self.hour >= 12 {
                    (self.hour - 12, "오후")
                } else {
                    (self.hour, "오전")
                };
                let hour = if hour == 0 { 12 } else { hour };
                write!(f, "{am_pm} {hour}시 {}분{}", self.minute, self.time_zone)
            }
            #[cfg(feature = "localization")]
            Lang::Japanese => {
                let (hour, am_pm) = if self.hour >= 12 {
                    (self.hour - 12, "午後")
                } else {
                    (self.hour, "午前")
                };
                let hour = if hour == 0 { 12 } else { hour };
                write!(f, "{am_pm}{hour}:{:02}{}", self.minute, self.time_zone)
            }
            #[cfg(feature = "localization")]
            Lang::ChineseSimplified | Lang::ChineseTraditional => {
                let (hour, am_pm) = if self.hour >= 12 {
                    (self.hour - 12, "下午")
                } else {
                    (self.hour, "上午")
                };
                let hour = if hour == 0 { 12 } else { hour };
                write!(f, "{am_pm}{hour}:{:02}{}", self.minute, self.time_zone)
            }
            #[cfg(feature = "localization")]
            _ => write!(f, "{:02}:{:02}{}", self.hour, self.minute, self.time_zone),
        }
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Copy, Clone)]
pub(crate) enum Text {
    StartSplit,
    StartSplitDescription,
    Reset,
    ResetDescription,
    UndoSplit,
    UndoSplitDescription,
    SkipSplit,
    SkipSplitDescription,
    Pause,
    PauseDescription,
    UndoAllPauses,
    UndoAllPausesDescription,
    PreviousComparison,
    PreviousComparisonDescription,
    NextComparison,
    NextComparisonDescription,
    ToggleTimingMethod,
    ToggleTimingMethodDescription,
    TimerBackground,
    TimerBackgroundDescription,
    SegmentTimer,
    SegmentTimerDescription,
    TimingMethod,
    TimingMethodDescription,
    Height,
    HeightDescription,
    TimerTextColor,
    TimerTextColorDescription,
    ShowGradient,
    ShowGradientDescription,
    DigitsFormat,
    DigitsFormatDescription,
    Accuracy,
    AccuracyDescription,
    TitleBackground,
    TitleBackgroundDescription,
    TitleTextColor,
    TitleTextColorDescription,
    ShowGameName,
    ShowGameNameDescription,
    ShowCategoryName,
    ShowCategoryNameDescription,
    ShowFinishedRunsCount,
    ShowFinishedRunsCountDescription,
    ShowAttemptCount,
    ShowAttemptCountDescription,
    TextAlignment,
    TextAlignmentDescription,
    DisplayTextAsSingleLine,
    DisplayTextAsSingleLineDescription,
    DisplayGameIcon,
    DisplayGameIconDescription,
    ShowRegion,
    ShowRegionDescription,
    ShowPlatform,
    ShowPlatformDescription,
    ShowVariables,
    ShowVariablesDescription,
    TotalPlaytimeBackground,
    TotalPlaytimeBackgroundDescription,
    DisplayTwoRows,
    DisplayTwoRowsDescription,
    ShowDays,
    ShowDaysDescription,
    LabelColor,
    LabelColorDescription,
    ValueColor,
    ValueColorDescription,
    BlankSpaceBackground,
    BlankSpaceBackgroundDescription,
    BlankSpaceSize,
    BlankSpaceSizeDescription,
    CurrentComparisonBackground,
    CurrentComparisonBackgroundDescription,
    CurrentComparisonDisplayTwoRows,
    CurrentComparisonDisplayTwoRowsDescription,
    CurrentComparisonLabelColor,
    CurrentComparisonLabelColorDescription,
    CurrentComparisonValueColor,
    CurrentComparisonValueColorDescription,
    CurrentPaceBackground,
    CurrentPaceBackgroundDescription,
    CurrentPaceComparison,
    CurrentPaceComparisonDescription,
    CurrentPaceDisplayTwoRows,
    CurrentPaceDisplayTwoRowsDescription,
    CurrentPaceLabelColor,
    CurrentPaceLabelColorDescription,
    CurrentPaceValueColor,
    CurrentPaceValueColorDescription,
    CurrentPaceAccuracy,
    CurrentPaceAccuracyDescription,
    DeltaBackground,
    DeltaBackgroundDescription,
    DeltaComparison,
    DeltaComparisonDescription,
    DeltaDisplayTwoRows,
    DeltaDisplayTwoRowsDescription,
    DeltaLabelColor,
    DeltaLabelColorDescription,
    DeltaDropDecimals,
    DeltaDropDecimalsDescription,
    DeltaAccuracy,
    DeltaAccuracyDescription,
    SumOfBestBackground,
    SumOfBestBackgroundDescription,
    SumOfBestDisplayTwoRows,
    SumOfBestDisplayTwoRowsDescription,
    SumOfBestLabelColor,
    SumOfBestLabelColorDescription,
    SumOfBestValueColor,
    SumOfBestValueColorDescription,
    SumOfBestAccuracy,
    SumOfBestAccuracyDescription,
    PbChanceBackground,
    PbChanceBackgroundDescription,
    PbChanceDisplayTwoRows,
    PbChanceDisplayTwoRowsDescription,
    PbChanceLabelColor,
    PbChanceLabelColorDescription,
    PbChanceValueColor,
    PbChanceValueColorDescription,
    PossibleTimeSaveBackground,
    PossibleTimeSaveBackgroundDescription,
    PossibleTimeSaveComparison,
    PossibleTimeSaveComparisonDescription,
    PossibleTimeSaveDisplayTwoRows,
    PossibleTimeSaveDisplayTwoRowsDescription,
    PossibleTimeSaveShowTotal,
    PossibleTimeSaveShowTotalDescription,
    PossibleTimeSaveLabelColor,
    PossibleTimeSaveLabelColorDescription,
    PossibleTimeSaveValueColor,
    PossibleTimeSaveValueColorDescription,
    PossibleTimeSaveAccuracy,
    PossibleTimeSaveAccuracyDescription,
    PreviousSegmentBackground,
    PreviousSegmentBackgroundDescription,
    PreviousSegmentComparison,
    PreviousSegmentComparisonDescription,
    PreviousSegmentDisplayTwoRows,
    PreviousSegmentDisplayTwoRowsDescription,
    PreviousSegmentLabelColor,
    PreviousSegmentLabelColorDescription,
    PreviousSegmentDropDecimals,
    PreviousSegmentDropDecimalsDescription,
    PreviousSegmentAccuracy,
    PreviousSegmentAccuracyDescription,
    PreviousSegmentShowPossibleTimeSave,
    PreviousSegmentShowPossibleTimeSaveDescription,
    SegmentTimeBackground,
    SegmentTimeBackgroundDescription,
    SegmentTimeComparison,
    SegmentTimeComparisonDescription,
    SegmentTimeDisplayTwoRows,
    SegmentTimeDisplayTwoRowsDescription,
    SegmentTimeLabelColor,
    SegmentTimeLabelColorDescription,
    SegmentTimeValueColor,
    SegmentTimeValueColorDescription,
    SegmentTimeAccuracy,
    SegmentTimeAccuracyDescription,
    GraphComparison,
    GraphComparisonDescription,
    GraphHeight,
    GraphHeightDescription,
    GraphShowBestSegments,
    GraphShowBestSegmentsDescription,
    GraphLiveGraph,
    GraphLiveGraphDescription,
    GraphFlipGraph,
    GraphFlipGraphDescription,
    GraphBehindBackgroundColor,
    GraphBehindBackgroundColorDescription,
    GraphAheadBackgroundColor,
    GraphAheadBackgroundColorDescription,
    GraphGridLinesColor,
    GraphGridLinesColorDescription,
    GraphLinesColor,
    GraphLinesColorDescription,
    GraphPartialFillColor,
    GraphPartialFillColorDescription,
    GraphCompleteFillColor,
    GraphCompleteFillColorDescription,
    DetailedTimerBackground,
    DetailedTimerBackgroundDescription,
    DetailedTimerTimingMethod,
    DetailedTimerTimingMethodDescription,
    DetailedTimerComparison1,
    DetailedTimerComparison1Description,
    DetailedTimerComparison2,
    DetailedTimerComparison2Description,
    DetailedTimerHideSecondComparison,
    DetailedTimerHideSecondComparisonDescription,
    DetailedTimerTimerHeight,
    DetailedTimerTimerHeightDescription,
    DetailedTimerSegmentTimerHeight,
    DetailedTimerSegmentTimerHeightDescription,
    DetailedTimerTimerColor,
    DetailedTimerTimerColorDescription,
    DetailedTimerShowTimerGradient,
    DetailedTimerShowTimerGradientDescription,
    DetailedTimerTimerDigitsFormat,
    DetailedTimerTimerDigitsFormatDescription,
    DetailedTimerTimerAccuracy,
    DetailedTimerTimerAccuracyDescription,
    DetailedTimerSegmentTimerColor,
    DetailedTimerSegmentTimerColorDescription,
    DetailedTimerShowSegmentTimerGradient,
    DetailedTimerShowSegmentTimerGradientDescription,
    DetailedTimerSegmentTimerDigitsFormat,
    DetailedTimerSegmentTimerDigitsFormatDescription,
    DetailedTimerSegmentTimerAccuracy,
    DetailedTimerSegmentTimerAccuracyDescription,
    DetailedTimerComparisonNamesColor,
    DetailedTimerComparisonNamesColorDescription,
    DetailedTimerComparisonTimesColor,
    DetailedTimerComparisonTimesColorDescription,
    DetailedTimerComparisonTimesAccuracy,
    DetailedTimerComparisonTimesAccuracyDescription,
    DetailedTimerShowSegmentName,
    DetailedTimerShowSegmentNameDescription,
    DetailedTimerSegmentNameColor,
    DetailedTimerSegmentNameColorDescription,
    DetailedTimerDisplayIcon,
    DetailedTimerDisplayIconDescription,
    SplitsBackground,
    SplitsBackgroundDescription,
    SplitsTotalRows,
    SplitsTotalRowsDescription,
    SplitsUpcomingSegments,
    SplitsUpcomingSegmentsDescription,
    SplitsShowThinSeparators,
    SplitsShowThinSeparatorsDescription,
    SplitsShowSeparatorBeforeLastSplit,
    SplitsShowSeparatorBeforeLastSplitDescription,
    SplitsAlwaysShowLastSplit,
    SplitsAlwaysShowLastSplitDescription,
    SplitsFillWithBlankSpace,
    SplitsFillWithBlankSpaceDescription,
    SplitsShowTimesBelowSegmentName,
    SplitsShowTimesBelowSegmentNameDescription,
    SplitsCurrentSegmentGradient,
    SplitsCurrentSegmentGradientDescription,
    SplitsSplitTimeAccuracy,
    SplitsSplitTimeAccuracyDescription,
    SplitsSegmentTimeAccuracy,
    SplitsSegmentTimeAccuracyDescription,
    SplitsDeltaTimeAccuracy,
    SplitsDeltaTimeAccuracyDescription,
    SplitsDropDeltaDecimals,
    SplitsDropDeltaDecimalsDescription,
    SplitsShowColumnLabels,
    SplitsShowColumnLabelsDescription,
    SplitsColumns,
    SplitsColumnsDescription,
    SplitsColumnName,
    SplitsColumnNameDescription,
    SplitsColumnType,
    SplitsColumnTypeDescription,
    SplitsVariableName,
    SplitsVariableNameDescription,
    SplitsStartWith,
    SplitsStartWithDescription,
    SplitsUpdateWith,
    SplitsUpdateWithDescription,
    SplitsUpdateTrigger,
    SplitsUpdateTriggerDescription,
    SplitsColumnComparison,
    SplitsColumnComparisonDescription,
    SplitsColumnTimingMethod,
    SplitsColumnTimingMethodDescription,
    TextComponentBackground,
    TextComponentBackgroundDescription,
    TextComponentUseVariable,
    TextComponentUseVariableDescription,
    TextComponentSplit,
    TextComponentSplitDescription,
    TextComponentText,
    TextComponentTextDescription,
    TextComponentLeft,
    TextComponentLeftDescription,
    TextComponentRight,
    TextComponentRightDescription,
    TextComponentVariable,
    TextComponentVariableDescription,
    TextComponentTextColor,
    TextComponentTextColorDescription,
    TextComponentLeftColor,
    TextComponentLeftColorDescription,
    TextComponentRightColor,
    TextComponentRightColorDescription,
    TextComponentNameColor,
    TextComponentNameColorDescription,
    TextComponentValueColor,
    TextComponentValueColorDescription,
    TextComponentDisplayTwoRows,
    TextComponentDisplayTwoRowsDescription,
    LayoutDirection,
    LayoutDirectionDescription,
    CustomTimerFont,
    CustomTimerFontDescription,
    CustomTimesFont,
    CustomTimesFontDescription,
    CustomTextFont,
    CustomTextFontDescription,
    TextShadow,
    TextShadowDescription,
    Background,
    BackgroundDescription,
    BestSegment,
    BestSegmentDescription,
    AheadGainingTime,
    AheadGainingTimeDescription,
    AheadLosingTime,
    AheadLosingTimeDescription,
    BehindGainingTime,
    BehindGainingTimeDescription,
    BehindLosingTime,
    BehindLosingTimeDescription,
    NotRunning,
    NotRunningDescription,
    PersonalBest,
    PersonalBestDescription,
    Paused,
    PausedDescription,
    ThinSeparators,
    ThinSeparatorsDescription,
    Separators,
    SeparatorsDescription,
    TextColor,
    TextColorDescription,
    ComponentBlankSpace,
    ComponentCurrentComparison,
    ComponentCurrentPace,
    ComponentDelta,
    ComponentDetailedTimer,
    ComponentGraph,
    ComponentPbChance,
    ComponentPossibleTimeSave,
    ComponentPreviousSegment,
    ComponentSegmentTime,
    ComponentSeparator,
    ComponentSplits,
    ComponentSumOfBest,
    ComponentText,
    ComponentTimer,
    ComponentSegmentTimer,
    ComponentTitle,
    ComponentTotalPlaytime,
    ComponentCurrentPaceBestPossibleTime,
    ComponentCurrentPaceWorstPossibleTime,
    ComponentCurrentPacePredictedTime,
    ComponentSegmentTimeBest,
    ComponentSegmentTimeWorst,
    ComponentSegmentTimeAverage,
    ComponentSegmentTimeMedian,
    ComponentSegmentTimeLatest,
    ComponentPossibleTimeSaveTotal,
    LiveSegment,
    LiveSegmentShort,
    PreviousSegmentShort,
    PreviousSegmentAbbreviation,
    ComparingAgainst,
    ComparisonShort,
    CurrentPaceBestPossibleTimeShort,
    CurrentPaceBestTimeShort,
    CurrentPaceBestPossibleTimeAbbreviation,
    CurrentPaceWorstPossibleTimeShort,
    CurrentPaceWorstTimeShort,
    CurrentPacePredictedTimeShort,
    CurrentPaceShort,
    CurrentPaceAbbreviation,
    Goal,
    SumOfBestSegments,
    SumOfBestShort,
    SumOfBestAbbreviation,
    PlaytimeShort,
    BestSegmentTimeShort,
    BestSegmentShort,
    WorstSegmentTimeShort,
    WorstSegmentShort,
    AverageSegmentTimeShort,
    AverageSegmentShort,
    MedianSegmentTimeShort,
    MedianSegmentShort,
    LatestSegmentTimeShort,
    LatestSegmentShort,
    SegmentTimeShort,
    PossibleTimeSaveShort,
    PossibleTimeSaveAbbreviation,
    TimeSaveShort,
    RealTime,
    GameTime,
    SumOfBestCleanerStartOfRun,
    SumOfBestCleanerShouldRemove,
}

/// Text with placeholders for dynamic formatting.
#[allow(clippy::enum_variant_names)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum PlaceholderText {
    SumOfBestCleanerSegmentTimeBetween,
    SumOfBestCleanerFasterThanCombined,
    SumOfBestCleanerRunOn,
}

/// A resolved placeholder text that can be formatted without allocations.
pub struct DynamicResolve<'a> {
    static_parts: &'static [Piece],
    params: &'a [&'a dyn fmt::Display],
}

enum Piece {
    Static(&'static str),
    Dynamic(u8),
}

impl fmt::Display for DynamicResolve<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for piece in self.static_parts {
            match piece {
                Piece::Static(part) => f.write_str(part)?,
                Piece::Dynamic(params) => {
                    let param = self.params.get(*params as usize).ok_or(fmt::Error)?;
                    fmt::Display::fmt(param, f)?;
                }
            }
        }
        Ok(())
    }
}

impl Text {
    pub(crate) const fn resolve(self, lang: Lang) -> &'static str {
        match lang {
            Lang::English => english::resolve(self),
            #[cfg(feature = "localization")]
            Lang::BrazilianPortuguese => brazilian_portuguese::resolve(self),
            #[cfg(feature = "localization")]
            Lang::ChineseSimplified => chinese_simplified::resolve(self),
            #[cfg(feature = "localization")]
            Lang::ChineseTraditional => chinese_traditional::resolve(self),
            #[cfg(feature = "localization")]
            Lang::Polish => polish::resolve(self),
            #[cfg(feature = "localization")]
            Lang::Dutch => dutch::resolve(self),
            #[cfg(feature = "localization")]
            Lang::French => french::resolve(self),
            #[cfg(feature = "localization")]
            Lang::German => german::resolve(self),
            #[cfg(feature = "localization")]
            Lang::Italian => italian::resolve(self),
            #[cfg(feature = "localization")]
            Lang::Japanese => japanese::resolve(self),
            #[cfg(feature = "localization")]
            Lang::Korean => korean::resolve(self),
            #[cfg(feature = "localization")]
            Lang::Portuguese => portuguese::resolve(self),
            #[cfg(feature = "localization")]
            Lang::Russian => russian::resolve(self),
            #[cfg(feature = "localization")]
            Lang::Spanish => spanish::resolve(self),
        }
    }
}

impl PlaceholderText {
    pub(crate) fn resolve<'a>(
        self,
        lang: Lang,
        params: &'a [&'a dyn fmt::Display],
    ) -> DynamicResolve<'a> {
        let static_parts = match lang {
            Lang::English => english::resolve_placeholder(self),
            #[cfg(feature = "localization")]
            Lang::BrazilianPortuguese => brazilian_portuguese::resolve_placeholder(self),
            #[cfg(feature = "localization")]
            Lang::ChineseSimplified => chinese_simplified::resolve_placeholder(self),
            #[cfg(feature = "localization")]
            Lang::ChineseTraditional => chinese_traditional::resolve_placeholder(self),
            #[cfg(feature = "localization")]
            Lang::Polish => polish::resolve_placeholder(self),
            #[cfg(feature = "localization")]
            Lang::Dutch => dutch::resolve_placeholder(self),
            #[cfg(feature = "localization")]
            Lang::French => french::resolve_placeholder(self),
            #[cfg(feature = "localization")]
            Lang::German => german::resolve_placeholder(self),
            #[cfg(feature = "localization")]
            Lang::Italian => italian::resolve_placeholder(self),
            #[cfg(feature = "localization")]
            Lang::Japanese => japanese::resolve_placeholder(self),
            #[cfg(feature = "localization")]
            Lang::Korean => korean::resolve_placeholder(self),
            #[cfg(feature = "localization")]
            Lang::Portuguese => portuguese::resolve_placeholder(self),
            #[cfg(feature = "localization")]
            Lang::Russian => russian::resolve_placeholder(self),
            #[cfg(feature = "localization")]
            Lang::Spanish => spanish::resolve_placeholder(self),
        };

        DynamicResolve {
            static_parts,
            params,
        }
    }
}
