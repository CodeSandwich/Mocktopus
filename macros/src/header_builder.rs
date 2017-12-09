use display_delegate::DisplayDelegate;
use lifetime_remover::remove_lifetimes_from_path;
use std::fmt::{Error, Formatter};
use syn::{self, ExprKind, FnArg, Ident, Mutability, Pat, Path, Stmt};
use quote::{Tokens, ToTokens};

const ARG_REPLACEMENT_TUPLE_NAME: &str = "replacement";

macro_rules! error_msg {
    ($msg:expr) => { concat!("Mocktopus internal error: ", $msg) }
}

#[derive(Clone, Default)]
pub struct HeaderBuilder<'a> {
    is_method: bool,
    trait_path: Option<&'a Path>,
    fn_ident: Option<&'a Ident>,
    self_arg: Option<&'a FnArg>,
    non_self_args: Option<&'a [FnArg]>,
}

impl<'a> HeaderBuilder<'a> {
    pub fn set_is_method(mut self) -> Self {
        self.is_method = true;
        self
    }

    pub fn set_trait_path(mut self, trait_path: Option<&'a Path>) -> Self {
        self.trait_path = trait_path;
        self
    }

    pub fn set_fn_name(mut self, fn_ident: &'a Ident) -> Self {
        self.fn_ident = Some(fn_ident);
        self
    }

    pub fn set_input_args(mut self, inputs: &'a Vec<FnArg>) -> Self {
        match inputs.first() {
            self_arg @ Some(&FnArg::SelfRef(_, _)) | self_arg @ Some(&FnArg::SelfValue(_)) => {
                self.self_arg = self_arg;
                self.non_self_args = Some(&inputs[1..]);
            },
            _ => {
                self.non_self_args = Some(inputs.as_slice());
            },
        };
        self
    }

    pub fn build(&self) -> Vec<Stmt> {
        let header_str = format!(
            r#"{{
                let ({non_self_args}) = {block_unsafety} {{
                    extern crate mocktopus;
                    match mocktopus::mocking::Mockable::call_mock(&{full_fn_name}, ({self_arg}{non_self_args})) {{
                        mocktopus::mocking::MockResult::Continue({arg_replacement_tuple}) => {{
                            {self_arg_replacement}
                            {non_self_arg_return}
                        }},
                        mocktopus::mocking::MockResult::Return(result) => return result,
                    }}
                }};
            }}"#,
            block_unsafety          = DisplayDelegate::new(|f| self.write_block_unsafety(f)),
            full_fn_name            = DisplayDelegate::new(|f| self.write_full_fn_name(f)),
            self_arg                = DisplayDelegate::new(|f| self.write_self_arg(f)),
            non_self_args           = DisplayDelegate::new(|f| self.write_non_self_args(f)),
            arg_replacement_tuple   = ARG_REPLACEMENT_TUPLE_NAME,
            self_arg_replacement    = DisplayDelegate::new(|f| self.write_self_replacement(f)),
            non_self_arg_return     = DisplayDelegate::new(|f| self.write_non_self_return(f)));
        let header_expr = syn::parse_expr(&header_str).expect(error_msg!("generated header unparsable"));
        match header_expr.node {
            ExprKind::Block(_, block) => block.stmts,
            _ => panic!(error_msg!("generated header is not a block")),
        }
    }

    fn write_block_unsafety(&self, formatter: &mut Formatter) -> Result<(), Error> {
        match self.self_arg {
            Some(_) => write!(formatter, "unsafe"),
            None => Ok(()),
        }
    }

    fn write_full_fn_name(&self, formatter: &mut Formatter) -> Result<(), Error> {
        match (self.is_method, self.trait_path) {
            (true, Some(path)) => write!(formatter, "<Self as {}>::",
                                         DisplayDelegate::new(|f| Self::write_trait_casting_name(f, path)))?,
            (true, None) => write!(formatter, "Self::")?,
            (false, Some(_)) => panic!(error_msg!("trait path set on non-method")),
            (false, None) => (),
        };
        write!(formatter, "{}", self.fn_ident.expect(error_msg!("fn name not set")).as_ref())
    }

    fn write_trait_casting_name(formatter: &mut Formatter, path: &Path) -> Result<(), Error> {
        let mut path_without_lifetimes = path.clone();
        remove_lifetimes_from_path(&mut path_without_lifetimes);
        let mut tokens = Tokens::new();
        path_without_lifetimes.to_tokens(&mut tokens);
        write!(formatter, "{}", tokens.as_str())
    }

    fn write_self_arg(&self, formatter: &mut Formatter) -> Result<(), Error> {
        match self.self_arg {
            Some(_) => write!(formatter, "std::mem::replace(&mut {}, std::mem::uninitialized()), ",
                              DisplayDelegate::new(|f| self.write_mut_self_acqusition(f))),
            None => Ok(()),
        }
    }

    fn write_non_self_args(&self, formatter: &mut Formatter) -> Result<(), Error> {
        for arg in self.non_self_args.expect(error_msg!("passed inputs not set")) {
            match *arg {
                FnArg::Captured(Pat::Ident(_, ref ident, None), _) => write!(formatter, "{}, ", ident.as_ref())?,
                _ => panic!(error_msg!("invalid function input '{:?}'"), arg),
            };
        };
        Ok(())
    }

    fn write_self_replacement(&self, formatter: &mut Formatter) -> Result<(), Error> {
        match self.self_arg {
            Some(_) => write!(formatter, "{} = {}.0;", DisplayDelegate::new(|f| self.write_mut_self_acqusition(f)),
                              ARG_REPLACEMENT_TUPLE_NAME),
            None => Ok(()),
        }
    }

    fn write_non_self_return(&self, formatter: &mut Formatter) -> Result<(), Error> {
        match self.self_arg {
            Some(_) => {
                write!(formatter, "(")?;
                for i in 0..self.non_self_args.expect(error_msg!("returned inputs not set")).len() {
                    write!(formatter, "{}.{}, ", ARG_REPLACEMENT_TUPLE_NAME, i + 1)?;
                }
                write!(formatter, ")")
            },
            None => write!(formatter, "{}", ARG_REPLACEMENT_TUPLE_NAME)
        }
    }

    fn write_mut_self_acqusition(&self, formatter: &mut Formatter) -> Result<(), Error> {
        write!(formatter, "*(&self as *const {0} as *mut {0})", DisplayDelegate::new(|f| self.write_self_type(f)))
    }

    fn write_self_type(&self, formatter: &mut Formatter) -> Result<(), Error> {
        match self.self_arg {
            Some(&FnArg::SelfRef(_, Mutability::Immutable)) => write!(formatter, "&Self"),
            Some(&FnArg::SelfRef(_, Mutability::Mutable)) => write!(formatter, "&mut Self"),
            Some(&FnArg::SelfValue(_)) => write!(formatter, "Self"),
            _ => panic!(error_msg!("invalid self arg: '{:?}'"), self.self_arg),
        }
    }
}
