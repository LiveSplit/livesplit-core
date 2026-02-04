use super::{Piece, PlaceholderText, Text};

pub const fn resolve(text: Text) -> &'static str {
    match text {
        Text::StartSplit => "Start / Split",
        Text::StartSplitDescription => {
            "De sneltoets om te splitten en een nieuwe poging te starten."
        }
        Text::Reset => "Reset",
        Text::ResetDescription => "De sneltoets om de huidige poging te resetten.",
        Text::UndoSplit => "Split ongedaan maken",
        Text::UndoSplitDescription => "De sneltoets om de laatste split ongedaan te maken.",
        Text::SkipSplit => "Split overslaan",
        Text::SkipSplitDescription => "De sneltoets om de huidige split over te slaan.",
        Text::Pause => "Pauze",
        Text::PauseDescription => {
            "De sneltoets om de huidige poging te pauzeren. Deze kan ook worden gebruikt om een nieuwe poging te starten."
        }
        Text::UndoAllPauses => "Alle pauzes ongedaan maken",
        Text::UndoAllPausesDescription => {
            "De sneltoets om alle pauzetijden uit de huidige tijd te verwijderen. Handig als je per ongeluk hebt gepauzeerd."
        }
        Text::PreviousComparison => "Vorige vergelijking",
        Text::PreviousComparisonDescription => {
            "De sneltoets om naar de vorige vergelijking te gaan."
        }
        Text::NextComparison => "Volgende vergelijking",
        Text::NextComparisonDescription => "De sneltoets om naar de volgende vergelijking te gaan.",
        Text::ToggleTimingMethod => "Timingmethode wisselen",
        Text::ToggleTimingMethodDescription => {
            "De sneltoets om te wisselen tussen «Realtime» en «Speltijd»."
        }
        Text::TimerBackground => "Achtergrond",
        Text::TimerBackgroundDescription => {
            "De achtergrond achter het component. Je kunt ook de kleur voor voor- of achterstand als achtergrondkleur gebruiken."
        }
        Text::SegmentTimer => "Segmenttimer",
        Text::SegmentTimerDescription => {
            "Geeft aan of de tijd sinds het begin van het huidige segment moet worden getoond in plaats van sinds het begin van de poging."
        }
        Text::TimingMethod => "Timingmethode",
        Text::TimingMethodDescription => {
            "Geeft de timingmethode aan. Als deze niet is opgegeven, wordt de huidige methode gebruikt."
        }
        Text::Height => "Hoogte",
        Text::HeightDescription => "De hoogte van de timer.",
        Text::TimerTextColor => "Tekstkleur",
        Text::TimerTextColorDescription => {
            "De kleur van de getoonde tijd. Als dit niet is opgegeven, wordt de kleur automatisch gekozen op basis van hoe de poging verloopt. Deze kleuren kun je instellen in de algemene layout-instellingen."
        }
        Text::ShowGradient => "Verloop tonen",
        Text::ShowGradientDescription => {
            "Bepaalt of de kleur van de timer als verloop wordt weergegeven."
        }
        Text::DigitsFormat => "Cijferformaat",
        Text::DigitsFormatDescription => {
            "Geeft aan hoeveel cijfers worden getoond. Als de duur lager is dan het aantal cijfers, worden nullen getoond."
        }
        Text::Accuracy => "Nauwkeurigheid",
        Text::AccuracyDescription => "De nauwkeurigheid van de getoonde tijd.",
        Text::TitleBackground => "Achtergrond",
        Text::TitleBackgroundDescription => "De achtergrond achter het component.",
        Text::TitleTextColor => "Tekstkleur",
        Text::TitleTextColorDescription => {
            "De kleur van de titeltekst. Als er geen kleur is opgegeven, wordt de lay-outkleur gebruikt."
        }
        Text::ShowGameName => "Spelnaam tonen",
        Text::ShowGameNameDescription => {
            "Geeft aan of de spelnaam onderdeel moet zijn van de titel die wordt getoond."
        }
        Text::ShowCategoryName => "Categorienaam tonen",
        Text::ShowCategoryNameDescription => {
            "Geeft aan of de categorienaam onderdeel moet zijn van de titel die wordt getoond."
        }
        Text::ShowFinishedRunsCount => "Aantal voltooide runs tonen",
        Text::ShowFinishedRunsCountDescription => {
            "Geeft aan of het aantal succesvol voltooide runs getoond moet worden."
        }
        Text::ShowAttemptCount => "Aantal pogingen tonen",
        Text::ShowAttemptCountDescription => {
            "Geeft aan of het totale aantal pogingen getoond moet worden."
        }
        Text::TextAlignment => "Tekstuitlijning",
        Text::TextAlignmentDescription => "Geeft de uitlijning van de titel aan.",
        Text::DisplayTextAsSingleLine => "Tekst op één regel tonen",
        Text::DisplayTextAsSingleLineDescription => {
            "Geeft aan of de titel op één regel moet worden getoond in plaats van gescheiden regels voor spel en categorie."
        }
        Text::DisplayGameIcon => "Spelicoon tonen",
        Text::DisplayGameIconDescription => {
            "Geeft aan of het spelicoon moet worden getoond als er een icoon in de splits is opgeslagen."
        }
        Text::ShowRegion => "Regio tonen",
        Text::ShowRegionDescription => {
            "De categorienaam kan worden uitgebreid met extra informatie. Dit voegt de regio van het spel toe als die is opgegeven in het tabblad variabelen van de splits-editor."
        }
        Text::ShowPlatform => "Platform tonen",
        Text::ShowPlatformDescription => {
            "De categorienaam kan worden uitgebreid met extra informatie. Dit voegt het platform toe waarop het spel wordt gespeeld als dat is opgegeven in het tabblad variabelen van de splits-editor."
        }
        Text::ShowVariables => "Variabelen tonen",
        Text::ShowVariablesDescription => {
            "De categorienaam kan worden uitgebreid met extra informatie. Dit voegt extra variabelen toe die zijn opgegeven in het tabblad variabelen van de splits-editor. Dit verwijst naar speedrun.com-variabelen, niet naar aangepaste variabelen."
        }
        Text::TotalPlaytimeBackground => "Achtergrond",
        Text::TotalPlaytimeBackgroundDescription => "De achtergrond achter het component.",
        Text::DisplayTwoRows => "2 rijen tonen",
        Text::DisplayTwoRowsDescription => {
            "Geeft aan of de naam van het component en de totale speeltijd in twee aparte rijen worden getoond."
        }
        Text::ShowDays => "Dagen tonen (>24u)",
        Text::ShowDaysDescription => {
            "Geeft aan of het aantal dagen moet worden getoond wanneer de totale speeltijd 24 uur of meer bereikt."
        }
        Text::LabelColor => "Labelkleur",
        Text::LabelColorDescription => {
            "De kleur van de naam van het component. Als deze niet is opgegeven, wordt de layoutkleur gebruikt."
        }
        Text::ValueColor => "Waarde-kleur",
        Text::ValueColorDescription => {
            "De kleur van de totale speeltijd. Als deze niet is opgegeven, wordt de layoutkleur gebruikt."
        }
        Text::BlankSpaceBackground => "Achtergrond",
        Text::BlankSpaceBackgroundDescription => "De achtergrond achter het component.",
        Text::BlankSpaceSize => "Grootte",
        Text::BlankSpaceSizeDescription => "De grootte van het component.",
        Text::CurrentComparisonBackground => "Achtergrond",
        Text::CurrentComparisonBackgroundDescription => "De achtergrond achter het component.",
        Text::CurrentComparisonDisplayTwoRows => "2 rijen tonen",
        Text::CurrentComparisonDisplayTwoRowsDescription => {
            "Geeft aan of de naam van het component en de vergelijking in twee aparte rijen worden getoond."
        }
        Text::CurrentComparisonLabelColor => "Labelkleur",
        Text::CurrentComparisonLabelColorDescription => {
            "De kleur van de naam van het component. Als deze niet is opgegeven, wordt de layoutkleur gebruikt."
        }
        Text::CurrentComparisonValueColor => "Waarde-kleur",
        Text::CurrentComparisonValueColorDescription => {
            "De kleur van de naam van de vergelijking. Als deze niet is opgegeven, wordt de layoutkleur gebruikt."
        }
        Text::CurrentPaceBackground => "Achtergrond",
        Text::CurrentPaceBackgroundDescription => "De achtergrond achter het component.",
        Text::CurrentPaceComparison => "Vergelijking",
        Text::CurrentPaceComparisonDescription => {
            "De vergelijking om de eindtijd te voorspellen. Als deze niet is opgegeven, wordt de huidige vergelijking gebruikt."
        }
        Text::CurrentPaceDisplayTwoRows => "2 rijen tonen",
        Text::CurrentPaceDisplayTwoRowsDescription => {
            "Geeft aan of de naam van het component en de voorspelde tijd in twee aparte rijen worden getoond."
        }
        Text::CurrentPaceLabelColor => "Labelkleur",
        Text::CurrentPaceLabelColorDescription => {
            "De kleur van de naam van het component. Als deze niet is opgegeven, wordt de layoutkleur gebruikt."
        }
        Text::CurrentPaceValueColor => "Waarde-kleur",
        Text::CurrentPaceValueColorDescription => {
            "De kleur van de voorspelde tijd. Als deze niet is opgegeven, wordt de layoutkleur gebruikt."
        }
        Text::CurrentPaceAccuracy => "Nauwkeurigheid",
        Text::CurrentPaceAccuracyDescription => {
            "De nauwkeurigheid van de getoonde voorspelde tijd."
        }
        Text::DeltaBackground => "Achtergrond",
        Text::DeltaBackgroundDescription => "De achtergrond achter het component.",
        Text::DeltaComparison => "Vergelijking",
        Text::DeltaComparisonDescription => {
            "De vergelijking die gebruikt wordt om te berekenen hoe ver je voor of achter ligt. Als deze niet is opgegeven, wordt de huidige vergelijking gebruikt."
        }
        Text::DeltaDisplayTwoRows => "2 rijen tonen",
        Text::DeltaDisplayTwoRowsDescription => {
            "Geeft aan of de naam van de vergelijking en het delta in twee aparte rijen worden getoond."
        }
        Text::DeltaLabelColor => "Labelkleur",
        Text::DeltaLabelColorDescription => {
            "De kleur van de vergelijkingsnaam. Als deze niet is opgegeven, wordt de layoutkleur gebruikt."
        }
        Text::DeltaDropDecimals => "Decimalen weglaten",
        Text::DeltaDropDecimalsDescription => {
            "Geeft aan of decimalen niet meer worden getoond wanneer het getoonde delta meer dan een minuut is."
        }
        Text::DeltaAccuracy => "Nauwkeurigheid",
        Text::DeltaAccuracyDescription => "De nauwkeurigheid van het getoonde delta.",
        Text::SumOfBestBackground => "Achtergrond",
        Text::SumOfBestBackgroundDescription => "De achtergrond achter het component.",
        Text::SumOfBestDisplayTwoRows => "2 rijen tonen",
        Text::SumOfBestDisplayTwoRowsDescription => {
            "Geeft aan of de naam van het component en de som van beste segmenten in twee aparte rijen worden getoond."
        }
        Text::SumOfBestLabelColor => "Labelkleur",
        Text::SumOfBestLabelColorDescription => {
            "De kleur van de naam van het component. Als deze niet is opgegeven, wordt de layoutkleur gebruikt."
        }
        Text::SumOfBestValueColor => "Waarde-kleur",
        Text::SumOfBestValueColorDescription => {
            "De kleur van de som van beste segmenten. Als deze niet is opgegeven, wordt de layoutkleur gebruikt."
        }
        Text::SumOfBestAccuracy => "Nauwkeurigheid",
        Text::SumOfBestAccuracyDescription => {
            "De nauwkeurigheid van de getoonde som van beste segmenten."
        }
        Text::PbChanceBackground => "Achtergrond",
        Text::PbChanceBackgroundDescription => "De achtergrond achter het component.",
        Text::PbChanceDisplayTwoRows => "2 rijen tonen",
        Text::PbChanceDisplayTwoRowsDescription => {
            "Geeft aan of de naam van het component en de PB-kans in twee aparte rijen worden getoond."
        }
        Text::PbChanceLabelColor => "Labelkleur",
        Text::PbChanceLabelColorDescription => {
            "De kleur van de naam van het component. Als deze niet is opgegeven, wordt de layoutkleur gebruikt."
        }
        Text::PbChanceValueColor => "Waarde-kleur",
        Text::PbChanceValueColorDescription => {
            "De kleur van de PB-kans. Als deze niet is opgegeven, wordt de layoutkleur gebruikt."
        }
        Text::PossibleTimeSaveBackground => "Achtergrond",
        Text::PossibleTimeSaveBackgroundDescription => "De achtergrond achter het component.",
        Text::PossibleTimeSaveComparison => "Vergelijking",
        Text::PossibleTimeSaveComparisonDescription => {
            "De vergelijking om de mogelijke tijdswinst te berekenen. Als deze niet is opgegeven, wordt de huidige vergelijking gebruikt."
        }
        Text::PossibleTimeSaveDisplayTwoRows => "2 rijen tonen",
        Text::PossibleTimeSaveDisplayTwoRowsDescription => {
            "Geeft aan of de naam van het component en de mogelijke tijdswinst in twee aparte rijen worden getoond."
        }
        Text::PossibleTimeSaveShowTotal => "Totale mogelijke tijdswinst tonen",
        Text::PossibleTimeSaveShowTotalDescription => {
            "Geeft aan of de totale mogelijke tijdswinst voor de rest van de poging moet worden getoond in plaats van die voor het huidige segment."
        }
        Text::PossibleTimeSaveLabelColor => "Labelkleur",
        Text::PossibleTimeSaveLabelColorDescription => {
            "De kleur van de naam van het component. Als deze niet is opgegeven, wordt de layoutkleur gebruikt."
        }
        Text::PossibleTimeSaveValueColor => "Waarde-kleur",
        Text::PossibleTimeSaveValueColorDescription => {
            "De kleur van de mogelijke tijdswinst. Als deze niet is opgegeven, wordt de layoutkleur gebruikt."
        }
        Text::PossibleTimeSaveAccuracy => "Nauwkeurigheid",
        Text::PossibleTimeSaveAccuracyDescription => {
            "De nauwkeurigheid van de getoonde mogelijke tijdswinst."
        }
        Text::PreviousSegmentBackground => "Achtergrond",
        Text::PreviousSegmentBackgroundDescription => "De achtergrond achter het component.",
        Text::PreviousSegmentComparison => "Vergelijking",
        Text::PreviousSegmentComparisonDescription => {
            "De vergelijking die wordt gebruikt om te berekenen hoeveel tijd is gewonnen of verloren. Als deze niet is opgegeven, wordt de huidige vergelijking gebruikt."
        }
        Text::PreviousSegmentDisplayTwoRows => "2 rijen tonen",
        Text::PreviousSegmentDisplayTwoRowsDescription => {
            "Geeft aan of de naam van het component en de gewonnen of verloren tijd in twee aparte rijen worden getoond."
        }
        Text::PreviousSegmentLabelColor => "Labelkleur",
        Text::PreviousSegmentLabelColorDescription => {
            "De kleur van de naam van het component. Als deze niet is opgegeven, wordt de layoutkleur gebruikt."
        }
        Text::PreviousSegmentDropDecimals => "Decimalen weglaten",
        Text::PreviousSegmentDropDecimalsDescription => {
            "Geeft aan of decimalen moeten worden weggelaten wanneer de getoonde tijd boven een minuut komt."
        }
        Text::PreviousSegmentAccuracy => "Nauwkeurigheid",
        Text::PreviousSegmentAccuracyDescription => "De nauwkeurigheid van de getoonde tijd.",
        Text::PreviousSegmentShowPossibleTimeSave => "Mogelijke tijdswinst tonen",
        Text::PreviousSegmentShowPossibleTimeSaveDescription => {
            "Geeft aan of naast de gewonnen of verloren tijd ook de mogelijke tijdswinst voor het vorige segment moet worden getoond."
        }
        Text::SegmentTimeBackground => "Achtergrond",
        Text::SegmentTimeBackgroundDescription => "De achtergrond achter het component.",
        Text::SegmentTimeComparison => "Vergelijking",
        Text::SegmentTimeComparisonDescription => {
            "De vergelijking voor de segmenttijd. Als deze niet is opgegeven, wordt de huidige vergelijking gebruikt."
        }
        Text::SegmentTimeDisplayTwoRows => "2 rijen tonen",
        Text::SegmentTimeDisplayTwoRowsDescription => {
            "Geeft aan of de naam van het component en de segmenttijd in twee aparte rijen worden getoond."
        }
        Text::SegmentTimeLabelColor => "Labelkleur",
        Text::SegmentTimeLabelColorDescription => {
            "De kleur van de naam van het component. Als deze niet is opgegeven, wordt de layoutkleur gebruikt."
        }
        Text::SegmentTimeValueColor => "Waarde-kleur",
        Text::SegmentTimeValueColorDescription => {
            "De kleur van de segmenttijd. Als deze niet is opgegeven, wordt de layoutkleur gebruikt."
        }
        Text::SegmentTimeAccuracy => "Nauwkeurigheid",
        Text::SegmentTimeAccuracyDescription => "De nauwkeurigheid van de getoonde segmenttijd.",
        Text::GraphComparison => "Vergelijking",
        Text::GraphComparisonDescription => {
            "De vergelijking die voor de grafiek wordt gebruikt. Als deze niet is opgegeven, wordt de huidige vergelijking gebruikt."
        }
        Text::GraphHeight => "Hoogte",
        Text::GraphHeightDescription => "De hoogte van de grafiek.",
        Text::GraphShowBestSegments => "Beste segmenten tonen",
        Text::GraphShowBestSegmentsDescription => {
            "Geeft aan of beste segmenten gekleurd moeten worden met de beste-segment-kleur van de layout."
        }
        Text::GraphLiveGraph => "Livegrafiek",
        Text::GraphLiveGraphDescription => {
            "Geeft aan of de grafiek continu moet vernieuwen. Als dit uitstaat, veranderen de gegevens alleen wanneer het huidige segment verandert."
        }
        Text::GraphFlipGraph => "Grafiek omdraaien",
        Text::GraphFlipGraphDescription => {
            "Geeft aan of de grafiek verticaal moet worden omgedraaid. Als dit uitstaat, worden tijden voor op de vergelijking onder de x-as weergegeven en tijden achter boven de x-as."
        }
        Text::GraphBehindBackgroundColor => "Achtergrondkleur (achter)",
        Text::GraphBehindBackgroundColorDescription => {
            "De achtergrondkleur voor het grafiekgebied met tijden die achter de vergelijking liggen."
        }
        Text::GraphAheadBackgroundColor => "Achtergrondkleur (voor)",
        Text::GraphAheadBackgroundColorDescription => {
            "De achtergrondkleur voor het grafiekgebied met tijden die vóór de vergelijking liggen."
        }
        Text::GraphGridLinesColor => "Kleur van rasterlijnen",
        Text::GraphGridLinesColorDescription => "De kleur van de rasterlijnen van de grafiek.",
        Text::GraphLinesColor => "Kleur van grafieklijnen",
        Text::GraphLinesColorDescription => {
            "De kleur van de lijnen die de punten in de grafiek verbinden."
        }
        Text::GraphPartialFillColor => "Kleur van gedeeltelijke vulling",
        Text::GraphPartialFillColorDescription => {
            "De kleur van het gebied tussen de x-as en de grafiek. Deze kleur wordt alleen gebruikt voor live wijzigingen, namelijk van de laatste split tot de huidige tijd."
        }
        Text::GraphCompleteFillColor => "Kleur van volledige vulling",
        Text::GraphCompleteFillColorDescription => {
            "De kleur van het gebied tussen de x-as en de grafiek, exclusief het live-segment."
        }
        Text::DetailedTimerBackground => "Achtergrond",
        Text::DetailedTimerBackgroundDescription => "De achtergrond achter het component.",
        Text::DetailedTimerTimingMethod => "Timingmethode",
        Text::DetailedTimerTimingMethodDescription => {
            "Geeft de te gebruiken timingmethode aan. Als deze niet is opgegeven, wordt de huidige methode gebruikt."
        }
        Text::DetailedTimerComparison1 => "Vergelijking 1",
        Text::DetailedTimerComparison1Description => {
            "De eerste vergelijking waarvan de segmenttijd wordt getoond. Als deze niet is opgegeven, wordt de huidige vergelijking gebruikt."
        }
        Text::DetailedTimerComparison2 => "Vergelijking 2",
        Text::DetailedTimerComparison2Description => {
            "De tweede vergelijking waarvan de segmenttijd wordt getoond. Als deze niet is opgegeven, wordt de huidige vergelijking gebruikt, tenzij de eerste vergelijking ook None is. Deze wordt niet getoond als de tweede vergelijking is verborgen."
        }
        Text::DetailedTimerHideSecondComparison => "Tweede vergelijking verbergen",
        Text::DetailedTimerHideSecondComparisonDescription => {
            "Geeft aan of slechts één vergelijking getoond moet worden."
        }
        Text::DetailedTimerTimerHeight => "Timerhoogte",
        Text::DetailedTimerTimerHeightDescription => "De hoogte van de hoofd-timer.",
        Text::DetailedTimerSegmentTimerHeight => "Segmenttimerhoogte",
        Text::DetailedTimerSegmentTimerHeightDescription => "De hoogte van de segmenttimer.",
        Text::DetailedTimerTimerColor => "Timerkleur",
        Text::DetailedTimerTimerColorDescription => {
            "In plaats van automatisch de kleur van de hoofd-timer te bepalen op basis van de voortgang, kan een vaste kleur worden opgegeven."
        }
        Text::DetailedTimerShowTimerGradient => "Timerverloop tonen",
        Text::DetailedTimerShowTimerGradientDescription => {
            "De hoofd-timer zet zijn kleur automatisch om in een verticaal verloop als deze instelling is ingeschakeld. Anders wordt de echte kleur gebruikt."
        }
        Text::DetailedTimerTimerDigitsFormat => "Timer-cijferformaat",
        Text::DetailedTimerTimerDigitsFormatDescription => {
            "Geeft aan hoeveel cijfers voor de hoofd-timer worden getoond. Als de duur kleiner is, worden nullen getoond."
        }
        Text::DetailedTimerTimerAccuracy => "Timer-nauwkeurigheid",
        Text::DetailedTimerTimerAccuracyDescription => {
            "De nauwkeurigheid van de getoonde tijd voor de hoofd-timer."
        }
        Text::DetailedTimerSegmentTimerColor => "Segmenttimerkleur",
        Text::DetailedTimerSegmentTimerColorDescription => {
            "Wijzigt de kleur van de segmenttimer naar een andere kleur dan de standaardkleur."
        }
        Text::DetailedTimerShowSegmentTimerGradient => "Segmenttimerverloop tonen",
        Text::DetailedTimerShowSegmentTimerGradientDescription => {
            "De segmenttimer zet zijn kleur automatisch om in een verticaal verloop als deze instelling is ingeschakeld. Anders wordt de echte kleur gebruikt."
        }
        Text::DetailedTimerSegmentTimerDigitsFormat => "Segmenttimer-cijferformaat",
        Text::DetailedTimerSegmentTimerDigitsFormatDescription => {
            "Geeft aan hoeveel cijfers voor de segmenttimer worden getoond. Als de duur kleiner is, worden nullen getoond."
        }
        Text::DetailedTimerSegmentTimerAccuracy => "Segmenttimer-nauwkeurigheid",
        Text::DetailedTimerSegmentTimerAccuracyDescription => {
            "De nauwkeurigheid van de getoonde tijd voor de segmenttimer."
        }
        Text::DetailedTimerComparisonNamesColor => "Kleur van vergelijkingsnamen",
        Text::DetailedTimerComparisonNamesColorDescription => {
            "De kleur van vergelijkingsnamen als die worden getoond. Als er geen kleur is opgegeven, wordt de layoutkleur gebruikt."
        }
        Text::DetailedTimerComparisonTimesColor => "Kleur van vergelijkingstijden",
        Text::DetailedTimerComparisonTimesColorDescription => {
            "De kleur van vergelijkingstijden als die worden getoond. Als er geen kleur is opgegeven, wordt de layoutkleur gebruikt."
        }
        Text::DetailedTimerComparisonTimesAccuracy => "Nauwkeurigheid van vergelijkingstijden",
        Text::DetailedTimerComparisonTimesAccuracyDescription => {
            "De nauwkeurigheid van de vergelijkingstijden."
        }
        Text::DetailedTimerShowSegmentName => "Segmentnaam tonen",
        Text::DetailedTimerShowSegmentNameDescription => {
            "Geeft aan of de segmentnaam getoond moet worden."
        }
        Text::DetailedTimerSegmentNameColor => "Kleur van segmentnaam",
        Text::DetailedTimerSegmentNameColorDescription => {
            "De kleur van de segmentnaam als deze wordt getoond. Als er geen kleur is opgegeven, wordt de layoutkleur gebruikt."
        }
        Text::DetailedTimerDisplayIcon => "Icoon tonen",
        Text::DetailedTimerDisplayIconDescription => {
            "Geeft aan of het segmenticoon getoond moet worden."
        }
        Text::SplitsBackground => "Achtergrond",
        Text::SplitsBackgroundDescription => {
            "De achtergrond achter het component. Je kunt afwisselende kleuren kiezen; in dat geval wisselt elke rij tussen de twee gekozen kleuren."
        }
        Text::SplitsTotalRows => "Totaal aantal rijen",
        Text::SplitsTotalRowsDescription => {
            "Het totale aantal segmentrijen dat wordt getoond. Als dit 0 is, worden alle segmenten getoond. Als dit lager is dan het totaal, wordt een venster getoond dat kan scrollen."
        }
        Text::SplitsUpcomingSegments => "Aankomende segmenten",
        Text::SplitsUpcomingSegmentsDescription => {
            "Als er meer segmenten zijn dan getoonde rijen, scrollt het venster automatisch wanneer het huidige segment verandert. Dit aantal bepaalt het minimum aan toekomstige segmenten dat getoond wordt."
        }
        Text::SplitsShowThinSeparators => "Dunne scheiders tonen",
        Text::SplitsShowThinSeparatorsDescription => {
            "Geeft aan of dunne scheiders tussen segmentrijen moeten worden getoond."
        }
        Text::SplitsShowSeparatorBeforeLastSplit => "Scheider voor laatste split tonen",
        Text::SplitsShowSeparatorBeforeLastSplitDescription => {
            "Als het laatste segment altijd wordt getoond, bepaalt dit of een duidelijke scheider vóór het laatste segment wordt getoond wanneer het niet direct aansluit op het vorige segment in het venster."
        }
        Text::SplitsAlwaysShowLastSplit => "Laatste split altijd tonen",
        Text::SplitsAlwaysShowLastSplitDescription => {
            "Als niet elk segment in het venster wordt getoond, bepaalt deze optie of het laatste segment altijd getoond wordt, omdat dit de totale duur van de gekozen vergelijking bevat."
        }
        Text::SplitsFillWithBlankSpace => "Opvullen met lege ruimte",
        Text::SplitsFillWithBlankSpaceDescription => {
            "Als er niet genoeg segmenten zijn om de lijst te vullen, kun je de resterende rijen opvullen met lege ruimte om altijd het totale aantal rijen te tonen."
        }
        Text::SplitsShowTimesBelowSegmentName => "Tijden onder segmentnaam tonen",
        Text::SplitsShowTimesBelowSegmentNameDescription => {
            "Geeft aan of tijden onder de segmentnaam moeten worden getoond. Anders worden de tijden naast de segmentnaam getoond."
        }
        Text::SplitsCurrentSegmentGradient => "Huidig segmentverloop",
        Text::SplitsCurrentSegmentGradientDescription => {
            "Het verloop dat achter het huidige segment wordt getoond als indicator."
        }
        Text::SplitsSplitTimeAccuracy => "Nauwkeurigheid van splittijd",
        Text::SplitsSplitTimeAccuracyDescription => {
            "Geeft de nauwkeurigheid aan voor kolommen met splittijden."
        }
        Text::SplitsSegmentTimeAccuracy => "Nauwkeurigheid van segmenttijd",
        Text::SplitsSegmentTimeAccuracyDescription => {
            "Geeft de nauwkeurigheid aan voor kolommen met segmenttijden."
        }
        Text::SplitsDeltaTimeAccuracy => "Nauwkeurigheid van delta",
        Text::SplitsDeltaTimeAccuracyDescription => {
            "Geeft de nauwkeurigheid aan voor kolommen met tijd voor of achter."
        }
        Text::SplitsDropDeltaDecimals => "Delta-decimalen verbergen bij minuten",
        Text::SplitsDropDeltaDecimalsDescription => {
            "Geeft aan of decimalen niet meer worden getoond wanneer een delta-kolom boven een minuut ligt."
        }
        Text::SplitsShowColumnLabels => "Kolomlabels tonen",
        Text::SplitsShowColumnLabelsDescription => {
            "Geeft aan of de namen van de kolommen bovenaan de lijst worden getoond."
        }
        Text::SplitsColumns => "Kolommen",
        Text::SplitsColumnsDescription => {
            "Het aantal kolommen per rij. Elke kolom kan verschillende informatie tonen. Kolommen worden van rechts naar links gedefinieerd."
        }
        Text::SplitsColumnName => "Kolomnaam",
        Text::SplitsColumnNameDescription => {
            "De naam van de kolom. Deze wordt bovenaan de lijst getoond als kolomlabels zijn ingeschakeld."
        }
        Text::SplitsColumnType => "Kolomtype",
        Text::SplitsColumnTypeDescription => {
            "Het type informatie dat deze kolom toont. Dit kan een tijd zijn of een aangepaste variabele."
        }
        Text::SplitsVariableName => "Variabelenaam",
        Text::SplitsVariableNameDescription => {
            "De naam van de aangepaste variabele die deze kolom toont."
        }
        Text::SplitsStartWith => "Beginnen met",
        Text::SplitsStartWithDescription => {
            "De waarde waarmee deze kolom voor elk segment start. De update-trigger bepaalt wanneer deze waarde wordt vervangen."
        }
        Text::SplitsUpdateWith => "Bijwerken met",
        Text::SplitsUpdateWithDescription => {
            "Zodra aan een bepaalde voorwaarde wordt voldaan, meestal het segment actief hebben of afgerond hebben, wordt de tijd bijgewerkt met de hier aangegeven waarde."
        }
        Text::SplitsUpdateTrigger => "Update-trigger",
        Text::SplitsUpdateTriggerDescription => {
            "De voorwaarde die moet worden voldaan om de tijd te updaten met de waarde uit «Bijwerken met». Hiervoor is de tijd de waarde uit «Beginnen met»."
        }
        Text::SplitsColumnComparison => "Vergelijking",
        Text::SplitsColumnComparisonDescription => {
            "De vergelijking waartegen deze kolom wordt vergeleken. Als deze niet is opgegeven, wordt de huidige vergelijking gebruikt."
        }
        Text::SplitsColumnTimingMethod => "Timingmethode",
        Text::SplitsColumnTimingMethodDescription => {
            "Geeft de timingmethode aan voor deze kolom. Als deze niet is opgegeven, wordt de huidige methode gebruikt."
        }
        Text::TextComponentBackground => "Achtergrond",
        Text::TextComponentBackgroundDescription => "De achtergrond achter het component.",
        Text::TextComponentUseVariable => "Variabele gebruiken",
        Text::TextComponentUseVariableDescription => {
            "Geeft aan of een aangepaste variabele moet worden gebruikt om een dynamische waarde te tonen. Aangepaste variabelen kunnen worden ingesteld in de splits-editor en automatisch worden aangeleverd door auto splitters."
        }
        Text::TextComponentSplit => "Splitsen",
        Text::TextComponentSplitDescription => {
            "Geeft aan of de tekst in een linker- en rechterdeel wordt gesplitst. Zo niet, wordt één gecentreerde tekst getoond."
        }
        Text::TextComponentText => "Tekst",
        Text::TextComponentTextDescription => "Geeft de tekst aan die in het midden wordt getoond.",
        Text::TextComponentLeft => "Links",
        Text::TextComponentLeftDescription => "Geeft de tekst aan die links wordt getoond.",
        Text::TextComponentRight => "Rechts",
        Text::TextComponentRightDescription => "Geeft de tekst aan die rechts wordt getoond.",
        Text::TextComponentVariable => "Variabele",
        Text::TextComponentVariableDescription => {
            "Geeft de naam van de aangepaste variabele aan die wordt getoond."
        }
        Text::TextComponentTextColor => "Tekstkleur",
        Text::TextComponentTextColorDescription => "De kleur van de tekst.",
        Text::TextComponentLeftColor => "Linkerkleur",
        Text::TextComponentLeftColorDescription => "De kleur van de tekst links.",
        Text::TextComponentRightColor => "Rechterkleur",
        Text::TextComponentRightColorDescription => "De kleur van de tekst rechts.",
        Text::TextComponentNameColor => "Naamkleur",
        Text::TextComponentNameColorDescription => "De kleur van de variabelenaam.",
        Text::TextComponentValueColor => "Waarde-kleur",
        Text::TextComponentValueColorDescription => "De kleur van de variabelewaarde.",
        Text::TextComponentDisplayTwoRows => "2 rijen tonen",
        Text::TextComponentDisplayTwoRowsDescription => {
            "Geeft aan of de linker- en rechtertekst in twee aparte rijen moeten worden getoond."
        }
        Text::LayoutDirection => "Lay-out richting",
        Text::LayoutDirectionDescription => "De richting waarin de componenten worden uitgelijnd.",
        Text::CustomTimerFont => "Aangepast timerlettertype",
        Text::CustomTimerFontDescription => {
            "Hiermee kun je een aangepast lettertype voor de timer instellen. Als dit niet is ingesteld, wordt het standaardlettertype gebruikt."
        }
        Text::CustomTimesFont => "Aangepast tijdenlettertype",
        Text::CustomTimesFontDescription => {
            "Hiermee kun je een aangepast lettertype voor de tijden instellen. Als dit niet is ingesteld, wordt het standaardlettertype gebruikt."
        }
        Text::CustomTextFont => "Aangepast tekstlettertype",
        Text::CustomTextFontDescription => {
            "Hiermee kun je een aangepast lettertype voor tekst instellen. Als dit niet is ingesteld, wordt het standaardlettertype gebruikt."
        }
        Text::TextShadow => "Tekstschaduw",
        Text::TextShadowDescription => {
            "Hiermee kun je optioneel een kleur voor tekstschaduwen instellen."
        }
        Text::Background => "Achtergrond",
        Text::BackgroundDescription => "De achtergrond achter de volledige layout.",
        Text::BestSegment => "Beste segment",
        Text::BestSegmentDescription => {
            "De kleur die wordt gebruikt wanneer je een nieuw beste segment behaalt."
        }
        Text::AheadGainingTime => "Voor (tijd winnen)",
        Text::AheadGainingTimeDescription => {
            "De kleur die wordt gebruikt wanneer je voor ligt en nog meer tijd wint."
        }
        Text::AheadLosingTime => "Voor (tijd verliezen)",
        Text::AheadLosingTimeDescription => {
            "De kleur die wordt gebruikt wanneer je voor ligt maar tijd verliest."
        }
        Text::BehindGainingTime => "Achter (tijd terugwinnen)",
        Text::BehindGainingTimeDescription => {
            "De kleur die wordt gebruikt wanneer je achter ligt maar tijd terugwint."
        }
        Text::BehindLosingTime => "Achter (tijd verliezen)",
        Text::BehindLosingTimeDescription => {
            "De kleur die wordt gebruikt wanneer je achter ligt en nog meer tijd verliest."
        }
        Text::NotRunning => "Niet actief",
        Text::NotRunningDescription => {
            "De kleur die wordt gebruikt wanneer er geen poging actief is."
        }
        Text::PersonalBest => "Persoonlijk record",
        Text::PersonalBestDescription => {
            "De kleur die wordt gebruikt wanneer je een nieuw persoonlijk record behaalt."
        }
        Text::Paused => "Gepauzeerd",
        Text::PausedDescription => "De kleur die wordt gebruikt wanneer de timer is gepauzeerd.",
        Text::ThinSeparators => "Dunne scheiders",
        Text::ThinSeparatorsDescription => "De kleur van dunne scheiders.",
        Text::Separators => "Scheiders",
        Text::SeparatorsDescription => "De kleur van normale scheiders.",
        Text::TextColor => "Tekst",
        Text::TextColorDescription => {
            "De kleur die wordt gebruikt voor tekst die geen eigen kleur opgeeft."
        }
        Text::ComponentBlankSpace => "Lege ruimte",
        Text::ComponentCurrentComparison => "Huidige vergelijking",
        Text::ComponentCurrentPace => "Huidige pace",
        Text::ComponentDelta => "Delta",
        Text::ComponentDetailedTimer => "Gedetailleerde timer",
        Text::ComponentGraph => "Grafiek",
        Text::ComponentPbChance => "PB-kans",
        Text::ComponentPossibleTimeSave => "Mogelijke tijdbesparing",
        Text::ComponentPreviousSegment => "Vorig segment",
        Text::ComponentSegmentTime => "Segmenttijd",
        Text::ComponentSeparator => "Scheiding",
        Text::ComponentSplits => "Splits",
        Text::ComponentSumOfBest => "Som van beste",
        Text::ComponentText => "Tekst",
        Text::ComponentTimer => "Timer",
        Text::ComponentSegmentTimer => "Segmenttimer",
        Text::ComponentTitle => "Titel",
        Text::ComponentTotalPlaytime => "Totale speeltijd",
        Text::ComponentCurrentPaceBestPossibleTime => "Best mogelijke tijd",
        Text::ComponentCurrentPaceWorstPossibleTime => "Slechtst mogelijke tijd",
        Text::ComponentCurrentPacePredictedTime => "Voorspelde tijd",
        Text::ComponentSegmentTimeBest => "Beste segmenttijd",
        Text::ComponentSegmentTimeWorst => "Slechtste segmenttijd",
        Text::ComponentSegmentTimeAverage => "Gemiddelde segmenttijd",
        Text::ComponentSegmentTimeMedian => "Mediane segmenttijd",
        Text::ComponentSegmentTimeLatest => "Laatste segmenttijd",
        Text::ComponentPossibleTimeSaveTotal => "Totale mogelijke tijdswinst",
        Text::LiveSegment => "Live segment",
        Text::LiveSegmentShort => "Live segment",
        Text::PreviousSegmentShort => "Vorig segment",
        Text::PreviousSegmentAbbreviation => "Vorig seg.",
        Text::ComparingAgainst => "Vergelijking met",
        Text::ComparisonShort => "Vergelijking",
        Text::CurrentPaceBestPossibleTimeShort => "Best mog. tijd",
        Text::CurrentPaceBestTimeShort => "Beste tijd",
        Text::CurrentPaceBestPossibleTimeAbbreviation => "BMT",
        Text::CurrentPaceWorstPossibleTimeShort => "Slechtst mog. tijd",
        Text::CurrentPaceWorstTimeShort => "Slechtste tijd",
        Text::CurrentPacePredictedTimeShort => "Voorsp. tijd",
        Text::CurrentPaceShort => "Huid. tempo",
        Text::CurrentPaceAbbreviation => "Tempo",
        Text::Goal => "Doel",
        Text::SumOfBestSegments => "Som van beste segmenten",
        Text::SumOfBestShort => "Som van beste",
        Text::SumOfBestAbbreviation => "SvB",
        Text::PlaytimeShort => "Speeltijd",
        Text::BestSegmentTimeShort => "Beste seg. tijd",
        Text::BestSegmentShort => "Beste segment",
        Text::WorstSegmentTimeShort => "Slechtste seg. tijd",
        Text::WorstSegmentShort => "Slechtste segment",
        Text::AverageSegmentTimeShort => "Gem. seg. tijd",
        Text::AverageSegmentShort => "Gemiddeld segment",
        Text::MedianSegmentTimeShort => "Mediaan seg. tijd",
        Text::MedianSegmentShort => "Mediaan segment",
        Text::LatestSegmentTimeShort => "Laatste seg. tijd",
        Text::LatestSegmentShort => "Laatste segment",
        Text::SegmentTimeShort => "Seg. tijd",
        Text::SplitTime => "Tijd",
        Text::PossibleTimeSaveShort => "Mogelijke tijdswinst",
        Text::PossibleTimeSaveAbbreviation => "Mogl. tijdswinst",
        Text::TimeSaveShort => "Tijdswinst",
        Text::RealTime => "Echte tijd",
        Text::GameTime => "Speltijd",
        Text::Untitled => "Naamloos",
        Text::SumOfBestCleanerStartOfRun => "het begin van de run",
        Text::SumOfBestCleanerShouldRemove => {
            ". Denk je dat deze segmenttijd onnauwkeurig is en verwijderd moet worden?"
        }
    }
}

pub const fn resolve_placeholder(text: PlaceholderText) -> &'static [Piece] {
    match text {
        PlaceholderText::SumOfBestCleanerSegmentTimeBetween => &[
            Piece::Static("Je had een "),
            Piece::Dynamic(0),
            Piece::Static("-segmenttijd van "),
            Piece::Dynamic(1),
            Piece::Static(" tussen “"),
            Piece::Dynamic(2),
            Piece::Static("” en “"),
            Piece::Dynamic(3),
            Piece::Static("”"),
        ],
        PlaceholderText::SumOfBestCleanerFasterThanCombined => &[
            Piece::Static(", wat sneller is dan de gecombineerde beste segmenten van "),
            Piece::Dynamic(0),
        ],
        PlaceholderText::SumOfBestCleanerRunOn => &[
            Piece::Static(" in een run op "),
            Piece::Dynamic(0),
            Piece::Static(" die begon om "),
            Piece::Dynamic(1),
        ],
    }
}
