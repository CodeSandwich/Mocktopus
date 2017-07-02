#![feature(const_fn, proc_macro)]

extern crate mocktopus_macro;
extern crate mocktopus;

use mocktopus_macro::inject_mocks;
use mocktopus::*;
use std::string::ToString;

#[inject_mocks]
pub fn single_generic_fn<T: ToString>(t: T) -> String {
    t.to_string()
}

mod mocking_works_for_generic_fns {
    use super::*;

    #[test]
    fn when_mocked_for_one_type_does_not_mock_for_others() {
        single_generic_fn::<u32>.set_mock(|_| MockResult::Return("mocked".to_string()));

        assert_eq!("mocked", single_generic_fn(1u32));
        assert_eq!("1", single_generic_fn(1i32));
        assert_eq!("not mocked", single_generic_fn("not mocked"));
    }

    #[test]
    fn when_mocked_for_type_does_not_mock_for_others() {
        single_generic_fn::<u32>.set_mock(|_| MockResult::Return("mocked".to_string()));

        assert_eq!("mocked", single_generic_fn(1u32));
        assert_eq!("1", single_generic_fn(1i32));
        assert_eq!("not mocked", single_generic_fn("not mocked"));
    }
}
