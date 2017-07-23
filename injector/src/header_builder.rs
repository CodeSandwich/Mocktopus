use syn::{ExprKind, FnArg, Ident, Mutability, Pat, self, Stmt};

const ARG_REPLACEMENT_TUPLE_NAME: &str = "replacement";

macro_rules! error_msg {
    ($msg:expr) => { concat!("Mocktopus internal error: ", $msg) }
}

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
        self.non_self_args = Some(inputs);
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
            let ({non_self_args}) = {block_unsafety} {{
                use mocktopus::*;
                match Mockable::call_mock(&{full_fn_name}, (({self_arg}{non_self_args}))) {{
                    MockResult::Continue({arg_replacement_tuple}) => {{
                        {self_arg_replacement}
                        {non_self_arg_return}
                    }},
                    MockResult::Return(result) => return result,
                }}
            }};
        }}"#,
            block_unsafety = self.create_block_unsafety_str(),
            full_fn_name = self.create_full_fn_name_str(),
            self_arg = self.create_self_arg_str(),
            non_self_args = self.create_non_self_args_str(),
            arg_replacement_tuple = ARG_REPLACEMENT_TUPLE_NAME,
            self_arg_replacement = self.create_self_replacement_str(),
            non_self_arg_return = self.create_non_self_return_str())
    }

    fn create_block_unsafety_str(&self) -> &str {
        match self.self_arg {
            Some(_) => "unsafe",
            None => "",
        }
    }

    fn create_full_fn_name_str(&self) -> String {
        format!("{}{}",
                if self.is_method { "Self::" } else { "" },
                self.fn_ident.expect(error_msg!("fn name not set")).as_ref())
    }

    fn create_self_arg_str(&self) -> String {
        match self.self_arg {
            Some(_) => format!("mem::replace(&mut {}, mem::uninitialized())", self.create_mut_self_acqusition_str()),
            None => "".to_string(),
        }
    }

    fn create_non_self_args_str(&self) -> String {
        let mut input_args_str = String::new();
        for arg in self.non_self_args.expect(error_msg!("passed inputs not set")) {
            match *arg {
                FnArg::Captured(Pat::Ident(_, ref ident, None), _) => input_args_str.push_str(ident.as_ref()),
                _ => panic!(error_msg!("invalid function input '{:?}'"), arg),
            };
            input_args_str.push_str(", ");
        };
        input_args_str
    }

    fn create_self_replacement_str(&self) -> String {
        match self.self_arg {
            Some(_) => format!("{} = {}.0;", self.create_mut_self_acqusition_str(), ARG_REPLACEMENT_TUPLE_NAME),
            None => "".to_string(),
        }
    }

    fn create_non_self_return_str(&self) -> String {
        match self.self_arg {
            Some(_) => {
                let mut non_self_return_str = "(".to_string();
                for i in 0..self.non_self_args.expect(error_msg!("returned inputs not set")).len() {
                    non_self_return_str.push_str(&format!("{}.{}, ", ARG_REPLACEMENT_TUPLE_NAME, i + 1));
                }
                non_self_return_str.push(')');
                non_self_return_str
            },
            None => ARG_REPLACEMENT_TUPLE_NAME.to_string()
        }
    }

    fn create_mut_self_acqusition_str(&self) -> String {
        format!("*(&self as *const {} as *mut {0}", self.create_self_type_str())
    }

    fn create_self_type_str(&self) -> &str {
        match self.self_arg {
            Some(&FnArg::SelfRef(_, Mutability::Immutable)) => "&Self",
            Some(&FnArg::SelfRef(_, Mutability::Mutable)) => "&mut Self",
            Some(&FnArg::SelfValue(_)) => "Self",
            _ => panic!(error_msg!("invalid self arg: '{:?}'"), self.self_arg),
        }
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
