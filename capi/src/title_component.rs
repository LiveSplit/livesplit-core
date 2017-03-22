use livesplit_core::component::title::Component as TitleComponent;
use livesplit_core::Timer;
use super::{alloc, drop, acc, output_vec, acc_mut};
use libc::c_char;
use title_component_state::OwnedTitleComponentState;

pub type OwnedTitleComponent = *mut TitleComponent;

#[no_mangle]
pub unsafe extern "C" fn TitleComponent_new() -> OwnedTitleComponent {
    alloc(TitleComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponent_drop(this: OwnedTitleComponent) {
    drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponent_state_as_json(this: *mut TitleComponent,
                                                      timer: *const Timer)
                                                      -> *const c_char {
    output_vec(|o| { acc_mut(this).state(acc(timer)).write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponent_state(this: *mut TitleComponent,
                                              timer: *const Timer)
                                              -> OwnedTitleComponentState {
    alloc(acc_mut(this).state(acc(timer)))
}
