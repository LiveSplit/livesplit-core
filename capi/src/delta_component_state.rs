use livesplit_core::component::delta::State as DeltaComponentState;
use super::{acc, output_str, output_vec, own_drop};
use std::io::Write;
use libc::c_char;

pub type OwnedDeltaComponentState = *mut DeltaComponentState;

#[no_mangle]
pub unsafe extern "C" fn DeltaComponentState_drop(this: OwnedDeltaComponentState) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn DeltaComponentState_text(
    this: *const DeltaComponentState,
) -> *const c_char {
    output_str(&acc(this).text)
}

#[no_mangle]
pub unsafe extern "C" fn DeltaComponentState_time(
    this: *const DeltaComponentState,
) -> *const c_char {
    output_str(&acc(this).time)
}

#[no_mangle]
pub unsafe extern "C" fn DeltaComponentState_semantic_color(
    this: *const DeltaComponentState,
) -> *const c_char {
    output_vec(|f| write!(f, "{:?}", acc(this).semantic_color).unwrap())
}
