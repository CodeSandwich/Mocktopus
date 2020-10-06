use super::*;

#[mockable]
async fn function<T: Display + Send>(arg: bool, fn_generic: T) -> String {
    format!("{} {}", arg, fn_generic)
}

#[tokio::test]
async fn and_not_mocked_then_runs_normally() {
    assert_eq!("true 2.5", function(true, 2.5f32).await);
    assert_eq!("true abc", function(true, "abc").await);
}

#[tokio::test]
async fn and_continue_mocked_then_runs_with_modified_args_for_mocked_type_only() {
    unsafe {
        function::<f32>.mock_raw(|a, b| MockResult::Continue((!a, b + 1.)));
    }

    assert_eq!("false 3.5", function(true, 2.5f32).await);
    assert_eq!("true abc", function(true, "abc").await);
}

#[tokio::test]
async fn and_return_mocked_then_returns_mocking_result_for_mocked_type_only() {
    unsafe {
        function::<f32>.mock_raw(|a, b| {
            MockResult::Return(Box::pin(async move { format!("mocked {} {}", a, b) }))
        });
    }

    assert_eq!("mocked true 2.5", function(true, 2.5f32).await);
    assert_eq!("true abc", function(true, "abc").await);
}
