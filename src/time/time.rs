use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};
use {TimeSpan, TimingMethod};

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct Time {
    pub real_time: Option<TimeSpan>,
    pub game_time: Option<TimeSpan>,
}

impl Time {
    #[inline]
    pub fn new() -> Self {
        Time::default()
    }

    #[inline]
    pub fn zero() -> Self {
        Time {
            real_time: Some(TimeSpan::zero()),
            game_time: Some(TimeSpan::zero()),
        }
    }

    #[inline]
    pub fn with_real_time(self, real_time: Option<TimeSpan>) -> Self {
        Time {
            real_time: real_time,
            ..self
        }
    }

    #[inline]
    pub fn with_game_time(self, game_time: Option<TimeSpan>) -> Self {
        Time {
            game_time: game_time,
            ..self
        }
    }

    #[inline]
    pub fn with_timing_method(
        mut self,
        timing_method: TimingMethod,
        time: Option<TimeSpan>,
    ) -> Self {
        self[timing_method] = time;
        self
    }

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

pub struct RealTime(pub Option<TimeSpan>);

impl From<RealTime> for Time {
    fn from(t: RealTime) -> Time {
        Time::new().with_real_time(t.0)
    }
}

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
