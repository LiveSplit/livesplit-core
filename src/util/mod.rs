//! Various utilities used in this crate.

pub(crate) mod byte_parsing;
mod clear_vec;
pub mod ordered_map;
mod populate_string;
#[cfg(test)]
pub mod tests_helper;
pub(crate) mod xml;

pub use self::{
    clear_vec::{Clear, ClearVec},
    populate_string::PopulateString,
};
