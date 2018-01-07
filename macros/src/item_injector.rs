use header_builder::HeaderBuilder;
use std::mem;
use syn::{Attribute, Block, FnArg, Ident, Item, ItemFn, ItemImpl, ItemMod, ItemTrait};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Const};

pub fn inject_item(item: &mut Item) {
    match *item {
        Item::Fn(ref mut item_fn)       => inject_fn(item_fn),
        Item::Mod(ref mut item_mod)     => inject_mod(item_mod),
        Item::Trait(ref mut item_trait) => inject_trait(item_trait),
        Item::Impl(ref mut item_impl)   => inject_impl(item_impl),
        _                               => (),
    }
}

fn inject_fn(item_fn: &mut ItemFn) {
    inject_any_fn(HeaderBuilder::default(), &item_fn.attrs, &item_fn.constness, &item_fn.ident,
                  &mut item_fn.decl.inputs, &mut *item_fn.block);
}

fn inject_mod(item_mod: &mut ItemMod) {
    if is_not_mockable(&item_mod.attrs) {
        return
    }
    item_mod.content.iter_mut()
        .flat_map(|c| &mut c.1)
        .for_each(inject_item)
}

fn inject_trait(item_trait: &mut ItemTrait) {
    if is_not_mockable(&item_trait.attrs) {
        return
    }
//    for item in items.iter_mut().filter(|i| do_item_attrs_let_injector_in(&i.attrs)) {
//        if let TraitItemKind::Method(
//            MethodSig {
//                unsafety: _,
//                ref constness,
//                abi: _,
//                ref mut decl,
//                generics: _},
//            Some(ref mut block)) = item.node {
//            let builder = HeaderBuilder::default()
//                .set_is_method();
//            inject_fn(builder, &item.ident, &mut decl.inputs, constness, block);
//        }
//    }
}

fn inject_impl(item_impl: &mut ItemImpl) {
    if is_not_mockable(&item_impl.attrs) {
        return
    }
//    for item in items.iter_mut().filter(|i| do_item_attrs_let_injector_in(&i.attrs)) {
//        if let ImplItemKind::Method(
//            MethodSig {
//                unsafety: _,
//                ref constness,
//                abi: _,
//                ref mut decl,
//                generics: _},
//            ref mut block) = item.node {
//            let builder = HeaderBuilder::default()
//                .set_is_method()
//                // full path with trait name is needed in impl of concrete struct to avoid ambiguity
//                .set_trait_path(trait_path);
//            inject_fn(builder, &item.ident, &mut decl.inputs, constness, block);
//        }
//    }
}

fn inject_any_fn(builder: HeaderBuilder, attrs: &Vec<Attribute>, constness: &Option<Const>, fn_name: &Ident,
                 inputs: &mut Punctuated<FnArg, Comma>, block: &mut Block) {
    if constness.is_some() || is_not_mockable(attrs) {
        return
    }
    unignore_fn_args(inputs);
    let header_stmt = builder
        .set_fn_name(fn_name)
        .set_input_args(inputs)
        .build();
    block.stmts.insert(0, header_stmt);
//    let mut body_stmts = mem::replace(&mut block.stmts, header_stmts);
//    block.stmts.append(&mut body_stmts);
}

fn unignore_fn_args(inputs: &mut Punctuated<FnArg, Comma>) {
//    inputs.iter_mut()
//        .enumerate()
//        .for_each(|(i, fn_arg)| )
//
//    for i in 0..inputs.len() {
//        let unignored = match inputs[i] {
//            FnArg::Captured(Pat::Wild, ref ty) =>
//                FnArg::Captured(
//                    Pat::Ident(
//                        BindingMode::ByValue(
//                            Mutability::Immutable),
//                        Ident::from(format!("__mocktopus_unignored_argument_{}__", i)),
//                        None),
//                    ty.clone()),
//            _ => continue,
//        };
//        inputs[i] = unignored;
//    }
}

const INJECTOR_STOPPER_ATTRS: [&str; 2] = ["mockable", "not_mockable"];

fn is_not_mockable(attrs: &Vec<Attribute>) -> bool {
    attrs.iter()
        .filter_map(|a| a.path.segments.last())
        .map(|s| s.value().ident.as_ref())
        .any(|i| INJECTOR_STOPPER_ATTRS.contains(&i))
}
