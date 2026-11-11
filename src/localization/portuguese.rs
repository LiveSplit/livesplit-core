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
            display::Text::InternationalBackslash => "Barra invertida internacional",
            display::Text::Backspace => "Retrocesso",
            display::Text::CapsLock => "Caps Lock",
            display::Text::ContextMenu => "Menu de contexto",
            display::Text::Enter => "Enter",
            display::Text::Space => "Espaço",
            display::Text::Tab => "Tabulação",
            display::Text::Convert => "Converter",
            display::Text::KanaMode => "Modo kana",
            display::Text::Lang1 => "Hangul / Kana",
            display::Text::Lang2 => "Hanja / Alfanumérico",
            display::Text::Lang3 => "Katakana",
            display::Text::Lang4 => "Hiragana",
            display::Text::Lang5 => "Meia largura / Largura total",
            display::Text::NonConvert => "Não converter",
            display::Text::Delete => "Apagar",
            display::Text::End => "Fim",
            display::Text::Help => "Ajuda",
            display::Text::Home => "Início",
            display::Text::Insert => "Inserir",
            display::Text::PageDown => "Página abaixo",
            display::Text::PageUp => "Página acima",
            display::Text::NumLock => "Num Lock",
            display::Text::Clear => "Limpar",
            display::Text::ClearEntry => "Limpar entrada",
            display::Text::Decimal => "Decimal",
            display::Text::Escape => "Esc",
            display::Text::FnLock => "Bloqueio Fn",
            display::Text::PrintScreen => "Print Screen",
            display::Text::ScrollLock => "Scroll Lock",
            display::Text::Pause => "Pausa / Interromper",
            display::Text::BrowserBack => "Recuar no navegador",
            display::Text::BrowserFavorites => "Favoritos do navegador",
            display::Text::BrowserForward => "Avançar no navegador",
            display::Text::BrowserHome => "Página inicial do navegador",
            display::Text::BrowserRefresh => "Atualizar navegador",
            display::Text::BrowserSearch => "Pesquisar no navegador",
            display::Text::BrowserStop => "Parar navegador",
            display::Text::Eject => "Ejetar",
            display::Text::LaunchMail => "Abrir correio",
            display::Text::MediaPlayPause => "Reproduzir / Pausa",
            display::Text::MediaSelect => "Selecionar média",
            display::Text::MediaStop => "Parar reprodução",
            display::Text::MediaTrackNext => "Faixa seguinte",
            display::Text::MediaTrackPrevious => "Faixa anterior",
            display::Text::Power => "Ligar/desligar",
            display::Text::Sleep => "Suspender",
            display::Text::AudioVolumeDown => "Diminuir volume",
            display::Text::AudioVolumeMute => "Silenciar",
            display::Text::AudioVolumeUp => "Aumentar volume",
            display::Text::WakeUp => "Reativar",
            display::Text::Again => "Repetir",
            display::Text::Copy => "Copiar",
            display::Text::Cut => "Cortar",
            display::Text::Find => "Localizar",
            display::Text::Open => "Abrir",
            display::Text::Paste => "Colar",
            display::Text::Properties => "Propriedades",
            display::Text::Select => "Selecionar",
            display::Text::Undo => "Anular",
            display::Text::BrightnessDown => "Diminuir brilho",
            display::Text::BrightnessUp => "Aumentar brilho",
            display::Text::DisplayToggle => "Alternar ecrã interno / externo",
            display::Text::KeyboardLayout => "Selecionar disposição do teclado",
            display::Text::LaunchAssistant => "Abrir assistente",
            display::Text::LaunchControlPanel => "Abrir painel de controlo",
            display::Text::LaunchScreenSaver => "Iniciar proteção de ecrã",
            display::Text::MailForward => "Reencaminhar correio",
            display::Text::MailReply => "Responder ao correio",
            display::Text::MailSend => "Enviar correio",
            display::Text::MediaFastForward => "Avanço rápido",
            display::Text::MediaPause => "Pausar reprodução",
            display::Text::MediaPlay => "Reproduzir média",
            display::Text::MediaRecord => "Gravar média",
            display::Text::MediaRewind => "Rebobinar",
            display::Text::MicrophoneMute => "Silenciar microfone",
            display::Text::PrivacyScreen => "Alternar ecrã de privacidade",
            display::Text::SelectTask => "Selecionar tarefa",
            display::Text::ShowAllWindows => "Mostrar todas as janelas",
            display::Text::ZoomToggle => "Alternar zoom",
        },
        #[cfg(feature = "std")]
        Text::StartSplit => "Iniciar / Fazer split",
        #[cfg(feature = "std")]
        Text::StartSplitDescription => {
            "A tecla de atalho para fazer split e iniciar uma nova tentativa."
        }
        #[cfg(feature = "std")]
        Text::Reset => "Reiniciar",
        #[cfg(feature = "std")]
        Text::ResetDescription => "A tecla de atalho para reiniciar a tentativa atual.",
        #[cfg(feature = "std")]
        Text::UndoSplit => "Desfazer split",
        #[cfg(feature = "std")]
        Text::UndoSplitDescription => "A tecla de atalho para desfazer o último split.",
        #[cfg(feature = "std")]
        Text::SkipSplit => "Ignorar split",
        #[cfg(feature = "std")]
        Text::SkipSplitDescription => "A tecla de atalho para ignorar o split atual.",
        #[cfg(feature = "std")]
        Text::Pause => "Pausar",
        #[cfg(feature = "std")]
        Text::PauseDescription => {
            "A tecla de atalho para pausar a tentativa atual. Também pode ser usado para iniciar uma nova tentativa."
        }
        #[cfg(feature = "std")]
        Text::UndoAllPauses => "Desfazer todas as pausas",
        #[cfg(feature = "std")]
        Text::UndoAllPausesDescription => {
            "A tecla de atalho para remover todos os tempos de pausa do tempo atual. É útil se colocou o temporizador em pausa por engano e pretende anular a pausa."
        }
        #[cfg(feature = "std")]
        Text::PreviousComparison => "Comparação anterior",
        #[cfg(feature = "std")]
        Text::PreviousComparisonDescription => {
            "A tecla de atalho para mudar para a comparação anterior."
        }
        #[cfg(feature = "std")]
        Text::NextComparison => "Próxima comparação",
        #[cfg(feature = "std")]
        Text::NextComparisonDescription => {
            "A tecla de atalho para mudar para a próxima comparação."
        }
        #[cfg(feature = "std")]
        Text::ToggleTimingMethod => "Alternar método de cronometragem",
        #[cfg(feature = "std")]
        Text::ToggleTimingMethodDescription => {
            "A tecla de atalho para alternar entre «Tempo real» e «Tempo de jogo»."
        }
        Text::TimerBackground => "Fundo",
        Text::TimerBackgroundDescription => {
            "O fundo mostrado atrás do componente. Também é possível aplicar a cor associada ao tempo à frente ou atrás como cor de fundo."
        }
        Text::SegmentTimer => "Temporizador do segmento",
        Text::SegmentTimerDescription => {
            "Indica se deve mostrar o tempo desde o início do segmento atual em vez do início da tentativa."
        }
        Text::TimingMethod => "Método de cronometragem",
        Text::TimingMethodDescription => {
            "Indica o método de cronometragem a usar. Se não for especificado, usa-se o método atual."
        }
        Text::Height => "Altura",
        Text::HeightDescription => "A altura do temporizador.",
        Text::TimerTextColor => "Cor do texto",
        Text::TimerTextColorDescription => {
            "A cor do tempo mostrado. Se não for especificada, a cor é escolhida automaticamente conforme o andamento da tentativa. Essas cores podem ser definidas nas definições gerais do layout."
        }
        Text::ShowGradient => "Mostrar gradiente",
        Text::ShowGradientDescription => {
            "Determina se a cor do temporizador é apresentada como gradiente."
        }
        Text::DigitsFormat => "Formato de dígitos",
        Text::DigitsFormatDescription => {
            "Indica quantos dígitos mostrar. Se a duração for menor do que os dígitos a mostrar, zeros serão apresentados."
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
            "Indica se o nome do jogo deve fazer parte do título apresentado."
        }
        Text::ShowCategoryName => "Mostrar nome da categoria",
        Text::ShowCategoryNameDescription => {
            "Indica se o nome da categoria deve fazer parte do título apresentado."
        }
        Text::ShowFinishedRunsCount => "Mostrar quantidade de runs concluídos",
        Text::ShowFinishedRunsCountDescription => {
            "Indica se o número de runs concluídos com sucesso deve ser apresentado."
        }
        Text::ShowAttemptCount => "Mostrar quantidade de tentativas",
        Text::ShowAttemptCountDescription => {
            "Indica se o número total de tentativas deve ser apresentado."
        }
        Text::TextAlignment => "Alinhamento do texto",
        Text::TextAlignmentDescription => "Indica o alinhamento do título.",
        Text::DisplayTextAsSingleLine => "Exibir texto em uma única linha",
        Text::DisplayTextAsSingleLineDescription => {
            "Indica se o título deve ser mostrado em uma única linha, em vez de separar em uma linha para o jogo e outra para a categoria."
        }
        Text::DisplayGameIcon => "Mostrar ícone do jogo",
        Text::DisplayGameIconDescription => {
            "Indica se o ícone do jogo deve ser apresentado, caso exista um ícone armazenado nos splits."
        }
        Text::ShowRegion => "Mostrar região",
        Text::ShowRegionDescription => {
            "O nome da categoria pode ser estendido com informações adicionais. Isso o estende com a região do jogo, se fornecida no separador de variáveis do editor de splits."
        }
        Text::ShowPlatform => "Mostrar plataforma",
        Text::ShowPlatformDescription => {
            "O nome da categoria pode ser estendido com informações adicionais. Isso o estende com a plataforma em que o jogo é jogado, se fornecida no separador de variáveis do editor de splits."
        }
        Text::ShowVariables => "Mostrar variáveis",
        Text::ShowVariablesDescription => {
            "O nome da categoria pode ser estendido com informações adicionais. Isso o estende com variáveis adicionais fornecidas no separador de variáveis do editor de splits. Refere-se às variáveis do Speedrun.com, não às variáveis personalizadas."
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
        Text::CurrentPaceAccuracyDescription => "A precisão do tempo previsto apresentado.",
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
            "Indica se os decimais devem ser ocultados quando o delta apresentado ultrapassa um minuto."
        }
        Text::DeltaAccuracy => "Precisão",
        Text::DeltaAccuracyDescription => "A precisão do delta apresentado.",
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
        Text::SumOfBestAccuracyDescription => {
            "A precisão da soma dos melhores segmentos apresentada."
        }
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
            "A precisão do tempo potencialmente economizável apresentado."
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
            "Indica se os decimais devem ser removidos quando o tempo apresentado ultrapassa um minuto."
        }
        Text::PreviousSegmentAccuracy => "Precisão",
        Text::PreviousSegmentAccuracyDescription => "A precisão do tempo apresentado.",
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
        Text::SegmentTimeAccuracyDescription => "A precisão do tempo de segmento apresentado.",
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
            "Indica se o gráfico deve ser invertido verticalmente. Se a opção estiver desativada, os tempos de split à frente da comparação são apresentados abaixo do eixo x e os tempos em atraso acima. Ativar esta opção inverte essas posições."
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
            "A primeira comparação cujo tempo de segmento é apresentado. Se não for especificada, usa-se a comparação atual."
        }
        Text::DetailedTimerComparison2 => "Comparação 2",
        Text::DetailedTimerComparison2Description => {
            "A segunda comparação cujo tempo de segmento é apresentado. Se não for especificada, usa-se a comparação atual, a menos que a primeira comparação também seja None. Não é apresentada se a segunda comparação estiver oculta."
        }
        Text::DetailedTimerHideSecondComparison => "Ocultar segunda comparação",
        Text::DetailedTimerHideSecondComparisonDescription => {
            "Indica se deve mostrar apenas uma comparação."
        }
        Text::DetailedTimerTimerHeight => "Altura do temporizador",
        Text::DetailedTimerTimerHeightDescription => "A altura do temporizador de corrida.",
        Text::DetailedTimerSegmentTimerHeight => "Altura do temporizador de segmento",
        Text::DetailedTimerSegmentTimerHeightDescription => "A altura do temporizador de segmento.",
        Text::DetailedTimerTimerColor => "Cor do temporizador",
        Text::DetailedTimerTimerColorDescription => {
            "Em vez de determinar automaticamente a cor do temporizador principal com base no andamento da tentativa, pode-se fornecer uma cor fixa."
        }
        Text::DetailedTimerShowTimerGradient => "Mostrar gradiente do temporizador",
        Text::DetailedTimerShowTimerGradientDescription => {
            "O temporizador principal transforma automaticamente sua cor em um gradiente vertical se esta opção estiver ativada. Caso contrário, usa-se a cor real."
        }
        Text::DetailedTimerTimerDigitsFormat => "Formato de dígitos do temporizador",
        Text::DetailedTimerTimerDigitsFormatDescription => {
            "Indica quantos dígitos mostrar no temporizador principal. Se a duração for menor, zeros são apresentados."
        }
        Text::DetailedTimerTimerAccuracy => "Precisão do temporizador",
        Text::DetailedTimerTimerAccuracyDescription => {
            "A precisão do tempo mostrado no temporizador principal."
        }
        Text::DetailedTimerSegmentTimerColor => "Cor do temporizador de segmento",
        Text::DetailedTimerSegmentTimerColorDescription => {
            "Altera a cor do temporizador de segmento para uma cor diferente da predefinida."
        }
        Text::DetailedTimerShowSegmentTimerGradient => {
            "Mostrar gradiente do temporizador de segmento"
        }
        Text::DetailedTimerShowSegmentTimerGradientDescription => {
            "O temporizador de segmento transforma automaticamente sua cor em um gradiente vertical se esta opção estiver ativada. Caso contrário, usa-se a cor real."
        }
        Text::DetailedTimerSegmentTimerDigitsFormat => {
            "Formato de dígitos do temporizador de segmento"
        }
        Text::DetailedTimerSegmentTimerDigitsFormatDescription => {
            "Indica quantos dígitos mostrar no temporizador de segmento. Se a duração for menor, zeros são apresentados."
        }
        Text::DetailedTimerSegmentTimerAccuracy => "Precisão do temporizador de segmento",
        Text::DetailedTimerSegmentTimerAccuracyDescription => {
            "A precisão do tempo mostrado no temporizador de segmento."
        }
        Text::DetailedTimerComparisonNamesColor => "Cor dos nomes de comparação",
        Text::DetailedTimerComparisonNamesColorDescription => {
            "A cor dos nomes de comparação se forem apresentados. Se não for especificada, usa-se a cor do layout."
        }
        Text::DetailedTimerComparisonTimesColor => "Cor dos tempos de comparação",
        Text::DetailedTimerComparisonTimesColorDescription => {
            "A cor dos tempos de comparação se forem apresentados. Se não for especificada, usa-se a cor do layout."
        }
        Text::DetailedTimerComparisonTimesAccuracy => "Precisão dos tempos de comparação",
        Text::DetailedTimerComparisonTimesAccuracyDescription => {
            "A precisão dos tempos de comparação."
        }
        Text::DetailedTimerShowSegmentName => "Mostrar nome do segmento",
        Text::DetailedTimerShowSegmentNameDescription => {
            "Indica se o nome do segmento deve ser apresentado."
        }
        Text::DetailedTimerSegmentNameColor => "Cor do nome do segmento",
        Text::DetailedTimerSegmentNameColorDescription => {
            "A cor do nome do segmento, se for apresentado. Se não for especificada, usa-se a cor do layout."
        }
        Text::DetailedTimerDisplayIcon => "Mostrar ícone",
        Text::DetailedTimerDisplayIconDescription => {
            "Controla se o ícone do segmento fica oculto, ocupa ambas as linhas ou apenas a primeira."
        }
        Text::SplitsBackground => "Fundo",
        Text::SplitsBackgroundDescription => {
            "O fundo mostrado atrás do componente. Pode escolher cores alternadas; nesse caso, cada linha alterna entre as duas cores escolhidas."
        }
        Text::SplitsTotalRows => "Total de linhas",
        Text::SplitsTotalRowsDescription => {
            "O número total de linhas de segmentos a apresentar na lista. Se definido como 0, são apresentados todos os segmentos. Se for inferior ao número total de segmentos, apenas parte da lista é apresentada numa janela que permite deslocar o conteúdo para cima ou para baixo."
        }
        Text::SplitsUpcomingSegments => "Segmentos futuros",
        Text::SplitsUpcomingSegmentsDescription => {
            "Se houver mais segmentos do que linhas apresentadas, a janela desloca-se automaticamente para cima ou para baixo quando o segmento atual muda. Este número determina o mínimo de segmentos seguintes a apresentar nessa janela."
        }
        Text::SplitsShowThinSeparators => "Mostrar separadores finos",
        Text::SplitsShowThinSeparatorsDescription => {
            "Indica se separadores finos devem ser mostrados entre as linhas de segmentos."
        }
        Text::SplitsShowGapSeparators => "Mostrar separadores de lacunas",
        Text::SplitsShowGapSeparatorsDescription => {
            "Determina se deve ser mostrado um separador mais pronunciado antes de uma linha quando uma ou mais linhas imediatamente anteriores são omitidas da janela de deslocamento."
        }
        Text::SplitsAlwaysShowLastSplit => "Mostrar sempre o último split",
        Text::SplitsAlwaysShowLastSplitDescription => {
            "Se nem todos os segmentos forem apresentados na janela de deslocamento, esta opção determina se o último segmento deve ser sempre apresentado, pois contém a duração total da comparação escolhida. Esta informação pode ser útil, pois corresponde frequentemente ao recorde pessoal do jogador."
        }
        Text::SplitsFillWithBlankSpace => "Preencher com espaço em branco",
        Text::SplitsFillWithBlankSpaceDescription => {
            "Se não houver segmentos suficientes para preencher a lista, esta opção permite preencher as linhas restantes com espaço em branco para apresentar sempre o número total de linhas indicado nas definições. Caso contrário, o número de linhas apresentadas é reduzido ao número real de segmentos."
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
            "Indica se os nomes das colunas devem ser apresentados no topo da lista."
        }
        Text::SplitsSubsplitDisplayMode => "Modo de exibição dos subsplits",
        Text::SplitsSubsplitDisplayModeDescription => {
            "Controla como os subsplits são apresentados."
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
            "Indica se o texto deve ser dividido em uma parte esquerda e direita. Caso contrário, apenas um texto centralizado é apresentado."
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
            "Indica se os textos da esquerda e direita devem ser apresentados em duas linhas separadas."
        }
        Text::LayoutDirection => "Direção do layout",
        Text::LayoutDirectionDescription => "A direção em que os componentes são dispostos.",
        Text::GroupFixedWidth => "Largura fixa",
        Text::GroupFixedWidthDescription => {
            "Uma largura fixa opcional para esta coluna de componentes. Se não for especificada, a largura é determinada automaticamente."
        }
        Text::GroupFixedHeight => "Altura fixa",
        Text::GroupFixedHeightDescription => {
            "Uma altura fixa opcional para esta linha de componentes. Se não for especificada, a altura é determinada automaticamente."
        }
        Text::CustomTimerFont => "Fonte personalizada do temporizador",
        Text::CustomTimerFontDescription => {
            "Permite especificar uma fonte personalizada para o temporizador. Se não for definida, usa-se a fonte predefinida."
        }
        Text::CustomTimesFont => "Fonte personalizada dos tempos",
        Text::CustomTimesFontDescription => {
            "Permite especificar uma fonte personalizada para os tempos. Se não for definida, usa-se a fonte predefinida."
        }
        Text::CustomTextFont => "Fonte personalizada do texto",
        Text::CustomTextFontDescription => {
            "Permite especificar uma fonte personalizada para o texto. Se não for definida, usa-se a fonte predefinida."
        }
        Text::TextShadow => "Sombra do texto",
        Text::TextShadowDescription => {
            "Permite especificar opcionalmente uma cor para sombras de texto."
        }
        Text::Background => "Fundo",
        Text::BackgroundDescription => "O fundo mostrado atrás de todo o layout.",
        Text::BestSegment => "Melhor segmento",
        Text::BestSegmentDescription => "A cor a usar quando alcança um novo melhor segmento.",
        Text::AheadGainingTime => "À frente (a ganhar tempo)",
        Text::AheadGainingTimeDescription => {
            "A cor a usar quando está à frente e a ganhar ainda mais tempo."
        }
        Text::AheadLosingTime => "À frente (a perder tempo)",
        Text::AheadLosingTimeDescription => {
            "A cor a usar quando está à frente, mas a perder tempo."
        }
        Text::BehindGainingTime => "Atrás (a recuperar tempo)",
        Text::BehindGainingTimeDescription => {
            "A cor a usar quando está atrás, mas a recuperar tempo."
        }
        Text::BehindLosingTime => "Atrás (a perder tempo)",
        Text::BehindLosingTimeDescription => {
            "A cor a usar quando está atrás e a perder ainda mais tempo."
        }
        Text::NotRunning => "Não em execução",
        Text::NotRunningDescription => "A cor a usar quando não há uma tentativa ativa.",
        Text::PersonalBest => "Recorde pessoal",
        Text::PersonalBestDescription => "A cor a usar quando consegue um novo recorde pessoal.",
        Text::Paused => "Pausado",
        Text::PausedDescription => "A cor a usar quando o temporizador está pausado.",
        Text::ThinSeparators => "Separadores finos",
        Text::ThinSeparatorsDescription => "A cor dos separadores finos.",
        Text::Separators => "Separadores",
        Text::SeparatorsDescription => "A cor dos separadores normais.",
        Text::TextColor => "Texto",
        Text::TextColorDescription => "A cor a usar para texto que não especifica sua própria cor.",
        Text::ComponentBlankSpace => "Espaço em branco",
        Text::ComponentCurrentComparison => "Comparação atual",
        Text::ComponentCurrentPace => "Ritmo atual",
        Text::ComponentDelta => "Delta",
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
        Text::ComponentSegmentTimer => "Temporizador do segmento",
        Text::ComponentTitle => "Título",
        Text::ComponentTotalPlaytime => "Tempo total de jogo",
        Text::Row => "Linha",
        Text::Column => "Coluna",
        Text::EmptyContainer => "Vazio",
        Text::Carousel => "Carrossel",
        Text::CarouselFixedSize => "Tamanho fixo",
        Text::CarouselFixedSizeDescription => {
            "Substitui o tamanho do carrossel. Se não for definido, o tamanho é determinado automaticamente com base no maior componente filho."
        }
        Text::CarouselInterval => "Intervalo (segundos)",
        Text::CarouselIntervalDescription => {
            "O número máximo de segundos durante os quais um componente filho é mostrado antes de passar ao seguinte. O carrossel pode mudar mais cedo se o conteúdo de outro componente filho tiver mudado de forma significativa."
        }
        Text::CarouselMinDisplay => "Exibição mínima (segundos)",
        Text::CarouselMinDisplayDescription => {
            "O número mínimo de segundos para mostrar um componente filho antes de mudar para outro."
        }
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
        PlaceholderText::Hotkey(template) => match template {
            display::Template::NumpadKey => &[Piece::Static("Teclado num. "), Piece::Dynamic(0)],
            display::Template::GamepadButton => &[Piece::Static("Comando "), Piece::Dynamic(0)],
            display::Template::LeftKey => &[Piece::Dynamic(0), Piece::Static(" esquerdo")],
            display::Template::RightKey => &[Piece::Dynamic(0), Piece::Static(" direito")],
            display::Template::LaunchApp => &[Piece::Static("Abrir aplicação "), Piece::Dynamic(0)],
        },
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
