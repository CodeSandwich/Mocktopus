use crate::display_delegate::display;
use proc_macro2::{Group, Span, TokenTree};
use quote::{ToTokens};
use std::fmt::{Error, Formatter};
use syn::{self, ArgCaptured, Block, Expr, ExprVerbatim, FnArg, FnDecl, GenericParam, Ident, Pat, PatIdent, PathSegment,
          Stmt};
use syn::punctuated::Punctuated;
use syn::token::{Colon2, Semi};

const MOCKTOPUS_CRATE_NAME:     &str = "__mocktopus_crate__";
const STD_CRATE_NAME:           &str = "__mocktopus_std__";
const ARGS_TO_CONTINUE_NAME:    &str = "__mocktopus_args_to_continue__";
const UNWIND_DATA_NAME:         &str = "__mocktopus_unwind_data__";

macro_rules! error_msg {
    ($msg:expr) => { concat!("Mocktopus internal error: ", $msg) }
}

pub enum FnHeaderBuilder<'a> {
    StaticFn,
    StructImpl,
    TraitDefault,
    TraitImpl(&'a Punctuated<PathSegment, Colon2>),
}

impl<'a> FnHeaderBuilder<'a> {
    pub fn build(&self, fn_ident: &Ident, fn_decl: &FnDecl, fn_block_span: Span) -> Stmt {
        let fn_args = &fn_decl.inputs;
        let header_str = format!(
r#"{{
    extern crate mocktopus as {mocktopus};
    extern crate std as {std_crate};
    match {std_crate}::panic::catch_unwind({std_crate}::panic::AssertUnwindSafe (
            || {mocktopus}::mocking::Mockable::call_mock(&{full_fn_name}, {extract_args}))) {{
        Ok({mocktopus}::mocking::MockResult::Continue({args_to_continue})) => {restore_args},
        Ok({mocktopus}::mocking::MockResult::Return(result)) => {{
            {forget_args}
            let returned = unsafe {{ {std_crate}::mem::transmute_copy(&result) }};
            {std_crate}::mem::forget(result);
            return returned;
        }},
        Err({unwind}) => {{
            {forget_args}
            {std_crate}::panic::resume_unwind({unwind});
        }},
    }}
}}"#,
        mocktopus           = MOCKTOPUS_CRATE_NAME,
        std_crate           = STD_CRATE_NAME,
        full_fn_name        = display(|f| write_full_fn_name(f, self, fn_ident, fn_decl)),
        extract_args        = display(|f| write_extract_args(f, fn_args)),
        args_to_continue    = ARGS_TO_CONTINUE_NAME,
        restore_args        = display(|f| write_restore_args(f, fn_args)),
        forget_args         = display(|f| write_forget_args(f, fn_args)),
        unwind              = UNWIND_DATA_NAME);
        let header_block = syn::parse_str::<Block>(&header_str)
            .expect(error_msg!("generated header unparsable"));
        create_call_site_spanned_stmt(header_block, fn_block_span)
    }
}

fn create_call_site_spanned_stmt(block: Block, span: Span) -> Stmt {
    let token_stream = block.into_token_stream()
        .into_iter()
        .map(|tt| make_token_tree_span_call_site(tt, span))
        .collect();
    Stmt::Semi(
        Expr::Verbatim(
            ExprVerbatim {
                tts: token_stream,
            }
        ),
        Semi([span])
    )
}

fn make_token_tree_span_call_site(mut token_tree: TokenTree, span: Span) -> TokenTree {
    token_tree.set_span(span);
    if let TokenTree::Group(ref mut group) = token_tree {
        let tokens = group.stream().into_iter()
            .map(|tt| make_token_tree_span_call_site(tt, span))
            .collect();
        *group = Group::new(group.delimiter(), tokens);
    }
    token_tree
}

fn write_full_fn_name(f: &mut Formatter, builder: &FnHeaderBuilder, fn_ident: &Ident, fn_decl: &FnDecl)
        -> Result<(), Error> {
    match *builder {
        FnHeaderBuilder::StaticFn               => (),
        FnHeaderBuilder::StructImpl |
        FnHeaderBuilder::TraitDefault           => write!(f, "Self::")?,
        FnHeaderBuilder::TraitImpl(ref path)    => write!(f, "<Self as {}>::", display(|f| write_trait_path(f, path)))?,
    }
    write!(f, "{}::<{}>", fn_ident, display(|f| write_fn_generics(f, fn_decl)))
}

fn write_trait_path<T: ToTokens + Clone>(f: &mut Formatter, path: &Punctuated<PathSegment, T>) -> Result<(), Error> {
    write!(f, "{}", path.into_token_stream())
}

fn write_fn_generics(f: &mut Formatter, fn_decl: &FnDecl) -> Result<(), Error> {
    fn_decl.generics.params.iter()
        .filter_map(get_generic_param_name)
        .map(|param| write!(f, "{},", param))
        .collect()
}

fn get_generic_param_name(param: &GenericParam) -> Option<String> {
    match *param {
        GenericParam::Type(ref type_param) => Some(type_param.ident.to_string()),
        _ => None,
    }
}

fn write_extract_args<T>(f: &mut Formatter, fn_args: &Punctuated<FnArg, T>) -> Result<(), Error> {
    if fn_args.is_empty() {
        return write!(f, "()");
    }
    write!(f, "unsafe {{ (")?;
    for fn_arg_name in iter_fn_arg_names(fn_args) {
        write!(f, "{}::mem::replace(&mut *(&{} as *const _ as *mut _), {}::mem::uninitialized()), ",
               STD_CRATE_NAME, fn_arg_name, STD_CRATE_NAME)?;
    }
    write!(f, ") }}")
}

fn write_restore_args<T>(f: &mut Formatter, fn_args: &Punctuated<FnArg, T>) -> Result<(), Error> {
    if fn_args.is_empty() {
        return writeln!(f, "()");
    }
    writeln!(f, "unsafe {{")?;
    for (fn_arg_index, fn_arg_name) in iter_fn_arg_names(fn_args).enumerate() {
        writeln!(f, "{}::mem::forget({}::mem::replace({}::mocking_utils::as_mut(&{}), {}.{}));",
            STD_CRATE_NAME, STD_CRATE_NAME, MOCKTOPUS_CRATE_NAME, fn_arg_name, ARGS_TO_CONTINUE_NAME, fn_arg_index)?;
    }
    writeln!(f, "}}")
}

fn write_forget_args<T>(f: &mut Formatter, fn_args: &Punctuated<FnArg, T>) -> Result<(), Error> {
    for fn_arg_name in iter_fn_arg_names(fn_args) {
        writeln!(f, "{}::mem::forget({});", STD_CRATE_NAME, fn_arg_name)?;
    }
    Ok(())
}

fn iter_fn_arg_names<'a, T>(input_args: &'a Punctuated<FnArg, T>) -> impl Iterator<Item = String> + 'a {
    input_args.iter()
        .map(|fn_arg| match *fn_arg {
            FnArg::SelfRef(_) | FnArg::SelfValue(_) => "self".to_string(),
            FnArg::Captured(
                ArgCaptured {
                    pat: Pat::Ident(
                        PatIdent {
                            ref ident,
                            subpat: None,
                            ..
                        }
                    ),
                    ..
                }
            ) => ident.to_string(),
            _ => panic!("{}: '{}'", error_msg!("invalid fn arg type"), fn_arg.clone().into_token_stream()),
        })
}
