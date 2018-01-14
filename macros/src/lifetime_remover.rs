//use syn::{BareFnTy, ConstExpr, FunctionRetTy, Path, PathParameters, Ty, TyParamBound};
//
//pub fn remove_lifetimes_from_path(path: &mut Path) {
//    for path_segment in &mut path.segments {
//        match path_segment.parameters {
//            PathParameters::AngleBracketed(ref mut angle_bracketed_parameter_data) => {
//                angle_bracketed_parameter_data.lifetimes.truncate(0);
//                for ty in &mut angle_bracketed_parameter_data.types {
//                    remove_lifetimes_from_ty(ty);
//                }
//                for type_binding in &mut angle_bracketed_parameter_data.bindings {
//                    remove_lifetimes_from_ty(&mut type_binding.ty);
//                }
//            },
//            PathParameters::Parenthesized(ref mut parenthesized_parameter_data) => {
//                for ty in &mut parenthesized_parameter_data.inputs {
//                    remove_lifetimes_from_ty(ty);
//                }
//                if let Some(ty) = parenthesized_parameter_data.output.as_mut() {
//                    remove_lifetimes_from_ty(ty);
//                }
//            },
//        }
//    }
//}
//set_input_argsset_input_argsset_input_args
//fn remove_lifetimes_from_ty(ty: &mut Ty) {
//    match *ty {
//        Ty::Slice(ref mut ty_box) => remove_lifetimes_from_ty(ty_box),
//        Ty::Array(ref mut ty_box, ref mut const_expr) => {
//            remove_lifetimes_from_ty(ty_box);
//            remove_lifetimes_from_const_expr(const_expr);
//        },
//        Ty::Ptr(ref mut mut_ty_box) => remove_lifetimes_from_ty(&mut mut_ty_box.ty),
//        Ty::Rptr(ref mut lifetime_opt, ref mut mut_ty_box) => {
//            lifetime_opt.take();
//            remove_lifetimes_from_ty(&mut mut_ty_box.ty);
//        },
//        Ty::BareFn(ref mut bare_fn_ty_box) => remove_lifetimes_from_bare_fn_ty(bare_fn_ty_box),
//        Ty::Never => (),
//        Ty::Tup(ref mut ty_vec) => {
//            for ty in ty_vec {
//                remove_lifetimes_from_ty(ty);
//            }
//        },
//        Ty::Path(ref mut qself_opt, ref mut path) => {
//            if let Some(qself) = qself_opt.as_mut() {
//                remove_lifetimes_from_ty(&mut qself.ty);
//            }
//            remove_lifetimes_from_path(path);
//        },
//        Ty::TraitObject(ref mut ty_param_bound_vec) => remove_lifetimes_from_ty_param_bounds(ty_param_bound_vec),
//        Ty::ImplTrait(ref mut ty_param_bound_vec) => remove_lifetimes_from_ty_param_bounds(ty_param_bound_vec),
//        Ty::Paren(ref mut ty_box) => remove_lifetimes_from_ty(ty_box),
//        Ty::Infer => (),
//        Ty::Mac(_) => (),
//    }
//}
//
//fn remove_lifetimes_from_const_expr(const_expr: &mut ConstExpr) {
//    match *const_expr {
//        ConstExpr::Call(ref mut const_expr_box, ref mut const_expr_vec) => {
//            remove_lifetimes_from_const_expr(const_expr_box);
//            for const_expr in const_expr_vec {
//                remove_lifetimes_from_const_expr(const_expr);
//            }
//        },
//        ConstExpr::Binary(_, ref mut const_expr_box_1, ref mut const_expr_box_2) => {
//            remove_lifetimes_from_const_expr(const_expr_box_1);
//            remove_lifetimes_from_const_expr(const_expr_box_2);
//        },
//        ConstExpr::Unary(_, ref mut const_expr_box) => remove_lifetimes_from_const_expr(const_expr_box),
//        ConstExpr::Lit(_) => (),
//        ConstExpr::Cast(ref mut const_expr_box, ref mut ty_box) => {
//            remove_lifetimes_from_const_expr(const_expr_box);
//            remove_lifetimes_from_ty(ty_box);
//        },
//        ConstExpr::Path(ref mut path) => remove_lifetimes_from_path(path),
//        ConstExpr::Index(ref mut const_expr_box_1, ref mut const_expr_box_2) => {
//            remove_lifetimes_from_const_expr(const_expr_box_1);
//            remove_lifetimes_from_const_expr(const_expr_box_2);
//        },
//        ConstExpr::Paren(ref mut const_expr_box) => remove_lifetimes_from_const_expr(const_expr_box),
//        ConstExpr::Other(_) => (), // Questionable reachability and very hard lifetime removal
//    }
//}
//
//fn remove_lifetimes_from_bare_fn_ty(bare_fn_ty: &mut BareFnTy) {
//    bare_fn_ty.lifetimes.truncate(0);
//    for input in &mut bare_fn_ty.inputs {
//        remove_lifetimes_from_ty(&mut input.ty);
//    }
//    if let FunctionRetTy::Ty(ref mut ty) = bare_fn_ty.output {
//        remove_lifetimes_from_ty(ty);
//    }
//}
//
//fn remove_lifetimes_from_ty_param_bounds(ty_param_bounds: &mut Vec<TyParamBound>) {
//    for i in (0..ty_param_bounds.len()).rev() {
//        match ty_param_bounds[i] {
//            TyParamBound::Trait(ref mut poly_trait_ref, _) => {
//                poly_trait_ref.bound_lifetimes.truncate(0);
//                remove_lifetimes_from_path(&mut poly_trait_ref.trait_ref);
//                continue;
//            },
//            TyParamBound::Region(_) => (),
//        }
//        ty_param_bounds.remove(i);
//    }
//}
