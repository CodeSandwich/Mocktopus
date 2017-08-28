#![feature(fn_traits, get_type_id, proc_macro, unboxed_closures)]

//! Mocking framework for Rust
//!
//! ```
//! #[mockable]
//! fn world() -> &'static str {
//!     "world"
//! }
//!
//! fn hello_world() -> String {
//!     format!("Hello {}!", world())
//! }
//!
//! #[test]
//! fn mock_test() {
//!     world.mock_safe(|| MockResult::Return("mocking"));
//!
//!     assert_eq!("Hello mocking!", hello_world());
//! }
//! ```
//! # About this document
//! This is a user guide showing Rust project set up for testing with mocks.
//! For in-depth developer guide visit [GitHub page](https://github.com/CodeSandwich/Mocktopus).
//!
//! **It is highly recommended to use mocks ONLY for test runs and NEVER in release builds!**
//! Mocktopus is not designed for high performance and will slow down code execution.
//!
//! Note: this guide shows set up of mocking for test builds only.
//! # Prerequisites
//! Add Mocktopus dev-dependency to project's `Cargo.toml`:
//!
//! ```
//! [dev-dependencies]
//! mocktopus = "0.1.0"
//! ```
//! Enable procedural macros in crate root:
//!
//! ```
//! #![cfg_attr(test, feature(proc_macro))]
//! ```
//! Import Mocktopus:
//!
//! ```
//! #[cfg(test)]
//! extern crate mocktopus;
//! ```
//! This import MUST NOT be aliased, mocking framework depends on Mocktopus root being visible as `mocktopus`.
//! # Making functions mockable
//! To make functions mockable they must be annotated with provided procedural macros.
//! See [documentation](../mocktopus_macros) to discover all their possibilities and rules.
//!
//! To use these macros import them into namespace:
//!
//! ```
//! #[cfg(test)]
//! use mocktopus::macros::*;
//! ```
//! It is possible to annotate modules, which makes all their potentially mockable content mockable.
//! To make every function in project mockable annotate each module in its root:
//!
//! ```
//! #[cfg_attr(test, mockable)]
//! mod my_module;
//! ```
//! # Mocking
//! There is a set of tools for using mocks in tests. Import them in module with tests:
//!
//! ```
//! #[cfg(test)]
//! mod tests {
//!     use mocktopus::mocking::*;
//! ```
//! Among others this imports trait `Mockable`.
//! It is implemented for all functions and provides an interface for setting up mocks:
//!
//! ```
//! #[test]
//! fn my_test() {
//!     my_function.mock_safe(|| MockResult::Return(1));
//!
//!     assert_eq!(1, my_function());
//! }
//! ```
//! It is also possible to mock struct methods, either from own impls, traits or trait defaults:
//!
//! ```
//! #[test]
//! fn my_test() {
//!     MyStruct::my_method.mock_safe(|| MockResult::Return(1));
//!     MyStruct::my_trait_method.mock_safe(|| MockResult::Return(2));
//!     MyStruct::my_trait_default_method.mock_safe(|| MockResult::Return(3));
//! ```
//! This guide focuses only on mocking with the simplest method, `mock_safe`,
//! but the `Mockable` trait provides more, see [documantation](mocking/trait.Mockable.html).
//!
//! `mock_safe` has 1 argument: a closure, which takes same input as mocked function and returns a `MockResult`.
//! Whenever the mocked function is called, its inputs are passed to the closure.
//!
//! If the closure returns `MockResult::Return`, the mocked function does not run.
//! It immediately returns with value from closure, which is passed inside `MockResult::Return`:
//!
//!
//!

extern crate mocktopus_macros;

/// For use in testing code: mocking tools
pub mod mocking;
/// For use in testing code: helper tools for writing tests using mocking
pub mod mocking_utils;
/// For use in tested code: tools making items mockable
pub mod macros {
    pub use mocktopus_macros::*;
}


