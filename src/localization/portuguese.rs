use super::{Piece, PlaceholderText, Text};

pub const fn resolve(text: Text) -> &'static str {
    match text {
        Text::StartSplit => "Iniciar / Split",
        Text::StartSplitDescription => {
            "O atalho de teclado para fazer split e iniciar uma nova tentativa."
        }
        Text::Reset => "Reiniciar",
        Text::ResetDescription => "O atalho de teclado para reiniciar a tentativa atual.",
        Text::UndoSplit => "Desfazer split",
        Text::UndoSplitDescription => "O atalho de teclado para desfazer o último split.",
        Text::SkipSplit => "Ignorar split",
        Text::SkipSplitDescription => "O atalho de teclado para ignorar o split atual.",
        Text::Pause => "Pausar",
        Text::PauseDescription => {
            "O atalho de teclado para pausar a tentativa atual. Também pode ser usado para iniciar uma nova tentativa."
        }
        Text::UndoAllPauses => "Desfazer todas as pausas",
        Text::UndoAllPausesDescription => {
            "O atalho de teclado para remover todos os tempos de pausa do tempo atual. Útil se você pausou por engano."
        }
        Text::PreviousComparison => "Comparação anterior",
        Text::PreviousComparisonDescription => {
            "O atalho de teclado para mudar para a comparação anterior."
        }
        Text::NextComparison => "Próxima comparação",
        Text::NextComparisonDescription => {
            "O atalho de teclado para mudar para a próxima comparação."
        }
        Text::ToggleTimingMethod => "Alternar método de cronometragem",
        Text::ToggleTimingMethodDescription => {
            "O atalho de teclado para alternar entre «Tempo real» e «Tempo de jogo»."
        }
        Text::TimerBackground => "Fundo",
        Text::TimerBackgroundDescription => {
            "O fundo mostrado atrás do componente. Também é possível aplicar a cor associada ao tempo à frente ou atrás como cor de fundo."
        }
        Text::SegmentTimer => "Cronômetro do segmento",
        Text::SegmentTimerDescription => {
            "Indica se deve mostrar o tempo desde o início do segmento atual em vez do início da tentativa."
        }
        Text::TimingMethod => "Método de cronometragem",
        Text::TimingMethodDescription => {
            "Indica o método de cronometragem a usar. Se não for especificado, usa-se o método atual."
        }
        Text::Height => "Altura",
        Text::HeightDescription => "A altura do cronômetro.",
        Text::TimerTextColor => "Cor do texto",
        Text::TimerTextColorDescription => {
            "A cor do tempo mostrado. Se não for especificada, a cor é escolhida automaticamente conforme o andamento da tentativa. Essas cores podem ser definidas nas configurações gerais do layout."
        }
        Text::ShowGradient => "Mostrar gradiente",
        Text::ShowGradientDescription => {
            "Determina se a cor do cronômetro é exibida como gradiente."
        }
        Text::DigitsFormat => "Formato de dígitos",
        Text::DigitsFormatDescription => {
            "Indica quantos dígitos mostrar. Se a duração for menor do que os dígitos a mostrar, zeros serão exibidos."
        }
        Text::Accuracy => "Precisão",
        Text::AccuracyDescription => "A precisão do tempo mostrado.",
        Text::TitleBackground => "Fundo",
        Text::TitleBackgroundDescription => "O fundo mostrado atrás do componente.",
        Text::TitleTextColor => "Cor do texto",
        Text::TitleTextColorDescription => {
            "A cor do texto do título. Se não houver cor especificada, usa-se a cor do layout."
        }
        Text::ShowGameName => "Mostrar nome do jogo",
        Text::ShowGameNameDescription => {
            "Indica se o nome do jogo deve fazer parte do título exibido."
        }
        Text::ShowCategoryName => "Mostrar nome da categoria",
        Text::ShowCategoryNameDescription => {
            "Indica se o nome da categoria deve fazer parte do título exibido."
        }
        Text::ShowFinishedRunsCount => "Mostrar quantidade de runs concluídos",
        Text::ShowFinishedRunsCountDescription => {
            "Indica se o número de runs concluídos com sucesso deve ser exibido."
        }
        Text::ShowAttemptCount => "Mostrar quantidade de tentativas",
        Text::ShowAttemptCountDescription => {
            "Indica se o número total de tentativas deve ser exibido."
        }
        Text::TextAlignment => "Alinhamento do texto",
        Text::TextAlignmentDescription => "Indica o alinhamento do título.",
        Text::DisplayTextAsSingleLine => "Exibir texto em uma única linha",
        Text::DisplayTextAsSingleLineDescription => {
            "Indica se o título deve ser mostrado em uma única linha, em vez de separar em uma linha para o jogo e outra para a categoria."
        }
        Text::DisplayGameIcon => "Mostrar ícone do jogo",
        Text::DisplayGameIconDescription => {
            "Indica se o ícone do jogo deve ser exibido, caso exista um ícone armazenado nos splits."
        }
        Text::ShowRegion => "Mostrar região",
        Text::ShowRegionDescription => {
            "O nome da categoria pode ser estendido com informações adicionais. Isso o estende com a região do jogo, se fornecida na aba de variáveis do editor de splits."
        }
        Text::ShowPlatform => "Mostrar plataforma",
        Text::ShowPlatformDescription => {
            "O nome da categoria pode ser estendido com informações adicionais. Isso o estende com a plataforma em que o jogo é jogado, se fornecida na aba de variáveis do editor de splits."
        }
        Text::ShowVariables => "Mostrar variáveis",
        Text::ShowVariablesDescription => {
            "O nome da categoria pode ser estendido com informações adicionais. Isso o estende com variáveis adicionais fornecidas na aba de variáveis do editor de splits. Refere-se às variáveis do speedrun.com, não às variáveis personalizadas."
        }
        Text::TotalPlaytimeBackground => "Fundo",
        Text::TotalPlaytimeBackgroundDescription => "O fundo mostrado atrás do componente.",
        Text::DisplayTwoRows => "Exibir 2 linhas",
        Text::DisplayTwoRowsDescription => {
            "Indica se deve exibir o nome do componente e o tempo total de jogo em duas linhas separadas."
        }
        Text::ShowDays => "Mostrar dias (>24h)",
        Text::ShowDaysDescription => {
            "Indica se deve mostrar o número de dias quando o tempo total de jogo atingir 24 horas ou mais."
        }
        Text::LabelColor => "Cor do rótulo",
        Text::LabelColorDescription => {
            "A cor do nome do componente. Se não for especificada, usa-se a cor do layout."
        }
        Text::ValueColor => "Cor do valor",
        Text::ValueColorDescription => {
            "A cor do tempo total de jogo. Se não for especificada, usa-se a cor do layout."
        }
        Text::BlankSpaceBackground => "Fundo",
        Text::BlankSpaceBackgroundDescription => "O fundo mostrado atrás do componente.",
        Text::BlankSpaceSize => "Tamanho",
        Text::BlankSpaceSizeDescription => "O tamanho do componente.",
        Text::CurrentComparisonBackground => "Fundo",
        Text::CurrentComparisonBackgroundDescription => "O fundo mostrado atrás do componente.",
        Text::CurrentComparisonDisplayTwoRows => "Exibir 2 linhas",
        Text::CurrentComparisonDisplayTwoRowsDescription => {
            "Indica se deve exibir o nome do componente e a comparação em duas linhas separadas."
        }
        Text::CurrentComparisonLabelColor => "Cor do rótulo",
        Text::CurrentComparisonLabelColorDescription => {
            "A cor do nome do componente. Se não for especificada, usa-se a cor do layout."
        }
        Text::CurrentComparisonValueColor => "Cor do valor",
        Text::CurrentComparisonValueColorDescription => {
            "A cor do nome da comparação. Se não for especificada, usa-se a cor do layout."
        }
        Text::CurrentPaceBackground => "Fundo",
        Text::CurrentPaceBackgroundDescription => "O fundo mostrado atrás do componente.",
        Text::CurrentPaceComparison => "Comparação",
        Text::CurrentPaceComparisonDescription => {
            "A comparação para prever o tempo final. Se não for especificada, usa-se a comparação atual."
        }
        Text::CurrentPaceDisplayTwoRows => "Exibir 2 linhas",
        Text::CurrentPaceDisplayTwoRowsDescription => {
            "Indica se deve exibir o nome do componente e o tempo previsto em duas linhas separadas."
        }
        Text::CurrentPaceLabelColor => "Cor do rótulo",
        Text::CurrentPaceLabelColorDescription => {
            "A cor do nome do componente. Se não for especificada, usa-se a cor do layout."
        }
        Text::CurrentPaceValueColor => "Cor do valor",
        Text::CurrentPaceValueColorDescription => {
            "A cor do tempo previsto. Se não for especificada, usa-se a cor do layout."
        }
        Text::CurrentPaceAccuracy => "Precisão",
        Text::CurrentPaceAccuracyDescription => "A precisão do tempo previsto exibido.",
        Text::DeltaBackground => "Fundo",
        Text::DeltaBackgroundDescription => "O fundo mostrado atrás do componente.",
        Text::DeltaComparison => "Comparação",
        Text::DeltaComparisonDescription => {
            "A comparação usada para calcular o quanto a tentativa está à frente ou atrás. Se não for especificada, usa-se a comparação atual."
        }
        Text::DeltaDisplayTwoRows => "Exibir 2 linhas",
        Text::DeltaDisplayTwoRowsDescription => {
            "Indica se deve exibir o nome da comparação e o delta em duas linhas separadas."
        }
        Text::DeltaLabelColor => "Cor do rótulo",
        Text::DeltaLabelColorDescription => {
            "A cor do nome da comparação. Se não for especificada, usa-se a cor do layout."
        }
        Text::DeltaDropDecimals => "Remover decimais",
        Text::DeltaDropDecimalsDescription => {
            "Indica se os decimais devem ser ocultados quando o delta exibido ultrapassa um minuto."
        }
        Text::DeltaAccuracy => "Precisão",
        Text::DeltaAccuracyDescription => "A precisão do delta exibido.",
        Text::SumOfBestBackground => "Fundo",
        Text::SumOfBestBackgroundDescription => "O fundo mostrado atrás do componente.",
        Text::SumOfBestDisplayTwoRows => "Exibir 2 linhas",
        Text::SumOfBestDisplayTwoRowsDescription => {
            "Indica se deve exibir o nome do componente e a soma dos melhores segmentos em duas linhas separadas."
        }
        Text::SumOfBestLabelColor => "Cor do rótulo",
        Text::SumOfBestLabelColorDescription => {
            "A cor do nome do componente. Se não for especificada, usa-se a cor do layout."
        }
        Text::SumOfBestValueColor => "Cor do valor",
        Text::SumOfBestValueColorDescription => {
            "A cor da soma dos melhores segmentos. Se não for especificada, usa-se a cor do layout."
        }
        Text::SumOfBestAccuracy => "Precisão",
        Text::SumOfBestAccuracyDescription => "A precisão da soma dos melhores segmentos exibida.",
        Text::PbChanceBackground => "Fundo",
        Text::PbChanceBackgroundDescription => "O fundo mostrado atrás do componente.",
        Text::PbChanceDisplayTwoRows => "Exibir 2 linhas",
        Text::PbChanceDisplayTwoRowsDescription => {
            "Indica se deve exibir o nome do componente e a chance de PB em duas linhas separadas."
        }
        Text::PbChanceLabelColor => "Cor do rótulo",
        Text::PbChanceLabelColorDescription => {
            "A cor do nome do componente. Se não for especificada, usa-se a cor do layout."
        }
        Text::PbChanceValueColor => "Cor do valor",
        Text::PbChanceValueColorDescription => {
            "A cor da chance de PB. Se não for especificada, usa-se a cor do layout."
        }
        Text::PossibleTimeSaveBackground => "Fundo",
        Text::PossibleTimeSaveBackgroundDescription => "O fundo mostrado atrás do componente.",
        Text::PossibleTimeSaveComparison => "Comparação",
        Text::PossibleTimeSaveComparisonDescription => {
            "A comparação para calcular o tempo potencialmente economizável. Se não for especificada, usa-se a comparação atual."
        }
        Text::PossibleTimeSaveDisplayTwoRows => "Exibir 2 linhas",
        Text::PossibleTimeSaveDisplayTwoRowsDescription => {
            "Indica se deve exibir o nome do componente e o tempo potencialmente economizável em duas linhas separadas."
        }
        Text::PossibleTimeSaveShowTotal => "Mostrar economia total possível de tempo",
        Text::PossibleTimeSaveShowTotalDescription => {
            "Indica se deve mostrar a economia total possível para o restante da tentativa, em vez da economia do segmento atual."
        }
        Text::PossibleTimeSaveLabelColor => "Cor do rótulo",
        Text::PossibleTimeSaveLabelColorDescription => {
            "A cor do nome do componente. Se não for especificada, usa-se a cor do layout."
        }
        Text::PossibleTimeSaveValueColor => "Cor do valor",
        Text::PossibleTimeSaveValueColorDescription => {
            "A cor do tempo potencialmente economizável. Se não for especificada, usa-se a cor do layout."
        }
        Text::PossibleTimeSaveAccuracy => "Precisão",
        Text::PossibleTimeSaveAccuracyDescription => {
            "A precisão do tempo potencialmente economizável exibido."
        }
        Text::PreviousSegmentBackground => "Fundo",
        Text::PreviousSegmentBackgroundDescription => "O fundo mostrado atrás do componente.",
        Text::PreviousSegmentComparison => "Comparação",
        Text::PreviousSegmentComparisonDescription => {
            "A comparação usada para calcular quanto tempo foi ganho ou perdido. Se não for especificada, usa-se a comparação atual."
        }
        Text::PreviousSegmentDisplayTwoRows => "Exibir 2 linhas",
        Text::PreviousSegmentDisplayTwoRowsDescription => {
            "Indica se deve exibir o nome do componente e o tempo ganho ou perdido em duas linhas separadas."
        }
        Text::PreviousSegmentLabelColor => "Cor do rótulo",
        Text::PreviousSegmentLabelColorDescription => {
            "A cor do nome do componente. Se não for especificada, usa-se a cor do layout."
        }
        Text::PreviousSegmentDropDecimals => "Remover decimais",
        Text::PreviousSegmentDropDecimalsDescription => {
            "Indica se os decimais devem ser removidos quando o tempo exibido ultrapassa um minuto."
        }
        Text::PreviousSegmentAccuracy => "Precisão",
        Text::PreviousSegmentAccuracyDescription => "A precisão do tempo exibido.",
        Text::PreviousSegmentShowPossibleTimeSave => "Mostrar economia de tempo possível",
        Text::PreviousSegmentShowPossibleTimeSaveDescription => {
            "Indica se deve mostrar, além do tempo ganho ou perdido, quanto tempo poderia ter sido economizado no segmento anterior."
        }
        Text::SegmentTimeBackground => "Fundo",
        Text::SegmentTimeBackgroundDescription => "O fundo mostrado atrás do componente.",
        Text::SegmentTimeComparison => "Comparação",
        Text::SegmentTimeComparisonDescription => {
            "A comparação para o tempo de segmento. Se não for especificada, usa-se a comparação atual."
        }
        Text::SegmentTimeDisplayTwoRows => "Exibir 2 linhas",
        Text::SegmentTimeDisplayTwoRowsDescription => {
            "Indica se deve exibir o nome do componente e o tempo de segmento em duas linhas separadas."
        }
        Text::SegmentTimeLabelColor => "Cor do rótulo",
        Text::SegmentTimeLabelColorDescription => {
            "A cor do nome do componente. Se não for especificada, usa-se a cor do layout."
        }
        Text::SegmentTimeValueColor => "Cor do valor",
        Text::SegmentTimeValueColorDescription => {
            "A cor do tempo de segmento. Se não for especificada, usa-se a cor do layout."
        }
        Text::SegmentTimeAccuracy => "Precisão",
        Text::SegmentTimeAccuracyDescription => "A precisão do tempo de segmento exibido.",
        Text::GraphComparison => "Comparação",
        Text::GraphComparisonDescription => {
            "A comparação a usar no gráfico. Se não for especificada, usa-se a comparação atual."
        }
        Text::GraphHeight => "Altura",
        Text::GraphHeightDescription => "A altura do gráfico.",
        Text::GraphShowBestSegments => "Mostrar melhores segmentos",
        Text::GraphShowBestSegmentsDescription => {
            "Indica se os melhores segmentos devem ser coloridos com a cor de melhor segmento do layout."
        }
        Text::GraphLiveGraph => "Gráfico ao vivo",
        Text::GraphLiveGraphDescription => {
            "Indica se o gráfico deve ser atualizado o tempo todo. Se desativado, as mudanças ocorrem apenas quando o segmento atual muda."
        }
        Text::GraphFlipGraph => "Inverter gráfico",
        Text::GraphFlipGraphDescription => {
            "Indica se o gráfico deve ser invertido verticalmente. Se não ativado, tempos à frente são exibidos abaixo do eixo x e tempos atrás acima."
        }
        Text::GraphBehindBackgroundColor => "Cor de fundo (atrás)",
        Text::GraphBehindBackgroundColorDescription => {
            "A cor de fundo para a região do gráfico contendo tempos atrás da comparação."
        }
        Text::GraphAheadBackgroundColor => "Cor de fundo (à frente)",
        Text::GraphAheadBackgroundColorDescription => {
            "A cor de fundo para a região do gráfico contendo tempos à frente da comparação."
        }
        Text::GraphGridLinesColor => "Cor das linhas de grade",
        Text::GraphGridLinesColorDescription => "A cor das linhas de grade do gráfico.",
        Text::GraphLinesColor => "Cor das linhas do gráfico",
        Text::GraphLinesColorDescription => "A cor das linhas que conectam os pontos do gráfico.",
        Text::GraphPartialFillColor => "Cor de preenchimento parcial",
        Text::GraphPartialFillColorDescription => {
            "A cor da região entre o eixo x e o gráfico. A cor parcial é usada apenas para mudanças ao vivo, mais especificamente do último split até o tempo atual."
        }
        Text::GraphCompleteFillColor => "Cor de preenchimento completo",
        Text::GraphCompleteFillColorDescription => {
            "A cor da região entre o eixo x e o gráfico, excluindo o segmento com mudanças ao vivo."
        }
        Text::DetailedTimerBackground => "Fundo",
        Text::DetailedTimerBackgroundDescription => "O fundo mostrado atrás do componente.",
        Text::DetailedTimerTimingMethod => "Método de cronometragem",
        Text::DetailedTimerTimingMethodDescription => {
            "Indica o método de cronometragem a usar. Se não for especificado, usa-se o método atual."
        }
        Text::DetailedTimerComparison1 => "Comparação 1",
        Text::DetailedTimerComparison1Description => {
            "A primeira comparação cujo tempo de segmento é exibido. Se não for especificada, usa-se a comparação atual."
        }
        Text::DetailedTimerComparison2 => "Comparação 2",
        Text::DetailedTimerComparison2Description => {
            "A segunda comparação cujo tempo de segmento é exibido. Se não for especificada, usa-se a comparação atual, a menos que a primeira comparação também seja None. Não é exibida se a segunda comparação estiver oculta."
        }
        Text::DetailedTimerHideSecondComparison => "Ocultar segunda comparação",
        Text::DetailedTimerHideSecondComparisonDescription => {
            "Indica se deve mostrar apenas uma comparação."
        }
        Text::DetailedTimerTimerHeight => "Altura do cronômetro",
        Text::DetailedTimerTimerHeightDescription => "A altura do cronômetro de corrida.",
        Text::DetailedTimerSegmentTimerHeight => "Altura do cronômetro de segmento",
        Text::DetailedTimerSegmentTimerHeightDescription => "A altura do cronômetro de segmento.",
        Text::DetailedTimerTimerColor => "Cor do cronômetro",
        Text::DetailedTimerTimerColorDescription => {
            "Em vez de determinar automaticamente a cor do cronômetro principal com base no andamento da tentativa, pode-se fornecer uma cor fixa."
        }
        Text::DetailedTimerShowTimerGradient => "Mostrar gradiente do cronômetro",
        Text::DetailedTimerShowTimerGradientDescription => {
            "O cronômetro principal transforma automaticamente sua cor em um gradiente vertical se esta opção estiver ativada. Caso contrário, usa-se a cor real."
        }
        Text::DetailedTimerTimerDigitsFormat => "Formato de dígitos do cronômetro",
        Text::DetailedTimerTimerDigitsFormatDescription => {
            "Indica quantos dígitos mostrar no cronômetro principal. Se a duração for menor, zeros são exibidos."
        }
        Text::DetailedTimerTimerAccuracy => "Precisão do cronômetro",
        Text::DetailedTimerTimerAccuracyDescription => {
            "A precisão do tempo mostrado no cronômetro principal."
        }
        Text::DetailedTimerSegmentTimerColor => "Cor do cronômetro de segmento",
        Text::DetailedTimerSegmentTimerColorDescription => {
            "Altera a cor do cronômetro de segmento para uma cor diferente da padrão."
        }
        Text::DetailedTimerShowSegmentTimerGradient => {
            "Mostrar gradiente do cronômetro de segmento"
        }
        Text::DetailedTimerShowSegmentTimerGradientDescription => {
            "O cronômetro de segmento transforma automaticamente sua cor em um gradiente vertical se esta opção estiver ativada. Caso contrário, usa-se a cor real."
        }
        Text::DetailedTimerSegmentTimerDigitsFormat => {
            "Formato de dígitos do cronômetro de segmento"
        }
        Text::DetailedTimerSegmentTimerDigitsFormatDescription => {
            "Indica quantos dígitos mostrar no cronômetro de segmento. Se a duração for menor, zeros são exibidos."
        }
        Text::DetailedTimerSegmentTimerAccuracy => "Precisão do cronômetro de segmento",
        Text::DetailedTimerSegmentTimerAccuracyDescription => {
            "A precisão do tempo mostrado no cronômetro de segmento."
        }
        Text::DetailedTimerComparisonNamesColor => "Cor dos nomes de comparação",
        Text::DetailedTimerComparisonNamesColorDescription => {
            "A cor dos nomes de comparação se forem exibidos. Se não for especificada, usa-se a cor do layout."
        }
        Text::DetailedTimerComparisonTimesColor => "Cor dos tempos de comparação",
        Text::DetailedTimerComparisonTimesColorDescription => {
            "A cor dos tempos de comparação se forem exibidos. Se não for especificada, usa-se a cor do layout."
        }
        Text::DetailedTimerComparisonTimesAccuracy => "Precisão dos tempos de comparação",
        Text::DetailedTimerComparisonTimesAccuracyDescription => {
            "A precisão dos tempos de comparação."
        }
        Text::DetailedTimerShowSegmentName => "Mostrar nome do segmento",
        Text::DetailedTimerShowSegmentNameDescription => {
            "Indica se o nome do segmento deve ser exibido."
        }
        Text::DetailedTimerSegmentNameColor => "Cor do nome do segmento",
        Text::DetailedTimerSegmentNameColorDescription => {
            "A cor do nome do segmento, se for exibido. Se não for especificada, usa-se a cor do layout."
        }
        Text::DetailedTimerDisplayIcon => "Mostrar ícone",
        Text::DetailedTimerDisplayIconDescription => {
            "Indica se o ícone do segmento deve ser exibido."
        }
        Text::SplitsBackground => "Fundo",
        Text::SplitsBackgroundDescription => {
            "O fundo mostrado atrás do componente. Você pode escolher cores alternadas; nesse caso, cada linha alterna entre as duas cores escolhidas."
        }
        Text::SplitsTotalRows => "Total de linhas",
        Text::SplitsTotalRowsDescription => {
            "O número total de linhas de segmentos a mostrar. Se definido como 0, todos os segmentos são mostrados. Se for menor que o total, apenas uma janela é mostrada e pode rolar."
        }
        Text::SplitsUpcomingSegments => "Segmentos futuros",
        Text::SplitsUpcomingSegmentsDescription => {
            "Se houver mais segmentos do que linhas exibidas, a janela rola automaticamente quando o segmento atual muda. Este número determina o mínimo de segmentos futuros a exibir."
        }
        Text::SplitsShowThinSeparators => "Mostrar separadores finos",
        Text::SplitsShowThinSeparatorsDescription => {
            "Indica se separadores finos devem ser mostrados entre as linhas de segmentos."
        }
        Text::SplitsShowSeparatorBeforeLastSplit => "Mostrar separador antes do último split",
        Text::SplitsShowSeparatorBeforeLastSplitDescription => {
            "Se o último segmento for sempre mostrado, isto determina se mostrar um separador mais pronunciado antes do último segmento, caso não seja adjacente ao segmento anterior na janela."
        }
        Text::SplitsAlwaysShowLastSplit => "Mostrar sempre o último split",
        Text::SplitsAlwaysShowLastSplitDescription => {
            "Se nem todos os segmentos são mostrados na janela, esta opção determina se o último segmento deve sempre ser mostrado, pois contém a duração total da comparação escolhida."
        }
        Text::SplitsFillWithBlankSpace => "Preencher com espaço em branco",
        Text::SplitsFillWithBlankSpaceDescription => {
            "Se não houver segmentos suficientes para preencher a lista, esta opção permite preencher as linhas restantes com espaço em branco para sempre mostrar o número total de linhas."
        }
        Text::SplitsShowTimesBelowSegmentName => "Mostrar tempos abaixo do nome do segmento",
        Text::SplitsShowTimesBelowSegmentNameDescription => {
            "Indica se os tempos devem ser mostrados abaixo do nome do segmento. Caso contrário, são mostrados ao lado."
        }
        Text::SplitsCurrentSegmentGradient => "Gradiente do segmento atual",
        Text::SplitsCurrentSegmentGradientDescription => {
            "O gradiente a mostrar atrás do segmento atual como indicador."
        }
        Text::SplitsSplitTimeAccuracy => "Precisão do tempo de split",
        Text::SplitsSplitTimeAccuracyDescription => {
            "Indica a precisão a usar para colunas que contêm tempos de split."
        }
        Text::SplitsSegmentTimeAccuracy => "Precisão do tempo de segmento",
        Text::SplitsSegmentTimeAccuracyDescription => {
            "Indica a precisão a usar para colunas que contêm tempos de segmento."
        }
        Text::SplitsDeltaTimeAccuracy => "Precisão do delta",
        Text::SplitsDeltaTimeAccuracyDescription => {
            "Indica a precisão a usar para colunas que contêm o tempo à frente ou atrás."
        }
        Text::SplitsDropDeltaDecimals => "Ocultar decimais do delta ao mostrar minutos",
        Text::SplitsDropDeltaDecimalsDescription => {
            "Indica se os decimais não devem mais ser mostrados quando uma coluna de delta for superior a um minuto."
        }
        Text::SplitsShowColumnLabels => "Mostrar rótulos das colunas",
        Text::SplitsShowColumnLabelsDescription => {
            "Indica se os nomes das colunas devem ser exibidos no topo da lista."
        }
        Text::SplitsColumns => "Colunas",
        Text::SplitsColumnsDescription => {
            "O número de colunas a mostrar em cada linha. Cada coluna pode ser configurada para mostrar informações diferentes. As colunas são definidas da direita para a esquerda."
        }
        Text::SplitsColumnName => "Nome da coluna",
        Text::SplitsColumnNameDescription => {
            "O nome da coluna. Ele é mostrado no topo da lista se a opção de mostrar rótulos de coluna estiver ativada."
        }
        Text::SplitsColumnType => "Tipo da coluna",
        Text::SplitsColumnTypeDescription => {
            "O tipo de informação que esta coluna exibe. Pode ser um tempo ou uma variável personalizada."
        }
        Text::SplitsVariableName => "Nome da variável",
        Text::SplitsVariableNameDescription => {
            "O nome da variável personalizada que esta coluna exibe."
        }
        Text::SplitsStartWith => "Começar com",
        Text::SplitsStartWithDescription => {
            "O valor com o qual esta coluna começa para cada segmento. O gatilho de atualização determina quando esse tempo é substituído."
        }
        Text::SplitsUpdateWith => "Atualizar com",
        Text::SplitsUpdateWithDescription => {
            "Quando uma determinada condição é atendida, geralmente estar no segmento ou tê-lo concluído, o tempo é atualizado com o valor especificado aqui."
        }
        Text::SplitsUpdateTrigger => "Gatilho de atualização",
        Text::SplitsUpdateTriggerDescription => {
            "A condição que precisa ser atendida para o tempo ser atualizado com o valor especificado em «Atualizar com». Antes disso, o tempo é o valor especificado em «Começar com»."
        }
        Text::SplitsColumnComparison => "Comparação",
        Text::SplitsColumnComparisonDescription => {
            "A comparação contra a qual esta coluna é comparada. Se não for especificada, usa-se a comparação atual."
        }
        Text::SplitsColumnTimingMethod => "Método de cronometragem",
        Text::SplitsColumnTimingMethodDescription => {
            "Indica o método de cronometragem a usar para esta coluna. Se não for especificado, usa-se o método atual."
        }
        Text::TextComponentBackground => "Fundo",
        Text::TextComponentBackgroundDescription => "O fundo mostrado atrás do componente.",
        Text::TextComponentUseVariable => "Usar variável",
        Text::TextComponentUseVariableDescription => {
            "Indica se usar uma variável personalizada para exibir um valor dinâmico. Variáveis personalizadas podem ser definidas no editor de splits e fornecidas automaticamente por auto splitters."
        }
        Text::TextComponentSplit => "Separar",
        Text::TextComponentSplitDescription => {
            "Indica se o texto deve ser dividido em uma parte esquerda e direita. Caso contrário, apenas um texto centralizado é exibido."
        }
        Text::TextComponentText => "Texto",
        Text::TextComponentTextDescription => "Indica o texto a exibir no centro.",
        Text::TextComponentLeft => "Esquerda",
        Text::TextComponentLeftDescription => "Indica o texto a exibir à esquerda.",
        Text::TextComponentRight => "Direita",
        Text::TextComponentRightDescription => "Indica o texto a exibir à direita.",
        Text::TextComponentVariable => "Variável",
        Text::TextComponentVariableDescription => {
            "Indica o nome da variável personalizada a exibir."
        }
        Text::TextComponentTextColor => "Cor do texto",
        Text::TextComponentTextColorDescription => "A cor do texto.",
        Text::TextComponentLeftColor => "Cor da esquerda",
        Text::TextComponentLeftColorDescription => "A cor do texto à esquerda.",
        Text::TextComponentRightColor => "Cor da direita",
        Text::TextComponentRightColorDescription => "A cor do texto à direita.",
        Text::TextComponentNameColor => "Cor do nome",
        Text::TextComponentNameColorDescription => "A cor do nome da variável.",
        Text::TextComponentValueColor => "Cor do valor",
        Text::TextComponentValueColorDescription => "A cor do valor da variável.",
        Text::TextComponentDisplayTwoRows => "Exibir 2 linhas",
        Text::TextComponentDisplayTwoRowsDescription => {
            "Indica se os textos da esquerda e direita devem ser exibidos em duas linhas separadas."
        }
        Text::LayoutDirection => "Direção do layout",
        Text::LayoutDirectionDescription => "A direção em que os componentes são dispostos.",
        Text::CustomTimerFont => "Fonte personalizada do cronômetro",
        Text::CustomTimerFontDescription => {
            "Permite especificar uma fonte personalizada para o cronômetro. Se não for definida, usa-se a fonte padrão."
        }
        Text::CustomTimesFont => "Fonte personalizada dos tempos",
        Text::CustomTimesFontDescription => {
            "Permite especificar uma fonte personalizada para os tempos. Se não for definida, usa-se a fonte padrão."
        }
        Text::CustomTextFont => "Fonte personalizada do texto",
        Text::CustomTextFontDescription => {
            "Permite especificar uma fonte personalizada para o texto. Se não for definida, usa-se a fonte padrão."
        }
        Text::TextShadow => "Sombra do texto",
        Text::TextShadowDescription => {
            "Permite especificar opcionalmente uma cor para sombras de texto."
        }
        Text::Background => "Fundo",
        Text::BackgroundDescription => "O fundo mostrado atrás de todo o layout.",
        Text::BestSegment => "Melhor segmento",
        Text::BestSegmentDescription => "A cor a usar quando você alcança um novo melhor segmento.",
        Text::AheadGainingTime => "À frente (ganhando tempo)",
        Text::AheadGainingTimeDescription => {
            "A cor a usar quando você está à frente e ganhando ainda mais tempo."
        }
        Text::AheadLosingTime => "À frente (perdendo tempo)",
        Text::AheadLosingTimeDescription => {
            "A cor a usar quando você está à frente, mas perdendo tempo."
        }
        Text::BehindGainingTime => "Atrás (recuperando tempo)",
        Text::BehindGainingTimeDescription => {
            "A cor a usar quando você está atrás, mas recuperando tempo."
        }
        Text::BehindLosingTime => "Atrás (perdendo tempo)",
        Text::BehindLosingTimeDescription => {
            "A cor a usar quando você está atrás e perdendo ainda mais tempo."
        }
        Text::NotRunning => "Não em execução",
        Text::NotRunningDescription => "A cor a usar quando não há uma tentativa ativa.",
        Text::PersonalBest => "Melhor pessoal",
        Text::PersonalBestDescription => {
            "A cor a usar quando você consegue um novo recorde pessoal."
        }
        Text::Paused => "Pausado",
        Text::PausedDescription => "A cor a usar quando o cronômetro está pausado.",
        Text::ThinSeparators => "Separadores finos",
        Text::ThinSeparatorsDescription => "A cor dos separadores finos.",
        Text::Separators => "Separadores",
        Text::SeparatorsDescription => "A cor dos separadores normais.",
        Text::TextColor => "Texto",
        Text::TextColorDescription => "A cor a usar para texto que não especifica sua própria cor.",
        Text::ComponentBlankSpace => "Espaço em branco",
        Text::ComponentCurrentComparison => "Comparação atual",
        Text::ComponentCurrentPace => "Ritmo atual",
        Text::ComponentDelta => "Diferença",
        Text::ComponentDetailedTimer => "Temporizador detalhado",
        Text::ComponentGraph => "Gráfico",
        Text::ComponentPbChance => "Probabilidade de PB",
        Text::ComponentPossibleTimeSave => "Poupança de tempo possível",
        Text::ComponentPreviousSegment => "Segmento anterior",
        Text::ComponentSegmentTime => "Tempo do segmento",
        Text::ComponentSeparator => "Separador",
        Text::ComponentSplits => "Splits",
        Text::ComponentSumOfBest => "Soma dos melhores",
        Text::ComponentText => "Texto",
        Text::ComponentTimer => "Temporizador",
        Text::ComponentSegmentTimer => "Cronômetro do segmento",
        Text::ComponentTitle => "Título",
        Text::ComponentTotalPlaytime => "Tempo total de jogo",
        Text::ComponentCurrentPaceBestPossibleTime => "Melhor tempo possível",
        Text::ComponentCurrentPaceWorstPossibleTime => "Pior tempo possível",
        Text::ComponentCurrentPacePredictedTime => "Tempo previsto",
        Text::ComponentSegmentTimeBest => "Melhor tempo de segmento",
        Text::ComponentSegmentTimeWorst => "Pior tempo de segmento",
        Text::ComponentSegmentTimeAverage => "Tempo médio de segmento",
        Text::ComponentSegmentTimeMedian => "Tempo mediano de segmento",
        Text::ComponentSegmentTimeLatest => "Último tempo de segmento",
        Text::ComponentPossibleTimeSaveTotal => "Economia total de tempo possível",
        Text::LiveSegment => "Segmento ao vivo",
        Text::LiveSegmentShort => "Segmento ao vivo",
        Text::PreviousSegmentShort => "Seg. anterior",
        Text::PreviousSegmentAbbreviation => "Seg. ant.",
        Text::ComparingAgainst => "Comparando com",
        Text::ComparisonShort => "Comparação",
        Text::CurrentPaceBestPossibleTimeShort => "Melhor tempo poss.",
        Text::CurrentPaceBestTimeShort => "Melhor tempo",
        Text::CurrentPaceBestPossibleTimeAbbreviation => "MTP",
        Text::CurrentPaceWorstPossibleTimeShort => "Pior tempo poss.",
        Text::CurrentPaceWorstTimeShort => "Pior tempo",
        Text::CurrentPacePredictedTimeShort => "Tempo prev.",
        Text::CurrentPaceShort => "Ritmo at.",
        Text::CurrentPaceAbbreviation => "Ritmo",
        Text::Goal => "Meta",
        Text::SumOfBestSegments => "Soma dos melhores segmentos",
        Text::SumOfBestShort => "Soma dos melhores",
        Text::SumOfBestAbbreviation => "SdM",
        Text::PlaytimeShort => "Tempo de jogo",
        Text::BestSegmentTimeShort => "Melhor t. seg.",
        Text::BestSegmentShort => "Melhor segmento",
        Text::WorstSegmentTimeShort => "Pior t. seg.",
        Text::WorstSegmentShort => "Pior segmento",
        Text::AverageSegmentTimeShort => "T. seg. médio",
        Text::AverageSegmentShort => "Segmento médio",
        Text::MedianSegmentTimeShort => "T. seg. med.",
        Text::MedianSegmentShort => "Segmento mediano",
        Text::LatestSegmentTimeShort => "Últ. t. seg.",
        Text::LatestSegmentShort => "Último segmento",
        Text::SegmentTimeShort => "T. seg.",
        Text::SplitTime => "Tempo",
        Text::PossibleTimeSaveShort => "Tempo possível a poupar",
        Text::PossibleTimeSaveAbbreviation => "T. poss. a poupar",
        Text::TimeSaveShort => "Tempo a poupar",
        Text::RealTime => "Tempo real",
        Text::GameTime => "Tempo de jogo",
        Text::Untitled => "Sem título",
        Text::SumOfBestCleanerStartOfRun => "o início da corrida",
        Text::SumOfBestCleanerShouldRemove => {
            ". Achas que este tempo de segmento é impreciso e deve ser removido?"
        }
    }
}

pub const fn resolve_placeholder(text: PlaceholderText) -> &'static [Piece] {
    match text {
        PlaceholderText::SumOfBestCleanerSegmentTimeBetween => &[
            Piece::Static("Tiveste um tempo de segmento de "),
            Piece::Dynamic(0),
            Piece::Static(" de "),
            Piece::Dynamic(1),
            Piece::Static(" entre «"),
            Piece::Dynamic(2),
            Piece::Static("» e «"),
            Piece::Dynamic(3),
            Piece::Static("»"),
        ],
        PlaceholderText::SumOfBestCleanerFasterThanCombined => &[
            Piece::Static(", que é mais rápido do que a combinação dos melhores segmentos de "),
            Piece::Dynamic(0),
        ],
        PlaceholderText::SumOfBestCleanerRunOn => &[
            Piece::Static(" numa corrida em "),
            Piece::Dynamic(0),
            Piece::Static(" que começou às "),
            Piece::Dynamic(1),
        ],
    }
}
