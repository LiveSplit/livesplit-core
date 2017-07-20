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

export interface LayoutStateJson {
    components: ComponentStateJson[];
    background_color: Color;
    thin_separators_color: Color;
    separators_color: Color;
    text_color: Color;
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
    height: number;
}

export interface TimerComponentStateJson {
    time: string;
    fraction: string;
    semantic_color: SemanticColor;
    top_color: Color;
    bottom_color: Color;
}

export interface TitleComponentStateJson {
    icon_change?: string;
    line1: string;
    line2?: string;
    is_centered: boolean;
    finished_runs?: number;
    attempts?: number;
}

export interface SplitsComponentStateJson {
    splits: SplitStateJson[];
    show_final_separator: boolean;
}

export interface SplitStateJson {
    icon_change?: string;
    name: string;
    delta: string;
    time: string;
    semantic_color: SemanticColor;
    visual_color: Color;
    is_current_split: boolean;
}

export interface PreviousSegmentComponentStateJson {
    text: string;
    time: string;
    semantic_color: SemanticColor;
    visual_color: Color;
}

export interface SumOfBestComponentStateJson {
    text: string;
    time: string;
}

export interface PossibleTimeSaveComponentStateJson {
    text: string;
    time: string;
}

export interface GraphComponentStateJson {
    points: GraphComponentStatePointJson[];
    horizontal_grid_lines: number[];
    vertical_grid_lines: number[];
    middle: number;
    is_live_delta_active: boolean;
    is_flipped: boolean;
}

export interface GraphComponentStatePointJson {
    x: number;
    y: number;
    is_best_segment: boolean;
}

export type TextComponentStateJson =
	{ Center: String } |
	{ Split: String[2] };

export interface TotalPlaytimeComponentStateJson {
    text: string;
    time: string;
}

export interface CurrentPaceComponentStateJson {
    text: string;
    time: string;
}

export interface DeltaComponentStateJson {
    text: string;
    time: string;
    semantic_color: SemanticColor;
    visual_color: Color;
}

export interface CurrentComparisonComponentStateJson {
    text: string;
    comparison: string;
}

export interface DetailedTimerComponentStateJson {
    timer: TimerComponentStateJson;
    segment_timer: TimerComponentStateJson;
    comparison1: DetailedTimerComponentComparisonStateJson;
    comparison2: DetailedTimerComponentComparisonStateJson;
    segment_name: string;
    icon_change: string;
}

export interface DetailedTimerComponentComparisonStateJson {
    name: string;
    time: string;
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
    { OptionalString: string } |
    { Float: number } |
    { Accuracy: AccuracyJson } |
    { DigitsFormat: DigitsFormatJson } |
    { Color: Color };

export type AccuracyJson = "Seconds" | "Tenths" | "Hundredths";

export type DigitsFormatJson =
    "SingleDigitSeconds" |
    "DoubleDigitSeconds" |
    "SingleDigitMinutes" |
    "DoubleDigitMinutes" |
    "SingleDigitHours" |
    "DoubleDigitHours";

export interface RunEditorStateJson {
    icon_change?: string,
    game: string,
    category: string,
    offset: string,
    attempts: string,
    timing_method: "RealTime" | "GameTime",
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
    icon_change?: string,
    name: string,
    split_time: string,
    segment_time: string,
    best_segment_time: string,
    comparison_times: string[],
    selected: "NotSelected" | "Selected" | "CurrentRow",
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
