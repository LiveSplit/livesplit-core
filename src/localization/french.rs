use super::{Piece, PlaceholderText, Text};

pub const fn resolve(text: Text) -> &'static str {
    match text {
        Text::StartSplit => "Démarrer / Split",
        Text::StartSplitDescription => {
            "Le raccourci à utiliser pour splitter et démarrer une nouvelle tentative."
        }
        Text::Reset => "Réinitialiser",
        Text::ResetDescription => {
            "Le raccourci à utiliser pour réinitialiser la tentative en cours."
        }
        Text::UndoSplit => "Annuler le split",
        Text::UndoSplitDescription => "Le raccourci à utiliser pour annuler le dernier split.",
        Text::SkipSplit => "Passer le split",
        Text::SkipSplitDescription => "Le raccourci à utiliser pour passer le split actuel.",
        Text::Pause => "Pause",
        Text::PauseDescription => {
            "Le raccourci à utiliser pour mettre en pause la tentative en cours. Il peut aussi servir à démarrer une nouvelle tentative."
        }
        Text::UndoAllPauses => "Annuler toutes les pauses",
        Text::UndoAllPausesDescription => {
            "Le raccourci à utiliser pour retirer tous les temps de pause du temps actuel. Utile si vous avez mis en pause par erreur."
        }
        Text::PreviousComparison => "Comparaison précédente",
        Text::PreviousComparisonDescription => {
            "Le raccourci à utiliser pour passer à la comparaison précédente."
        }
        Text::NextComparison => "Comparaison suivante",
        Text::NextComparisonDescription => {
            "Le raccourci à utiliser pour passer à la comparaison suivante."
        }
        Text::ToggleTimingMethod => "Basculer la méthode de timing",
        Text::ToggleTimingMethodDescription => {
            "Le raccourci à utiliser pour basculer entre « Temps réel » et « Temps de jeu »."
        }
        Text::TimerBackground => "Arrière-plan",
        Text::TimerBackgroundDescription => {
            "L’arrière-plan affiché derrière le composant. Il est aussi possible d’appliquer la couleur associée à l’avance ou au retard comme couleur d’arrière-plan."
        }
        Text::SegmentTimer => "Minuteur de segment",
        Text::SegmentTimerDescription => {
            "Indique s’il faut afficher le temps écoulé depuis le début du segment en cours plutôt que depuis le début de la tentative."
        }
        Text::TimingMethod => "Méthode de timing",
        Text::TimingMethodDescription => {
            "Indique la méthode de timing à utiliser. Si non précisé, la méthode actuelle est utilisée."
        }
        Text::Height => "Hauteur",
        Text::HeightDescription => "La hauteur du timer.",
        Text::TimerTextColor => "Couleur du texte",
        Text::TimerTextColorDescription => {
            "La couleur du temps affiché. Si non précisée, la couleur est choisie automatiquement selon la progression de la tentative. Ces couleurs peuvent être définies dans les paramètres généraux du layout."
        }
        Text::ShowGradient => "Afficher le dégradé",
        Text::ShowGradientDescription => {
            "Détermine s’il faut afficher la couleur du timer en dégradé."
        }
        Text::DigitsFormat => "Format des chiffres",
        Text::DigitsFormatDescription => {
            "Indique combien de chiffres afficher. Si la durée est inférieure, des zéros sont affichés."
        }
        Text::Accuracy => "Précision",
        Text::AccuracyDescription => "La précision du temps affiché.",
        Text::TitleBackground => "Arrière-plan",
        Text::TitleBackgroundDescription => "L’arrière-plan affiché derrière le composant.",
        Text::TitleTextColor => "Couleur du texte",
        Text::TitleTextColorDescription => {
            "La couleur du texte du titre. Si aucune couleur n’est indiquée, la couleur du layout est utilisée."
        }
        Text::ShowGameName => "Afficher le nom du jeu",
        Text::ShowGameNameDescription => {
            "Indique si le nom du jeu doit faire partie du titre affiché."
        }
        Text::ShowCategoryName => "Afficher le nom de la catégorie",
        Text::ShowCategoryNameDescription => {
            "Indique si le nom de la catégorie doit faire partie du titre affiché."
        }
        Text::ShowFinishedRunsCount => "Afficher le nombre de runs terminés",
        Text::ShowFinishedRunsCountDescription => {
            "Indique si le nombre de runs terminés avec succès doit être affiché."
        }
        Text::ShowAttemptCount => "Afficher le nombre de tentatives",
        Text::ShowAttemptCountDescription => {
            "Indique si le nombre total de tentatives doit être affiché."
        }
        Text::TextAlignment => "Alignement du texte",
        Text::TextAlignmentDescription => "Indique l’alignement du titre.",
        Text::DisplayTextAsSingleLine => "Afficher le texte sur une seule ligne",
        Text::DisplayTextAsSingleLineDescription => {
            "Indique si le titre doit être affiché sur une seule ligne, plutôt que séparé en une ligne pour le nom du jeu et une pour la catégorie."
        }
        Text::DisplayGameIcon => "Afficher l’icône du jeu",
        Text::DisplayGameIconDescription => {
            "Indique si l’icône du jeu doit être affichée, si une icône est stockée dans les splits."
        }
        Text::ShowRegion => "Afficher la région",
        Text::ShowRegionDescription => {
            "Le nom de la catégorie peut être étendu avec des informations supplémentaires. Ceci l’étend avec la région du jeu si elle est renseignée dans l’onglet variables de l’éditeur de splits."
        }
        Text::ShowPlatform => "Afficher la plateforme",
        Text::ShowPlatformDescription => {
            "Le nom de la catégorie peut être étendu avec des informations supplémentaires. Ceci l’étend avec la plateforme du jeu si elle est renseignée dans l’onglet variables de l’éditeur de splits."
        }
        Text::ShowVariables => "Afficher les variables",
        Text::ShowVariablesDescription => {
            "Le nom de la catégorie peut être étendu avec des informations supplémentaires. Ceci l’étend avec des variables supplémentaires renseignées dans l’onglet variables de l’éditeur de splits. Il s’agit des variables speedrun.com, pas des variables personnalisées."
        }
        Text::TotalPlaytimeBackground => "Arrière-plan",
        Text::TotalPlaytimeBackgroundDescription => "L’arrière-plan affiché derrière le composant.",
        Text::DisplayTwoRows => "Afficher sur 2 lignes",
        Text::DisplayTwoRowsDescription => {
            "Indique s’il faut afficher le nom du composant et le temps total de jeu sur deux lignes distinctes."
        }
        Text::ShowDays => "Afficher les jours (>24h)",
        Text::ShowDaysDescription => {
            "Indique s’il faut afficher le nombre de jours lorsque le temps total de jeu atteint 24 heures ou plus."
        }
        Text::LabelColor => "Couleur du libellé",
        Text::LabelColorDescription => {
            "La couleur du nom du composant. Si non précisée, la couleur du layout est utilisée."
        }
        Text::ValueColor => "Couleur de la valeur",
        Text::ValueColorDescription => {
            "La couleur du temps total de jeu. Si non précisée, la couleur du layout est utilisée."
        }
        Text::BlankSpaceBackground => "Arrière-plan",
        Text::BlankSpaceBackgroundDescription => "L’arrière-plan affiché derrière le composant.",
        Text::BlankSpaceSize => "Taille",
        Text::BlankSpaceSizeDescription => "La taille du composant.",
        Text::CurrentComparisonBackground => "Arrière-plan",
        Text::CurrentComparisonBackgroundDescription => {
            "L’arrière-plan affiché derrière le composant."
        }
        Text::CurrentComparisonDisplayTwoRows => "Afficher sur 2 lignes",
        Text::CurrentComparisonDisplayTwoRowsDescription => {
            "Indique s’il faut afficher le nom du composant et la comparaison sur deux lignes distinctes."
        }
        Text::CurrentComparisonLabelColor => "Couleur du libellé",
        Text::CurrentComparisonLabelColorDescription => {
            "La couleur du nom du composant. Si non précisée, la couleur du layout est utilisée."
        }
        Text::CurrentComparisonValueColor => "Couleur de la valeur",
        Text::CurrentComparisonValueColorDescription => {
            "La couleur du nom de la comparaison. Si non précisée, la couleur du layout est utilisée."
        }
        Text::CurrentPaceBackground => "Arrière-plan",
        Text::CurrentPaceBackgroundDescription => "L’arrière-plan affiché derrière le composant.",
        Text::CurrentPaceComparison => "Comparaison",
        Text::CurrentPaceComparisonDescription => {
            "La comparaison utilisée pour prédire le temps final. Si non précisée, la comparaison actuelle est utilisée."
        }
        Text::CurrentPaceDisplayTwoRows => "Afficher sur 2 lignes",
        Text::CurrentPaceDisplayTwoRowsDescription => {
            "Indique s’il faut afficher le nom du composant et le temps prédit sur deux lignes distinctes."
        }
        Text::CurrentPaceLabelColor => "Couleur du libellé",
        Text::CurrentPaceLabelColorDescription => {
            "La couleur du nom du composant. Si non précisée, la couleur du layout est utilisée."
        }
        Text::CurrentPaceValueColor => "Couleur de la valeur",
        Text::CurrentPaceValueColorDescription => {
            "La couleur du temps prédit. Si non précisée, la couleur du layout est utilisée."
        }
        Text::CurrentPaceAccuracy => "Précision",
        Text::CurrentPaceAccuracyDescription => "La précision du temps prédit affiché.",
        Text::DeltaBackground => "Arrière-plan",
        Text::DeltaBackgroundDescription => "L’arrière-plan affiché derrière le composant.",
        Text::DeltaComparison => "Comparaison",
        Text::DeltaComparisonDescription => {
            "La comparaison utilisée pour calculer l’avance ou le retard. Si non précisée, la comparaison actuelle est utilisée."
        }
        Text::DeltaDisplayTwoRows => "Afficher sur 2 lignes",
        Text::DeltaDisplayTwoRowsDescription => {
            "Indique s’il faut afficher le nom de la comparaison et le delta sur deux lignes distinctes."
        }
        Text::DeltaLabelColor => "Couleur du libellé",
        Text::DeltaLabelColorDescription => {
            "La couleur du nom de la comparaison. Si non précisée, la couleur du layout est utilisée."
        }
        Text::DeltaDropDecimals => "Supprimer les décimales",
        Text::DeltaDropDecimalsDescription => {
            "Indique si les décimales doivent être masquées lorsque le delta affiché dépasse une minute."
        }
        Text::DeltaAccuracy => "Précision",
        Text::DeltaAccuracyDescription => "La précision du delta affiché.",
        Text::SumOfBestBackground => "Arrière-plan",
        Text::SumOfBestBackgroundDescription => "L’arrière-plan affiché derrière le composant.",
        Text::SumOfBestDisplayTwoRows => "Afficher sur 2 lignes",
        Text::SumOfBestDisplayTwoRowsDescription => {
            "Indique s’il faut afficher le nom du composant et la somme des meilleurs segments sur deux lignes distinctes."
        }
        Text::SumOfBestLabelColor => "Couleur du libellé",
        Text::SumOfBestLabelColorDescription => {
            "La couleur du nom du composant. Si non précisée, la couleur du layout est utilisée."
        }
        Text::SumOfBestValueColor => "Couleur de la valeur",
        Text::SumOfBestValueColorDescription => {
            "La couleur de la somme des meilleurs segments. Si non précisée, la couleur du layout est utilisée."
        }
        Text::SumOfBestAccuracy => "Précision",
        Text::SumOfBestAccuracyDescription => {
            "La précision de la somme des meilleurs segments affichée."
        }
        Text::PbChanceBackground => "Arrière-plan",
        Text::PbChanceBackgroundDescription => "L’arrière-plan affiché derrière le composant.",
        Text::PbChanceDisplayTwoRows => "Afficher sur 2 lignes",
        Text::PbChanceDisplayTwoRowsDescription => {
            "Indique s’il faut afficher le nom du composant et la chance de PB sur deux lignes distinctes."
        }
        Text::PbChanceLabelColor => "Couleur du libellé",
        Text::PbChanceLabelColorDescription => {
            "La couleur du nom du composant. Si non précisée, la couleur du layout est utilisée."
        }
        Text::PbChanceValueColor => "Couleur de la valeur",
        Text::PbChanceValueColorDescription => {
            "La couleur de la chance de PB. Si non précisée, la couleur du layout est utilisée."
        }
        Text::PossibleTimeSaveBackground => "Arrière-plan",
        Text::PossibleTimeSaveBackgroundDescription => {
            "L’arrière-plan affiché derrière le composant."
        }
        Text::PossibleTimeSaveComparison => "Comparaison",
        Text::PossibleTimeSaveComparisonDescription => {
            "La comparaison pour calculer le temps potentiellement gagnable. Si non précisée, la comparaison actuelle est utilisée."
        }
        Text::PossibleTimeSaveDisplayTwoRows => "Afficher sur 2 lignes",
        Text::PossibleTimeSaveDisplayTwoRowsDescription => {
            "Indique s’il faut afficher le nom du composant et le temps potentiellement gagnable sur deux lignes distinctes."
        }
        Text::PossibleTimeSaveShowTotal => "Afficher le temps total potentiellement gagnable",
        Text::PossibleTimeSaveShowTotalDescription => {
            "Indique s’il faut afficher le temps total potentiellement gagnable pour le reste de la tentative plutôt que celui du segment actuel."
        }
        Text::PossibleTimeSaveLabelColor => "Couleur du libellé",
        Text::PossibleTimeSaveLabelColorDescription => {
            "La couleur du nom du composant. Si non précisée, la couleur du layout est utilisée."
        }
        Text::PossibleTimeSaveValueColor => "Couleur de la valeur",
        Text::PossibleTimeSaveValueColorDescription => {
            "La couleur du temps potentiellement gagnable. Si non précisée, la couleur du layout est utilisée."
        }
        Text::PossibleTimeSaveAccuracy => "Précision",
        Text::PossibleTimeSaveAccuracyDescription => {
            "La précision du temps potentiellement gagnable affiché."
        }
        Text::PreviousSegmentBackground => "Arrière-plan",
        Text::PreviousSegmentBackgroundDescription => {
            "L’arrière-plan affiché derrière le composant."
        }
        Text::PreviousSegmentComparison => "Comparaison",
        Text::PreviousSegmentComparisonDescription => {
            "La comparaison utilisée pour calculer le temps gagné ou perdu. Si non précisée, la comparaison actuelle est utilisée."
        }
        Text::PreviousSegmentDisplayTwoRows => "Afficher sur 2 lignes",
        Text::PreviousSegmentDisplayTwoRowsDescription => {
            "Indique s’il faut afficher le nom du composant et le temps gagné ou perdu sur deux lignes distinctes."
        }
        Text::PreviousSegmentLabelColor => "Couleur du libellé",
        Text::PreviousSegmentLabelColorDescription => {
            "La couleur du nom du composant. Si non précisée, la couleur du layout est utilisée."
        }
        Text::PreviousSegmentDropDecimals => "Supprimer les décimales",
        Text::PreviousSegmentDropDecimalsDescription => {
            "Indique s’il faut masquer les décimales lorsque le temps affiché dépasse une minute."
        }
        Text::PreviousSegmentAccuracy => "Précision",
        Text::PreviousSegmentAccuracyDescription => "La précision du temps affiché.",
        Text::PreviousSegmentShowPossibleTimeSave => "Afficher le temps potentiellement gagnable",
        Text::PreviousSegmentShowPossibleTimeSaveDescription => {
            "Indique s’il faut afficher, en plus du temps gagné ou perdu, le temps potentiellement gagnable pour le segment précédent."
        }
        Text::SegmentTimeBackground => "Arrière-plan",
        Text::SegmentTimeBackgroundDescription => "L’arrière-plan affiché derrière le composant.",
        Text::SegmentTimeComparison => "Comparaison",
        Text::SegmentTimeComparisonDescription => {
            "La comparaison pour le temps de segment. Si non précisée, la comparaison actuelle est utilisée."
        }
        Text::SegmentTimeDisplayTwoRows => "Afficher sur 2 lignes",
        Text::SegmentTimeDisplayTwoRowsDescription => {
            "Indique s’il faut afficher le nom du composant et le temps de segment sur deux lignes distinctes."
        }
        Text::SegmentTimeLabelColor => "Couleur du libellé",
        Text::SegmentTimeLabelColorDescription => {
            "La couleur du nom du composant. Si non précisée, la couleur du layout est utilisée."
        }
        Text::SegmentTimeValueColor => "Couleur de la valeur",
        Text::SegmentTimeValueColorDescription => {
            "La couleur du temps de segment. Si non précisée, la couleur du layout est utilisée."
        }
        Text::SegmentTimeAccuracy => "Précision",
        Text::SegmentTimeAccuracyDescription => "La précision du temps de segment affiché.",
        Text::GraphComparison => "Comparaison",
        Text::GraphComparisonDescription => {
            "La comparaison utilisée pour le graphique. Si non précisée, la comparaison actuelle est utilisée."
        }
        Text::GraphHeight => "Hauteur",
        Text::GraphHeightDescription => "La hauteur du graphique.",
        Text::GraphShowBestSegments => "Afficher les meilleurs segments",
        Text::GraphShowBestSegmentsDescription => {
            "Indique si les meilleurs segments doivent être colorés avec la couleur des meilleurs segments du layout."
        }
        Text::GraphLiveGraph => "Graphique en direct",
        Text::GraphLiveGraphDescription => {
            "Indique si le graphique doit se mettre à jour en continu. Si désactivé, les changements n’ont lieu que lorsque le segment actuel change."
        }
        Text::GraphFlipGraph => "Inverser le graphique",
        Text::GraphFlipGraphDescription => {
            "Indique si le graphique doit être inversé verticalement. Sinon, les temps en avance sont affichés sous l’axe des x et les temps en retard au-dessus."
        }
        Text::GraphBehindBackgroundColor => "Couleur d’arrière-plan (retard)",
        Text::GraphBehindBackgroundColorDescription => {
            "La couleur d’arrière-plan pour la zone du graphique contenant les temps en retard par rapport à la comparaison."
        }
        Text::GraphAheadBackgroundColor => "Couleur d’arrière-plan (avance)",
        Text::GraphAheadBackgroundColorDescription => {
            "La couleur d’arrière-plan pour la zone du graphique contenant les temps en avance par rapport à la comparaison."
        }
        Text::GraphGridLinesColor => "Couleur des lignes de grille",
        Text::GraphGridLinesColorDescription => "La couleur des lignes de grille du graphique.",
        Text::GraphLinesColor => "Couleur des lignes du graphique",
        Text::GraphLinesColorDescription => {
            "La couleur des lignes reliant les points du graphique."
        }
        Text::GraphPartialFillColor => "Couleur de remplissage partiel",
        Text::GraphPartialFillColorDescription => {
            "La couleur de la zone entre l’axe des x et le graphique. La couleur partielle est utilisée uniquement pour les changements en direct, entre le dernier split et le temps actuel."
        }
        Text::GraphCompleteFillColor => "Couleur de remplissage complet",
        Text::GraphCompleteFillColorDescription => {
            "La couleur de la zone entre l’axe des x et le graphique, en excluant le segment avec les changements en direct."
        }
        Text::DetailedTimerBackground => "Arrière-plan",
        Text::DetailedTimerBackgroundDescription => "L’arrière-plan affiché derrière le composant.",
        Text::DetailedTimerTimingMethod => "Méthode de timing",
        Text::DetailedTimerTimingMethodDescription => {
            "Indique la méthode de timing à utiliser. Si non précisée, la méthode actuelle est utilisée."
        }
        Text::DetailedTimerComparison1 => "Comparaison 1",
        Text::DetailedTimerComparison1Description => {
            "La première comparaison dont le temps de segment est affiché. Si non précisée, la comparaison actuelle est utilisée."
        }
        Text::DetailedTimerComparison2 => "Comparaison 2",
        Text::DetailedTimerComparison2Description => {
            "La seconde comparaison dont le temps de segment est affiché. Si non précisée, la comparaison actuelle est utilisée, sauf si la première comparaison est aussi None. Non affichée si la seconde comparaison est masquée."
        }
        Text::DetailedTimerHideSecondComparison => "Masquer la deuxième comparaison",
        Text::DetailedTimerHideSecondComparisonDescription => {
            "Indique s’il faut n’afficher qu’une seule comparaison."
        }
        Text::DetailedTimerTimerHeight => "Hauteur du timer",
        Text::DetailedTimerTimerHeightDescription => "La hauteur du timer principal.",
        Text::DetailedTimerSegmentTimerHeight => "Hauteur du timer de segment",
        Text::DetailedTimerSegmentTimerHeightDescription => "La hauteur du timer de segment.",
        Text::DetailedTimerTimerColor => "Couleur du timer",
        Text::DetailedTimerTimerColorDescription => {
            "Au lieu de déterminer automatiquement la couleur du timer principal, une couleur fixe peut être fournie."
        }
        Text::DetailedTimerShowTimerGradient => "Afficher le dégradé du timer",
        Text::DetailedTimerShowTimerGradientDescription => {
            "Le timer principal transforme automatiquement sa couleur en dégradé vertical si cette option est activée. Sinon, la couleur réelle est utilisée."
        }
        Text::DetailedTimerTimerDigitsFormat => "Format des chiffres du timer",
        Text::DetailedTimerTimerDigitsFormatDescription => {
            "Indique combien de chiffres afficher pour le timer principal. Si la durée est inférieure, des zéros sont affichés."
        }
        Text::DetailedTimerTimerAccuracy => "Précision du timer",
        Text::DetailedTimerTimerAccuracyDescription => {
            "La précision du temps affiché pour le timer principal."
        }
        Text::DetailedTimerSegmentTimerColor => "Couleur du timer de segment",
        Text::DetailedTimerSegmentTimerColorDescription => {
            "Change la couleur du timer de segment par une couleur différente de la couleur par défaut."
        }
        Text::DetailedTimerShowSegmentTimerGradient => "Afficher le dégradé du timer de segment",
        Text::DetailedTimerShowSegmentTimerGradientDescription => {
            "Le timer de segment transforme automatiquement sa couleur en dégradé vertical si cette option est activée. Sinon, la couleur réelle est utilisée."
        }
        Text::DetailedTimerSegmentTimerDigitsFormat => "Format des chiffres du timer de segment",
        Text::DetailedTimerSegmentTimerDigitsFormatDescription => {
            "Indique combien de chiffres afficher pour le timer de segment. Si la durée est inférieure, des zéros sont affichés."
        }
        Text::DetailedTimerSegmentTimerAccuracy => "Précision du timer de segment",
        Text::DetailedTimerSegmentTimerAccuracyDescription => {
            "La précision du temps affiché pour le timer de segment."
        }
        Text::DetailedTimerComparisonNamesColor => "Couleur des noms de comparaison",
        Text::DetailedTimerComparisonNamesColorDescription => {
            "La couleur des noms de comparaison s’ils sont affichés. Si non précisée, la couleur du layout est utilisée."
        }
        Text::DetailedTimerComparisonTimesColor => "Couleur des temps de comparaison",
        Text::DetailedTimerComparisonTimesColorDescription => {
            "La couleur des temps de comparaison s’ils sont affichés. Si non précisée, la couleur du layout est utilisée."
        }
        Text::DetailedTimerComparisonTimesAccuracy => "Précision des temps de comparaison",
        Text::DetailedTimerComparisonTimesAccuracyDescription => {
            "La précision des temps de comparaison."
        }
        Text::DetailedTimerShowSegmentName => "Afficher le nom du segment",
        Text::DetailedTimerShowSegmentNameDescription => {
            "Indique si le nom du segment doit être affiché."
        }
        Text::DetailedTimerSegmentNameColor => "Couleur du nom du segment",
        Text::DetailedTimerSegmentNameColorDescription => {
            "La couleur du nom du segment s’il est affiché. Si non précisée, la couleur du layout est utilisée."
        }
        Text::DetailedTimerDisplayIcon => "Afficher l’icône",
        Text::DetailedTimerDisplayIconDescription => {
            "Indique si l’icône du segment doit être affichée."
        }
        Text::SplitsBackground => "Arrière-plan",
        Text::SplitsBackgroundDescription => {
            "L’arrière-plan affiché derrière le composant. Vous pouvez choisir des couleurs alternées, chaque ligne alternant entre les deux couleurs."
        }
        Text::SplitsTotalRows => "Nombre total de lignes",
        Text::SplitsTotalRowsDescription => {
            "Le nombre total de lignes de segments à afficher. 0 affiche tous les segments. Une valeur inférieure affiche une fenêtre qui peut défiler."
        }
        Text::SplitsUpcomingSegments => "Segments à venir",
        Text::SplitsUpcomingSegmentsDescription => {
            "Si plus de segments que de lignes sont affichés, la fenêtre défile automatiquement. Ce nombre détermine le minimum de segments futurs affichés."
        }
        Text::SplitsShowThinSeparators => "Afficher les séparateurs fins",
        Text::SplitsShowThinSeparatorsDescription => {
            "Indique si des séparateurs fins doivent être affichés entre les lignes de segments."
        }
        Text::SplitsShowSeparatorBeforeLastSplit => "Afficher un séparateur avant le dernier split",
        Text::SplitsShowSeparatorBeforeLastSplitDescription => {
            "Si le dernier segment est toujours affiché, indique si un séparateur plus prononcé doit être montré avant lui lorsqu’il n’est pas adjacent au segment précédent."
        }
        Text::SplitsAlwaysShowLastSplit => "Toujours afficher le dernier split",
        Text::SplitsAlwaysShowLastSplitDescription => {
            "Si tous les segments ne sont pas affichés, cette option indique si le dernier segment doit toujours être visible, car il contient la durée totale de la comparaison choisie."
        }
        Text::SplitsFillWithBlankSpace => "Remplir avec des espaces vides",
        Text::SplitsFillWithBlankSpaceDescription => {
            "S’il n’y a pas assez de segments pour remplir la liste, cette option permet de remplir les lignes restantes avec des espaces vides pour conserver le nombre de lignes total."
        }
        Text::SplitsShowTimesBelowSegmentName => "Afficher les temps sous le nom du segment",
        Text::SplitsShowTimesBelowSegmentNameDescription => {
            "Indique si les temps doivent être affichés sous le nom du segment. Sinon, ils sont affichés à côté."
        }
        Text::SplitsCurrentSegmentGradient => "Dégradé du segment actuel",
        Text::SplitsCurrentSegmentGradientDescription => {
            "Le dégradé affiché derrière le segment actuel pour l’indiquer."
        }
        Text::SplitsSplitTimeAccuracy => "Précision des temps de split",
        Text::SplitsSplitTimeAccuracyDescription => {
            "Indique la précision utilisée pour les colonnes affichant des temps de split."
        }
        Text::SplitsSegmentTimeAccuracy => "Précision des temps de segment",
        Text::SplitsSegmentTimeAccuracyDescription => {
            "Indique la précision utilisée pour les colonnes affichant des temps de segment."
        }
        Text::SplitsDeltaTimeAccuracy => "Précision du delta",
        Text::SplitsDeltaTimeAccuracyDescription => {
            "Indique la précision utilisée pour les colonnes affichant l’avance ou le retard."
        }
        Text::SplitsDropDeltaDecimals => "Supprimer les décimales du delta quand minutes",
        Text::SplitsDropDeltaDecimalsDescription => {
            "Indique si les décimales ne doivent plus être affichées lorsqu’une colonne de delta dépasse une minute."
        }
        Text::SplitsShowColumnLabels => "Afficher les en-têtes de colonnes",
        Text::SplitsShowColumnLabelsDescription => {
            "Indique si les noms des colonnes doivent être affichés en haut de la liste."
        }
        Text::SplitsColumns => "Colonnes",
        Text::SplitsColumnsDescription => {
            "Le nombre de colonnes à afficher par ligne. Chaque colonne peut afficher des informations différentes. Les colonnes sont définies de droite à gauche."
        }
        Text::SplitsColumnName => "Nom de colonne",
        Text::SplitsColumnNameDescription => {
            "Le nom de la colonne. Il est affiché en haut de la liste si les en-têtes de colonnes sont activés."
        }
        Text::SplitsColumnType => "Type de colonne",
        Text::SplitsColumnTypeDescription => {
            "Le type d’information affichée par cette colonne. Cela peut être un temps ou une variable personnalisée."
        }
        Text::SplitsVariableName => "Nom de variable",
        Text::SplitsVariableNameDescription => {
            "Le nom de la variable personnalisée affichée dans cette colonne."
        }
        Text::SplitsStartWith => "Commencer avec",
        Text::SplitsStartWithDescription => {
            "La valeur de départ pour chaque segment. Le déclencheur de mise à jour indique quand elle est remplacée."
        }
        Text::SplitsUpdateWith => "Mettre à jour avec",
        Text::SplitsUpdateWithDescription => {
            "Une fois une condition remplie (souvent être sur le segment ou l’avoir terminé), la valeur est mise à jour avec celle indiquée ici."
        }
        Text::SplitsUpdateTrigger => "Déclencheur de mise à jour",
        Text::SplitsUpdateTriggerDescription => {
            "La condition à remplir pour mettre à jour la valeur. Avant cette condition, la valeur est celle du champ « Commencer avec »."
        }
        Text::SplitsColumnComparison => "Comparaison",
        Text::SplitsColumnComparisonDescription => {
            "La comparaison utilisée pour cette colonne. Si non précisée, la comparaison actuelle est utilisée."
        }
        Text::SplitsColumnTimingMethod => "Méthode de timing",
        Text::SplitsColumnTimingMethodDescription => {
            "Indique la méthode de timing à utiliser pour cette colonne. Si non précisée, la méthode actuelle est utilisée."
        }
        Text::TextComponentBackground => "Arrière-plan",
        Text::TextComponentBackgroundDescription => "L’arrière-plan affiché derrière le composant.",
        Text::TextComponentUseVariable => "Utiliser une variable",
        Text::TextComponentUseVariableDescription => {
            "Indique s’il faut utiliser une variable personnalisée pour afficher une valeur dynamique. Les variables peuvent être définies dans l’éditeur de splits et fournies par les auto splitters."
        }
        Text::TextComponentSplit => "Séparer",
        Text::TextComponentSplitDescription => {
            "Indique si le texte doit être séparé en une partie gauche et une partie droite. Sinon, seul un texte centré est affiché."
        }
        Text::TextComponentText => "Texte",
        Text::TextComponentTextDescription => "Indique le texte à afficher au centre.",
        Text::TextComponentLeft => "Gauche",
        Text::TextComponentLeftDescription => "Indique le texte à afficher à gauche.",
        Text::TextComponentRight => "Droite",
        Text::TextComponentRightDescription => "Indique le texte à afficher à droite.",
        Text::TextComponentVariable => "Variable",
        Text::TextComponentVariableDescription => {
            "Indique le nom de la variable personnalisée à afficher."
        }
        Text::TextComponentTextColor => "Couleur du texte",
        Text::TextComponentTextColorDescription => "La couleur du texte.",
        Text::TextComponentLeftColor => "Couleur de gauche",
        Text::TextComponentLeftColorDescription => "La couleur du texte à gauche.",
        Text::TextComponentRightColor => "Couleur de droite",
        Text::TextComponentRightColorDescription => "La couleur du texte à droite.",
        Text::TextComponentNameColor => "Couleur du nom",
        Text::TextComponentNameColorDescription => "La couleur du nom de la variable.",
        Text::TextComponentValueColor => "Couleur de la valeur",
        Text::TextComponentValueColorDescription => "La couleur de la valeur de la variable.",
        Text::TextComponentDisplayTwoRows => "Afficher sur 2 lignes",
        Text::TextComponentDisplayTwoRowsDescription => {
            "Indique si les textes gauche et droite doivent être affichés sur deux lignes distinctes."
        }
        Text::LayoutDirection => "Direction de la mise en page",
        Text::LayoutDirectionDescription => {
            "La direction dans laquelle les composants sont disposés."
        }
        Text::CustomTimerFont => "Police personnalisée du timer",
        Text::CustomTimerFontDescription => {
            "Permet de spécifier une police personnalisée pour le timer. Si non définie, la police par défaut est utilisée."
        }
        Text::CustomTimesFont => "Police personnalisée des temps",
        Text::CustomTimesFontDescription => {
            "Permet de spécifier une police personnalisée pour les temps. Si non définie, la police par défaut est utilisée."
        }
        Text::CustomTextFont => "Police personnalisée du texte",
        Text::CustomTextFontDescription => {
            "Permet de spécifier une police personnalisée pour le texte. Si non définie, la police par défaut est utilisée."
        }
        Text::TextShadow => "Ombre du texte",
        Text::TextShadowDescription => {
            "Permet de spécifier une couleur optionnelle pour les ombres de texte."
        }
        Text::Background => "Arrière-plan",
        Text::BackgroundDescription => "L’arrière-plan affiché derrière l’ensemble du layout.",
        Text::BestSegment => "Meilleur segment",
        Text::BestSegmentDescription => {
            "La couleur utilisée lorsque vous réalisez un nouveau meilleur segment."
        }
        Text::AheadGainingTime => "En avance (gain de temps)",
        Text::AheadGainingTimeDescription => {
            "La couleur utilisée lorsque vous êtes en avance et gagnez encore du temps."
        }
        Text::AheadLosingTime => "En avance (perte de temps)",
        Text::AheadLosingTimeDescription => {
            "La couleur utilisée lorsque vous êtes en avance mais perdez du temps."
        }
        Text::BehindGainingTime => "En retard (rattrape du temps)",
        Text::BehindGainingTimeDescription => {
            "La couleur utilisée lorsque vous êtes en retard mais rattrapez du temps."
        }
        Text::BehindLosingTime => "En retard (perte de temps)",
        Text::BehindLosingTimeDescription => {
            "La couleur utilisée lorsque vous êtes en retard et perdez encore du temps."
        }
        Text::NotRunning => "Pas en cours",
        Text::NotRunningDescription => {
            "La couleur utilisée lorsqu’aucune tentative n’est en cours."
        }
        Text::PersonalBest => "Record personnel",
        Text::PersonalBestDescription => {
            "La couleur utilisée lorsque vous obtenez un nouveau record personnel."
        }
        Text::Paused => "En pause",
        Text::PausedDescription => "La couleur utilisée lorsque le timer est en pause.",
        Text::ThinSeparators => "Séparateurs fins",
        Text::ThinSeparatorsDescription => "La couleur des séparateurs fins.",
        Text::Separators => "Séparateurs",
        Text::SeparatorsDescription => "La couleur des séparateurs normaux.",
        Text::TextColor => "Texte",
        Text::TextColorDescription => {
            "La couleur utilisée pour le texte qui ne précise pas sa propre couleur."
        }
        Text::ComponentBlankSpace => "Espace vide",
        Text::ComponentCurrentComparison => "Comparaison actuelle",
        Text::ComponentCurrentPace => "Rythme actuel",
        Text::ComponentDelta => "Delta",
        Text::ComponentDetailedTimer => "Timer détaillé",
        Text::ComponentGraph => "Graphique",
        Text::ComponentPbChance => "Chance de PB",
        Text::ComponentPossibleTimeSave => "Gain de temps possible",
        Text::ComponentPreviousSegment => "Segment précédent",
        Text::ComponentSegmentTime => "Temps du segment",
        Text::ComponentSeparator => "Séparateur",
        Text::ComponentSplits => "Splits",
        Text::ComponentSumOfBest => "Somme des meilleurs",
        Text::ComponentText => "Texte",
        Text::ComponentTimer => "Timer",
        Text::ComponentSegmentTimer => "Minuteur de segment",
        Text::ComponentTitle => "Titre",
        Text::ComponentTotalPlaytime => "Temps de jeu total",
        Text::ComponentCurrentPaceBestPossibleTime => "Meilleur temps possible",
        Text::ComponentCurrentPaceWorstPossibleTime => "Pire temps possible",
        Text::ComponentCurrentPacePredictedTime => "Temps prédit",
        Text::ComponentSegmentTimeBest => "Meilleur temps de segment",
        Text::ComponentSegmentTimeWorst => "Pire temps de segment",
        Text::ComponentSegmentTimeAverage => "Temps moyen de segment",
        Text::ComponentSegmentTimeMedian => "Temps médian de segment",
        Text::ComponentSegmentTimeLatest => "Dernier temps de segment",
        Text::ComponentPossibleTimeSaveTotal => "Temps total potentiellement gagné",
        Text::LiveSegment => "Segment en direct",
        Text::LiveSegmentShort => "Segment en direct",
        Text::PreviousSegmentShort => "Seg. précédent",
        Text::PreviousSegmentAbbreviation => "Seg. préc.",
        Text::ComparingAgainst => "Comparé à",
        Text::ComparisonShort => "Comparaison",
        Text::CurrentPaceBestPossibleTimeShort => "Meill. temps poss.",
        Text::CurrentPaceBestTimeShort => "Meill. temps",
        Text::CurrentPaceBestPossibleTimeAbbreviation => "MTP",
        Text::CurrentPaceWorstPossibleTimeShort => "Pire temps poss.",
        Text::CurrentPaceWorstTimeShort => "Pire temps",
        Text::CurrentPacePredictedTimeShort => "Temps préd.",
        Text::CurrentPaceShort => "Rythme act.",
        Text::CurrentPaceAbbreviation => "Rythme",
        Text::Goal => "Objectif",
        Text::SumOfBestSegments => "Somme des meilleurs segments",
        Text::SumOfBestShort => "Somme des meilleurs",
        Text::SumOfBestAbbreviation => "SoB",
        Text::PlaytimeShort => "Temps de jeu",
        Text::BestSegmentTimeShort => "Meil. tps seg.",
        Text::BestSegmentShort => "Meilleur segment",
        Text::WorstSegmentTimeShort => "Pire tps seg.",
        Text::WorstSegmentShort => "Pire segment",
        Text::AverageSegmentTimeShort => "Tps seg. moyen",
        Text::AverageSegmentShort => "Segment moyen",
        Text::MedianSegmentTimeShort => "Tps seg. méd.",
        Text::MedianSegmentShort => "Segment médian",
        Text::LatestSegmentTimeShort => "Dern. tps seg.",
        Text::LatestSegmentShort => "Dernier segment",
        Text::SegmentTimeShort => "Tps seg.",
        Text::SplitTime => "Temps",
        Text::PossibleTimeSaveShort => "Temps potentiellement gagné",
        Text::PossibleTimeSaveAbbreviation => "Tps pot. gagné",
        Text::TimeSaveShort => "Temps gagné",
        Text::RealTime => "Temps réel",
        Text::GameTime => "Temps de jeu",
        Text::Untitled => "Sans titre",
        Text::SumOfBestCleanerStartOfRun => "le début du run",
        Text::SumOfBestCleanerShouldRemove => {
            ". Pensez-vous que ce temps de segment est inexact et devrait être supprimé ?"
        }
    }
}

pub const fn resolve_placeholder(text: PlaceholderText) -> &'static [Piece] {
    match text {
        PlaceholderText::SumOfBestCleanerSegmentTimeBetween => &[
            Piece::Static("Vous avez eu un temps de segment en "),
            Piece::Dynamic(0),
            Piece::Static(" de "),
            Piece::Dynamic(1),
            Piece::Static(" entre « "),
            Piece::Dynamic(2),
            Piece::Static(" » et « "),
            Piece::Dynamic(3),
            Piece::Static(" »"),
        ],
        PlaceholderText::SumOfBestCleanerFasterThanCombined => &[
            Piece::Static(", ce qui est plus rapide que la combinaison des meilleurs segments de "),
            Piece::Dynamic(0),
        ],
        PlaceholderText::SumOfBestCleanerRunOn => &[
            Piece::Static(" lors d’un run le "),
            Piece::Dynamic(0),
            Piece::Static(" qui a commencé à "),
            Piece::Dynamic(1),
        ],
    }
}
