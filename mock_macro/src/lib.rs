#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use quote::{Tokens, ToTokens};
use std::str::FromStr;
use syn::{Abi, Block, Constness, FnDecl, Generics, Item, ItemKind, Unsafety};

#[proc_macro_attribute]
pub fn mock_it(_: TokenStream, token_stream: TokenStream) -> TokenStream {
    let in_string = token_stream.to_string();
    let mut parsed = syn::parse_item(&in_string).unwrap();
    inject_fns_in_item(&mut parsed);
    let mut tokens = Tokens::new();
    parsed.to_tokens(&mut tokens);
    let out_string = tokens.as_str();
    let out_token_stream = TokenStream::from_str(out_string).unwrap();
    out_token_stream
}

fn inject_fns_in_item(item: &mut Item) {
    match item.node {
        ItemKind::Fn(ref mut decl, ref mut unsafety, ref mut constness, ref mut abi, ref mut generics, ref mut block) =>
            inject_fn(decl, unsafety, constness, abi, generics, block),
        ItemKind::Mod(Some(ref mut items)) => for item in items {inject_fns_in_item(item)},
//        ItemKind::Trait(ref mut unsafety, ref mut generics, ref mut ty_param_bound, ref mut items) => unimplemented!(),
//        ItemKind::Impl(ref mut unsafety, ref mut impl_polarity, ref mut generics, ref mut path, ref mut ty, ref mut items) => unimplemented!(),
//        ItemKind::Mac(ref mut mac) => unimplemented!(),
        _ => (),
    }
}

fn inject_fn(fn_decl: &mut Box<FnDecl>, unsafety: &mut Unsafety, constness: &mut Constness,
             abi: &mut Option<Abi>, generics: &mut Generics, block: &mut Box<Block>) {
//    let args = fn_decl.as_mut().inputs;

    //DO NOT MOCK if any argument is `_`
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fake_test() {
    }
}
