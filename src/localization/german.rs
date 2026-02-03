use super::{Piece, PlaceholderText, Text};

pub const fn resolve(text: Text) -> &'static str {
    match text {
        Text::StartSplit => "Start / Split",
        Text::StartSplitDescription => {
            "Die Hotkey-Taste zum Splitten und Starten eines neuen Versuchs."
        }
        Text::Reset => "Zurücksetzen",
        Text::ResetDescription => "Die Hotkey-Taste zum Zurücksetzen des aktuellen Versuchs.",
        Text::UndoSplit => "Split rückgängig",
        Text::UndoSplitDescription => "Die Hotkey-Taste zum Rückgängig machen des letzten Splits.",
        Text::SkipSplit => "Split überspringen",
        Text::SkipSplitDescription => "Die Hotkey-Taste zum Überspringen des aktuellen Splits.",
        Text::Pause => "Pause",
        Text::PauseDescription => {
            "Die Hotkey-Taste zum Pausieren des aktuellen Versuchs. Sie kann auch zum Starten eines neuen Versuchs verwendet werden."
        }
        Text::UndoAllPauses => "Alle Pausen rückgängig",
        Text::UndoAllPausesDescription => {
            "Die Hotkey-Taste zum Entfernen aller Pausenzeiten aus der aktuellen Zeit. Das ist nützlich, wenn du versehentlich pausiert hast und das rückgängig machen möchtest."
        }
        Text::PreviousComparison => "Vorheriger Vergleich",
        Text::PreviousComparisonDescription => {
            "Die Hotkey-Taste zum Wechseln zum vorherigen Vergleich."
        }
        Text::NextComparison => "Nächster Vergleich",
        Text::NextComparisonDescription => "Die Hotkey-Taste zum Wechseln zum nächsten Vergleich.",
        Text::ToggleTimingMethod => "Zeitmessmethode umschalten",
        Text::ToggleTimingMethodDescription => {
            "Die Hotkey-Taste zum Umschalten zwischen den Zeitmessmethoden „Echtzeit“ und „Spielzeit“."
        }
        Text::TimerBackground => "Hintergrund",
        Text::TimerBackgroundDescription => {
            "Der Hintergrund, der hinter der Komponente angezeigt wird. Es ist auch möglich, die mit der Vor- oder Nachzeit verbundene Farbe als Hintergrundfarbe anzuwenden."
        }
        Text::SegmentTimer => "Segment-Timer",
        Text::SegmentTimerDescription => {
            "Legt fest, ob angezeigt werden soll, wie viel Zeit seit Beginn des aktuellen Segments vergangen ist, anstatt seit Beginn des aktuellen Versuchs."
        }
        Text::TimingMethod => "Zeitmessmethode",
        Text::TimingMethodDescription => {
            "Legt die zu verwendende Zeitmessmethode fest. Wenn nicht angegeben, wird die aktuelle Zeitmessmethode verwendet."
        }
        Text::Height => "Höhe",
        Text::HeightDescription => "Die Höhe des Timers.",
        Text::TimerTextColor => "Textfarbe",
        Text::TimerTextColorDescription => {
            "Die Farbe der angezeigten Zeit. Wenn keine Farbe angegeben ist, wird die Farbe automatisch anhand des Fortschritts des aktuellen Versuchs gewählt. Diese Farben können in den allgemeinen Layout-Einstellungen festgelegt werden."
        }
        Text::ShowGradient => "Farbverlauf anzeigen",
        Text::ShowGradientDescription => {
            "Legt fest, ob die Farbe des Timers als Verlauf angezeigt wird."
        }
        Text::DigitsFormat => "Ziffernformat",
        Text::DigitsFormatDescription => {
            "Legt fest, wie viele Ziffern angezeigt werden. Ist die Dauer kleiner als die anzuzeigenden Ziffern, werden Nullen angezeigt."
        }
        Text::Accuracy => "Genauigkeit",
        Text::AccuracyDescription => "Die Genauigkeit der angezeigten Zeit.",
        Text::TitleBackground => "Hintergrund",
        Text::TitleBackgroundDescription => {
            "Der Hintergrund, der hinter der Komponente angezeigt wird."
        }
        Text::TitleTextColor => "Textfarbe",
        Text::TitleTextColorDescription => {
            "Die Farbe des Titeltexts. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::ShowGameName => "Spielname anzeigen",
        Text::ShowGameNameDescription => {
            "Legt fest, ob der Spielname Teil des angezeigten Titels sein soll."
        }
        Text::ShowCategoryName => "Kategorie anzeigen",
        Text::ShowCategoryNameDescription => {
            "Legt fest, ob der Kategoriename Teil des angezeigten Titels sein soll."
        }
        Text::ShowFinishedRunsCount => "Anzahl abgeschlossener Runs anzeigen",
        Text::ShowFinishedRunsCountDescription => {
            "Legt fest, ob die Anzahl der erfolgreich abgeschlossenen Runs angezeigt werden soll."
        }
        Text::ShowAttemptCount => "Anzahl der Versuche anzeigen",
        Text::ShowAttemptCountDescription => {
            "Legt fest, ob die Gesamtanzahl der Versuche angezeigt werden soll."
        }
        Text::TextAlignment => "Textausrichtung",
        Text::TextAlignmentDescription => "Legt die Ausrichtung des Titels fest.",
        Text::DisplayTextAsSingleLine => "Text einzeilig anzeigen",
        Text::DisplayTextAsSingleLineDescription => {
            "Legt fest, ob der Titel als eine Zeile angezeigt wird, statt in eine Zeile für den Spielnamen und eine für die Kategorie getrennt zu werden."
        }
        Text::DisplayGameIcon => "Spielsymbol anzeigen",
        Text::DisplayGameIconDescription => {
            "Legt fest, ob das Spielsymbol angezeigt werden soll, wenn ein Spielsymbol in den Splits gespeichert ist."
        }
        Text::ShowRegion => "Region anzeigen",
        Text::ShowRegionDescription => {
            "Der Kategoriename kann mit zusätzlichen Informationen erweitert werden. Dies fügt die Region des Spiels hinzu, wenn sie im Variablen-Tab des Splits-Editors angegeben ist."
        }
        Text::ShowPlatform => "Plattform anzeigen",
        Text::ShowPlatformDescription => {
            "Der Kategoriename kann mit zusätzlichen Informationen erweitert werden. Dies fügt die Plattform hinzu, auf der das Spiel gespielt wird, wenn sie im Variablen-Tab des Splits-Editors angegeben ist."
        }
        Text::ShowVariables => "Variablen anzeigen",
        Text::ShowVariablesDescription => {
            "Der Kategoriename kann mit zusätzlichen Informationen erweitert werden. Dies fügt zusätzliche Variablen hinzu, die im Variablen-Tab des Splits-Editors angegeben sind. Dies bezieht sich auf speedrun.com-Variablen, nicht auf benutzerdefinierte Variablen."
        }
        Text::TotalPlaytimeBackground => "Hintergrund",
        Text::TotalPlaytimeBackgroundDescription => {
            "Der Hintergrund, der hinter der Komponente angezeigt wird."
        }
        Text::DisplayTwoRows => "2 Zeilen anzeigen",
        Text::DisplayTwoRowsDescription => {
            "Legt fest, ob der Name der Komponente und die Gesamtspielzeit in zwei separaten Zeilen angezeigt werden."
        }
        Text::ShowDays => "Tage anzeigen (>24h)",
        Text::ShowDaysDescription => {
            "Legt fest, ob die Anzahl der Tage angezeigt werden soll, wenn die Gesamtspielzeit 24 Stunden oder mehr erreicht."
        }
        Text::LabelColor => "Beschriftungsfarbe",
        Text::LabelColorDescription => {
            "Die Farbe des Komponentennamens. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::ValueColor => "Wertfarbe",
        Text::ValueColorDescription => {
            "Die Farbe der Gesamtspielzeit. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::BlankSpaceBackground => "Hintergrund",
        Text::BlankSpaceBackgroundDescription => {
            "Der Hintergrund, der hinter der Komponente angezeigt wird."
        }
        Text::BlankSpaceSize => "Größe",
        Text::BlankSpaceSizeDescription => "Die Größe der Komponente.",
        Text::CurrentComparisonBackground => "Hintergrund",
        Text::CurrentComparisonBackgroundDescription => {
            "Der Hintergrund, der hinter der Komponente angezeigt wird."
        }
        Text::CurrentComparisonDisplayTwoRows => "2 Zeilen anzeigen",
        Text::CurrentComparisonDisplayTwoRowsDescription => {
            "Legt fest, ob der Name der Komponente und der Vergleich in zwei separaten Zeilen angezeigt werden."
        }
        Text::CurrentComparisonLabelColor => "Beschriftungsfarbe",
        Text::CurrentComparisonLabelColorDescription => {
            "Die Farbe des Komponentennamens. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::CurrentComparisonValueColor => "Wertfarbe",
        Text::CurrentComparisonValueColorDescription => {
            "Die Farbe des Vergleichsnamens. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::CurrentPaceBackground => "Hintergrund",
        Text::CurrentPaceBackgroundDescription => {
            "Der Hintergrund, der hinter der Komponente angezeigt wird."
        }
        Text::CurrentPaceComparison => "Vergleich",
        Text::CurrentPaceComparisonDescription => {
            "Der Vergleich, aus dem die Zielzeit vorhergesagt wird. Wenn nicht angegeben, wird der aktuelle Vergleich verwendet."
        }
        Text::CurrentPaceDisplayTwoRows => "2 Zeilen anzeigen",
        Text::CurrentPaceDisplayTwoRowsDescription => {
            "Legt fest, ob der Name der Komponente und die vorhergesagte Zeit in zwei separaten Zeilen angezeigt werden."
        }
        Text::CurrentPaceLabelColor => "Beschriftungsfarbe",
        Text::CurrentPaceLabelColorDescription => {
            "Die Farbe des Komponentennamens. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::CurrentPaceValueColor => "Wertfarbe",
        Text::CurrentPaceValueColorDescription => {
            "Die Farbe der vorhergesagten Zeit. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::CurrentPaceAccuracy => "Genauigkeit",
        Text::CurrentPaceAccuracyDescription => {
            "Die Genauigkeit der angezeigten vorhergesagten Zeit."
        }
        Text::DeltaBackground => "Hintergrund",
        Text::DeltaBackgroundDescription => {
            "Der Hintergrund, der hinter der Komponente angezeigt wird."
        }
        Text::DeltaComparison => "Vergleich",
        Text::DeltaComparisonDescription => {
            "Der Vergleich, der verwendet wird, um zu berechnen, wie weit der aktuelle Versuch voraus oder zurück ist. Wenn nicht angegeben, wird der aktuelle Vergleich verwendet."
        }
        Text::DeltaDisplayTwoRows => "2 Zeilen anzeigen",
        Text::DeltaDisplayTwoRowsDescription => {
            "Legt fest, ob der Name des Vergleichs und die Delta-Zeit in zwei separaten Zeilen angezeigt werden."
        }
        Text::DeltaLabelColor => "Beschriftungsfarbe",
        Text::DeltaLabelColorDescription => {
            "Die Farbe des Vergleichsnamens. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::DeltaDropDecimals => "Dezimalstellen entfernen",
        Text::DeltaDropDecimalsDescription => {
            "Legt fest, ob Dezimalstellen nicht mehr angezeigt werden, wenn das Delta größer als eine Minute ist."
        }
        Text::DeltaAccuracy => "Genauigkeit",
        Text::DeltaAccuracyDescription => "Die Genauigkeit des angezeigten Deltas.",
        Text::SumOfBestBackground => "Hintergrund",
        Text::SumOfBestBackgroundDescription => {
            "Der Hintergrund, der hinter der Komponente angezeigt wird."
        }
        Text::SumOfBestDisplayTwoRows => "2 Zeilen anzeigen",
        Text::SumOfBestDisplayTwoRowsDescription => {
            "Legt fest, ob der Name der Komponente und die Summe der besten Segmente in zwei separaten Zeilen angezeigt werden."
        }
        Text::SumOfBestLabelColor => "Beschriftungsfarbe",
        Text::SumOfBestLabelColorDescription => {
            "Die Farbe des Komponentennamens. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::SumOfBestValueColor => "Wertfarbe",
        Text::SumOfBestValueColorDescription => {
            "Die Farbe der Summe der besten Segmente. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::SumOfBestAccuracy => "Genauigkeit",
        Text::SumOfBestAccuracyDescription => {
            "Die Genauigkeit der angezeigten Summe der besten Segmente."
        }
        Text::PbChanceBackground => "Hintergrund",
        Text::PbChanceBackgroundDescription => {
            "Der Hintergrund, der hinter der Komponente angezeigt wird."
        }
        Text::PbChanceDisplayTwoRows => "2 Zeilen anzeigen",
        Text::PbChanceDisplayTwoRowsDescription => {
            "Legt fest, ob der Name der Komponente und die PB-Chance in zwei separaten Zeilen angezeigt werden."
        }
        Text::PbChanceLabelColor => "Beschriftungsfarbe",
        Text::PbChanceLabelColorDescription => {
            "Die Farbe des Komponentennamens. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::PbChanceValueColor => "Wertfarbe",
        Text::PbChanceValueColorDescription => {
            "Die Farbe der PB-Chance. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::PossibleTimeSaveBackground => "Hintergrund",
        Text::PossibleTimeSaveBackgroundDescription => {
            "Der Hintergrund, der hinter der Komponente angezeigt wird."
        }
        Text::PossibleTimeSaveComparison => "Vergleich",
        Text::PossibleTimeSaveComparisonDescription => {
            "Der Vergleich, für den die mögliche Zeitersparnis berechnet wird. Wenn nicht angegeben, wird der aktuelle Vergleich verwendet."
        }
        Text::PossibleTimeSaveDisplayTwoRows => "2 Zeilen anzeigen",
        Text::PossibleTimeSaveDisplayTwoRowsDescription => {
            "Legt fest, ob der Name der Komponente und die mögliche Zeitersparnis in zwei separaten Zeilen angezeigt werden."
        }
        Text::PossibleTimeSaveShowTotal => "Gesamte mögliche Zeitersparnis anzeigen",
        Text::PossibleTimeSaveShowTotalDescription => {
            "Legt fest, ob die gesamte mögliche Zeitersparnis für den restlichen Versuch angezeigt wird, statt der möglichen Zeitersparnis für das aktuelle Segment."
        }
        Text::PossibleTimeSaveLabelColor => "Beschriftungsfarbe",
        Text::PossibleTimeSaveLabelColorDescription => {
            "Die Farbe des Komponentennamens. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::PossibleTimeSaveValueColor => "Wertfarbe",
        Text::PossibleTimeSaveValueColorDescription => {
            "Die Farbe der möglichen Zeitersparnis. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::PossibleTimeSaveAccuracy => "Genauigkeit",
        Text::PossibleTimeSaveAccuracyDescription => {
            "Die Genauigkeit der angezeigten möglichen Zeitersparnis."
        }
        Text::PreviousSegmentBackground => "Hintergrund",
        Text::PreviousSegmentBackgroundDescription => {
            "Der Hintergrund, der hinter der Komponente angezeigt wird."
        }
        Text::PreviousSegmentComparison => "Vergleich",
        Text::PreviousSegmentComparisonDescription => {
            "Der Vergleich, der verwendet wird, um zu berechnen, wie viel Zeit gespart oder verloren wurde. Wenn nicht angegeben, wird der aktuelle Vergleich verwendet."
        }
        Text::PreviousSegmentDisplayTwoRows => "2 Zeilen anzeigen",
        Text::PreviousSegmentDisplayTwoRowsDescription => {
            "Legt fest, ob der Name der Komponente und die gesparte oder verlorene Zeit in zwei separaten Zeilen angezeigt werden."
        }
        Text::PreviousSegmentLabelColor => "Beschriftungsfarbe",
        Text::PreviousSegmentLabelColorDescription => {
            "Die Farbe des Komponentennamens. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::PreviousSegmentDropDecimals => "Dezimalstellen entfernen",
        Text::PreviousSegmentDropDecimalsDescription => {
            "Legt fest, ob Dezimalstellen entfernt werden, wenn die angezeigte Zeit über eine Minute liegt."
        }
        Text::PreviousSegmentAccuracy => "Genauigkeit",
        Text::PreviousSegmentAccuracyDescription => "Die Genauigkeit der angezeigten Zeit.",
        Text::PreviousSegmentShowPossibleTimeSave => "Mögliche Zeitersparnis anzeigen",
        Text::PreviousSegmentShowPossibleTimeSaveDescription => {
            "Legt fest, ob zusätzlich zur gesparten oder verlorenen Zeit angezeigt wird, wie viel Zeit im vorherigen Segment hätte gespart werden können."
        }
        Text::SegmentTimeBackground => "Hintergrund",
        Text::SegmentTimeBackgroundDescription => {
            "Der Hintergrund, der hinter der Komponente angezeigt wird."
        }
        Text::SegmentTimeComparison => "Vergleich",
        Text::SegmentTimeComparisonDescription => {
            "Der Vergleich für die Segmentzeit. Wenn nicht angegeben, wird der aktuelle Vergleich verwendet."
        }
        Text::SegmentTimeDisplayTwoRows => "2 Zeilen anzeigen",
        Text::SegmentTimeDisplayTwoRowsDescription => {
            "Legt fest, ob der Name der Komponente und die Segmentzeit in zwei separaten Zeilen angezeigt werden."
        }
        Text::SegmentTimeLabelColor => "Beschriftungsfarbe",
        Text::SegmentTimeLabelColorDescription => {
            "Die Farbe des Komponentennamens. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::SegmentTimeValueColor => "Wertfarbe",
        Text::SegmentTimeValueColorDescription => {
            "Die Farbe der Segmentzeit. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::SegmentTimeAccuracy => "Genauigkeit",
        Text::SegmentTimeAccuracyDescription => "Die Genauigkeit der angezeigten Segmentzeit.",
        Text::GraphComparison => "Vergleich",
        Text::GraphComparisonDescription => {
            "Der Vergleich, der für das Diagramm verwendet wird. Wenn nicht angegeben, wird der aktuelle Vergleich verwendet."
        }
        Text::GraphHeight => "Höhe",
        Text::GraphHeightDescription => "Die Höhe des Diagramms.",
        Text::GraphShowBestSegments => "Beste Segmente anzeigen",
        Text::GraphShowBestSegmentsDescription => {
            "Legt fest, ob die besten Segmente mit der Bestsegment-Farbe des Layouts eingefärbt werden."
        }
        Text::GraphLiveGraph => "Live-Diagramm",
        Text::GraphLiveGraphDescription => {
            "Legt fest, ob das Diagramm ständig automatisch aktualisiert wird. Ist dies deaktiviert, ändern sich die Diagrammdaten nur, wenn sich das aktuelle Segment ändert."
        }
        Text::GraphFlipGraph => "Diagramm spiegeln",
        Text::GraphFlipGraphDescription => {
            "Legt fest, ob das Diagramm vertikal gespiegelt werden soll. Wenn nicht aktiviert, werden Zeiten, die vor dem Vergleich liegen, unterhalb der x-Achse angezeigt und Zeiten, die dahinter liegen, oberhalb. Diese Einstellung kehrt das um."
        }
        Text::GraphBehindBackgroundColor => "Hintergrundfarbe (hinten)",
        Text::GraphBehindBackgroundColorDescription => {
            "Die Hintergrundfarbe für den Diagrammbereich mit Zeiten, die hinter dem Vergleich liegen."
        }
        Text::GraphAheadBackgroundColor => "Hintergrundfarbe (vorne)",
        Text::GraphAheadBackgroundColorDescription => {
            "Die Hintergrundfarbe für den Diagrammbereich mit Zeiten, die vor dem Vergleich liegen."
        }
        Text::GraphGridLinesColor => "Gitterlinienfarbe",
        Text::GraphGridLinesColorDescription => "Die Farbe der Gitterlinien des Diagramms.",
        Text::GraphLinesColor => "Diagrammlinienfarbe",
        Text::GraphLinesColorDescription => {
            "Die Farbe der Linien, die die Punkte des Diagramms verbinden."
        }
        Text::GraphPartialFillColor => "Teilfüllfarbe",
        Text::GraphPartialFillColorDescription => {
            "Die Farbe der Fläche zwischen x-Achse und Diagramm. Die Teilfüllfarbe wird nur für Live-Änderungen verwendet. Genauer gesagt gilt sie für den Bereich vom letzten Split bis zur aktuellen Zeit."
        }
        Text::GraphCompleteFillColor => "Vollfüllfarbe",
        Text::GraphCompleteFillColorDescription => {
            "Die Farbe der Fläche zwischen x-Achse und Diagramm ohne das Diagrammsegment mit Live-Änderungen."
        }
        Text::DetailedTimerBackground => "Hintergrund",
        Text::DetailedTimerBackgroundDescription => {
            "Der Hintergrund, der hinter der Komponente angezeigt wird."
        }
        Text::DetailedTimerTimingMethod => "Zeitmessmethode",
        Text::DetailedTimerTimingMethodDescription => {
            "Legt die zu verwendende Zeitmessmethode fest. Wenn nicht angegeben, wird die aktuelle Zeitmessmethode verwendet."
        }
        Text::DetailedTimerComparison1 => "Vergleich 1",
        Text::DetailedTimerComparison1Description => {
            "Der erste Vergleich, dessen Segmentzeit angezeigt wird. Wenn nicht angegeben, wird der aktuelle Vergleich verwendet."
        }
        Text::DetailedTimerComparison2 => "Vergleich 2",
        Text::DetailedTimerComparison2Description => {
            "Der zweite Vergleich, dessen Segmentzeit angezeigt wird. Wenn nicht angegeben, wird der aktuelle Vergleich verwendet, es sei denn, der erste Vergleich ist ebenfalls None. Dies wird nicht angezeigt, wenn der zweite Vergleich verborgen ist."
        }
        Text::DetailedTimerHideSecondComparison => "Zweiten Vergleich ausblenden",
        Text::DetailedTimerHideSecondComparisonDescription => {
            "Legt fest, ob nur ein einzelner Vergleich angezeigt werden soll."
        }
        Text::DetailedTimerTimerHeight => "Timer-Höhe",
        Text::DetailedTimerTimerHeightDescription => "Die Höhe des Run-Timers.",
        Text::DetailedTimerSegmentTimerHeight => "Segment-Timer-Höhe",
        Text::DetailedTimerSegmentTimerHeightDescription => "Die Höhe des Segment-Timers.",
        Text::DetailedTimerTimerColor => "Timer-Farbe",
        Text::DetailedTimerTimerColorDescription => {
            "Statt die Farbe des Haupttimers automatisch anhand des Fortschritts des aktuellen Versuchs zu bestimmen, kann eine feste Farbe angegeben werden."
        }
        Text::DetailedTimerShowTimerGradient => "Timer-Farbverlauf anzeigen",
        Text::DetailedTimerShowTimerGradientDescription => {
            "Der Haupttimer wandelt seine Farbe automatisch in einen vertikalen Verlauf, wenn diese Einstellung aktiviert ist. Andernfalls wird die tatsächliche Farbe statt eines Verlaufs verwendet."
        }
        Text::DetailedTimerTimerDigitsFormat => "Timer-Ziffernformat",
        Text::DetailedTimerTimerDigitsFormatDescription => {
            "Legt fest, wie viele Ziffern für den Haupttimer angezeigt werden. Ist die Dauer kleiner als die anzuzeigenden Ziffern, werden Nullen angezeigt."
        }
        Text::DetailedTimerTimerAccuracy => "Timer-Genauigkeit",
        Text::DetailedTimerTimerAccuracyDescription => {
            "Die Genauigkeit der für den Haupttimer angezeigten Zeit."
        }
        Text::DetailedTimerSegmentTimerColor => "Segment-Timer-Farbe",
        Text::DetailedTimerSegmentTimerColorDescription => {
            "Ändert die Farbe des Segment-Timers auf eine Farbe, die sich von der Standardfarbe unterscheidet."
        }
        Text::DetailedTimerShowSegmentTimerGradient => "Segment-Timer-Farbverlauf anzeigen",
        Text::DetailedTimerShowSegmentTimerGradientDescription => {
            "Der Segment-Timer wandelt seine Farbe automatisch in einen vertikalen Verlauf, wenn diese Einstellung aktiviert ist. Andernfalls wird die tatsächliche Farbe statt eines Verlaufs verwendet."
        }
        Text::DetailedTimerSegmentTimerDigitsFormat => "Segment-Timer-Ziffernformat",
        Text::DetailedTimerSegmentTimerDigitsFormatDescription => {
            "Legt fest, wie viele Ziffern für den Segment-Timer angezeigt werden. Ist die Dauer kleiner als die anzuzeigenden Ziffern, werden Nullen angezeigt."
        }
        Text::DetailedTimerSegmentTimerAccuracy => "Segment-Timer-Genauigkeit",
        Text::DetailedTimerSegmentTimerAccuracyDescription => {
            "Die Genauigkeit der für den Segment-Timer angezeigten Zeit."
        }
        Text::DetailedTimerComparisonNamesColor => "Vergleichsnamen-Farbe",
        Text::DetailedTimerComparisonNamesColorDescription => {
            "Die Farbe der Vergleichsnamen, falls sie angezeigt werden. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::DetailedTimerComparisonTimesColor => "Vergleichszeiten-Farbe",
        Text::DetailedTimerComparisonTimesColorDescription => {
            "Die Farbe der Vergleichszeiten, falls sie angezeigt werden. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::DetailedTimerComparisonTimesAccuracy => "Vergleichszeiten-Genauigkeit",
        Text::DetailedTimerComparisonTimesAccuracyDescription => {
            "Die Genauigkeit der Vergleichszeiten."
        }
        Text::DetailedTimerShowSegmentName => "Segmentname anzeigen",
        Text::DetailedTimerShowSegmentNameDescription => {
            "Legt fest, ob der Segmentname angezeigt werden soll."
        }
        Text::DetailedTimerSegmentNameColor => "Segmentnamen-Farbe",
        Text::DetailedTimerSegmentNameColorDescription => {
            "Die Farbe des Segmentnamens, falls angezeigt. Wenn keine Farbe angegeben ist, wird die Farbe aus dem Layout übernommen."
        }
        Text::DetailedTimerDisplayIcon => "Symbol anzeigen",
        Text::DetailedTimerDisplayIconDescription => {
            "Legt fest, ob das Segmentsymbol angezeigt werden soll."
        }
        Text::SplitsBackground => "Hintergrund",
        Text::SplitsBackgroundDescription => {
            "Der Hintergrund, der hinter der Komponente angezeigt wird. Die Farben können abwechselnd verwendet werden. In diesem Fall wechselt jede Zeile zwischen den beiden gewählten Farben."
        }
        Text::SplitsTotalRows => "Gesamtzeilen",
        Text::SplitsTotalRowsDescription => {
            "Die Gesamtanzahl der Segmentzeilen, die in der Liste angezeigt werden. Bei 0 werden alle Segmente angezeigt. Bei einer Zahl kleiner als die Gesamtanzahl wird nur ein bestimmtes Fenster angezeigt, das nach oben oder unten scrollen kann."
        }
        Text::SplitsUpcomingSegments => "Kommende Segmente",
        Text::SplitsUpcomingSegmentsDescription => {
            "Wenn es mehr Segmente als angezeigte Zeilen gibt, scrollt das Segmentfenster automatisch, wenn sich das aktuelle Segment ändert. Diese Zahl bestimmt die Mindestanzahl zukünftiger Segmente, die in diesem Fenster angezeigt werden."
        }
        Text::SplitsShowThinSeparators => "Dünne Trennlinien anzeigen",
        Text::SplitsShowThinSeparatorsDescription => {
            "Legt fest, ob dünne Trennlinien zwischen den einzelnen Segmentzeilen angezeigt werden."
        }
        Text::SplitsShowSeparatorBeforeLastSplit => "Trennlinie vor letztem Split anzeigen",
        Text::SplitsShowSeparatorBeforeLastSplitDescription => {
            "Wenn das letzte Segment immer angezeigt werden soll, bestimmt dies, ob vor dem letzten Segment eine deutlichere Trennlinie angezeigt wird, wenn es im Scrollfenster nicht direkt neben dem vorherigen Segment liegt."
        }
        Text::SplitsAlwaysShowLastSplit => "Letzten Split immer anzeigen",
        Text::SplitsAlwaysShowLastSplitDescription => {
            "Wenn nicht alle Segmente im Scrollfenster angezeigt werden, bestimmt diese Option, ob das letzte Segment immer angezeigt wird, da es die Gesamtdauer des gewählten Vergleichs enthält. Das ist oft die persönliche Bestzeit."
        }
        Text::SplitsFillWithBlankSpace => "Mit Leerraum auffüllen",
        Text::SplitsFillWithBlankSpaceDescription => {
            "Wenn nicht genügend Segmente vorhanden sind, kann die Liste mit Leerraum aufgefüllt werden, um immer die eingestellte Gesamtzahl von Zeilen anzuzeigen. Andernfalls wird die Anzahl der angezeigten Zeilen auf die tatsächliche Segmentanzahl reduziert."
        }
        Text::SplitsShowTimesBelowSegmentName => "Zeiten unter dem Segmentnamen anzeigen",
        Text::SplitsShowTimesBelowSegmentNameDescription => {
            "Legt fest, ob die Zeiten unter dem Segmentnamen angezeigt werden. Andernfalls werden die Zeiten neben dem Segmentnamen angezeigt."
        }
        Text::SplitsCurrentSegmentGradient => "Aktueller Segmentverlauf",
        Text::SplitsCurrentSegmentGradientDescription => {
            "Der Verlauf, der hinter dem aktuellen Segment als Indikator angezeigt wird."
        }
        Text::SplitsSplitTimeAccuracy => "Splitzeit-Genauigkeit",
        Text::SplitsSplitTimeAccuracyDescription => {
            "Legt die Genauigkeit fest, die für Spalten mit Splitzeiten verwendet wird."
        }
        Text::SplitsSegmentTimeAccuracy => "Segmentzeit-Genauigkeit",
        Text::SplitsSegmentTimeAccuracyDescription => {
            "Legt die Genauigkeit fest, die für Spalten mit Segmentzeiten verwendet wird."
        }
        Text::SplitsDeltaTimeAccuracy => "Delta-Zeit-Genauigkeit",
        Text::SplitsDeltaTimeAccuracyDescription => {
            "Legt die Genauigkeit fest, die für Spalten mit der Vor- oder Nachzeit verwendet wird."
        }
        Text::SplitsDropDeltaDecimals => "Delta-Dezimalstellen bei Minuten ausblenden",
        Text::SplitsDropDeltaDecimalsDescription => {
            "Legt fest, ob Dezimalstellen nicht mehr angezeigt werden, wenn eine Spalte mit Vor- oder Nachzeit über eine Minute beträgt."
        }
        Text::SplitsShowColumnLabels => "Spaltenbeschriftungen anzeigen",
        Text::SplitsShowColumnLabelsDescription => {
            "Legt fest, ob die Namen der Spalten oben in der Liste angezeigt werden."
        }
        Text::SplitsColumns => "Spalten",
        Text::SplitsColumnsDescription => {
            "Die Anzahl der Spalten pro Zeile. Jede Spalte kann unterschiedliche Informationen anzeigen. Die Spalten sind von rechts nach links definiert."
        }
        Text::SplitsColumnName => "Spaltenname",
        Text::SplitsColumnNameDescription => {
            "Der Name der Spalte. Dieser wird oben in der Liste angezeigt, wenn Spaltenbeschriftungen aktiviert sind."
        }
        Text::SplitsColumnType => "Spaltentyp",
        Text::SplitsColumnTypeDescription => {
            "Die Art der Informationen, die diese Spalte anzeigt. Dies kann eine Zeit oder eine benutzerdefinierte Variable aus den Splits sein."
        }
        Text::SplitsVariableName => "Variablenname",
        Text::SplitsVariableNameDescription => {
            "Der Name der benutzerdefinierten Variable, die diese Spalte anzeigt."
        }
        Text::SplitsStartWith => "Startwert",
        Text::SplitsStartWithDescription => {
            "Der Wert, mit dem diese Spalte für jedes Segment beginnt. Der Update-Trigger bestimmt, wann dieser Wert ersetzt wird."
        }
        Text::SplitsUpdateWith => "Aktualisieren mit",
        Text::SplitsUpdateWithDescription => {
            "Sobald eine bestimmte Bedingung erfüllt ist, üblicherweise auf dem Segment oder nachdem es abgeschlossen ist, wird der Wert mit dem hier angegebenen ersetzt."
        }
        Text::SplitsUpdateTrigger => "Update-Trigger",
        Text::SplitsUpdateTriggerDescription => {
            "Die Bedingung, die erfüllt sein muss, damit der Wert mit dem in „Aktualisieren mit“ angegebenen ersetzt wird. Vorher ist der Wert der im Startwert angegebene."
        }
        Text::SplitsColumnComparison => "Vergleich",
        Text::SplitsColumnComparisonDescription => {
            "Der Vergleich, gegen den diese Spalte verglichen wird. Wenn nicht angegeben, wird der aktuelle Vergleich verwendet."
        }
        Text::SplitsColumnTimingMethod => "Zeitmessmethode",
        Text::SplitsColumnTimingMethodDescription => {
            "Legt die Zeitmessmethode fest, die für diese Spalte verwendet wird. Wenn nicht angegeben, wird die aktuelle Zeitmessmethode verwendet."
        }
        Text::TextComponentBackground => "Hintergrund",
        Text::TextComponentBackgroundDescription => {
            "Der Hintergrund, der hinter der Komponente angezeigt wird."
        }
        Text::TextComponentUseVariable => "Variable verwenden",
        Text::TextComponentUseVariableDescription => {
            "Legt fest, ob eine benutzerdefinierte Variable verwendet wird, um einen dynamischen Wert anzuzeigen. Benutzerdefinierte Variablen können im Splits-Editor angegeben und automatisch von Auto-Splittern bereitgestellt werden."
        }
        Text::TextComponentSplit => "Trennen",
        Text::TextComponentSplitDescription => {
            "Legt fest, ob der Text in einen linken und rechten Teil getrennt wird. Wenn nicht, wird nur ein zentrierter Text angezeigt."
        }
        Text::TextComponentText => "Text",
        Text::TextComponentTextDescription => "Legt den Text fest, der zentriert angezeigt wird.",
        Text::TextComponentLeft => "Links",
        Text::TextComponentLeftDescription => "Legt den Text fest, der links angezeigt wird.",
        Text::TextComponentRight => "Rechts",
        Text::TextComponentRightDescription => "Legt den Text fest, der rechts angezeigt wird.",
        Text::TextComponentVariable => "Variable",
        Text::TextComponentVariableDescription => {
            "Legt den Namen der benutzerdefinierten Variable fest, die angezeigt wird."
        }
        Text::TextComponentTextColor => "Textfarbe",
        Text::TextComponentTextColorDescription => "Die Farbe des Texts.",
        Text::TextComponentLeftColor => "Linke Farbe",
        Text::TextComponentLeftColorDescription => "Die Farbe des Texts auf der linken Seite.",
        Text::TextComponentRightColor => "Rechte Farbe",
        Text::TextComponentRightColorDescription => "Die Farbe des Texts auf der rechten Seite.",
        Text::TextComponentNameColor => "Namensfarbe",
        Text::TextComponentNameColorDescription => "Die Farbe des Variablennamens.",
        Text::TextComponentValueColor => "Wertfarbe",
        Text::TextComponentValueColorDescription => "Die Farbe des Variablenwerts.",
        Text::TextComponentDisplayTwoRows => "2 Zeilen anzeigen",
        Text::TextComponentDisplayTwoRowsDescription => {
            "Legt fest, ob der linke und rechte Text in zwei separaten Zeilen angezeigt werden."
        }
        Text::LayoutDirection => "Layout-Richtung",
        Text::LayoutDirectionDescription => "Die Richtung, in der die Komponenten angeordnet sind.",
        Text::CustomTimerFont => "Benutzerdefinierte Timer-Schriftart",
        Text::CustomTimerFontDescription => {
            "Ermöglicht das Festlegen einer benutzerdefinierten Schriftart für den Timer. Wenn dies nicht gesetzt ist, wird die Standardschriftart verwendet."
        }
        Text::CustomTimesFont => "Benutzerdefinierte Zeiten-Schriftart",
        Text::CustomTimesFontDescription => {
            "Ermöglicht das Festlegen einer benutzerdefinierten Schriftart für die Zeiten. Wenn dies nicht gesetzt ist, wird die Standardschriftart verwendet."
        }
        Text::CustomTextFont => "Benutzerdefinierte Text-Schriftart",
        Text::CustomTextFontDescription => {
            "Ermöglicht das Festlegen einer benutzerdefinierten Schriftart für den Text. Wenn dies nicht gesetzt ist, wird die Standardschriftart verwendet."
        }
        Text::TextShadow => "Textschatten",
        Text::TextShadowDescription => {
            "Ermöglicht das optionale Festlegen einer Farbe für Textschatten."
        }
        Text::Background => "Hintergrund",
        Text::BackgroundDescription => {
            "Der Hintergrund, der hinter dem gesamten Layout angezeigt wird."
        }
        Text::BestSegment => "Bestes Segment",
        Text::BestSegmentDescription => {
            "Die Farbe, die verwendet wird, wenn du ein neues Bestsegment erreichst."
        }
        Text::AheadGainingTime => "Vorne (Zeit gewinnen)",
        Text::AheadGainingTimeDescription => {
            "Die Farbe, die verwendet wird, wenn du vor dem Vergleich liegst und noch mehr Zeit gewinnst."
        }
        Text::AheadLosingTime => "Vorne (Zeit verlieren)",
        Text::AheadLosingTimeDescription => {
            "Die Farbe, die verwendet wird, wenn du vor dem Vergleich liegst, aber Zeit verlierst."
        }
        Text::BehindGainingTime => "Hinten (Zeit aufholen)",
        Text::BehindGainingTimeDescription => {
            "Die Farbe, die verwendet wird, wenn du hinter dem Vergleich liegst, aber Zeit aufholst."
        }
        Text::BehindLosingTime => "Hinten (Zeit verlieren)",
        Text::BehindLosingTimeDescription => {
            "Die Farbe, die verwendet wird, wenn du hinter dem Vergleich liegst und noch mehr Zeit verlierst."
        }
        Text::NotRunning => "Nicht laufend",
        Text::NotRunningDescription => {
            "Die Farbe, die verwendet wird, wenn kein Versuch aktiv ist."
        }
        Text::PersonalBest => "Persönliche Bestzeit",
        Text::PersonalBestDescription => {
            "Die Farbe, die verwendet wird, wenn du eine neue persönliche Bestzeit erreichst."
        }
        Text::Paused => "Pausiert",
        Text::PausedDescription => "Die Farbe, die verwendet wird, wenn der Timer pausiert ist.",
        Text::ThinSeparators => "Dünne Trennlinien",
        Text::ThinSeparatorsDescription => "Die Farbe der dünnen Trennlinien.",
        Text::Separators => "Trennlinien",
        Text::SeparatorsDescription => "Die Farbe der normalen Trennlinien.",
        Text::TextColor => "Text",
        Text::TextColorDescription => {
            "Die Farbe, die verwendet wird, wenn der Text keine eigene Farbe angibt."
        }
        Text::ComponentBlankSpace => "Leerraum",
        Text::ComponentCurrentComparison => "Aktueller Vergleich",
        Text::ComponentCurrentPace => "Aktuelles Tempo",
        Text::ComponentDelta => "Delta",
        Text::ComponentDetailedTimer => "Detaillierter Timer",
        Text::ComponentGraph => "Graph",
        Text::ComponentPbChance => "PB-Chance",
        Text::ComponentPossibleTimeSave => "Mögliche Zeitersparnis",
        Text::ComponentPreviousSegment => "Vorheriges Segment",
        Text::ComponentSegmentTime => "Segmentzeit",
        Text::ComponentSeparator => "Trenner",
        Text::ComponentSplits => "Splits",
        Text::ComponentSumOfBest => "Summe der Besten",
        Text::ComponentText => "Text",
        Text::ComponentTimer => "Timer",
        Text::ComponentSegmentTimer => "Segment-Timer",
        Text::ComponentTitle => "Titel",
        Text::ComponentTotalPlaytime => "Gesamtspielzeit",
        Text::ComponentCurrentPaceBestPossibleTime => "Bestmögliche Zeit",
        Text::ComponentCurrentPaceWorstPossibleTime => "Schlechtestmögliche Zeit",
        Text::ComponentCurrentPacePredictedTime => "Vorhergesagte Zeit",
        Text::ComponentSegmentTimeBest => "Beste Segmentzeit",
        Text::ComponentSegmentTimeWorst => "Schlechteste Segmentzeit",
        Text::ComponentSegmentTimeAverage => "Durchschnittliche Segmentzeit",
        Text::ComponentSegmentTimeMedian => "Mediane Segmentzeit",
        Text::ComponentSegmentTimeLatest => "Letzte Segmentzeit",
        Text::ComponentPossibleTimeSaveTotal => "Gesamte mögliche Zeitersparnis",
        Text::LiveSegment => "Live-Segment",
        Text::LiveSegmentShort => "Live-Segment",
        Text::PreviousSegmentShort => "Vorh. Segment",
        Text::PreviousSegmentAbbreviation => "Vorh. Seg.",
        Text::ComparingAgainst => "Vergleich mit",
        Text::ComparisonShort => "Vergleich",
        Text::CurrentPaceBestPossibleTimeShort => "Bestmög. Zeit",
        Text::CurrentPaceBestTimeShort => "Beste Zeit",
        Text::CurrentPaceBestPossibleTimeAbbreviation => "BmZ",
        Text::CurrentPaceWorstPossibleTimeShort => "Schlechtestmög. Zeit",
        Text::CurrentPaceWorstTimeShort => "Schlechteste Zeit",
        Text::CurrentPacePredictedTimeShort => "Vorh. Zeit",
        Text::CurrentPaceShort => "Akt. Tempo",
        Text::CurrentPaceAbbreviation => "Tempo",
        Text::Goal => "Ziel",
        Text::SumOfBestSegments => "Summe der besten Segmente",
        Text::SumOfBestShort => "Summe der Besten",
        Text::SumOfBestAbbreviation => "SdB",
        Text::PlaytimeShort => "Spielzeit",
        Text::BestSegmentTimeShort => "Beste Seg.-Zeit",
        Text::BestSegmentShort => "Bestes Segment",
        Text::WorstSegmentTimeShort => "Schlechteste Seg.-Zeit",
        Text::WorstSegmentShort => "Schlechtestes Segment",
        Text::AverageSegmentTimeShort => "Durchschn. Seg.-Zeit",
        Text::AverageSegmentShort => "Durchschnittssegment",
        Text::MedianSegmentTimeShort => "Median-Seg.-Zeit",
        Text::MedianSegmentShort => "Median-Segment",
        Text::LatestSegmentTimeShort => "Letzte Seg.-Zeit",
        Text::LatestSegmentShort => "Letztes Segment",
        Text::SegmentTimeShort => "Seg.-Zeit",
        Text::SplitTime => "Zeit",
        Text::PossibleTimeSaveShort => "Mögliche Zeitersparnis",
        Text::PossibleTimeSaveAbbreviation => "Mögl. Zeitersp.",
        Text::TimeSaveShort => "Zeitersparnis",
        Text::RealTime => "Echtzeit",
        Text::GameTime => "Spielzeit",
        Text::Untitled => "Unbenannt",
        Text::SumOfBestCleanerStartOfRun => "dem Start des Runs",
        Text::SumOfBestCleanerShouldRemove => {
            ". Glaubst du, dass diese Segmentzeit ungenau ist und entfernt werden sollte?"
        }
    }
}

pub const fn resolve_placeholder(text: PlaceholderText) -> &'static [Piece] {
    match text {
        PlaceholderText::SumOfBestCleanerSegmentTimeBetween => &[
            Piece::Static("Du hattest eine "),
            Piece::Dynamic(0),
            Piece::Static("-Segmentzeit von "),
            Piece::Dynamic(1),
            Piece::Static(" zwischen „"),
            Piece::Dynamic(2),
            Piece::Static("“ und „"),
            Piece::Dynamic(3),
            Piece::Static("“"),
        ],
        PlaceholderText::SumOfBestCleanerFasterThanCombined => &[
            Piece::Static(", die schneller ist als die kombinierten Bestsegmente von "),
            Piece::Dynamic(0),
        ],
        PlaceholderText::SumOfBestCleanerRunOn => &[
            Piece::Static(" in einem Run am "),
            Piece::Dynamic(0),
            Piece::Static(", der um "),
            Piece::Dynamic(1),
            Piece::Static(" gestartet wurde"),
        ],
    }
}
