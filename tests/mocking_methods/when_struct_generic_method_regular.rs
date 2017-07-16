use super::*;

struct Struct<T>(T);

#[inject_mocks]
impl<T: Display> Struct<T> {
    fn static_method(arg: bool) -> String {
        format!("{}", arg)
    }
}

mod and_method_is_static {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("true", Struct::<u8>::static_method(true));
        assert_eq!("true", Struct::<&str>::static_method(true));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args_for_mocked_type_only() {
        Struct::<u8>::static_method.set_mock(|a| MockResult::Continue((!a,)));

        assert_eq!("false", Struct::<u8>::static_method(true));
        assert_eq!("true", Struct::<&str>::static_method(true));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result_for_mocked_type_only() {
        Struct::<u8>::static_method.set_mock(|a| MockResult::Return(format!("mocked {}", a), ));

        assert_eq!("mocked true", Struct::<u8>::static_method(true));
        assert_eq!("true", Struct::<&str>::static_method(true));
    }
}
