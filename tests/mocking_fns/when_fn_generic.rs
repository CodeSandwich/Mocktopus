use super::*;

#[inject_mocks]
fn function<T: Display>(arg: u8, fn_generic: T) -> String {
    format!("{} {}", arg, fn_generic)
}

#[test]
fn and_not_mocked_then_runs_normally() {
    assert_eq!("1 true", function(1, true));
    assert_eq!("1 abc", function(1, STATIC_STR));
}

#[test]
fn and_continue_mocked_then_runs_with_modified_args_for_mocked_type_only() {
    function::<bool>.set_mock(|a, b| MockResult::Continue((a + 1, !b)));

    assert_eq!("2 false", function(1, true));
    assert_eq!("1 abc", function(1, STATIC_STR));
}

#[test]
fn and_return_mocked_then_returns_mocking_result_for_mocked_type_only() {
    function::<bool>.set_mock(|a, b| MockResult::Return(format!("mocked {} {}", a, b),));

    assert_eq!("mocked 1 true", function(1, true));
    assert_eq!("1 abc", function(1, STATIC_STR));
}
