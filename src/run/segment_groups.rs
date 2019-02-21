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

    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SegmentGroups(Vec<SegmentGroup>);

impl SegmentGroups {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push_back(&mut self, group: SegmentGroup) -> Result<(), SegmentGroup> {
        if self.0.last().map_or(true, |last| group.start >= last.end) {
            self.0.push(group);
            Ok(())
        } else {
            Err(group)
        }
    }
}
