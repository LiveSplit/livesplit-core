use livesplit_core::component::timer::State as TimerComponentState;
use super::{own_drop, acc, output_str, output_str_with};
use libc::c_char;
use std::fmt::Write;

pub type OwnedTimerComponentState = *mut TimerComponentState;

#[no_mangle]
pub unsafe extern "C" fn TimerComponentState_drop(this: OwnedTimerComponentState) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn TimerComponentState_time(this: *const TimerComponentState)
                                                  -> *const c_char {
    output_str(&acc(this).time)
}

#[no_mangle]
pub unsafe extern "C" fn TimerComponentState_fraction(this: *const TimerComponentState)
                                                      -> *const c_char {
    output_str(&acc(this).fraction)
}

#[no_mangle]
pub unsafe extern "C" fn TimerComponentState_color(this: *const TimerComponentState)
                                                   -> *const c_char {
    output_str_with(|f| write!(f, "{:?}", acc(this).color).unwrap())
}
