use display_delegate::DisplayDelegate;
//use lifetime_remover::remove_lifetimes_from_path;
use std::fmt::{Error, Formatter};
use syn::{self, ArgCaptured, FnArg, Ident, Pat, PatIdent, Stmt};
use syn::punctuated::Punctuated;
use syn::token::{Comma};

const ARGS_REPLACEMENT_TUPLE_NAME: &str  = "__mocktopus_args_replacement_tuple__";
const MOCKTOPUS_EXTERN_CRATE_NAME: &str = "__mocktopus_extern_crate_inside_header__";

macro_rules! error_msg {
    ($msg:expr) => { concat!("Mocktopus internal error: ", $msg) }
}

#[derive(Clone, Default)]
pub struct HeaderBuilder<'a> {
//    is_method: bool,
//    trait_path: Option<&'a Path>,
    fn_ident: Option<&'a str>,
    fn_args_names: Option<Vec<&'a str>>,
}

impl<'a> HeaderBuilder<'a> {
//    pub fn set_is_method(mut self) -> Self {
//        self.is_method = true;
//        self
//    }
//
//    pub fn set_trait_path(mut self, trait_path: Option<&'a Path>) -> Self {
//        self.trait_path = trait_path;
//        self
//    }

    pub fn set_fn_name(mut self, fn_ident: &'a Ident) -> Self {
        self.fn_ident = Some(fn_ident.as_ref());
        self
    }

    pub fn set_input_args(mut self, input_args: &'a Punctuated<FnArg, Comma>) -> Self {
        let fn_args_names = input_args.iter()
            .map(|a| match *a {
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
                _ => panic!(error_msg!("invalid fn arg type")),
            })
            .collect();
        self.fn_args_names = Some(fn_args_names);
        self
    }

    pub fn build(&self) -> Stmt {
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
            full_fn_name            = DisplayDelegate::new(|f| self.write_full_fn_name(f)),
            args_tuple              = DisplayDelegate::new(|f| self.write_args_tuple(f)),
            args_replacement_tuple  = ARGS_REPLACEMENT_TUPLE_NAME,
            args_replacement        = DisplayDelegate::new(|f| self.write_args_replacement(f)),
            args_forget             = DisplayDelegate::new(|f| self.write_args_forget(f)));
        /*let header_stmt: Stmt = */syn::parse_str(&header_str).expect(error_msg!("generated header unparsable"))
//        match header_expr.node {
//            ExprKind::Block(_, block) => block.stmts,
//            _ => panic!(error_msg!("generated header is not a block")),
//        }
    }

    fn write_full_fn_name(&self, f: &mut Formatter) -> Result<(), Error> {
//        match (self.is_method, self.trait_path) {
//            (true, Some(path)) => write!(f, "<Self as {}>::",
//                                         DisplayDelegate::new(|f| Self::write_trait_casting_name(f, path)))?,
//            (true, None) => write!(f, "Self::")?,
//            (false, Some(_)) => panic!(error_msg!("trait path set on non-method")),
//            (false, None) => (),
//        };
        write!(f, "{}", self.fn_ident.expect(error_msg!("fn name not set")))
    }

//    fn write_trait_casting_name(f: &mut Formatter, path: &Path) -> Result<(), Error> {
//        let mut path_without_lifetimes = path.clone();
//        remove_lifetimes_from_path(&mut path_without_lifetimes);
//        let mut tokens = Tokens::new();
//        path_without_lifetimes.to_tokens(&mut tokens);
//        write!(f, "{}", tokens.as_str())
//    }

    fn write_args_tuple(&self, f: &mut Formatter) -> Result<(), Error> {
        let fn_args_names = self.fn_args_names.as_ref().expect(error_msg!("fn_arg_names not set"));
        if fn_args_names.is_empty() {
            return write!(f, "()");
        }
        write!(f, "unsafe {{ (")?;
        for fn_arg_name in fn_args_names {
            write!(f, "::std::mem::replace({}::mocking_utils::as_mut(&{}), ::std::mem::uninitialized()), ",
                   MOCKTOPUS_EXTERN_CRATE_NAME, fn_arg_name)?;
        }
        write!(f, ") }}")
    }

    fn write_args_replacement(&self, f: &mut Formatter) -> Result<(), Error> {
        let fn_args_names = self.fn_args_names.as_ref().expect(error_msg!("fn_arg_names not set"));
        if fn_args_names.is_empty() {
            return writeln!(f, "()");
        }
        writeln!(f, "unsafe {{")?;
        for (fn_arg_index, fn_arg_name) in fn_args_names.iter().enumerate() {
            writeln!(f, "::std::mem::replace({}::mocking_utils::as_mut(&{}), {}.{});",
                   MOCKTOPUS_EXTERN_CRATE_NAME, fn_arg_name, ARGS_REPLACEMENT_TUPLE_NAME, fn_arg_index)?;
        }
        writeln!(f, "}}")
    }

    fn write_args_forget(&self, f: &mut Formatter) -> Result<(), Error> {
        let fn_args_names = self.fn_args_names.as_ref().expect(error_msg!("fn_arg_names not set"));
        for fn_arg_name in fn_args_names {
            writeln!(f, "::std::mem::forget({});", fn_arg_name)?;
        }
        Ok(())
    }
}
