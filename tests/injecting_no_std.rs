#![no_std]

extern crate mocktopus;

use mocktopus::macros::*;
use mocktopus::mocking::*;

mod injector_injects_fns_when_crate_is_no_std_but_std_is_available {
    use super::*;

    #[mockable]
    fn function(arg: u8) -> u8 {
        arg * 2
    }

    #[test]
    fn when_not_mocked_then_runs_normally() {
        assert_eq!(4, function(2));
    }

    #[test]
    fn when_mocked_then_runs_mock() {
        function.mock_safe(|_| MockResult::Return(3));

        assert_eq!(3, function(2));
    }
}
