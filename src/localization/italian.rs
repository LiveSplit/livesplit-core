use super::{Piece, PlaceholderText, Text};

pub const fn resolve(text: Text) -> &'static str {
    match text {
        Text::StartSplit => "Avvia / Split",
        Text::StartSplitDescription => {
            "Il tasto ad azione rapida da usare per splittare e avviare un nuovo tentativo."
        }
        Text::Reset => "Reimposta",
        Text::ResetDescription => {
            "Il tasto ad azione rapida da usare per reimpostare il tentativo corrente."
        }
        Text::UndoSplit => "Annulla split",
        Text::UndoSplitDescription => {
            "Il tasto ad azione rapida da usare per annullare l’ultimo split."
        }
        Text::SkipSplit => "Salta split",
        Text::SkipSplitDescription => {
            "Il tasto ad azione rapida da usare per saltare lo split corrente."
        }
        Text::Pause => "Pausa",
        Text::PauseDescription => {
            "Il tasto ad azione rapida da usare per mettere in pausa il tentativo corrente. Può anche essere usato per avviare un nuovo tentativo."
        }
        Text::UndoAllPauses => "Annulla tutte le pause",
        Text::UndoAllPausesDescription => {
            "Il tasto ad azione rapida da usare per rimuovere tutti i tempi di pausa dal tempo corrente. Utile se hai messo in pausa per errore."
        }
        Text::PreviousComparison => "Confronto precedente",
        Text::PreviousComparisonDescription => {
            "Il tasto ad azione rapida da usare per passare al confronto precedente."
        }
        Text::NextComparison => "Confronto successivo",
        Text::NextComparisonDescription => {
            "Il tasto ad azione rapida da usare per passare al confronto successivo."
        }
        Text::ToggleTimingMethod => "Cambia metodo di cronometraggio",
        Text::ToggleTimingMethodDescription => {
            "Il tasto ad azione rapida da usare per alternare tra «Tempo reale» e «Tempo di gioco»."
        }
        Text::TimerBackground => "Sfondo",
        Text::TimerBackgroundDescription => {
            "Lo sfondo mostrato dietro il componente. È anche possibile applicare come sfondo il colore associato al tempo avanti o indietro."
        }
        Text::SegmentTimer => "Timer del segmento",
        Text::SegmentTimerDescription => {
            "Indica se mostrare il tempo trascorso dall’inizio del segmento corrente invece che dall’inizio del tentativo."
        }
        Text::TimingMethod => "Metodo di cronometraggio",
        Text::TimingMethodDescription => {
            "Indica il metodo di cronometraggio da usare. Se non specificato, viene usato il metodo corrente."
        }
        Text::Height => "Altezza",
        Text::HeightDescription => "L’altezza del timer.",
        Text::TimerTextColor => "Colore del testo",
        Text::TimerTextColorDescription => {
            "Il colore del tempo mostrato. Se non specificato, il colore viene scelto automaticamente in base all’andamento del tentativo. Questi colori possono essere impostati nelle impostazioni generali del layout."
        }
        Text::ShowGradient => "Mostra gradiente",
        Text::ShowGradientDescription => {
            "Determina se visualizzare il colore del timer come gradiente."
        }
        Text::DigitsFormat => "Formato cifre",
        Text::DigitsFormatDescription => {
            "Indica quante cifre mostrare. Se la durata è inferiore alle cifre da mostrare, vengono mostrati zeri."
        }
        Text::Accuracy => "Precisione",
        Text::AccuracyDescription => "La precisione del tempo mostrato.",
        Text::TitleBackground => "Sfondo",
        Text::TitleBackgroundDescription => "Lo sfondo mostrato dietro il componente.",
        Text::TitleTextColor => "Colore del testo",
        Text::TitleTextColorDescription => {
            "Il colore del testo del titolo. Se non specificato, viene usato il colore del layout."
        }
        Text::ShowGameName => "Mostra nome del gioco",
        Text::ShowGameNameDescription => {
            "Indica se il nome del gioco deve far parte del titolo mostrato."
        }
        Text::ShowCategoryName => "Mostra nome della categoria",
        Text::ShowCategoryNameDescription => {
            "Indica se il nome della categoria deve far parte del titolo mostrato."
        }
        Text::ShowFinishedRunsCount => "Mostra numero di run completati",
        Text::ShowFinishedRunsCountDescription => {
            "Indica se mostrare il numero di run completati con successo."
        }
        Text::ShowAttemptCount => "Mostra numero di tentativi",
        Text::ShowAttemptCountDescription => "Indica se mostrare il numero totale di tentativi.",
        Text::TextAlignment => "Allineamento del testo",
        Text::TextAlignmentDescription => "Indica l’allineamento del titolo.",
        Text::DisplayTextAsSingleLine => "Mostra testo su una sola riga",
        Text::DisplayTextAsSingleLineDescription => {
            "Indica se il titolo deve essere mostrato su una sola riga invece di essere separato in una riga per il gioco e una per la categoria."
        }
        Text::DisplayGameIcon => "Mostra icona del gioco",
        Text::DisplayGameIconDescription => {
            "Indica se mostrare l’icona del gioco, se presente negli split."
        }
        Text::ShowRegion => "Mostra regione",
        Text::ShowRegionDescription => {
            "Il nome della categoria può essere esteso con informazioni aggiuntive. Questo lo estende con la regione del gioco, se fornita nella scheda variabili dell’editor degli split."
        }
        Text::ShowPlatform => "Mostra piattaforma",
        Text::ShowPlatformDescription => {
            "Il nome della categoria può essere esteso con informazioni aggiuntive. Questo lo estende con la piattaforma su cui si gioca, se fornita nella scheda variabili dell’editor degli split."
        }
        Text::ShowVariables => "Mostra variabili",
        Text::ShowVariablesDescription => {
            "Il nome della categoria può essere esteso con informazioni aggiuntive. Questo lo estende con variabili aggiuntive fornite nella scheda variabili dell’editor degli split. Si riferisce alle variabili di speedrun.com, non alle variabili personalizzate."
        }
        Text::TotalPlaytimeBackground => "Sfondo",
        Text::TotalPlaytimeBackgroundDescription => "Lo sfondo mostrato dietro il componente.",
        Text::DisplayTwoRows => "Mostra 2 righe",
        Text::DisplayTwoRowsDescription => {
            "Indica se mostrare il nome del componente e il tempo di gioco totale su due righe separate."
        }
        Text::ShowDays => "Mostra giorni (>24h)",
        Text::ShowDaysDescription => {
            "Indica se mostrare il numero di giorni quando il tempo di gioco totale raggiunge 24 ore o più."
        }
        Text::LabelColor => "Colore etichetta",
        Text::LabelColorDescription => {
            "Il colore del nome del componente. Se non specificato, viene usato il colore del layout."
        }
        Text::ValueColor => "Colore valore",
        Text::ValueColorDescription => {
            "Il colore del tempo di gioco totale. Se non specificato, viene usato il colore del layout."
        }
        Text::BlankSpaceBackground => "Sfondo",
        Text::BlankSpaceBackgroundDescription => "Lo sfondo mostrato dietro il componente.",
        Text::BlankSpaceSize => "Dimensione",
        Text::BlankSpaceSizeDescription => "La dimensione del componente.",
        Text::CurrentComparisonBackground => "Sfondo",
        Text::CurrentComparisonBackgroundDescription => "Lo sfondo mostrato dietro il componente.",
        Text::CurrentComparisonDisplayTwoRows => "Mostra 2 righe",
        Text::CurrentComparisonDisplayTwoRowsDescription => {
            "Indica se mostrare il nome del componente e il confronto su due righe separate."
        }
        Text::CurrentComparisonLabelColor => "Colore etichetta",
        Text::CurrentComparisonLabelColorDescription => {
            "Il colore del nome del componente. Se non specificato, viene usato il colore del layout."
        }
        Text::CurrentComparisonValueColor => "Colore valore",
        Text::CurrentComparisonValueColorDescription => {
            "Il colore del nome del confronto. Se non specificato, viene usato il colore del layout."
        }
        Text::CurrentPaceBackground => "Sfondo",
        Text::CurrentPaceBackgroundDescription => "Lo sfondo mostrato dietro il componente.",
        Text::CurrentPaceComparison => "Confronto",
        Text::CurrentPaceComparisonDescription => {
            "Il confronto da cui prevedere il tempo finale. Se non specificato, viene usato il confronto corrente."
        }
        Text::CurrentPaceDisplayTwoRows => "Mostra 2 righe",
        Text::CurrentPaceDisplayTwoRowsDescription => {
            "Indica se mostrare il nome del componente e il tempo previsto su due righe separate."
        }
        Text::CurrentPaceLabelColor => "Colore etichetta",
        Text::CurrentPaceLabelColorDescription => {
            "Il colore del nome del componente. Se non specificato, viene usato il colore del layout."
        }
        Text::CurrentPaceValueColor => "Colore valore",
        Text::CurrentPaceValueColorDescription => {
            "Il colore del tempo previsto. Se non specificato, viene usato il colore del layout."
        }
        Text::CurrentPaceAccuracy => "Precisione",
        Text::CurrentPaceAccuracyDescription => "La precisione del tempo previsto mostrato.",
        Text::DeltaBackground => "Sfondo",
        Text::DeltaBackgroundDescription => "Lo sfondo mostrato dietro il componente.",
        Text::DeltaComparison => "Confronto",
        Text::DeltaComparisonDescription => {
            "Il confronto usato per calcolare quanto si è avanti o indietro. Se non specificato, viene usato il confronto corrente."
        }
        Text::DeltaDisplayTwoRows => "Mostra 2 righe",
        Text::DeltaDisplayTwoRowsDescription => {
            "Indica se mostrare il nome del confronto e il delta su due righe separate."
        }
        Text::DeltaLabelColor => "Colore etichetta",
        Text::DeltaLabelColorDescription => {
            "Il colore del nome del confronto. Se non specificato, viene usato il colore del layout."
        }
        Text::DeltaDropDecimals => "Rimuovi decimali",
        Text::DeltaDropDecimalsDescription => {
            "Indica se i decimali devono essere nascosti quando il delta visualizzato supera un minuto."
        }
        Text::DeltaAccuracy => "Precisione",
        Text::DeltaAccuracyDescription => "La precisione del delta mostrato.",
        Text::SumOfBestBackground => "Sfondo",
        Text::SumOfBestBackgroundDescription => "Lo sfondo mostrato dietro il componente.",
        Text::SumOfBestDisplayTwoRows => "Mostra 2 righe",
        Text::SumOfBestDisplayTwoRowsDescription => {
            "Indica se mostrare il nome del componente e la somma dei migliori segmenti su due righe separate."
        }
        Text::SumOfBestLabelColor => "Colore etichetta",
        Text::SumOfBestLabelColorDescription => {
            "Il colore del nome del componente. Se non specificato, viene usato il colore del layout."
        }
        Text::SumOfBestValueColor => "Colore valore",
        Text::SumOfBestValueColorDescription => {
            "Il colore della somma dei migliori segmenti. Se non specificato, viene usato il colore del layout."
        }
        Text::SumOfBestAccuracy => "Precisione",
        Text::SumOfBestAccuracyDescription => {
            "La precisione della somma dei migliori segmenti mostrata."
        }
        Text::PbChanceBackground => "Sfondo",
        Text::PbChanceBackgroundDescription => "Lo sfondo mostrato dietro il componente.",
        Text::PbChanceDisplayTwoRows => "Mostra 2 righe",
        Text::PbChanceDisplayTwoRowsDescription => {
            "Indica se mostrare il nome del componente e la probabilità di PB su due righe separate."
        }
        Text::PbChanceLabelColor => "Colore etichetta",
        Text::PbChanceLabelColorDescription => {
            "Il colore del nome del componente. Se non specificato, viene usato il colore del layout."
        }
        Text::PbChanceValueColor => "Colore valore",
        Text::PbChanceValueColorDescription => {
            "Il colore della probabilità di PB. Se non specificato, viene usato il colore del layout."
        }
        Text::PossibleTimeSaveBackground => "Sfondo",
        Text::PossibleTimeSaveBackgroundDescription => "Lo sfondo mostrato dietro il componente.",
        Text::PossibleTimeSaveComparison => "Confronto",
        Text::PossibleTimeSaveComparisonDescription => {
            "Il confronto per calcolare il tempo potenzialmente risparmiabile. Se non specificato, viene usato il confronto corrente."
        }
        Text::PossibleTimeSaveDisplayTwoRows => "Mostra 2 righe",
        Text::PossibleTimeSaveDisplayTwoRowsDescription => {
            "Indica se mostrare il nome del componente e il tempo potenzialmente risparmiabile su due righe separate."
        }
        Text::PossibleTimeSaveShowTotal => "Mostra risparmio totale possibile",
        Text::PossibleTimeSaveShowTotalDescription => {
            "Indica se mostrare il risparmio totale possibile per il resto del tentativo invece di quello del segmento corrente."
        }
        Text::PossibleTimeSaveLabelColor => "Colore etichetta",
        Text::PossibleTimeSaveLabelColorDescription => {
            "Il colore del nome del componente. Se non specificato, viene usato il colore del layout."
        }
        Text::PossibleTimeSaveValueColor => "Colore valore",
        Text::PossibleTimeSaveValueColorDescription => {
            "Il colore del tempo potenzialmente risparmiabile. Se non specificato, viene usato il colore del layout."
        }
        Text::PossibleTimeSaveAccuracy => "Precisione",
        Text::PossibleTimeSaveAccuracyDescription => {
            "La precisione del tempo potenzialmente risparmiabile mostrato."
        }
        Text::PreviousSegmentBackground => "Sfondo",
        Text::PreviousSegmentBackgroundDescription => "Lo sfondo mostrato dietro il componente.",
        Text::PreviousSegmentComparison => "Confronto",
        Text::PreviousSegmentComparisonDescription => {
            "Il confronto usato per calcolare quanto tempo è stato risparmiato o perso. Se non specificato, viene usato il confronto corrente."
        }
        Text::PreviousSegmentDisplayTwoRows => "Mostra 2 righe",
        Text::PreviousSegmentDisplayTwoRowsDescription => {
            "Indica se mostrare il nome del componente e il tempo risparmiato o perso su due righe separate."
        }
        Text::PreviousSegmentLabelColor => "Colore etichetta",
        Text::PreviousSegmentLabelColorDescription => {
            "Il colore del nome del componente. Se non specificato, viene usato il colore del layout."
        }
        Text::PreviousSegmentDropDecimals => "Rimuovi decimali",
        Text::PreviousSegmentDropDecimalsDescription => {
            "Indica se rimuovere i decimali quando il tempo mostrato supera un minuto."
        }
        Text::PreviousSegmentAccuracy => "Precisione",
        Text::PreviousSegmentAccuracyDescription => "La precisione del tempo mostrato.",
        Text::PreviousSegmentShowPossibleTimeSave => "Mostra tempo potenzialmente risparmiabile",
        Text::PreviousSegmentShowPossibleTimeSaveDescription => {
            "Indica se mostrare, oltre al tempo risparmiato o perso, quanto tempo si sarebbe potuto risparmiare nel segmento precedente."
        }
        Text::SegmentTimeBackground => "Sfondo",
        Text::SegmentTimeBackgroundDescription => "Lo sfondo mostrato dietro il componente.",
        Text::SegmentTimeComparison => "Confronto",
        Text::SegmentTimeComparisonDescription => {
            "Il confronto per il tempo di segmento. Se non specificato, viene usato il confronto corrente."
        }
        Text::SegmentTimeDisplayTwoRows => "Mostra 2 righe",
        Text::SegmentTimeDisplayTwoRowsDescription => {
            "Indica se mostrare il nome del componente e il tempo di segmento su due righe separate."
        }
        Text::SegmentTimeLabelColor => "Colore etichetta",
        Text::SegmentTimeLabelColorDescription => {
            "Il colore del nome del componente. Se non specificato, viene usato il colore del layout."
        }
        Text::SegmentTimeValueColor => "Colore valore",
        Text::SegmentTimeValueColorDescription => {
            "Il colore del tempo di segmento. Se non specificato, viene usato il colore del layout."
        }
        Text::SegmentTimeAccuracy => "Precisione",
        Text::SegmentTimeAccuracyDescription => "La precisione del tempo di segmento mostrato.",
        Text::GraphComparison => "Confronto",
        Text::GraphComparisonDescription => {
            "Il confronto da usare per il grafico. Se non specificato, viene usato il confronto corrente."
        }
        Text::GraphHeight => "Altezza",
        Text::GraphHeightDescription => "L’altezza del grafico.",
        Text::GraphShowBestSegments => "Mostra migliori segmenti",
        Text::GraphShowBestSegmentsDescription => {
            "Indica se colorare i migliori segmenti con il colore del miglior segmento del layout."
        }
        Text::GraphLiveGraph => "Grafico in tempo reale",
        Text::GraphLiveGraphDescription => {
            "Indica se il grafico deve aggiornarsi continuamente. Se disattivato, i cambiamenti avvengono solo quando cambia il segmento corrente."
        }
        Text::GraphFlipGraph => "Inverti grafico",
        Text::GraphFlipGraphDescription => {
            "Indica se il grafico deve essere capovolto verticalmente. Se non attivo, i tempi avanti sono sotto l’asse x e quelli indietro sopra."
        }
        Text::GraphBehindBackgroundColor => "Colore sfondo (indietro)",
        Text::GraphBehindBackgroundColorDescription => {
            "Il colore di sfondo per l’area del grafico contenente i tempi dietro al confronto."
        }
        Text::GraphAheadBackgroundColor => "Colore sfondo (avanti)",
        Text::GraphAheadBackgroundColorDescription => {
            "Il colore di sfondo per l’area del grafico contenente i tempi avanti rispetto al confronto."
        }
        Text::GraphGridLinesColor => "Colore griglia",
        Text::GraphGridLinesColorDescription => "Il colore delle linee della griglia del grafico.",
        Text::GraphLinesColor => "Colore linee del grafico",
        Text::GraphLinesColorDescription => {
            "Il colore delle linee che collegano i punti del grafico."
        }
        Text::GraphPartialFillColor => "Colore riempimento parziale",
        Text::GraphPartialFillColorDescription => {
            "Il colore dell’area tra l’asse x e il grafico. Il riempimento parziale è usato solo per i cambiamenti live, dall’ultimo split al tempo attuale."
        }
        Text::GraphCompleteFillColor => "Colore riempimento completo",
        Text::GraphCompleteFillColorDescription => {
            "Il colore dell’area tra l’asse x e il grafico, escludendo il segmento con cambiamenti live."
        }
        Text::DetailedTimerBackground => "Sfondo",
        Text::DetailedTimerBackgroundDescription => "Lo sfondo mostrato dietro il componente.",
        Text::DetailedTimerTimingMethod => "Metodo di cronometraggio",
        Text::DetailedTimerTimingMethodDescription => {
            "Indica il metodo di cronometraggio da usare. Se non specificato, viene usato il metodo corrente."
        }
        Text::DetailedTimerComparison1 => "Confronto 1",
        Text::DetailedTimerComparison1Description => {
            "Il primo confronto di cui mostrare il tempo di segmento. Se non specificato, viene usato il confronto corrente."
        }
        Text::DetailedTimerComparison2 => "Confronto 2",
        Text::DetailedTimerComparison2Description => {
            "Il secondo confronto di cui mostrare il tempo di segmento. Se non specificato, viene usato il confronto corrente, a meno che il primo confronto sia anch’esso None. Non viene mostrato se il secondo confronto è nascosto."
        }
        Text::DetailedTimerHideSecondComparison => "Nascondi secondo confronto",
        Text::DetailedTimerHideSecondComparisonDescription => {
            "Indica se mostrare solo un confronto."
        }
        Text::DetailedTimerTimerHeight => "Altezza timer",
        Text::DetailedTimerTimerHeightDescription => "L’altezza del timer principale.",
        Text::DetailedTimerSegmentTimerHeight => "Altezza timer segmento",
        Text::DetailedTimerSegmentTimerHeightDescription => "L’altezza del timer di segmento.",
        Text::DetailedTimerTimerColor => "Colore timer",
        Text::DetailedTimerTimerColorDescription => {
            "Invece di determinare automaticamente il colore del timer principale in base all’andamento, è possibile fornire un colore fisso."
        }
        Text::DetailedTimerShowTimerGradient => "Mostra gradiente del timer",
        Text::DetailedTimerShowTimerGradientDescription => {
            "Il timer principale trasforma automaticamente il suo colore in un gradiente verticale se questa opzione è attiva. Altrimenti viene usato il colore reale."
        }
        Text::DetailedTimerTimerDigitsFormat => "Formato cifre del timer",
        Text::DetailedTimerTimerDigitsFormatDescription => {
            "Indica quante cifre mostrare per il timer principale. Se la durata è inferiore, vengono mostrati zeri."
        }
        Text::DetailedTimerTimerAccuracy => "Precisione del timer",
        Text::DetailedTimerTimerAccuracyDescription => {
            "La precisione del tempo mostrato per il timer principale."
        }
        Text::DetailedTimerSegmentTimerColor => "Colore timer segmento",
        Text::DetailedTimerSegmentTimerColorDescription => {
            "Cambia il colore del timer di segmento con un colore diverso da quello predefinito."
        }
        Text::DetailedTimerShowSegmentTimerGradient => "Mostra gradiente del timer segmento",
        Text::DetailedTimerShowSegmentTimerGradientDescription => {
            "Il timer di segmento trasforma automaticamente il suo colore in un gradiente verticale se questa opzione è attiva. Altrimenti viene usato il colore reale."
        }
        Text::DetailedTimerSegmentTimerDigitsFormat => "Formato cifre timer segmento",
        Text::DetailedTimerSegmentTimerDigitsFormatDescription => {
            "Indica quante cifre mostrare per il timer di segmento. Se la durata è inferiore, vengono mostrati zeri."
        }
        Text::DetailedTimerSegmentTimerAccuracy => "Precisione del timer segmento",
        Text::DetailedTimerSegmentTimerAccuracyDescription => {
            "La precisione del tempo mostrato per il timer di segmento."
        }
        Text::DetailedTimerComparisonNamesColor => "Colore nomi confronto",
        Text::DetailedTimerComparisonNamesColorDescription => {
            "Il colore dei nomi dei confronti se sono mostrati. Se non specificato, viene usato il colore del layout."
        }
        Text::DetailedTimerComparisonTimesColor => "Colore tempi confronto",
        Text::DetailedTimerComparisonTimesColorDescription => {
            "Il colore dei tempi dei confronti se sono mostrati. Se non specificato, viene usato il colore del layout."
        }
        Text::DetailedTimerComparisonTimesAccuracy => "Precisione tempi confronto",
        Text::DetailedTimerComparisonTimesAccuracyDescription => {
            "La precisione dei tempi di confronto."
        }
        Text::DetailedTimerShowSegmentName => "Mostra nome segmento",
        Text::DetailedTimerShowSegmentNameDescription => "Indica se mostrare il nome del segmento.",
        Text::DetailedTimerSegmentNameColor => "Colore nome segmento",
        Text::DetailedTimerSegmentNameColorDescription => {
            "Il colore del nome del segmento se è mostrato. Se non specificato, viene usato il colore del layout."
        }
        Text::DetailedTimerDisplayIcon => "Mostra icona",
        Text::DetailedTimerDisplayIconDescription => "Indica se mostrare l’icona del segmento.",
        Text::SplitsBackground => "Sfondo",
        Text::SplitsBackgroundDescription => {
            "Lo sfondo mostrato dietro il componente. È possibile scegliere colori alternati; in tal caso ogni riga alterna tra i due colori scelti."
        }
        Text::SplitsTotalRows => "Righe totali",
        Text::SplitsTotalRowsDescription => {
            "Il numero totale di righe di segmenti da mostrare. Se impostato a 0, vengono mostrati tutti i segmenti. Se inferiore al totale, viene mostrata una finestra scorrevole."
        }
        Text::SplitsUpcomingSegments => "Segmenti imminenti",
        Text::SplitsUpcomingSegmentsDescription => {
            "Se ci sono più segmenti delle righe mostrate, la finestra scorre automaticamente quando cambia il segmento corrente. Questo numero determina il minimo di segmenti futuri visibili."
        }
        Text::SplitsShowThinSeparators => "Mostra separatori sottili",
        Text::SplitsShowThinSeparatorsDescription => {
            "Indica se mostrare separatori sottili tra le righe dei segmenti."
        }
        Text::SplitsShowSeparatorBeforeLastSplit => "Mostra separatore prima dell’ultimo split",
        Text::SplitsShowSeparatorBeforeLastSplitDescription => {
            "Se l’ultimo segmento deve essere sempre mostrato, questa opzione determina se mostrare un separatore più marcato prima dell’ultimo segmento quando non è adiacente al precedente nella finestra scorrevole."
        }
        Text::SplitsAlwaysShowLastSplit => "Mostra sempre l’ultimo split",
        Text::SplitsAlwaysShowLastSplitDescription => {
            "Se non tutti i segmenti sono mostrati, questa opzione determina se l’ultimo segmento deve essere sempre visibile, poiché contiene la durata totale del confronto scelto."
        }
        Text::SplitsFillWithBlankSpace => "Riempi con spazio vuoto",
        Text::SplitsFillWithBlankSpaceDescription => {
            "Se non ci sono abbastanza segmenti per riempire la lista, questa opzione consente di riempire le righe rimanenti con spazio vuoto per mantenere il numero totale di righe."
        }
        Text::SplitsShowTimesBelowSegmentName => "Mostra tempi sotto il nome del segmento",
        Text::SplitsShowTimesBelowSegmentNameDescription => {
            "Indica se mostrare i tempi sotto il nome del segmento. In caso contrario, i tempi vengono mostrati accanto al nome."
        }
        Text::SplitsCurrentSegmentGradient => "Gradiente del segmento corrente",
        Text::SplitsCurrentSegmentGradientDescription => {
            "Il gradiente da mostrare dietro al segmento corrente come indicatore."
        }
        Text::SplitsSplitTimeAccuracy => "Precisione tempi split",
        Text::SplitsSplitTimeAccuracyDescription => {
            "Indica la precisione da usare per le colonne che contengono tempi split."
        }
        Text::SplitsSegmentTimeAccuracy => "Precisione tempi segmento",
        Text::SplitsSegmentTimeAccuracyDescription => {
            "Indica la precisione da usare per le colonne che contengono tempi di segmento."
        }
        Text::SplitsDeltaTimeAccuracy => "Precisione delta",
        Text::SplitsDeltaTimeAccuracyDescription => {
            "Indica la precisione da usare per le colonne che contengono il tempo avanti o indietro."
        }
        Text::SplitsDropDeltaDecimals => "Nascondi decimali del delta quando si mostrano i minuti",
        Text::SplitsDropDeltaDecimalsDescription => {
            "Indica se i decimali non devono più essere mostrati quando una colonna di delta supera un minuto."
        }
        Text::SplitsShowColumnLabels => "Mostra etichette colonne",
        Text::SplitsShowColumnLabelsDescription => {
            "Indica se mostrare i nomi delle colonne in cima alla lista."
        }
        Text::SplitsColumns => "Colonne",
        Text::SplitsColumnsDescription => {
            "Il numero di colonne da mostrare in ogni riga. Ogni colonna può mostrare informazioni diverse. Le colonne sono definite da destra a sinistra."
        }
        Text::SplitsColumnName => "Nome colonna",
        Text::SplitsColumnNameDescription => {
            "Il nome della colonna. Viene mostrato in cima alla lista se le etichette delle colonne sono abilitate."
        }
        Text::SplitsColumnType => "Tipo di colonna",
        Text::SplitsColumnTypeDescription => {
            "Il tipo di informazioni che questa colonna mostra. Può essere un tempo o una variabile personalizzata."
        }
        Text::SplitsVariableName => "Nome variabile",
        Text::SplitsVariableNameDescription => {
            "Il nome della variabile personalizzata mostrata da questa colonna."
        }
        Text::SplitsStartWith => "Inizia con",
        Text::SplitsStartWithDescription => {
            "Il valore con cui questa colonna inizia per ogni segmento. Il trigger di aggiornamento determina quando viene sostituito."
        }
        Text::SplitsUpdateWith => "Aggiorna con",
        Text::SplitsUpdateWithDescription => {
            "Quando una certa condizione è soddisfatta, di solito essere sul segmento o averlo completato, il tempo viene aggiornato con il valore indicato qui."
        }
        Text::SplitsUpdateTrigger => "Trigger di aggiornamento",
        Text::SplitsUpdateTriggerDescription => {
            "La condizione che deve essere soddisfatta per aggiornare il valore con quello indicato in «Aggiorna con». Prima di ciò, il valore è quello di «Inizia con»."
        }
        Text::SplitsColumnComparison => "Confronto",
        Text::SplitsColumnComparisonDescription => {
            "Il confronto rispetto al quale questa colonna viene confrontata. Se non specificato, viene usato il confronto corrente."
        }
        Text::SplitsColumnTimingMethod => "Metodo di cronometraggio",
        Text::SplitsColumnTimingMethodDescription => {
            "Indica il metodo di cronometraggio da usare per questa colonna. Se non specificato, viene usato il metodo corrente."
        }
        Text::TextComponentBackground => "Sfondo",
        Text::TextComponentBackgroundDescription => "Lo sfondo mostrato dietro il componente.",
        Text::TextComponentUseVariable => "Usa variabile",
        Text::TextComponentUseVariableDescription => {
            "Indica se usare una variabile personalizzata per mostrare un valore dinamico. Le variabili personalizzate possono essere impostate nell’editor degli split e fornite dagli auto splitter."
        }
        Text::TextComponentSplit => "Dividi",
        Text::TextComponentSplitDescription => {
            "Indica se dividere il testo in una parte sinistra e una destra. Se non è così, viene mostrato un unico testo centrato."
        }
        Text::TextComponentText => "Testo",
        Text::TextComponentTextDescription => "Indica il testo da mostrare al centro.",
        Text::TextComponentLeft => "Sinistra",
        Text::TextComponentLeftDescription => "Indica il testo da mostrare a sinistra.",
        Text::TextComponentRight => "Destra",
        Text::TextComponentRightDescription => "Indica il testo da mostrare a destra.",
        Text::TextComponentVariable => "Variabile",
        Text::TextComponentVariableDescription => {
            "Indica il nome della variabile personalizzata da mostrare."
        }
        Text::TextComponentTextColor => "Colore del testo",
        Text::TextComponentTextColorDescription => "Il colore del testo.",
        Text::TextComponentLeftColor => "Colore sinistro",
        Text::TextComponentLeftColorDescription => "Il colore del testo a sinistra.",
        Text::TextComponentRightColor => "Colore destro",
        Text::TextComponentRightColorDescription => "Il colore del testo a destra.",
        Text::TextComponentNameColor => "Colore del nome",
        Text::TextComponentNameColorDescription => "Il colore del nome della variabile.",
        Text::TextComponentValueColor => "Colore valore",
        Text::TextComponentValueColorDescription => "Il colore del valore della variabile.",
        Text::TextComponentDisplayTwoRows => "Mostra 2 righe",
        Text::TextComponentDisplayTwoRowsDescription => {
            "Indica se il testo sinistro e destro devono essere mostrati su due righe separate."
        }
        Text::LayoutDirection => "Direzione del layout",
        Text::LayoutDirectionDescription => "La direzione in cui i componenti sono disposti.",
        Text::CustomTimerFont => "Font personalizzato del timer",
        Text::CustomTimerFontDescription => {
            "Consente di specificare un font personalizzato per il timer. Se non impostato, viene usato il font predefinito."
        }
        Text::CustomTimesFont => "Font personalizzato dei tempi",
        Text::CustomTimesFontDescription => {
            "Consente di specificare un font personalizzato per i tempi. Se non impostato, viene usato il font predefinito."
        }
        Text::CustomTextFont => "Font personalizzato del testo",
        Text::CustomTextFontDescription => {
            "Consente di specificare un font personalizzato per il testo. Se non impostato, viene usato il font predefinito."
        }
        Text::TextShadow => "Ombra del testo",
        Text::TextShadowDescription => {
            "Consente di specificare opzionalmente un colore per le ombre del testo."
        }
        Text::Background => "Sfondo",
        Text::BackgroundDescription => "Lo sfondo mostrato dietro l’intero layout.",
        Text::BestSegment => "Miglior segmento",
        Text::BestSegmentDescription => {
            "Il colore da usare quando ottieni un nuovo miglior segmento."
        }
        Text::AheadGainingTime => "Avanti (guadagnando tempo)",
        Text::AheadGainingTimeDescription => {
            "Il colore da usare quando sei avanti e stai guadagnando ancora più tempo."
        }
        Text::AheadLosingTime => "Avanti (perdendo tempo)",
        Text::AheadLosingTimeDescription => {
            "Il colore da usare quando sei avanti ma stai perdendo tempo."
        }
        Text::BehindGainingTime => "Indietro (recuperando tempo)",
        Text::BehindGainingTimeDescription => {
            "Il colore da usare quando sei indietro ma stai recuperando tempo."
        }
        Text::BehindLosingTime => "Indietro (perdendo tempo)",
        Text::BehindLosingTimeDescription => {
            "Il colore da usare quando sei indietro e stai perdendo ancora più tempo."
        }
        Text::NotRunning => "Non in esecuzione",
        Text::NotRunningDescription => "Il colore da usare quando non c’è un tentativo attivo.",
        Text::PersonalBest => "Miglior tempo personale",
        Text::PersonalBestDescription => {
            "Il colore da usare quando ottieni un nuovo miglior tempo personale."
        }
        Text::Paused => "In pausa",
        Text::PausedDescription => "Il colore da usare quando il timer è in pausa.",
        Text::ThinSeparators => "Separatori sottili",
        Text::ThinSeparatorsDescription => "Il colore dei separatori sottili.",
        Text::Separators => "Separatori",
        Text::SeparatorsDescription => "Il colore dei separatori normali.",
        Text::TextColor => "Testo",
        Text::TextColorDescription => {
            "Il colore da usare per il testo che non specifica il proprio colore."
        }
        Text::ComponentBlankSpace => "Spazio vuoto",
        Text::ComponentCurrentComparison => "Confronto attuale",
        Text::ComponentCurrentPace => "Pace attuale",
        Text::ComponentDelta => "Delta",
        Text::ComponentDetailedTimer => "Timer dettagliato",
        Text::ComponentGraph => "Grafico",
        Text::ComponentPbChance => "Probabilità PB",
        Text::ComponentPossibleTimeSave => "Tempo risparmiabile",
        Text::ComponentPreviousSegment => "Segmento precedente",
        Text::ComponentSegmentTime => "Tempo segmento",
        Text::ComponentSeparator => "Separatore",
        Text::ComponentSplits => "Splits",
        Text::ComponentSumOfBest => "Somma dei migliori",
        Text::ComponentText => "Testo",
        Text::ComponentTimer => "Timer",
        Text::ComponentSegmentTimer => "Timer del segmento",
        Text::ComponentTitle => "Titolo",
        Text::ComponentTotalPlaytime => "Tempo di gioco totale",
        Text::ComponentCurrentPaceBestPossibleTime => "Miglior tempo possibile",
        Text::ComponentCurrentPaceWorstPossibleTime => "Peggior tempo possibile",
        Text::ComponentCurrentPacePredictedTime => "Tempo previsto",
        Text::ComponentSegmentTimeBest => "Miglior tempo di segmento",
        Text::ComponentSegmentTimeWorst => "Peggior tempo di segmento",
        Text::ComponentSegmentTimeAverage => "Tempo medio di segmento",
        Text::ComponentSegmentTimeMedian => "Tempo mediano di segmento",
        Text::ComponentSegmentTimeLatest => "Ultimo tempo di segmento",
        Text::ComponentPossibleTimeSaveTotal => "Tempo totale possibile da risparmiare",
        Text::LiveSegment => "Segmento live",
        Text::LiveSegmentShort => "Segmento live",
        Text::PreviousSegmentShort => "Seg. precedente",
        Text::PreviousSegmentAbbreviation => "Seg. prec.",
        Text::ComparingAgainst => "Confronto con",
        Text::ComparisonShort => "Confronto",
        Text::CurrentPaceBestPossibleTimeShort => "Migl. tempo poss.",
        Text::CurrentPaceBestTimeShort => "Migl. tempo",
        Text::CurrentPaceBestPossibleTimeAbbreviation => "MTP",
        Text::CurrentPaceWorstPossibleTimeShort => "Peggior tempo poss.",
        Text::CurrentPaceWorstTimeShort => "Peggior tempo",
        Text::CurrentPacePredictedTimeShort => "Tempo prev.",
        Text::CurrentPaceShort => "Ritmo att.",
        Text::CurrentPaceAbbreviation => "Ritmo",
        Text::Goal => "Obiettivo",
        Text::SumOfBestSegments => "Somma dei migliori segmenti",
        Text::SumOfBestShort => "Somma dei migliori",
        Text::SumOfBestAbbreviation => "SdM",
        Text::PlaytimeShort => "Tempo di gioco",
        Text::BestSegmentTimeShort => "Miglior t. seg.",
        Text::BestSegmentShort => "Miglior segmento",
        Text::WorstSegmentTimeShort => "Peggior t. seg.",
        Text::WorstSegmentShort => "Peggior segmento",
        Text::AverageSegmentTimeShort => "T. seg. medio",
        Text::AverageSegmentShort => "Segmento medio",
        Text::MedianSegmentTimeShort => "T. seg. med.",
        Text::MedianSegmentShort => "Segmento mediano",
        Text::LatestSegmentTimeShort => "Ult. t. seg.",
        Text::LatestSegmentShort => "Ultimo segmento",
        Text::SegmentTimeShort => "T. seg.",
        Text::SplitTime => "Tempo",
        Text::PossibleTimeSaveShort => "Tempo possibile da risparmiare",
        Text::PossibleTimeSaveAbbreviation => "T. poss. da risp.",
        Text::TimeSaveShort => "Tempo da risp.",
        Text::RealTime => "Tempo reale",
        Text::GameTime => "Tempo di gioco",
        Text::Untitled => "Senza titolo",
        Text::SumOfBestCleanerStartOfRun => "l'inizio della run",
        Text::SumOfBestCleanerShouldRemove => {
            ". Pensi che questo tempo di segmento sia impreciso e debba essere rimosso?"
        }
    }
}

pub const fn resolve_placeholder(text: PlaceholderText) -> &'static [Piece] {
    match text {
        PlaceholderText::SumOfBestCleanerSegmentTimeBetween => &[
            Piece::Static("Hai avuto un tempo di segmento "),
            Piece::Dynamic(0),
            Piece::Static(" di "),
            Piece::Dynamic(1),
            Piece::Static(" tra «"),
            Piece::Dynamic(2),
            Piece::Static("» e «"),
            Piece::Dynamic(3),
            Piece::Static("»"),
        ],
        PlaceholderText::SumOfBestCleanerFasterThanCombined => &[
            Piece::Static(", che è più veloce dei migliori segmenti combinati di "),
            Piece::Dynamic(0),
        ],
        PlaceholderText::SumOfBestCleanerRunOn => &[
            Piece::Static(" in una run del "),
            Piece::Dynamic(0),
            Piece::Static(" iniziata alle "),
            Piece::Dynamic(1),
        ],
    }
}
