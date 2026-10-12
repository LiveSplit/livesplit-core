//! The detailed timer shares each row between a timer and companion text.
//! Measure first, place both rows, then draw: fitting one label while drawing
//! another would make the layout depend on the live text or rendering order.

mod layout;

use self::layout::{Bounds, Timer, fit_ratio, place_comparisons, place_rows};
use crate::{
    component::detailed_timer::State,
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
    segment_name_size_hints: Vec<CachedLabel<L>>,
    // Only the detailed timer needs the shortest format to reserve the
    // largest possible segment-name size beside the attempt timer.
    timer_minimum_size_hint: CachedLabel<L>,
    timer_bounds: CachedLabel<L>,
    comparison_text_bounds: CachedLabel<L>,
    comparison_number_bounds: CachedLabel<L>,
    comparison_dash: CachedLabel<L>,
    comparisons: [ComparisonCache<L>; 2],
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
    let left = render_icon(context, [width, height], component);
    let total_height = component.timer.height as f32 + component.segment_timer.height as f32;
    if total_height <= 0.0 || width <= left + PADDING || height <= 0.0 {
        return;
    }
    let top = height * component.timer.height as f32 / total_height;
    let dims = Dimensions {
        width,
        height,
        left,
        top,
        bottom: height - top,
    };
    if dims.top <= 0.0 || dims.bottom <= 0.0 {
        return;
    }

    context.measure_timer(NUMERIC_REFERENCE, &mut cache.timer_bounds, 1.0);
    let reference = Bounds::from(cache.timer_bounds.vertical_bounds(1.0));
    let mut lower = measure_lower_row(cache, context, component, reference, dims);
    let main = measure_main_timer(cache, context, component, reference, dims);
    let main = place_content(cache, context, component, main, &mut lower, dims);
    render_timers(cache, context, component, main, &lower, dims);
    render_name(
        cache,
        context,
        component,
        layout_state,
        main,
        dims,
        name_floor(&lower, dims),
    );
    render_comparisons(cache, context, component, layout_state, &lower, left);
}

fn place_content<A: ResourceAllocator>(
    cache: &mut Labels<A::Label>,
    context: &mut RenderContext<A>,
    component: &State,
    mut main: Timer,
    lower: &mut LowerRow,
    dims: Dimensions,
) -> Timer {
    let upper_row = reserve_names(
        cache,
        context,
        component,
        main,
        dims,
        name_floor(lower, dims),
    );
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
    main
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
        dims.left - PADDING
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

fn render_icon<A: ResourceAllocator>(
    context: &mut RenderContext<A>,
    [width, height]: [f32; 2],
    component: &State,
) -> f32 {
    let padding = vertical_padding(height);
    // Leave a drawable text area even in an unusually narrow component.
    let size = (height - 2.0 * padding).min((width - 3.0 * PADDING).max(0.0));
    if let Some(icon) = context.create_image(&component.icon) {
        context.render_image([PADDING, padding], [size, size], icon);
        BOTH_PADDINGS + size
    } else {
        PADDING
    }
}

fn measure_main_timer<A: ResourceAllocator>(
    cache: &mut Labels<A::Label>,
    context: &mut RenderContext<A>,
    component: &State,
    reference: Bounds,
    dims: Dimensions,
) -> Timer {
    let Dimensions {
        width,
        top: height,
        left,
        ..
    } = dims;
    cache
        .segment_name_size_hints
        .resize_with(component.segment_name_size_hints.len(), CachedLabel::new);
    let mut widest = 0.0_f32;
    for (name, label) in component
        .segment_name_size_hints
        .iter()
        .zip(&mut cache.segment_name_size_hints)
    {
        widest = widest.max(context.measure_text(name, label, 0.5 * height, None));
    }
    let hint_width = timer::measure(&mut cache.timer, context, height, &component.timer);
    let available = (width - left - PADDING).max(0.0);
    let fitted_width = hint_width.min(available);
    // All names, including future ones, participate. A short or empty current
    // name must not grow the timer again when the user splits.
    let extra = if widest > 0.0 {
        ((available - PADDING) / (widest + fitted_width)).clamp(MINIMUM_TIMER_SCALE, 1.0)
    } else {
        1.0
    };
    let font = height * fit_ratio(available, hint_width) * extra;
    Timer {
        font,
        baseline: 0.5 * height + 0.35 * font,
        reference,
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

fn measure_lower_row<A: ResourceAllocator>(
    cache: &mut Labels<A::Label>,
    context: &mut RenderContext<A>,
    component: &State,
    reference: Bounds,
    dims: Dimensions,
) -> LowerRow {
    let Dimensions {
        width,
        top,
        bottom: height,
        left,
        ..
    } = dims;
    let font = 0.5 * height;
    let ComparisonMeasurements {
        count,
        widths,
        ink,
        reserved: bounds,
    } = measure_comparisons(cache, context, component, font);
    let hint_width = timer::measure(
        &mut cache.segment_timer,
        context,
        1.0,
        &component.segment_timer,
    );
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
fn reserve_names<A: ResourceAllocator>(
    cache: &mut Labels<A::Label>,
    context: &mut RenderContext<A>,
    component: &State,
    main: Timer,
    dims: Dimensions,
    floor: f32,
) -> Bounds {
    let Dimensions {
        width,
        top: height,
        left,
        ..
    } = dims;
    let minimum_time_width = context.measure_timer(
        &component.timer_minimum_size_hint,
        &mut cache.timer_minimum_size_hint,
        1.0,
    );
    let minimum_width = (minimum_time_width
        + timer::measure_fraction_size_hint(&mut cache.timer, context, 1.0, &component.timer))
        * main.font;
    let available = (width - left - BOTH_PADDINGS - minimum_width).max(0.0);
    let configured = 0.5 * height;
    let mut row = main.bounds();
    for (name, label) in component
        .segment_name_size_hints
        .iter()
        .zip(&mut cache.segment_name_size_hints)
    {
        if name.is_empty() {
            continue;
        }
        let needed = context.measure_text(name, label, configured, None);
        let font = (configured * fit_ratio(available, needed)).max(floor);
        let baseline = main.baseline + 0.35 * (font - main.font);
        row = row.union(Bounds::from(label.vertical_bounds(font)).translated(baseline));
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
            [lower.value_right, lower.baselines[index]],
            lower.font,
            times_color,
        );
    }
}
