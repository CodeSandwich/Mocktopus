#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use quote::{Tokens, ToTokens};
use std::str::FromStr;
use syn::{Abi, BindingMode, Block, Constness, ExprKind, FnArg, FnDecl, FunctionRetTy, Generics, Ident, Item, ItemKind,
          Mutability, Pat, Unsafety};

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
        ItemKind::Fn(ref mut decl, ref mut unsafety, ref constness, _, ref generics, ref mut block) =>
            inject_fn(&item.ident, &mut decl.inputs, constness, generics, block),
        ItemKind::Mod(Some(ref mut items)) => for item in items {inject_fns_in_item(item)},
//        ItemKind::Trait(ref mut unsafety, ref mut generics, ref mut ty_param_bound, ref mut items) => unimplemented!(),
//        ItemKind::Impl(ref mut unsafety, ref mut impl_polarity, ref mut generics, ref mut path, ref mut ty, ref mut items) => unimplemented!(),
        _ => (),
    }
}

fn inject_fn(ident: &Ident, inputs: &mut Vec<FnArg>, constness: &Constness, _generics: &Generics, block: &mut Box<Block>) {
    if *constness == Constness::Const {
        return
    }
    unignore_fn_args(inputs);
    inject_fn_block(ident, inputs, block)
}

fn unignore_fn_args(inputs: &mut Vec<FnArg>) {
    for i in 0..inputs.len() {
        let unignored = match inputs[i] {
            FnArg::Captured(Pat::Wild, ref ty) =>
                FnArg::Captured(
                    Pat::Ident(
                        BindingMode::ByValue(
                            Mutability::Immutable),
                        Ident::from(format!("__mock_unignored_argument_{}__", i)),
                        None),
                    ty.clone()),
            _ => continue,
        };
        inputs[i] = unignored;
    }
}

fn inject_fn_block(ident: &Ident, inputs: &Vec<FnArg>, block: &mut Box<Block>) {
    let arg_names = args_to_names(inputs);
    let header_str = format!(
        "{{\
            let ({0}) = {{\
                use mock_trait::{{MockResult, MockTrait}};\
                match {1}.call_mock(({0})) {{\
                    MockResult::Continue(input) => input,\
                    MockResult::Return(result) => return result,\
                }}\
            }};\
        }}", arg_names, &ident);
    let header_expr = syn::parse_expr(&header_str).unwrap();
    let header_stmts = match header_expr.node {
        ExprKind::Block(_, block) => block.stmts,
        _ => unreachable!(),
    };
    block.stmts.extend(header_stmts);


//    let mut tokens = Tokens::new();
//    for stmt in header_stmts {
//        stmt.to_tokens(&mut tokens);
//    }
//    println!("{}", tokens.as_str());
}

fn args_to_names(inputs: &Vec<FnArg>) -> String {
    inputs.iter()
        .fold(String::new(), |mut result, input| {
            match *input {
                FnArg::SelfRef(_, _) | FnArg::SelfValue(_) => result.push_str("self"),
                FnArg::Captured(Pat::Ident(_, ref ident, None), _) => result.push_str(ident.as_ref()),
                _ => panic!("Invalid function input '{:?}'", input),
            };
            result.push_str(", ");
            result
        })
}

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
