#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use quote::{Tokens, ToTokens};
use std::mem;
use std::str::FromStr;
use syn::{BindingMode, Block, Constness, ExprKind, FnArg, Generics, Ident, ImplItem, Item, ItemKind,
        Mutability, Pat, Path, Ty};

#[proc_macro_attribute]
pub fn inject_mocks(_: TokenStream, token_stream: TokenStream) -> TokenStream {
    let in_string = token_stream.to_string();
    let mut parsed = match syn::parse_item(&in_string) {
        Ok(parsed) => parsed,
        Err(_) => return token_stream,
    };
    inject_item(&mut parsed);
    let mut tokens = Tokens::new();
    parsed.to_tokens(&mut tokens);
    let out_string = tokens.as_str();
    let out_token_stream = TokenStream::from_str(out_string).unwrap();
    out_token_stream
}

fn inject_item(item: &mut Item) {
    match item.node {
        ItemKind::Mod(ref mut items_opt) =>
            inject_mod(items_opt.as_mut()),
        ItemKind::Fn(ref mut decl, _, ref constness, _, ref generics, ref mut block) =>
            inject_static_fn(&item.ident, &mut decl.inputs, constness, generics, block),
        ItemKind::Impl(_, _, ref generics, ref path, ref ty, ref mut items) =>
            inject_impl(generics, path.as_ref(), ty, items),
        //        ItemKind::Trait(ref mut unsafety, ref mut generics, ref mut ty_param_bound, ref mut items) => unimplemented!(),
        _ => (),
    }
}

fn inject_mod(items_opt: Option<&mut Vec<Item>>) {
    if let Some(items) = items_opt {
        for item in items {
            inject_item(item)
        }
    }
}

fn inject_impl(_generics: &Generics, path: Option<&Path>, ty: &Box<Ty>, items: &mut Vec<ImplItem>) {
    println!("PATH\n{:#?}\nTY\n{:#?}\nITEMS\n{:#?}", path, ty, items);
    if path.is_some() {
        return; // no trait support yet
    }
//    if let Ty::Path(None, Path { })


    // impl [<path> for] ty {
    //      <items>
    // }


}

fn inject_static_fn(ident: &Ident, inputs: &mut Vec<FnArg>, constness: &Constness, generics: &Generics, block: &mut Box<Block>) {
    if *constness == Constness::Const {
        return
    }
    let mut fn_name = ident.to_string();
    append_generics(&mut fn_name, generics);
    inject_fn(&fn_name, inputs, block);
}

fn inject_fn(fn_name: &str, inputs: &mut Vec<FnArg>, block: &mut Box<Block>) {
    let original_arg_names = args_to_names(inputs);
    unignore_fn_args(inputs);
    let unignored_arg_names = args_to_names(inputs);
    let header_str = format!(
        r#"{{
            let ({}) = {{
                use mocktopus::*;
                match Mockable::call_mock(&{}, (({}))) {{
                    MockResult::Continue(input) => input,
                    MockResult::Return(result) => return result,
                }}
            }};
        }}"#, original_arg_names, fn_name, unignored_arg_names);
    let header_expr = syn::parse_expr(&header_str).unwrap();
    let header_stmts = match header_expr.node {
        ExprKind::Block(_, block) => block.stmts,
        _ => unreachable!(),
    };
    //    let mut tokens = Tokens::new();
    //    for stmt in &header_stmts {
    //        stmt.to_tokens(&mut tokens);
    //    }
    //    println!("{}", tokens.as_str());
    let mut body_stmts = mem::replace(&mut block.stmts, header_stmts);
    block.stmts.append(&mut body_stmts);
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

fn args_to_names(inputs: &Vec<FnArg>) -> String {
    inputs.iter()
        .fold(String::new(), |mut result, input| {
            match *input {
                FnArg::SelfRef(_, _) | FnArg::SelfValue(_) => result.push_str("self"),
                FnArg::Captured(Pat::Wild, _) => result.push_str("_"),
                FnArg::Captured(Pat::Ident(_, ref ident, None), _) => result.push_str(ident.as_ref()),
                _ => panic!("Invalid function input '{:?}'", input),
            };
            result.push_str(", ");
            result
        })
}

fn append_generics(fn_name: &mut String, generics: &Generics) {
    if generics.ty_params.is_empty() {
        return
    }
    fn_name.push_str("::<");
    for ty_param in &generics.ty_params {
        fn_name.push_str(&ty_param.ident.as_ref());
        fn_name.push(',');
    }
    fn_name.push('>');
}
