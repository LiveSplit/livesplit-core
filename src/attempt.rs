use {AtomicDateTime, Time, TimeSpan};

#[derive(new, Clone, Debug)]
pub struct Attempt {
    index: i32,
    time: Time,
    started: Option<AtomicDateTime>,
    ended: Option<AtomicDateTime>,
    pause_time: Option<TimeSpan>,
}

impl Attempt {
    /// Returns the Real Time Duration of the attempt.
    /// This either returns a 1.6+ Time Stamp based duration
    /// or the duration of the run (assuming it's not resetted)
    /// if it's from before LiveSplit 1.6. If it is from before
    /// 1.6 and resetted then it will return null.
    pub fn duration(&self) -> Option<TimeSpan> {
        AtomicDateTime::option_op(self.started, self.ended, |s, e| e - s).or(self.time.real_time)
    }

    #[inline]
    pub fn index(&self) -> i32 {
        self.index
    }

    #[inline]
    pub fn time(&self) -> Time {
        self.time
    }

    #[inline]
    pub fn pause_time(&self) -> Option<TimeSpan> {
        self.pause_time
    }

    #[inline]
    pub fn started(&self) -> Option<AtomicDateTime> {
        self.started
    }

    #[inline]
    pub fn ended(&self) -> Option<AtomicDateTime> {
        self.ended
    }
}
