use super::{Piece, PlaceholderText, Text};

pub const fn resolve(text: Text) -> &'static str {
    match text {
        Text::StartSplit => "Iniciar / Split",
        Text::StartSplitDescription => "El atajo para hacer split e iniciar un nuevo intento.",
        Text::Reset => "Reiniciar",
        Text::ResetDescription => "El atajo para reiniciar el intento actual.",
        Text::UndoSplit => "Deshacer split",
        Text::UndoSplitDescription => "El atajo para deshacer el último split.",
        Text::SkipSplit => "Omitir split",
        Text::SkipSplitDescription => "El atajo para omitir el split actual.",
        Text::Pause => "Pausa",
        Text::PauseDescription => {
            "El atajo para pausar el intento actual. También puede usarse para iniciar un nuevo intento."
        }
        Text::UndoAllPauses => "Deshacer todas las pausas",
        Text::UndoAllPausesDescription => {
            "El atajo para quitar todos los tiempos de pausa del tiempo actual. Útil si pausaste por error."
        }
        Text::PreviousComparison => "Comparación anterior",
        Text::PreviousComparisonDescription => "El atajo para cambiar a la comparación anterior.",
        Text::NextComparison => "Siguiente comparación",
        Text::NextComparisonDescription => "El atajo para cambiar a la siguiente comparación.",
        Text::ToggleTimingMethod => "Alternar método de cronometraje",
        Text::ToggleTimingMethodDescription => {
            "El atajo para alternar entre los métodos «Tiempo real» y «Tiempo de juego»."
        }
        Text::TimerBackground => "Fondo",
        Text::TimerBackgroundDescription => {
            "El fondo mostrado detrás del componente. También se puede aplicar el color asociado a ir por delante o por detrás como color de fondo."
        }
        Text::SegmentTimer => "Temporizador de segmento",
        Text::SegmentTimerDescription => {
            "Indica si se debe mostrar el tiempo transcurrido desde el inicio del segmento actual en lugar del inicio del intento."
        }
        Text::TimingMethod => "Método de cronometraje",
        Text::TimingMethodDescription => {
            "Indica el método de cronometraje a usar. Si no se especifica, se usa el método actual."
        }
        Text::Height => "Altura",
        Text::HeightDescription => "La altura del temporizador.",
        Text::TimerTextColor => "Color del texto",
        Text::TimerTextColorDescription => {
            "El color del tiempo mostrado. Si no se especifica, se elige automáticamente según el progreso del intento. Estos colores se pueden definir en la configuración general del layout."
        }
        Text::ShowGradient => "Mostrar degradado",
        Text::ShowGradientDescription => {
            "Determina si el color del temporizador se muestra como degradado."
        }
        Text::DigitsFormat => "Formato de dígitos",
        Text::DigitsFormatDescription => {
            "Indica cuántos dígitos mostrar. Si la duración es menor que los dígitos a mostrar, se muestran ceros."
        }
        Text::Accuracy => "Precisión",
        Text::AccuracyDescription => "La precisión del tiempo mostrado.",
        Text::TitleBackground => "Fondo",
        Text::TitleBackgroundDescription => "El fondo mostrado detrás del componente.",
        Text::TitleTextColor => "Color del texto",
        Text::TitleTextColorDescription => {
            "El color del texto del título. Si no se especifica, se usa el color del layout."
        }
        Text::ShowGameName => "Mostrar nombre del juego",
        Text::ShowGameNameDescription => {
            "Indica si el nombre del juego debe formar parte del título mostrado."
        }
        Text::ShowCategoryName => "Mostrar nombre de la categoría",
        Text::ShowCategoryNameDescription => {
            "Indica si el nombre de la categoría debe formar parte del título mostrado."
        }
        Text::ShowFinishedRunsCount => "Mostrar número de runs terminados",
        Text::ShowFinishedRunsCountDescription => {
            "Indica si debe mostrarse el número de runs finalizados con éxito."
        }
        Text::ShowAttemptCount => "Mostrar número de intentos",
        Text::ShowAttemptCountDescription => {
            "Indica si debe mostrarse el número total de intentos."
        }
        Text::TextAlignment => "Alineación del texto",
        Text::TextAlignmentDescription => "Indica la alineación del título.",
        Text::DisplayTextAsSingleLine => "Mostrar texto en una sola línea",
        Text::DisplayTextAsSingleLineDescription => {
            "Indica si el título debe mostrarse en una sola línea en lugar de separarse en una línea para el juego y otra para la categoría."
        }
        Text::DisplayGameIcon => "Mostrar icono del juego",
        Text::DisplayGameIconDescription => {
            "Indica si debe mostrarse el icono del juego, si hay un icono almacenado en los splits."
        }
        Text::ShowRegion => "Mostrar región",
        Text::ShowRegionDescription => {
            "El nombre de la categoría puede ampliarse con información adicional. Esto lo amplía con la región del juego, si se proporciona en la pestaña de variables del editor de splits."
        }
        Text::ShowPlatform => "Mostrar plataforma",
        Text::ShowPlatformDescription => {
            "El nombre de la categoría puede ampliarse con información adicional. Esto lo amplía con la plataforma en la que se juega, si se proporciona en la pestaña de variables del editor de splits."
        }
        Text::ShowVariables => "Mostrar variables",
        Text::ShowVariablesDescription => {
            "El nombre de la categoría puede ampliarse con información adicional. Esto lo amplía con variables adicionales proporcionadas en la pestaña de variables del editor de splits. Se refiere a variables de speedrun.com, no a variables personalizadas."
        }
        Text::TotalPlaytimeBackground => "Fondo",
        Text::TotalPlaytimeBackgroundDescription => "El fondo mostrado detrás del componente.",
        Text::DisplayTwoRows => "Mostrar en 2 filas",
        Text::DisplayTwoRowsDescription => {
            "Indica si se debe mostrar el nombre del componente y el tiempo total de juego en dos filas separadas."
        }
        Text::ShowDays => "Mostrar días (>24h)",
        Text::ShowDaysDescription => {
            "Indica si se debe mostrar el número de días cuando el tiempo total alcanza 24 horas o más."
        }
        Text::LabelColor => "Color de etiqueta",
        Text::LabelColorDescription => {
            "El color del nombre del componente. Si no se especifica, se usa el color del layout."
        }
        Text::ValueColor => "Color del valor",
        Text::ValueColorDescription => {
            "El color del tiempo total de juego. Si no se especifica, se usa el color del layout."
        }
        Text::BlankSpaceBackground => "Fondo",
        Text::BlankSpaceBackgroundDescription => "El fondo mostrado detrás del componente.",
        Text::BlankSpaceSize => "Tamaño",
        Text::BlankSpaceSizeDescription => "El tamaño del componente.",
        Text::CurrentComparisonBackground => "Fondo",
        Text::CurrentComparisonBackgroundDescription => "El fondo mostrado detrás del componente.",
        Text::CurrentComparisonDisplayTwoRows => "Mostrar en 2 filas",
        Text::CurrentComparisonDisplayTwoRowsDescription => {
            "Indica si se debe mostrar el nombre del componente y la comparación en dos filas separadas."
        }
        Text::CurrentComparisonLabelColor => "Color de etiqueta",
        Text::CurrentComparisonLabelColorDescription => {
            "El color del nombre del componente. Si no se especifica, se usa el color del layout."
        }
        Text::CurrentComparisonValueColor => "Color del valor",
        Text::CurrentComparisonValueColorDescription => {
            "El color del nombre de la comparación. Si no se especifica, se usa el color del layout."
        }
        Text::CurrentPaceBackground => "Fondo",
        Text::CurrentPaceBackgroundDescription => "El fondo mostrado detrás del componente.",
        Text::CurrentPaceComparison => "Comparación",
        Text::CurrentPaceComparisonDescription => {
            "La comparación para predecir el tiempo final. Si no se especifica, se usa la comparación actual."
        }
        Text::CurrentPaceDisplayTwoRows => "Mostrar en 2 filas",
        Text::CurrentPaceDisplayTwoRowsDescription => {
            "Indica si se debe mostrar el nombre del componente y el tiempo predicho en dos filas separadas."
        }
        Text::CurrentPaceLabelColor => "Color de etiqueta",
        Text::CurrentPaceLabelColorDescription => {
            "El color del nombre del componente. Si no se especifica, se usa el color del layout."
        }
        Text::CurrentPaceValueColor => "Color del valor",
        Text::CurrentPaceValueColorDescription => {
            "El color del tiempo predicho. Si no se especifica, se usa el color del layout."
        }
        Text::CurrentPaceAccuracy => "Precisión",
        Text::CurrentPaceAccuracyDescription => "La precisión del tiempo predicho mostrado.",
        Text::DeltaBackground => "Fondo",
        Text::DeltaBackgroundDescription => "El fondo mostrado detrás del componente.",
        Text::DeltaComparison => "Comparación",
        Text::DeltaComparisonDescription => {
            "La comparación usada para calcular cuánto vas por delante o por detrás. Si no se especifica, se usa la comparación actual."
        }
        Text::DeltaDisplayTwoRows => "Mostrar en 2 filas",
        Text::DeltaDisplayTwoRowsDescription => {
            "Indica si se debe mostrar el nombre de la comparación y el delta en dos filas separadas."
        }
        Text::DeltaLabelColor => "Color de etiqueta",
        Text::DeltaLabelColorDescription => {
            "El color del nombre de la comparación. Si no se especifica, se usa el color del layout."
        }
        Text::DeltaDropDecimals => "Quitar decimales",
        Text::DeltaDropDecimalsDescription => {
            "Indica si los decimales deben ocultarse cuando el delta visualizado supera un minuto."
        }
        Text::DeltaAccuracy => "Precisión",
        Text::DeltaAccuracyDescription => "La precisión del delta mostrado.",
        Text::SumOfBestBackground => "Fondo",
        Text::SumOfBestBackgroundDescription => "El fondo mostrado detrás del componente.",
        Text::SumOfBestDisplayTwoRows => "Mostrar en 2 filas",
        Text::SumOfBestDisplayTwoRowsDescription => {
            "Indica si se debe mostrar el nombre del componente y la suma de mejores segmentos en dos filas separadas."
        }
        Text::SumOfBestLabelColor => "Color de etiqueta",
        Text::SumOfBestLabelColorDescription => {
            "El color del nombre del componente. Si no se especifica, se usa el color del layout."
        }
        Text::SumOfBestValueColor => "Color del valor",
        Text::SumOfBestValueColorDescription => {
            "El color de la suma de mejores segmentos. Si no se especifica, se usa el color del layout."
        }
        Text::SumOfBestAccuracy => "Precisión",
        Text::SumOfBestAccuracyDescription => {
            "La precisión de la suma de mejores segmentos mostrada."
        }
        Text::PbChanceBackground => "Fondo",
        Text::PbChanceBackgroundDescription => "El fondo mostrado detrás del componente.",
        Text::PbChanceDisplayTwoRows => "Mostrar en 2 filas",
        Text::PbChanceDisplayTwoRowsDescription => {
            "Indica si se debe mostrar el nombre del componente y la probabilidad de PB en dos filas separadas."
        }
        Text::PbChanceLabelColor => "Color de etiqueta",
        Text::PbChanceLabelColorDescription => {
            "El color del nombre del componente. Si no se especifica, se usa el color del layout."
        }
        Text::PbChanceValueColor => "Color del valor",
        Text::PbChanceValueColorDescription => {
            "El color de la probabilidad de PB. Si no se especifica, se usa el color del layout."
        }
        Text::PossibleTimeSaveBackground => "Fondo",
        Text::PossibleTimeSaveBackgroundDescription => "El fondo mostrado detrás del componente.",
        Text::PossibleTimeSaveComparison => "Comparación",
        Text::PossibleTimeSaveComparisonDescription => {
            "La comparación para calcular el posible ahorro de tiempo. Si no se especifica, se usa la comparación actual."
        }
        Text::PossibleTimeSaveDisplayTwoRows => "Mostrar en 2 filas",
        Text::PossibleTimeSaveDisplayTwoRowsDescription => {
            "Indica si se debe mostrar el nombre del componente y el posible ahorro de tiempo en dos filas separadas."
        }
        Text::PossibleTimeSaveShowTotal => "Mostrar ahorro total posible",
        Text::PossibleTimeSaveShowTotalDescription => {
            "Indica si se debe mostrar el ahorro total posible para el resto del intento en lugar del ahorro del segmento actual."
        }
        Text::PossibleTimeSaveLabelColor => "Color de etiqueta",
        Text::PossibleTimeSaveLabelColorDescription => {
            "El color del nombre del componente. Si no se especifica, se usa el color del layout."
        }
        Text::PossibleTimeSaveValueColor => "Color del valor",
        Text::PossibleTimeSaveValueColorDescription => {
            "El color del posible ahorro de tiempo. Si no se especifica, se usa el color del layout."
        }
        Text::PossibleTimeSaveAccuracy => "Precisión",
        Text::PossibleTimeSaveAccuracyDescription => {
            "La precisión del posible ahorro de tiempo mostrado."
        }
        Text::PreviousSegmentBackground => "Fondo",
        Text::PreviousSegmentBackgroundDescription => "El fondo mostrado detrás del componente.",
        Text::PreviousSegmentComparison => "Comparación",
        Text::PreviousSegmentComparisonDescription => {
            "La comparación usada para calcular cuánto tiempo se ganó o perdió. Si no se especifica, se usa la comparación actual."
        }
        Text::PreviousSegmentDisplayTwoRows => "Mostrar en 2 filas",
        Text::PreviousSegmentDisplayTwoRowsDescription => {
            "Indica si se debe mostrar el nombre del componente y el tiempo ganado o perdido en dos filas separadas."
        }
        Text::PreviousSegmentLabelColor => "Color de etiqueta",
        Text::PreviousSegmentLabelColorDescription => {
            "El color del nombre del componente. Si no se especifica, se usa el color del layout."
        }
        Text::PreviousSegmentDropDecimals => "Quitar decimales",
        Text::PreviousSegmentDropDecimalsDescription => {
            "Indica si deben ocultarse los decimales cuando el tiempo mostrado supera un minuto."
        }
        Text::PreviousSegmentAccuracy => "Precisión",
        Text::PreviousSegmentAccuracyDescription => "La precisión del tiempo mostrado.",
        Text::PreviousSegmentShowPossibleTimeSave => "Mostrar posible ahorro de tiempo",
        Text::PreviousSegmentShowPossibleTimeSaveDescription => {
            "Indica si se debe mostrar, además del tiempo ganado o perdido, cuánto tiempo se podría haber ahorrado en el segmento anterior."
        }
        Text::SegmentTimeBackground => "Fondo",
        Text::SegmentTimeBackgroundDescription => "El fondo mostrado detrás del componente.",
        Text::SegmentTimeComparison => "Comparación",
        Text::SegmentTimeComparisonDescription => {
            "La comparación para el tiempo de segmento. Si no se especifica, se usa la comparación actual."
        }
        Text::SegmentTimeDisplayTwoRows => "Mostrar en 2 filas",
        Text::SegmentTimeDisplayTwoRowsDescription => {
            "Indica si se debe mostrar el nombre del componente y el tiempo de segmento en dos filas separadas."
        }
        Text::SegmentTimeLabelColor => "Color de etiqueta",
        Text::SegmentTimeLabelColorDescription => {
            "El color del nombre del componente. Si no se especifica, se usa el color del layout."
        }
        Text::SegmentTimeValueColor => "Color del valor",
        Text::SegmentTimeValueColorDescription => {
            "El color del tiempo de segmento. Si no se especifica, se usa el color del layout."
        }
        Text::SegmentTimeAccuracy => "Precisión",
        Text::SegmentTimeAccuracyDescription => "La precisión del tiempo de segmento mostrado.",
        Text::GraphComparison => "Comparación",
        Text::GraphComparisonDescription => {
            "La comparación a usar para el gráfico. Si no se especifica, se usa la comparación actual."
        }
        Text::GraphHeight => "Altura",
        Text::GraphHeightDescription => "La altura del gráfico.",
        Text::GraphShowBestSegments => "Mostrar mejores segmentos",
        Text::GraphShowBestSegmentsDescription => {
            "Indica si los mejores segmentos deben colorearse con el color de mejor segmento del layout."
        }
        Text::GraphLiveGraph => "Gráfico en vivo",
        Text::GraphLiveGraphDescription => {
            "Indica si el gráfico debe actualizarse continuamente. Si se desactiva, los cambios solo ocurren cuando cambia el segmento actual."
        }
        Text::GraphFlipGraph => "Invertir gráfico",
        Text::GraphFlipGraphDescription => {
            "Indica si el gráfico debe invertirse verticalmente. Si no se activa, los tiempos por delante se muestran debajo del eje x y los tiempos por detrás por encima."
        }
        Text::GraphBehindBackgroundColor => "Color de fondo (detrás)",
        Text::GraphBehindBackgroundColorDescription => {
            "El color de fondo de la zona del gráfico con tiempos por detrás de la comparación."
        }
        Text::GraphAheadBackgroundColor => "Color de fondo (delante)",
        Text::GraphAheadBackgroundColorDescription => {
            "El color de fondo de la zona del gráfico con tiempos por delante de la comparación."
        }
        Text::GraphGridLinesColor => "Color de líneas de rejilla",
        Text::GraphGridLinesColorDescription => "El color de las líneas de rejilla del gráfico.",
        Text::GraphLinesColor => "Color de líneas del gráfico",
        Text::GraphLinesColorDescription => {
            "El color de las líneas que conectan los puntos del gráfico."
        }
        Text::GraphPartialFillColor => "Color de relleno parcial",
        Text::GraphPartialFillColorDescription => {
            "El color de la región entre el eje x y el gráfico. El relleno parcial se usa solo para cambios en vivo, desde el último split hasta el tiempo actual."
        }
        Text::GraphCompleteFillColor => "Color de relleno completo",
        Text::GraphCompleteFillColorDescription => {
            "El color de la región entre el eje x y el gráfico, excluyendo el segmento con cambios en vivo."
        }
        Text::DetailedTimerBackground => "Fondo",
        Text::DetailedTimerBackgroundDescription => "El fondo mostrado detrás del componente.",
        Text::DetailedTimerTimingMethod => "Método de cronometraje",
        Text::DetailedTimerTimingMethodDescription => {
            "Indica el método de cronometraje a usar. Si no se especifica, se usa el método actual."
        }
        Text::DetailedTimerComparison1 => "Comparación 1",
        Text::DetailedTimerComparison1Description => {
            "La primera comparación de la que se muestra el tiempo de segmento. Si no se especifica, se usa la comparación actual."
        }
        Text::DetailedTimerComparison2 => "Comparación 2",
        Text::DetailedTimerComparison2Description => {
            "La segunda comparación de la que se muestra el tiempo de segmento. Si no se especifica, se usa la comparación actual, a menos que la primera también sea None. No se muestra si la segunda comparación está oculta."
        }
        Text::DetailedTimerHideSecondComparison => "Ocultar segunda comparación",
        Text::DetailedTimerHideSecondComparisonDescription => {
            "Indica si solo se debe mostrar una comparación."
        }
        Text::DetailedTimerTimerHeight => "Altura del temporizador",
        Text::DetailedTimerTimerHeightDescription => "La altura del temporizador principal.",
        Text::DetailedTimerSegmentTimerHeight => "Altura del temporizador de segmento",
        Text::DetailedTimerSegmentTimerHeightDescription => {
            "La altura del temporizador de segmento."
        }
        Text::DetailedTimerTimerColor => "Color del temporizador",
        Text::DetailedTimerTimerColorDescription => {
            "En lugar de determinar automáticamente el color del temporizador principal, se puede proporcionar un color fijo."
        }
        Text::DetailedTimerShowTimerGradient => "Mostrar degradado del temporizador",
        Text::DetailedTimerShowTimerGradientDescription => {
            "El temporizador principal convierte su color en un degradado vertical si esta opción está activada. En caso contrario, se usa el color real."
        }
        Text::DetailedTimerTimerDigitsFormat => "Formato de dígitos del temporizador",
        Text::DetailedTimerTimerDigitsFormatDescription => {
            "Indica cuántos dígitos mostrar para el temporizador principal. Si la duración es menor, se muestran ceros."
        }
        Text::DetailedTimerTimerAccuracy => "Precisión del temporizador",
        Text::DetailedTimerTimerAccuracyDescription => {
            "La precisión del tiempo mostrado para el temporizador principal."
        }
        Text::DetailedTimerSegmentTimerColor => "Color del temporizador de segmento",
        Text::DetailedTimerSegmentTimerColorDescription => {
            "Cambia el color del temporizador de segmento a un color distinto del predeterminado."
        }
        Text::DetailedTimerShowSegmentTimerGradient => {
            "Mostrar degradado del temporizador de segmento"
        }
        Text::DetailedTimerShowSegmentTimerGradientDescription => {
            "El temporizador de segmento convierte su color en un degradado vertical si esta opción está activada. En caso contrario, se usa el color real."
        }
        Text::DetailedTimerSegmentTimerDigitsFormat => {
            "Formato de dígitos del temporizador de segmento"
        }
        Text::DetailedTimerSegmentTimerDigitsFormatDescription => {
            "Indica cuántos dígitos mostrar para el temporizador de segmento. Si la duración es menor, se muestran ceros."
        }
        Text::DetailedTimerSegmentTimerAccuracy => "Precisión del temporizador de segmento",
        Text::DetailedTimerSegmentTimerAccuracyDescription => {
            "La precisión del tiempo mostrado para el temporizador de segmento."
        }
        Text::DetailedTimerComparisonNamesColor => "Color de nombres de comparación",
        Text::DetailedTimerComparisonNamesColorDescription => {
            "El color de los nombres de comparación si se muestran. Si no se especifica, se usa el color del layout."
        }
        Text::DetailedTimerComparisonTimesColor => "Color de tiempos de comparación",
        Text::DetailedTimerComparisonTimesColorDescription => {
            "El color de los tiempos de comparación si se muestran. Si no se especifica, se usa el color del layout."
        }
        Text::DetailedTimerComparisonTimesAccuracy => "Precisión de tiempos de comparación",
        Text::DetailedTimerComparisonTimesAccuracyDescription => {
            "La precisión de los tiempos de comparación."
        }
        Text::DetailedTimerShowSegmentName => "Mostrar nombre del segmento",
        Text::DetailedTimerShowSegmentNameDescription => {
            "Indica si debe mostrarse el nombre del segmento."
        }
        Text::DetailedTimerSegmentNameColor => "Color del nombre del segmento",
        Text::DetailedTimerSegmentNameColorDescription => {
            "El color del nombre del segmento si se muestra. Si no se especifica, se usa el color del layout."
        }
        Text::DetailedTimerDisplayIcon => "Mostrar icono",
        Text::DetailedTimerDisplayIconDescription => {
            "Indica si debe mostrarse el icono del segmento."
        }
        Text::SplitsBackground => "Fondo",
        Text::SplitsBackgroundDescription => {
            "El fondo mostrado detrás del componente. Puedes elegir colores alternos; en ese caso cada fila alterna entre los dos colores."
        }
        Text::SplitsTotalRows => "Filas totales",
        Text::SplitsTotalRowsDescription => {
            "El número total de filas de segmentos a mostrar. Si es 0, se muestran todos. Si es menor que el total, se muestra una ventana que puede desplazarse."
        }
        Text::SplitsUpcomingSegments => "Segmentos próximos",
        Text::SplitsUpcomingSegmentsDescription => {
            "Si hay más segmentos que filas mostradas, la ventana se desplaza automáticamente cuando cambia el segmento actual. Este número determina el mínimo de segmentos futuros visibles."
        }
        Text::SplitsShowThinSeparators => "Mostrar separadores finos",
        Text::SplitsShowThinSeparatorsDescription => {
            "Indica si deben mostrarse separadores finos entre las filas de segmentos."
        }
        Text::SplitsShowSeparatorBeforeLastSplit => "Mostrar separador antes del último split",
        Text::SplitsShowSeparatorBeforeLastSplitDescription => {
            "Si el último segmento siempre se muestra, esto determina si se muestra un separador más marcado antes del último segmento cuando no está adyacente al anterior en la ventana."
        }
        Text::SplitsAlwaysShowLastSplit => "Mostrar siempre el último split",
        Text::SplitsAlwaysShowLastSplitDescription => {
            "Si no se muestran todos los segmentos, esta opción determina si el último segmento debe mostrarse siempre, ya que contiene la duración total de la comparación elegida."
        }
        Text::SplitsFillWithBlankSpace => "Rellenar con espacio en blanco",
        Text::SplitsFillWithBlankSpaceDescription => {
            "Si no hay suficientes segmentos para llenar la lista, esta opción permite rellenar las filas restantes con espacio en blanco para mantener el número total de filas."
        }
        Text::SplitsShowTimesBelowSegmentName => "Mostrar tiempos bajo el nombre del segmento",
        Text::SplitsShowTimesBelowSegmentNameDescription => {
            "Indica si los tiempos se muestran debajo del nombre del segmento. De lo contrario se muestran al lado."
        }
        Text::SplitsCurrentSegmentGradient => "Degradado del segmento actual",
        Text::SplitsCurrentSegmentGradientDescription => {
            "El degradado mostrado detrás del segmento actual como indicador."
        }
        Text::SplitsSplitTimeAccuracy => "Precisión del tiempo de split",
        Text::SplitsSplitTimeAccuracyDescription => {
            "Indica la precisión a usar para columnas con tiempos de split."
        }
        Text::SplitsSegmentTimeAccuracy => "Precisión del tiempo de segmento",
        Text::SplitsSegmentTimeAccuracyDescription => {
            "Indica la precisión a usar para columnas con tiempos de segmento."
        }
        Text::SplitsDeltaTimeAccuracy => "Precisión del delta",
        Text::SplitsDeltaTimeAccuracyDescription => {
            "Indica la precisión a usar para columnas con el tiempo por delante o por detrás."
        }
        Text::SplitsDropDeltaDecimals => "Ocultar decimales del delta al mostrar minutos",
        Text::SplitsDropDeltaDecimalsDescription => {
            "Indica si los decimales deben ocultarse cuando una columna de delta supera un minuto."
        }
        Text::SplitsShowColumnLabels => "Mostrar etiquetas de columnas",
        Text::SplitsShowColumnLabelsDescription => {
            "Indica si se deben mostrar los nombres de las columnas en la parte superior de la lista."
        }
        Text::SplitsColumns => "Columnas",
        Text::SplitsColumnsDescription => {
            "El número de columnas a mostrar por fila. Cada columna puede configurarse para mostrar distinta información. Las columnas se definen de derecha a izquierda."
        }
        Text::SplitsColumnName => "Nombre de columna",
        Text::SplitsColumnNameDescription => {
            "El nombre de la columna. Se muestra en la parte superior si las etiquetas de columnas están activadas."
        }
        Text::SplitsColumnType => "Tipo de columna",
        Text::SplitsColumnTypeDescription => {
            "El tipo de información que muestra esta columna. Puede ser un tiempo o una variable personalizada."
        }
        Text::SplitsVariableName => "Nombre de variable",
        Text::SplitsVariableNameDescription => {
            "El nombre de la variable personalizada que muestra esta columna."
        }
        Text::SplitsStartWith => "Comenzar con",
        Text::SplitsStartWithDescription => {
            "El valor con el que esta columna comienza para cada segmento. El disparador de actualización determina cuándo se reemplaza."
        }
        Text::SplitsUpdateWith => "Actualizar con",
        Text::SplitsUpdateWithDescription => {
            "Una vez que se cumple cierta condición, normalmente estar en el segmento o haberlo completado, el tiempo se actualiza con el valor indicado aquí."
        }
        Text::SplitsUpdateTrigger => "Disparador de actualización",
        Text::SplitsUpdateTriggerDescription => {
            "La condición que debe cumplirse para actualizar el valor con el especificado en «Actualizar con». Antes de eso, el valor es el de «Comenzar con»."
        }
        Text::SplitsColumnComparison => "Comparación",
        Text::SplitsColumnComparisonDescription => {
            "La comparación contra la que se compara esta columna. Si no se especifica, se usa la comparación actual."
        }
        Text::SplitsColumnTimingMethod => "Método de cronometraje",
        Text::SplitsColumnTimingMethodDescription => {
            "Indica el método de cronometraje para esta columna. Si no se especifica, se usa el método actual."
        }
        Text::TextComponentBackground => "Fondo",
        Text::TextComponentBackgroundDescription => "El fondo mostrado detrás del componente.",
        Text::TextComponentUseVariable => "Usar variable",
        Text::TextComponentUseVariableDescription => {
            "Indica si se debe usar una variable personalizada para mostrar un valor dinámico. Las variables personalizadas pueden definirse en el editor de splits y ser proporcionadas por auto splitters."
        }
        Text::TextComponentSplit => "Separar",
        Text::TextComponentSplitDescription => {
            "Indica si el texto debe dividirse en una parte izquierda y derecha. Si no, se muestra un texto centrado."
        }
        Text::TextComponentText => "Texto",
        Text::TextComponentTextDescription => "Indica el texto a mostrar en el centro.",
        Text::TextComponentLeft => "Izquierda",
        Text::TextComponentLeftDescription => "Indica el texto a mostrar a la izquierda.",
        Text::TextComponentRight => "Derecha",
        Text::TextComponentRightDescription => "Indica el texto a mostrar a la derecha.",
        Text::TextComponentVariable => "Variable",
        Text::TextComponentVariableDescription => {
            "Indica el nombre de la variable personalizada a mostrar."
        }
        Text::TextComponentTextColor => "Color del texto",
        Text::TextComponentTextColorDescription => "El color del texto.",
        Text::TextComponentLeftColor => "Color izquierdo",
        Text::TextComponentLeftColorDescription => "El color del texto de la izquierda.",
        Text::TextComponentRightColor => "Color derecho",
        Text::TextComponentRightColorDescription => "El color del texto de la derecha.",
        Text::TextComponentNameColor => "Color del nombre",
        Text::TextComponentNameColorDescription => "El color del nombre de la variable.",
        Text::TextComponentValueColor => "Color del valor",
        Text::TextComponentValueColorDescription => "El color del valor de la variable.",
        Text::TextComponentDisplayTwoRows => "Mostrar en 2 filas",
        Text::TextComponentDisplayTwoRowsDescription => {
            "Indica si los textos izquierdo y derecho se muestran en dos filas separadas."
        }
        Text::LayoutDirection => "Dirección del layout",
        Text::LayoutDirectionDescription => "La dirección en la que se disponen los componentes.",
        Text::CustomTimerFont => "Fuente personalizada del temporizador",
        Text::CustomTimerFontDescription => {
            "Permite especificar una fuente personalizada para el temporizador. Si no se configura, se usa la fuente predeterminada."
        }
        Text::CustomTimesFont => "Fuente personalizada de tiempos",
        Text::CustomTimesFontDescription => {
            "Permite especificar una fuente personalizada para los tiempos. Si no se configura, se usa la fuente predeterminada."
        }
        Text::CustomTextFont => "Fuente personalizada de texto",
        Text::CustomTextFontDescription => {
            "Permite especificar una fuente personalizada para el texto. Si no se configura, se usa la fuente predeterminada."
        }
        Text::TextShadow => "Sombra de texto",
        Text::TextShadowDescription => {
            "Permite especificar opcionalmente un color para las sombras de texto."
        }
        Text::Background => "Fondo",
        Text::BackgroundDescription => "El fondo mostrado detrás de todo el layout.",
        Text::BestSegment => "Mejor segmento",
        Text::BestSegmentDescription => "El color a usar cuando logras un nuevo mejor segmento.",
        Text::AheadGainingTime => "Por delante (ganando tiempo)",
        Text::AheadGainingTimeDescription => {
            "El color a usar cuando estás por delante y sigues ganando tiempo."
        }
        Text::AheadLosingTime => "Por delante (perdiendo tiempo)",
        Text::AheadLosingTimeDescription => {
            "El color a usar cuando estás por delante pero pierdes tiempo."
        }
        Text::BehindGainingTime => "Por detrás (recuperando tiempo)",
        Text::BehindGainingTimeDescription => {
            "El color a usar cuando estás por detrás pero recuperas tiempo."
        }
        Text::BehindLosingTime => "Por detrás (perdiendo tiempo)",
        Text::BehindLosingTimeDescription => {
            "El color a usar cuando estás por detrás y sigues perdiendo tiempo."
        }
        Text::NotRunning => "No en ejecución",
        Text::NotRunningDescription => "El color a usar cuando no hay un intento activo.",
        Text::PersonalBest => "Mejor marca personal",
        Text::PersonalBestDescription => {
            "El color a usar cuando consigues una nueva mejor marca personal."
        }
        Text::Paused => "Pausado",
        Text::PausedDescription => "El color a usar cuando el temporizador está en pausa.",
        Text::ThinSeparators => "Separadores finos",
        Text::ThinSeparatorsDescription => "El color de los separadores finos.",
        Text::Separators => "Separadores",
        Text::SeparatorsDescription => "El color de los separadores normales.",
        Text::TextColor => "Texto",
        Text::TextColorDescription => {
            "El color a usar para el texto que no especifica su propio color."
        }
        Text::ComponentBlankSpace => "Espacio en blanco",
        Text::ComponentCurrentComparison => "Comparación actual",
        Text::ComponentCurrentPace => "Ritmo actual",
        Text::ComponentDelta => "Diferencia",
        Text::ComponentDetailedTimer => "Temporizador detallado",
        Text::ComponentGraph => "Gráfico",
        Text::ComponentPbChance => "Probabilidad de PB",
        Text::ComponentPossibleTimeSave => "Ahorro de tiempo posible",
        Text::ComponentPreviousSegment => "Segmento anterior",
        Text::ComponentSegmentTime => "Tiempo de segmento",
        Text::ComponentSeparator => "Separador",
        Text::ComponentSplits => "Splits",
        Text::ComponentSumOfBest => "Suma de mejores",
        Text::ComponentText => "Texto",
        Text::ComponentTimer => "Temporizador",
        Text::ComponentSegmentTimer => "Temporizador de segmento",
        Text::ComponentTitle => "Título",
        Text::ComponentTotalPlaytime => "Tiempo total de juego",
        Text::ComponentCurrentPaceBestPossibleTime => "Mejor tiempo posible",
        Text::ComponentCurrentPaceWorstPossibleTime => "Peor tiempo posible",
        Text::ComponentCurrentPacePredictedTime => "Tiempo previsto",
        Text::ComponentSegmentTimeBest => "Mejor tiempo de segmento",
        Text::ComponentSegmentTimeWorst => "Peor tiempo de segmento",
        Text::ComponentSegmentTimeAverage => "Tiempo medio de segmento",
        Text::ComponentSegmentTimeMedian => "Tiempo mediano de segmento",
        Text::ComponentSegmentTimeLatest => "Último tiempo de segmento",
        Text::ComponentPossibleTimeSaveTotal => "Tiempo total posible a ahorrar",
        Text::LiveSegment => "Segmento en directo",
        Text::LiveSegmentShort => "Segmento en directo",
        Text::PreviousSegmentShort => "Seg. anterior",
        Text::PreviousSegmentAbbreviation => "Seg. ant.",
        Text::ComparingAgainst => "Comparando con",
        Text::ComparisonShort => "Comparación",
        Text::CurrentPaceBestPossibleTimeShort => "Mejor t. posible",
        Text::CurrentPaceBestTimeShort => "Mejor tiempo",
        Text::CurrentPaceBestPossibleTimeAbbreviation => "MTP",
        Text::CurrentPaceWorstPossibleTimeShort => "Peor t. posible",
        Text::CurrentPaceWorstTimeShort => "Peor tiempo",
        Text::CurrentPacePredictedTimeShort => "T. previsto",
        Text::CurrentPaceShort => "Ritmo act.",
        Text::CurrentPaceAbbreviation => "Ritmo",
        Text::Goal => "Meta",
        Text::SumOfBestSegments => "Suma de mejores segmentos",
        Text::SumOfBestShort => "Suma de mejores",
        Text::SumOfBestAbbreviation => "SdM",
        Text::PlaytimeShort => "Tiempo de juego",
        Text::BestSegmentTimeShort => "Mejor t. seg.",
        Text::BestSegmentShort => "Mejor segmento",
        Text::WorstSegmentTimeShort => "Peor t. seg.",
        Text::WorstSegmentShort => "Peor segmento",
        Text::AverageSegmentTimeShort => "T. seg. medio",
        Text::AverageSegmentShort => "Segmento medio",
        Text::MedianSegmentTimeShort => "T. seg. med.",
        Text::MedianSegmentShort => "Segmento mediano",
        Text::LatestSegmentTimeShort => "Últ. t. seg.",
        Text::LatestSegmentShort => "Último segmento",
        Text::SegmentTimeShort => "T. seg.",
        Text::PossibleTimeSaveShort => "Tiempo posible a ahorrar",
        Text::PossibleTimeSaveAbbreviation => "T. posible a ahorrar",
        Text::TimeSaveShort => "Tiempo a ahorrar",
        Text::RealTime => "Tiempo real",
        Text::GameTime => "Tiempo de juego",
        Text::SumOfBestCleanerStartOfRun => "el inicio de la run",
        Text::SumOfBestCleanerShouldRemove => {
            ". ¿Crees que este tiempo de segmento es inexacto y debería eliminarse?"
        }
    }
}

pub const fn resolve_placeholder(text: PlaceholderText) -> &'static [Piece] {
    match text {
        PlaceholderText::SumOfBestCleanerSegmentTimeBetween => &[
            Piece::Static("Tuviste un tiempo de segmento de "),
            Piece::Dynamic(0),
            Piece::Static(" de "),
            Piece::Dynamic(1),
            Piece::Static(" entre «"),
            Piece::Dynamic(2),
            Piece::Static("» y «"),
            Piece::Dynamic(3),
            Piece::Static("»"),
        ],
        PlaceholderText::SumOfBestCleanerFasterThanCombined => &[
            Piece::Static(", que es más rápido que la combinación de los mejores segmentos de "),
            Piece::Dynamic(0),
        ],
        PlaceholderText::SumOfBestCleanerRunOn => &[
            Piece::Static(" en una run el "),
            Piece::Dynamic(0),
            Piece::Static(" que comenzó a las "),
            Piece::Dynamic(1),
        ],
    }
}
