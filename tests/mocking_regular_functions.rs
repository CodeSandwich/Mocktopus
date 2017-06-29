#![feature(proc_macro)]

extern crate mocktopus_macro;
extern crate mocktopus;

#[inject_mocks]
mod regular_functions;

use mocktopus_macro::inject_mocks;
use mocktopus::*;
use regular_functions::*;

mod no_args_no_ret_panics {
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

mod one_arg_multiplies_by_2 {
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
