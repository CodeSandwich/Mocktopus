use std::mem;
use syn::{AngleBracketedGenericArguments, GenericArgument, ParenthesizedGenericArguments, PathArguments, PathSegment,
          TypeBareFn, TypeParamBound, TypePath, TypeReference, ReturnType, Type};
use syn::punctuated::{Pair, Punctuated};

pub fn remove_lifetimes_from_path<T>(path: &mut Punctuated<PathSegment, T>) {
    for path_segment in path {
        match path_segment.arguments {
            PathArguments::AngleBracketed(ref mut args) => remove_lifetimes_from_angle_bracketed_arguments(args),
            PathArguments::Parenthesized(ref mut args)  => remove_lifetimes_from_parenthesized_arguments(args),
            PathArguments::None                         => (),
        }
    }
}

fn remove_lifetimes_from_angle_bracketed_arguments(generic_arguments: &mut AngleBracketedGenericArguments) {
    filter_map_punctuated(&mut generic_arguments.args, |mut generic_argument| {
        match generic_argument {
            GenericArgument::Lifetime(_)            => return None,
            GenericArgument::Type(ref mut type_)    => remove_lifetimes_from_type(type_),
            _                                       => (),
        };
        Some(generic_argument)
    });
}

fn remove_lifetimes_from_parenthesized_arguments(generic_arguments: &mut ParenthesizedGenericArguments) {
    modify_punctuated(&mut generic_arguments.inputs, remove_lifetimes_from_type);
    if let ReturnType::Type(_, ref mut type_box) = generic_arguments.output {
        remove_lifetimes_from_type(type_box)
    }
}

fn remove_lifetimes_from_type(type_: &mut Type) {
    match *type_ {
        Type::Slice(ref mut slice)              => remove_lifetimes_from_type(&mut slice.elem),
        Type::Array(ref mut array)              => remove_lifetimes_from_type(&mut array.elem),
        Type::Ptr(ref mut ptr)                  => remove_lifetimes_from_type(&mut ptr.elem),
        Type::Reference(ref mut reference)      => remove_lifetimes_from_type_reference(reference),
        Type::BareFn(ref mut bare_fn)           => remove_lifetimes_from_type_bare_fn(bare_fn),
        Type::Never(_)                          => (),
        Type::Tuple(ref mut tuple)              => modify_punctuated(&mut tuple.elems, remove_lifetimes_from_type),
        Type::Path(ref mut path)                => remove_lifetimes_from_type_path(path),
        Type::TraitObject(ref mut trait_object) => remove_lifetimes_from_type_param_bounds(&mut trait_object.bounds),
        Type::ImplTrait(ref mut impl_trait)     => remove_lifetimes_from_type_param_bounds(&mut impl_trait.bounds),
        Type::Paren(ref mut paren)              => remove_lifetimes_from_type(&mut paren.elem),
        Type::Group(ref mut group)              => remove_lifetimes_from_type(&mut group.elem),
        Type::Infer(_)                          => (),
        Type::Macro(_)                          => (),
        Type::Verbatim(_)                       => (),
    }
}

fn remove_lifetimes_from_type_reference(type_reference: &mut TypeReference) {
    type_reference.lifetime.take();
    remove_lifetimes_from_type(&mut type_reference.elem);
}

fn remove_lifetimes_from_type_bare_fn(type_bare_fn: &mut TypeBareFn) {
    type_bare_fn.lifetimes.take();
    modify_punctuated(&mut type_bare_fn.inputs, |t| remove_lifetimes_from_type(&mut t.ty));
}

fn remove_lifetimes_from_type_path(type_path: &mut TypePath) {
    type_path.qself.as_mut().map(|q| remove_lifetimes_from_type(&mut q.ty));
    remove_lifetimes_from_path(&mut type_path.path.segments);
}

fn remove_lifetimes_from_type_param_bounds<T>(type_param_bounds: &mut Punctuated<TypeParamBound, T>,) {
    filter_map_punctuated(type_param_bounds, |type_param_bound|
        match type_param_bound {
            TypeParamBound::Trait(mut trait_bound) => {
                trait_bound.lifetimes.take();
                remove_lifetimes_from_path(&mut trait_bound.path.segments);
                Some(TypeParamBound::Trait(trait_bound))
            },
            TypeParamBound::Lifetime(_) => None,
        });
}

fn modify_punctuated<T, P, F: Fn(&mut T)>(punctuated: &mut Punctuated<T, P>, modifier: F) {
    punctuated.iter_mut().for_each(modifier)
}

fn filter_map_punctuated<T, P, F: Fn(T) -> Option<T>>(punctuated: &mut Punctuated<T, P>, filter_map: F) {
    let has_no_trailing_punct = !punctuated.empty_or_trailing();
    let filtered_iter = mem::replace(punctuated, Punctuated::new())
        .into_pairs()
        .filter_map(|p| match p {
            Pair::Punctuated(t, p)  => filter_map(t).map(|t| Pair::Punctuated(t, p)),
            Pair::End(t)            => filter_map(t).map(|t| Pair::End(t)),
        });
    punctuated.extend(filtered_iter);
    if has_no_trailing_punct && punctuated.trailing_punct() {
        if let Some(pair) = punctuated.pop() {
            punctuated.push_value(pair.into_value())
        }
    }
}
