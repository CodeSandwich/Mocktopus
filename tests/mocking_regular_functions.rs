#![feature(const_fn, proc_macro)]

extern crate mocktopus_macro;
extern crate mocktopus;

use mocktopus_macro::inject_mocks;
use mocktopus::*;

#[inject_mocks]
pub fn no_args_no_ret_panics() {
    panic!("no_args_no_ret_panics was called");
}

#[inject_mocks]
pub fn one_arg_multiplies_by_2(x: u32) -> u32 {
    x * 2
}

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
