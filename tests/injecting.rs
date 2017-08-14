#![feature(const_fn, proc_macro)]

extern crate mocktopus;

use mocktopus::*;

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

        impl module::Trait for Struct {

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

    mod injects_nested_mods_content {
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

mod injector_does_not_inject_item_twice {
    use super::*;

    #[mockable]
    mod mocked_mod {
        #[mockable]
        pub fn mocked_fn(x: u32) -> u32 {
            x * 2
        }
    }

    #[test]
    fn double_annotated_function_gets_injected_once() {
        unsafe {
            mocked_mod::mocked_fn.mock_raw(|x| MockResult::Continue((x + 1,)));
        }

        assert_eq!(4, mocked_mod::mocked_fn(1));
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
