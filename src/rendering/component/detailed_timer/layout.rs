//! Vertical layout uses glyph bounds rather than line boxes. Independently
//! centering two line boxes would leave twice as much space between the timers
//! as at the outer edges. Instead, the timer reference boxes get three equal
//! gaps, with companion text kept inside the component and below its divider.

#[derive(Clone, Copy, Debug)]
pub(super) struct Bounds {
    pub top: f32,
    pub bottom: f32,
}

impl From<[f32; 2]> for Bounds {
    fn from([top, bottom]: [f32; 2]) -> Self {
        Self { top, bottom }
    }
}

impl Bounds {
    pub const fn height(self) -> f32 {
        self.bottom - self.top
    }

    pub const fn center(self) -> f32 {
        0.5 * (self.top + self.bottom)
    }

    pub const fn scaled(self, scale: f32) -> Self {
        Self {
            top: self.top * scale,
            bottom: self.bottom * scale,
        }
    }

    pub const fn translated(self, offset: f32) -> Self {
        Self {
            top: self.top + offset,
            bottom: self.bottom + offset,
        }
    }

    pub const fn union(self, other: Self) -> Self {
        Self {
            top: self.top.min(other.top),
            bottom: self.bottom.max(other.bottom),
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct Timer {
    pub font: f32,
    pub baseline: f32,
    /// A fixed box containing all timer digits and punctuation. Using the
    /// current glyphs here would make both timers and comparisons jump as the
    /// digits change, even when the hint and font size stay the same.
    pub reference: Bounds,
}

impl Timer {
    pub const fn bounds(self) -> Bounds {
        self.reference.scaled(self.font).translated(self.baseline)
    }
}

/// Position the timers with three equal gaps, then move their complete rows
/// only as far as necessary to protect accents, descenders, and reserved names.
/// Returns the group offsets and a logical divider that comparisons may not
/// cross. Existing configured heights determine font sizes, not this divider.
pub(super) fn place_rows(
    height: f32,
    upper_timer: Bounds,
    lower_timer: Bounds,
    upper_row: Bounds,
    lower_row: Bounds,
) -> ([f32; 2], f32) {
    let gap = ((height - upper_timer.height() - lower_timer.height()) / 3.0).max(0.0);
    let upper_shift = gap - upper_timer.top;
    let lower_shift = 2.0 * gap + upper_timer.height() - lower_timer.top;
    let mut upper_top = (upper_row.top + upper_shift).max(0.0);
    let mut lower_top = (lower_row.top + lower_shift).min(height - lower_row.height());

    if lower_top < upper_top + upper_row.height() {
        let middle = 0.5 * (upper_top + upper_row.height() + lower_top);
        upper_top = middle - upper_row.height();
        lower_top = middle;
    }
    if upper_top < 0.0 {
        lower_top -= upper_top;
        upper_top = 0.0;
    }
    if lower_top + lower_row.height() > height {
        let overflow = lower_top + lower_row.height() - height;
        upper_top -= overflow;
        lower_top -= overflow;
    }

    let upper_bottom = upper_top + upper_row.height();
    let divider = upper_bottom.max(0.5 * (upper_bottom + lower_top)).max(0.0);
    (
        [upper_top - upper_row.top, lower_top - lower_row.top],
        divider,
    )
}

/// Expand comparison spacing around the fixed segment-timer center. The two
/// outer gaps and the middle gap are equal within the largest symmetric region
/// below the divider. Its midpoint stays at the timer center even when the
/// available space above and below differs.
///
/// If a line cannot fit on its side, shrink only the comparisons. Repositioning
/// or resizing the timers here would defeat their stable reference layout.
pub(super) fn place_comparisons(
    bounds: &[Bounds],
    mut font: f32,
    center: f32,
    divider: f32,
    height: f32,
    padding: f32,
) -> (f32, [f32; 2]) {
    let above = (center - divider).max(0.0);
    let below = (height - center).max(0.0);
    let mut baselines = [0.0; 2];
    match bounds {
        [first, second] => {
            // Half of the middle gap plus one outer gap consumes 1.5 gaps
            // on either side of the fixed center.
            let fit = fit_ratio(above - 1.5 * padding, first.height() * font)
                .min(fit_ratio(below - 1.5 * padding, second.height() * font));
            font *= fit;
            let gap = ((above - first.height() * font) / 1.5)
                .min((below - second.height() * font) / 1.5)
                .max(0.0);
            baselines[0] = center - 0.5 * gap - first.bottom * font;
            baselines[1] = center + 0.5 * gap - second.top * font;
        }
        [line] => {
            font *= fit_ratio(above.min(below) - padding, 0.5 * line.height() * font);
            baselines[0] = center - line.center() * font;
        }
        _ => {}
    }
    (font, baselines)
}

pub(super) fn fit_ratio(available: f32, needed: f32) -> f32 {
    if needed > 0.0 {
        (available / needed).clamp(0.0, 1.0)
    } else {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 0.0001, "{actual} != {expected}");
    }

    #[test]
    fn timer_boxes_have_three_equal_gaps() {
        let upper = Bounds::from([10.0, 40.0]);
        let lower = Bounds::from([70.0, 90.0]);
        let (shifts, divider) = place_rows(110.0, upper, lower, upper, lower);
        let upper = upper.translated(shifts[0]);
        let lower = lower.translated(shifts[1]);
        near(upper.top, 20.0);
        near(lower.top - upper.bottom, 20.0);
        near(110.0 - lower.bottom, 20.0);
        near(divider, 60.0);
    }

    #[test]
    fn complete_rows_are_protected_when_descenders_limit_timer_gaps() {
        let upper_row = Bounds::from([5.0, 35.0]);
        let lower_row = Bounds::from([55.0, 125.0]);
        let (shifts, divider) = place_rows(
            120.0,
            [10.0, 30.0].into(),
            [60.0, 80.0].into(),
            upper_row,
            lower_row,
        );
        let upper = upper_row.translated(shifts[0]);
        let lower = lower_row.translated(shifts[1]);
        assert!(upper.top >= 0.0);
        assert!(upper.bottom <= lower.top);
        near(lower.bottom, 120.0);
        assert!(divider >= upper.bottom && divider <= lower.top);
    }

    #[test]
    fn centered_comparison_gap_stays_centered_in_an_asymmetric_region() {
        let lines = [Bounds::from([-0.9, 0.3]), Bounds::from([-0.7, 0.2])];
        let (font, baselines) = place_comparisons(&lines, 20.0, 80.0, 45.0, 120.0, 2.0);
        near(font, 20.0);
        let first = lines[0].scaled(font).translated(baselines[0]);
        let second = lines[1].scaled(font).translated(baselines[1]);
        near(0.5 * (first.bottom + second.top), 80.0);
        let gap = second.top - first.bottom;
        near(first.top - gap, 45.0);
        assert!(second.bottom + gap <= 120.0);
        assert!(gap >= 2.0);
    }

    #[test]
    fn comparisons_shrink_to_fit_their_own_sides_with_padding() {
        let lines = [Bounds::from([-1.1, 0.4]), Bounds::from([-0.8, 0.3])];
        let (font, baselines) = place_comparisons(&lines, 30.0, 70.0, 50.0, 100.0, 2.0);
        assert!(font < 30.0);
        let first = lines[0].scaled(font).translated(baselines[0]);
        let second = lines[1].scaled(font).translated(baselines[1]);
        near(0.5 * (first.bottom + second.top), 70.0);
        assert!(first.top >= 52.0 - 0.0001);
        assert!(second.bottom <= 98.0 + 0.0001);
        assert!(second.top - first.bottom >= 2.0 - 0.0001);
    }

    #[test]
    fn single_comparison_centers_its_full_ink_including_descenders() {
        let line = Bounds::from([-0.8, 0.3]);
        let (font, baselines) = place_comparisons(&[line], 30.0, 70.0, 60.0, 110.0, 2.0);
        assert!(font < 30.0);
        let placed = line.scaled(font).translated(baselines[0]);
        near(placed.center(), 70.0);
        assert!(placed.top >= 62.0 - 0.0001);
        assert!(placed.bottom <= 108.0 + 0.0001);
    }

    #[test]
    fn no_comparisons_and_empty_available_space_are_finite() {
        let (font, baselines) = place_comparisons(&[], 20.0, 50.0, 40.0, 100.0, 2.0);
        near(font, 20.0);
        assert_eq!(baselines, [0.0; 2]);
        let lines = [Bounds::from([-0.8, 0.2]); 2];
        let (font, baselines) = place_comparisons(&lines, 20.0, 50.0, 50.0, 50.0, 2.0);
        near(font, 0.0);
        assert_eq!(baselines, [50.0; 2]);
        near(fit_ratio(0.0, 0.0), 1.0);
        near(fit_ratio(-1.0, 2.0), 0.0);
    }
}
