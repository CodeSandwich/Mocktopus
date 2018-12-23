#![feature(const_fn, proc_macro_gen, proc_macro_mod)]

// Test if injecting works even if mocktopus is aliased
extern crate mocktopus as mocktopus_aliased;

use mocktopus_aliased::macros::*;
use mocktopus_aliased::mocking::*;

mod injector_injects_annotated_fns {
    use super::*;

    #[mockable]
    fn function() -> &'static str {
        "not mocked"
    }

    #[test]
    fn when_not_mocked_then_runs_normally() {
        assert_eq!("not mocked", function());
    }

    #[test]
    fn when_mocked_then_runs_mock() {
        unsafe {
            function.mock_raw(|| MockResult::Return("mocked"))
        }

        assert_eq!("mocked", function());
    }
}

mod injector_injects_annotated_impl_blocks {
    use super::*;

    struct Struct;

    #[mockable]
    impl Struct {
        fn function() -> &'static str {
            "not mocked"
        }
    }

    #[test]
    fn when_not_mocked_then_runs_normally() {
        assert_eq!("not mocked", Struct::function());
    }

    #[test]
    fn when_mocked_then_runs_mock() {
        unsafe {
            Struct::function.mock_raw(|| MockResult::Return("mocked"))
        }

        assert_eq!("mocked", Struct::function());
    }
}

mod injector_injects_annotated_traits {
    use super::*;

    #[mockable]
    trait Trait {
        fn function() -> &'static str {
            "not mocked"
        }
    }

    struct Struct;

    impl Trait for Struct {

    }

    #[test]
    fn when_not_mocked_then_runs_normally() {
        assert_eq!("not mocked", Struct::function());
    }

    #[test]
    fn when_mocked_then_runs_mock() {
        unsafe {
            Struct::function.mock_raw(|| MockResult::Return("mocked"))
        }

        assert_eq!("mocked", Struct::function());
    }
}

#[mockable]
mod multi_file_module;

mod injector_injects_annotated_items {
    use super::*;

    mod injects_fns {
        use super::*;

        #[mockable]
        mod module {
            pub fn function() -> &'static str {
                "not mocked"
            }
        }

        #[test]
        fn when_not_mocked_then_runs_normally() {
            assert_eq!("not mocked", module::function());
        }

        #[test]
        fn when_mocked_then_runs_mock() {
            unsafe {
                module::function.mock_raw(|| MockResult::Return("mocked"))
            }

            assert_eq!("mocked", module::function());
        }
    }

    mod injects_impl_blocks {
        use super::*;

        struct Struct;

        #[mockable]
        mod module {
            use super::*;

            impl Struct {
                pub fn function() -> &'static str {
                    "not mocked"
                }
            }
        }

        #[test]
        fn when_not_mocked_then_runs_normally() {
            assert_eq!("not mocked", Struct::function());
        }

        #[test]
        fn when_mocked_then_runs_mock() {
            unsafe {
                Struct::function.mock_raw(|| MockResult::Return("mocked"))
            }

            assert_eq!("mocked", Struct::function());
        }
    }

    mod injects_traits {
        use super::*;
        use self::module::Trait;

        #[mockable]
        mod module {
            pub trait Trait {
                fn function() -> &'static str {
                    "not mocked"
                }
            }
        }

        struct Struct;

        impl Trait for Struct {

        }

        #[test]
        fn when_not_mocked_then_runs_normally() {
            assert_eq!("not mocked", Struct::function());
        }

        #[test]
        fn when_mocked_then_runs_mock() {
            unsafe {
                Struct::function.mock_raw(|| MockResult::Return("mocked"))
            }

            assert_eq!("mocked", Struct::function());
        }
    }

    mod injects_nested_mod_content {
        use super::*;

        #[mockable]
        mod module {
            pub mod module {
                pub fn function() -> &'static str {
                    "not mocked"
                }
            }
        }

        #[test]
        fn when_not_mocked_then_runs_normally() {
            assert_eq!("not mocked", module::module::function());
        }

        #[test]
        fn when_mocked_then_runs_mock() {
            unsafe {
                module::module::function.mock_raw(|| MockResult::Return("mocked"))
            }

            assert_eq!("mocked", module::module::function());
        }
    }

    mod injects_nested_multi_file_mods_content {
        use super::*;

        #[test]
        fn when_not_mocked_then_runs_normally() {
            assert_eq!("not mocked", multi_file_module::function());
        }

        #[test]
        fn when_mocked_then_runs_mock() {
            unsafe {
                multi_file_module::function.mock_raw(|| MockResult::Return("mocked"))
            }

            assert_eq!("mocked", multi_file_module::function());
        }
    }
}

mod injector_does_not_inject_items_twice {
    use super::*;

    mod injects_explicitly_double_annotated_fn_once {
        use super::*;

        #[mockable]
        #[mockable]
        pub fn mocked_fn(x: u32) -> u32 {
            x * 2
        }

        #[test]
        fn when_not_mocked_then_runs_normally() {
            assert_eq!(2, mocked_fn(1));
        }

        #[test]
        fn when_mocked_then_runs_mock_once() {
            unsafe {
                mocked_fn.mock_raw(|x| MockResult::Continue((x + 1,)));
            }

            assert_eq!(4, mocked_fn(1));
        }
    }

    mod injects_explicitly_double_annotated_impl_block_once {
        use super::*;

        struct MockedStruct;

        #[mockable]
        #[mockable]
        impl MockedStruct {
            pub fn mocked_fn(x: u32) -> u32 {
                x * 2
            }
        }

        #[test]
        fn when_not_mocked_then_runs_normally() {
            assert_eq!(2, MockedStruct::mocked_fn(1));
        }

        #[test]
        fn when_mocked_then_runs_mock_once() {
            unsafe {
                MockedStruct::mocked_fn.mock_raw(|x| MockResult::Continue((x + 1,)))
            }

            assert_eq!(4, MockedStruct::mocked_fn(1));
        }
    }

    mod injects_explicitly_double_annotated_traits_once {
        use super::*;

        #[mockable]
        #[mockable]
        pub trait MockedTrait {
            fn mocked_fn(x: u32) -> u32 {
                x * 2
            }
        }

        struct Struct;

        impl MockedTrait for Struct {

        }

        #[test]
        fn when_not_mocked_then_runs_normally() {
            assert_eq!(2, Struct::mocked_fn(1));
        }

        #[test]
        fn when_mocked_then_runs_mock_once() {
            unsafe {
                Struct::mocked_fn.mock_raw(|x| MockResult::Continue((x + 1,)))
            }

            assert_eq!(4, Struct::mocked_fn(1));
        }
    }

    mod injects_explicitly_double_annotated_mod_content_once {
        use super::*;

        #[mockable]
        #[mockable]
        pub mod mocked_mod {
            pub fn mocked_fn(x: u32) -> u32 {
                x * 2
            }
        }

        #[test]
        fn when_not_mocked_then_runs_normally() {
            assert_eq!(2, mocked_mod::mocked_fn(1));
        }

        #[test]
        fn when_mocked_then_runs_mock() {
            unsafe {
                mocked_mod::mocked_fn.mock_raw(|x| MockResult::Continue((x + 1,)))
            }

            assert_eq!(4, mocked_mod::mocked_fn(1));
        }
    }

    mod injects_implicitly_double_annotated_fn_once {
        use super::*;

        #[mockable]
        mod mocked_mod {
            use super::*;

            #[mockable]
            pub fn mocked_fn(x: u32) -> u32 {
                x * 2
            }
        }

        #[test]
        fn when_not_mocked_then_runs_normally() {
            assert_eq!(2, mocked_mod::mocked_fn(1));
        }

        #[test]
        fn when_mocked_then_runs_mock_once() {
            unsafe {
                mocked_mod::mocked_fn.mock_raw(|x| MockResult::Continue((x + 1,)));
            }

            assert_eq!(4, mocked_mod::mocked_fn(1));
        }
    }

    mod injects_implicitly_double_annotated_impl_block_once {
        use super::*;

        struct MockedStruct;

        #[mockable]
        mod mocked_mod {
            use super::*;

            #[mockable]
            impl MockedStruct {
                pub fn mocked_fn(x: u32) -> u32 {
                    x * 2
                }
            }
        }

        #[test]
        fn when_not_mocked_then_runs_normally() {
            assert_eq!(2, MockedStruct::mocked_fn(1));
        }

        #[test]
        fn when_mocked_then_runs_mock_once() {
            unsafe {
                MockedStruct::mocked_fn.mock_raw(|x| MockResult::Continue((x + 1,)))
            }

            assert_eq!(4, MockedStruct::mocked_fn(1));
        }
    }

    mod injects_implicitly_double_annotated_traits_once {
        use super::*;
        use self::mocked_mod::MockedTrait;

        #[mockable]
        mod mocked_mod {
            use super::*;

            #[mockable]
            pub trait MockedTrait {
                fn mocked_fn(x: u32) -> u32 {
                    x * 2
                }
            }
        }

        struct Struct;

        impl MockedTrait for Struct {

        }

        #[test]
        fn when_not_mocked_then_runs_normally() {
            assert_eq!(2, Struct::mocked_fn(1));
        }

        #[test]
        fn when_mocked_then_runs_mock_once() {
            unsafe {
                Struct::mocked_fn.mock_raw(|x| MockResult::Continue((x + 1,)))
            }

            assert_eq!(4, Struct::mocked_fn(1));
        }
    }

    mod injects_implicitly_double_annotated_mod_content_once {
        use super::*;

        #[mockable]
        mod mocked_mod {
            use super::*;

            #[mockable]
            pub mod mocked_mod {
                pub fn mocked_fn(x: u32) -> u32 {
                    x * 2
                }
            }
        }

        #[test]
        fn when_not_mocked_then_runs_normally() {
            assert_eq!(2, mocked_mod::mocked_mod::mocked_fn(1));
        }

        #[test]
        fn when_mocked_then_runs_mock() {
            unsafe {
                mocked_mod::mocked_mod::mocked_fn.mock_raw(|x| MockResult::Continue((x + 1,)))
            }

            assert_eq!(4, mocked_mod::mocked_mod::mocked_fn(1));
        }
    }
}

mod injector_does_not_inject_not_mockable_items {
    use super::*;

    mod does_not_injects_not_mockable_fn {
        use super::*;

        #[mockable]
        mod mocked_mod {
            use super::*;

            #[not_mockable]
            pub fn not_mocked_fn() -> &'static str {
                "not mocked"
            }
        }

        #[test]
        fn when_not_mocked_then_runs_normally() {
            assert_eq!("not mocked", mocked_mod::not_mocked_fn());
        }

        #[test]
        fn when_mocked_then_runs_normally() {
            unsafe {
                mocked_mod::not_mocked_fn.mock_raw(|| MockResult::Return("mocked"));
            }

            assert_eq!("not mocked", mocked_mod::not_mocked_fn());
        }
    }

    mod does_not_injects_not_mockable_impl_block {
        use super::*;

        struct MockedStruct;

        #[mockable]
        mod mocked_mod {
            use super::*;

            #[not_mockable]
            impl MockedStruct {
                pub fn not_mocked_fn() -> &'static str {
                    "not mocked"
                }
            }
        }

        #[test]
        fn when_not_mocked_then_runs_normally() {
            assert_eq!("not mocked", MockedStruct::not_mocked_fn());
        }

        #[test]
        fn when_mocked_then_runs_normally() {
            unsafe {
                MockedStruct::not_mocked_fn.mock_raw(|| MockResult::Return("mocked"))
            }

            assert_eq!("not mocked", MockedStruct::not_mocked_fn());
        }
    }

    mod does_not_injects_not_mockable_trait {
        use super::*;
        use self::mocked_mod::MockedTrait;

        #[mockable]
        mod mocked_mod {
            use super::*;

            #[not_mockable]
            pub trait MockedTrait {
                fn not_mocked_fn() -> &'static str {
                    "not mocked"
                }
            }
        }

        struct Struct;

        impl MockedTrait for Struct {

        }

        #[test]
        fn when_not_mocked_then_runs_normally() {
            assert_eq!("not mocked", Struct::not_mocked_fn());
        }

        #[test]
        fn when_mocked_then_runs_normally() {
            unsafe {
                Struct::not_mocked_fn.mock_raw(|| MockResult::Return("mocked"))
            }

            assert_eq!("not mocked", Struct::not_mocked_fn());
        }
    }

    mod does_not_injects_not_mockable_mod_content {
        use super::*;

        #[mockable]
        mod mocked_mod {
            use super::*;

            #[not_mockable]
            pub mod mocked_mod {
                pub fn not_mocked_fn() -> &'static str {
                    "not mocked"
                }
            }
        }

        #[test]
        fn when_not_mocked_then_runs_normally() {
            assert_eq!("not mocked", mocked_mod::mocked_mod::not_mocked_fn());
        }

        #[test]
        fn when_mocked_then_runs_normally() {
            unsafe {
                mocked_mod::mocked_mod::not_mocked_fn.mock_raw(|| MockResult::Return("mocked"))
            }

            assert_eq!("not mocked", mocked_mod::mocked_mod::not_mocked_fn());
        }
    }
}

mod injector_ignores_const_fns {
    use super::*;

    #[mockable]
    pub const fn const_fn() -> u32 {
        1
    }

    #[test]
    fn when_not_mocked_then_returns_1() {
        assert_eq!(1, const_fn());
    }

    #[test]
    fn when_mocked_then_returns_1() {
        unsafe {
            const_fn.mock_raw(|| MockResult::Return(2));
        }

        assert_eq!(1, const_fn());
    }
}

mod injector_ignores_unsafe_fns {
    use super::*;

    #[mockable]
    unsafe fn function() -> &'static str {
        "not mocked"
    }

    #[test]
    fn when_not_mocked_then_runs_normally() {
        assert_eq!("not mocked", unsafe { function() } );
    }

    // Trait Mockable is not implemented for unsafe functions
}

mod injector_ignores_unsafe_impls {
    use super::*;

    struct Struct;

    #[mockable]
    impl Struct {
        unsafe fn function() -> &'static str {
            "not mocked"
        }
    }

    #[test]
    fn when_not_mocked_then_runs_normally() {
        assert_eq!("not mocked", unsafe { Struct::function() } );
    }

    // Trait Mockable is not implemented for unsafe functions
}

mod injector_does_not_inject_macro_generated_fns {
    use super::*;

    macro_rules! fn_generating_macro {
        () => {
            pub fn macro_generated_fn() -> u32 {
                1
            }
        }
    }

    #[mockable]
    fn_generating_macro!();

    #[test]
    fn when_not_mocked_then_returns_1() {
        assert_eq!(1, macro_generated_fn());
    }

    #[test]
    fn when_mocked_then_returns_1() {
        unsafe {
            macro_generated_fn.mock_raw(|| MockResult::Return(2));
        }

        assert_eq!(1, macro_generated_fn());
    }
}

mod injector_unignores_args {
    use super::*;

    #[mockable]
    pub fn two_args_returns_first_ignores_second(x: u32, _: u32) -> u32 {
        x
    }

    #[test]
    fn when_not_mocked_then_returns_first_arg() {
        assert_eq!(1, two_args_returns_first_ignores_second(1, 2));
    }

    #[test]
    fn when_mocked_then_returns_second_arg() {
        unsafe {
            two_args_returns_first_ignores_second.mock_raw(|x, y| MockResult::Continue((y, x)));
        }

        assert_eq!(2, two_args_returns_first_ignores_second(1, 2));
    }
}

mod injecting_trait_impl_where_fn_return_type_has_longer_lifetime_than_required_by_trait {
    use super::*;

    struct Struct;

    trait Trait {
        fn function(&self) -> &str;
    }

    #[mockable]
    impl Trait for Struct {
        fn function(&self) -> &'static str {
            "not mocked"
        }
    }

    #[test]
    fn when_not_mocked_then_runs_normally() {
        assert_eq!("not mocked", Struct.function());
    }

    #[test]
    fn when_mocked_then_returns_mock() {
        Struct::function.mock_safe(|_| MockResult::Return("mocked"));

        assert_eq!("mocked", Struct.function());
    }
}

mod injecting_fn_with_generic_return_type {
    use super::*;

    struct Struct<T>(T);

    #[mockable]
    impl<T> Struct<T> {
        fn function(self) -> T {
            self.0
        }
    }

    mod when_return_type_has_no_destructor {
        use super::*;

        #[test]
        fn and_not_mocked_then_runs_normally() {
            assert_eq!("not mocked", Struct("not mocked").function());
        }

        #[test]
        fn and_mocked_then_returns_mock() {
            Struct::function.mock_safe(|_| MockResult::Return("mocked"));

            assert_eq!("mocked", Struct("not mocked").function());
        }
    }

    mod when_return_type_has_destructor {
        use super::*;

        #[test]
        fn and_not_mocked_then_runs_normally() {
            assert_eq!(vec!["not mocked"], Struct(vec!["not mocked"]).function());
        }

        #[test]
        fn and_mocked_then_returns_mock() {
            Struct::function.mock_safe(|_| MockResult::Return(vec!["mocked"]));

            assert_eq!(vec!["mocked"], Struct(vec!["not mocked"]).function());
        }
    }
}

mod injecting_fn_with_arg_requiring_drop {
    use super::*;

    #[mockable]
    fn function(vec: Vec<&'static str>) -> &'static str {
        vec[0]
    }

    #[test]
    fn when_not_mocked_then_runs_normally() {
        assert_eq!("not mocked", function(vec!["not mocked"]));
    }

    #[test]
    fn when_mocked_then_returns_mock() {
        function.mock_safe(|_| MockResult::Return("mocked"));

        assert_eq!("mocked", function(vec!["not mocked"]));
    }
}

mod injecting_fn_with_unused_generic_param {
    use super::*;

    #[mockable]
    fn function<T>() -> &'static str {
        "not mocked"
    }

    #[test]
    fn when_not_mocked_then_runs_normally() {
        assert_eq!("not mocked", function::<u8>());
    }

    #[test]
    fn when_mocked_then_returns_mock() {
        function::<u8>.mock_safe(|| MockResult::Return("mocked"));

        assert_eq!("mocked", function::<u8>());
    }
}

mod injecting_method_with_unused_generic_param {
    use super::*;

    struct Struct;

    #[mockable]
    impl Struct {
        fn method<T>() -> &'static str {
            "not mocked"
        }
    }

    #[test]
    fn when_not_mocked_then_runs_normally() {
        assert_eq!("not mocked", Struct::method::<u8>());
    }

    #[test]
    fn when_mocked_then_returns_mock() {
        Struct::method::<u8>.mock_safe(|| MockResult::Return("mocked"));

        assert_eq!("mocked", Struct::method::<u8>());
    }
}

mod injecting_trait_method_with_unused_generic_param {
    use super::*;

    trait Trait {
        fn method<T>() -> &'static str;
    }

    struct Struct;

    #[mockable]
    impl Trait for Struct {
        fn method<T>() -> &'static str {
            "not mocked"
        }
    }

    #[test]
    fn when_not_mocked_then_runs_normally() {
        assert_eq!("not mocked", Struct::method::<u8>());
    }

    #[test]
    fn when_mocked_then_returns_mock() {
        Struct::method::<u8>.mock_safe(|| MockResult::Return("mocked"));

        assert_eq!("mocked", Struct::method::<u8>());
    }
}

mod injecting_trait_default_method_with_unused_generic_param {
    use super::*;

    #[mockable]
    trait Trait {
        fn method<T>() -> &'static str {
            "not mocked"
        }
    }

    struct Struct;

    impl Trait for Struct {}

    #[test]
    fn when_not_mocked_then_runs_normally() {
        assert_eq!("not mocked", Struct::method::<u8>());
    }

    #[test]
    fn when_mocked_then_returns_mock() {
        Struct::method::<u8>.mock_safe(|| MockResult::Return("mocked"));

        assert_eq!("mocked", Struct::method::<u8>());
    }
}

mod injecting_structs_with_drop_does_nothing {
    use super::*;
    use std::mem::drop;

    static mut DROPPED: &str = "not dropped";

    struct Struct;

    #[mockable]
    impl Drop for Struct {
        fn drop(&mut self) {
            unsafe {
                DROPPED = "dropped"
            }
        }
    }

    #[test]
    fn when_not_mocked_then_runs_normally() {
        assert_eq!("not dropped", unsafe { DROPPED });

        drop(Struct);

        assert_eq!("dropped", unsafe { DROPPED });
    }
}

mod injecting_lifetimed_fn_of_same_lifetimed_trait_impl_of_same_lifetimed_struct_where_lifetime_on_fn_is_absent {
    use super::*;

    trait Trait<'a> {
        fn function(arg: &'a str) -> &'a str;
    }

    struct Struct<'a>(&'a str);

    #[mockable]
    impl<'a> Trait<'a> for Struct<'a> {
        fn function(arg: &str) -> &str {
            arg
        }
    }

    #[test]
    fn when_not_mocked_then_runs_normally() {
        assert_eq!("not mocked", Struct::function("not mocked"));
    }

    #[test]
    fn when_mocked_then_returns_mock() {
        Struct::function.mock_safe(|_| MockResult::Return("mocked"));

        assert_eq!("mocked", Struct::function("not mocked"));
    }
}
