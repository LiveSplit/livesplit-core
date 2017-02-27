use livesplit_core::component::timer::State as TimerComponentState;
use super::{drop, acc, output_str};
use libc::c_char;

pub type OwnedTimerComponentState = *mut TimerComponentState;

#[no_mangle]
pub unsafe extern "C" fn TimerComponentState_drop(this: OwnedTimerComponentState) {
    drop(this);
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
    output_str(&format!("{:?}", acc(this).color))
}
