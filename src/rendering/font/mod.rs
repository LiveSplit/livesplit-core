mod cache;

use super::{
    Handle, Label, Pos, Transform,
    resource::{LabelHandle, ResourceAllocator},
};
use crate::{platform::prelude::*, util::ClearVec};

pub use self::cache::FontCache;

/// The default font to use for the text and the times.
pub const TEXT_FONT: &[u8] = include_bytes!("assets/FiraSans-Regular.ttf");
/// The default font to use for the timer.
pub const TIMER_FONT: &[u8] = include_bytes!("assets/Timer.ttf");

pub struct AbbreviatedLabel<L> {
    abbreviations: ClearVec<String>,
    max_width: f32,
    chosen: String,
    label: CachedLabel<L>,
}

impl<L> AbbreviatedLabel<L> {
    pub const fn new() -> Self {
        Self {
            abbreviations: ClearVec::new(),
            max_width: 0.0,
            chosen: String::new(),
            label: CachedLabel::new(),
        }
    }

    pub fn update<'a, F, R: ResourceAllocator<Label = LabelHandle<L>, Font = Handle<F>>>(
        &mut self,
        abbreviations: impl IntoIterator<Item = &'a str> + Clone,
        allocator: &mut R,
        font: &mut R::Font,
        max_width: f32,
    ) -> &mut LabelHandle<L>
    where
        L: Label,
    {
        if self.abbreviations.iter().ne(abbreviations.clone())
            || self.max_width.to_bits() != max_width.to_bits()
        {
            self.max_width = max_width;

            self.abbreviations.clear();
            for abbreviation in abbreviations {
                self.abbreviations.push().push_str(abbreviation);
            }

            let mut abbreviations = self.abbreviations.iter().map(|s| s.as_str());
            let abbreviation = abbreviations.next().unwrap_or("");
            let width = self
                .label
                .update(abbreviation, allocator, font, None)
                .width(1.0);
            let (mut total_longest, mut total_longest_width) = (abbreviation, width);
            let (mut within_longest, mut within_longest_width) = if width <= max_width {
                (abbreviation, width)
            } else {
                ("", 0.0)
            };

            for abbreviation in abbreviations {
                let width = self
                    .label
                    .update(abbreviation, allocator, font, None)
                    .width(1.0);
                if width <= max_width && width > within_longest_width {
                    within_longest_width = width;
                    within_longest = abbreviation;
                }
                if width > total_longest_width {
                    total_longest_width = width;
                    total_longest = abbreviation;
                }
            }

            let chosen = if within_longest.is_empty() {
                total_longest
            } else {
                within_longest
            };

            self.chosen.clear();
            self.chosen.push_str(chosen);
        }

        self.label
            .update(&self.chosen, allocator, font, Some(max_width))
    }
}

pub struct CachedLabel<L> {
    value: String,
    max_width: Option<f32>,
    label: Option<LabelHandle<L>>,
    font_id: usize,
}

impl<L> CachedLabel<L> {
    pub const fn new() -> Self {
        Self {
            value: String::new(),
            max_width: None,
            label: None,
            font_id: 0,
        }
    }

    pub fn update<F, R: ResourceAllocator<Label = LabelHandle<L>, Font = Handle<F>>>(
        &mut self,
        value: &str,
        allocator: &mut R,
        font: &mut R::Font,
        max_width: Option<f32>,
    ) -> &mut LabelHandle<L>
    where
        L: Label,
    {
        let is_definitely_dirty = self.value != value || self.font_id != font.id;
        let is_dirty = if is_definitely_dirty || self.max_width != max_width {
            let is_dirty = is_definitely_dirty
                || self
                    .label
                    .as_ref()
                    .zip(max_width)
                    .zip(self.max_width)
                    .is_none_or(|((l, max_width), old_max_width)| {
                        // If both the current and old max width fall outside the
                        // range where the label would need to use ellipsis, then we
                        // can consider the label to not be dirty.
                        let width_without_max_width = l.width_without_max_width(1.0);
                        old_max_width < width_without_max_width
                            || max_width < width_without_max_width
                    });

            self.value.clear();
            self.value.push_str(value);
            self.max_width = max_width;
            self.font_id = font.id;

            is_dirty
        } else {
            self.label.is_none()
        };

        if is_dirty {
            let text = self.value.trim();

            if let Some(label) = &mut self.label {
                allocator.update_label(label, text, font, max_width);
            } else {
                self.label = Some(allocator.create_label(&self.value, font, max_width));
            }
        }

        self.label.as_mut().unwrap()
    }
}

pub fn left_aligned(transform: &Transform, [x, y]: Pos, scale: f32) -> Transform {
    transform.pre_translate(x, y).pre_scale(scale, scale)
}

pub fn right_aligned(transform: &Transform, [x, y]: Pos, scale: f32, width: f32) -> Transform {
    transform
        .pre_translate(x - width, y)
        .pre_scale(scale, scale)
}

pub fn centered(
    transform: &Transform,
    [x, y]: Pos,
    scale: f32,
    width: f32,
    min_x: f32,
    max_x: f32,
) -> Transform {
    let mut x = x - 0.5 * width;
    if x < min_x {
        x = min_x;
    } else if x + width > max_x {
        x = max_x - width;
    }
    transform.pre_translate(x, y).pre_scale(scale, scale)
}
