use {SegmentHistory, Time};

#[derive(Clone, Default)]
pub struct Segment {
    name: String,
    personal_best_split_time: Time,
    best_segment_time: Time,
    split_time: Time,
    segment_history: SegmentHistory,
}

impl Segment {
    pub fn new<S>(name: S) -> Self
        where S: Into<String>
    {
        Segment { name: name.into(), ..Default::default() }
    }

    #[inline]
    pub fn set_personal_best_split_time(&mut self, time: Time) {
        self.personal_best_split_time = time;
    }

    #[inline]
    pub fn set_best_segment_time(&mut self, time: Time) {
        self.best_segment_time = time;
    }

    #[inline]
    pub fn split_time(&self) -> Time {
        self.split_time
    }

    #[inline]
    pub fn set_split_time(&mut self, time: Time) {
        self.split_time = time;
    }

    #[inline]
    pub fn clear_split_time(&mut self) {
        self.set_split_time(Default::default());
    }
}
