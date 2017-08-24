//! Macros making items mockable

#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
extern crate quote;

mod display_delegate;
mod item_injector;
mod header_builder;
mod lifetime_remover;

use proc_macro::TokenStream;
use quote::{Tokens, ToTokens};
use std::str::FromStr;

/// Procedural macro, makes items and their sub-items mockable
///
/// # Valid to annotate
/// - modules (makes all its valid to annotate items annotated)
///
/// ```ignore
/// #[mockable]
/// mod module {
///     fn mockable() { ... }
/// }
/// ```
/// - standalone functions
///
/// ```ignore
/// #[mockable]
/// fn mockable() { ... }
/// ```
/// - struct impl blocks (makes all functions inside mockable)
///
/// ```ignore
/// #[mockable]
/// impl Structure {
///     fn mockable() { ... }
/// }
/// ```
/// - trait impl blocks (makes all functions inside mockable)
///
/// ```ignore
/// #[mockable]
/// impl Trait for Structure {
///     fn mockable() { ... }
/// }
/// ```
/// - traits (makes all default functions inside mockable)
///
/// ```ignore
/// #[mockable]
/// trait Trait {
///     fn mockable() { ... }
/// }
/// ```
/// # Invalid to annotate
/// **CAUTION! will break mocking or fail to compile**
///
/// - single functions in struct impls
///
/// ```ignore
/// impl Structure {
///     #[mockable] //INVALID USAGE!
///     fn mockable() { ... }
/// }
/// ```
/// - single functions in trait impls
///
/// ```ignore
/// impl Trait for Structure {
///     #[mockable] //INVALID USAGE!
///     fn mockable() { ... }
/// }
/// ```
/// - single default functions in traits
///
/// ```ignore
/// trait Trait {
///     #[mockable] //INVALID USAGE!
///     fn mockable() { ... }
/// }
/// ```
///
/// # Indifferent to annotate
/// - items in annotated modules
/// - const functions
/// - any macro generated items
/// - any other items
#[proc_macro_attribute]
pub fn mockable(_: TokenStream, token_stream: TokenStream) -> TokenStream {
    let in_string = token_stream.to_string();
    let mut parsed = match syn::parse_item(&in_string) {
        Ok(parsed) => parsed,
        Err(_) => return token_stream,
    };
    item_injector::inject_item(&mut parsed);
    let mut tokens = Tokens::new();
    parsed.to_tokens(&mut tokens);
    let out_string = tokens.as_str();
    let out_token_stream = TokenStream::from_str(out_string).unwrap();
    out_token_stream
}

/// Procedural macro, guards items from being made mockable by enclosing item.
///
/// # Valid to annotate
/// - modules
///
/// ```ignore
/// #[mockable]
/// mod module {
///     #[not_mockable]
///     mod module {
///         fn not_mockable() { ... }
///     }
/// }
/// ```
/// - standalone functions
///
/// ```ignore
/// #[mockable]
/// mod module {
///     #[not_mockable]
///     fn not_mockable() { ... }
/// }
/// ```
/// - struct impl blocks
///
/// ```ignore
/// #[mockable]
/// mod module {
///     #[not_mockable]
///     impl Struct {
///         fn not_mockable() { ... }
///     }
/// }
/// ```
/// - single functions in struct impls
///
/// ```ignore
/// #[mockable]
/// impl Struct {
///     #[not_mockable]
///     fn not_mockable() { ... }
/// }
/// ```
/// - trait impl blocks
///
/// ```ignore
/// #[mockable]
/// mod module {
///     #[not_mockable]
///     impl Trait for Struct {
///         fn not_mockable() { ... }
///     }
/// }
/// ```
/// - single functions in trait impls
///
/// ```ignore
/// #[mockable]
/// impl Trait for Struct {
///     #[not_mockable]
///     fn not_mockable() { ... }
/// }
/// ```
/// - traits
///
/// ```ignore
/// #[mockable]
/// mod module {
///     #[not_mockable]
///     trait Trait {
///         fn not_mockable() { ... }
///     }
/// }
/// ```
/// - single default functions in traits
///
/// ```ignore
/// #[mockable]
/// trait Trait {
///     #[not_mockable]
///     fn not_mockable() { ... }
/// }
/// ```
///
/// # Indifferent to annotate
/// - items not made mockable by enclosing item
/// - any other items
#[proc_macro_attribute]
pub fn not_mockable(_: TokenStream, token_stream: TokenStream) -> TokenStream {
    token_stream
}

