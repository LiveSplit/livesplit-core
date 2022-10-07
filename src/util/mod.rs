//! Various utilities used in this crate.

pub(crate) mod ascii_char;
pub(crate) mod ascii_set;
pub(crate) mod byte_parsing;
mod clear_vec;
pub(crate) mod not_nan;
pub mod ordered_map;
mod populate_string;
#[cfg(test)]
pub mod tests_helper;
pub(crate) mod xml;

pub use self::{
    clear_vec::{Clear, ClearVec},
    populate_string::PopulateString,
};
