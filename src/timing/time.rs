use crate::{TimeSpan, TimingMethod};
use core::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};

/// A time that can store a Real Time and a Game Time. Both of them are
/// optional.
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct Time {
    /// The Real Time value.
    pub real_time: Option<TimeSpan>,
    /// The Game Time value.
    pub game_time: Option<TimeSpan>,
}

impl Time {
    /// Creates a new Time with empty Real Time and Game Time.
    #[inline]
    pub fn new() -> Self {
        Time::default()
    }

    /// Creates a new Time where Real Time and Game Time are zero. Keep in mind
    /// that a zero Time Span is not the same as a `None` Time Span as created
    /// by `Time::new()`.
    #[inline]
    pub fn zero() -> Self {
        Time {
            real_time: Some(TimeSpan::zero()),
            game_time: Some(TimeSpan::zero()),
        }
    }

    /// Creates a new Time based on the current one where the Real Time is
    /// replaced by the given Time Span.
    #[inline]
    pub fn with_real_time(self, real_time: Option<TimeSpan>) -> Self {
        Time { real_time, ..self }
    }

    /// Creates a new Time based on the current one where the Game Time is
    /// replaced by the given Time Span.
    #[inline]
    pub fn with_game_time(self, game_time: Option<TimeSpan>) -> Self {
        Time { game_time, ..self }
    }

    /// Creates a new Time based on the current one where the specified timing
    /// method is replaced by the given Time Span.
    #[inline]
    pub fn with_timing_method(
        mut self,
        timing_method: TimingMethod,
        time: Option<TimeSpan>,
    ) -> Self {
        self[timing_method] = time;
        self
    }

    /// Applies an operation to both Timing Methods of the two times provided
    /// and creates a new Time from the result.
    pub fn op<F>(a: Time, b: Time, mut f: F) -> Time
    where
        F: FnMut(TimeSpan, TimeSpan) -> TimeSpan,
    {
        Time {
            real_time: catch! { f(a.real_time?, b.real_time?) },
            game_time: catch! { f(a.game_time?, b.game_time?) },
        }
    }
}

/// Represents a Time Span intended to be used as a Real Time.
pub struct RealTime(pub Option<TimeSpan>);

impl From<RealTime> for Time {
    fn from(t: RealTime) -> Time {
        Time::new().with_real_time(t.0)
    }
}

/// Represents a Time Span intended to be used as a Game Time.
pub struct GameTime(pub Option<TimeSpan>);

impl From<GameTime> for Time {
    fn from(t: GameTime) -> Time {
        Time::new().with_game_time(t.0)
    }
}

impl Add for Time {
    type Output = Time;

    fn add(self, rhs: Time) -> Self {
        Time::op(self, rhs, Add::add)
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, rhs: Time) {
        *self = *self + rhs;
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, rhs: Time) -> Self {
        Time::op(self, rhs, Sub::sub)
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, rhs: Time) {
        *self = *self - rhs;
    }
}

impl Index<TimingMethod> for Time {
    type Output = Option<TimeSpan>;

    fn index(&self, timing_method: TimingMethod) -> &Self::Output {
        match timing_method {
            TimingMethod::RealTime => &self.real_time,
            TimingMethod::GameTime => &self.game_time,
        }
    }
}

impl IndexMut<TimingMethod> for Time {
    fn index_mut(&mut self, timing_method: TimingMethod) -> &mut Self::Output {
        match timing_method {
            TimingMethod::RealTime => &mut self.real_time,
            TimingMethod::GameTime => &mut self.game_time,
        }
    }
}
