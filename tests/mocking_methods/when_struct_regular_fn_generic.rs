use super::*;

struct Struct(u8);

#[inject_mocks]
impl Struct {
    fn function<T: Display>(arg: bool, fn_generic: T) -> String {
        format!("{} {}", arg, fn_generic)
    }
}

mod and_method_is_function {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("true 1.5", Struct::function(true, 1.5f32));
        assert_eq!("true abc", Struct::function(true, STATIC_STR));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args_for_mocked_type_only() {
        Struct::function::<f32>.set_mock(|a, b| MockResult::Continue((!a, b + 1.)));

        assert_eq!("false 3.5", Struct::function(true, 2.5f32));
        assert_eq!("true abc", Struct::function(true, STATIC_STR));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result_for_mocked_type_only() {
        Struct::function::<f32>.set_mock(|a, b| MockResult::Return(format!("mocked {} {}", a, b), ));

        assert_eq!("mocked true 2.5", Struct::function(true, 2.5f32));
        assert_eq!("true abc", Struct::function(true, STATIC_STR));
    }
}
