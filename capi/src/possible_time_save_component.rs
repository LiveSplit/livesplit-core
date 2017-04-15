use livesplit_core::component::possible_time_save::Component as PossibleTimeSaveComponent;
use livesplit_core::Timer;
use super::{alloc, own_drop, acc, output_vec};
use libc::c_char;
use possible_time_save_component_state::OwnedPossibleTimeSaveComponentState;

pub type OwnedPossibleTimeSaveComponent = *mut PossibleTimeSaveComponent;

#[no_mangle]
pub unsafe extern "C" fn PossibleTimeSaveComponent_new() -> OwnedPossibleTimeSaveComponent {
    alloc(PossibleTimeSaveComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn PossibleTimeSaveComponent_drop(this: OwnedPossibleTimeSaveComponent) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn PossibleTimeSaveComponent_state_as_json(this: *const PossibleTimeSaveComponent,
                                                                 timer: *const Timer)
                                                                 -> *const c_char {
    output_vec(|o| { acc(this).state(acc(timer)).write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn PossibleTimeSaveComponent_state(this: *const PossibleTimeSaveComponent,
                                                         timer: *const Timer)
                                                         -> OwnedPossibleTimeSaveComponentState {
    alloc(acc(this).state(acc(timer)))
}
