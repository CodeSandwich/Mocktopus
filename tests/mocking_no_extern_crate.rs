#![feature(const_fn, proc_macro, extern_absolute_paths, proc_macro_path_invoc)]

mod mocking_with_absolute_path {
    use super::*;

    #[::mocktopus::macros::mockable]
    pub fn no_args_returns_str() -> &'static str {
        "not mocked"
    }

    #[test]
    fn when_not_mocked_then_returns_not_mocked() {
        assert_eq!("not mocked", no_args_returns_str());
    }

    #[test]
    fn when_mocked_then_returns_mocked() {
        extern crate mocktopus;

        use mocktopus::mocking::*;

        no_args_returns_str.mock_safe(|| MockResult::Return("mocked"));

        assert_eq!("mocked", no_args_returns_str());
    }
}
