//! The detailed timer shares each row between a timer and companion text.
//! Measure first, place both rows, then draw: fitting one label while drawing
//! another would make the layout depend on the live text or rendering order.

mod layout;

use self::layout::{
    Bounds, Timer, centered_icon_capacity, fit_icon_width, fit_ratio, place_comparisons,
    place_rows, text_priority_icon_width,
};
use crate::{
    component::detailed_timer::{IconDisplayMode, State},
    layout::LayoutState,
    platform::prelude::*,
    rendering::{
        RenderContext,
        component::timer,
        consts::{
            BOTH_PADDINGS, DEFAULT_TEXT_DESCENT, DEFAULT_TEXT_SIZE, PADDING, vertical_padding,
        },
        font::CachedLabel,
        resource::ResourceAllocator,
        scene::Layer,
        solid,
    },
    timing::formatter::DASH,
};

/// Extra shrinking to accommodate names is limited to 25%. Width fitting of
/// the time hint happens first, so even an overrun always fits its own space.
const MINIMUM_TIMER_SCALE: f32 = 0.75;
// Measured in the timer and times fonts separately. These boxes determine
// vertical placement; the time hints still determine horizontal fitting.
const NUMERIC_REFERENCE: &str = "0123456789:−.—";
const TEXT_REFERENCE: &str = "Hgyp0123456789";

struct ComparisonCache<L> {
    name: CachedLabel<L>,
    time: CachedLabel<L>,
    time_size_hint: CachedLabel<L>,
}

impl<L> ComparisonCache<L> {
    const fn new() -> Self {
        Self {
            name: CachedLabel::new(),
            time: CachedLabel::new(),
            time_size_hint: CachedLabel::new(),
        }
    }
}

/// Keep the many measured labels out of the shared component-cache enum.
/// Otherwise every component would occupy as much memory as a detailed timer.
/// Allocate them once on the first render; construction remains allocation free.
pub struct Cache<L> {
    labels: Option<Box<Labels<L>>>,
}

impl<L> Cache<L> {
    pub const fn new() -> Self {
        Self { labels: None }
    }
}

struct Labels<L> {
    timer: timer::Cache<L>,
    segment_timer: timer::Cache<L>,
    segment_name: CachedLabel<L>,
    segment_name_unconstrained: CachedLabel<L>,
    segment_name_size_hints: Vec<NameCache<L>>,
    // Only the detailed timer needs the shortest format to reserve the
    // largest possible segment-name size beside the attempt timer.
    timer_minimum_size_hint: CachedLabel<L>,
    timer_bounds: CachedLabel<L>,
    comparison_text_bounds: CachedLabel<L>,
    comparison_number_bounds: CachedLabel<L>,
    comparison_dash: CachedLabel<L>,
    comparisons: [ComparisonCache<L>; 2],
}

/// Measure run-wide names once before fitting. Icon fitting can then explore
/// different widths without reshaping labels or consulting the current name.
struct NameCache<L> {
    label: CachedLabel<L>,
    width: f32,
    bounds: Bounds,
    has_icon: bool,
}

impl<L> NameCache<L> {
    const fn new() -> Self {
        Self {
            label: CachedLabel::new(),
            width: 0.0,
            bounds: Bounds {
                top: 0.0,
                bottom: 0.0,
            },
            has_icon: false,
        }
    }
}

struct Measurements {
    reference: Bounds,
    /// Expected width at the configured attempt-timer font size.
    main_hint_width: f32,
    /// Shortest time format, including its fraction, at unit font size.
    main_minimum_width: f32,
    /// Widest run name at its configured size (half the attempt row height).
    widest_name: f32,
    /// Expected segment-timer width at unit font size.
    segment_hint_width: f32,
    comparisons: ComparisonMeasurements,
}

struct Content {
    main: Timer,
    lower: LowerRow,
    divider: f32,
}

impl<L> Labels<L> {
    const fn new() -> Self {
        Self {
            timer: timer::Cache::new(),
            segment_timer: timer::Cache::new(),
            segment_name: CachedLabel::new(),
            segment_name_unconstrained: CachedLabel::new(),
            segment_name_size_hints: Vec::new(),
            timer_minimum_size_hint: CachedLabel::new(),
            timer_bounds: CachedLabel::new(),
            comparison_text_bounds: CachedLabel::new(),
            comparison_number_bounds: CachedLabel::new(),
            comparison_dash: CachedLabel::new(),
            comparisons: [ComparisonCache::new(), ComparisonCache::new()],
        }
    }
}

struct LowerRow {
    timer: Timer,
    font: f32,
    baselines: [f32; 2],
    /// Complete row bounds, including descenders reserved before placement.
    reserved: Bounds,
    /// Stable ink bounds: actual names plus the fixed numeric reference.
    ink: [Bounds; 2],
    count: usize,
    value_right: f32,
    padding: f32,
}

#[derive(Clone, Copy)]
struct ComparisonMeasurements {
    count: usize,
    widths: [f32; 2],
    ink: [Bounds; 2],
    reserved: [Bounds; 2],
}

#[derive(Clone, Copy)]
struct Dimensions {
    width: f32,
    height: f32,
    left: f32,
    /// The lower row reclaims the icon column when the icon only spans row one.
    comparison_left: f32,
    top: f32,
    bottom: f32,
}

pub(in crate::rendering) fn render<A: ResourceAllocator>(
    cache: &mut Cache<A::Label>,
    context: &mut RenderContext<A>,
    [width, height]: [f32; 2],
    component: &State,
    layout_state: &LayoutState,
) {
    let cache = cache.labels.get_or_insert_with(|| Box::new(Labels::new()));
    context.render_background([width, height], &component.background);
    let total_height = component.timer.height as f32 + component.segment_timer.height as f32;
    if total_height <= 0.0 || width <= PADDING || height <= 0.0 {
        return;
    }
    let top = height * component.timer.height as f32 / total_height;
    if top <= 0.0 || height - top <= 0.0 {
        return;
    }
    let dims = Dimensions {
        width,
        height,
        left: PADDING,
        comparison_left: PADDING,
        top,
        bottom: height - top,
    };
    let measured = measure_labels(cache, context, component, dims);
    let icon = (component.display_icon != IconDisplayMode::Hidden)
        .then(|| context.create_image(&component.icon))
        .flatten();
    // The selected mode reserves the icon column even before starting or on
    // a segment without an image. This is only a fitting reservation: absent
    // icons let the companion text move left without resizing either timer.
    let (dims, content, icon_bounds) = if component.display_icon != IconDisplayMode::Hidden {
        fit_icon(cache, &measured, dims, component.display_icon)
    } else {
        (
            dims,
            fit_content(cache, &measured, dims),
            Bounds {
                top: 0.0,
                bottom: 0.0,
            },
        )
    };
    if let Some(icon) = icon {
        let size = icon_bounds.height();
        context.render_image([PADDING, icon_bounds.top], [size, size], icon);
    }
    let Content { main, lower, .. } = content;
    render_timers(cache, context, component, main, &lower, dims);
    // Reclaim the empty column only while drawing the text. In both-row mode
    // the comparison names and values translate together; their fonts and
    // baselines, and the segment timer's origin, retain the reserved layout.
    // First-row mode already gives the comparisons the normal left margin.
    let missing_icon = component.icon.is_empty();
    let name_left = if missing_icon { PADDING } else { dims.left };
    let comparison_left = if missing_icon {
        PADDING
    } else {
        dims.comparison_left
    };
    render_name(
        cache,
        context,
        component,
        layout_state,
        main,
        Dimensions {
            left: name_left,
            ..dims
        },
        name_floor(&lower, dims),
    );
    render_comparisons(
        cache,
        context,
        component,
        layout_state,
        &lower,
        comparison_left,
        lower.value_right - (dims.comparison_left - comparison_left),
    );
}

/// Fit and place both rows without any font-engine work. The logical divider
/// protects the lower row and also bounds first-row icons with real padding.
fn fit_content<L>(cache: &Labels<L>, measured: &Measurements, dims: Dimensions) -> Content {
    let mut main = fit_main_timer(measured, dims);
    let mut lower = fit_lower_row(measured, dims);
    let upper_row = reserve_names(cache, measured, main, dims, name_floor(&lower, dims));
    let (shifts, divider) = place_rows(
        dims.height,
        main.bounds(),
        lower.timer.bounds(),
        upper_row,
        lower.reserved,
    );
    main.baseline += shifts[0];
    lower.timer.baseline += shifts[1];
    (lower.font, lower.baselines) = place_comparisons(
        &lower.ink[..lower.count],
        lower.font,
        lower.timer.bounds().center(),
        divider,
        dims.height,
        lower.padding,
    );
    Content {
        main,
        lower,
        divider,
    }
}

fn render_timers<A: ResourceAllocator>(
    cache: &mut Labels<A::Label>,
    context: &mut RenderContext<A>,
    component: &State,
    main: Timer,
    lower: &LowerRow,
    dims: Dimensions,
) {
    timer::render_fitted(
        &mut cache.timer,
        context,
        [dims.width, dims.top],
        &component.timer,
        main.font,
        main.baseline,
    );
    // Backgrounds retain their configured row allocation. Only the baseline
    // moves; it can lie outside the old row center under the three-gap layout.
    let origin = if lower.count > 0 {
        lower.value_right
    } else {
        dims.comparison_left - PADDING
    };
    context.translate(origin, dims.top);
    timer::render_fitted(
        &mut cache.segment_timer,
        context,
        [dims.width - origin, dims.bottom],
        &component.segment_timer,
        lower.timer.font,
        lower.timer.baseline - dims.top,
    );
    context.translate(-origin, -dims.top);
}

/// Text has priority, but an enabled icon always keeps a small responsive
/// share. The entire fit uses the expected timer formats and every run name,
/// so starting, ticking, or splitting cannot resize or move the icon.
fn fit_icon<L>(
    cache: &Labels<L>,
    measured: &Measurements,
    dims: Dimensions,
    mode: IconDisplayMode,
) -> (Dimensions, Content, Bounds) {
    let first_row = mode == IconDisplayMode::FirstRow;
    let height = if first_row { dims.top } else { dims.height };
    let padding = vertical_padding(height);
    let preferred = (height - 2.0 * padding)
        .min((dims.width - 3.0 * PADDING).max(0.0))
        .max(0.0);
    let available = (dims.width - BOTH_PADDINGS).max(0.0);
    let timer_reservation = measured.main_hint_width.min(available) * MINIMUM_TIMER_SCALE;
    // Names may shrink as far as the comparisons. Reserve the widest name at
    // that configured comparison size, capped at the name's own normal size.
    let name_reservation = measured.widest_name * (dims.bottom / dims.top).min(1.0);
    let width_limit = text_priority_icon_width(
        preferred,
        available,
        timer_reservation,
        name_reservation,
        BOTH_PADDINGS,
    );
    let fit = |size| {
        let left = BOTH_PADDINGS + size;
        let fitted_dims = Dimensions {
            left,
            comparison_left: if first_row { PADDING } else { left },
            ..dims
        };
        let content = fit_content(cache, measured, fitted_dims);
        let (center, divider) = if first_row {
            (
                content.main.bounds().center(),
                content.divider.min(dims.height),
            )
        } else {
            (0.5 * dims.height, dims.height)
        };
        (
            fitted_dims,
            content,
            center,
            centered_icon_capacity(center, divider, padding),
        )
    };
    // All iterations use the same measured reservations; no labels are
    // reshaped or allocated while solving the width/divider dependency.
    let size = fit_icon_width(width_limit, |size| fit(size).3);
    let (dims, content, center, _) = fit(size);
    (
        dims,
        content,
        Bounds {
            top: center - 0.5 * size,
            bottom: center + 0.5 * size,
        },
    )
}

/// All measurements use hints or fixed glyph references. Live times and the
/// current segment name are used only when drawing, never to place the icon.
fn measure_labels<A: ResourceAllocator>(
    cache: &mut Labels<A::Label>,
    context: &mut RenderContext<A>,
    component: &State,
    dims: Dimensions,
) -> Measurements {
    context.measure_timer(NUMERIC_REFERENCE, &mut cache.timer_bounds, 1.0);
    let reference = Bounds::from(cache.timer_bounds.vertical_bounds(1.0));
    let comparisons = measure_comparisons(cache, context, component, 0.5 * dims.bottom);
    let segment_hint_width = timer::measure(
        &mut cache.segment_timer,
        context,
        1.0,
        &component.segment_timer,
    );
    cache
        .segment_name_size_hints
        .resize_with(component.segment_name_size_hints.len(), NameCache::new);
    let mut widest_name = 0.0_f32;
    for (hint, cached) in component
        .segment_name_size_hints
        .iter()
        .zip(&mut cache.segment_name_size_hints)
    {
        cached.width = context.measure_text(&hint.name, &mut cached.label, 0.5 * dims.top, None);
        cached.bounds = cached.label.vertical_bounds(1.0).into();
        cached.has_icon = hint.has_icon;
        widest_name = widest_name.max(cached.width);
    }
    let main_hint_width = timer::measure(&mut cache.timer, context, dims.top, &component.timer);
    let main_minimum_width =
        context.measure_timer(
            &component.timer_minimum_size_hint,
            &mut cache.timer_minimum_size_hint,
            1.0,
        ) + timer::measure_fraction_size_hint(&mut cache.timer, context, 1.0, &component.timer);
    Measurements {
        reference,
        main_hint_width,
        main_minimum_width,
        widest_name,
        segment_hint_width,
        comparisons,
    }
}

fn fit_main_timer(measured: &Measurements, dims: Dimensions) -> Timer {
    let available = (dims.width - dims.left - PADDING).max(0.0);
    let fitted_width = measured.main_hint_width.min(available);
    // All names, including future ones, participate. A short or empty current
    // name must not grow the timer again when the user splits.
    let extra = if measured.widest_name > 0.0 {
        ((available - PADDING) / (measured.widest_name + fitted_width))
            .clamp(MINIMUM_TIMER_SCALE, 1.0)
    } else {
        1.0
    };
    let font = dims.top * fit_ratio(available, measured.main_hint_width) * extra;
    Timer {
        font,
        baseline: 0.5 * dims.top + 0.35 * font,
        reference: measured.reference,
    }
}

fn measure_comparisons<A: ResourceAllocator>(
    cache: &mut Labels<A::Label>,
    context: &mut RenderContext<A>,
    component: &State,
    font: f32,
) -> ComparisonMeasurements {
    context.measure_text(TEXT_REFERENCE, &mut cache.comparison_text_bounds, 1.0, None);
    context.measure_numbers(NUMERIC_REFERENCE, &mut cache.comparison_number_bounds, 1.0);
    let numbers = Bounds::from(cache.comparison_number_bounds.vertical_bounds(1.0));
    let mut reference =
        Bounds::from(cache.comparison_text_bounds.vertical_bounds(1.0)).union(numbers);
    reference.bottom = reference
        .bottom
        .max(DEFAULT_TEXT_DESCENT / DEFAULT_TEXT_SIZE);
    let mut widths = [0.0_f32; 2];
    let mut ink = [numbers; 2];
    let mut reserved = [reference; 2];
    let mut count = 0;
    // Visibility and current values must not affect the reserved lower row.
    // The hints include every segment and stay available before an attempt.
    let dash_width = context.measure_numbers(DASH, &mut cache.comparison_dash, font);
    for (comparison, labels) in component
        .comparison_size_hints
        .iter()
        .flatten()
        .zip(&mut cache.comparisons)
    {
        widths[0] =
            widths[0].max(context.measure_text(&comparison.name, &mut labels.name, font, None));
        widths[1] = widths[1].max(dash_width).max(context.measure_numbers(
            &comparison.time,
            &mut labels.time_size_hint,
            font,
        ));
        // A missing value (dash) and any digits have one stable numeric box.
        // Empty names must not introduce an artificial baseline into that box.
        if !comparison.name.is_empty() {
            ink[count] = ink[count].union(labels.name.vertical_bounds(1.0).into());
        }
        reserved[count] = reference.union(ink[count]);
        count += 1;
    }
    ComparisonMeasurements {
        count,
        widths,
        ink,
        reserved,
    }
}

fn fit_lower_row(measured: &Measurements, dims: Dimensions) -> LowerRow {
    let Dimensions {
        width,
        top,
        bottom: height,
        comparison_left: left,
        ..
    } = dims;
    let font = 0.5 * height;
    let ComparisonMeasurements {
        count,
        widths,
        ink,
        reserved: bounds,
    } = measured.comparisons;
    let hint_width = measured.segment_hint_width;
    let reference = measured.reference;
    let leading = if count == 2 {
        font * 1.1_f32.max(bounds[0].bottom - bounds[1].top + 0.1)
    } else {
        0.0
    };
    let span = if count > 0 {
        -bounds[0].top * font + leading
    } else {
        0.0
    };
    // Initially size two comparison lines to span the timer's reference cap
    // height. Their descenders reserve additional room below its baseline.
    let timer_font = if count == 2 && reference.top < 0.0 {
        span / -reference.top
    } else {
        height
    };
    let block = reference.scaled(timer_font);
    let block = if count > 0 {
        block.union(Bounds {
            top: -span,
            bottom: bounds[count - 1].bottom * font,
        })
    } else {
        block
    };
    let padding = vertical_padding(height / count.max(1) as f32);
    let available = (width - left - PADDING - if count > 0 { BOTH_PADDINGS } else { 0.0 }).max(0.0);
    let needed = widths[0] + widths[1] + hint_width * timer_font;
    // Shrink the shared lower row only when it needs the room. No comparison
    // ellipsis is used: scaling is enough, and avoids rounding-induced flicker.
    let fit = fit_ratio(available, needed).min(fit_ratio(height - 2.0 * padding, block.height()));
    let baseline = top + 0.5 * (height - block.height() * fit) - block.top * fit;
    LowerRow {
        timer: Timer {
            font: timer_font * fit,
            baseline,
            reference,
        },
        font: font * fit,
        baselines: [0.0; 2],
        reserved: block.scaled(fit).translated(baseline),
        ink,
        count,
        value_right: left
            + widths[0] * fit
            + if count > 0 {
                PADDING + widths[1] * fit
            } else {
                0.0
            },
        padding,
    }
}

fn name_floor(lower: &LowerRow, dims: Dimensions) -> f32 {
    (if lower.count > 0 {
        lower.font
    } else {
        0.5 * dims.bottom
    })
    .min(0.5 * dims.top)
}

/// Reserve every name at its largest reachable fitted font. The shortest
/// configured time format leaves the most room, so this reservation is stable
/// across splits and digit changes without reserving impossible name sizes.
fn reserve_names<L>(
    cache: &Labels<L>,
    measured: &Measurements,
    main: Timer,
    dims: Dimensions,
    floor: f32,
) -> Bounds {
    let Dimensions {
        width, top: height, ..
    } = dims;
    let minimum_width = measured.main_minimum_width * main.font;
    let configured = 0.5 * height;
    let mut row = main.bounds();
    for cached in &cache.segment_name_size_hints {
        if cached.bounds.height() <= 0.0 {
            continue;
        }
        // A missing-icon segment can display its name at a larger font after
        // reclaiming the column. Reserve that reachable size for the entire
        // run, so the larger name never changes the divider at a split.
        let left = if cached.has_icon { dims.left } else { PADDING };
        let available = (width - left - BOTH_PADDINGS - minimum_width).max(0.0);
        let font = (configured * fit_ratio(available, cached.width)).max(floor);
        let baseline = main.baseline + 0.35 * (font - main.font);
        row = row.union(cached.bounds.scaled(font).translated(baseline));
    }
    row
}

fn render_name<A: ResourceAllocator>(
    cache: &mut Labels<A::Label>,
    context: &mut RenderContext<A>,
    component: &State,
    layout_state: &LayoutState,
    main: Timer,
    dims: Dimensions,
    floor: f32,
) {
    let Dimensions {
        width,
        top: height,
        left,
        ..
    } = dims;
    let Some(name) = &component.segment_name else {
        return;
    };
    let actual_width =
        timer::measure_current(&mut cache.timer, context, &component.timer) * main.font;
    let end = width - BOTH_PADDINGS - actual_width;
    let available = end - left;
    if available <= 0.0 {
        return;
    }
    let configured = 0.5 * height;
    let needed = context.measure_text(
        name,
        &mut cache.segment_name_unconstrained,
        configured,
        None,
    );
    let fitted = configured * fit_ratio(available, needed);
    let font = fitted.max(floor);
    let position = [left, main.baseline + 0.35 * (font - main.font)];
    let color = solid(
        &component
            .segment_name_color
            .unwrap_or(layout_state.text_color),
    );
    if fitted >= floor {
        // Never truncate text that already fits by scaling. Reapplying a
        // width constraint can flicker into an ellipsis at fractional widths.
        context.render_text(
            name,
            &mut cache.segment_name_unconstrained,
            position,
            font,
            color,
        );
    } else if context.measure_text(name, &mut cache.segment_name, font, Some(available))
        <= available
    {
        context.render_text_ellipsis(name, &mut cache.segment_name, position, font, color, end);
    }
    // If even the ellipsis is too wide at the floor, omit the name.
}

fn render_comparisons<A: ResourceAllocator>(
    cache: &mut Labels<A::Label>,
    context: &mut RenderContext<A>,
    component: &State,
    layout_state: &LayoutState,
    lower: &LowerRow,
    left: f32,
    value_right: f32,
) {
    if lower.font <= 0.0 {
        return;
    }
    let names_color = solid(
        &component
            .comparison_names_color
            .unwrap_or(layout_state.text_color),
    );
    let times_color = solid(
        &component
            .comparison_times_color
            .unwrap_or(layout_state.text_color),
    );
    // Keep the value column's right edge from the horizontal fit. Further
    // vertical shrinking only adds room; it must not resize the segment timer.
    for (index, (comparison, labels)) in [&component.comparison1, &component.comparison2]
        .into_iter()
        .flatten()
        .zip(&mut cache.comparisons)
        .enumerate()
    {
        context.render_text(
            &comparison.name,
            &mut labels.name,
            [left, lower.baselines[index]],
            lower.font,
            names_color,
        );
    }
    for (index, (comparison, labels)) in [&component.comparison1, &component.comparison2]
        .into_iter()
        .flatten()
        .zip(&mut cache.comparisons)
        .enumerate()
    {
        context.render_numbers(
            &comparison.time,
            &mut labels.time,
            Layer::Bottom,
            [value_right, lower.baselines[index]],
            lower.font,
            times_color,
        );
    }
}
