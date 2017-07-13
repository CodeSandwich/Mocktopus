#![feature(const_fn, proc_macro)]

extern crate mocktopus_injector;
extern crate mocktopus;

use mocktopus_injector::inject_mocks;
use mocktopus::*;

struct Struct();

#[inject_mocks]
impl Struct {
    fn one_arg_multiplies_by_2(i: u32) -> u32 {
        i * 2
    }
}

mod mocking_works_for_single_arg_and_ret_fns {
    use super::*;

    #[test]
    fn when_mocked_continues_with_argument_incremented_by_1() {
        Struct::one_arg_multiplies_by_2.set_mock(|i| MockResult::Continue((i + 1,)));

        assert_eq!(6, Struct::one_arg_multiplies_by_2(2));
        assert_eq!(8, Struct::one_arg_multiplies_by_2(3));
        assert_eq!(10, Struct::one_arg_multiplies_by_2(4));
    }
}
