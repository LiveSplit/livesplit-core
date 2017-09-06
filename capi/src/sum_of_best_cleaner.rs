use livesplit_core::run::editor::cleaning::{PotentialCleanUp, SumOfBestCleaner};
use super::{acc, acc_mut, alloc, output_str_with, own, own_drop};
use libc::c_char;
use std::fmt::Write;
use std::ptr;

pub type OwnedSumOfBestCleaner<'a> = *mut SumOfBestCleaner<'a>;
pub type OwnedPotentialCleanUp<'a> = *mut PotentialCleanUp<'a>;
pub type NullableOwnedPotentialCleanUp<'a> = OwnedPotentialCleanUp<'a>;

#[no_mangle]
pub unsafe extern "C" fn SumOfBestCleaner_drop(this: OwnedSumOfBestCleaner) {
    own_drop(this);
}

/// # Safety
/// `this` must outlive `NullableOwnedPotentialCleanUp`
#[no_mangle]
pub unsafe extern "C" fn SumOfBestCleaner_next_potential_clean_up<'a>(
    this: *mut SumOfBestCleaner<'a>,
) -> NullableOwnedPotentialCleanUp<'a> {
    (&mut *this)
        .next_potential_clean_up()
        .map_or_else(ptr::null_mut, alloc)
}

#[no_mangle]
pub unsafe extern "C" fn SumOfBestCleaner_apply<'a>(
    this: *mut SumOfBestCleaner<'a>,
    clean_up: OwnedPotentialCleanUp<'a>,
) {
    acc_mut(&this).apply(own(clean_up).into());
}

#[no_mangle]
pub unsafe extern "C" fn PotentialCleanUp_drop(this: OwnedPotentialCleanUp) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn PotentialCleanUp_message<'a>(
    this: *const PotentialCleanUp<'a>,
) -> *const c_char {
    output_str_with(|s| write!(s, "{}", acc(&this)).unwrap())
}
