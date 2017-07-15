use super::*;

#[inject_mocks]
fn function(arg: u8) -> String {
    format!("{}", arg)
}

#[test]
fn when_not_mocked_then_runs_normally() {
    assert_eq!("1", function(1));
}

#[test]
fn when_continue_mocked_then_runs_with_modified_args() {
    function.set_mock(|a| MockResult::Continue((a + 1,)));

    assert_eq!("2", function(1));
}

#[test]
fn when_return_mocked_then_returns_mocking_result() {
    function.set_mock(|a| MockResult::Return(format!("mocked {}", a),));

    assert_eq!("mocked 1", function(1));
}
