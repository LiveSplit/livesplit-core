use livesplit_core::component::splits::Component as SplitsComponent;
use livesplit_core::{GeneralLayoutSettings, Timer};
use super::{acc, acc_mut, alloc, output_vec, own, own_drop, Json};
use splits_component_state::OwnedSplitsComponentState;
use component::OwnedComponent;

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
pub unsafe extern "C" fn SplitsComponent_into_generic(
    this: OwnedSplitsComponent,
) -> OwnedComponent {
    alloc(own(this).into())
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_state_as_json(
    this: *mut SplitsComponent,
    timer: *const Timer,
    layout_settings: *const GeneralLayoutSettings,
) -> Json {
    output_vec(|o| {
        acc_mut(&this)
            .state(acc(&timer), acc(&layout_settings))
            .write_json(o)
            .unwrap();
    })
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_state(
    this: *mut SplitsComponent,
    timer: *const Timer,
    layout_settings: *const GeneralLayoutSettings,
) -> OwnedSplitsComponentState {
    alloc(acc_mut(&this).state(acc(&timer), acc(&layout_settings)))
}


#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_scroll_up(this: *mut SplitsComponent) {
    acc_mut(&this).scroll_up();
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_scroll_down(this: *mut SplitsComponent) {
    acc_mut(&this).scroll_down();
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_set_visual_split_count(
    this: *mut SplitsComponent,
    count: usize,
) {
    acc_mut(&this).settings_mut().visual_split_count = count;
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_set_split_preview_count(
    this: *mut SplitsComponent,
    count: usize,
) {
    acc_mut(&this).settings_mut().split_preview_count = count;
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_set_always_show_last_split(
    this: *mut SplitsComponent,
    always_show_last_split: bool,
) {
    acc_mut(&this).settings_mut().always_show_last_split = always_show_last_split;
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_set_separator_last_split(
    this: *mut SplitsComponent,
    separator_last_split: bool,
) {
    acc_mut(&this).settings_mut().separator_last_split = separator_last_split;
}
