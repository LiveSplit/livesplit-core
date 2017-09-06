use livesplit_core::run::editor::cleaning::{PotentialCleanUp, SumOfBestCleaner};
use super::{acc, acc_mut, alloc, output_str_with, own, own_drop};
use libc::c_char;
use std::fmt::Write;
use std::ptr;

pub type OwnedSumOfBestCleaner = *mut SumOfBestCleaner<'static>;
pub type OwnedPotentialCleanUp = *mut PotentialCleanUp<'static>;
pub type NullableOwnedPotentialCleanUp = OwnedPotentialCleanUp;

#[no_mangle]
pub unsafe extern "C" fn SumOfBestCleaner_drop(this: OwnedSumOfBestCleaner) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn SumOfBestCleaner_next_potential_clean_up(
    this: *mut SumOfBestCleaner<'static>,
) -> NullableOwnedPotentialCleanUp {
    acc_mut(&this)
        .next_potential_clean_up()
        .map_or_else(ptr::null_mut, alloc)
}

#[no_mangle]
pub unsafe extern "C" fn SumOfBestCleaner_apply(
    this: *mut SumOfBestCleaner<'static>,
    clean_up: OwnedPotentialCleanUp,
) {
    acc_mut(&this).apply(own(clean_up).into());
}

#[no_mangle]
pub unsafe extern "C" fn PotentialCleanUp_drop(this: OwnedPotentialCleanUp) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn PotentialCleanUp_message(
    this: *const PotentialCleanUp<'static>,
) -> *const c_char {
    output_str_with(|s| write!(s, "{}", acc(&this)).unwrap())
}
