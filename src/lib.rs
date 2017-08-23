#![feature(fn_traits, get_type_id, proc_macro, unboxed_closures)]

extern crate mocktopus_macros;

/// For use in tested code: tools making items mockable
pub mod macros {
    pub use mocktopus_macros::*;
}
/// For use in testing code: mocking tools
pub mod mocking;

