//! A Sum of Best Cleaner allows you to interactively remove potential issues in
//! the Segment History that lead to an inaccurate Sum of Best. If you skip a
//! split, whenever you get to the next split, the combined segment time might
//! be faster than the sum of the individual best segments. The Sum of Best
//! Cleaner will point out all of occurrences of this and allows you to delete
//! them individually if any of them seem wrong.

use livesplit_core::run::editor::cleaning::SumOfBestCleaner;
use super::{acc_mut, alloc, own, own_drop};
use std::ptr;
use potential_clean_up::{NullableOwnedPotentialCleanUp, OwnedPotentialCleanUp};

/// type
pub type OwnedSumOfBestCleaner = *mut SumOfBestCleaner<'static>;

/// drop
#[no_mangle]
pub unsafe extern "C" fn SumOfBestCleaner_drop(this: OwnedSumOfBestCleaner) {
    own_drop(this);
}

/// Returns the next potential clean up. If there are no more potential
/// clean ups, <NULL> is returned.
#[no_mangle]
pub unsafe extern "C" fn SumOfBestCleaner_next_potential_clean_up(
    this: *mut SumOfBestCleaner<'static>,
) -> NullableOwnedPotentialCleanUp {
    acc_mut(this)
        .next_potential_clean_up()
        .map_or_else(ptr::null_mut, alloc)
}

/// Applies a clean up to the Run.
#[no_mangle]
pub unsafe extern "C" fn SumOfBestCleaner_apply(
    this: *mut SumOfBestCleaner<'static>,
    clean_up: OwnedPotentialCleanUp,
) {
    acc_mut(this).apply(own(clean_up).into());
}
