use super::*;

#[mockable]
async fn function(arg: bool) -> String {
    format!("{}", arg)
}

#[tokio::test]
async fn and_not_mocked_then_runs_normally() {
    assert_eq!("true", function(true).await);
}

#[tokio::test]
async fn and_continue_mocked_then_runs_with_modified_args() {
    unsafe {
        function.mock_raw(|a| MockResult::Continue((!a,)));
    }

    assert_eq!("false", function(true).await);
}

#[tokio::test]
async fn and_return_mocked_then_returns_mocking_result() {
    unsafe {
        function.mock_raw(|a| MockResult::Return(Box::pin(async move { format!("mocked {}", a) })));
    }

    assert_eq!("mocked true", function(true).await);
}
