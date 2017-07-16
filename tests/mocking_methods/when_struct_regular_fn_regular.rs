use super::*;

struct Struct(u8);

#[inject_mocks]
impl Struct {
    fn function(arg: bool) -> String {
        format!("{}", arg)
    }
}

mod and_method_is_function {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("true", Struct::function(true));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        Struct::function.set_mock(|a| MockResult::Continue((!a,)));

        assert_eq!("false", Struct::function(true));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        Struct::function.set_mock(|a| MockResult::Return(format!("mocked {}", a),));

        assert_eq!("mocked true", Struct::function(true));
    }
}
