use super::*;

#[mockable]
fn function<T: Display>(arg: bool, fn_generic: T) -> String {
    format!("{} {}", arg, fn_generic)
}

#[test]
fn and_not_mocked_then_runs_normally() {
    assert_eq!("true 2.5", function(true, 2.5f32));
    assert_eq!("true abc", function(true, "abc"));
}

#[test]
fn and_continue_mocked_then_runs_with_modified_args_for_mocked_type_only() {
    unsafe {
        function::<f32>.mock_raw(|a, b| MockResult::Continue((!a, b + 1.)));
    }

    assert_eq!("false 3.5", function(true, 2.5f32));
    assert_eq!("true abc", function(true, "abc"));
}

#[test]
fn and_return_mocked_then_returns_mocking_result_for_mocked_type_only() {
    unsafe {
        function::<f32>.mock_raw(|a, b| MockResult::Return(format!("mocked {} {}", a, b)));
    }

    assert_eq!("mocked true 2.5", function(true, 2.5f32));
    assert_eq!("true abc", function(true, "abc"));
}
