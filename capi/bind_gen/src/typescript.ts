/** The state object for one of the components available. */
export type ComponentStateJson =
    { BlankSpace: BlankSpaceComponentStateJson } |
    { DetailedTimer: DetailedTimerComponentStateJson } |
    { Graph: GraphComponentStateJson } |
    { KeyValue: KeyValueComponentStateJson } |
    { Separator: null } |
    { Splits: SplitsComponentStateJson } |
    { Text: TextComponentStateJson } |
    { Timer: TimerComponentStateJson } |
    { Title: TitleComponentStateJson };

/**
 * Colors can be used to describe what color to use for visualizing backgrounds,
 * texts, lines and various other elements that are being shown. They are stored
 * as RGBA colors with floating point numbers ranging from 0.0 to 1.0 per channel.
 */
export type Color = number[];

/**
 * Describes a Gradient for coloring a region with more than just a single
 * color.
 */
export type Gradient =
    "Transparent" |
    { Plain: Color } |
    { Vertical: Color[] } |
    { Horizontal: Color[] };

/**
 * Describes an extended form of a gradient, specifically made for use with
 * lists. It allows specifying different coloration for the rows in a list.
 */
export type ListGradient =
    { Same: Gradient } |
    { Alternating: Color[] };

/**
 * The ID of an image that can be used for looking up an image in an image
 * cache.
 */
export type ImageId = string;

/**
 * A constant that is part of the formula to calculate the sigma of a gaussian
 * blur for a background image. Check its documentation for a deeper
 * explanation.
 */
export const BLUR_FACTOR = 0.05;

export interface BackgroundImage {
    /** The image ID to look up the actual image in an image cache. */
    image: ImageId,
    /**
     * The brightness of the image in the range from `0` to `1`. This is for
     * darkening the image if it's too bright.
     */
    brightness: number,
    /**
     * The opacity of the image in the range from `0` to `1`. This is for making
     * the image more transparent.
     */
    opacity: number,
    /**
     * An additional gaussian blur that is applied to the image. It is in the
     * range from `0` to `1` and is meant to be multiplied with the larger of
     * the two dimensions of the image to ensure that the blur is independent of
     * the resolution of the image and then multiplied by `BLUR_FACTOR` to scale
     * it to a reasonable value. The resulting value is the sigma (standard
     * deviation) of the gaussian blur.
     *
     * sigma = BLUR_FACTOR * blur * max(width, height)
     */
    blur: number,
}

/** The background of a layout. */
export type LayoutBackground = Gradient | BackgroundImage;

/** Describes the Alignment of the Title in the Title Component. */
export type Alignment = "Auto" | "Left" | "Center";

/** The state object describes the information to visualize for the layout. */
export interface LayoutStateJson {
    /** The state objects for all of the components in the layout. */
    components: ComponentStateJson[],
    /** The direction which the components are laid out in. */
    direction: LayoutDirection,
    /**
     * The font to use for the timer text. `null` means a default font should be
     * used.
     */
    timer_font: Font | null,
    /**
     * The font to use for the times and other values. `null` means a default
     * font should be used.
     */
    times_font: Font | null,
    /**
     * The font to use for regular text. `null` means a default font should be
     * used.
     */
    text_font: Font | null,
    /** The background to show behind the layout. */
    background: Gradient,
    /** The color of thin separators. */
    thin_separators_color: Color,
    /** The color of normal separators. */
    separators_color: Color,
    /** The text color to use for text that doesn't specify its own color. */
    text_color: Color,
}

/**
 * Describes a Font to visualize text with. Depending on the platform a font
 * that matches the settings most closely is chosen. The settings may be ignored
 * entirely if the platform can't support different fonts such as in a terminal
 * for example.
 */
export interface Font {
    /**
     * The family name of the font to use. This corresponds with the
     * `Typographic Family Name` (Name ID 16) in the name table of the font. If
     * no such entry exists, the `Font Family Name` (Name ID 1) is to be used
     * instead. If there are multiple entries for the name, the english entry is
     * the one to choose. The subfamily is not specified at all and instead a
     * suitable subfamily is chosen based on the style, weight and stretch
     * values. https://docs.microsoft.com/en-us/typography/opentype/spec/name
     *
     * This is to ensure the highest portability across various platforms.
     * Platforms often select fonts very differently, so if necessary it is also
     * fine to store a different font identifier here at the cost of sacrificing
     * portability.
     */
    family: string,
    /** The style of the font to prefer selecting. */
    style: FontStyle,
    /** The weight of the font to prefer selecting. */
    weight: FontWeight,
    /** The stretch of the font to prefer selecting. */
    stretch: FontStretch,
}

/**
 * The style specifies whether to use a normal or italic version of a font. The
 * style may be emulated if no font dedicated to the style can be found.
 */
export type FontStyle = "normal" | "italic";

/**
 * The weight specifies the weight / boldness of a font. If there is no font
 * with the exact weight value, a font with a similar weight is to be chosen
 * based on an algorithm similar to this:
 * https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight#Fallback_weights
 */
export type FontWeight =
    "thin" |
    "extra-light" |
    "light" |
    "semi-light" |
    "normal" |
    "medium" |
    "semi-bold" |
    "bold" |
    "extra-bold" |
    "black" |
    "extra-black";

/**
 * The stretch specifies how wide a font should be. For example it may make
 * sense to reduce the stretch of a font to ensure split names are not cut off.
 * A font with a stretch value that is close is to be selected.
 * https://developer.mozilla.org/en-US/docs/Web/CSS/font-stretch#Font_face_selection
 */
export type FontStretch =
    "ultra-condensed" |
    "extra-condensed" |
    "condensed" |
    "semi-condensed" |
    "normal" |
    "semi-expanded" |
    "expanded" |
    "extra-expanded" |
    "ultra-expanded";

/**
 * A Timing Method describes which form of timing is used. This can either be
 * Real Time or Game Time.
 */
export enum TimingMethod {
    /**
     * Real Time is the unmodified timing that is as close to an atomic clock as
     * possible.
     */
    RealTime = 0,
    /**
     * Game Time describes the timing that is provided by the game that is being
     * run. This is entirely optional and may either be Real Time with loading
     * times removed or some time provided by the game.
     */
    GameTime = 1,
}

/**
 * Describes which phase the timer is currently in. This tells you if there's an
 * active speedrun attempt and whether it is paused or it ended.
 */
export enum TimerPhase {
    /** There's currently no active attempt. */
    NotRunning = 0,
    /** There's an active attempt that didn't end yet and isn't paused. */
    Running = 1,
    /** There's an attempt that already ended, but didn't get reset yet. */
    Ended = 2,
    /** There's an active attempt that is currently paused. */
    Paused = 3,
}

/** The state object describes the information to visualize for this component. */
export interface BlankSpaceComponentStateJson {
    /** The background shown behind the component. */
    background: Gradient,
    /** The size of the component. */
    size: number,
}

/** The state object describes the information to visualize for this component. */
export interface TimerComponentStateJson {
    /** The background shown behind the component. */
    background: Gradient,
    /** The time shown by the component without the fractional part. */
    time: string,
    /** The fractional part of the time shown (including the dot). */
    fraction: string,
    /** The semantic coloring information the time carries. */
    semantic_color: SemanticColor,
    /** The top color of the timer's gradient. */
    top_color: Color,
    /** The bottom color of the timer's gradient. */
    bottom_color: Color,
    /** The height of the timer. */
    height: number,
    /**
     * This value indicates whether the timer is currently frequently being
     * updated. This can be used for rendering optimizations.
     */
    updates_frequently: boolean,
}

/** The state object describes the information to visualize for this component. */
export interface TitleComponentStateJson {
    /** The background shown behind the component. */
    background: Gradient,
    /**
     * The color of the text. If `null` is specified, the color is taken from
     * the layout.
     */
    text_color: Color | null,
    /**
     * The game icon to show. The associated image can be looked up in the image
     * cache. The image may be the empty image. This indicates that there is no
     * icon.
     */
    icon: ImageId,
    /**
     * The first title line to show. This is either the game's name, or a
     * combination of the game's name and the category. This is a list of all
     * the possible abbreviations. It contains at least one element and the last
     * element is the unabbreviated value.
     */
    line1: string[],
    /**
     * By default the category name is shown on the second line. Based on the
     * settings, it can however instead be shown in a single line together with
     * the game name. This is a list of all the possible abbreviations. If this
     * is empty, only a single line is supposed to be shown. If it contains at
     * least one element, the last element is the unabbreviated value.
     */
    line2: string[],
    /**
     * Specifies whether the title should centered or aligned to the left
     * instead.
     */
    is_centered: boolean,
    /**
     * The amount of successfully finished attempts. If `null` is specified, the
     * amount of successfully finished attempts isn't supposed to be shown.
     */
    finished_runs: number | null,
    /**
     * The amount of total attempts. If `null` is specified, the amount of total
     * attempts isn't supposed to be shown.
     */
    attempts: number | null,
}

/** The state object describes the information to visualize for this component. */
export interface SplitsComponentStateJson {
    /** The background shown behind the splits. */
    background: ListGradient,
    /**
     * The column labels to visualize about the list of splits. If this is
     * `null`, no labels are supposed to be visualized. The list is specified
     * from right to left.
     */
    column_labels: string[] | null,
    /** The list of all the segments to visualize. */
    splits: SplitStateJson[],
    /**
     * Specifies whether the current run has any icons, even those that are not
     * currently visible by the splits component. This allows for properly
     * indenting the icon column, even when the icons are scrolled outside the
     * splits component.
     */
    has_icons: boolean,
    /**
     * Specifies whether thin separators should be shown between the individual
     * segments shown by the component.
     */
    show_thin_separators: boolean,
    /**
     * Describes whether a more pronounced separator should be shown in front of
     * the last segment provided.
     */
    show_final_separator: boolean,
    /**
     * Specifies whether to display each split as two rows, with the segment
     * name being in one row and the times being in the other.
     */
    display_two_rows: boolean,
    /**
     * The gradient to show behind the current segment as an indicator of it
     * being the current segment.
     */
    current_split_gradient: Gradient,
}

/** The state object that describes a single segment's information to visualize. */
export interface SplitStateJson {
    /**
     * The icon of the segment. The associated image can be looked up in the
     * image cache. The image may be the empty image. This indicates that there
     * is no icon.
     */
    icon: ImageId,
    /** The name of the segment. */
    name: string,
    /**
     * The state of each column from right to left. The amount of columns is
     * not guaranteed to be the same across different splits.
     */
    columns: SplitColumnState[],
    /**
     * Describes if this segment is the segment the active attempt is currently
     * on.
     */
    is_current_split: boolean,
    /**
     * The index of the segment based on all the segments of the run. This may
     * differ from the index of this `SplitStateJson` in the
     * `SplitsComponentStateJson` object, as there can be a scrolling window,
     * showing only a subset of segments. Each index is guaranteed to be unique.
     */
    index: number,
}

/** Describes the state of a single segment's column to visualize. */
export interface SplitColumnState {
    /** The value shown in the column. */
    value: string,
    /** The semantic coloring information the value carries. */
    semantic_color: SemanticColor,
    /** The visual color of the value. */
    visual_color: Color,
    /**
     * This value indicates whether the column is currently frequently being
     * updated. This can be used for rendering optimizations.
     */
    updates_frequently: boolean,
}

/**
 * The state object describes the information to visualize for a key value based
 * component.
 */
export interface KeyValueComponentStateJson {
    /** The background shown behind the component. */
    background: Gradient,
    /**
     * The color of the key. If `null` is specified, the color is taken from the
     * layout.
     */
    key_color: Color | null,
    /**
     * The color of the key. If `null` is specified, the color is taken from the
     * layout.
     */
    value_color: Color | null,
    /** The semantic coloring information the value carries. */
    semantic_color: SemanticColor,
    /** The key to visualize. */
    key: string,
    /** The value to visualize. */
    value: string,
    /**
     * Specifies additional abbreviations for the key that can be used instead
     * of the key, if there is not enough space to show the whole key.
     */
    key_abbreviations: string[],
    /**
     * Specifies whether to display the name of the component and its value in
     * two separate rows.
     */
    display_two_rows: boolean,
    /**
     * This value indicates whether the value is currently frequently being
     * updated. This can be used for rendering optimizations.
     */
    updates_frequently: boolean,
}

/**
 * The state object describes the information to visualize for this component.
 * All the coordinates are in the range 0..1.
 */
export interface GraphComponentStateJson {
    /**
     * All of the graph's points. Connect all of them to visualize the graph. If
     * the live delta is active, the last point is to be interpreted as a
     * preview of the next split that is about to happen. Use the partial fill
     * color to visualize the region beneath that graph segment.
     */
    points: GraphComponentStatePointJson[],
    /** Contains the y coordinates of all the horizontal grid lines. */
    horizontal_grid_lines: number[],
    /** Contains the x coordinates of all the vertical grid lines. */
    vertical_grid_lines: number[],
    /**
     * The y coordinate that separates the region that shows the times that are
     * ahead of the comparison and those that are behind.
     */
    middle: number,
    /**
     * If the live delta is active, the last point is to be interpreted as a
     * preview of the next split that is about to happen. Use the partial fill
     * color to visualize the region beneath that graph segment.
     */
    is_live_delta_active: boolean,
    /**
     * Describes whether the graph is flipped vertically. For visualizing the
     * graph, this usually doesn't need to be interpreted, as this information
     * is entirely encoded into the other variables.
     */
    is_flipped: boolean,
    /**
     * The background color to use for the top region of the graph. The top
     * region ends at the y coordinate of the middle.
     */
    top_background_color: Color,
    /**
     * The background color to use for the bottom region of the graph. The top
     * region begins at the y coordinate of the middle.
     */
    bottom_background_color: Color,
    /** The color of the grid lines on the graph. */
    grid_lines_color: Color,
    /** The color of the lines connecting all the graph's points. */
    graph_lines_color: Color,
    /**
     * The color of the polygon connecting all the graph's points. The partial
     * fill color is only used for live changes.
     */
    partial_fill_color: Color,
    /** The color of the polygon connecting all the graph's points. */
    complete_fill_color: Color,
    /**
     * The best segment color to use for coloring graph segments that achieved a
     * new best segment time.
     */
    best_segment_color: Color,
    /** The height of the graph. */
    height: number,
}

/** Describes a point on the graph to visualize. */
export interface GraphComponentStatePointJson {
    /** The x coordinate of the point. */
    x: number,
    /** The y coordinate of the point. */
    y: number,
    /**
     * Describes whether the segment this point is visualizing achieved a new
     * best segment time. Use the best segment color for it, in that case.
     */
    is_best_segment: boolean,
}

/** The state object describes the information to visualize for this component. */
export interface TextComponentStateJson {
    /** The background shown behind the component. */
    background: Gradient,
    /**
     * Specifies whether to display the left and right text is supposed to be
     * displayed as two rows.
     */
    display_two_rows: boolean,
    /**
     * The color of the left part of the split up text or the whole text if
     * it's not split up. If `None` is specified, the color is taken from the
     * layout.
     */
    left_center_color: Color,
    /**
     * The color of the right part of the split up text. This can be ignored if
     * the text is not split up. If `None` is specified, the color is taken
     * from the layout.
     */
    right_color: Color,
    /** The text to show for the component. */
    text: TextComponentStateText,
}

/** The text that is supposed to be shown. */
export type TextComponentStateText =
    { Center: string } |
    { Split: string[] };

/** The state object describes the information to visualize for this component. */
export interface DetailedTimerComponentStateJson {
    /** The background shown behind the component. */
    background: Gradient,
    /** The state of the attempt timer. */
    timer: TimerComponentStateJson,
    /** The state of the segment timer. */
    segment_timer: TimerComponentStateJson,
    /** The first comparison to visualize. */
    comparison1: DetailedTimerComponentComparisonStateJson | null,
    /** The second comparison to visualize. */
    comparison2: DetailedTimerComponentComparisonStateJson | null,
    /**
     * The name of the segment. This may be `null` if it's not supposed to be
     * visualized.
     */
    segment_name: string | null,
    /**
     * The icon of the segment. The associated image can be looked up in the
     * image cache. The image may be the empty image. This indicates that there
     * is no icon.
     */
    icon: ImageId,
    /**
     * The color of the segment name if it's shown. If `null` is specified, the
     * color is taken from the layout.
     */
    segment_name_color: Color | null,
    /**
     * The color of the comparison names if they are shown. If `null` is
     * specified, the color is taken from the layout.
     */
    comparison_names_color: Color | null,
    /**
     * The color of the comparison times if they are shown. If `null` is
     * specified, the color is taken from the layout.
     */
    comparison_times_color: Color | null,
}

/** The state object describing a comparison to visualize. */
export interface DetailedTimerComponentComparisonStateJson {
    /** The name of the comparison. */
    name: string,
    /** The time to show for the comparison. */
    time: string,
}

/**
 * Represents the current state of the Layout Editor in order to visualize it
 * properly.
 */
export interface LayoutEditorStateJson {
    /** The name of all the components in the layout. */
    components: string[],
    /** Describes which actions are currently available. */
    buttons: LayoutEditorButtonsJson,
    /** The index of the currently selected component. */
    selected_component: number,
    /**
     * A generic description of the settings available for the selected
     * component and their current values.
     */
    component_settings: SettingsDescriptionJson,
    /**
     * A generic description of the general settings available for the layout
     * and their current values.
     */
    general_settings: SettingsDescriptionJson,
}

/**
 * Describes which actions are currently available. Depending on how many
 * components exist and which one is selected, only some actions can be executed
 * successfully.
 */
export interface LayoutEditorButtonsJson {
    /**
     * Describes whether the currently selected component can be removed. If
     * there's only one component in the layout, it can't be removed.
     */
    can_remove: boolean,
    /**
     * Describes whether the currently selected component can be moved up. If
     * the first component is selected, it can't be moved.
     */
    can_move_up: boolean,
    /**
     * Describes whether the currently selected component can be moved down. If
     * the last component is selected, it can't be moved.
     */
    can_move_down: boolean,
}

/** A generic description of the settings available and their current values. */
export interface SettingsDescriptionJson {
    /**
     * All of the different settings that are available and their current
     * values.
     */
    fields: SettingsDescriptionFieldJson[],
}

/** A Field describes a single setting by its name and its current value. */
export interface SettingsDescriptionFieldJson {
    /** The name of the setting. */
    text: string,
    /** The current value of the setting. */
    value: SettingsDescriptionValueJson,
}

/**
 * Describes a setting's value. Such a value can be of a variety of different
 * types.
 */
export type SettingsDescriptionValueJson =
    { Bool: boolean } |
    { UInt: number } |
    { Int: number } |
    { String: string } |
    { OptionalString: string | null } |
    { Accuracy: AccuracyJson } |
    { DigitsFormat: DigitsFormatJson } |
    { OptionalTimingMethod: TimingMethodJson | null } |
    { Color: Color } |
    { OptionalColor: Color | null } |
    { Gradient: Gradient } |
    { ListGradient: ListGradient } |
    { Alignment: Alignment } |
    { ColumnKind: ColumnKind } |
    { ColumnStartWith: ColumnStartWith } |
    { ColumnUpdateWith: ColumnUpdateWith } |
    { ColumnUpdateTrigger: ColumnUpdateTrigger } |
    { Hotkey: string } |
    { LayoutDirection: LayoutDirection } |
    { Font: Font | null } |
    { DeltaGradient: DeltaGradient } |
    { LayoutBackground: LayoutBackground } |
    { CustomCombobox: CustomCombobox };

/** Describes the kind of a column. */
export type ColumnKind = "Time" | "Variable";

/** Represents the possible backgrounds for a timer. */
export type DeltaGradient = Gradient | "DeltaPlain" | "DeltaVertical" | "DeltaHorizontal";

/** Describes the direction the components of a layout are laid out in. */
export type LayoutDirection = "Vertical" | "Horizontal";

/**
 * A custom Combobox containing its current value and a list of possible
 * values.
 */
export interface CustomCombobox {
    value: string,
    list: string[],
    mandatory: boolean,
}

/**
 * Specifies the value a segment starts out with before it gets replaced
 * with the current attempt's information when splitting.
 */
export type ColumnStartWith =
    "Empty" |
    "ComparisonTime" |
    "ComparisonSegmentTime" |
    "PossibleTimeSave";

/**
 * Once a certain condition is met, which is usually being on the split or
 * already having completed the split, the time gets updated with the value
 * specified here.
 */
export type ColumnUpdateWith =
    "DontUpdate" |
    "SplitTime" |
    "Delta" |
    "DeltaWithFallback" |
    "SegmentTime" |
    "SegmentDelta" |
    "SegmentDeltaWithFallback";

/** Specifies when a column's value gets updated. */
export type ColumnUpdateTrigger =
    "OnStartingSegment" |
    "Contextual" |
    "OnEndingSegment";

/**
 * The Accuracy describes how many digits to show for the fractional part of a
 * time.
 */
export type AccuracyJson = "Seconds" | "Tenths" | "Hundredths" | "Milliseconds";

/**
 * A Timing Method describes which form of timing is used. This can either be
 * Real Time or Game Time.
 */
export type TimingMethodJson = "RealTime" | "GameTime";

/**
 * A Digits Format describes how many digits of a time to always shown. The
 * times are prefixed by zeros to fill up the remaining digits.
 */
export type DigitsFormatJson =
    "SingleDigitSeconds" |
    "DoubleDigitSeconds" |
    "SingleDigitMinutes" |
    "DoubleDigitMinutes" |
    "SingleDigitHours" |
    "DoubleDigitHours";

/**
 * Represents the current state of the Run Editor in order to visualize it
 * properly.
 */
export interface RunEditorStateJson {
    /**
     * The game icon of the run. The associated image can be looked up in the
     * image cache. The image may be the empty image. This indicates that there
     * is no icon.
     */
    icon: ImageId,
    /** The name of the game the Run is for. */
    game: string,
    /** The name of the category the Run is for. */
    category: string,
    /**
     * The timer offset specifies the time that the timer starts at when starting a
     * new attempt.
     */
    offset: string,
    /**
     * The number of times this Run has been attempted by the runner. This
     * is mostly just a visual number and has no effect on any history.
     */
    attempts: number,
    /**
     * The timing method that is currently selected to be visualized and
     * edited.
     */
    timing_method: TimingMethodJson,
    /** The state of all the segments. */
    segments: RunEditorRowJson[],
    /** The names of all the custom comparisons that exist for this Run. */
    comparison_names: string[],
    /** Describes which actions are currently available. */
    buttons: RunEditorButtonsJson,
    /**
     * Additional metadata of this Run, like the platform and region of the
     * game.
     */
    metadata: RunMetadataJson,
}

/**
 * The Run Metadata stores additional information about a run, like the
 * platform and region of the game. All of this information is optional.
 */
export interface RunMetadataJson {
    /**
     * The speedrun.com Run ID of the run. You need to ensure that the record
     * on speedrun.com matches up with the Personal Best of this run. This may
     * be empty if there's no association.
     */
    run_id: string,
    /**
     * The name of the platform this game is run on. This may be empty if it's
     * not specified.
     */
    platform_name: string,
    /**
     * Specifies whether this speedrun is done on an emulator. Keep in mind
     * that `false` may also mean that this information is simply not known.
     */
    uses_emulator: boolean,
    /**
     * The name of the region this game is from. This may be empty if it's not
     * specified.
     */
    region_name: string,
    /**
     * Stores all the speedrun.com variables. A variable is an arbitrary key
     * value pair storing additional information about the category. An example
     * of this may be whether Amiibos are used in this category.
     */
    speedrun_com_variables: { [key: string]: string | undefined },
    /**
     * Stores all the custom variables. A custom variable is a key value pair
     * storing additional information about a run. Unlike the speedrun.com
     * variables, these can be fully custom and don't need to correspond to
     * anything on speedrun.com. Permanent custom variables can be specified by
     * the runner. Additionally auto splitters or other sources may provide
     * temporary custom variables that are not stored in the splits files.
     */
    custom_variables: { [key: string]: CustomVariableJson | undefined },
}
/**
 * A custom variable is a key value pair storing additional information about a
 * run. Unlike the speedrun.com variables, these can be fully custom and don't
 * need to correspond to anything on speedrun.com. Permanent custom variables
 * can be specified by the runner. Additionally auto splitters or other sources
 * may provide temporary custom variables that are not stored in the splits
 * files.
 */
export interface CustomVariableJson {
    /**
     * The current value of the custom variable. This may be provided by the
     * runner in the run editor or it may be provided through other means such
     * as an auto splitter.
     */
    value: string,
    /**
     * States whether the variable is permanent. Temporary variables don't get
     * stored in splits files. They also don't get shown in the run editor.
     */
    is_permanent: boolean,
}

/**
 * Describes which actions are currently available. Depending on how many
 * segments exist and which ones are selected, only some actions can be
 * executed successfully.
 */
export interface RunEditorButtonsJson {
    /**
     * Describes whether the currently selected segments can be removed. If all
     * segments are selected, they can't be removed.
     */
    can_remove: boolean,
    /**
     * Describes whether the currently selected segments can be moved up. If
     * any one of the selected segments is the first segment, then they can't
     * be moved.
     */
    can_move_up: boolean,
    /**
     * Describes whether the currently selected segments can be moved down. If
     * any one of the selected segments is the last segment, then they can't be
     * moved.
     */
    can_move_down: boolean,
}

/** Describes the current state of a segment. */
export interface RunEditorRowJson {
    /**
     * The icon of the segment. The associated image can be looked up in the
     * image cache. The image may be the empty image. This indicates that there
     * is no icon.
     */
    icon: ImageId,
    /** The name of the segment. */
    name: string,
    /** The segment's split time for the active timing method. */
    split_time: string,
    /** The segment time for the active timing method. */
    segment_time: string,
    /** The best segment time for the active timing method. */
    best_segment_time: string,
    /**
     * All of the times of the custom comparison for the active timing method.
     * The order of these matches up with the order of the custom comparisons
     * provided by the Run Editor's State object.
     */
    comparison_times: string[],
    /** Describes the segment's selection state. */
    selected: "NotSelected" | "Selected" | "Active",
}

/**
 * A Semantic Color describes a color by some meaningful event that is
 * happening. This information can be visualized as a color, but can also be
 * interpreted in other ways by the consumer of this API.
 */
export type SemanticColor = "Default" |
    "AheadGainingTime" |
    "AheadLosingTime" |
    "BehindLosingTime" |
    "BehindGainingTime" |
    "BestSegment" |
    "NotRunning" |
    "Paused" |
    "PersonalBest";
