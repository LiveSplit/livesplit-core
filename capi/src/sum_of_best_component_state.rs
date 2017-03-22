use livesplit_core::component::sum_of_best::State as SumOfBestComponentState;
use super::{drop, acc, output_str};
use libc::c_char;

pub type OwnedSumOfBestComponentState = *mut SumOfBestComponentState;

#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponentState_drop(this: OwnedSumOfBestComponentState) {
    drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponentState_text(this: *const SumOfBestComponentState)
                                                      -> *const c_char {
    output_str(&acc(this).text)
}

#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponentState_time(this: *const SumOfBestComponentState)
                                                      -> *const c_char {
    output_str(&acc(this).time)
}
