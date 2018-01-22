use display_delegate::display;
use lifetime_remover::remove_lifetimes_from_path;
use quote::{ToTokens};
use std::fmt::{Error, Formatter};
use syn::{self, ArgCaptured, FnArg, Ident, Pat, PatIdent, PathSegment, Stmt};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Colon2};

const MOCKTOPUS_CRATE_NAME:     &str = "__mocktopus_crate__";
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
    pub fn build(&self, fn_ident: &Ident, fn_args: &Punctuated<FnArg, Comma>) -> Stmt {
        let header_str = format!(
r#"{{
    extern crate mocktopus as {mocktopus};
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe (
            || {mocktopus}::mocking::Mockable::call_mock(&{full_fn_name}, {extract_args}))) {{
        Ok({mocktopus}::mocking::MockResult::Continue({args_to_continue})) => {restore_args},
        Ok({mocktopus}::mocking::MockResult::Return(result)) => return result,
        Err({unwind}) => {{
            {forget_args}
            ::std::panic::resume_unwind({unwind});
        }}
    }}
}}"#,
            mocktopus           = MOCKTOPUS_CRATE_NAME,
            full_fn_name        = display(|f| write_full_fn_name(f, self, fn_ident)),
            extract_args        = display(|f| write_extract_args(f, fn_args)),
            args_to_continue    = ARGS_TO_CONTINUE_NAME,
            restore_args        = display(|f| write_restore_args(f, fn_args)),
            forget_args         = display(|f| write_forget_args(f, fn_args)),
            unwind              = UNWIND_DATA_NAME);
        syn::parse_str(&header_str)
            .unwrap_or_else(|e| panic!("{}\ndetails: {}\nheader:\n{}",
                                       error_msg!("generated header unparsable"), e, &header_str))
    }
}

fn write_full_fn_name(f: &mut Formatter, builder: &FnHeaderBuilder, fn_ident: &Ident) -> Result<(), Error> {
    match *builder {
        FnHeaderBuilder::StaticFn               => (),
        FnHeaderBuilder::StructImpl |
        FnHeaderBuilder::TraitDefault           => write!(f, "Self::")?,
        FnHeaderBuilder::TraitImpl(ref path)    => write!(f, "<Self as {}>::", display(|f| write_trait_path(f, path)))?,
    }
    write!(f, "{}", fn_ident.as_ref())
}

fn write_trait_path<T: ToTokens + Clone>(f: &mut Formatter, trait_path: &Punctuated<PathSegment, T>) -> Result<(), Error> {
    let mut trait_path_without_lifetimes = trait_path.clone();
    remove_lifetimes_from_path(&mut trait_path_without_lifetimes);
    write!(f, "{}", trait_path_without_lifetimes.into_tokens())
}

fn write_extract_args<T>(f: &mut Formatter, fn_args: &Punctuated<FnArg, T>) -> Result<(), Error> {
    if fn_args.is_empty() {
        return write!(f, "()");
    }
    write!(f, "unsafe {{ (")?;
    for fn_arg_name in iter_fn_arg_names(fn_args) {
        write!(f, "::std::mem::replace({}::mocking_utils::as_mut(&{}), ::std::mem::uninitialized()), ",
               MOCKTOPUS_CRATE_NAME, fn_arg_name)?;
    }
    write!(f, ") }}")
}

fn write_restore_args<T>(f: &mut Formatter, fn_args: &Punctuated<FnArg, T>) -> Result<(), Error> {
    if fn_args.is_empty() {
        return writeln!(f, "()");
    }
    writeln!(f, "unsafe {{")?;
    for (fn_arg_index, fn_arg_name) in iter_fn_arg_names(fn_args).enumerate() {
        writeln!(f, "::std::mem::replace({}::mocking_utils::as_mut(&{}), {}.{});",
                 MOCKTOPUS_CRATE_NAME, fn_arg_name, ARGS_TO_CONTINUE_NAME, fn_arg_index)?;
    }
    writeln!(f, "}}")
}

fn write_forget_args<T>(f: &mut Formatter, fn_args: &Punctuated<FnArg, T>) -> Result<(), Error> {
    for fn_arg_name in iter_fn_arg_names(fn_args) {
        writeln!(f, "::std::mem::forget({});", fn_arg_name)?;
    }
    Ok(())
}

fn iter_fn_arg_names<'a, T>(input_args: &'a Punctuated<FnArg, T>) -> impl Iterator<Item = &'a str> {
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
