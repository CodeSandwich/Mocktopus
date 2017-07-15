#![feature(const_fn, proc_macro)]

extern crate mocktopus_injector;
extern crate mocktopus;

use mocktopus_injector::inject_mocks;
use mocktopus::*;

#[inject_mocks]
pub fn no_args_returns_str() -> &'static str {
    "not mocked"
}

#[inject_mocks]
pub const fn const_fn() -> u32 {
    1
}

#[inject_mocks]
pub fn two_args_returns_first_ignores_second(x: u32, _: u32) -> u32 {
    x
}

macro_rules! fn_generating_macro {
    () => {
        pub fn macro_generated_fn() -> u32 {
            1
        }
    }
}

#[inject_mocks]
fn_generating_macro!();

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

#[inject_mocks]
mod mod_file_1;

#[inject_mocks]
mod twice_mock_annotated_mod {
    #[inject_mocks]
    pub fn twice_mock_annotated_fn(x: u32) -> u32 {
        x * 2
    }
}

mod mocks_do_not_leak_between_tests {
    use super::*;

    macro_rules! generate_tests {
        ($($fn_name:ident),+) => {
            $(
                #[test]
                fn $fn_name() {
                    assert_eq!("not mocked", no_args_returns_str(), "function was mocked before mocking");

                    no_args_returns_str.set_mock(|| MockResult::Return((stringify!($fn_name))));

                    assert_eq!(stringify!($fn_name), no_args_returns_str(), "mocking failed");
                }
            )+
        }
    }

    generate_tests!(t01, t02, t03, t04, t05, t06, t07, t08, t09, t10, t11, t12, t13, t14, t15, t16);
    generate_tests!(t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32);
    generate_tests!(t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48);
    generate_tests!(t49, t50, t51, t52, t53, t54, t55, t56, t57, t58, t59, t60, t61, t62, t63, t64);
}

mod mocking_does_not_works_for_const_fns {
    use super::*;

    #[test]
    fn when_not_mocked_then_returns_1() {
        assert_eq!(1, const_fn());
    }

    #[test]
    fn when_mocked_then_returns_1() {
        const_fn.set_mock(|| MockResult::Return(2));

        assert_eq!(1, const_fn());
    }
}

mod mocking_captures_ignored_args {
    use super::*;

    #[test]
    fn when_not_mocked_then_returns_first_arg() {
        assert_eq!(1, two_args_returns_first_ignores_second(1, 2));
    }

    #[test]
    fn when_mocked_then_returns_second_arg() {
        two_args_returns_first_ignores_second.set_mock(|x, y| MockResult::Continue((y, x)));

        assert_eq!(2, two_args_returns_first_ignores_second(1, 2));
    }
}

mod mocking_does_not_work_for_macro_generated_fns {
    use super::*;

    #[test]
    fn when_not_mocked_then_returns_1() {
        assert_eq!(1, macro_generated_fn());
    }

    #[test]
    fn when_mocked_then_returns_1() {
        macro_generated_fn.set_mock(|| MockResult::Return(2));

        assert_eq!(1, macro_generated_fn());
    }
}

mod mock_injecting_works_for_nested_mods {
    use super::*;

    #[test]
    fn when_not_mocked_then_returns_not_mocked_strs() {
        assert_eq!("mod_1_fn not mocked", mod_1::mod_1_fn());
        assert_eq!("mod_2_fn not mocked", mod_1::mod_2::mod_2_fn());
        assert_eq!("mod_3_fn not mocked", mod_1::mod_3::mod_3_fn());
    }

    #[test]
    fn when_mocked_then_returns_mocked_strs() {
        mod_1::mod_1_fn.set_mock(|| MockResult::Return("mod_1_fn mocked"));
        mod_1::mod_2::mod_2_fn.set_mock(|| MockResult::Return("mod_2_fn mocked"));
        mod_1::mod_3::mod_3_fn.set_mock(|| MockResult::Return("mod_3_fn mocked"));

        assert_eq!("mod_1_fn mocked", mod_1::mod_1_fn());
        assert_eq!("mod_2_fn mocked", mod_1::mod_2::mod_2_fn());
        assert_eq!("mod_3_fn mocked", mod_1::mod_3::mod_3_fn());
    }
}

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
        mod_file_1::mod_file_1_fn.set_mock(|| MockResult::Return("mod_file_1_fn mocked"));
        mod_file_1::mod_file_2::mod_file_2_fn.set_mock(|| MockResult::Return("mod_file_2_fn mocked"));
        mod_file_1::mod_file_3::mod_file_3_fn.set_mock(|| MockResult::Return("mod_file_3_fn mocked"));

        assert_eq!("mod_file_1_fn mocked", mod_file_1::mod_file_1_fn());
        assert_eq!("mod_file_2_fn mocked", mod_file_1::mod_file_2::mod_file_2_fn());
        assert_eq!("mod_file_3_fn mocked", mod_file_1::mod_file_3::mod_file_3_fn());
    }
}

mod twice_mock_annotated_fns {
    use super::*;
    use twice_mock_annotated_mod::twice_mock_annotated_fn;

    #[test]
    fn ___failing___when_fn_mock_annotated_twice_then_gets_injected_once() {
        twice_mock_annotated_fn.set_mock(|x| MockResult::Continue((x + 1,)));

        // Actually it injects twice. TODO fix
        //assert_eq!(4, twice_mock_annotated_fn(1));
    }
}
