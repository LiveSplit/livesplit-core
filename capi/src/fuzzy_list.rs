//! With a Fuzzy List, you can implement a fuzzy searching algorithm. The list
//! stores all the items that can be searched for. With the `search` method you
//! can then execute the actual fuzzy search which returns a list of all the
//! elements found. This can be used to implement searching in a list of games.

use super::{Json, output_vec, str};
use livesplit_core::run::editor::FuzzyList;
use serde_json::to_writer;
use std::os::raw::c_char;

/// type
pub type OwnedFuzzyList = Box<FuzzyList>;

/// Creates a new Fuzzy List.
#[unsafe(no_mangle)]
pub extern "C" fn FuzzyList_new() -> OwnedFuzzyList {
    Box::new(FuzzyList::new())
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn FuzzyList_drop(this: OwnedFuzzyList) {
    drop(this);
}

/// Adds a new element to the list.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FuzzyList_push(this: &mut FuzzyList, text: *const c_char) {
    // SAFETY: The caller guarantees that `text` is valid.
    this.push(unsafe { str(text) });
}

/// Searches for the pattern provided in the list. A list of all the
/// matching elements is returned. The returned list has a maximum amount of
/// elements provided to this method.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FuzzyList_search(
    this: &FuzzyList,
    pattern: *const c_char,
    max: usize,
) -> Json {
    // SAFETY: The caller guarantees that `pattern` is valid.
    output_vec(|o| {
        to_writer(o, &this.search(unsafe { str(pattern) }, max)).unwrap();
    })
}
