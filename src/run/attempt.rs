use crate::{AtomicDateTime, Time, TimeSpan};

/// An Attempt describes information about an attempt to run a specific category
/// by a specific runner in the past. Every time a new attempt is started and
/// then reset, an Attempt describing general information about it is created.
#[derive(Clone, Debug, PartialEq)]
pub struct Attempt {
    index: i32,
    time: Time,
    started: Option<AtomicDateTime>,
    ended: Option<AtomicDateTime>,
    pause_time: Option<TimeSpan>,
}

impl Attempt {
    /// Creates a new Attempt, logging an attempt to speedrun a category. You
    /// need the provide a unique index for the attempt (The index needs to be
    /// unique for the Run, not across all the Run objects). Additionally you
    /// provide the split time of the last segment. If it is empty, the attempt
    /// is considered being reset early. If there's information available about
    /// when the attempt was started and when it ended, this information can be
    /// provided. Both of these should be provided for unfinished attempts as
    /// well, if possible. If it is known that the attempt was paused for a
    /// certain amount of time, this can be provided as well.
    pub fn new(
        index: i32,
        time: Time,
        started: Option<AtomicDateTime>,
        ended: Option<AtomicDateTime>,
        pause_time: Option<TimeSpan>,
    ) -> Self {
        Self {
            index,
            time,
            started,
            ended,
            pause_time,
        }
    }

    /// Returns the total duration of the attempt, from the point in time it
    /// started to the point in time it ended. This is different from the real
    /// time of the run, as it includes all the pause times and the timer offset
    /// at the beginning of the run. If not enough information is known to
    /// derive this, the real time is used as a fallback. So if for example the
    /// timer started at -2s, the runner paused it for 5s and the timer ended at
    /// a real time value of 10s, then the actual duration of the attempt was
    /// 17s.
    pub fn duration(&self) -> Option<TimeSpan> {
        let diff = catch! { self.ended? - self.started? };
        diff.or(self.time.real_time)
    }

    /// Accesses the unique index of the attempt. This index is unique for the
    /// Run, not for all of them.
    #[inline]
    pub fn index(&self) -> i32 {
        self.index
    }

    /// Accesses the split time of the last segment. If the attempt got reset
    /// early and didn't finish, this may be empty.
    #[inline]
    pub fn time(&self) -> Time {
        self.time
    }

    /// Accesses the amount of time the attempt has been paused for. If it is
    /// not known, this returns `None`. This means that it may not necessarily
    /// be possible to differentiate whether a Run has not been paused or it
    /// simply wasn't stored.
    #[inline]
    pub fn pause_time(&self) -> Option<TimeSpan> {
        self.pause_time
    }

    /// Accesses the point in time the attempt was started at. This returns
    /// `None` if this information is not known.
    #[inline]
    pub fn started(&self) -> Option<AtomicDateTime> {
        self.started
    }

    /// Accesses the point in time the attempt was ended at. This returns `None`
    /// if this information is not known.
    #[inline]
    pub fn ended(&self) -> Option<AtomicDateTime> {
        self.ended
    }
}
