use {
    crate::Segment,
    std::{
        iter::Peekable,
        slice::{self, Iter},
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct SegmentGroup {
    start: usize,
    /// exclusive
    end: usize,
    name: Option<String>,
}

impl SegmentGroup {
    pub fn new(start: usize, end: usize, name: Option<String>) -> Result<Self, Option<String>> {
        if end > start {
            Ok(Self { start, end, name })
        } else {
            Err(name)
        }
    }

    pub fn new_lossy(start: usize, end: usize, name: Option<String>) -> Self {
        Self {
            start,
            end: end.max(start + 1),
            name,
        }
    }

    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_str)
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SegmentGroups(Vec<SegmentGroup>);

impl SegmentGroups {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_vec_lossy(mut unordered_groups: Vec<SegmentGroup>) -> Self {
        unordered_groups.sort_unstable_by_key(|g| g.start);
        let mut min_start = 0;
        for group in &mut unordered_groups {
            group.start = group.start.max(min_start);
            group.end = group.end.max(group.start + 1);
            min_start = group.end;
        }
        Self(unordered_groups)
    }

    // TODO: Implement iterator instead (look at SegmentHistory)
    pub fn groups(&self) -> &[SegmentGroup] {
        &self.0
    }

    pub fn push_back(&mut self, group: SegmentGroup) -> Result<(), SegmentGroup> {
        if self.0.last().map_or(true, |last| group.start >= last.end) {
            self.0.push(group);
            Ok(())
        } else {
            Err(group)
        }
    }

    pub fn iter_with<'groups, 'segments>(
        &'groups self,
        segments: &'segments [Segment],
    ) -> SegmentGroupsIter<'groups, 'segments> {
        SegmentGroupsIter {
            iter: self.0.iter().peekable(),
            segments,
            index: 0,
        }
    }
}

#[derive(Debug)]
pub struct SegmentGroupView<'group, 'segments> {
    name: Option<&'group str>,
    segments: &'segments [Segment],
    ending_segment: &'segments Segment,
    start_index: usize,
    end_index: usize,
}

impl<'group, 'segments> SegmentGroupView<'group, 'segments> {
    pub fn name(&self) -> Option<&'group str> {
        self.name
    }

    pub fn name_or_default<'a>(&self) -> &'a str
    where
        'group: 'a,
        'segments: 'a,
    {
        self.name.unwrap_or_else(|| self.ending_segment.name())
    }

    pub fn segments(&self) -> &'segments [Segment] {
        self.segments
    }

    pub fn ending_segment(&self) -> &'segments Segment {
        self.ending_segment
    }

    pub fn start_index(&self) -> usize {
        self.start_index
    }

    pub fn contains(&self, index: usize) -> bool {
        index >= self.start_index && index < self.end_index
    }

    pub fn len(&self) -> usize {
        self.segments.len()
    }
}

pub struct SegmentGroupsIter<'groups, 'segments> {
    iter: Peekable<Iter<'groups, SegmentGroup>>,
    segments: &'segments [Segment],
    index: usize,
}

impl<'groups, 'segments> Iterator for SegmentGroupsIter<'groups, 'segments> {
    type Item = SegmentGroupView<'groups, 'segments>;

    fn next(&mut self) -> Option<Self::Item> {
        let start_index = self.index;
        if self
            .iter
            .peek()
            .map_or(true, |group| group.start > start_index)
        {
            self.index += 1;
            let ending_segment = self.segments.get(start_index)?;
            Some(SegmentGroupView {
                name: None,
                segments: slice::from_ref(ending_segment),
                ending_segment,
                start_index,
                end_index: self.index,
            })
        } else {
            let group = self.iter.next()?;
            self.index = group.end;
            let segments = self.segments.get(group.start..group.end)?;
            Some(SegmentGroupView {
                name: group.name.as_ref().map(String::as_str),
                segments,
                ending_segment: segments.last().unwrap(),
                start_index,
                end_index: self.index,
            })
        }
    }
}
