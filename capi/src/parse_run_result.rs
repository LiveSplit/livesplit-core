//! A run parsed by the Composite Parser. This contains the Run itself and
//! information about which parser parsed it.

use livesplit_core::run::parser::composite::{ParsedRun, Result};
use super::{acc, alloc, output_vec, own, own_drop};
use run::OwnedRun;
use std::io::Write;
use std::os::raw::c_char;

/// type
pub type ParseRunResult = Result<ParsedRun>;
/// type
pub type OwnedParseRunResult = *mut ParseRunResult;

/// drop
#[no_mangle]
pub unsafe extern "C" fn ParseRunResult_drop(this: OwnedParseRunResult) {
    own_drop(this);
}

/// Returns <TRUE> if the Run got parsed successfully. <FALSE> is returned otherwise.
#[no_mangle]
pub unsafe extern "C" fn ParseRunResult_parsed_successfully(this: *const ParseRunResult) -> bool {
    acc(this).is_ok()
}

/// Moves the actual Run object out of the Result. You may not call this if the
/// Run wasn't parsed successfully.
#[no_mangle]
pub unsafe extern "C" fn ParseRunResult_unwrap(this: OwnedParseRunResult) -> OwnedRun {
    alloc(own(this).unwrap().run)
}

/// Accesses the name of the Parser that parsed the Run.
#[no_mangle]
pub unsafe extern "C" fn ParseRunResult_timer_kind(this: *const ParseRunResult) -> *const c_char {
    output_vec(|f| write!(f, "{}", acc(this).as_ref().unwrap().kind).unwrap())
}
