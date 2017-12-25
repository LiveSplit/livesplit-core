pub static HEADER: &str = r#"export type ComponentStateJson =
    { BlankSpace: BlankSpaceComponentStateJson } |
    { CurrentComparison: CurrentComparisonComponentStateJson } |
    { CurrentPace: CurrentPaceComponentStateJson } |
    { Delta: DeltaComponentStateJson } |
    { Graph: GraphComponentStateJson } |
    { PossibleTimeSave: PossibleTimeSaveComponentStateJson } |
    { PreviousSegment: PreviousSegmentComponentStateJson } |
    { Separator: null } |
    { Splits: SplitsComponentStateJson } |
    { SumOfBest: SumOfBestComponentStateJson } |
    { Text: TextComponentStateJson } |
    { Timer: TimerComponentStateJson } |
    { Title: TitleComponentStateJson } |
    { TotalPlaytime: TotalPlaytimeComponentStateJson };

export type Color = number[];

export type Gradient =
    "Transparent" |
    { Plain: Color } |
    { Vertical: Color[] } |
    { Horizontal: Color[] };

export interface LayoutStateJson {
    components: ComponentStateJson[],
    background: Gradient,
    thin_separators_color: Color,
    separators_color: Color,
    text_color: Color,
}

export enum TimingMethod {
    RealTime = 0,
    GameTime = 1,
}

export enum TimerPhase {
    NotRunning = 0,
    Running = 1,
    Ended = 2,
    Paused = 3,
}

export interface BlankSpaceComponentStateJson {
    background: Gradient,
    height: number,
}

export interface TimerComponentStateJson {
    background: Gradient,
    time: string,
    fraction: string,
    semantic_color: SemanticColor,
    top_color: Color,
    bottom_color: Color,
    height: number,
}

export interface TitleComponentStateJson {
    background: Gradient,
    text_color: Color | null,
    icon_change: string | null,
    line1: string,
    line2: string | null,
    is_centered: boolean,
    finished_runs: number | null,
    attempts: number | null,
}

export interface SplitsComponentStateJson {
    splits: SplitStateJson[],
    icon_changes: SplitsComponentIconChangeJson[],
    show_final_separator: boolean,
    current_split_gradient: Gradient,
}

export interface SplitsComponentIconChangeJson {
    segment_index: number,
    icon: string,
}

export interface SplitStateJson {
    name: string,
    delta: string,
    time: string,
    semantic_color: SemanticColor,
    visual_color: Color,
    is_current_split: boolean,
    index: number,
}

export interface PreviousSegmentComponentStateJson {
    background: Gradient,
    label_color: Color | null,
    text: string,
    time: string,
    semantic_color: SemanticColor,
    visual_color: Color,
}

export interface SumOfBestComponentStateJson {
    background: Gradient,
    label_color: Color | null,
    value_color: Color | null,
    text: string,
    time: string,
}

export interface PossibleTimeSaveComponentStateJson {
    background: Gradient,
    label_color: Color | null,
    value_color: Color | null,
    text: string,
    time: string,
}

export interface GraphComponentStateJson {
    points: GraphComponentStatePointJson[],
    horizontal_grid_lines: number[],
    vertical_grid_lines: number[],
    middle: number,
    is_live_delta_active: boolean,
    is_flipped: boolean,
    top_background_color: Color,
    bottom_background_color: Color,
    grid_lines_color: Color,
    graph_lines_color: Color,
    partial_fill_color: Color,
    complete_fill_color: Color,
    best_segment_color: Color,
    height: number,
}

export interface GraphComponentStatePointJson {
    x: number,
    y: number,
    is_best_segment: boolean,
}

export type TextComponentStateJson =
	{ Center: string } |
	{ Split: string[] };

export interface TotalPlaytimeComponentStateJson {
    background: Gradient,
    label_color: Color | null,
    value_color: Color | null,
    text: string,
    time: string,
}

export interface CurrentPaceComponentStateJson {
    background: Gradient,
    label_color: Color | null,
    value_color: Color | null,
    text: string,
    time: string,
}

export interface DeltaComponentStateJson {
    background: Gradient,
    label_color: Color | null,
    text: string,
    time: string,
    semantic_color: SemanticColor,
    visual_color: Color,
}

export interface CurrentComparisonComponentStateJson {
    background: Gradient,
    label_color: Color | null,
    value_color: Color | null,
    text: string,
    comparison: string,
}

export interface DetailedTimerComponentStateJson {
    background: Gradient,
    timer: TimerComponentStateJson,
    segment_timer: TimerComponentStateJson,
    comparison1: DetailedTimerComponentComparisonStateJson | null,
    comparison2: DetailedTimerComponentComparisonStateJson | null,
    segment_name: string | null,
    icon_change: string | null,
}

export interface DetailedTimerComponentComparisonStateJson {
    name: string,
    time: string,
}

export interface LayoutEditorStateJson {
    components: string[],
    buttons: LayoutEditorButtonsJson,
    selected_component: number,
    component_settings: SettingsDescriptionJson,
    general_settings: SettingsDescriptionJson,
}

export interface LayoutEditorButtonsJson {
    can_remove: boolean,
    can_move_up: boolean,
    can_move_down: boolean,
}

export interface SettingsDescriptionJson {
    fields: SettingsDescriptionFieldJson[],
}

export interface SettingsDescriptionFieldJson {
    text: string,
    value: SettingsDescriptionValueJson,
}

export type SettingsDescriptionValueJson =
    { Bool: boolean } |
    { UInt: number } |
    { Int: number } |
    { String: string } |
    { OptionalString: string | null } |
    { Float: number } |
    { Accuracy: AccuracyJson } |
    { DigitsFormat: DigitsFormatJson } |
    { OptionalTimingMethod: TimingMethodJson | null } |
    { Color: Color } |
    { OptionalColor: Color | null } |
    { Gradient: Gradient };

export type AccuracyJson = "Seconds" | "Tenths" | "Hundredths";

export type TimingMethodJson = "RealTime" | "GameTime";

export type DigitsFormatJson =
    "SingleDigitSeconds" |
    "DoubleDigitSeconds" |
    "SingleDigitMinutes" |
    "DoubleDigitMinutes" |
    "SingleDigitHours" |
    "DoubleDigitHours";

export interface RunEditorStateJson {
    icon_change: string | null,
    game: string,
    category: string,
    offset: string,
    attempts: number,
    timing_method: TimingMethodJson,
    segments: RunEditorRowJson[],
    comparison_names: string[],
    buttons: RunEditorButtonsJson,
}

export interface RunEditorButtonsJson {
    can_remove: boolean,
    can_move_up: boolean,
    can_move_down: boolean,
}

export interface RunEditorRowJson {
    icon_change: string | null,
    name: string,
    split_time: string,
    segment_time: string,
    best_segment_time: string,
    comparison_times: string[],
    selected: "NotSelected" | "Selected" | "Active",
}

export type SemanticColor = "Default" |
    "AheadGainingTime" |
    "AheadLosingTime" |
    "BehindLosingTime" |
    "BehindGainingTime" |
    "BestSegment" |
    "NotRunning" |
    "Paused" |
    "PersonalBest";
"#;
