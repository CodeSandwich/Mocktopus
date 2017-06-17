#![feature(fn_traits, get_type_id, unboxed_closures)]

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

mod mock_trait;

pub use mock_trait::{MockResult, MockTrait};

