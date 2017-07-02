#![feature(const_fn, proc_macro)]

extern crate mocktopus_macro;
extern crate mocktopus;

use mocktopus_macro::inject_mocks;
use mocktopus::*;
use std::str::FromStr;
use std::string::ToString;

#[inject_mocks]
pub fn single_generic_fn<T: ToString>(t: T) -> String {
    t.to_string()
}

#[inject_mocks]
pub fn single_generic_ret_fn<T: FromStr>(s: &str) -> T {
    s.parse().ok().unwrap()
}

static STATIC_U32: u32 = 1;

mod mocking_works_for_generic_fns {
    use super::*;

    #[test]
    fn when_mocked_for_one_type_does_not_mock_for_others() {
        single_generic_fn::<u32>.set_mock(|i| MockResult::Return(format!("mocked {}", i)));

        assert_eq!("mocked 1", single_generic_fn(1u32));
        assert_eq!("1", single_generic_fn(1i32));
    }

    #[test]
    fn when_mocked_for_one_type_mocks_for_all_lifetime_variants_but_not_other_types() {
        single_generic_fn::<&u32>.set_mock(|i| MockResult::Return(format!("mocked {}", i)));
        let local_u32: u32 = 1;

        assert_eq!("mocked 1", single_generic_fn(&1u32));
        assert_eq!("mocked 1", single_generic_fn(&local_u32));
        assert_eq!("mocked 1", single_generic_fn(&STATIC_U32));
        assert_eq!("1", single_generic_fn(&1i32));
    }
}

mod mocking_works_for_generic_ret_fns {
    use super::*;

    #[test]
    fn when_mocked_for_one_type_does_not_mock_for_others() {
        single_generic_ret_fn::<u32>.set_mock(|_| MockResult::Return(123));

        assert_eq!(123, single_generic_ret_fn::<u32>("1"));
        assert_eq!(1, single_generic_ret_fn::<i32>("1"));
    }
}
