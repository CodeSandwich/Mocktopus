use super::*;

struct Struct<T>(T);

#[inject_mocks]
impl<T: Display> Struct<T> {
    fn static_method<U: Display>(arg: bool, method_generic: U) -> String {
        format!("{} {}", arg, method_generic)
    }
}

mod and_method_is_static {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("true 2.5", Struct::<u8>::static_method(true, 2.5f32));
        assert_eq!("true abc", Struct::<u8>::static_method(true, "abc"));
        assert_eq!("true 2.5", Struct::<&str>::static_method(true, 2.5f32));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args_for_mocked_type_only() {
        Struct::<u8>::static_method::<f32>.mock_raw(|a, b| MockResult::Continue((!a, b + 1.)));

        assert_eq!("false 3.5", Struct::<u8>::static_method(true, 2.5f32));
        assert_eq!("true abc", Struct::<u8>::static_method(true, "abc"));
        assert_eq!("true 2.5", Struct::<&str>::static_method(true, 2.5f32));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result_for_mocked_type_only() {
        Struct::<u8>::static_method::<f32>.mock_raw(|a, b| MockResult::Return(format!("mocked {} {}", a, b), ));

        assert_eq!("mocked true 2.5", Struct::<u8>::static_method(true, 2.5f32));
        assert_eq!("true abc", Struct::<u8>::static_method(true, "abc"));
        assert_eq!("true 2.5", Struct::<&str>::static_method(true, 2.5f32));
    }
}
