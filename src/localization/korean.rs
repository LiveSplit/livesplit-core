use super::{Piece, PlaceholderText, Text};

pub const fn resolve(text: Text) -> &'static str {
    match text {
        Text::StartSplit => "시작 / 스플릿",
        Text::StartSplitDescription => "스플릿과 새 시도를 시작하는 단축키입니다.",
        Text::Reset => "리셋",
        Text::ResetDescription => "현재 시도를 리셋하는 단축키입니다.",
        Text::UndoSplit => "스플릿 되돌리기",
        Text::UndoSplitDescription => "마지막 스플릿을 되돌리는 단축키입니다.",
        Text::SkipSplit => "스플릿 건너뛰기",
        Text::SkipSplitDescription => "현재 스플릿을 건너뛰는 단축키입니다.",
        Text::Pause => "일시정지",
        Text::PauseDescription => {
            "현재 시도를 일시정지하는 단축키입니다. 새 시도를 시작하는 데도 사용할 수 있습니다."
        }
        Text::UndoAllPauses => "모든 일시정지 취소",
        Text::UndoAllPausesDescription => {
            "현재 시간에서 모든 일시정지 시간을 제거하는 단축키입니다. 실수로 일시정지했을 때 유용합니다."
        }
        Text::PreviousComparison => "이전 비교",
        Text::PreviousComparisonDescription => "이전 비교로 전환하는 단축키입니다.",
        Text::NextComparison => "다음 비교",
        Text::NextComparisonDescription => "다음 비교로 전환하는 단축키입니다.",
        Text::ToggleTimingMethod => "타이밍 방법 전환",
        Text::ToggleTimingMethodDescription => "«실시간»과 «게임 시간»을 전환하는 단축키입니다.",
        Text::TimerBackground => "배경",
        Text::TimerBackgroundDescription => {
            "컴포넌트 뒤에 표시되는 배경입니다. 앞섬/뒤처짐 색상을 배경색으로 적용할 수도 있습니다."
        }
        Text::SegmentTimer => "세그먼트 타이머",
        Text::SegmentTimerDescription => {
            "현재 세그먼트 시작 이후 경과 시간을 표시할지, 시도 시작 이후 경과 시간을 표시할지 지정합니다."
        }
        Text::TimingMethod => "타이밍 방법",
        Text::TimingMethodDescription => {
            "사용할 타이밍 방법을 지정합니다. 지정하지 않으면 현재 방법이 사용됩니다."
        }
        Text::Height => "높이",
        Text::HeightDescription => "타이머의 높이입니다.",
        Text::TimerTextColor => "텍스트 색상",
        Text::TimerTextColorDescription => {
            "표시되는 시간의 색상입니다. 지정하지 않으면 현재 시도의 진행 상황에 따라 자동으로 선택됩니다. 이 색상은 레이아웃 일반 설정에서 지정할 수 있습니다."
        }
        Text::ShowGradient => "그라디언트 표시",
        Text::ShowGradientDescription => "타이머의 색상을 그라디언트로 표시할지 결정합니다.",
        Text::DigitsFormat => "자릿수 형식",
        Text::DigitsFormatDescription => {
            "표시할 자릿수를 지정합니다. 표시할 자릿수보다 시간이 짧으면 0이 표시됩니다."
        }
        Text::Accuracy => "정확도",
        Text::AccuracyDescription => "표시되는 시간의 정확도입니다.",
        Text::TitleBackground => "배경",
        Text::TitleBackgroundDescription => "컴포넌트 뒤에 표시되는 배경입니다.",
        Text::TitleTextColor => "텍스트 색상",
        Text::TitleTextColorDescription => {
            "제목 텍스트의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::ShowGameName => "게임 이름 표시",
        Text::ShowGameNameDescription => "표시되는 제목에 게임 이름을 포함할지 지정합니다.",
        Text::ShowCategoryName => "카테고리 이름 표시",
        Text::ShowCategoryNameDescription => "표시되는 제목에 카테고리 이름을 포함할지 지정합니다.",
        Text::ShowFinishedRunsCount => "완주 횟수 표시",
        Text::ShowFinishedRunsCountDescription => "성공적으로 완주한 횟수를 표시할지 지정합니다.",
        Text::ShowAttemptCount => "시도 횟수 표시",
        Text::ShowAttemptCountDescription => "총 시도 횟수를 표시할지 지정합니다.",
        Text::TextAlignment => "텍스트 정렬",
        Text::TextAlignmentDescription => "제목의 정렬을 지정합니다.",
        Text::DisplayTextAsSingleLine => "텍스트를 한 줄로 표시",
        Text::DisplayTextAsSingleLineDescription => {
            "제목을 게임 이름과 카테고리 이름으로 나누지 않고 한 줄로 표시할지 지정합니다."
        }
        Text::DisplayGameIcon => "게임 아이콘 표시",
        Text::DisplayGameIconDescription => {
            "스플릿에 게임 아이콘이 저장되어 있다면 표시할지 지정합니다."
        }
        Text::ShowRegion => "지역 표시",
        Text::ShowRegionDescription => {
            "카테고리 이름에 추가 정보를 붙입니다. 스플릿 편집기의 변수 탭에 지역이 제공된 경우 지역을 추가합니다."
        }
        Text::ShowPlatform => "플랫폼 표시",
        Text::ShowPlatformDescription => {
            "카테고리 이름에 추가 정보를 붙입니다. 스플릿 편집기의 변수 탭에 플랫폼이 제공된 경우 플랫폼을 추가합니다."
        }
        Text::ShowVariables => "변수 표시",
        Text::ShowVariablesDescription => {
            "카테고리 이름에 추가 정보를 붙입니다. 스플릿 편집기의 변수 탭에 제공된 추가 변수를 추가합니다. speedrun.com 변수에 해당하며 사용자 정의 변수는 아닙니다."
        }
        Text::TotalPlaytimeBackground => "배경",
        Text::TotalPlaytimeBackgroundDescription => "컴포넌트 뒤에 표시되는 배경입니다.",
        Text::DisplayTwoRows => "2줄 표시",
        Text::DisplayTwoRowsDescription => {
            "컴포넌트 이름과 총 플레이 시간을 두 줄로 표시할지 지정합니다."
        }
        Text::ShowDays => "일수 표시(>24시간)",
        Text::ShowDaysDescription => {
            "총 플레이 시간이 24시간 이상일 때 일수 표시 여부를 지정합니다."
        }
        Text::LabelColor => "라벨 색상",
        Text::LabelColorDescription => {
            "컴포넌트 이름의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::ValueColor => "값 색상",
        Text::ValueColorDescription => {
            "총 플레이 시간의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::BlankSpaceBackground => "배경",
        Text::BlankSpaceBackgroundDescription => "컴포넌트 뒤에 표시되는 배경입니다.",
        Text::BlankSpaceSize => "크기",
        Text::BlankSpaceSizeDescription => "컴포넌트의 크기입니다.",
        Text::CurrentComparisonBackground => "배경",
        Text::CurrentComparisonBackgroundDescription => "컴포넌트 뒤에 표시되는 배경입니다.",
        Text::CurrentComparisonDisplayTwoRows => "2줄 표시",
        Text::CurrentComparisonDisplayTwoRowsDescription => {
            "컴포넌트 이름과 비교를 두 줄로 표시할지 지정합니다."
        }
        Text::CurrentComparisonLabelColor => "라벨 색상",
        Text::CurrentComparisonLabelColorDescription => {
            "컴포넌트 이름의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::CurrentComparisonValueColor => "값 색상",
        Text::CurrentComparisonValueColorDescription => {
            "비교 이름의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::CurrentPaceBackground => "배경",
        Text::CurrentPaceBackgroundDescription => "컴포넌트 뒤에 표시되는 배경입니다.",
        Text::CurrentPaceComparison => "비교",
        Text::CurrentPaceComparisonDescription => {
            "최종 시간을 예측할 비교입니다. 지정하지 않으면 현재 비교를 사용합니다."
        }
        Text::CurrentPaceDisplayTwoRows => "2줄 표시",
        Text::CurrentPaceDisplayTwoRowsDescription => {
            "컴포넌트 이름과 예측 시간을 두 줄로 표시할지 지정합니다."
        }
        Text::CurrentPaceLabelColor => "라벨 색상",
        Text::CurrentPaceLabelColorDescription => {
            "컴포넌트 이름의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::CurrentPaceValueColor => "값 색상",
        Text::CurrentPaceValueColorDescription => {
            "예측 시간의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::CurrentPaceAccuracy => "정확도",
        Text::CurrentPaceAccuracyDescription => "표시되는 예측 시간의 정확도입니다.",
        Text::DeltaBackground => "배경",
        Text::DeltaBackgroundDescription => "컴포넌트 뒤에 표시되는 배경입니다.",
        Text::DeltaComparison => "비교",
        Text::DeltaComparisonDescription => {
            "현재 시도가 얼마나 앞서거나 뒤처졌는지 계산할 비교입니다. 지정하지 않으면 현재 비교를 사용합니다."
        }
        Text::DeltaDisplayTwoRows => "2줄 표시",
        Text::DeltaDisplayTwoRowsDescription => "비교 이름과 델타를 두 줄로 표시할지 지정합니다.",
        Text::DeltaLabelColor => "라벨 색상",
        Text::DeltaLabelColorDescription => {
            "비교 이름의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::DeltaDropDecimals => "소수점 생략",
        Text::DeltaDropDecimalsDescription => {
            "표시되는 델타가 1분을 넘을 때 소수점을 숨길지 지정합니다."
        }
        Text::DeltaAccuracy => "정확도",
        Text::DeltaAccuracyDescription => "표시되는 델타의 정확도입니다.",
        Text::SumOfBestBackground => "배경",
        Text::SumOfBestBackgroundDescription => "컴포넌트 뒤에 표시되는 배경입니다.",
        Text::SumOfBestDisplayTwoRows => "2줄 표시",
        Text::SumOfBestDisplayTwoRowsDescription => {
            "컴포넌트 이름과 베스트 세그먼트 합계를 두 줄로 표시할지 지정합니다."
        }
        Text::SumOfBestLabelColor => "라벨 색상",
        Text::SumOfBestLabelColorDescription => {
            "컴포넌트 이름의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::SumOfBestValueColor => "값 색상",
        Text::SumOfBestValueColorDescription => {
            "베스트 세그먼트 합계의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::SumOfBestAccuracy => "정확도",
        Text::SumOfBestAccuracyDescription => "표시되는 베스트 합계의 정확도입니다.",
        Text::PbChanceBackground => "배경",
        Text::PbChanceBackgroundDescription => "컴포넌트 뒤에 표시되는 배경입니다.",
        Text::PbChanceDisplayTwoRows => "2줄 표시",
        Text::PbChanceDisplayTwoRowsDescription => {
            "컴포넌트 이름과 PB 확률을 두 줄로 표시할지 지정합니다."
        }
        Text::PbChanceLabelColor => "라벨 색상",
        Text::PbChanceLabelColorDescription => {
            "컴포넌트 이름의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::PbChanceValueColor => "값 색상",
        Text::PbChanceValueColorDescription => {
            "PB 확률의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::PossibleTimeSaveBackground => "배경",
        Text::PossibleTimeSaveBackgroundDescription => "컴포넌트 뒤에 표시되는 배경입니다.",
        Text::PossibleTimeSaveComparison => "비교",
        Text::PossibleTimeSaveComparisonDescription => {
            "가능한 타임세이브를 계산할 비교입니다. 지정하지 않으면 현재 비교를 사용합니다."
        }
        Text::PossibleTimeSaveDisplayTwoRows => "2줄 표시",
        Text::PossibleTimeSaveDisplayTwoRowsDescription => {
            "컴포넌트 이름과 가능한 타임세이브를 두 줄로 표시할지 지정합니다."
        }
        Text::PossibleTimeSaveShowTotal => "전체 가능한 타임세이브 표시",
        Text::PossibleTimeSaveShowTotalDescription => {
            "현재 세그먼트의 타임세이브 대신 남은 전체의 타임세이브 합계를 표시할지 지정합니다."
        }
        Text::PossibleTimeSaveLabelColor => "라벨 색상",
        Text::PossibleTimeSaveLabelColorDescription => {
            "컴포넌트 이름의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::PossibleTimeSaveValueColor => "값 색상",
        Text::PossibleTimeSaveValueColorDescription => {
            "가능한 타임세이브의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::PossibleTimeSaveAccuracy => "정확도",
        Text::PossibleTimeSaveAccuracyDescription => "표시되는 가능한 타임세이브의 정확도입니다.",
        Text::PreviousSegmentBackground => "배경",
        Text::PreviousSegmentBackgroundDescription => "컴포넌트 뒤에 표시되는 배경입니다.",
        Text::PreviousSegmentComparison => "비교",
        Text::PreviousSegmentComparisonDescription => {
            "얼마나 시간을 얻거나 잃었는지 계산할 비교입니다. 지정하지 않으면 현재 비교를 사용합니다."
        }
        Text::PreviousSegmentDisplayTwoRows => "2줄 표시",
        Text::PreviousSegmentDisplayTwoRowsDescription => {
            "컴포넌트 이름과 얻거나 잃은 시간을 두 줄로 표시할지 지정합니다."
        }
        Text::PreviousSegmentLabelColor => "라벨 색상",
        Text::PreviousSegmentLabelColorDescription => {
            "컴포넌트 이름의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::PreviousSegmentDropDecimals => "소수점 생략",
        Text::PreviousSegmentDropDecimalsDescription => {
            "표시되는 시간이 1분을 넘을 때 소수점을 숨길지 지정합니다."
        }
        Text::PreviousSegmentAccuracy => "정확도",
        Text::PreviousSegmentAccuracyDescription => "표시되는 시간의 정확도입니다.",
        Text::PreviousSegmentShowPossibleTimeSave => "가능한 타임세이브 표시",
        Text::PreviousSegmentShowPossibleTimeSaveDescription => {
            "이전 세그먼트에서 얻거나 잃은 시간 외에 가능한 타임세이브도 표시할지 지정합니다."
        }
        Text::SegmentTimeBackground => "배경",
        Text::SegmentTimeBackgroundDescription => "컴포넌트 뒤에 표시되는 배경입니다.",
        Text::SegmentTimeComparison => "비교",
        Text::SegmentTimeComparisonDescription => {
            "세그먼트 시간에 대한 비교입니다. 지정하지 않으면 현재 비교를 사용합니다."
        }
        Text::SegmentTimeDisplayTwoRows => "2줄 표시",
        Text::SegmentTimeDisplayTwoRowsDescription => {
            "컴포넌트 이름과 세그먼트 시간을 두 줄로 표시할지 지정합니다."
        }
        Text::SegmentTimeLabelColor => "라벨 색상",
        Text::SegmentTimeLabelColorDescription => {
            "컴포넌트 이름의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::SegmentTimeValueColor => "값 색상",
        Text::SegmentTimeValueColorDescription => {
            "세그먼트 시간의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::SegmentTimeAccuracy => "정확도",
        Text::SegmentTimeAccuracyDescription => "표시되는 세그먼트 시간의 정확도입니다.",
        Text::GraphComparison => "비교",
        Text::GraphComparisonDescription => {
            "그래프에 사용할 비교입니다. 지정하지 않으면 현재 비교를 사용합니다."
        }
        Text::GraphHeight => "높이",
        Text::GraphHeightDescription => "그래프 높이입니다.",
        Text::GraphShowBestSegments => "베스트 세그먼트 표시",
        Text::GraphShowBestSegmentsDescription => {
            "베스트 세그먼트를 레이아웃의 베스트 세그먼트 색상으로 표시할지 지정합니다."
        }
        Text::GraphLiveGraph => "라이브 그래프",
        Text::GraphLiveGraphDescription => {
            "그래프를 항상 자동으로 새로고침할지 지정합니다. 비활성화하면 현재 세그먼트가 바뀔 때만 변경됩니다."
        }
        Text::GraphFlipGraph => "그래프 뒤집기",
        Text::GraphFlipGraphDescription => {
            "그래프를 수직으로 뒤집을지 지정합니다. 비활성화하면 앞서는 시간은 x축 아래, 뒤처진 시간은 위에 표시됩니다."
        }
        Text::GraphBehindBackgroundColor => "뒤처짐 배경색",
        Text::GraphBehindBackgroundColorDescription => "비교보다 뒤처진 시간 영역의 배경색입니다.",
        Text::GraphAheadBackgroundColor => "앞섬 배경색",
        Text::GraphAheadBackgroundColorDescription => "비교보다 앞선 시간 영역의 배경색입니다.",
        Text::GraphGridLinesColor => "격자선 색상",
        Text::GraphGridLinesColorDescription => "그래프의 격자선 색상입니다.",
        Text::GraphLinesColor => "그래프 선 색상",
        Text::GraphLinesColorDescription => "그래프 점을 연결하는 선의 색상입니다.",
        Text::GraphPartialFillColor => "부분 채우기 색상",
        Text::GraphPartialFillColorDescription => {
            "x축과 그래프 사이 영역의 색상입니다. 부분 채우기 색상은 라이브 변화에만 사용되며 마지막 스플릿부터 현재 시간까지 적용됩니다."
        }
        Text::GraphCompleteFillColor => "전체 채우기 색상",
        Text::GraphCompleteFillColorDescription => {
            "x축과 그래프 사이 영역의 색상(라이브 변화 구간 제외)."
        }
        Text::DetailedTimerBackground => "배경",
        Text::DetailedTimerBackgroundDescription => "컴포넌트 뒤에 표시되는 배경입니다.",
        Text::DetailedTimerTimingMethod => "타이밍 방법",
        Text::DetailedTimerTimingMethodDescription => {
            "사용할 타이밍 방법을 지정합니다. 지정하지 않으면 현재 방법이 사용됩니다."
        }
        Text::DetailedTimerComparison1 => "비교 1",
        Text::DetailedTimerComparison1Description => {
            "세그먼트 시간을 표시할 첫 번째 비교입니다. 지정하지 않으면 현재 비교를 사용합니다."
        }
        Text::DetailedTimerComparison2 => "비교 2",
        Text::DetailedTimerComparison2Description => {
            "세그먼트 시간을 표시할 두 번째 비교입니다. 지정하지 않으면 현재 비교를 사용합니다(첫 번째 비교도 None인 경우 제외). 두 번째 비교가 숨겨진 경우 표시되지 않습니다."
        }
        Text::DetailedTimerHideSecondComparison => "두 번째 비교 숨기기",
        Text::DetailedTimerHideSecondComparisonDescription => "비교를 하나만 표시할지 지정합니다.",
        Text::DetailedTimerTimerHeight => "타이머 높이",
        Text::DetailedTimerTimerHeightDescription => "런 타이머의 높이입니다.",
        Text::DetailedTimerSegmentTimerHeight => "세그먼트 타이머 높이",
        Text::DetailedTimerSegmentTimerHeightDescription => "세그먼트 타이머의 높이입니다.",
        Text::DetailedTimerTimerColor => "타이머 색상",
        Text::DetailedTimerTimerColorDescription => {
            "현재 진행에 따라 자동으로 색상을 결정하는 대신, 항상 사용할 색상을 지정할 수 있습니다."
        }
        Text::DetailedTimerShowTimerGradient => "타이머 그라디언트 표시",
        Text::DetailedTimerShowTimerGradientDescription => {
            "이 옵션이 활성화되면 타이머 색상이 자동으로 수직 그라디언트로 변합니다. 그렇지 않으면 실제 색상이 사용됩니다."
        }
        Text::DetailedTimerTimerDigitsFormat => "타이머 자릿수 형식",
        Text::DetailedTimerTimerDigitsFormatDescription => {
            "타이머에 표시할 자릿수를 지정합니다. 시간이 짧으면 0이 표시됩니다."
        }
        Text::DetailedTimerTimerAccuracy => "타이머 정확도",
        Text::DetailedTimerTimerAccuracyDescription => "타이머에 표시되는 시간의 정확도입니다.",
        Text::DetailedTimerSegmentTimerColor => "세그먼트 타이머 색상",
        Text::DetailedTimerSegmentTimerColorDescription => {
            "세그먼트 타이머의 색상을 기본색과 다른 색으로 변경합니다."
        }
        Text::DetailedTimerShowSegmentTimerGradient => "세그먼트 타이머 그라디언트 표시",
        Text::DetailedTimerShowSegmentTimerGradientDescription => {
            "이 옵션이 활성화되면 세그먼트 타이머 색상이 자동으로 수직 그라디언트로 변합니다. 그렇지 않으면 실제 색상이 사용됩니다."
        }
        Text::DetailedTimerSegmentTimerDigitsFormat => "세그먼트 타이머 자릿수 형식",
        Text::DetailedTimerSegmentTimerDigitsFormatDescription => {
            "세그먼트 타이머에 표시할 자릿수를 지정합니다. 시간이 짧으면 0이 표시됩니다."
        }
        Text::DetailedTimerSegmentTimerAccuracy => "세그먼트 타이머 정확도",
        Text::DetailedTimerSegmentTimerAccuracyDescription => {
            "세그먼트 타이머에 표시되는 시간의 정확도입니다."
        }
        Text::DetailedTimerComparisonNamesColor => "비교 이름 색상",
        Text::DetailedTimerComparisonNamesColorDescription => {
            "비교 이름을 표시할 때의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::DetailedTimerComparisonTimesColor => "비교 시간 색상",
        Text::DetailedTimerComparisonTimesColorDescription => {
            "비교 시간을 표시할 때의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::DetailedTimerComparisonTimesAccuracy => "비교 시간 정확도",
        Text::DetailedTimerComparisonTimesAccuracyDescription => "비교 시간의 정확도입니다.",
        Text::DetailedTimerShowSegmentName => "세그먼트 이름 표시",
        Text::DetailedTimerShowSegmentNameDescription => "세그먼트 이름을 표시할지 지정합니다.",
        Text::DetailedTimerSegmentNameColor => "세그먼트 이름 색상",
        Text::DetailedTimerSegmentNameColorDescription => {
            "세그먼트 이름을 표시할 때의 색상입니다. 지정하지 않으면 레이아웃의 색상을 사용합니다."
        }
        Text::DetailedTimerDisplayIcon => "아이콘 표시",
        Text::DetailedTimerDisplayIconDescription => "세그먼트 아이콘을 표시할지 지정합니다.",
        Text::SplitsBackground => "배경",
        Text::SplitsBackgroundDescription => {
            "컴포넌트 뒤에 표시되는 배경입니다. 교차 색상을 선택하면 각 줄이 두 색을 번갈아 사용합니다."
        }
        Text::SplitsTotalRows => "총 행 수",
        Text::SplitsTotalRowsDescription => {
            "표시할 세그먼트 행의 총 개수입니다. 0이면 모든 세그먼트를 표시합니다. 총 개수보다 작은 값이면 창이 표시되며 위아래로 스크롤됩니다."
        }
        Text::SplitsUpcomingSegments => "다가오는 세그먼트",
        Text::SplitsUpcomingSegmentsDescription => {
            "표시 행보다 세그먼트가 많으면 현재 세그먼트가 바뀔 때 창이 자동으로 스크롤됩니다. 이 값은 표시할 최소 미래 세그먼트 수를 지정합니다."
        }
        Text::SplitsShowThinSeparators => "얇은 구분선 표시",
        Text::SplitsShowThinSeparatorsDescription => {
            "세그먼트 행 사이에 얇은 구분선을 표시할지 지정합니다."
        }
        Text::SplitsShowSeparatorBeforeLastSplit => "마지막 스플릿 앞 구분선 표시",
        Text::SplitsShowSeparatorBeforeLastSplitDescription => {
            "마지막 세그먼트를 항상 표시하는 경우, 스크롤 창에서 바로 앞 세그먼트와 인접하지 않을 때 더 두드러진 구분선을 표시할지 지정합니다."
        }
        Text::SplitsAlwaysShowLastSplit => "마지막 스플릿 항상 표시",
        Text::SplitsAlwaysShowLastSplitDescription => {
            "모든 세그먼트를 표시하지 않는 경우, 마지막 세그먼트를 항상 표시할지 지정합니다. 마지막 세그먼트에는 선택한 비교의 총 시간이 포함됩니다."
        }
        Text::SplitsFillWithBlankSpace => "빈 공간으로 채우기",
        Text::SplitsFillWithBlankSpaceDescription => {
            "세그먼트가 부족하면 빈 공간으로 남은 행을 채워 항상 총 행 수를 표시할지 지정합니다."
        }
        Text::SplitsShowTimesBelowSegmentName => "세그먼트 이름 아래에 시간 표시",
        Text::SplitsShowTimesBelowSegmentNameDescription => {
            "세그먼트 이름 아래에 시간을 표시할지 지정합니다. 그렇지 않으면 이름 옆에 표시됩니다."
        }
        Text::SplitsCurrentSegmentGradient => "현재 세그먼트 그라디언트",
        Text::SplitsCurrentSegmentGradientDescription => {
            "현재 세그먼트를 표시하기 위해 뒤에 표시되는 그라디언트입니다."
        }
        Text::SplitsSplitTimeAccuracy => "스플릿 시간 정확도",
        Text::SplitsSplitTimeAccuracyDescription => {
            "스플릿 시간이 포함된 열에 사용할 정확도입니다."
        }
        Text::SplitsSegmentTimeAccuracy => "세그먼트 시간 정확도",
        Text::SplitsSegmentTimeAccuracyDescription => {
            "세그먼트 시간이 포함된 열에 사용할 정확도입니다."
        }
        Text::SplitsDeltaTimeAccuracy => "델타 시간 정확도",
        Text::SplitsDeltaTimeAccuracyDescription => {
            "앞섬/뒤처짐 시간이 포함된 열에 사용할 정확도입니다."
        }
        Text::SplitsDropDeltaDecimals => "분 표시 시 델타 소수점 숨기기",
        Text::SplitsDropDeltaDecimalsDescription => {
            "델타가 1분을 넘을 때 소수점을 더 이상 표시하지 않을지 지정합니다."
        }
        Text::SplitsShowColumnLabels => "열 라벨 표시",
        Text::SplitsShowColumnLabelsDescription => "목록 상단에 열 이름을 표시할지 지정합니다.",
        Text::SplitsColumns => "열",
        Text::SplitsColumnsDescription => {
            "각 행에 표시할 열 수입니다. 각 열은 서로 다른 정보를 표시할 수 있으며, 열은 오른쪽에서 왼쪽 순으로 정의됩니다."
        }
        Text::SplitsColumnName => "열 이름",
        Text::SplitsColumnNameDescription => {
            "열의 이름입니다. 열 라벨을 표시하는 경우 목록 상단에 표시됩니다."
        }
        Text::SplitsColumnType => "열 유형",
        Text::SplitsColumnTypeDescription => {
            "이 열에 표시되는 정보의 유형입니다. 시간 또는 사용자 정의 변수를 표시할 수 있습니다."
        }
        Text::SplitsVariableName => "변수 이름",
        Text::SplitsVariableNameDescription => "이 열에 표시되는 사용자 정의 변수의 이름입니다.",
        Text::SplitsStartWith => "시작 값",
        Text::SplitsStartWithDescription => {
            "각 세그먼트에서 이 열이 시작하는 값입니다. 업데이트 트리거가 충족되면 값이 대체됩니다."
        }
        Text::SplitsUpdateWith => "업데이트 값",
        Text::SplitsUpdateWithDescription => {
            "특정 조건이 충족되면(보통 세그먼트에 있거나 완료했을 때) 이 값으로 업데이트됩니다."
        }
        Text::SplitsUpdateTrigger => "업데이트 트리거",
        Text::SplitsUpdateTriggerDescription => {
            "업데이트 값으로 바꾸기 위한 조건입니다. 이 조건 전에는 시작 값이 표시됩니다."
        }
        Text::SplitsColumnComparison => "비교",
        Text::SplitsColumnComparisonDescription => {
            "이 열이 비교할 비교 항목입니다. 지정하지 않으면 현재 비교를 사용합니다."
        }
        Text::SplitsColumnTimingMethod => "타이밍 방법",
        Text::SplitsColumnTimingMethodDescription => {
            "이 열에 사용할 타이밍 방법입니다. 지정하지 않으면 현재 방법을 사용합니다."
        }
        Text::TextComponentBackground => "배경",
        Text::TextComponentBackgroundDescription => "컴포넌트 뒤에 표시되는 배경입니다.",
        Text::TextComponentUseVariable => "변수 사용",
        Text::TextComponentUseVariableDescription => {
            "동적 값을 표시하기 위해 사용자 정의 변수를 사용할지 지정합니다. 사용자 정의 변수는 스플릿 편집기에서 설정하거나 오토 스플리터가 제공할 수 있습니다."
        }
        Text::TextComponentSplit => "분할",
        Text::TextComponentSplitDescription => {
            "텍스트를 좌우로 분할할지 지정합니다. 분할하지 않으면 가운데에 하나의 텍스트만 표시됩니다."
        }
        Text::TextComponentText => "텍스트",
        Text::TextComponentTextDescription => "가운데에 표시할 텍스트입니다.",
        Text::TextComponentLeft => "왼쪽",
        Text::TextComponentLeftDescription => "왼쪽에 표시할 텍스트입니다.",
        Text::TextComponentRight => "오른쪽",
        Text::TextComponentRightDescription => "오른쪽에 표시할 텍스트입니다.",
        Text::TextComponentVariable => "변수",
        Text::TextComponentVariableDescription => "표시할 사용자 정의 변수의 이름입니다.",
        Text::TextComponentTextColor => "텍스트 색상",
        Text::TextComponentTextColorDescription => "텍스트의 색상입니다.",
        Text::TextComponentLeftColor => "왼쪽 색상",
        Text::TextComponentLeftColorDescription => "왼쪽 텍스트 색상입니다.",
        Text::TextComponentRightColor => "오른쪽 색상",
        Text::TextComponentRightColorDescription => "오른쪽 텍스트 색상입니다.",
        Text::TextComponentNameColor => "이름 색상",
        Text::TextComponentNameColorDescription => "변수 이름의 색상입니다.",
        Text::TextComponentValueColor => "값 색상",
        Text::TextComponentValueColorDescription => "변수 값의 색상입니다.",
        Text::TextComponentDisplayTwoRows => "2줄 표시",
        Text::TextComponentDisplayTwoRowsDescription => {
            "왼쪽과 오른쪽 텍스트를 두 줄로 표시할지 지정합니다."
        }
        Text::LayoutDirection => "레이아웃 방향",
        Text::LayoutDirectionDescription => "컴포넌트를 배치하는 방향입니다.",
        Text::CustomTimerFont => "타이머 사용자 지정 글꼴",
        Text::CustomTimerFontDescription => {
            "타이머에 사용자 지정 글꼴을 지정할 수 있습니다. 지정하지 않으면 기본 글꼴이 사용됩니다."
        }
        Text::CustomTimesFont => "시간 사용자 지정 글꼴",
        Text::CustomTimesFontDescription => {
            "시간에 사용자 지정 글꼴을 지정할 수 있습니다. 지정하지 않으면 기본 글꼴이 사용됩니다."
        }
        Text::CustomTextFont => "텍스트 사용자 지정 글꼴",
        Text::CustomTextFontDescription => {
            "텍스트에 사용자 지정 글꼴을 지정할 수 있습니다. 지정하지 않으면 기본 글꼴이 사용됩니다."
        }
        Text::TextShadow => "텍스트 그림자",
        Text::TextShadowDescription => "텍스트 그림자 색상을 선택적으로 지정할 수 있습니다.",
        Text::Background => "배경",
        Text::BackgroundDescription => "레이아웃 전체의 배경입니다.",
        Text::BestSegment => "베스트 세그먼트",
        Text::BestSegmentDescription => "새로운 베스트 세그먼트를 달성했을 때 사용할 색상입니다.",
        Text::AheadGainingTime => "앞섬(시간 벌기)",
        Text::AheadGainingTimeDescription => "비교보다 앞서면서 더 시간을 벌 때 사용할 색상입니다.",
        Text::AheadLosingTime => "앞섬(시간 잃기)",
        Text::AheadLosingTimeDescription => {
            "비교보다 앞서 있지만 시간을 잃을 때 사용할 색상입니다."
        }
        Text::BehindGainingTime => "뒤처짐(시간 회복)",
        Text::BehindGainingTimeDescription => {
            "비교보다 뒤처져 있지만 시간을 회복할 때 사용할 색상입니다."
        }
        Text::BehindLosingTime => "뒤처짐(시간 잃기)",
        Text::BehindLosingTimeDescription => {
            "비교보다 뒤처져 있고 더 시간을 잃을 때 사용할 색상입니다."
        }
        Text::NotRunning => "비실행",
        Text::NotRunningDescription => "활성 시도가 없을 때 사용할 색상입니다.",
        Text::PersonalBest => "개인 최고",
        Text::PersonalBestDescription => "새로운 개인 최고를 달성했을 때 사용할 색상입니다.",
        Text::Paused => "일시정지",
        Text::PausedDescription => "타이머가 일시정지일 때 사용할 색상입니다.",
        Text::ThinSeparators => "얇은 구분선",
        Text::ThinSeparatorsDescription => "얇은 구분선의 색상입니다.",
        Text::Separators => "구분선",
        Text::SeparatorsDescription => "일반 구분선의 색상입니다.",
        Text::TextColor => "텍스트",
        Text::TextColorDescription => "자체 색상을 지정하지 않은 텍스트에 사용할 색상입니다.",
        Text::ComponentBlankSpace => "빈 공간",
        Text::ComponentCurrentComparison => "현재 비교",
        Text::ComponentCurrentPace => "현재 페이스",
        Text::ComponentDelta => "차이",
        Text::ComponentDetailedTimer => "상세 타이머",
        Text::ComponentGraph => "그래프",
        Text::ComponentPbChance => "PB 가능성",
        Text::ComponentPossibleTimeSave => "가능한 시간 절약",
        Text::ComponentPreviousSegment => "이전 세그먼트",
        Text::ComponentSegmentTime => "세그먼트 시간",
        Text::ComponentSeparator => "구분선",
        Text::ComponentSplits => "스플릿",
        Text::ComponentSumOfBest => "최고 합계",
        Text::ComponentText => "텍스트",
        Text::ComponentTimer => "타이머",
        Text::ComponentSegmentTimer => "세그먼트 타이머",
        Text::ComponentTitle => "제목",
        Text::ComponentTotalPlaytime => "총 플레이 시간",
        Text::ComponentCurrentPaceBestPossibleTime => "최적 가능 시간",
        Text::ComponentCurrentPaceWorstPossibleTime => "최악 가능 시간",
        Text::ComponentCurrentPacePredictedTime => "예측 시간",
        Text::ComponentSegmentTimeBest => "최단 세그먼트 시간",
        Text::ComponentSegmentTimeWorst => "최악 세그먼트 시간",
        Text::ComponentSegmentTimeAverage => "평균 세그먼트 시간",
        Text::ComponentSegmentTimeMedian => "중앙값 세그먼트 시간",
        Text::ComponentSegmentTimeLatest => "최신 세그먼트 시간",
        Text::ComponentPossibleTimeSaveTotal => "총 절약 가능 시간",
        Text::LiveSegment => "라이브 세그먼트",
        Text::LiveSegmentShort => "라이브 세그먼트",
        Text::PreviousSegmentShort => "이전 세그먼트",
        Text::PreviousSegmentAbbreviation => "이전 세그",
        Text::ComparingAgainst => "비교 대상",
        Text::ComparisonShort => "비교",
        Text::CurrentPaceBestPossibleTimeShort => "최적 가능",
        Text::CurrentPaceBestTimeShort => "최적 시간",
        Text::CurrentPaceBestPossibleTimeAbbreviation => "최적",
        Text::CurrentPaceWorstPossibleTimeShort => "최악 가능",
        Text::CurrentPaceWorstTimeShort => "최악 시간",
        Text::CurrentPacePredictedTimeShort => "예측 시간",
        Text::CurrentPaceShort => "현 페이스",
        Text::CurrentPaceAbbreviation => "페이스",
        Text::Goal => "목표",
        Text::SumOfBestSegments => "베스트 세그먼트 합계",
        Text::SumOfBestShort => "베스트 합계",
        Text::SumOfBestAbbreviation => "베합",
        Text::PlaytimeShort => "플레이 시간",
        Text::BestSegmentTimeShort => "최고 세그 시간",
        Text::BestSegmentShort => "최고 세그먼트",
        Text::WorstSegmentTimeShort => "최악 세그 시간",
        Text::WorstSegmentShort => "최악 세그먼트",
        Text::AverageSegmentTimeShort => "평균 세그 시간",
        Text::AverageSegmentShort => "평균 세그먼트",
        Text::MedianSegmentTimeShort => "중앙값 세그 시간",
        Text::MedianSegmentShort => "중앙값 세그먼트",
        Text::LatestSegmentTimeShort => "최신 세그 시간",
        Text::LatestSegmentShort => "최신 세그먼트",
        Text::SegmentTimeShort => "세그 시간",
        Text::PossibleTimeSaveShort => "절약 가능 시간",
        Text::PossibleTimeSaveAbbreviation => "절약 가능",
        Text::TimeSaveShort => "절약",
        Text::RealTime => "실시간",
        Text::GameTime => "게임 시간",
        Text::SumOfBestCleanerStartOfRun => "런 시작",
        Text::SumOfBestCleanerShouldRemove => {
            " 이 세그먼트 시간이 부정확하다고 보시나요? 그렇다면 삭제하는 게 좋을까요?"
        }
    }
}

pub const fn resolve_placeholder(text: PlaceholderText) -> &'static [Piece] {
    match text {
        PlaceholderText::SumOfBestCleanerSegmentTimeBetween => &[
            Piece::Dynamic(0),
            Piece::Static(" 기준으로 “"),
            Piece::Dynamic(2),
            Piece::Static("”부터 “"),
            Piece::Dynamic(3),
            Piece::Static("”까지의 세그먼트 시간이 "),
            Piece::Dynamic(1),
            Piece::Static("로 기록되었습니다."),
        ],
        PlaceholderText::SumOfBestCleanerFasterThanCombined => &[
            Piece::Static(" 이는 베스트 세그먼트 합계 "),
            Piece::Dynamic(0),
            Piece::Static("보다 빠른 기록입니다."),
        ],
        PlaceholderText::SumOfBestCleanerRunOn => &[
            Piece::Static(" 이 기록은 "),
            Piece::Dynamic(0),
            Piece::Static(" "),
            Piece::Dynamic(1),
            Piece::Static("에 시작한 런에서 나온 것입니다."),
        ],
    }
}
