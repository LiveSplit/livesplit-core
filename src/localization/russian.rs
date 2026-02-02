use super::{Piece, PlaceholderText, Text};

pub const fn resolve(text: Text) -> &'static str {
    match text {
        Text::StartSplit => "Старт / Сплит",
        Text::StartSplitDescription => "Горячая клавиша для сплита и начала новой попытки.",
        Text::Reset => "Сброс",
        Text::ResetDescription => "Горячая клавиша для сброса текущей попытки.",
        Text::UndoSplit => "Отменить сплит",
        Text::UndoSplitDescription => "Горячая клавиша для отмены последнего сплита.",
        Text::SkipSplit => "Пропустить сплит",
        Text::SkipSplitDescription => "Горячая клавиша для пропуска текущего сплита.",
        Text::Pause => "Пауза",
        Text::PauseDescription => {
            "Горячая клавиша для паузы текущей попытки. Также может начать новую попытку."
        }
        Text::UndoAllPauses => "Отменить все паузы",
        Text::UndoAllPausesDescription => {
            "Горячая клавиша для удаления всего времени паузы из текущего времени. Полезно, если пауза была нажата случайно."
        }
        Text::PreviousComparison => "Предыдущее сравнение",
        Text::PreviousComparisonDescription => {
            "Горячая клавиша для переключения на предыдущее сравнение."
        }
        Text::NextComparison => "Следующее сравнение",
        Text::NextComparisonDescription => {
            "Горячая клавиша для переключения на следующее сравнение."
        }
        Text::ToggleTimingMethod => "Переключить метод тайминга",
        Text::ToggleTimingMethodDescription => {
            "Горячая клавиша для переключения между «Реальное время» и «Игровое время»."
        }
        Text::TimerBackground => "Фон",
        Text::TimerBackgroundDescription => {
            "Фон за компонентом. Также можно применить цвета отставания/опережения в качестве фона."
        }
        Text::SegmentTimer => "Таймер сегмента",
        Text::SegmentTimerDescription => {
            "Определяет, показывать ли время от начала сегмента или от начала попытки."
        }
        Text::TimingMethod => "Метод тайминга",
        Text::TimingMethodDescription => {
            "Определяет метод тайминга. Если не указано, используется текущий метод."
        }
        Text::Height => "Высота",
        Text::HeightDescription => "Высота таймера.",
        Text::TimerTextColor => "Цвет текста",
        Text::TimerTextColorDescription => {
            "Цвет отображаемого времени. Если не указан, выбирается автоматически в зависимости от прогресса. Эти цвета можно задать в общих настройках макета."
        }
        Text::ShowGradient => "Показывать градиент",
        Text::ShowGradientDescription => {
            "Определяет, следует ли отображать градиент вместо сплошного цвета."
        }
        Text::DigitsFormat => "Формат разрядов",
        Text::DigitsFormatDescription => {
            "Определяет количество разрядов. Если времени меньше, заполняется нулями."
        }
        Text::Accuracy => "Точность",
        Text::AccuracyDescription => "Точность отображаемого времени.",
        Text::TitleBackground => "Фон",
        Text::TitleBackgroundDescription => "Фон за компонентом.",
        Text::TitleTextColor => "Цвет текста",
        Text::TitleTextColorDescription => {
            "Цвет текста заголовка. Если не указан, используется цвет из макета."
        }
        Text::ShowGameName => "Показывать название игры",
        Text::ShowGameNameDescription => {
            "Определяет, следует ли показывать название игры в заголовке."
        }
        Text::ShowCategoryName => "Показывать название категории",
        Text::ShowCategoryNameDescription => {
            "Определяет, следует ли показывать название категории в заголовке."
        }
        Text::ShowFinishedRunsCount => "Показывать число завершённых забегов",
        Text::ShowFinishedRunsCountDescription => {
            "Определяет, следует ли показывать количество успешно завершённых забегов."
        }
        Text::ShowAttemptCount => "Показывать число попыток",
        Text::ShowAttemptCountDescription => {
            "Определяет, следует ли показывать общее количество попыток."
        }
        Text::TextAlignment => "Выравнивание текста",
        Text::TextAlignmentDescription => "Определяет выравнивание заголовка.",
        Text::DisplayTextAsSingleLine => "Показывать текст в одну строку",
        Text::DisplayTextAsSingleLineDescription => {
            "Определяет, показывать ли заголовок в одну строку вместо двух строк (игра и категория)."
        }
        Text::DisplayGameIcon => "Показывать иконку игры",
        Text::DisplayGameIconDescription => {
            "Если в сплитах есть иконка игры, определяет, показывать ли её."
        }
        Text::ShowRegion => "Показывать регион",
        Text::ShowRegionDescription => {
            "Категория может быть дополнена. Если в редакторе сплитов на вкладке переменных указан регион, он добавляется."
        }
        Text::ShowPlatform => "Показывать платформу",
        Text::ShowPlatformDescription => {
            "Категория может быть дополнена. Если в редакторе сплитов на вкладке переменных указана платформа, она добавляется."
        }
        Text::ShowVariables => "Показывать переменные",
        Text::ShowVariablesDescription => {
            "Категория может быть дополнена переменными с вкладки переменных редактора сплитов. Это переменные speedrun.com, а не пользовательские."
        }
        Text::TotalPlaytimeBackground => "Фон",
        Text::TotalPlaytimeBackgroundDescription => "Фон за компонентом.",
        Text::DisplayTwoRows => "Показывать в две строки",
        Text::DisplayTwoRowsDescription => {
            "Определяет, показывать ли название компонента и общее время игры в две строки."
        }
        Text::ShowDays => "Показывать дни (>24ч)",
        Text::ShowDaysDescription => {
            "При общем времени 24 часа и более определяет, показывать ли дни."
        }
        Text::LabelColor => "Цвет подписи",
        Text::LabelColorDescription => {
            "Цвет названия компонента. Если не указан, используется цвет из макета."
        }
        Text::ValueColor => "Цвет значения",
        Text::ValueColorDescription => {
            "Цвет общего времени игры. Если не указан, используется цвет из макета."
        }
        Text::BlankSpaceBackground => "Фон",
        Text::BlankSpaceBackgroundDescription => "Фон за компонентом.",
        Text::BlankSpaceSize => "Размер",
        Text::BlankSpaceSizeDescription => "Размер компонента.",
        Text::CurrentComparisonBackground => "Фон",
        Text::CurrentComparisonBackgroundDescription => "Фон за компонентом.",
        Text::CurrentComparisonDisplayTwoRows => "Показывать в две строки",
        Text::CurrentComparisonDisplayTwoRowsDescription => {
            "Определяет, показывать ли название компонента и сравнение в две строки."
        }
        Text::CurrentComparisonLabelColor => "Цвет подписи",
        Text::CurrentComparisonLabelColorDescription => {
            "Цвет названия компонента. Если не указан, используется цвет из макета."
        }
        Text::CurrentComparisonValueColor => "Цвет значения",
        Text::CurrentComparisonValueColorDescription => {
            "Цвет названия сравнения. Если не указан, используется цвет из макета."
        }
        Text::CurrentPaceBackground => "Фон",
        Text::CurrentPaceBackgroundDescription => "Фон за компонентом.",
        Text::CurrentPaceComparison => "Сравнение",
        Text::CurrentPaceComparisonDescription => {
            "Сравнение для прогнозирования итогового времени. Если не указано, используется текущее сравнение."
        }
        Text::CurrentPaceDisplayTwoRows => "Показывать в две строки",
        Text::CurrentPaceDisplayTwoRowsDescription => {
            "Определяет, показывать ли название компонента и прогноз времени в две строки."
        }
        Text::CurrentPaceLabelColor => "Цвет подписи",
        Text::CurrentPaceLabelColorDescription => {
            "Цвет названия компонента. Если не указан, используется цвет из макета."
        }
        Text::CurrentPaceValueColor => "Цвет значения",
        Text::CurrentPaceValueColorDescription => {
            "Цвет прогнозируемого времени. Если не указан, используется цвет из макета."
        }
        Text::CurrentPaceAccuracy => "Точность",
        Text::CurrentPaceAccuracyDescription => "Точность отображаемого прогнозируемого времени.",
        Text::DeltaBackground => "Фон",
        Text::DeltaBackgroundDescription => "Фон за компонентом.",
        Text::DeltaComparison => "Сравнение",
        Text::DeltaComparisonDescription => {
            "Сравнение для расчёта опережения/отставания. Если не указано, используется текущее сравнение."
        }
        Text::DeltaDisplayTwoRows => "Показывать в две строки",
        Text::DeltaDisplayTwoRowsDescription => {
            "Определяет, показывать ли название сравнения и дельту в две строки."
        }
        Text::DeltaLabelColor => "Цвет подписи",
        Text::DeltaLabelColorDescription => {
            "Цвет названия сравнения. Если не указан, используется цвет из макета."
        }
        Text::DeltaDropDecimals => "Не показывать десятые",
        Text::DeltaDropDecimalsDescription => {
            "Когда дельта превышает 1 минуту, определяет, скрывать ли дробную часть."
        }
        Text::DeltaAccuracy => "Точность",
        Text::DeltaAccuracyDescription => "Точность отображаемой дельты.",
        Text::SumOfBestBackground => "Фон",
        Text::SumOfBestBackgroundDescription => "Фон за компонентом.",
        Text::SumOfBestDisplayTwoRows => "Показывать в две строки",
        Text::SumOfBestDisplayTwoRowsDescription => {
            "Определяет, показывать ли название компонента и сумму лучших сегментов в две строки."
        }
        Text::SumOfBestLabelColor => "Цвет подписи",
        Text::SumOfBestLabelColorDescription => {
            "Цвет названия компонента. Если не указан, используется цвет из макета."
        }
        Text::SumOfBestValueColor => "Цвет значения",
        Text::SumOfBestValueColorDescription => {
            "Цвет суммы лучших сегментов. Если не указан, используется цвет из макета."
        }
        Text::SumOfBestAccuracy => "Точность",
        Text::SumOfBestAccuracyDescription => "Точность отображаемой суммы лучших сегментов.",
        Text::PbChanceBackground => "Фон",
        Text::PbChanceBackgroundDescription => "Фон за компонентом.",
        Text::PbChanceDisplayTwoRows => "Показывать в две строки",
        Text::PbChanceDisplayTwoRowsDescription => {
            "Определяет, показывать ли название компонента и шанс PB в две строки."
        }
        Text::PbChanceLabelColor => "Цвет подписи",
        Text::PbChanceLabelColorDescription => {
            "Цвет названия компонента. Если не указан, используется цвет из макета."
        }
        Text::PbChanceValueColor => "Цвет значения",
        Text::PbChanceValueColorDescription => {
            "Цвет шанса PB. Если не указан, используется цвет из макета."
        }
        Text::PossibleTimeSaveBackground => "Фон",
        Text::PossibleTimeSaveBackgroundDescription => "Фон за компонентом.",
        Text::PossibleTimeSaveComparison => "Сравнение",
        Text::PossibleTimeSaveComparisonDescription => {
            "Сравнение для расчёта возможной экономии времени. Если не указано, используется текущее сравнение."
        }
        Text::PossibleTimeSaveDisplayTwoRows => "Показывать в две строки",
        Text::PossibleTimeSaveDisplayTwoRowsDescription => {
            "Определяет, показывать ли название компонента и возможную экономию времени в две строки."
        }
        Text::PossibleTimeSaveShowTotal => "Показывать общую экономию",
        Text::PossibleTimeSaveShowTotalDescription => {
            "Определяет, показывать ли общую возможную экономию оставшейся части забега вместо текущего сегмента."
        }
        Text::PossibleTimeSaveLabelColor => "Цвет подписи",
        Text::PossibleTimeSaveLabelColorDescription => {
            "Цвет названия компонента. Если не указан, используется цвет из макета."
        }
        Text::PossibleTimeSaveValueColor => "Цвет значения",
        Text::PossibleTimeSaveValueColorDescription => {
            "Цвет возможной экономии времени. Если не указан, используется цвет из макета."
        }
        Text::PossibleTimeSaveAccuracy => "Точность",
        Text::PossibleTimeSaveAccuracyDescription => {
            "Точность отображаемой возможной экономии времени."
        }
        Text::PreviousSegmentBackground => "Фон",
        Text::PreviousSegmentBackgroundDescription => "Фон за компонентом.",
        Text::PreviousSegmentComparison => "Сравнение",
        Text::PreviousSegmentComparisonDescription => {
            "Сравнение для расчёта сэкономленного/потерянного времени в предыдущем сегменте. Если не указано, используется текущее сравнение."
        }
        Text::PreviousSegmentDisplayTwoRows => "Показывать в две строки",
        Text::PreviousSegmentDisplayTwoRowsDescription => {
            "Определяет, показывать ли название компонента и сэкономленное/потерянное время в две строки."
        }
        Text::PreviousSegmentLabelColor => "Цвет подписи",
        Text::PreviousSegmentLabelColorDescription => {
            "Цвет названия компонента. Если не указан, используется цвет из макета."
        }
        Text::PreviousSegmentDropDecimals => "Не показывать десятые",
        Text::PreviousSegmentDropDecimalsDescription => {
            "Когда отображаемое время превышает 1 минуту, определяет, скрывать ли дробную часть."
        }
        Text::PreviousSegmentAccuracy => "Точность",
        Text::PreviousSegmentAccuracyDescription => "Точность отображаемого времени.",
        Text::PreviousSegmentShowPossibleTimeSave => "Показывать возможную экономию",
        Text::PreviousSegmentShowPossibleTimeSaveDescription => {
            "Определяет, следует ли также показывать возможную экономию времени в предыдущем сегменте."
        }
        Text::SegmentTimeBackground => "Фон",
        Text::SegmentTimeBackgroundDescription => "Фон за компонентом.",
        Text::SegmentTimeComparison => "Сравнение",
        Text::SegmentTimeComparisonDescription => {
            "Сравнение, используемое для времени сегмента. Если не указано, используется текущее сравнение."
        }
        Text::SegmentTimeDisplayTwoRows => "Показывать в две строки",
        Text::SegmentTimeDisplayTwoRowsDescription => {
            "Определяет, показывать ли название компонента и время сегмента в две строки."
        }
        Text::SegmentTimeLabelColor => "Цвет подписи",
        Text::SegmentTimeLabelColorDescription => {
            "Цвет названия компонента. Если не указан, используется цвет из макета."
        }
        Text::SegmentTimeValueColor => "Цвет значения",
        Text::SegmentTimeValueColorDescription => {
            "Цвет времени сегмента. Если не указан, используется цвет из макета."
        }
        Text::SegmentTimeAccuracy => "Точность",
        Text::SegmentTimeAccuracyDescription => "Точность отображаемого времени сегмента.",
        Text::GraphComparison => "Сравнение",
        Text::GraphComparisonDescription => {
            "Сравнение, используемое графиком. Если не указано, используется текущее сравнение."
        }
        Text::GraphHeight => "Высота",
        Text::GraphHeightDescription => "Высота графика.",
        Text::GraphShowBestSegments => "Показывать лучшие сегменты",
        Text::GraphShowBestSegmentsDescription => {
            "Определяет, следует ли использовать цвет лучших сегментов для выделения лучших сегментов."
        }
        Text::GraphLiveGraph => "Живой график",
        Text::GraphLiveGraphDescription => {
            "Определяет, следует ли автоматически обновлять график. Если отключено, обновляется только при смене сегмента."
        }
        Text::GraphFlipGraph => "Отразить график",
        Text::GraphFlipGraphDescription => {
            "Определяет, нужно ли отражать график по вертикали. Без отражения опережение ниже оси, отставание выше; с отражением наоборот."
        }
        Text::GraphBehindBackgroundColor => "Цвет фона отставания",
        Text::GraphBehindBackgroundColorDescription => "Цвет фона зоны отставания на графике.",
        Text::GraphAheadBackgroundColor => "Цвет фона опережения",
        Text::GraphAheadBackgroundColorDescription => "Цвет фона зоны опережения на графике.",
        Text::GraphGridLinesColor => "Цвет линий сетки",
        Text::GraphGridLinesColorDescription => "Цвет линий сетки графика.",
        Text::GraphLinesColor => "Цвет линий",
        Text::GraphLinesColorDescription => "Цвет линий, соединяющих точки графика.",
        Text::GraphPartialFillColor => "Цвет частичной заливки",
        Text::GraphPartialFillColorDescription => {
            "Цвет заливки между осью X и графиком. Частичная заливка применяется только в живой зоне (от последнего сплита до текущего времени)."
        }
        Text::GraphCompleteFillColor => "Цвет полной заливки",
        Text::GraphCompleteFillColorDescription => {
            "Цвет заливки между осью X и графиком (исключая живую зону)."
        }
        Text::DetailedTimerBackground => "Фон",
        Text::DetailedTimerBackgroundDescription => "Фон за компонентом.",
        Text::DetailedTimerTimingMethod => "Метод тайминга",
        Text::DetailedTimerTimingMethodDescription => {
            "Определяет метод тайминга. Если не указано, используется текущий метод."
        }
        Text::DetailedTimerComparison1 => "Сравнение 1",
        Text::DetailedTimerComparison1Description => {
            "Первое сравнение, отображаемое для времени сегмента. Если не указано, используется текущее сравнение."
        }
        Text::DetailedTimerComparison2 => "Сравнение 2",
        Text::DetailedTimerComparison2Description => {
            "Второе сравнение, отображаемое для времени сегмента. Если не указано, используется текущее сравнение, кроме случая, когда сравнение 1 тоже None. Если второе сравнение скрыто, не отображается."
        }
        Text::DetailedTimerHideSecondComparison => "Скрыть второе сравнение",
        Text::DetailedTimerHideSecondComparisonDescription => {
            "Определяет, следует ли показывать только одно сравнение."
        }
        Text::DetailedTimerTimerHeight => "Высота таймера",
        Text::DetailedTimerTimerHeightDescription => "Высота основного таймера.",
        Text::DetailedTimerSegmentTimerHeight => "Высота таймера сегмента",
        Text::DetailedTimerSegmentTimerHeightDescription => "Высота таймера сегмента.",
        Text::DetailedTimerTimerColor => "Цвет таймера",
        Text::DetailedTimerTimerColorDescription => {
            "Можно указать фиксированный цвет основного таймера вместо автоматического выбора по прогрессу."
        }
        Text::DetailedTimerShowTimerGradient => "Показывать градиент таймера",
        Text::DetailedTimerShowTimerGradientDescription => {
            "Если включено, цвет основного таймера автоматически становится вертикальным градиентом; иначе используется сплошной цвет."
        }
        Text::DetailedTimerTimerDigitsFormat => "Формат разрядов таймера",
        Text::DetailedTimerTimerDigitsFormatDescription => {
            "Определяет количество разрядов основного таймера. Если времени меньше, заполняется нулями."
        }
        Text::DetailedTimerTimerAccuracy => "Точность таймера",
        Text::DetailedTimerTimerAccuracyDescription => {
            "Точность отображаемого времени основного таймера."
        }
        Text::DetailedTimerSegmentTimerColor => "Цвет таймера сегмента",
        Text::DetailedTimerSegmentTimerColorDescription => {
            "Позволяет задать цвет таймера сегмента, отличный от основного."
        }
        Text::DetailedTimerShowSegmentTimerGradient => "Показывать градиент таймера сегмента",
        Text::DetailedTimerShowSegmentTimerGradientDescription => {
            "Если включено, цвет таймера сегмента автоматически становится вертикальным градиентом; иначе используется сплошной цвет."
        }
        Text::DetailedTimerSegmentTimerDigitsFormat => "Формат разрядов таймера сегмента",
        Text::DetailedTimerSegmentTimerDigitsFormatDescription => {
            "Определяет количество разрядов таймера сегмента. Если времени меньше, заполняется нулями."
        }
        Text::DetailedTimerSegmentTimerAccuracy => "Точность таймера сегмента",
        Text::DetailedTimerSegmentTimerAccuracyDescription => {
            "Точность отображаемого времени таймера сегмента."
        }
        Text::DetailedTimerComparisonNamesColor => "Цвет названий сравнений",
        Text::DetailedTimerComparisonNamesColorDescription => {
            "Цвет отображаемых названий сравнений. Если не указан, используется цвет из макета."
        }
        Text::DetailedTimerComparisonTimesColor => "Цвет времени сравнений",
        Text::DetailedTimerComparisonTimesColorDescription => {
            "Цвет отображаемых времён сравнений. Если не указан, используется цвет из макета."
        }
        Text::DetailedTimerComparisonTimesAccuracy => "Точность времени сравнений",
        Text::DetailedTimerComparisonTimesAccuracyDescription => {
            "Точность отображаемого времени сравнений."
        }
        Text::DetailedTimerShowSegmentName => "Показывать название сегмента",
        Text::DetailedTimerShowSegmentNameDescription => {
            "Определяет, следует ли показывать название сегмента."
        }
        Text::DetailedTimerSegmentNameColor => "Цвет названия сегмента",
        Text::DetailedTimerSegmentNameColorDescription => {
            "Цвет отображаемого названия сегмента. Если не указан, используется цвет из макета."
        }
        Text::DetailedTimerDisplayIcon => "Показывать иконку",
        Text::DetailedTimerDisplayIconDescription => {
            "Определяет, следует ли показывать иконку сегмента."
        }
        Text::SplitsBackground => "Фон",
        Text::SplitsBackgroundDescription => {
            "Фон за компонентом. Можно выбрать чередующиеся цвета; при включении строки будут чередоваться между двумя цветами."
        }
        Text::SplitsTotalRows => "Всего строк",
        Text::SplitsTotalRowsDescription => {
            "Общее количество отображаемых строк. 0 показывает все сегменты. Если меньше общего количества сегментов, отображается прокручиваемое окно."
        }
        Text::SplitsUpcomingSegments => "Предстоящие сегменты",
        Text::SplitsUpcomingSegmentsDescription => {
            "Когда сегментов больше, чем строк, окно автоматически прокручивается вместе с текущим сегментом. Это значение определяет, сколько сегментов впереди должно оставаться в окне."
        }
        Text::SplitsShowThinSeparators => "Показывать тонкие разделители",
        Text::SplitsShowThinSeparatorsDescription => {
            "Определяет, следует ли показывать тонкие разделители между строками сегментов."
        }
        Text::SplitsShowSeparatorBeforeLastSplit => {
            "Показывать разделитель перед последним сегментом"
        }
        Text::SplitsShowSeparatorBeforeLastSplitDescription => {
            "Если последний сегмент всегда показывается, определяет, показывать ли более заметный разделитель перед ним, если он не рядом с предыдущим сегментом."
        }
        Text::SplitsAlwaysShowLastSplit => "Всегда показывать последний сегмент",
        Text::SplitsAlwaysShowLastSplitDescription => {
            "Если окно не показывает все сегменты, определяет, следует ли всегда показывать последний сегмент, так как он содержит общее время выбранного сравнения."
        }
        Text::SplitsFillWithBlankSpace => "Заполнить пустым пространством",
        Text::SplitsFillWithBlankSpaceDescription => {
            "Если сегментов меньше, чем строк, оставшиеся строки можно заполнить пустым пространством, чтобы всегда показывать заданное количество строк."
        }
        Text::SplitsShowTimesBelowSegmentName => "Показывать времена под названием сегмента",
        Text::SplitsShowTimesBelowSegmentNameDescription => {
            "Определяет, показывать ли времена под названием сегмента, а не рядом с ним."
        }
        Text::SplitsCurrentSegmentGradient => "Градиент текущего сегмента",
        Text::SplitsCurrentSegmentGradientDescription => {
            "Фоновый градиент для выделения текущего сегмента."
        }
        Text::SplitsSplitTimeAccuracy => "Точность общего времени",
        Text::SplitsSplitTimeAccuracyDescription => "Точность отображаемого общего времени.",
        Text::SplitsSegmentTimeAccuracy => "Точность времени сегмента",
        Text::SplitsSegmentTimeAccuracyDescription => "Точность отображаемого времени сегмента.",
        Text::SplitsDeltaTimeAccuracy => "Точность дельты",
        Text::SplitsDeltaTimeAccuracyDescription => "Точность отображаемой дельты.",
        Text::SplitsDropDeltaDecimals => "Не показывать десятые при отображении минут",
        Text::SplitsDropDeltaDecimalsDescription => {
            "Определяет, следует ли скрывать дробную часть в колонке дельты, если значение превышает 1 минуту."
        }
        Text::SplitsShowColumnLabels => "Показывать заголовки колонок",
        Text::SplitsShowColumnLabelsDescription => {
            "Определяет, следует ли показывать названия колонок вверху списка."
        }
        Text::SplitsColumns => "Колонки",
        Text::SplitsColumnsDescription => {
            "Количество колонок на строку. Каждая колонка может отображать разные данные. Колонки определяются справа налево."
        }
        Text::SplitsColumnName => "Название колонки",
        Text::SplitsColumnNameDescription => {
            "Название колонки. Если заголовки колонок включены, отображается вверху списка."
        }
        Text::SplitsColumnType => "Тип колонки",
        Text::SplitsColumnTypeDescription => {
            "Тип данных, отображаемых в колонке. Это может быть время или пользовательская переменная."
        }
        Text::SplitsVariableName => "Имя переменной",
        Text::SplitsVariableNameDescription => {
            "Имя пользовательской переменной, отображаемой в колонке."
        }
        Text::SplitsStartWith => "Начальное значение",
        Text::SplitsStartWithDescription => {
            "Значение в начале каждого сегмента. Заменяется на обновлённое значение при выполнении условий обновления."
        }
        Text::SplitsUpdateWith => "Обновлённое значение",
        Text::SplitsUpdateWithDescription => {
            "Значение, на которое обновляется время при выполнении условия (обычно текущий сегмент или завершённый)."
        }
        Text::SplitsUpdateTrigger => "Условие обновления",
        Text::SplitsUpdateTriggerDescription => {
            "Условие, при котором значение времени заменяется. Пока не выполнено, используется начальное значение."
        }
        Text::SplitsColumnComparison => "Сравнение",
        Text::SplitsColumnComparisonDescription => {
            "Сравнение для этой колонки. Если не указано, используется текущее сравнение."
        }
        Text::SplitsColumnTimingMethod => "Метод тайминга",
        Text::SplitsColumnTimingMethodDescription => {
            "Метод тайминга для этой колонки. Если не указан, используется текущий метод."
        }
        Text::TextComponentBackground => "Фон",
        Text::TextComponentBackgroundDescription => "Фон за компонентом.",
        Text::TextComponentUseVariable => "Использовать переменную",
        Text::TextComponentUseVariableDescription => {
            "Определяет, следует ли использовать пользовательскую переменную для отображения значения. Пользовательские переменные настраиваются в редакторе сплитов и могут предоставляться авто-сплиттером."
        }
        Text::TextComponentSplit => "Разделить",
        Text::TextComponentSplitDescription => {
            "Определяет, следует ли разделить текст на левую и правую части. Иначе отображается только центрированный текст."
        }
        Text::TextComponentText => "Текст",
        Text::TextComponentTextDescription => "Текст для отображения по центру.",
        Text::TextComponentLeft => "Слева",
        Text::TextComponentLeftDescription => "Текст для отображения слева.",
        Text::TextComponentRight => "Справа",
        Text::TextComponentRightDescription => "Текст для отображения справа.",
        Text::TextComponentVariable => "Переменная",
        Text::TextComponentVariableDescription => {
            "Имя пользовательской переменной для отображения."
        }
        Text::TextComponentTextColor => "Цвет текста",
        Text::TextComponentTextColorDescription => "Цвет текста.",
        Text::TextComponentLeftColor => "Цвет слева",
        Text::TextComponentLeftColorDescription => "Цвет текста слева.",
        Text::TextComponentRightColor => "Цвет справа",
        Text::TextComponentRightColorDescription => "Цвет текста справа.",
        Text::TextComponentNameColor => "Цвет названия",
        Text::TextComponentNameColorDescription => "Цвет названия переменной.",
        Text::TextComponentValueColor => "Цвет значения",
        Text::TextComponentValueColorDescription => "Цвет значения переменной.",
        Text::TextComponentDisplayTwoRows => "Показывать в две строки",
        Text::TextComponentDisplayTwoRowsDescription => {
            "Определяет, показывать ли левый и правый текст в две строки."
        }
        Text::LayoutDirection => "Направление макета",
        Text::LayoutDirectionDescription => "Направление расположения компонентов.",
        Text::CustomTimerFont => "Пользовательский шрифт таймера",
        Text::CustomTimerFontDescription => {
            "Позволяет указать пользовательский шрифт для таймера. Если не задан, используется шрифт по умолчанию."
        }
        Text::CustomTimesFont => "Пользовательский шрифт времени",
        Text::CustomTimesFontDescription => {
            "Позволяет указать пользовательский шрифт для времени. Если не задан, используется шрифт по умолчанию."
        }
        Text::CustomTextFont => "Пользовательский шрифт текста",
        Text::CustomTextFontDescription => {
            "Позволяет указать пользовательский шрифт для текста. Если не задан, используется шрифт по умолчанию."
        }
        Text::TextShadow => "Тень текста",
        Text::TextShadowDescription => "Позволяет при желании задать цвет тени текста.",
        Text::Background => "Фон",
        Text::BackgroundDescription => "Фон всего макета.",
        Text::BestSegment => "Лучший сегмент",
        Text::BestSegmentDescription => "Цвет, используемый при установке нового лучшего сегмента.",
        Text::AheadGainingTime => "Опережение (набирается)",
        Text::AheadGainingTimeDescription => {
            "Цвет, используемый при опережении и дальнейшем улучшении времени."
        }
        Text::AheadLosingTime => "Опережение (теряется)",
        Text::AheadLosingTimeDescription => "Цвет, используемый при опережении и потере времени.",
        Text::BehindGainingTime => "Отставание (сокращается)",
        Text::BehindGainingTimeDescription => {
            "Цвет, используемый при отставании и сокращении времени."
        }
        Text::BehindLosingTime => "Отставание (усиливается)",
        Text::BehindLosingTimeDescription => {
            "Цвет, используемый при отставании и дальнейшем ухудшении времени."
        }
        Text::NotRunning => "Не запущено",
        Text::NotRunningDescription => "Цвет, используемый, когда нет активной попытки.",
        Text::PersonalBest => "Личный рекорд",
        Text::PersonalBestDescription => "Цвет, используемый при установке нового личного рекорда.",
        Text::Paused => "Пауза",
        Text::PausedDescription => "Цвет, используемый, когда таймер на паузе.",
        Text::ThinSeparators => "Тонкие разделители",
        Text::ThinSeparatorsDescription => "Цвет тонких разделителей.",
        Text::Separators => "Разделители",
        Text::SeparatorsDescription => "Цвет обычных разделителей.",
        Text::TextColor => "Текст",
        Text::TextColorDescription => "Цвет текста, который не имеет собственного цвета.",
        Text::ComponentBlankSpace => "Пустое место",
        Text::ComponentCurrentComparison => "Текущее сравнение",
        Text::ComponentCurrentPace => "Текущий темп",
        Text::ComponentDelta => "Разница",
        Text::ComponentDetailedTimer => "Детальный таймер",
        Text::ComponentGraph => "График",
        Text::ComponentPbChance => "Шанс PB",
        Text::ComponentPossibleTimeSave => "Возможная экономия времени",
        Text::ComponentPreviousSegment => "Предыдущий сегмент",
        Text::ComponentSegmentTime => "Время сегмента",
        Text::ComponentSeparator => "Разделитель",
        Text::ComponentSplits => "Сплиты",
        Text::ComponentSumOfBest => "Сумма лучших",
        Text::ComponentText => "Текст",
        Text::ComponentTimer => "Таймер",
        Text::ComponentSegmentTimer => "Таймер сегмента",
        Text::ComponentTitle => "Заголовок",
        Text::ComponentTotalPlaytime => "Общее время игры",
        Text::ComponentCurrentPaceBestPossibleTime => "Лучшее возможное время",
        Text::ComponentCurrentPaceWorstPossibleTime => "Худшее возможное время",
        Text::ComponentCurrentPacePredictedTime => "Прогнозируемое время",
        Text::ComponentSegmentTimeBest => "Лучшее время сегмента",
        Text::ComponentSegmentTimeWorst => "Худшее время сегмента",
        Text::ComponentSegmentTimeAverage => "Среднее время сегмента",
        Text::ComponentSegmentTimeMedian => "Медианное время сегмента",
        Text::ComponentSegmentTimeLatest => "Последнее время сегмента",
        Text::ComponentPossibleTimeSaveTotal => "Общая возможная экономия времени",
        Text::LiveSegment => "Живой сегмент",
        Text::LiveSegmentShort => "Живой сегмент",
        Text::PreviousSegmentShort => "Пред. сегмент",
        Text::PreviousSegmentAbbreviation => "Пред. сег.",
        Text::ComparingAgainst => "Сравнение с",
        Text::ComparisonShort => "Сравнение",
        Text::CurrentPaceBestPossibleTimeShort => "Лучшее возм. время",
        Text::CurrentPaceBestTimeShort => "Лучшее время",
        Text::CurrentPaceBestPossibleTimeAbbreviation => "ЛВВ",
        Text::CurrentPaceWorstPossibleTimeShort => "Худшее возм. время",
        Text::CurrentPaceWorstTimeShort => "Худшее время",
        Text::CurrentPacePredictedTimeShort => "Прогн. время",
        Text::CurrentPaceShort => "Тек. темп",
        Text::CurrentPaceAbbreviation => "Темп",
        Text::Goal => "Цель",
        Text::SumOfBestSegments => "Сумма лучших сегментов",
        Text::SumOfBestShort => "Сумма лучших",
        Text::SumOfBestAbbreviation => "СЛС",
        Text::PlaytimeShort => "Время игры",
        Text::BestSegmentTimeShort => "Лучшее вр. сег.",
        Text::BestSegmentShort => "Лучший сегмент",
        Text::WorstSegmentTimeShort => "Худшее вр. сег.",
        Text::WorstSegmentShort => "Худший сегмент",
        Text::AverageSegmentTimeShort => "Ср. вр. сег.",
        Text::AverageSegmentShort => "Средний сегмент",
        Text::MedianSegmentTimeShort => "Мед. вр. сег.",
        Text::MedianSegmentShort => "Медианный сегмент",
        Text::LatestSegmentTimeShort => "Последн. вр. сег.",
        Text::LatestSegmentShort => "Последний сегмент",
        Text::SegmentTimeShort => "Вр. сег.",
        Text::PossibleTimeSaveShort => "Возможная экономия времени",
        Text::PossibleTimeSaveAbbreviation => "Возможн. экономия",
        Text::TimeSaveShort => "Экономия времени",
        Text::RealTime => "Реальное время",
        Text::GameTime => "Игровое время",
        Text::SumOfBestCleanerStartOfRun => "началом забега",
        Text::SumOfBestCleanerShouldRemove => {
            ". Считаете ли вы, что это время сегмента неточно и должно быть удалено?"
        }
    }
}

pub const fn resolve_placeholder(text: PlaceholderText) -> &'static [Piece] {
    match text {
        PlaceholderText::SumOfBestCleanerSegmentTimeBetween => &[
            Piece::Static("У вас было время сегмента "),
            Piece::Dynamic(0),
            Piece::Static(" "),
            Piece::Dynamic(1),
            Piece::Static(" между «"),
            Piece::Dynamic(2),
            Piece::Static("» и «"),
            Piece::Dynamic(3),
            Piece::Static("»"),
        ],
        PlaceholderText::SumOfBestCleanerFasterThanCombined => &[
            Piece::Static(", что быстрее суммы лучших сегментов "),
            Piece::Dynamic(0),
        ],
        PlaceholderText::SumOfBestCleanerRunOn => &[
            Piece::Static(" в забеге "),
            Piece::Dynamic(0),
            Piece::Static(", начавшемся в "),
            Piece::Dynamic(1),
        ],
    }
}
