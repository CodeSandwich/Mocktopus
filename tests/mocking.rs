#![feature(const_fn, proc_macro)]

//extern crate mocktopus_injector;
extern crate mocktopus;

//use mocktopus_injector::inject_mocks;
use mocktopus::*;

mod mocks_do_not_leak_between_tests {
    use super::*;

    #[inject_mocks]
    pub fn no_args_returns_str() -> &'static str {
        "not mocked"
    }

    macro_rules! generate_tests {
        ($($fn_name:ident),+) => {
            $(
                #[test]
                fn $fn_name() {
                    assert_eq!("not mocked", no_args_returns_str(), "function was mocked before mocking");

                    no_args_returns_str.mock_raw(|| MockResult::Return((stringify!($fn_name))));

                    assert_eq!(stringify!($fn_name), no_args_returns_str(), "mocking failed");
                }
            )+
        }
    }

    generate_tests!(t00, t01, t02, t03, t04, t05, t06, t07, t08, t09, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19);
    generate_tests!(t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39);
    generate_tests!(t40, t41, t42, t43, t44, t45, t46, t47, t48, t49, t50, t51, t52, t53, t54, t55, t56, t57, t58, t59);
    generate_tests!(t60, t61, t62, t63, t64, t65, t66, t67, t68, t69, t70, t71, t72, t73, t74, t75, t76, t77, t78, t79);
    generate_tests!(t80, t81, t82, t83, t84, t85, t86, t87, t88, t89, t90, t91, t92, t93, t94, t95, t96, t97, t98, t99);
}

mod mocking_does_not_works_for_const_fns {
    use super::*;

    #[inject_mocks]
    pub const fn const_fn() -> u32 {
        1
    }

    #[test]
    fn when_not_mocked_then_returns_1() {
        assert_eq!(1, const_fn());
    }

    #[test]
    fn when_mocked_then_returns_1() {
        const_fn.mock_raw(|| MockResult::Return(2));

        assert_eq!(1, const_fn());
    }
}

mod mocking_captures_ignored_args {
    use super::*;

    #[inject_mocks]
    pub fn two_args_returns_first_ignores_second(x: u32, _: u32) -> u32 {
        x
    }

    #[test]
    fn when_not_mocked_then_returns_first_arg() {
        assert_eq!(1, two_args_returns_first_ignores_second(1, 2));
    }

    #[test]
    fn when_mocked_then_returns_second_arg() {
        two_args_returns_first_ignores_second.mock_raw(|x, y| MockResult::Continue((y, x)));

        assert_eq!(2, two_args_returns_first_ignores_second(1, 2));
    }
}

mod mocking_does_not_work_for_macro_generated_fns {
    use super::*;

    macro_rules! fn_generating_macro {
        () => {
            pub fn macro_generated_fn() -> u32 {
                1
            }
        }
    }

    #[inject_mocks]
    fn_generating_macro!();

    #[test]
    fn when_not_mocked_then_returns_1() {
        assert_eq!(1, macro_generated_fn());
    }

    #[test]
    fn when_mocked_then_returns_1() {
        macro_generated_fn.mock_raw(|| MockResult::Return(2));

        assert_eq!(1, macro_generated_fn());
    }
}

mod mock_injecting_works_for_nested_mods {
    use super::*;

    #[inject_mocks]
    mod mod_1 {
        pub fn mod_1_fn() -> &'static str {
            "mod_1_fn not mocked"
        }

        pub mod mod_2 {
            pub fn mod_2_fn() -> &'static str {
                "mod_2_fn not mocked"
            }
        }

        pub mod mod_3 {
            pub fn mod_3_fn() -> &'static str {
                "mod_3_fn not mocked"
            }
        }
    }

    #[test]
    fn when_not_mocked_then_returns_not_mocked_strs() {
        assert_eq!("mod_1_fn not mocked", mod_1::mod_1_fn());
        assert_eq!("mod_2_fn not mocked", mod_1::mod_2::mod_2_fn());
        assert_eq!("mod_3_fn not mocked", mod_1::mod_3::mod_3_fn());
    }

    #[test]
    fn when_mocked_then_returns_mocked_strs() {
        mod_1::mod_1_fn.mock_raw(|| MockResult::Return("mod_1_fn mocked"));
        mod_1::mod_2::mod_2_fn.mock_raw(|| MockResult::Return("mod_2_fn mocked"));
        mod_1::mod_3::mod_3_fn.mock_raw(|| MockResult::Return("mod_3_fn mocked"));

        assert_eq!("mod_1_fn mocked", mod_1::mod_1_fn());
        assert_eq!("mod_2_fn mocked", mod_1::mod_2::mod_2_fn());
        assert_eq!("mod_3_fn mocked", mod_1::mod_3::mod_3_fn());
    }
}

#[inject_mocks]
mod mod_file_1;

mod mock_injecting_works_for_nested_mods_in_separate_files {
    use super::*;

    #[test]
    fn when_not_mocked_then_returns_not_mocked_strs() {
        assert_eq!("mod_file_1_fn not mocked", mod_file_1::mod_file_1_fn());
        assert_eq!("mod_file_2_fn not mocked", mod_file_1::mod_file_2::mod_file_2_fn());
        assert_eq!("mod_file_3_fn not mocked", mod_file_1::mod_file_3::mod_file_3_fn());
    }

    #[test]
    fn when_mocked_then_returns_mocked_strs() {
        mod_file_1::mod_file_1_fn.mock_raw(|| MockResult::Return("mod_file_1_fn mocked"));
        mod_file_1::mod_file_2::mod_file_2_fn.mock_raw(|| MockResult::Return("mod_file_2_fn mocked"));
        mod_file_1::mod_file_3::mod_file_3_fn.mock_raw(|| MockResult::Return("mod_file_3_fn mocked"));

        assert_eq!("mod_file_1_fn mocked", mod_file_1::mod_file_1_fn());
        assert_eq!("mod_file_2_fn mocked", mod_file_1::mod_file_2::mod_file_2_fn());
        assert_eq!("mod_file_3_fn mocked", mod_file_1::mod_file_3::mod_file_3_fn());
    }
}

mod annotating_function_twice_makes_it_injected_once {
    use super::*;

    #[inject_mocks]
    mod mock_annotated_mod {
        #[inject_mocks]
        pub fn mock_annotated_fn(x: u32) -> u32 {
            x * 2
        }
    }

    //TODO TEST TRAIT VARIANT DEFAULT LEAKAGE
    //TODO TEST REGULAR AND TRAIT METHODS LEAKAGE
    //TODO TEST COMPLEX TRAIT NAMES (WHAT ABOUT LIFETIMES?)
    #[test]
    // Actually it gets injects twice TODO fix
    fn ___fix_me___function_gets_injected_once() {
        mock_annotated_mod::mock_annotated_fn.mock_raw(|x| MockResult::Continue((x + 1,)));

//        assert_eq!(4, mock_annotated_mod::mock_annotated_fn(1));
    }
}

mod mocking_generic_over_a_type_with_lifetime_mocks_all_lifetime_variants {
    use super::*;
    use std::fmt::Display;

    #[inject_mocks]
    fn function<T: Display>(generic: T) -> String {
        format!("not mocked {}", generic)
    }

    static STATIC_CHAR: char = 'S';

    #[test]
    fn all_lifetime_variants_get_mocked() {
        function::<&char>.mock_raw(|c| MockResult::Return(format!("mocked {}", c)));
        let local_char = 'L';

        assert_eq!("mocked L", function(&local_char));
        assert_eq!("mocked S", function(&STATIC_CHAR));
        assert_eq!("not mocked 3", function(&3));
    }
}

mod mocking_generic_over_a_reference_does_not_mock_opposite_mutability_variant {
    use super::*;
    use std::fmt::Display;

    #[inject_mocks]
    fn function<T: Display>(generic: T) -> String {
        format!("not mocked {}", generic)
    }

    #[test]
    fn mocking_for_ref_does_not_mock_for_mut_ref() {
        function::<&char>.mock_raw(|c| MockResult::Return(format!("mocked {}", c)));

        assert_eq!("mocked R", function(&'R'));
        assert_eq!("not mocked M", function(&mut 'M'));
    }

    #[test]
    fn mocking_for_mut_ref_does_not_mock_for_ref() {
        function::<&mut char>.mock_raw(|c| MockResult::Return(format!("mocked {}", c)));

        assert_eq!("not mocked R", function(&'R'));
        assert_eq!("mocked M", function(&mut 'M'));
    }
}

mod mocking_impls_of_traits_with_path {
    use super::*;
    use self::trait_mod::Trait;

    struct Struct();

    mod trait_mod {
        pub trait Trait {
            fn method() -> &'static str;
        }
    }

    #[inject_mocks]
    impl self::trait_mod::Trait for Struct {
        fn method() -> &'static str {
            "not mocked"
        }
    }

    #[test]
    fn mocks_successfully() {
        Struct::method.mock_raw(|| MockResult::Return("mocked"));

        assert_eq!("mocked", Struct::method());
    }
}

mod mocking_impls_of_traits_generic_over_generic_refs {
    use super::*;

    struct Struct();

    trait Trait<T> {
        fn method() -> &'static str;
    }

    #[inject_mocks]
    impl<'a, T> Trait<&'a T> for Struct {
        fn method() -> &'static str {
            "not mocked"
        }
    }

    #[test]
    fn mocks_successfully() {
        <Struct as Trait<&u32>>::method.mock_raw(|| MockResult::Return("mocked"));

        assert_eq!("mocked", <Struct as Trait<&u32>>::method());
    }
}
