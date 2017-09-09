use livesplit_core::run::parser::composite::{ParsedRun, Result};
use super::{acc, alloc, output_str_with, own, own_drop};
use run::OwnedRun;
use std::fmt::Write;
use libc::c_char;

pub type ParseRunResult = Result<ParsedRun>;
pub type OwnedParseRunResult = *mut ParseRunResult;

#[no_mangle]
pub unsafe extern "C" fn ParseRunResult_drop(this: OwnedParseRunResult) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn ParseRunResult_parsed_successfully(this: *const ParseRunResult) -> bool {
    acc(this).is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn ParseRunResult_unwrap(this: OwnedParseRunResult) -> OwnedRun {
    alloc(own(this).unwrap().run)
}

#[no_mangle]
pub unsafe extern "C" fn ParseRunResult_timer_kind(this: *const ParseRunResult) -> *const c_char {
    output_str_with(|f| {
        write!(f, "{}", acc(this).as_ref().unwrap().kind).unwrap()
    })
}
