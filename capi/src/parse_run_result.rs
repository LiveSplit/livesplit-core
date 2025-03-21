//! A run parsed by the Composite Parser. This contains the Run itself and
//! information about which parser parsed it.

use super::output_vec;
use crate::run::OwnedRun;
use livesplit_core::run::parser::{composite::ParsedRun, TimerKind};
use std::{io::Write, os::raw::c_char};

/// type
pub type ParseRunResult = Option<ParsedRun<'static>>;
/// type
pub type OwnedParseRunResult = Box<ParseRunResult>;

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn ParseRunResult_drop(this: OwnedParseRunResult) {
    drop(this);
}

/// Returns <TRUE> if the Run got parsed successfully. <FALSE> is returned otherwise.
#[unsafe(no_mangle)]
pub extern "C" fn ParseRunResult_parsed_successfully(this: &ParseRunResult) -> bool {
    this.is_some()
}

/// Moves the actual Run object out of the Result. You may not call this if the
/// Run wasn't parsed successfully.
#[unsafe(no_mangle)]
pub extern "C" fn ParseRunResult_unwrap(this: OwnedParseRunResult) -> OwnedRun {
    Box::new((*this).unwrap().run)
}

/// Accesses the name of the Parser that parsed the Run. You may not call this
/// if the Run wasn't parsed successfully.
#[unsafe(no_mangle)]
pub extern "C" fn ParseRunResult_timer_kind(this: &ParseRunResult) -> *const c_char {
    output_vec(|f| write!(f, "{}", this.as_ref().unwrap().kind).unwrap())
}

/// Checks whether the Parser parsed a generic timer. Since a generic timer can
/// have any name, it may clash with the specific timer formats that
/// livesplit-core supports. With this function you can determine if a generic
/// timer format was parsed, instead of one of the more specific timer formats.
#[unsafe(no_mangle)]
pub extern "C" fn ParseRunResult_is_generic_timer(this: &ParseRunResult) -> bool {
    matches!(
        this,
        Some(ParsedRun {
            kind: TimerKind::Generic(_),
            ..
        })
    )
}
