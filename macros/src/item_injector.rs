use crate::header_builder::FnHeaderBuilder;
use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use std::iter::FromIterator;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_quote, Attribute, Block, FnArg, GenericParam, Ident, ImplItem, ImplItemMethod, Item,
    ItemFn, ItemImpl, ItemMod, ItemTrait, Pat, PatIdent, PatType, Receiver, ReturnType, Signature,
    TraitItem, TraitItemMethod, Type, Visibility, WhereClause,
};

#[derive(Clone, Copy)]
enum Context<'a> {
    Trait,
    Impl { receiver: &'a Type },
    Fn,
}

pub fn inject_item(item: &mut Item) {
    match *item {
        Item::Fn(ref mut item_fn) => inject_fn(item_fn),
        Item::Mod(ref mut item_mod) => inject_mod(item_mod),
        Item::Trait(ref mut item_trait) => inject_trait(item_trait),
        Item::Impl(ref mut item_impl) => inject_impl(item_impl),
        _ => (),
    }
}

fn inject_fn(item_fn: &mut ItemFn) {
    inject_any_fn(
        Context::Fn,
        &FnHeaderBuilder::StaticFn,
        &item_fn.attrs,
        &mut item_fn.sig,
        &mut *item_fn.block,
    );
}

fn inject_mod(item_mod: &mut ItemMod) {
    if is_not_mockable(&item_mod.attrs) {
        return;
    }
    item_mod
        .content
        .iter_mut()
        .flat_map(|c| &mut c.1)
        .for_each(inject_item)
}

fn inject_trait(item_trait: &mut ItemTrait) {
    if is_not_mockable(&item_trait.attrs) {
        return;
    }

    let context = Context::Trait;
    for item in &mut item_trait.items {
        if let TraitItem::Method(TraitItemMethod {
            ref attrs,
            ref mut sig,
            default: Some(ref mut block),
            ..
        }) = *item
        {
            inject_any_fn(context, &FnHeaderBuilder::TraitDefault, attrs, sig, block);
        }
    }
}

fn inject_impl(item_impl: &mut ItemImpl) {
    if is_not_mockable(&item_impl.attrs) {
        return;
    }
    let builder = match item_impl.trait_ {
        Some((_, ref path, _)) => FnHeaderBuilder::TraitImpl(&path.segments),
        None => FnHeaderBuilder::StructImpl,
    };

    let context = Context::Impl {
        receiver: &item_impl.self_ty,
    };

    for impl_item in &mut item_impl.items {
        if let ImplItem::Method(ref mut item_method) = *impl_item {
            if is_impl_fn_mockabile(&builder, item_method) {
                inject_any_fn(
                    context,
                    &builder,
                    &item_method.attrs,
                    &mut item_method.sig,
                    &mut item_method.block,
                );
            }
        }
    }
}

fn is_impl_fn_mockabile(builder: &FnHeaderBuilder, item_method: &ImplItemMethod) -> bool {
    if let FnHeaderBuilder::TraitImpl(ref segments) = *builder {
        if let Some(segment) = segments.last() {
            if segment.arguments.is_empty() && segment.ident == "Drop" {
                if item_method.sig.ident == "drop" {
                    return false;
                }
            }
        }
    }
    true
}

fn inject_any_fn(
    context: Context,
    builder: &FnHeaderBuilder,
    attrs: &Vec<Attribute>,
    fn_decl: &mut Signature,
    block: &mut Block,
) {
    if fn_decl.constness.is_some()
        || fn_decl.unsafety.is_some()
        || fn_decl.variadic.is_some()
        || is_not_mockable(attrs)
    {
        return;
    }

    // Transform async functions as `async-trait`
    // See: https://github.com/dtolnay/async-trait
    if let Some(_) = fn_decl.asyncness {
        let inner = format_ident!("__{}", fn_decl.ident);
        let args = fn_decl.inputs.iter().enumerate().map(|(i, arg)| match arg {
            FnArg::Receiver(Receiver { self_token, .. }) => quote!(#self_token),
            FnArg::Typed(arg) => {
                if let Pat::Ident(PatIdent { ident, .. }) = &*arg.pat {
                    quote!(#ident)
                } else {
                    positional_arg(i).into_token_stream()
                }
            }
        });

        for stmt in &mut block.stmts {
            replace_self_in_stmt(stmt);
        }

        let mut sig = fn_decl.clone();
        sig.ident = inner.clone();
        match sig.inputs.iter_mut().next() {
            Some(
                arg
                @
                FnArg::Receiver(Receiver {
                    reference: Some(_), ..
                }),
            ) => {
                let (self_token, mutability, lifetime) = match arg {
                    FnArg::Receiver(Receiver {
                        self_token,
                        mutability,
                        reference: Some((_, lifetime)),
                        ..
                    }) => (self_token, mutability, lifetime),
                    _ => unreachable!(),
                };
                let under_self = Ident::new("_self", self_token.span);
                match context {
                    Context::Impl { receiver, .. } => {
                        // TODO: parse reference
                        *arg = parse_quote! {
                            #under_self: &#lifetime #mutability #receiver
                        };
                    }
                    _ => (),
                };
            }
            Some(arg @ FnArg::Receiver(_)) => {
                let (self_token, mutability) = match arg {
                    FnArg::Receiver(Receiver {
                        self_token,
                        mutability,
                        ..
                    }) => (self_token, mutability),
                    _ => unreachable!(),
                };
                let under_self = Ident::new("_self", self_token.span);
                match context {
                    Context::Impl { receiver, .. } => {
                        // TODO: parse reference
                        *arg = parse_quote! {
                            #under_self: #mutability #receiver
                        };
                    }
                    _ => (),
                };
            }
            _ => {}
        };

        let func = ItemFn {
            attrs: attrs.clone(),
            vis: Visibility::Inherited,
            sig,
            block: Box::new(block.clone()),
        };

        let brace = block.brace_token;
        let box_pin = quote_spanned!(brace.span=> {
            Box::pin(#inner(#(#args),*))
        });
        *block = parse_quote!(#box_pin);
        block.brace_token = brace;

        block.stmts.insert(0, syn::Stmt::Item(Item::Fn(func)));

        fn_decl.asyncness = None;

        let where_clause = fn_decl
            .generics
            .where_clause
            .get_or_insert_with(|| WhereClause {
                where_token: Default::default(),
                predicates: Punctuated::new(),
            });

        match fn_decl.inputs.iter_mut().next() {
            Some(
                arg
                @
                FnArg::Receiver(Receiver {
                    reference: Some(_), ..
                }),
            ) => {
                let (self_token, mutability, lifetime) = match arg {
                    FnArg::Receiver(Receiver {
                        self_token,
                        mutability,
                        reference: Some((_, lifetime)),
                        ..
                    }) => (self_token, mutability, lifetime),
                    _ => unreachable!(),
                };
                *arg = parse_quote! {
                    &'life_self #lifetime #mutability #self_token
                };
            }
            Some(arg @ FnArg::Receiver(_)) => {
                let (self_token, mutability) = match arg {
                    FnArg::Receiver(Receiver {
                        self_token,
                        mutability,
                        ..
                    }) => (self_token, mutability),
                    _ => unreachable!(),
                };
                *arg = parse_quote! {
                    #mutability #self_token
                };
            }
            _ => {}
        };

        fn_decl.generics.params.push(parse_quote!('life_self));

        for param in fn_decl.generics.params.iter() {
            match param {
                GenericParam::Type(param) => {
                    let param = &param.ident;
                    where_clause
                        .predicates
                        .push(parse_quote!(#param: 'mocktopus));
                }
                GenericParam::Lifetime(param) => {
                    let param = &param.lifetime;
                    where_clause
                        .predicates
                        .push(parse_quote!(#param: 'mocktopus));
                }
                GenericParam::Const(_) => {}
            }
        }
        fn_decl.generics.params.push(parse_quote!('mocktopus));

        if let Context::Impl { .. } = context {
            where_clause.predicates.push(parse_quote!(Self: 'mocktopus));
        }

        let ret = match &fn_decl.output {
            ReturnType::Default => quote!(()),
            ReturnType::Type(_, ret) => quote!(#ret),
        };
        let bounds = quote!(::core::marker::Send + 'mocktopus);
        fn_decl.output = parse_quote! {
            -> ::core::pin::Pin<Box<
                dyn ::core::future::Future<Output = #ret> + #bounds
            >>
        };
    }

    unignore_fn_args(&mut fn_decl.inputs);
    let header_stmt = builder.build(fn_decl, block.brace_token.span);
    block.stmts.insert(0, header_stmt);
}

fn unignore_fn_args(inputs: &mut Punctuated<FnArg, Comma>) {
    for (i, fn_arg) in inputs.iter_mut().enumerate() {
        if let FnArg::Typed(PatType { ref mut pat, .. }) = *fn_arg {
            let (span, attrs) = match **pat {
                Pat::Wild(ref pat_wild) => {
                    (pat_wild.underscore_token.spans[0], pat_wild.attrs.clone())
                }
                _ => continue,
            };
            *pat = Box::new(Pat::Ident(PatIdent {
                by_ref: None,
                mutability: None,
                ident: Ident::new(&format!("__mocktopus_unignored_argument_{}__", i), span),
                subpat: None,
                attrs,
            }));
        }
    }
}

const INJECTOR_STOPPER_ATTRS: [&str; 2] = ["mockable", "not_mockable"];

fn is_not_mockable(attrs: &Vec<Attribute>) -> bool {
    attrs
        .iter()
        .filter_map(|a| a.path.segments.last())
        .map(|segment| segment.ident.to_string())
        .any(|i| INJECTOR_STOPPER_ATTRS.contains(&&*i))
}

fn positional_arg(i: usize) -> Ident {
    format_ident!("__arg{}", i)
}

fn replace_self_in_stmt(stmt: &mut syn::Stmt) {
    match stmt {
        syn::Stmt::Semi(expr, _) => replace_self_in_expr(expr),
        syn::Stmt::Expr(expr) => replace_self_in_expr(expr),
        _ => (),
    }
}

fn replace_self_in_expr(expr: &mut syn::Expr) {
    match expr {
        syn::Expr::Assign(expr) => {
            replace_self_in_expr(&mut expr.left);
            replace_self_in_expr(&mut expr.right);
        }
        syn::Expr::AssignOp(expr) => {
            replace_self_in_expr(&mut expr.left);
            replace_self_in_expr(&mut expr.right);
        }
        syn::Expr::Await(expr) => {
            replace_self_in_expr(&mut expr.base);
        }
        syn::Expr::Block(expr) => {
            for stmt in &mut expr.block.stmts {
                replace_self_in_stmt(stmt);
            }
        }
        syn::Expr::Call(expr) => {
            replace_self_in_expr(&mut expr.func);
            for arg in &mut expr.args {
                replace_self_in_expr(arg);
            }
        }
        syn::Expr::Field(expr) => {
            replace_self_in_expr(&mut expr.base);
        }
        syn::Expr::Let(expr) => {
            replace_self_in_expr(&mut expr.expr);
        }
        syn::Expr::Macro(expr) => {
            replace_self_in_token_stream(&mut expr.mac.tokens);
            replace_self_in_path(&mut expr.mac.path);
            for attr in &mut expr.attrs {
                replace_self_in_path(&mut attr.path);
            }
        }
        syn::Expr::Path(expr) => replace_self_in_path(&mut expr.path),
        _ => (),
    }
}

fn replace_self_in_path(path: &mut syn::Path) {
    for segment in &mut path.segments {
        if segment.ident == "self" {
            let span = segment.ident.span();
            segment.ident = Ident::new("_self", span);
        }
    }
}

fn replace_self_in_token_stream(tokens: &mut TokenStream) {
    let mut out = Vec::new();
    let mut iter = tokens.clone().into_iter().peekable();
    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Ident(mut ident) => {
                if ident == "self" {
                    let span = ident.span();
                    ident = Ident::new("_self", span);
                    out.push(TokenTree::Ident(ident));
                } else {
                    out.push(TokenTree::Ident(ident));
                }
            }
            other => out.push(other),
        }
    }
    *tokens = TokenStream::from_iter(out);
}
