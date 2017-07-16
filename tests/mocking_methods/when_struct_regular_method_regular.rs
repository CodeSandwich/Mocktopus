use super::*;

struct Struct(u8);

#[inject_mocks]
impl Struct {
    fn static_method(arg: bool) -> String {
        format!("{}", arg)
    }
}

mod and_method_is_static {
    use super::*;

    #[test]
    fn and_not_mocked_then_runs_normally() {
        assert_eq!("true", Struct::static_method(true));
    }

    #[test]
    fn and_continue_mocked_then_runs_with_modified_args() {
        Struct::static_method.mock_raw(|a| MockResult::Continue((!a,)));

        assert_eq!("false", Struct::static_method(true));
    }

    #[test]
    fn and_return_mocked_then_returns_mocking_result() {
        Struct::static_method.mock_raw(|a| MockResult::Return(format!("mocked {}", a),));

        assert_eq!("mocked true", Struct::static_method(true));
    }
}
