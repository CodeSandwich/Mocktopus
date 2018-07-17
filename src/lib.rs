#![feature(fn_traits, get_type_id, unboxed_closures, use_extern_macros)]

//! Mocking framework for Rust (currently only nightly)
//!
//! ```
//! #[mockable]
//! mod hello_world {
//!     pub fn world() -> &'static str {
//!         "world"
//!     }
//!
//!     pub fn hello_world() -> String {
//!         format!("Hello {}!", world())
//!     }
//! }
//!
//! #[test]
//! fn mock_test() {
//!     hello_world::world.mock_safe(|| MockResult::Return("mocking"));
//!
//!     assert_eq!("Hello mocking!", hello_world::hello_world());
//! }
//! ```
//! # Introduction
//! This is a user guide showing Rust project set up for testing with mocks.
//!
//! It is highly recommended to **use mocks ONLY for test runs and NEVER in release builds!**
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
//! #![cfg_attr(test, feature(proc_macro_mod, use_extern_macros))]
//! ```
//! Import Mocktopus:
//!
//! ```
//! #[cfg(test)]
//! extern crate mocktopus;
//! ```
//! # Making functions mockable
//! To make functions mockable they must be annotated with provided procedural macros.
//! See [documentation](https://docs.rs/mocktopus_macros) for all their possibilities and rules.
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
//! Import tools for mocking in test module:
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
//! // Mocking method
//! MyStruct::my_method.mock_safe(|| MockResult::Return(1));
//! // Mocking trait method
//! MyStruct::my_trait_method.mock_safe(|| MockResult::Return(2));
//! // Mocking default trait method
//! MyStruct::my_trait_default_method.mock_safe(|| MockResult::Return(3));
//! ```
//! Mocking with `mock_safe` is simplest, but the `Mockable` trait has more,
//! see [documantation](mocking/trait.Mockable.html).
//!
//! ## Mocking range
//! Every mock works only in thread, in which it was set.
//! All Rust test runs are executed in independent threads, so mocks do not leak between them:
//!
//! ```
//! #[cfg_attr(test, mockable)]
//! fn common_fn() -> u32 {
//!     0
//! }
//!
//! #[test]
//! fn common_fn_test_1() {
//!     assert_eq!(0, common_fn());
//!
//!     common_fn.mock_safe(|| MockResult::Return(1));
//!
//!     assert_eq!(1, common_fn());
//! }
//!
//! #[test]
//! fn common_fn_test_2() {
//!     assert_eq!(0, common_fn());
//!
//!     common_fn.mock_safe(|| MockResult::Return(2));
//!
//!     assert_eq!(2, common_fn());
//! }
//! ```
//!
//! ## Mock closure
//! `mock_safe` has single argument: a closure, which takes same input as mocked function and returns a `MockResult`.
//! Whenever the mocked function is called, its inputs are passed to the closure:
//!
//! ```
//! #[cfg_attr(test, mockable)]
//! fn my_function_1(_: u32) {
//!     return
//! }
//!
//! #[test]
//! fn my_function_1_test() {
//!     my_function_1.mock_safe(|x| {
//!         assert_eq!(2, x);
//!         MockResult::Return(())
//!     });
//!
//!     my_function_1(2); // Passes
//!     my_function_1(3); // Panics
//! }
//! ```
//! If the closure returns `MockResult::Return`, the mocked function does not run.
//! It immediately returns with a value, which is passed inside `MockResult::Return`:
//!
//! ```
//! #[cfg_attr(test, mockable)]
//! fn my_function_2() -> u32 {
//!     unreachable!()
//! }
//!
//! #[test]
//! fn my_function_2_test() {
//!     my_function_2.mock_safe(|| MockResult::Return(3));
//!
//!     assert_eq!(3, my_function_2());
//! }
//! ```
//! If the closure returns `MockResult::Continue`, the mocked function runs normally, but with changed arguments.
//! The new arguments are returned from closure in tuple inside `MockResult::Continue`:
//!
//! ```
//! #[cfg_attr(test, mockable)]
//! fn my_function_3(x: u32, y: u32) -> u32 {
//!     x + y
//! }
//!
//! #[test]
//! fn my_function_3_test() {
//!     my_function_3.mock_safe(|x, y| MockResult::Continue((x, y + 1)));
//!
//!     assert_eq!(3, my_function_3(1, 1));
//! }
//! ```
//!
//! ##Mocking generics
//! When mocking generic functions, all its generics must be defined and only this variant will be affected:
//!
//! ```
//! #[cfg_attr(test, mockable)]
//! fn generic_fn<T: Display>(t: T) -> String {
//!     t.to_string()
//! }
//!
//! #[test]
//! fn generic_fn_test() {
//!     generic_fn::<u32>.mock_safe(|_| MockResult::Return("mocked".to_string()));
//!
//!     assert_eq!("1", generic_fn(1i32));
//!     assert_eq!("mocked", generic_fn(1u32));
//! }
//! ```
//! The only exception are lifetimes, they are ignored:
//!
//! ```
//! #[cfg_attr(test, mockable)]
//! fn lifetime_generic_fn<'a>(string: &'a String) -> &'a str {
//!     string.as_ref()
//! }
//!
//! #[test]
//! fn lifetime_generic_fn_test() {
//!     lifetime_generic_fn.mock_safe(|_| MockResult::Return("mocked"));
//!
//!     assert_eq!("mocked", lifetime_generic_fn(&"not mocked".to_string()));
//! }
//! ```
//! Same rules apply to methods and structures:
//!
//! ```
//! struct GenericStruct<'a, T: Display + 'a>(&'a T);
//!
//! #[cfg_attr(test, mockable)]
//! impl<'a, T: Display + 'a> GenericStruct<'a, T> {
//!     fn to_string(&self) -> String {
//!         self.0.to_string()
//!     }
//! }
//!
//! static VALUE: u32 = 1;
//!
//! #[test]
//! fn lifetime_generic_fn_test() {
//!     GenericStruct::<u32>::to_string.mock_safe(|_| MockResult::Return("mocked".to_string()));
//!
//!     assert_eq!("mocked", GenericStruct(&VALUE).to_string());
//!     assert_eq!("mocked", GenericStruct(&2u32).to_string());
//!     assert_eq!("2", GenericStruct(&2i32).to_string());
//! }
//! ```
#![doc(html_logo_url = "https://raw.githubusercontent.com/CodeSandwich/mocktopus/master/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/CodeSandwich/mocktopus/master/logo.png")]

extern crate mocktopus_macros;

/// For use in testing code: mocking tools
pub mod mocking;

/// For use in testing code: helper tools for writing tests using mocking
pub mod mocking_utils;

/// For use in tested code: tools making items mockable
pub mod macros {
    pub use mocktopus_macros::*;
}


