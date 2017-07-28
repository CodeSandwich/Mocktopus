use display_delegate::DisplayDelegate;
use std::fmt::{Error, Formatter};
use syn::{ExprKind, FnArg, Ident, Mutability, Pat, self, Stmt};

const ARG_REPLACEMENT_TUPLE_NAME: &str = "replacement";

#[derive(Default)]
pub struct HeaderBuilder<'a> {
    is_method: bool,
    fn_ident: Option<&'a Ident>,
    self_arg: Option<&'a FnArg>,
    non_self_args: Option<&'a [FnArg]>,
}

impl<'a> HeaderBuilder<'a> {
    pub fn set_is_method(mut self, is_method: bool) -> Self {
        self.is_method = is_method;
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
                self.self_arg = None;
                self.non_self_args = Some(inputs.as_slice());
            },
        };
        self
    }

    pub fn build(&self) -> String {
        format!(
            r#"{{
            let ({non_self_args}) = {block_unsafety} {{
                match mocktopus::Mockable::call_mock(&{full_fn_name}, ({self_arg}{non_self_args})) {{
                    mocktopus::MockResult::Continue({arg_replacement_tuple}) => {{
                        {self_arg_replacement}
                        {non_self_arg_return}
                    }},
                    mocktopus::MockResult::Return(result) => return result,
                }}
            }};
        }}"#,
            block_unsafety          = DisplayDelegate::new(|f| self.write_block_unsafety(f)),
            full_fn_name            = DisplayDelegate::new(|f| self.write_full_fn_name(f)),
            self_arg                = DisplayDelegate::new(|f| self.write_self_arg(f)),
            non_self_args           = DisplayDelegate::new(|f| self.write_non_self_args(f)),
            arg_replacement_tuple   = ARG_REPLACEMENT_TUPLE_NAME,
            self_arg_replacement    = DisplayDelegate::new(|f| self.write_self_replacement(f)),
            non_self_arg_return     = DisplayDelegate::new(|f| self.write_non_self_return(f)))
    }

    fn write_block_unsafety(&self, formatter: &mut Formatter) -> Result<(), Error> {
        match self.self_arg {
            Some(_) => write!(formatter, "unsafe"),
            None => Ok(()),
        }
    }

    fn write_full_fn_name(&self, formatter: &mut Formatter) -> Result<(), Error> {
        if self.is_method {
            write!(formatter, "Self::")?;
        }
        write!(formatter, "{}", self.fn_ident.expect(error_msg!("fn name not set")).as_ref())
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
        let prefix = match self.self_arg {
            Some(&FnArg::SelfRef(_, Mutability::Immutable)) => "&",
            Some(&FnArg::SelfRef(_, Mutability::Mutable)) => "&mut ",
            Some(&FnArg::SelfValue(_)) => "",
            _ => panic!(error_msg!("invalid self arg: '{:?}'"), self.self_arg),
        };
        write!(formatter, "{}Self", prefix)
    }
}

//fn create_generics_str(generics_opt: Option<&Generics>) -> String {
//    let generics = match generics_opt {
//        Some(generics) if !generics.ty_params.is_empty() => generics,
//        _ => return String::new(),
//    };
//    let mut generics_str = "::<".to_string();
//    for ty_param in &generics.ty_params {
//        generics_str.push_str(&ty_param.ident.as_ref());
//        generics_str.push(',');
//    }
//    generics_str.push('>');
//    generics_str
//}
