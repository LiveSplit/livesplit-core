use std::ops::{Index, IndexMut, Add, Sub};
use {TimingMethod, TimeSpan};

#[derive(Copy, Clone, Default, Eq, PartialEq)]
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
    pub fn with_real_time(self, real_time: Option<TimeSpan>) -> Self {
        Time { real_time: real_time, ..self }
    }

    #[inline]
    pub fn with_game_time(self, game_time: Option<TimeSpan>) -> Self {
        Time { game_time: game_time, ..self }
    }

    #[inline]
    pub fn with_timing_method(mut self,
                              timing_method: TimingMethod,
                              time: Option<TimeSpan>)
                              -> Self {
        self[timing_method] = time;
        self
    }
}

impl Add for Time {
    type Output = Time;

    fn add(self, rhs: Time) -> Self {
        Time {
            real_time: self.real_time.and_then(|a| rhs.real_time.map(|b| a + b)),
            game_time: self.game_time.and_then(|a| rhs.game_time.map(|b| a + b)),
        }
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, rhs: Time) -> Self {
        Time {
            real_time: self.real_time.and_then(|a| rhs.real_time.map(|b| a - b)),
            game_time: self.game_time.and_then(|a| rhs.game_time.map(|b| a - b)),
        }
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
