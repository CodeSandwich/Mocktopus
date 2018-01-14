use display_delegate::DisplayDelegate;
//use lifetime_remover::remove_lifetimes_from_path;
use quote::ToTokens;
use std::fmt::{Error, Formatter};
use syn::{self, ArgCaptured, FnArg, Ident, Pat, PatIdent, PathSegment, Stmt};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Colon2};

const ARGS_REPLACEMENT_TUPLE_NAME: &str  = "__mocktopus_args_replacement_tuple__";
const MOCKTOPUS_EXTERN_CRATE_NAME: &str = "__mocktopus_extern_crate_inside_header__";

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
    pub fn build(&self, fn_ident: &Ident, fn_args: &Punctuated<FnArg, Comma>) -> Stmt {
        let header_str = format!(
r#"{{
    extern crate mocktopus as {mocktopus_crate};
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe (
            || {mocktopus_crate}::mocking::Mockable::call_mock(&{full_fn_name}, {args_tuple}))) {{
        Ok({mocktopus_crate}::mocking::MockResult::Continue({args_replacement_tuple})) => {args_replacement},
        Ok({mocktopus_crate}::mocking::MockResult::Return(result)) => return result,
        Err(unwind) => {{
            {args_forget}
            ::std::panic::resume_unwind(unwind);
        }}
    }}
}}"#,
            mocktopus_crate         = MOCKTOPUS_EXTERN_CRATE_NAME,
            full_fn_name            = DisplayDelegate::new(|f| self.write_full_fn_name(f, fn_ident)),
            args_tuple              = DisplayDelegate::new(|f| write_args_tuple(f, fn_args)),
            args_replacement_tuple  = ARGS_REPLACEMENT_TUPLE_NAME,
            args_replacement        = DisplayDelegate::new(|f| write_args_replacement(f, fn_args)),
            args_forget             = DisplayDelegate::new(|f| write_args_forget(f, fn_args)));
        syn::parse_str(&header_str).expect(error_msg!("generated header unparsable"))
    }

    fn write_full_fn_name(&self, f: &mut Formatter, fn_ident: &Ident) -> Result<(), Error> {
        match *self {
            FnHeaderBuilder::StaticFn => (),
            FnHeaderBuilder::StructImpl |
                FnHeaderBuilder::TraitDefault => write!(f, "Self::")?,
            FnHeaderBuilder::TraitImpl(_trait_path) => unimplemented!(),//write!(f, "<Self as {}>::", trait_name.as_ref())?,
        }
        write!(f, "{}", fn_ident.as_ref())
    }

//    fn write_trait_casting_name(f: &mut Formatter, path: &Path) -> Result<(), Error> {
//        let mut path_without_lifetimes = path.clone();
//        remove_lifetimes_from_path(&mut path_without_lifetimes);
//        let mut tokens = Tokens::new();
//        path_without_lifetimes.to_tokens(&mut tokens);
//        write!(f, "{}", tokens.as_str())
//    }
}

fn write_args_tuple<T>(f: &mut Formatter, fn_args: &Punctuated<FnArg, T>) -> Result<(), Error> {
    if fn_args.is_empty() {
        return write!(f, "()");
    }
    write!(f, "unsafe {{ (")?;
    for fn_arg_name in iter_fn_arg_names(fn_args) {
        write!(f, "::std::mem::replace({}::mocking_utils::as_mut(&{}), ::std::mem::uninitialized()), ",
               MOCKTOPUS_EXTERN_CRATE_NAME, fn_arg_name)?;
    }
    write!(f, ") }}")
}

fn write_args_replacement<T>(f: &mut Formatter, fn_args: &Punctuated<FnArg, T>) -> Result<(), Error> {
    if fn_args.is_empty() {
        return writeln!(f, "()");
    }
    writeln!(f, "unsafe {{")?;
    for (fn_arg_index, fn_arg_name) in iter_fn_arg_names(fn_args).enumerate() {
        writeln!(f, "::std::mem::replace({}::mocking_utils::as_mut(&{}), {}.{});",
                 MOCKTOPUS_EXTERN_CRATE_NAME, fn_arg_name, ARGS_REPLACEMENT_TUPLE_NAME, fn_arg_index)?;
    }
    writeln!(f, "}}")
}

fn write_args_forget<T>(f: &mut Formatter, fn_args: &Punctuated<FnArg, T>) -> Result<(), Error> {
    for fn_arg_name in iter_fn_arg_names(fn_args) {
        writeln!(f, "::std::mem::forget({});", fn_arg_name)?;
    }
    Ok(())
}

pub fn iter_fn_arg_names<'a, T>(input_args: &'a Punctuated<FnArg, T>) -> impl Iterator<Item = &'a str> {
    input_args.iter()
        .map(|fn_arg| match *fn_arg {
            FnArg::SelfRef(_) | FnArg::SelfValue(_) => "self",
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
            ) => ident.as_ref(),
            _ => panic!("{}: '{}'", error_msg!("invalid fn arg type"), fn_arg.clone().into_tokens()),
        })
}
