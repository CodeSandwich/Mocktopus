use super::*;

#[inject_mocks]
fn function(arg: bool) -> String {
    format!("{}", arg)
}

#[test]
fn and_not_mocked_then_runs_normally() {
    assert_eq!("true", function(true));
}

#[test]
fn and_continue_mocked_then_runs_with_modified_args() {
    function.mock_raw(|a| MockResult::Continue((!a,)));

    assert_eq!("false", function(true));
}

#[test]
fn and_return_mocked_then_returns_mocking_result() {
    function.mock_raw(|a| MockResult::Return(format!("mocked {}", a),));

    assert_eq!("mocked true", function(true));
}
