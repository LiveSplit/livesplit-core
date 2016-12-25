use {AtomicDateTime, Time, TimeSpan};

#[derive(Clone)]
pub struct Attempt {
    index: i64,
    time: Time,
    started: Option<AtomicDateTime>,
    ended: Option<AtomicDateTime>,
}

impl Attempt {
    pub fn new(index: i64,
               time: Time,
               started: Option<AtomicDateTime>,
               ended: Option<AtomicDateTime>)
               -> Self {
        Attempt {
            index: index,
            time: time,
            started: started,
            ended: ended,
        }
    }

    /// Returns the Real Time Duration of the attempt.
    /// This either returns a 1.6+ Time Stamp based duration
    /// or the duration of the run (assuming it's not resetted)
    /// if it's from before LiveSplit 1.6. If it is from before
    /// 1.6 and resetted then it will return null.
    pub fn duration(&self) -> Option<TimeSpan> {
        if let (Some(started), Some(ended)) = (self.started, self.ended) {
            Some(ended - started)
        } else {
            self.time.real_time
        }
    }
}
