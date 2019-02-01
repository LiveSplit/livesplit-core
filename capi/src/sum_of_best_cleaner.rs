//! A Sum of Best Cleaner allows you to interactively remove potential issues in
//! the Segment History that lead to an inaccurate Sum of Best. If you skip a
//! split, whenever you get to the next split, the combined segment time might
//! be faster than the sum of the individual best segments. The Sum of Best
//! Cleaner will point out all of occurrences of this and allows you to delete
//! them individually if any of them seem wrong.

use crate::potential_clean_up::{NullableOwnedPotentialCleanUp, OwnedPotentialCleanUp};
use livesplit_core::run::editor::cleaning::SumOfBestCleaner;

/// type
pub type OwnedSumOfBestCleaner = Box<SumOfBestCleaner<'static>>;

/// drop
#[no_mangle]
pub extern "C" fn SumOfBestCleaner_drop(this: OwnedSumOfBestCleaner) {
    drop(this);
}

/// Returns the next potential clean up. If there are no more potential
/// clean ups, <NULL> is returned.
#[no_mangle]
pub extern "C" fn SumOfBestCleaner_next_potential_clean_up(
    this: &'static mut SumOfBestCleaner<'static>,
) -> NullableOwnedPotentialCleanUp {
    this.next_potential_clean_up().map(Box::new)
}

/// Applies a clean up to the Run.
#[no_mangle]
pub extern "C" fn SumOfBestCleaner_apply(
    this: &'static mut SumOfBestCleaner<'static>,
    clean_up: OwnedPotentialCleanUp,
) {
    this.apply((*clean_up).into());
}
