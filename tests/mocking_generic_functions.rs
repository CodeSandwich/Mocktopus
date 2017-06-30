#![feature(const_fn, proc_macro)]

extern crate mocktopus_macro;
extern crate mocktopus;

#[inject_mocks]
mod generic_functions;

use mocktopus_macro::inject_mocks;
use mocktopus::*;
use generic_functions::*;

mod mocking_works_for_generic_fns {
    use super::*;

    #[test]
    fn when_mocked_for_one_type_does_not_mock_for_others() {
        single_generic_fn::<u32>.set_mock(|_| MockResult::Return("mocked".to_string()));

        assert_eq!("mocked", single_generic_fn(1u32));
        assert_eq!("1", single_generic_fn(1i32));
        assert_eq!("not mocked", single_generic_fn("not mocked"));
    }

//    #[test]
//    fn when_not_mocked_multiplies_by_2() {
//        assert_eq!(4, one_arg_multiplies_by_2(2));
//        assert_eq!(6, one_arg_multiplies_by_2(3));
//        assert_eq!(8, one_arg_multiplies_by_2(4));
//    }
//
//    #[test]
//    fn when_mocked_returns_1() {
//        one_arg_multiplies_by_2.set_mock(|_| MockResult::Return(1));
//
//        assert_eq!(1, one_arg_multiplies_by_2(2));
//        assert_eq!(1, one_arg_multiplies_by_2(3));
//        assert_eq!(1, one_arg_multiplies_by_2(4));
//    }
//
//    #[test]
//    fn when_mocked_continues_with_argument_incremented_by_1() {
//        one_arg_multiplies_by_2.set_mock(|i| MockResult::Continue((i + 1,)));
//
//        assert_eq!(6, one_arg_multiplies_by_2(2));
//        assert_eq!(8, one_arg_multiplies_by_2(3));
//        assert_eq!(10, one_arg_multiplies_by_2(4));
//    }
}
