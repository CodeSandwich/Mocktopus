#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use quote::{Tokens, ToTokens};
use std::mem;
use std::str::FromStr;
use syn::{BindingMode, Block, Constness, ExprKind, FnArg, Generics, Ident, ImplItem, ImplItemKind, Item, ItemKind,
        MethodSig, Mutability, Pat, Path, Stmt, Ty};

macro_rules! error_msg {
    ($msg:expr) => { concat!("Mocktopus internal error: ", $msg) }
}

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

fn inject_impl(_generics: &Generics, path: Option<&Path>, _ty: &Box<Ty>, items: &mut Vec<ImplItem>) {
//    println!("PATH\n{:#?}\nTY\n{:#?}\nITEMS\n{:#?}", path, ty, items);
    if path.is_some() {
        return; // no trait support yet
    }
    for item in items {
        if let ImplItemKind::Method(
            MethodSig {
                unsafety: _,
                constness: ref constness_ref,
                abi: _,
                decl: ref mut decl_ref,
                generics: ref generics_ref },
            ref mut block) = item.node {
            match decl_ref.inputs.get(0) { // no non-static methods support yet
                Some(&FnArg::SelfRef(..)) | Some(&FnArg::SelfValue(..)) => continue,
                _ => (),
            };
            let builder = HeaderStmtBuilder::default()
                .set_is_method(true)
                .set_fn_name(&item.ident)
                .set_fn_generics(generics_ref);
            inject_fn(builder, &mut decl_ref.inputs, constness_ref, block);
        }
    }
}

//    pub struct MethodSig {
//        pub unsafety: Unsafety,
//        pub constness: Constness,
//        pub abi: Option<Abi>,
//        pub decl: FnDecl,
//        pub generics: Generics,
//    }


//    pub struct ImplItem {
//        pub ident: Ident,
//        pub vis: Visibility,
//        pub defaultness: Defaultness,
//        pub attrs: Vec<Attribute>,
//        pub node: ImplItemKind,
//    }


    // impl [<path> for] ty {
    //      <items>
    // }

fn inject_static_fn(ident: &Ident, inputs: &mut Vec<FnArg>, constness: &Constness, generics: &Generics, block: &mut Box<Block>) {
    let builder = HeaderStmtBuilder::default()
        .set_fn_name(ident)
        .set_fn_generics(generics);
    inject_fn(builder, inputs, constness, block);
}

fn inject_fn(builder: HeaderStmtBuilder, inputs: &mut Vec<FnArg>, constness: &Constness, block: &mut Block) {
    if *constness == Constness::Const {
        return
    }
    unignore_fn_args(inputs);
    let header_stmts = builder.set_input_args(inputs).build();
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

#[derive(Default)]
struct HeaderStmtBuilder<'a> {
    is_method: bool,
    fn_ident: Option<&'a Ident>,
    fn_generics: Option<&'a Generics>,
    input_args: Option<&'a Vec<FnArg>>,

}

impl<'a> HeaderStmtBuilder<'a> {
    pub fn set_is_method(mut self, is_method: bool) -> Self {
        self.is_method = is_method;
        self
    }

    pub fn set_fn_name(mut self, fn_ident: &'a Ident) -> Self {
        self.fn_ident = Some(fn_ident);
        self
    }

    pub fn set_fn_generics(mut self, fn_generics: &'a Generics) -> Self {
        self.fn_generics = Some(fn_generics);
        self
    }
    pub fn set_input_args(mut self, inputs: &'a Vec<FnArg>) -> Self {
        self.input_args = Some(inputs);
        self
    }

    pub fn build(&self) -> Vec<Stmt> {
        let header_str = self.create_header_block_str();
        let header_expr = syn::parse_expr(&header_str).expect(error_msg!("generated header unparsable"));
        match header_expr.node {
            ExprKind::Block(_, block) => block.stmts,
            _ => panic!(error_msg!("generated header not a block")),
        }
    }

    fn create_header_block_str(&self) -> String {
        format!(
            r#"{{
            let ({0}) = {{
                use mocktopus::*;
                match Mockable::call_mock(&{1}, (({0}))) {{
                    MockResult::Continue(input) => input,
                    MockResult::Return(result) => return result,
                }}
            }};
        }}"#, self.create_input_args_str(), self.create_full_fn_name_str())
    }

    fn create_input_args_str(&self) -> String {
        let mut input_args_str = String::new();
        for input_arg in self.input_args.expect(error_msg!("inputs not set")) {
            match *input_arg {
                FnArg::SelfRef(_, _) | FnArg::SelfValue(_) => input_args_str.push_str("self"),
                FnArg::Captured(Pat::Ident(_, ref ident, None), _) => input_args_str.push_str(ident.as_ref()),
                _ => panic!(error_msg!("invalid function input '{:?}'"), input_arg),
            };
            input_args_str.push_str(", ");
        };
        input_args_str
    }

    fn create_full_fn_name_str(&self) -> String {
        format!("{}{}{}",
                if self.is_method { "Self::" } else { "" },
                self.fn_ident.expect(error_msg!("fn name not set")).as_ref(),
                Self::create_generics_str(self.fn_generics))
    }

    fn create_generics_str(generics_opt: Option<&Generics>) -> String {
        let generics = match generics_opt {
            Some(generics) if !generics.ty_params.is_empty() => generics,
            _ => return String::new(),
        };
        let mut generics_str = "::<".to_string();
        for ty_param in &generics.ty_params {
            generics_str.push_str(&ty_param.ident.as_ref());
            generics_str.push(',');
        }
        generics_str.push('>');
        generics_str
    }
}
