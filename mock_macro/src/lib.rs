#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use quote::{Tokens, ToTokens};
use std::str::FromStr;
use syn::{Abi, Block, Constness, ExprKind, FnArg, FnDecl, FunctionRetTy, Generics, Ident, Item, ItemKind, Unsafety};

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
            inject_fn(&item.ident, decl, unsafety, constness, abi, generics, block),
        ItemKind::Mod(Some(ref mut items)) => for item in items {inject_fns_in_item(item)},
//        ItemKind::Trait(ref mut unsafety, ref mut generics, ref mut ty_param_bound, ref mut items) => unimplemented!(),
//        ItemKind::Impl(ref mut unsafety, ref mut impl_polarity, ref mut generics, ref mut path, ref mut ty, ref mut items) => unimplemented!(),
        _ => (),
    }
}

fn inject_fn(ident: &Ident, fn_decl: &mut Box<FnDecl>, _: &mut Unsafety, constness: &mut Constness,
             abi: &mut Option<Abi>, generics: &mut Generics, block: &mut Box<Block>) {
    inject_fn_block(ident, &fn_decl.inputs, &fn_decl.output, block)
//    let args = fn_decl.as_mut().inputs;

    //DO NOT MOCK if any argument is `_`, is const
}

fn inject_fn_block(ident: &Ident, inputs: &Vec<FnArg>, output: &FunctionRetTy, block: &mut Box<Block>) {
//    let statements = &mut block.stmts;
//    let injection = Stmt::Local(Box::new())
}
//    let (input,) = match mock_injected_function.call_mock((input, )) {
//        MockResult::Continue(input) => input,
//        MockResult::Return(result) => return result,
//    };
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fake_test() {
//        let s = "match mock_injected_function.call_mock((input, )) {
//            MockResult::Continue(input) => input,
//            MockResult::Return(result) => return result,
//        }";
        let s = "{let (input,) = match mock_injected_function.call_mock((input, )) {
            MockResult::Continue(input) => input,
            MockResult::Return(result) => return result,
        };}";
//        let s = "let (input,) = match mock_injected_function.call_mock((input, )) {
//            MockResult::Continue(input) => input,
//            MockResult::Return(result) => return result,
//        };";
//        let expr = syn::parse_token_trees(s);
        let expr = syn::parse_expr(s).unwrap();
        let stmts = match expr.node {
            ExprKind::Block(_, block) => block.stmts,
            _ => unreachable!(),
        };
//        let local = Local {
//            pub pat: Box<Pat>,
//            pub ty: Option<Box<Ty>>,
//
//            /// Initializer expression to set the value, if any
//            pub init: Option<Box<Expr>>,
//            pub attrs: Vec<Attribute>,
//        }
//        let stmt = Stmt::Local(Box::new(local));
        let mut tokens = Tokens::new();
        for stmt in stmts {
            stmt.to_tokens(&mut tokens);
        }

        println!("{}", tokens.as_str());
        panic!();
    }
}
