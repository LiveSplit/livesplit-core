use livesplit_core::component::splits::Component as SplitsComponent;
use livesplit_core::Timer;
use super::{Json, alloc, own_drop, acc, output_vec, acc_mut};
use splits_component_state::OwnedSplitsComponentState;

pub type OwnedSplitsComponent = *mut SplitsComponent;

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_new() -> OwnedSplitsComponent {
    alloc(SplitsComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_drop(this: OwnedSplitsComponent) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_state_as_json(this: *mut SplitsComponent,
                                                       timer: *const Timer)
                                                       -> Json {
    output_vec(|o| { acc_mut(this).state(acc(timer)).write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_state(this: *mut SplitsComponent,
                                               timer: *const Timer)
                                               -> OwnedSplitsComponentState {
    alloc(acc_mut(this).state(acc(timer)))
}


#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_scroll_up(this: *mut SplitsComponent) {
    acc_mut(this).scroll_up();
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_scroll_down(this: *mut SplitsComponent) {
    acc_mut(this).scroll_down();
}
