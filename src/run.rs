use std::path::PathBuf;
use {TimeSpan, Attempt, RunMetadata, Segment};

#[derive(Default, Clone)]
pub struct Run {
    game_name: String,
    category_name: String,
    offset: TimeSpan,
    attempt_count: u64,
    attempt_history: Vec<Attempt>,
    metadata: RunMetadata,
    has_changed: bool,
    path: Option<PathBuf>,
    segments: Vec<Segment>,
}

impl Run {
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn offset(&self) -> TimeSpan {
        self.offset
    }

    pub fn start_next_run(&mut self) {
        self.attempt_count += 1;
        self.has_changed = true;
    }

    #[inline]
    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    #[inline]
    pub fn segments_mut(&mut self) -> &mut [Segment] {
        &mut self.segments
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    #[inline]
    pub fn mark_as_changed(&mut self) {
        self.has_changed = true;
    }
}
