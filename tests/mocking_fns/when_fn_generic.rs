use super::*;

#[inject_mocks]
fn function<T: Display>(arg: u8, fn_generic: T) -> String {
    format!("{} {}", arg, fn_generic)
}

#[test]
fn when_not_mocked_then_runs_normally() {
    assert_eq!("1 true", function(1, true));
}

#[test]
fn when_continue_mocked_then_runs_with_modified_args() {
    function::<bool>.set_mock(|a, b| MockResult::Continue((a + 1, !b)));

    assert_eq!("2 false", function(1, true));
}

#[test]
fn when_return_mocked_then_returns_mocking_result() {
    function::<bool>.set_mock(|a, b| MockResult::Return(format!("mocked {} {}", a, b),));

    assert_eq!("mocked 1 true", function(1, true));
}

#[test]
fn when_mocked_for_generic_type_then_does_not_mock_for_other_generic_types() {
    function::<bool>.set_mock(|a, b| MockResult::Return(format!("mocked {} {}", a, b),));

    assert_eq!("mocked 1 true", function(1, true));
    assert_eq!("2 3", function(2, 3));
}

#[test]
fn when_mocked_for_generic_with_lifetime_then_mocks_for_all_lifetime_variants() {
    function::<&bool>.set_mock(|a, b| MockResult::Return(format!("mocked {} {}", a, b),));
    let local = false;

    assert_eq!("mocked 1 false", function(1, &local));
    assert_eq!("mocked 2 true", function(2, &STATIC_BOOL));
}
