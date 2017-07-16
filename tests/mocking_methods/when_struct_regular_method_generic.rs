use super::*;

struct Struct(u8);

#[inject_mocks]
impl Struct {
    fn static_method<T: Display>(arg: bool, method_generic: T) -> String {
        format!("{} {}", arg, method_generic)
    }
}

mod and_method_is_static {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("true 1.5", Struct::static_method(true, 1.5f32));
        assert_eq!("true abc", Struct::static_method(true, "abc"));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args_for_mocked_type_only() {
        Struct::static_method::<f32>.mock_raw(|a, b| MockResult::Continue((!a, b + 1.)));

        assert_eq!("false 3.5", Struct::static_method(true, 2.5f32));
        assert_eq!("true abc", Struct::static_method(true, "abc"));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result_for_mocked_type_only() {
        Struct::static_method::<f32>.mock_raw(|a, b| MockResult::Return(format!("mocked {} {}", a, b), ));

        assert_eq!("mocked true 2.5", Struct::static_method(true, 2.5f32));
        assert_eq!("true abc", Struct::static_method(true, "abc"));
    }
}
