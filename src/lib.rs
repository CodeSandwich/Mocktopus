#![feature(fn_traits, get_type_id, proc_macro, unboxed_closures)]

extern crate mocktopus_macros;

pub mod utils;
pub mod macros {
    pub use mocktopus_macros::*;
}
pub mod mocking;

