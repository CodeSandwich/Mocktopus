#![feature(const_fn, proc_macro)]

extern crate mocktopus_macro;
extern crate mocktopus;

use mocktopus_macro::inject_mocks;
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
    fn when_not_mocked_returns_1() {
        assert_eq!(1, const_fn());
    }

    #[test]
    fn when_mocked_returns_1() {
        const_fn.set_mock(|| MockResult::Return(2));

        assert_eq!(1, const_fn());
    }
}

mod mocking_captures_ignored_args {
    use super::*;

    #[test]
    fn when_not_mocked_returns_first_arg() {
        assert_eq!(1, two_args_returns_first_ignores_second(1, 2));
    }

    #[test]
    fn when_mocked_returns_second_arg() {
        two_args_returns_first_ignores_second.set_mock(|x, y| MockResult::Continue((y, x)));

        assert_eq!(2, two_args_returns_first_ignores_second(1, 2));
    }
}

mod mocking_does_not_work_for_macro_generated_fns {
    use super::*;

    #[test]
    fn when_not_mocked_returns_1() {
        assert_eq!(1, macro_generated_fn());
    }

    #[test]
    fn when_mocked_returns_1() {
        macro_generated_fn.set_mock(|| MockResult::Return(2));

        assert_eq!(1, macro_generated_fn());
    }
}
