use super::*;
use std::time::Duration;
use tokio::time::delay_for;

struct Struct(u64);

#[mockable]
impl Struct {
    async fn ref_method(&self, arg: bool) -> String {
        delay_for(Duration::from_secs(self.0)).await;
        format!("{} {}", self.0, arg)
    }
}

mod and_method_is_ref_method {
    use super::*;

    #[tokio::test]
    async fn and_return_mocked_async_then_returns_mocking_result() {
        let struct_0 = Struct(2);
        unsafe {
            Struct::ref_method.mock_raw(|a, b| {
                MockResult::Return(Box::pin(async move { format!("mocked {} {}", a.0, b) }))
            });
        }

        assert_eq!("mocked 2 true", struct_0.ref_method(true).await);
        assert_eq!(2, struct_0.0);
    }
}
