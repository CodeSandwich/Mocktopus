use header_builder::HeaderBuilder;
use std::mem;
use syn::{Attribute, BindingMode, Block, Constness, FnArg, Ident, ImplItem, ImplItemKind, Item, ItemKind, MethodSig,
          Mutability, Pat, Path, TraitItem, TraitItemKind};

pub fn inject_item(item: &mut Item) {
    if !do_item_attrs_let_injector_in(&item.attrs) {
        return
    }
    match item.node {
        ItemKind::Mod(ref mut items_opt) =>
            inject_mod(items_opt.as_mut()),
        ItemKind::Fn(ref mut decl, _, ref constness, _, _, ref mut block) =>
            inject_fn(HeaderBuilder::default(), &item.ident, &mut decl.inputs, constness, block),
        ItemKind::Impl(_, _, _, ref path, _, ref mut items) =>
            inject_impl(path.as_ref(), items),
        ItemKind::Trait(_, _, _, ref mut items) =>
            inject_trait_defaults(items),
        _ => (),
    }
}

fn inject_mod(items_opt: Option<&mut Vec<Item>>) {
    items_opt.into_iter()
        .flat_map(|v| v)
        .for_each(inject_item);
}

fn inject_impl(trait_path: Option<&Path>, items: &mut Vec<ImplItem>) {
    for item in items.iter_mut().filter(|i| do_item_attrs_let_injector_in(&i.attrs)) {
        if let ImplItemKind::Method(
            MethodSig {
                unsafety: _,
                ref constness,
                abi: _,
                ref mut decl,
                generics: _},
            ref mut block) = item.node {
            let builder = HeaderBuilder::default()
                .set_is_method()
                .set_trait_path(trait_path);
            inject_fn(builder, &item.ident, &mut decl.inputs, constness, block);
        }
    }
}

fn inject_trait_defaults(items: &mut Vec<TraitItem>) {
    for item in items.iter_mut().filter(|i| do_item_attrs_let_injector_in(&i.attrs)) {
        if let TraitItemKind::Method(
            MethodSig {
                unsafety: _,
                ref constness,
                abi: _,
                ref mut decl,
                generics: _},
            Some(ref mut block)) = item.node {
            let builder = HeaderBuilder::default()
                .set_is_method();
            inject_fn(builder, &item.ident, &mut decl.inputs, constness, block);
        }
    }
}

fn inject_fn(builder: HeaderBuilder, fn_name: &Ident, inputs: &mut Vec<FnArg>, constness: &Constness, block: &mut Block) {
    if *constness == Constness::Const {
        return
    }
    unignore_fn_args(inputs);
    let header_stmts = builder
        .set_fn_name(fn_name)
        .set_input_args(inputs)
        .build();
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

const INJECTOR_STOPPER_ATTRS: [&str; 2] = ["mockable", "not_mockable"];

fn do_item_attrs_let_injector_in(attrs: &Vec<Attribute>) -> bool {
    attrs.iter().all(|a| !INJECTOR_STOPPER_ATTRS.contains(&a.name()))
}
