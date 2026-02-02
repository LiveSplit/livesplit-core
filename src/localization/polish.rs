use super::{Piece, PlaceholderText, Text};

pub const fn resolve(text: Text) -> &'static str {
    match text {
        Text::StartSplit => "Start / Podział",
        Text::StartSplitDescription => "Skrót klawiszowy do dzielenia i rozpoczęcia nowej próby.",
        Text::Reset => "Reset",
        Text::ResetDescription => "Skrót klawiszowy do resetowania bieżącej próby.",
        Text::UndoSplit => "Cofnij split",
        Text::UndoSplitDescription => "Skrót klawiszowy do cofnięcia ostatniego splitu.",
        Text::SkipSplit => "Pomiń split",
        Text::SkipSplitDescription => "Skrót klawiszowy do pominięcia bieżącego splitu.",
        Text::Pause => "Pauza",
        Text::PauseDescription => {
            "Skrót klawiszowy do wstrzymania bieżącej próby. Może też służyć do rozpoczęcia nowej próby."
        }
        Text::UndoAllPauses => "Cofnij wszystkie pauzy",
        Text::UndoAllPausesDescription => {
            "Skrót klawiszowy do usunięcia całego czasu pauz z bieżącego czasu. Przydatne, gdy pauza została włączona przypadkowo."
        }
        Text::PreviousComparison => "Poprzednie porównanie",
        Text::PreviousComparisonDescription => {
            "Skrót klawiszowy do przełączenia na poprzednie porównanie."
        }
        Text::NextComparison => "Następne porównanie",
        Text::NextComparisonDescription => {
            "Skrót klawiszowy do przełączenia na następne porównanie."
        }
        Text::ToggleTimingMethod => "Przełącz metodę pomiaru czasu",
        Text::ToggleTimingMethodDescription => {
            "Skrót klawiszowy do przełączania między metodami „Czas rzeczywisty” i „Czas gry”."
        }
        Text::TimerBackground => "Tło",
        Text::TimerBackgroundDescription => {
            "Tło wyświetlane za komponentem. Można też zastosować kolor powiązany z czasem do przodu lub do tyłu jako kolor tła."
        }
        Text::SegmentTimer => "Timer segmentu",
        Text::SegmentTimerDescription => {
            "Określa, czy wyświetlać czas od rozpoczęcia bieżącego segmentu zamiast czasu od rozpoczęcia bieżącej próby."
        }
        Text::TimingMethod => "Metoda pomiaru czasu",
        Text::TimingMethodDescription => {
            "Określa metodę pomiaru czasu. Jeśli nie określono, używana jest bieżąca metoda."
        }
        Text::Height => "Wysokość",
        Text::HeightDescription => "Wysokość timera.",
        Text::TimerTextColor => "Kolor tekstu",
        Text::TimerTextColorDescription => {
            "Kolor wyświetlanego czasu. Jeśli nie określono, kolor jest automatycznie dobierany na podstawie tego, jak dobrze idzie bieżąca próba. Te kolory można ustawić w ogólnych ustawieniach układu."
        }
        Text::ShowGradient => "Pokaż gradient",
        Text::ShowGradientDescription => {
            "Określa, czy kolor timera ma być wyświetlany jako gradient."
        }
        Text::DigitsFormat => "Format cyfr",
        Text::DigitsFormatDescription => {
            "Określa, ile cyfr wyświetlać. Jeśli czas jest krótszy niż liczba wyświetlanych cyfr, wyświetlane są zera."
        }
        Text::Accuracy => "Dokładność",
        Text::AccuracyDescription => "Dokładność wyświetlanego czasu.",
        Text::TitleBackground => "Tło",
        Text::TitleBackgroundDescription => "Tło wyświetlane za komponentem.",
        Text::TitleTextColor => "Kolor tekstu",
        Text::TitleTextColorDescription => {
            "Kolor tekstu tytułu. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::ShowGameName => "Pokaż nazwę gry",
        Text::ShowGameNameDescription => {
            "Określa, czy nazwa gry ma być częścią wyświetlanego tytułu."
        }
        Text::ShowCategoryName => "Pokaż nazwę kategorii",
        Text::ShowCategoryNameDescription => {
            "Określa, czy nazwa kategorii ma być częścią wyświetlanego tytułu."
        }
        Text::ShowFinishedRunsCount => "Pokaż liczbę ukończonych prób",
        Text::ShowFinishedRunsCountDescription => {
            "Określa, czy ma być wyświetlana liczba pomyślnie ukończonych prób."
        }
        Text::ShowAttemptCount => "Pokaż liczbę prób",
        Text::ShowAttemptCountDescription => "Określa, czy ma być wyświetlana łączna liczba prób.",
        Text::TextAlignment => "Wyrównanie tekstu",
        Text::TextAlignmentDescription => "Określa wyrównanie tytułu.",
        Text::DisplayTextAsSingleLine => "Wyświetl tekst w jednej linii",
        Text::DisplayTextAsSingleLineDescription => {
            "Określa, czy tytuł ma być wyświetlany w jednej linii, zamiast być podzielony na osobną linię dla nazwy gry i osobną dla nazwy kategorii."
        }
        Text::DisplayGameIcon => "Wyświetl ikonę gry",
        Text::DisplayGameIconDescription => {
            "Określa, czy ikona gry ma być wyświetlana, jeśli została zapisana w splitach."
        }
        Text::ShowRegion => "Pokaż region",
        Text::ShowRegionDescription => {
            "Nazwa kategorii może zostać rozszerzona o dodatkowe informacje. Ta opcja rozszerza ją o region gry, jeśli jest on podany w zakładce Zmienne edytora splitów."
        }
        Text::ShowPlatform => "Pokaż platformę",
        Text::ShowPlatformDescription => {
            "Nazwa kategorii może zostać rozszerzona o dodatkowe informacje. Ta opcja rozszerza ją o platformę, na której grana jest gra, jeśli jest ona podana w zakładce Zmienne edytora splitów."
        }
        Text::ShowVariables => "Pokaż zmienne",
        Text::ShowVariablesDescription => {
            "Nazwa kategorii może zostać rozszerzona o dodatkowe informacje. Ta opcja rozszerza ją o dodatkowe zmienne z zakładki Zmienne edytora splitów. Dotyczy to zmiennych speedrun.com, a nie zmiennych niestandardowych."
        }
        Text::TotalPlaytimeBackground => "Tło",
        Text::TotalPlaytimeBackgroundDescription => "Tło wyświetlane za komponentem.",
        Text::DisplayTwoRows => "Wyświetl 2 wiersze",
        Text::DisplayTwoRowsDescription => {
            "Określa, czy wyświetlać nazwę komponentu i łączny czas gry w dwóch oddzielnych wierszach."
        }
        Text::ShowDays => "Pokaż dni (>24h)",
        Text::ShowDaysDescription => {
            "Określa, czy wyświetlać liczbę dni, gdy łączny czas gry osiąga 24 godziny lub więcej."
        }
        Text::LabelColor => "Kolor etykiety",
        Text::LabelColorDescription => {
            "Kolor nazwy komponentu. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::ValueColor => "Kolor wartości",
        Text::ValueColorDescription => {
            "Kolor całkowitego czasu gry. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::BlankSpaceBackground => "Tło",
        Text::BlankSpaceBackgroundDescription => "Tło wyświetlane za komponentem.",
        Text::BlankSpaceSize => "Rozmiar",
        Text::BlankSpaceSizeDescription => "Rozmiar komponentu.",
        Text::CurrentComparisonBackground => "Tło",
        Text::CurrentComparisonBackgroundDescription => "Tło wyświetlane za komponentem.",
        Text::CurrentComparisonDisplayTwoRows => "Wyświetl 2 wiersze",
        Text::CurrentComparisonDisplayTwoRowsDescription => {
            "Określa, czy wyświetlać nazwę komponentu i porównanie w dwóch oddzielnych wierszach."
        }
        Text::CurrentComparisonLabelColor => "Kolor etykiety",
        Text::CurrentComparisonLabelColorDescription => {
            "Kolor nazwy komponentu. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::CurrentComparisonValueColor => "Kolor wartości",
        Text::CurrentComparisonValueColorDescription => {
            "Kolor nazwy porównania. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::CurrentPaceBackground => "Tło",
        Text::CurrentPaceBackgroundDescription => "Tło wyświetlane za komponentem.",
        Text::CurrentPaceComparison => "Porównanie",
        Text::CurrentPaceComparisonDescription => {
            "Porównanie, na podstawie którego przewidywany jest czas końcowy. Jeśli nie określono, używane jest bieżące porównanie."
        }
        Text::CurrentPaceDisplayTwoRows => "Wyświetl 2 wiersze",
        Text::CurrentPaceDisplayTwoRowsDescription => {
            "Określa, czy wyświetlać nazwę komponentu i przewidywany czas w dwóch oddzielnych wierszach."
        }
        Text::CurrentPaceLabelColor => "Kolor etykiety",
        Text::CurrentPaceLabelColorDescription => {
            "Kolor nazwy komponentu. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::CurrentPaceValueColor => "Kolor wartości",
        Text::CurrentPaceValueColorDescription => {
            "Kolor przewidywanego czasu. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::CurrentPaceAccuracy => "Dokładność",
        Text::CurrentPaceAccuracyDescription => "Dokładność wyświetlanego przewidywanego czasu.",
        Text::DeltaBackground => "Tło",
        Text::DeltaBackgroundDescription => "Tło wyświetlane za komponentem.",
        Text::DeltaComparison => "Porównanie",
        Text::DeltaComparisonDescription => {
            "Porównanie używane do obliczania, o ile bieżąca próba jest do przodu lub do tyłu. Jeśli nie określono, używane jest bieżące porównanie."
        }
        Text::DeltaDisplayTwoRows => "Wyświetl 2 wiersze",
        Text::DeltaDisplayTwoRowsDescription => {
            "Określa, czy wyświetlać nazwę porównania i deltę w dwóch oddzielnych wierszach."
        }
        Text::DeltaLabelColor => "Kolor etykiety",
        Text::DeltaLabelColorDescription => {
            "Kolor nazwy porównania. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::DeltaDropDecimals => "Usuń miejsca dziesiętne",
        Text::DeltaDropDecimalsDescription => {
            "Określa, czy nie wyświetlać miejsc dziesiętnych, gdy wizualizowana delta przekracza minutę."
        }
        Text::DeltaAccuracy => "Dokładność",
        Text::DeltaAccuracyDescription => "Dokładność wyświetlanej delty.",
        Text::SumOfBestBackground => "Tło",
        Text::SumOfBestBackgroundDescription => "Tło wyświetlane za komponentem.",
        Text::SumOfBestDisplayTwoRows => "Wyświetl 2 wiersze",
        Text::SumOfBestDisplayTwoRowsDescription => {
            "Określa, czy wyświetlać nazwę komponentu i sumę najlepszych segmentów w dwóch oddzielnych wierszach."
        }
        Text::SumOfBestLabelColor => "Kolor etykiety",
        Text::SumOfBestLabelColorDescription => {
            "Kolor nazwy komponentu. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::SumOfBestValueColor => "Kolor wartości",
        Text::SumOfBestValueColorDescription => {
            "Kolor sumy najlepszych segmentów. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::SumOfBestAccuracy => "Dokładność",
        Text::SumOfBestAccuracyDescription => "Dokładność wyświetlanej sumy najlepszych segmentów.",
        Text::PbChanceBackground => "Tło",
        Text::PbChanceBackgroundDescription => "Tło wyświetlane za komponentem.",
        Text::PbChanceDisplayTwoRows => "Wyświetl 2 wiersze",
        Text::PbChanceDisplayTwoRowsDescription => {
            "Określa, czy wyświetlać nazwę komponentu i szansę PB w dwóch oddzielnych wierszach."
        }
        Text::PbChanceLabelColor => "Kolor etykiety",
        Text::PbChanceLabelColorDescription => {
            "Kolor nazwy komponentu. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::PbChanceValueColor => "Kolor wartości",
        Text::PbChanceValueColorDescription => {
            "Kolor szansy PB. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::PossibleTimeSaveBackground => "Tło",
        Text::PossibleTimeSaveBackgroundDescription => "Tło wyświetlane za komponentem.",
        Text::PossibleTimeSaveComparison => "Porównanie",
        Text::PossibleTimeSaveComparisonDescription => {
            "Porównanie używane do obliczenia możliwej oszczędności czasu. Jeśli nie określono, używane jest bieżące porównanie."
        }
        Text::PossibleTimeSaveDisplayTwoRows => "Wyświetl 2 wiersze",
        Text::PossibleTimeSaveDisplayTwoRowsDescription => {
            "Określa, czy wyświetlać nazwę komponentu i możliwą oszczędność czasu w dwóch oddzielnych wierszach."
        }
        Text::PossibleTimeSaveShowTotal => "Pokaż łączną możliwą oszczędność czasu",
        Text::PossibleTimeSaveShowTotalDescription => {
            "Określa, czy wyświetlać łączną możliwą oszczędność czasu dla reszty bieżącej próby zamiast możliwej oszczędności czasu dla bieżącego segmentu."
        }
        Text::PossibleTimeSaveLabelColor => "Kolor etykiety",
        Text::PossibleTimeSaveLabelColorDescription => {
            "Kolor nazwy komponentu. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::PossibleTimeSaveValueColor => "Kolor wartości",
        Text::PossibleTimeSaveValueColorDescription => {
            "Kolor możliwej oszczędności czasu. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::PossibleTimeSaveAccuracy => "Dokładność",
        Text::PossibleTimeSaveAccuracyDescription => {
            "Dokładność wyświetlanej możliwej oszczędności czasu."
        }
        Text::PreviousSegmentBackground => "Tło",
        Text::PreviousSegmentBackgroundDescription => "Tło wyświetlane za komponentem.",
        Text::PreviousSegmentComparison => "Porównanie",
        Text::PreviousSegmentComparisonDescription => {
            "Porównanie używane do obliczenia, ile czasu zaoszczędzono lub stracono. Jeśli nie określono, używane jest bieżące porównanie."
        }
        Text::PreviousSegmentDisplayTwoRows => "Wyświetl 2 wiersze",
        Text::PreviousSegmentDisplayTwoRowsDescription => {
            "Określa, czy wyświetlać nazwę komponentu i to, ile czasu zaoszczędzono lub stracono, w dwóch oddzielnych wierszach."
        }
        Text::PreviousSegmentLabelColor => "Kolor etykiety",
        Text::PreviousSegmentLabelColorDescription => {
            "Kolor nazwy komponentu. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::PreviousSegmentDropDecimals => "Usuń miejsca dziesiętne",
        Text::PreviousSegmentDropDecimalsDescription => {
            "Określa, czy usuwać miejsca dziesiętne z czasu, gdy wyświetlany czas przekracza minutę."
        }
        Text::PreviousSegmentAccuracy => "Dokładność",
        Text::PreviousSegmentAccuracyDescription => "Dokładność wyświetlanego czasu.",
        Text::PreviousSegmentShowPossibleTimeSave => "Pokaż możliwą oszczędność czasu",
        Text::PreviousSegmentShowPossibleTimeSaveDescription => {
            "Określa, czy wyświetlać, ile czasu mogło zostać zaoszczędzone w poprzednim segmencie, oprócz czasu zaoszczędzonego lub straconego."
        }
        Text::SegmentTimeBackground => "Tło",
        Text::SegmentTimeBackgroundDescription => "Tło wyświetlane za komponentem.",
        Text::SegmentTimeComparison => "Porównanie",
        Text::SegmentTimeComparisonDescription => {
            "Porównanie dla czasu segmentu. Jeśli nie określono, używane jest bieżące porównanie."
        }
        Text::SegmentTimeDisplayTwoRows => "Wyświetl 2 wiersze",
        Text::SegmentTimeDisplayTwoRowsDescription => {
            "Określa, czy wyświetlać nazwę komponentu i czas segmentu w dwóch oddzielnych wierszach."
        }
        Text::SegmentTimeLabelColor => "Kolor etykiety",
        Text::SegmentTimeLabelColorDescription => {
            "Kolor nazwy komponentu. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::SegmentTimeValueColor => "Kolor wartości",
        Text::SegmentTimeValueColorDescription => {
            "Kolor czasu segmentu. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::SegmentTimeAccuracy => "Dokładność",
        Text::SegmentTimeAccuracyDescription => "Dokładność wyświetlanego czasu segmentu.",
        Text::GraphComparison => "Porównanie",
        Text::GraphComparisonDescription => {
            "Porównanie używane dla wykresu. Jeśli nie określono, używane jest bieżące porównanie."
        }
        Text::GraphHeight => "Wysokość",
        Text::GraphHeightDescription => "Wysokość wykresu.",
        Text::GraphShowBestSegments => "Pokaż najlepsze segmenty",
        Text::GraphShowBestSegmentsDescription => {
            "Określa, czy kolorować najlepsze segmenty kolorem najlepszego segmentu z układu."
        }
        Text::GraphLiveGraph => "Wykres na żywo",
        Text::GraphLiveGraphDescription => {
            "Określa, czy wykres ma odświeżać się automatycznie cały czas. Jeśli wyłączone, zmiany pojawiają się tylko przy zmianie bieżącego segmentu."
        }
        Text::GraphFlipGraph => "Odwróć wykres",
        Text::GraphFlipGraphDescription => {
            "Określa, czy wykres ma być odwrócony pionowo. Jeśli wyłączone, czasy przed porównaniem są poniżej osi X, a czasy za porównaniem powyżej. Włączenie odwraca ten układ."
        }
        Text::GraphBehindBackgroundColor => "Kolor tła (z tyłu)",
        Text::GraphBehindBackgroundColorDescription => {
            "Kolor tła obszaru wykresu zawierającego czasy, które są za porównaniem."
        }
        Text::GraphAheadBackgroundColor => "Kolor tła (z przodu)",
        Text::GraphAheadBackgroundColorDescription => {
            "Kolor tła obszaru wykresu zawierającego czasy, które są przed porównaniem."
        }
        Text::GraphGridLinesColor => "Kolor linii siatki",
        Text::GraphGridLinesColorDescription => "Kolor linii siatki wykresu.",
        Text::GraphLinesColor => "Kolor linii wykresu",
        Text::GraphLinesColorDescription => "Kolor linii łączących punkty wykresu.",
        Text::GraphPartialFillColor => "Kolor częściowego wypełnienia",
        Text::GraphPartialFillColorDescription => {
            "Kolor obszaru ograniczonego osią X i wykresem. Częściowe wypełnienie jest używane tylko dla zmian na żywo – dokładniej w przedziale od ostatniego czasu splitu do bieżącego czasu."
        }
        Text::GraphCompleteFillColor => "Kolor pełnego wypełnienia",
        Text::GraphCompleteFillColorDescription => {
            "Kolor obszaru ograniczonego osią X i wykresem, z wyłączeniem segmentu wykresu ze zmianami na żywo."
        }
        Text::DetailedTimerBackground => "Tło",
        Text::DetailedTimerBackgroundDescription => "Tło wyświetlane za komponentem.",
        Text::DetailedTimerTimingMethod => "Metoda pomiaru czasu",
        Text::DetailedTimerTimingMethodDescription => {
            "Określa metodę pomiaru czasu. Jeśli nie określono, używana jest bieżąca metoda."
        }
        Text::DetailedTimerComparison1 => "Porównanie 1",
        Text::DetailedTimerComparison1Description => {
            "Pierwsze porównanie, dla którego ma być wyświetlany czas segmentu. Jeśli nie określono, używane jest bieżące porównanie."
        }
        Text::DetailedTimerComparison2 => "Porównanie 2",
        Text::DetailedTimerComparison2Description => {
            "Drugie porównanie, dla którego ma być wyświetlany czas segmentu. Jeśli nie określono, używane jest bieżące porównanie, chyba że pierwsze porównanie jest również None. Nie jest wyświetlane, jeśli drugie porównanie jest ukryte."
        }
        Text::DetailedTimerHideSecondComparison => "Ukryj drugie porównanie",
        Text::DetailedTimerHideSecondComparisonDescription => {
            "Określa, czy wyświetlać tylko jedno porównanie."
        }
        Text::DetailedTimerTimerHeight => "Wysokość timera",
        Text::DetailedTimerTimerHeightDescription => "Wysokość timera biegu.",
        Text::DetailedTimerSegmentTimerHeight => "Wysokość timera segmentu",
        Text::DetailedTimerSegmentTimerHeightDescription => "Wysokość timera segmentu.",
        Text::DetailedTimerTimerColor => "Kolor timera",
        Text::DetailedTimerTimerColorDescription => {
            "Zamiast automatycznie dobierać kolor głównego timera na podstawie tego, jak dobrze idzie bieżąca próba, można podać stały kolor."
        }
        Text::DetailedTimerShowTimerGradient => "Pokaż gradient timera",
        Text::DetailedTimerShowTimerGradientDescription => {
            "Główny timer automatycznie zmienia kolor na pionowy gradient, jeśli to ustawienie jest aktywne. W przeciwnym razie używany jest rzeczywisty kolor zamiast gradientu."
        }
        Text::DetailedTimerTimerDigitsFormat => "Format cyfr timera",
        Text::DetailedTimerTimerDigitsFormatDescription => {
            "Określa, ile cyfr wyświetlać dla głównego timera. Jeśli czas jest krótszy niż liczba wyświetlanych cyfr, wyświetlane są zera."
        }
        Text::DetailedTimerTimerAccuracy => "Dokładność timera",
        Text::DetailedTimerTimerAccuracyDescription => {
            "Dokładność wyświetlanego czasu dla głównego timera."
        }
        Text::DetailedTimerSegmentTimerColor => "Kolor timera segmentu",
        Text::DetailedTimerSegmentTimerColorDescription => {
            "Zmienia kolor timera segmentu na inny niż domyślny."
        }
        Text::DetailedTimerShowSegmentTimerGradient => "Pokaż gradient timera segmentu",
        Text::DetailedTimerShowSegmentTimerGradientDescription => {
            "Timer segmentu automatycznie zmienia kolor na pionowy gradient, jeśli to ustawienie jest aktywne. W przeciwnym razie używany jest rzeczywisty kolor zamiast gradientu."
        }
        Text::DetailedTimerSegmentTimerDigitsFormat => "Format cyfr timera segmentu",
        Text::DetailedTimerSegmentTimerDigitsFormatDescription => {
            "Określa, ile cyfr wyświetlać dla timera segmentu. Jeśli czas jest krótszy niż liczba wyświetlanych cyfr, wyświetlane są zera."
        }
        Text::DetailedTimerSegmentTimerAccuracy => "Dokładność timera segmentu",
        Text::DetailedTimerSegmentTimerAccuracyDescription => {
            "Dokładność wyświetlanego czasu dla timera segmentu."
        }
        Text::DetailedTimerComparisonNamesColor => "Kolor nazw porównań",
        Text::DetailedTimerComparisonNamesColorDescription => {
            "Kolor nazw porównań, jeśli są wyświetlane. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::DetailedTimerComparisonTimesColor => "Kolor czasów porównań",
        Text::DetailedTimerComparisonTimesColorDescription => {
            "Kolor czasów porównań, jeśli są wyświetlane. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::DetailedTimerComparisonTimesAccuracy => "Dokładność czasów porównań",
        Text::DetailedTimerComparisonTimesAccuracyDescription => "Dokładność czasów porównań.",
        Text::DetailedTimerShowSegmentName => "Pokaż nazwę segmentu",
        Text::DetailedTimerShowSegmentNameDescription => "Określa, czy wyświetlać nazwę segmentu.",
        Text::DetailedTimerSegmentNameColor => "Kolor nazwy segmentu",
        Text::DetailedTimerSegmentNameColorDescription => {
            "Kolor nazwy segmentu, jeśli jest wyświetlana. Jeśli nie określono, kolor jest pobierany z układu."
        }
        Text::DetailedTimerDisplayIcon => "Wyświetl ikonę",
        Text::DetailedTimerDisplayIconDescription => "Określa, czy wyświetlać ikonę segmentu.",
        Text::SplitsBackground => "Tło",
        Text::SplitsBackgroundDescription => {
            "Tło wyświetlane za komponentem. Możesz wybrać kolory naprzemienne. W takim przypadku każdy wiersz naprzemiennie używa dwóch wybranych kolorów."
        }
        Text::SplitsTotalRows => "Łączna liczba wierszy",
        Text::SplitsTotalRowsDescription => {
            "Łączna liczba wierszy segmentów do wyświetlenia na liście. Jeśli ustawiono 0, wszystkie segmenty są wyświetlane. Jeśli ustawiono wartość mniejszą niż liczba segmentów, wyświetlane jest tylko pewne okno segmentów. Okno to może przewijać się w górę lub w dół."
        }
        Text::SplitsUpcomingSegments => "Nadchodzące segmenty",
        Text::SplitsUpcomingSegmentsDescription => {
            "Jeśli segmentów jest więcej niż wyświetlanych wierszy, okno segmentów automatycznie przewija się w górę i w dół, gdy zmienia się bieżący segment. Ta liczba określa minimalną liczbę przyszłych segmentów widocznych w tym oknie."
        }
        Text::SplitsShowThinSeparators => "Pokaż cienkie separatory",
        Text::SplitsShowThinSeparatorsDescription => {
            "Określa, czy wyświetlać cienkie separatory pomiędzy poszczególnymi wierszami segmentów."
        }
        Text::SplitsShowSeparatorBeforeLastSplit => "Pokaż separator przed ostatnim splitem",
        Text::SplitsShowSeparatorBeforeLastSplitDescription => {
            "Jeśli ostatni segment ma być zawsze widoczny, to określa, czy pokazywać bardziej wyraźny separator przed ostatnim segmentem, jeśli nie sąsiaduje bezpośrednio z poprzednim segmentem w oknie przewijania."
        }
        Text::SplitsAlwaysShowLastSplit => "Zawsze pokazuj ostatni split",
        Text::SplitsAlwaysShowLastSplitDescription => {
            "Jeśli nie wszystkie segmenty są widoczne w oknie przewijania, ta opcja określa, czy ostatni segment ma być zawsze widoczny, ponieważ zawiera całkowity czas wybranego porównania. To może być cenne, ponieważ często jest to osobisty rekord biegacza."
        }
        Text::SplitsFillWithBlankSpace => "Wypełnij pustą przestrzenią",
        Text::SplitsFillWithBlankSpaceDescription => {
            "Jeśli segmentów jest za mało, aby wypełnić listę, ta opcja pozwala wypełnić pozostałe wiersze pustą przestrzenią, aby zawsze wyświetlać zadeklarowaną liczbę wierszy. W przeciwnym razie liczba wyświetlanych wierszy zostanie zmniejszona do faktycznej liczby segmentów."
        }
        Text::SplitsShowTimesBelowSegmentName => "Pokaż czasy pod nazwą segmentu",
        Text::SplitsShowTimesBelowSegmentNameDescription => {
            "Określa, czy wyświetlać czasy pod nazwą segmentu. W przeciwnym razie czasy są wyświetlane obok nazwy segmentu."
        }
        Text::SplitsCurrentSegmentGradient => "Gradient bieżącego segmentu",
        Text::SplitsCurrentSegmentGradientDescription => {
            "Gradient wyświetlany za bieżącym segmentem jako wskaźnik, że jest on bieżącym segmentem."
        }
        Text::SplitsSplitTimeAccuracy => "Dokładność czasu splitu",
        Text::SplitsSplitTimeAccuracyDescription => {
            "Określa dokładność używaną do wizualizacji kolumn zawierających czasy splitów."
        }
        Text::SplitsSegmentTimeAccuracy => "Dokładność czasu segmentu",
        Text::SplitsSegmentTimeAccuracyDescription => {
            "Określa dokładność używaną do wizualizacji kolumn zawierających czasy segmentów."
        }
        Text::SplitsDeltaTimeAccuracy => "Dokładność czasu delty",
        Text::SplitsDeltaTimeAccuracyDescription => {
            "Określa dokładność używaną do wizualizacji kolumn zawierających informację o tym, o ile jesteś do przodu lub do tyłu."
        }
        Text::SplitsDropDeltaDecimals => "Usuń miejsca dziesiętne delty przy minutach",
        Text::SplitsDropDeltaDecimalsDescription => {
            "Określa, czy nie wyświetlać miejsc dziesiętnych, gdy kolumna zawierająca informację o tym, o ile jesteś do przodu lub do tyłu, przekracza minutę."
        }
        Text::SplitsShowColumnLabels => "Pokaż etykiety kolumn",
        Text::SplitsShowColumnLabelsDescription => {
            "Określa, czy wyświetlać nazwy kolumn na górze listy."
        }
        Text::SplitsColumns => "Kolumny",
        Text::SplitsColumnsDescription => {
            "Liczba kolumn do wyświetlenia w każdym wierszu. Każda kolumna może być skonfigurowana do wyświetlania różnych informacji. Kolumny są zdefiniowane od prawej do lewej."
        }
        Text::SplitsColumnName => "Nazwa kolumny",
        Text::SplitsColumnNameDescription => {
            "Nazwa kolumny. Jest wyświetlana u góry listy, jeśli włączono opcję wyświetlania etykiet kolumn."
        }
        Text::SplitsColumnType => "Typ kolumny",
        Text::SplitsColumnTypeDescription => {
            "Rodzaj informacji wyświetlanych w tej kolumnie. Może to być czas lub niestandardowa zmienna zapisana w splitach."
        }
        Text::SplitsVariableName => "Nazwa zmiennej",
        Text::SplitsVariableNameDescription => {
            "Nazwa niestandardowej zmiennej wyświetlanej w tej kolumnie."
        }
        Text::SplitsStartWith => "Zacznij od",
        Text::SplitsStartWithDescription => {
            "Wartość, od której ta kolumna zaczyna się dla każdego segmentu. Wyzwalacz aktualizacji określa, kiedy ta wartość zostaje zastąpiona."
        }
        Text::SplitsUpdateWith => "Aktualizuj na",
        Text::SplitsUpdateWithDescription => {
            "Po spełnieniu określonego warunku (zwykle bycie na segmencie lub jego ukończenie) czas jest aktualizowany do wartości określonej tutaj."
        }
        Text::SplitsUpdateTrigger => "Wyzwalacz aktualizacji",
        Text::SplitsUpdateTriggerDescription => {
            "Warunek, który musi zostać spełniony, aby czas został zaktualizowany do wartości określonej w polu „Aktualizuj na”. Przed spełnieniem warunku czas jest wartością z pola „Zacznij od”."
        }
        Text::SplitsColumnComparison => "Porównanie",
        Text::SplitsColumnComparisonDescription => {
            "Porównanie, względem którego porównywana jest ta kolumna. Jeśli nie określono, używane jest bieżące porównanie."
        }
        Text::SplitsColumnTimingMethod => "Metoda pomiaru czasu",
        Text::SplitsColumnTimingMethodDescription => {
            "Określa metodę pomiaru czasu dla tej kolumny. Jeśli nie określono, używana jest bieżąca metoda."
        }
        Text::TextComponentBackground => "Tło",
        Text::TextComponentBackgroundDescription => "Tło wyświetlane za komponentem.",
        Text::TextComponentUseVariable => "Użyj zmiennej",
        Text::TextComponentUseVariableDescription => {
            "Określa, czy używać niestandardowej zmiennej do wyświetlania dynamicznej wartości. Zmienne niestandardowe można określić w edytorze splitów i mogą być dostarczane automatycznie przez auto splittery."
        }
        Text::TextComponentSplit => "Podział",
        Text::TextComponentSplitDescription => {
            "Określa, czy podzielić tekst na lewą i prawą część. W przeciwnym razie wyświetlany jest tylko jeden wyśrodkowany tekst."
        }
        Text::TextComponentText => "Tekst",
        Text::TextComponentTextDescription => "Określa tekst do wyświetlenia na środku.",
        Text::TextComponentLeft => "Lewy",
        Text::TextComponentLeftDescription => "Określa tekst do wyświetlenia po lewej stronie.",
        Text::TextComponentRight => "Prawy",
        Text::TextComponentRightDescription => "Określa tekst do wyświetlenia po prawej stronie.",
        Text::TextComponentVariable => "Zmienna",
        Text::TextComponentVariableDescription => {
            "Określa nazwę niestandardowej zmiennej do wyświetlenia."
        }
        Text::TextComponentTextColor => "Kolor tekstu",
        Text::TextComponentTextColorDescription => "Kolor tekstu.",
        Text::TextComponentLeftColor => "Kolor lewego tekstu",
        Text::TextComponentLeftColorDescription => "Kolor tekstu po lewej stronie.",
        Text::TextComponentRightColor => "Kolor prawego tekstu",
        Text::TextComponentRightColorDescription => "Kolor tekstu po prawej stronie.",
        Text::TextComponentNameColor => "Kolor nazwy",
        Text::TextComponentNameColorDescription => "Kolor nazwy zmiennej.",
        Text::TextComponentValueColor => "Kolor wartości",
        Text::TextComponentValueColorDescription => "Kolor wartości zmiennej.",
        Text::TextComponentDisplayTwoRows => "Wyświetl 2 wiersze",
        Text::TextComponentDisplayTwoRowsDescription => {
            "Określa, czy wyświetlać lewy i prawy tekst w dwóch oddzielnych wierszach."
        }
        Text::LayoutDirection => "Kierunek układu",
        Text::LayoutDirectionDescription => "Kierunek, w którym rozmieszczone są komponenty.",
        Text::CustomTimerFont => "Własna czcionka timera",
        Text::CustomTimerFontDescription => {
            "Pozwala określić niestandardową czcionkę dla timera. Jeśli nie ustawiono, używana jest domyślna czcionka."
        }
        Text::CustomTimesFont => "Własna czcionka czasów",
        Text::CustomTimesFontDescription => {
            "Pozwala określić niestandardową czcionkę dla czasów. Jeśli nie ustawiono, używana jest domyślna czcionka."
        }
        Text::CustomTextFont => "Własna czcionka tekstu",
        Text::CustomTextFontDescription => {
            "Pozwala określić niestandardową czcionkę dla tekstu. Jeśli nie ustawiono, używana jest domyślna czcionka."
        }
        Text::TextShadow => "Cień tekstu",
        Text::TextShadowDescription => "Pozwala opcjonalnie określić kolor cieni tekstu.",
        Text::Background => "Tło",
        Text::BackgroundDescription => "Tło wyświetlane za całym układem.",
        Text::BestSegment => "Najlepszy segment",
        Text::BestSegmentDescription => "Kolor używany po uzyskaniu nowego najlepszego segmentu.",
        Text::AheadGainingTime => "Przed (zyskujesz czas)",
        Text::AheadGainingTimeDescription => {
            "Kolor używany, gdy jesteś przed porównaniem i zyskujesz jeszcze więcej czasu."
        }
        Text::AheadLosingTime => "Przed (tracisz czas)",
        Text::AheadLosingTimeDescription => {
            "Kolor używany, gdy jesteś przed porównaniem, ale tracisz czas."
        }
        Text::BehindGainingTime => "Za (odzyskujesz czas)",
        Text::BehindGainingTimeDescription => {
            "Kolor używany, gdy jesteś za porównaniem, ale odzyskujesz czas."
        }
        Text::BehindLosingTime => "Za (tracisz czas)",
        Text::BehindLosingTimeDescription => {
            "Kolor używany, gdy jesteś za porównaniem i tracisz jeszcze więcej czasu."
        }
        Text::NotRunning => "Nie uruchomiono",
        Text::NotRunningDescription => "Kolor używany, gdy nie ma aktywnej próby.",
        Text::PersonalBest => "Rekord osobisty",
        Text::PersonalBestDescription => "Kolor używany po uzyskaniu nowego rekordu osobistego.",
        Text::Paused => "Wstrzymano",
        Text::PausedDescription => "Kolor używany, gdy timer jest wstrzymany.",
        Text::ThinSeparators => "Cienkie separatory",
        Text::ThinSeparatorsDescription => "Kolor cienkich separatorów.",
        Text::Separators => "Separatory",
        Text::SeparatorsDescription => "Kolor zwykłych separatorów.",
        Text::TextColor => "Tekst",
        Text::TextColorDescription => {
            "Kolor używany dla tekstu, który nie określa własnego koloru."
        }
        Text::ComponentBlankSpace => "Pusta przestrzeń",
        Text::ComponentCurrentComparison => "Bieżące porównanie",
        Text::ComponentCurrentPace => "Bieżące tempo",
        Text::ComponentDelta => "Delta",
        Text::ComponentDetailedTimer => "Szczegółowy timer",
        Text::ComponentGraph => "Wykres",
        Text::ComponentPbChance => "Szansa PB",
        Text::ComponentPossibleTimeSave => "Możliwa oszczędność czasu",
        Text::ComponentPreviousSegment => "Poprzedni segment",
        Text::ComponentSegmentTime => "Czas segmentu",
        Text::ComponentSeparator => "Separator",
        Text::ComponentSplits => "Splity",
        Text::ComponentSumOfBest => "Suma najlepszych",
        Text::ComponentText => "Tekst",
        Text::ComponentTimer => "Timer",
        Text::ComponentSegmentTimer => "Timer segmentu",
        Text::ComponentTitle => "Tytuł",
        Text::ComponentTotalPlaytime => "Łączny czas gry",
        Text::ComponentCurrentPaceBestPossibleTime => "Najlepszy możliwy czas",
        Text::ComponentCurrentPaceWorstPossibleTime => "Najgorszy możliwy czas",
        Text::ComponentCurrentPacePredictedTime => "Przewidywany czas",
        Text::ComponentSegmentTimeBest => "Najlepszy czas segmentu",
        Text::ComponentSegmentTimeWorst => "Najgorszy czas segmentu",
        Text::ComponentSegmentTimeAverage => "Średni czas segmentu",
        Text::ComponentSegmentTimeMedian => "Mediana czasu segmentu",
        Text::ComponentSegmentTimeLatest => "Ostatni czas segmentu",
        Text::ComponentPossibleTimeSaveTotal => "Łączna możliwa oszczędność czasu",
        Text::LiveSegment => "Bieżący segment",
        Text::LiveSegmentShort => "Bież. seg.",
        Text::PreviousSegmentShort => "Poprz. segment",
        Text::PreviousSegmentAbbreviation => "Poprz. seg.",
        Text::ComparingAgainst => "Porównywane do",
        Text::ComparisonShort => "Porównanie",
        Text::CurrentPaceBestPossibleTimeShort => "Najl. możl. czas",
        Text::CurrentPaceBestTimeShort => "Najlepszy czas",
        Text::CurrentPaceBestPossibleTimeAbbreviation => "NMC",
        Text::CurrentPaceWorstPossibleTimeShort => "Najg. możl. czas",
        Text::CurrentPaceWorstTimeShort => "Najgorszy czas",
        Text::CurrentPacePredictedTimeShort => "Przew. czas",
        Text::CurrentPaceShort => "Bież. tempo",
        Text::CurrentPaceAbbreviation => "Tempo",
        Text::Goal => "Cel",
        Text::SumOfBestSegments => "Suma najlepszych segmentów",
        Text::SumOfBestShort => "Suma najlepszych",
        Text::SumOfBestAbbreviation => "SoB",
        Text::PlaytimeShort => "Czas gry",
        Text::BestSegmentTimeShort => "Najl. czas seg.",
        Text::BestSegmentShort => "Najlepszy segment",
        Text::WorstSegmentTimeShort => "Najg. czas seg.",
        Text::WorstSegmentShort => "Najgorszy segment",
        Text::AverageSegmentTimeShort => "Śr. czas seg.",
        Text::AverageSegmentShort => "Średni segment",
        Text::MedianSegmentTimeShort => "Med. czas seg.",
        Text::MedianSegmentShort => "Mediana segmentu",
        Text::LatestSegmentTimeShort => "Ost. czas seg.",
        Text::LatestSegmentShort => "Ostatni segment",
        Text::SegmentTimeShort => "Czas seg.",
        Text::PossibleTimeSaveShort => "Możl. oszcz. czasu",
        Text::PossibleTimeSaveAbbreviation => "Możl. oszcz.",
        Text::TimeSaveShort => "Oszczędność",
        Text::RealTime => "Czas rzeczywisty",
        Text::GameTime => "Czas gry",
        Text::SumOfBestCleanerStartOfRun => "początek biegu",
        Text::SumOfBestCleanerShouldRemove => {
            ". Czy uważasz, że ten czas segmentu jest nieprawidłowy i powinien zostać usunięty?"
        }
    }
}

pub const fn resolve_placeholder(text: PlaceholderText) -> &'static [Piece] {
    match text {
        PlaceholderText::SumOfBestCleanerSegmentTimeBetween => &[
            Piece::Static("Masz "),
            Piece::Dynamic(0),
            Piece::Static(" czas segmentu "),
            Piece::Dynamic(1),
            Piece::Static(" pomiędzy „"),
            Piece::Dynamic(2),
            Piece::Static("” i „"),
            Piece::Dynamic(3),
            Piece::Static("”"),
        ],
        PlaceholderText::SumOfBestCleanerFasterThanCombined => &[
            Piece::Static(", co jest szybciej niż łączna suma najlepszych segmentów "),
            Piece::Dynamic(0),
        ],
        PlaceholderText::SumOfBestCleanerRunOn => &[
            Piece::Static(" w biegu z dnia "),
            Piece::Dynamic(0),
            Piece::Static(", który rozpoczął się o "),
            Piece::Dynamic(1),
        ],
    }
}
