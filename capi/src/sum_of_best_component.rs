//! The Sum of Best Segments Component shows the fastest possible time to
//! complete a run of this category, based on information collected from all the
//! previous attempts. This often matches up with the sum of the best segment
//! times of all the segments, but that may not always be the case, as skipped
//! segments may introduce combined segments that may be faster than the actual
//! sum of their best segment times. The name is therefore a bit misleading, but
//! sticks around for historical reasons.

use super::{acc, alloc, output_vec, own, own_drop, Json};
use component::OwnedComponent;
use livesplit_core::component::sum_of_best::Component as SumOfBestComponent;
use livesplit_core::Timer;
use sum_of_best_component_state::OwnedSumOfBestComponentState;

/// type
pub type OwnedSumOfBestComponent = *mut SumOfBestComponent;

/// Creates a new Sum of Best Segments Component.
#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponent_new() -> OwnedSumOfBestComponent {
    alloc(SumOfBestComponent::new())
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponent_drop(this: OwnedSumOfBestComponent) {
    own_drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponent_into_generic(
    this: OwnedSumOfBestComponent,
) -> OwnedComponent {
    alloc(own(this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponent_state_as_json(
    this: *const SumOfBestComponent,
    timer: *const Timer,
) -> Json {
    output_vec(|o| {
        acc(this).state(acc(timer)).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponent_state(
    this: *const SumOfBestComponent,
    timer: *const Timer,
) -> OwnedSumOfBestComponentState {
    alloc(acc(this).state(acc(timer)))
}
