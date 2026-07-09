use crate::{Segment, platform::prelude::*, settings::Image};
use core::{iter::FusedIterator, slice};

/// A contiguous, non-overlapping group of segments.
///
/// The range is half-open: `start` is inclusive and `end` is exclusive. The
/// final segment in the range acts as the major split for the group.
/// Segment groups are intentionally one level deep. Nested groups are not
/// represented.
#[derive(Clone, Debug, PartialEq)]
pub struct SegmentGroup {
    start: usize,
    end: usize,
    name: Option<String>,
    icon: Image,
}

impl SegmentGroup {
    /// Creates a new segment group if the provided range contains at least one
    /// segment.
    pub fn new(start: usize, end: usize, name: Option<String>) -> Option<Self> {
        (end > start).then_some(Self {
            start,
            end,
            name: name.filter(|n| !n.is_empty()),
            icon: Image::EMPTY.clone(),
        })
    }

    /// Creates a group without validating the range. Call
    /// [`SegmentGroups::repair`] before exposing it.
    pub(crate) fn new_unchecked(start: usize, end: usize, name: Option<String>) -> Self {
        Self::new_unchecked_with_icon(start, end, name, Image::EMPTY.clone())
    }

    /// Creates a group without validating the range. Call
    /// [`SegmentGroups::repair`] before exposing it.
    pub(crate) fn new_unchecked_with_icon(
        start: usize,
        end: usize,
        name: Option<String>,
        icon: Image,
    ) -> Self {
        Self {
            start,
            end,
            name: name.filter(|n| !n.is_empty()),
            icon,
        }
    }

    /// Accesses the start index of the group.
    pub const fn start(&self) -> usize {
        self.start
    }

    /// Accesses the exclusive end index of the group.
    pub const fn end(&self) -> usize {
        self.end
    }

    /// Accesses the index of the group's major split.
    pub const fn major_index(&self) -> usize {
        self.end - 1
    }

    /// Accesses the explicit name of the group, if there is one.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Sets the explicit name of the group.
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name.filter(|n| !n.is_empty());
    }

    /// Accesses the explicit icon of the group, if there is one.
    pub fn icon(&self) -> Option<&Image> {
        (!self.icon.is_empty()).then_some(&self.icon)
    }

    /// Sets the explicit icon of the group.
    pub fn set_icon(&mut self, icon: Image) {
        self.icon = icon;
    }

    /// Removes the explicit icon of the group.
    pub fn remove_icon(&mut self) {
        self.icon = Image::EMPTY.clone();
    }
}

/// A validated list of one-level segment groups over a flat segment list.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SegmentGroups {
    groups: Vec<SegmentGroup>,
}

impl SegmentGroups {
    /// Creates an empty group list.
    pub const fn new() -> Self {
        Self { groups: Vec::new() }
    }

    /// Creates segment groups from possibly invalid ranges. Groups are sorted,
    /// clamped to the segment count, repaired to avoid overlaps, and invalid
    /// groups are dropped.
    pub fn from_vec_lossy(groups: Vec<SegmentGroup>, segment_count: usize) -> Self {
        let mut this = Self { groups };
        this.repair(segment_count);
        this
    }

    /// Accesses all groups.
    pub fn groups(&self) -> &[SegmentGroup] {
        &self.groups
    }

    /// Clears all groups.
    pub fn clear(&mut self) {
        self.groups.clear();
    }

    /// Adds a group if it can be represented after validation.
    pub fn push_lossy(
        &mut self,
        start: usize,
        end: usize,
        name: Option<String>,
        segment_count: usize,
    ) {
        if let Some(group) = SegmentGroup::new(start, end, name) {
            self.groups.push(group);
            self.repair(segment_count);
        }
    }

    /// Adds a group and repairs the group list.
    pub fn push_group_lossy(&mut self, group: SegmentGroup, segment_count: usize) {
        self.groups.push(group);
        self.repair(segment_count);
    }

    /// Replaces a group and repairs the group list.
    pub fn replace_lossy(
        &mut self,
        index: usize,
        group: SegmentGroup,
        segment_count: usize,
    ) -> bool {
        let Some(existing_group) = self.groups.get_mut(index) else {
            return false;
        };
        *existing_group = group;
        self.repair(segment_count);
        true
    }

    /// Replaces multiple groups and repairs the group list once afterwards.
    pub fn replace_many_lossy<I>(&mut self, groups: I, segment_count: usize)
    where
        I: IntoIterator<Item = (usize, SegmentGroup)>,
    {
        for (index, group) in groups {
            if let Some(existing_group) = self.groups.get_mut(index) {
                *existing_group = group;
            }
        }
        self.repair(segment_count);
    }

    /// Removes a group.
    pub fn remove(&mut self, index: usize) -> Option<SegmentGroup> {
        (index < self.groups.len()).then(|| self.groups.remove(index))
    }

    /// Sets the explicit name of a group.
    pub fn set_name(&mut self, index: usize, name: Option<String>) -> bool {
        let Some(group) = self.groups.get_mut(index) else {
            return false;
        };
        group.set_name(name);
        true
    }

    /// Sets the explicit icon of a group.
    pub fn set_icon(&mut self, index: usize, icon: Image) -> bool {
        let Some(group) = self.groups.get_mut(index) else {
            return false;
        };
        group.set_icon(icon);
        true
    }

    /// Removes the explicit icon of a group.
    pub fn remove_icon(&mut self, index: usize) -> bool {
        let Some(group) = self.groups.get_mut(index) else {
            return false;
        };
        group.remove_icon();
        true
    }

    /// Finds the group containing the provided segment index.
    pub fn group_index_for_segment(&self, index: usize) -> Option<usize> {
        self.groups
            .iter()
            .position(|group| index >= group.start && index < group.end)
    }

    /// Repairs all groups against the provided segment count.
    pub fn repair(&mut self, segment_count: usize) {
        self.groups.sort_unstable_by_key(|group| group.start);

        let mut min_start = 0;
        self.groups.retain_mut(|group| {
            group.start = group.start.max(min_start).min(segment_count);
            group.end = group.end.max(group.start).min(segment_count);
            let valid = group.end > group.start;
            if valid {
                min_start = group.end;
            }
            valid
        });
    }

    /// Updates groups after inserting a segment at `index`.
    pub fn segment_inserted(&mut self, index: usize, segment_count: usize) {
        for group in &mut self.groups {
            if index <= group.start {
                group.start += 1;
                group.end += 1;
            } else if index < group.end {
                group.end += 1;
            }
        }
        self.repair(segment_count);
    }

    /// Updates groups after removing a segment at `index`.
    pub fn segment_removed(&mut self, index: usize, segment_count: usize) {
        for group in &mut self.groups {
            if index < group.start {
                group.start -= 1;
                group.end -= 1;
            } else if index < group.end {
                group.end -= 1;
            }
        }
        self.repair(segment_count);
    }

    /// Updates groups after swapping two adjacent segments. Groups are ranges
    /// over the segment list, so swapping segments keeps the ranges intact.
    /// This lets segments move into and out of groups without deleting group
    /// metadata.
    pub fn adjacent_segments_swapped(&mut self, _first_index: usize, segment_count: usize) {
        self.repair(segment_count);
    }

    /// Iterates over the segment list as group views.
    pub fn iter_with<'groups, 'segments>(
        &'groups self,
        segments: &'segments [Segment],
    ) -> SegmentGroupsIter<'groups, 'segments> {
        SegmentGroupsIter {
            groups: &self.groups,
            segments,
            group_index: 0,
            segment_index: 0,
        }
    }
}

/// A view of either a native group or a single ungrouped segment.
#[derive(Clone, Copy, Debug)]
pub struct SegmentGroupView<'groups, 'segments> {
    group_index: Option<usize>,
    name: Option<&'groups str>,
    icon: Option<&'groups Image>,
    segments: &'segments [Segment],
    start_index: usize,
}

impl<'groups, 'segments> SegmentGroupView<'groups, 'segments> {
    /// The native group index, or `None` if this view is an ungrouped segment.
    pub const fn group_index(&self) -> Option<usize> {
        self.group_index
    }

    /// The explicit group name, if one exists.
    pub const fn name(&self) -> Option<&'groups str> {
        self.name
    }

    /// The explicit group icon, if one exists.
    pub const fn icon(&self) -> Option<&'groups Image> {
        self.icon
    }

    /// The group display name, falling back to the major segment name.
    pub fn name_or_default(&self) -> &str {
        self.name.unwrap_or_else(|| self.ending_segment().name())
    }

    /// The group display icon, falling back to the major segment icon.
    pub fn icon_or_default(&self) -> &Image {
        self.icon.unwrap_or_else(|| self.ending_segment().icon())
    }

    /// The segments included in this view.
    pub const fn segments(&self) -> &'segments [Segment] {
        self.segments
    }

    /// The segment that acts as the major split.
    pub const fn ending_segment(&self) -> &'segments Segment {
        self.segments.last().unwrap()
    }

    /// The flat start index.
    pub const fn start_index(&self) -> usize {
        self.start_index
    }

    /// The exclusive flat end index.
    pub const fn end_index(&self) -> usize {
        self.start_index + self.segments.len()
    }

    /// The flat index of the major split.
    pub const fn major_index(&self) -> usize {
        self.end_index() - 1
    }

    /// Whether this view contains the provided flat segment index.
    pub const fn contains(&self, index: usize) -> bool {
        index >= self.start_index() && index < self.end_index()
    }
}

/// Iterator over grouped and ungrouped segment views.
pub struct SegmentGroupsIter<'groups, 'segments> {
    groups: &'groups [SegmentGroup],
    segments: &'segments [Segment],
    group_index: usize,
    segment_index: usize,
}

impl<'groups, 'segments> Iterator for SegmentGroupsIter<'groups, 'segments> {
    type Item = SegmentGroupView<'groups, 'segments>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.segment_index >= self.segments.len() {
            return None;
        }

        if let Some(group) = self.groups.get(self.group_index)
            && group.start == self.segment_index
        {
            self.group_index += 1;
            self.segment_index = group.end;
            return Some(SegmentGroupView {
                group_index: Some(self.group_index - 1),
                name: group.name(),
                icon: group.icon(),
                segments: &self.segments[group.start..group.end],
                start_index: group.start,
            });
        }

        let start_index = self.segment_index;
        self.segment_index += 1;
        Some(SegmentGroupView {
            group_index: None,
            name: None,
            icon: None,
            segments: slice::from_ref(&self.segments[start_index]),
            start_index,
        })
    }
}

impl FusedIterator for SegmentGroupsIter<'_, '_> {}
