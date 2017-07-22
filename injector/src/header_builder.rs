use syn::{ExprKind, FnArg, Generics, Ident, Pat, self, Stmt};

macro_rules! error_msg {
    ($msg:expr) => { concat!("Mocktopus internal error: ", $msg) }
}

#[derive(Default)]
pub struct HeaderBuilder<'a> {
    is_method: bool,
    fn_ident: Option<&'a Ident>,
    fn_generics: Option<&'a Generics>,
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

    pub fn set_fn_generics(mut self, fn_generics: &'a Generics) -> Self {
        self.fn_generics = Some(fn_generics);
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
            let ({non_self_args}) = {{
                use mocktopus::*;
                match Mockable::call_mock(&{full_fn_name}, (({self_arg}{non_self_args}))) {{
                    MockResult::Continue(input) => input,
                    MockResult::Return(result) => return result,
                }}
            }};
        }}"#,
            self_arg = self.create_self_arg_str(),
            non_self_args = self.create_non_self_args_str(),
            full_fn_name = self.create_full_fn_name_str())
    }

    fn create_non_self_args_str(&self) -> String {
        let mut input_args_str = String::new();
        for arg in self.non_self_args.expect(error_msg!("inputs not set")) {
            match *arg {
                FnArg::Captured(Pat::Ident(_, ref ident, None), _) => input_args_str.push_str(ident.as_ref()),
                _ => panic!(error_msg!("invalid function input '{:?}'"), arg),
            };
            input_args_str.push_str(", ");
        };
        input_args_str
    }

    fn create_self_arg_str(&self) -> &str {
        match self.self_arg {
            Some(_) => "self, ",
            None => "",
        }
    }

    fn create_full_fn_name_str(&self) -> String {
        format!("{}{}{}",
                if self.is_method { "Self::" } else { "" },
                self.fn_ident.expect(error_msg!("fn name not set")).as_ref(),
                create_generics_str(self.fn_generics))
    }
}

fn create_args_str<'a, T: Iterator<Item = &'a FnArg>>(args_iter: T) -> String {
    let mut input_args_str = String::new();
    for arg in args_iter {
        match *arg {
            FnArg::SelfRef(_, _) | FnArg::SelfValue(_) => input_args_str.push_str("self"),
            FnArg::Captured(Pat::Ident(_, ref ident, None), _) => input_args_str.push_str(ident.as_ref()),
            _ => panic!(error_msg!("invalid function input '{:?}'"), arg),
        };
        input_args_str.push_str(", ");
    };
    input_args_str
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
