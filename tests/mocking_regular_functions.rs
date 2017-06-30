#![feature(const_fn, proc_macro)]

extern crate mocktopus_macro;
extern crate mocktopus;

#[inject_mocks]
mod regular_functions;

use mocktopus_macro::inject_mocks;
use mocktopus::*;
use regular_functions::*;

mod mocking_works_for_no_arg_no_ret_fns {
    use super::*;

    #[test]
    #[should_panic]
    fn when_not_mocked_panics() {
        no_args_no_ret_panics();
    }

    #[test]
    fn when_mocked_returns_and_does_not_panic() {
        no_args_no_ret_panics.set_mock(|| MockResult::Return(()));

        no_args_no_ret_panics();
    }

    #[test]
    #[should_panic]
    fn when_mocked_continues_and_panics() {
        no_args_no_ret_panics.set_mock(|| MockResult::Continue(()));

        no_args_no_ret_panics();
    }
}

mod mocking_works_for_single_arg_and_ret_fns {
    use super::*;

    #[test]
    fn when_not_mocked_multiplies_by_2() {
        assert_eq!(4, one_arg_multiplies_by_2(2));
        assert_eq!(6, one_arg_multiplies_by_2(3));
        assert_eq!(8, one_arg_multiplies_by_2(4));
    }

    #[test]
    fn when_mocked_returns_1() {
        one_arg_multiplies_by_2.set_mock(|_| MockResult::Return(1));

        assert_eq!(1, one_arg_multiplies_by_2(2));
        assert_eq!(1, one_arg_multiplies_by_2(3));
        assert_eq!(1, one_arg_multiplies_by_2(4));
    }

    #[test]
    fn when_mocked_continues_with_argument_incremented_by_1() {
        one_arg_multiplies_by_2.set_mock(|i| MockResult::Continue((i + 1,)));

        assert_eq!(6, one_arg_multiplies_by_2(2));
        assert_eq!(8, one_arg_multiplies_by_2(3));
        assert_eq!(10, one_arg_multiplies_by_2(4));
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
